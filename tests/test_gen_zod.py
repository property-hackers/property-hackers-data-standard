from __future__ import annotations

import ast
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from tools.zod_generator import GenerationError, generate_zod
from tools.zod_generator.generator import _definition_dependencies

ROOT = Path(__file__).resolve().parents[1]


class ZodGeneratorTests(unittest.TestCase):
    def test_generator_source_parses_with_python_3_10_grammar(self):
        source = (ROOT / "tools/zod_generator/generator.py").read_text(
            encoding="utf-8"
        )

        tree = ast.parse(source, feature_version=(3, 10))
        formatted_expressions = (
            ast.get_source_segment(source, node.value) or ""
            for node in ast.walk(tree)
            if isinstance(node, ast.FormattedValue)
        )
        self.assertFalse(
            any("\\" in expression for expression in formatted_expressions),
            "Python 3.10 and 3.11 reject backslashes in f-string expressions",
        )

    def test_generated_source_has_do_not_edit_banner(self):
        source = generate_zod(
            {"type": "string"},
            root_name="Fixture",
            source_name="fixture.schema.json",
        )

        self.assertTrue(
            source.startswith(
                "// Generated from fixture.schema.json by tools/gen_zod.py. "
                "DO NOT EDIT.\n"
            )
        )

    def assertStrictTypeScriptCompiles(self, source: str) -> None:
        with tempfile.TemporaryDirectory(dir=ROOT) as directory:
            fixture = Path(directory) / "recursive.zod.ts"
            fixture.write_text(source, encoding="utf-8")
            result = subprocess.run(
                [
                    ROOT / "node_modules/.bin/tsc",
                    "--allowImportingTsExtensions",
                    "--forceConsistentCasingInFileNames",
                    "--module",
                    "ESNext",
                    "--moduleResolution",
                    "Bundler",
                    "--noEmit",
                    "--skipLibCheck",
                    "false",
                    "--strict",
                    "--target",
                    "ES2022",
                    "--types",
                    "node",
                    fixture,
                ],
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
        self.assertEqual("", result.stdout + result.stderr)
        self.assertEqual(0, result.returncode)

    def test_any_of_emits_union(self):
        source = generate_zod(
            {"anyOf": [{"type": "string"}, {"type": "number"}]},
            root_name="StringOrNumber",
        )

        self.assertIn("z.union([z.string(), z.number()])", source)

    def test_single_branch_any_of_emits_branch_directly(self):
        source = generate_zod(
            {"anyOf": [{"type": "string"}]}, root_name="OnlyString"
        )

        self.assertIn("export const OnlyStringSchema = z.string();", source)

    def test_all_of_emits_stable_left_associated_intersections(self):
        source = generate_zod(
            {
                "allOf": [
                    {"type": "number", "minimum": 0},
                    {"type": "number", "maximum": 10},
                    {"type": "integer"},
                ]
            },
            root_name="BoundedInteger",
        )

        self.assertIn(
            "z.intersection(z.intersection(z.number().min(0), "
            "z.number().max(10)), z.number().int())",
            source,
        )

    def test_one_of_requires_exactly_one_successful_branch(self):
        source = generate_zod(
            {"oneOf": [{"type": "number"}, {"type": "integer"}]},
            root_name="ExclusiveNumber",
        )

        self.assertIn("z.union([z.number(), z.number().int()])", source)
        self.assertIn("schema.safeParse(value).success", source)
        self.assertIn("matchCount !== 1", source)

    def test_not_negates_branch_safe_parse(self):
        source = generate_zod(
            {"not": {"const": "forbidden"}}, root_name="AllowedValue"
        )

        self.assertIn("z.unknown().refine", source)
        self.assertIn("!z.literal(\"forbidden\").safeParse(value).success", source)
        self.assertNotIn("z.any()", source)

    def test_conditionals_select_a_branch_for_the_original_value(self):
        source = generate_zod(
            {
                "type": "object",
                "properties": {"kind": {"type": "string"}},
                "if": {
                    "type": "object",
                    "properties": {"kind": {"const": "business"}},
                    "required": ["kind"],
                },
                "then": {"required": ["ein"]},
                "else": {"required": ["ssn"]},
            },
            root_name="Party",
        )

        self.assertIn(".superRefine((value, ctx) =>", source)
        self.assertIn(".safeParse(value).success ?", source)
        self.assertIn("const result = selected.safeParse(value)", source)
        self.assertIn("for (const issue of result.error.issues)", source)
        self.assertIn("ctx.addIssue(issue)", source)

    def test_conditional_object_branch_uses_properties_inferred_base_type(self):
        source = generate_zod(
            {
                "properties": {"kind": {"type": "string"}},
                "if": {"properties": {"kind": {"const": "business"}}},
                "then": {"required": ["ein"]},
                "else": {"required": ["ssn"]},
            },
            root_name="InferredParty",
        )

        self.assertIn("z.looseObject", source)
        self.assertIn("Required property ein is missing", source)
        self.assertIn("Required property ssn is missing", source)

    def test_conditional_object_branch_preserves_nullable_object_base(self):
        source = generate_zod(
            {
                "type": ["object", "null"],
                "if": {"type": "object"},
                "then": {"required": ["name"]},
            },
            root_name="NullableObject",
        )

        self.assertIn("z.looseObject", source)
        self.assertIn("Required property name is missing", source)
        self.assertIn("z.null()", source)

    def test_string_type_rejects_incompatible_object_assertion(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/properties/value/required: "
            r"unsupported schema keyword 'required'",
        ):
            generate_zod(
                {
                    "type": "object",
                    "properties": {
                        "value": {"type": "string", "required": ["x"]}
                    },
                },
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_array_type_rejects_incompatible_object_assertion(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/properties/values/minProperties: "
            r"unsupported schema keyword 'minProperties'",
        ):
            generate_zod(
                {
                    "type": "object",
                    "properties": {
                        "values": {"type": "array", "minProperties": 1}
                    },
                },
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_explicit_null_type_rejects_object_assertion_at_type_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/type: unsupported schema type None",
        ):
            generate_zod(
                {"type": None, "required": ["x"]},
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_nested_unknown_keyword_reports_exact_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/properties/name/unhandled: "
            r"unsupported schema keyword 'unhandled'",
        ):
            generate_zod(
                {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string", "unhandled": 1}
                    },
                },
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_nested_definition_unknown_keyword_reports_exact_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/properties/name/\$defs/Local/"
            r"properties/value/unhandled: "
            r"unsupported schema keyword 'unhandled'",
        ):
            generate_zod(
                {
                    "type": "object",
                    "properties": {
                        "name": {
                            "$defs": {
                                "Local": {
                                    "type": "object",
                                    "properties": {
                                        "value": {
                                            "type": "string",
                                            "unhandled": 1,
                                        }
                                    },
                                }
                            },
                            "type": "string",
                        }
                    },
                },
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_then_and_else_without_if_are_inert_but_validated(self):
        source = generate_zod(
            {
                "then": {"type": "string"},
                "else": {"type": "number"},
            },
            root_name="InertConditionalBranches",
        )
        self.assertIn(
            "export const InertConditionalBranchesSchema = z.unknown();",
            source,
        )

        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/then/unhandled: "
            r"unsupported schema keyword 'unhandled'",
        ):
            generate_zod(
                {"then": {"type": "string", "unhandled": True}},
                root_name="InvalidInertBranch",
                source_name="fixture.schema.json",
            )

    def test_supported_annotations_are_accepted_at_nested_nodes(self):
        source = generate_zod(
            {
                "type": "object",
                "properties": {
                    "name": {
                        "$schema": "https://json-schema.org/draft/2020-12/schema",
                        "$id": "urn:example:name",
                        "$defs": {},
                        "title": "Name",
                        "description": "A name",
                        "examples": ["Ada"],
                        "deprecated": False,
                        "default": "Ada",
                        "readOnly": True,
                        "metamodel_version": "1.0",
                        "type": "string",
                    }
                },
            },
            root_name="Annotated",
        )

        self.assertIn('"name": z.string()', source)
        self.assertIn('readOnly: true', source)

    def test_examples_and_deprecated_are_preserved_exactly_in_metadata(self):
        source = generate_zod(
            {
                "type": "string",
                "examples": ["Ada", {"nested": [1, True, None]}],
                "deprecated": False,
            },
            root_name="Annotated",
        )

        self.assertIn(
            'deprecated: false, examples: ["Ada",{"nested":[1,true,null]}]',
            source,
        )

    def test_document_root_metadata_includes_identity_annotations(self):
        source = generate_zod(
            {
                "$schema": "https://json-schema.org/draft/2020-12/schema",
                "$id": "https://example.test/root.schema.json",
                "metamodel_version": "4.2.0",
                "version": "1.2.3",
                "readOnly": True,
                "type": "string",
            },
            root_name="MetadataRoot",
        )

        self.assertIn('$id: "https://example.test/root.schema.json"', source)
        self.assertIn(
            '$schema: "https://json-schema.org/draft/2020-12/schema"', source
        )
        self.assertIn('metamodel_version: "4.2.0"', source)
        self.assertIn('version: "1.2.3"', source)
        self.assertIn('readOnly: true', source)

    def test_all_live_json_schemas_generate_without_any(self):
        schemas = (
            ("schema/generated/phds.schema.json", "Phds"),
            ("schema/generated/standards/uad_3_6.schema.json", "Uad36"),
            (
                "schema/generated/standards/boma_building_class.metro.schema.json",
                "BomaMetro",
            ),
            (
                "schema/generated/standards/boma_building_class.international.schema.json",
                "BomaInternational",
            ),
        )
        for relative_path, root_name in schemas:
            with self.subTest(schema=relative_path):
                path = ROOT / relative_path
                source = generate_zod(
                    json.loads(path.read_text(encoding="utf-8")),
                    root_name=root_name,
                    source_name=relative_path,
                )
                self.assertIn(f"export const {root_name}Schema", source)
                self.assertTrue(source.endswith("\n"))
                self.assertFalse(source.endswith("\n\n"))
                self.assertNotIn("z.any()", source)

    def test_definition_dependencies_decode_escaped_pointer_names(self):
        definitions = {
            "Consumer": {
                "type": "object",
                "properties": {
                    "slash": {"$ref": "#/$defs/Slash~1Name"},
                    "tilde": {"$ref": "#/$defs/Tilde~0Name"},
                },
            },
            "Slash/Name": {"type": "string"},
            "Tilde~Name": {"type": "string"},
        }

        dependencies = _definition_dependencies(definitions)

        self.assertEqual(
            frozenset(("Slash/Name", "Tilde~Name")), dependencies["Consumer"]
        )

    def test_definition_dependencies_walk_only_schema_positions(self):
        target_names = (
            "PropertyTarget",
            "ItemsTarget",
            "PrefixTarget",
            "AnyTarget",
            "AllTarget",
            "OneTarget",
            "AdditionalTarget",
            "NotTarget",
            "IfTarget",
            "ThenTarget",
            "ElseTarget",
        )
        definitions = {name: {"type": "string"} for name in target_names}
        definitions["Consumer"] = {
            "properties": {"value": {"$ref": "#/$defs/PropertyTarget"}},
            "items": {"$ref": "#/$defs/ItemsTarget"},
            "prefixItems": [{"$ref": "#/$defs/PrefixTarget"}],
            "anyOf": [{"$ref": "#/$defs/AnyTarget"}],
            "allOf": [{"$ref": "#/$defs/AllTarget"}],
            "oneOf": [{"$ref": "#/$defs/OneTarget"}],
            "additionalProperties": {"$ref": "#/$defs/AdditionalTarget"},
            "not": {"$ref": "#/$defs/NotTarget"},
            "if": {"$ref": "#/$defs/IfTarget"},
            "then": {"$ref": "#/$defs/ThenTarget"},
            "else": {"$ref": "#/$defs/ElseTarget"},
            "description": {"$ref": "#/$defs/NotASchemaPosition"},
        }

        dependencies = _definition_dependencies(definitions)

        self.assertEqual(frozenset(target_names), dependencies["Consumer"])

    def test_named_definitions_emit_in_dependency_order(self):
        source = generate_zod(
            {
                "$defs": {
                    "Invoice": {
                        "type": "object",
                        "properties": {"total": {"$ref": "#/$defs/Money"}},
                        "required": ["total"],
                        "additionalProperties": False,
                    },
                    "Money": {
                        "description": "Non-negative invoice money.",
                        "type": "number",
                        "minimum": 0,
                    },
                },
                "$ref": "#/$defs/Invoice",
            },
            root_name="InvoiceDocument",
        )

        money_position = source.index("export const MoneySchema")
        invoice_position = source.index("export const InvoiceSchema")
        root_position = source.index("export const InvoiceDocumentSchema")
        self.assertLess(money_position, invoice_position)
        self.assertLess(invoice_position, root_position)
        self.assertIn(
            "/**\n * Non-negative invoice money.\n */\n"
            "export const MoneySchema",
            source,
        )
        self.assertIn('"total": MoneySchema', source)
        self.assertIn(
            "export type Money = z.infer<typeof MoneySchema>;", source
        )
        self.assertIn(
            "export type Invoice = z.infer<typeof InvoiceSchema>;", source
        )

    def test_document_root_is_emitted_without_its_definitions_keyword(self):
        source = generate_zod(
            {
                "$defs": {"Name": {"type": "string", "minLength": 1}},
                "type": "object",
                "properties": {"name": {"$ref": "#/$defs/Name"}},
                "required": ["name"],
                "additionalProperties": True,
            },
            root_name="PersonDocument",
        )

        self.assertIn("export const NameSchema = z.string().min(1);", source)
        self.assertIn(
            'export const PersonDocumentSchema = z.looseObject({"name": NameSchema});',
            source,
        )

    def test_root_name_cannot_collide_with_definition_export(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/root_name: root export name 'Person' "
            r"collides with definition 'Person'",
        ):
            generate_zod(
                {
                    "$defs": {"Person": {"type": "string"}},
                    "$ref": "#/$defs/Person",
                },
                root_name="Person",
                source_name="fixture.schema.json",
            )

    def test_self_recursive_object_property_uses_getter(self):
        source = generate_zod(
            {
                "$defs": {
                    "Node": {
                        "type": "object",
                        "properties": {
                            "value": {"type": "string"},
                            "child": {"$ref": "#/$defs/Node"},
                        },
                        "required": ["value"],
                        "additionalProperties": False,
                    }
                },
                "$ref": "#/$defs/Node",
            },
            root_name="NodeDocument",
        )

        self.assertIn("get child()", source)
        self.assertIn("return NodeSchema.optional();", source)
        self.assertNotIn("z.any()", source)

    def test_self_recursive_object_property_compiles_strict(self):
        source = generate_zod(
            {
                "$defs": {
                    "Node": {
                        "type": "object",
                        "properties": {
                            "value": {"type": "string"},
                            "child": {"$ref": "#/$defs/Node"},
                        },
                        "required": ["value"],
                        "additionalProperties": False,
                    }
                },
                "$ref": "#/$defs/Node",
            },
            root_name="NodeDocument",
        )

        self.assertStrictTypeScriptCompiles(source)

    def test_mutually_recursive_object_properties_use_getters(self):
        source = generate_zod(
            {
                "$defs": {
                    "B": {
                        "type": "object",
                        "properties": {"a": {"$ref": "#/$defs/A"}},
                        "additionalProperties": False,
                    },
                    "A": {
                        "type": "object",
                        "properties": {"b": {"$ref": "#/$defs/B"}},
                        "additionalProperties": False,
                    },
                },
                "$ref": "#/$defs/A",
            },
            root_name="MutualDocument",
        )

        self.assertIn("get b() { return BSchema.optional(); }", source)
        self.assertIn("get a() { return ASchema.optional(); }", source)
        self.assertNotIn("z.lazy", source)
        self.assertNotIn("z.any()", source)

    def test_mutually_recursive_object_properties_compile_strict(self):
        source = generate_zod(
            {
                "$defs": {
                    "B": {
                        "type": "object",
                        "properties": {"a": {"$ref": "#/$defs/A"}},
                        "additionalProperties": False,
                    },
                    "A": {
                        "type": "object",
                        "properties": {"b": {"$ref": "#/$defs/B"}},
                        "additionalProperties": False,
                    },
                },
                "$ref": "#/$defs/A",
            },
            root_name="MutualDocument",
        )

        self.assertStrictTypeScriptCompiles(source)

    def test_dangling_local_reference_reports_source_and_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/\$defs/Invoice/properties/total/\$ref: "
            r"dangling local reference: '#/\$defs/Missing'",
        ):
            generate_zod(
                {
                    "$defs": {
                        "Invoice": {
                            "type": "object",
                            "properties": {
                                "total": {"$ref": "#/$defs/Missing"}
                            },
                        }
                    },
                    "$ref": "#/$defs/Invoice",
                },
                root_name="InvoiceDocument",
                source_name="fixture.schema.json",
            )

    def test_external_reference_reports_source_and_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/\$ref: external reference is not supported: "
            r"'other\.schema\.json#/\$defs/Money'",
        ):
            generate_zod(
                {"$ref": "other.schema.json#/$defs/Money"},
                root_name="ExternalDocument",
                source_name="fixture.schema.json",
            )

    def test_reference_preserves_pattern_sibling_constraint(self):
        source = generate_zod(
            {
                "$defs": {"Code": {"type": "string"}},
                "$ref": "#/$defs/Code",
                "pattern": "^[A-Z]+$",
            },
            root_name="CodeDocument",
        )

        self.assertIn(
            'z.intersection(CodeSchema, z.string().regex(new RegExp("^[A-Z]+$")))',
            source,
        )

    def test_reference_preserves_numeric_bound_siblings(self):
        source = generate_zod(
            {
                "$defs": {"Amount": {"type": "number"}},
                "$ref": "#/$defs/Amount",
                "minimum": 0,
                "exclusiveMaximum": 100,
            },
            root_name="AmountDocument",
        )

        self.assertIn(
            "z.intersection(AmountSchema, z.number().min(0).lt(100))",
            source,
        )

    def test_reference_rejects_unsupported_assertion_sibling_at_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/properties/code/mystery: "
            r"unsupported schema keyword 'mystery'",
        ):
            generate_zod(
                {
                    "$defs": {"Code": {"type": "string"}},
                    "type": "object",
                    "properties": {
                        "code": {
                            "$ref": "#/$defs/Code",
                            "mystery": True,
                        }
                    },
                },
                root_name="CodeDocument",
                source_name="fixture.schema.json",
            )

    def test_non_object_reference_cycle_lists_all_members(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/\$defs: recursive definitions cannot be "
            r"emitted without weakening inference: A, B",
        ):
            generate_zod(
                {
                    "$defs": {
                        "A": {
                            "type": "array",
                            "items": {"$ref": "#/$defs/B"},
                        },
                        "B": {
                            "type": "array",
                            "items": {"$ref": "#/$defs/A"},
                        },
                    },
                    "$ref": "#/$defs/A",
                },
                root_name="CycleDocument",
                source_name="fixture.schema.json",
            )

    def test_recursive_catchall_fails_even_with_a_safe_property_edge(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/\$defs: recursive definitions cannot be "
            r"emitted without weakening inference: Node",
        ):
            generate_zod(
                {
                    "$defs": {
                        "Node": {
                            "type": "object",
                            "properties": {
                                "child": {"$ref": "#/$defs/Node"}
                            },
                            "additionalProperties": {
                                "$ref": "#/$defs/Node"
                            },
                        }
                    },
                    "$ref": "#/$defs/Node",
                },
                root_name="NodeDocument",
                source_name="fixture.schema.json",
            )

    def test_object_property_presence_and_nullability_are_independent(self):
        source = generate_zod(
            {
                "type": "object",
                "properties": {
                    "required_text": {"type": "string"},
                    "optional_text": {"type": "string"},
                    "nullable_text": {"type": ["string", "null"]},
                    "optional_nullable": {"type": ["string", "null"]},
                },
                "required": ["required_text", "nullable_text"],
                "additionalProperties": False,
            },
            root_name="Presence",
        )
        self.assertIn("z.strictObject({", source)
        self.assertIn('"required_text": z.string()', source)
        self.assertIn('"optional_text": z.string().optional()', source)
        self.assertIn('"nullable_text": z.string().nullable()', source)
        self.assertIn(
            '"optional_nullable": z.string().nullable().optional()', source
        )

    def test_open_object_checks_required_names_absent_from_properties(self):
        source = generate_zod(
            {
                "type": "object",
                "required": ["extension"],
                "additionalProperties": True,
            },
            root_name="OpenRequired",
        )
        self.assertIn("z.looseObject({})", source)
        self.assertIn(
            'Object.prototype.hasOwnProperty.call(value, "extension")', source
        )

    def test_closed_object_with_undeclared_required_name_is_unsatisfiable(self):
        source = generate_zod(
            {
                "type": "object",
                "required": ["undeclared"],
                "additionalProperties": False,
            },
            root_name="ClosedRequired",
        )
        self.assertIn("z.strictObject({})", source)
        self.assertIn(
            'Object.prototype.hasOwnProperty.call(value, "undeclared")', source
        )

    def test_current_formats_and_core_value_constraints(self):
        source = generate_zod(
            {
                "type": "object",
                "properties": {
                    "on": {"type": "string", "format": "date"},
                    "at": {"type": "string", "format": "date-time"},
                    "uri": {"type": "string", "format": "uri"},
                    "ratio": {"type": "number", "minimum": 0, "maximum": 1},
                    "count": {"type": "integer", "minimum": 1},
                    "status": {"enum": ["open", "closed"]},
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"},
                        "minItems": 1,
                        "maxItems": 3,
                    },
                },
                "required": ["on", "at", "uri", "ratio", "count", "status", "tags"],
                "additionalProperties": False,
            },
            root_name="Constrained",
        )
        self.assertIn('"on": z.iso.date()', source)
        self.assertIn('"at": rfc3339DateTimeSchema', source)
        self.assertIn('"uri": rfc3986UriSchema', source)
        self.assertIn("z.number().min(0).max(1)", source)
        self.assertIn("z.number().int().min(1)", source)
        self.assertIn('z.enum(["open", "closed"])', source)
        self.assertIn("z.array(z.string()).min(1).max(3)", source)
        self.assertNotIn("z.any()", source)

    def test_open_object_uses_loose_object(self):
        source = generate_zod(
            {"type": "object", "additionalProperties": True},
            root_name="OpenObject",
        )
        self.assertIn("z.looseObject({})", source)

    def test_schema_additional_properties_uses_typed_catchall(self):
        source = generate_zod(
            {"type": "object", "additionalProperties": {"type": "number"}},
            root_name="NumberValues",
        )
        self.assertIn("z.object({}).catchall(z.number())", source)

    def test_empty_schema_additional_properties_uses_unknown_catchall(self):
        source = generate_zod(
            {"type": "object", "additionalProperties": {}},
            root_name="UnknownValues",
        )
        self.assertIn("z.object({}).catchall(z.unknown())", source)
        self.assertNotIn("z.any()", source)

    def test_string_length_constraints(self):
        source = generate_zod(
            {"type": "string", "minLength": 2, "maxLength": 8},
            root_name="SizedText",
        )
        self.assertIn("z.string().min(2).max(8)", source)

    def test_format_preserves_pattern(self):
        source = generate_zod(
            {"type": "string", "format": "date", "pattern": "^2026-"},
            root_name="DatedText",
        )
        self.assertIn('z.iso.date().regex(new RegExp("^2026-"))', source)

    def test_unknown_format_reports_format_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/format: unsupported string format 'email'",
        ):
            generate_zod(
                {"type": "string", "format": "email"},
                root_name="Email",
                source_name="fixture.schema.json",
            )

    def test_exclusive_numeric_bounds(self):
        source = generate_zod(
            {"type": "number", "exclusiveMinimum": 0, "exclusiveMaximum": 10},
            root_name="ExclusiveNumber",
        )
        self.assertIn("z.number().gt(0).lt(10)", source)

    def test_numeric_multiple_of(self):
        source = generate_zod(
            {"type": "number", "multipleOf": 0.25}, root_name="QuarterStep"
        )
        self.assertIn("z.number().multipleOf(0.25)", source)

    def test_compound_const_renders_exact_recursive_schema(self):
        source = generate_zod(
            {"const": {"kind": "point", "coordinates": [1, 2]}},
            root_name="PointConstant",
        )
        self.assertIn(
            'z.strictObject({"kind": z.literal("point"), '
            '"coordinates": z.tuple([z.literal(1), z.literal(2)])})',
            source,
        )
        self.assertNotIn('z.literal({"coordinates"', source)

    def test_compound_enum_renders_union_of_exact_recursive_schemas(self):
        source = generate_zod(
            {"enum": [["open", 1], {"state": "closed"}]},
            root_name="CompoundChoice",
        )
        self.assertIn(
            'z.union([z.tuple([z.literal("open"), z.literal(1)]), '
            'z.strictObject({"state": z.literal("closed")})])',
            source,
        )

    def test_enum_is_intersected_with_sibling_string_constraints(self):
        source = generate_zod(
            {
                "type": "string",
                "enum": ["ok", "accepted"],
                "minLength": 3,
            },
            root_name="ConstrainedEnum",
        )
        self.assertIn(
            'z.intersection(z.enum(["ok", "accepted"]), z.string().min(3))',
            source,
        )

    def test_const_is_intersected_with_sibling_numeric_constraints(self):
        source = generate_zod(
            {"type": "number", "const": 1, "exclusiveMinimum": 1},
            root_name="ImpossibleConstant",
        )
        self.assertIn(
            "z.intersection(z.literal(1), z.number().gt(1))", source
        )

    def test_mixed_type_union_partitions_type_specific_constraints(self):
        source = generate_zod(
            {
                "type": ["string", "number"],
                "minLength": 2,
                "pattern": "^[A-Z]",
                "minimum": 0,
                "maximum": 10,
            },
            root_name="TextOrNumber",
        )
        self.assertIn(
            'z.union([z.string().regex(new RegExp("^[A-Z]")).min(2), '
            "z.number().min(0).max(10)])",
            source,
        )

    def test_null_only_type_union_rejects_unknown_keyword_at_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/unsupportedConstraint: "
            r"unsupported schema keyword 'unsupportedConstraint'",
        ):
            generate_zod(
                {"type": ["null"], "unsupportedConstraint": True},
                root_name="NullOnly",
                source_name="fixture.schema.json",
            )

    def test_prefix_items_emit_variadic_tuple(self):
        source = generate_zod(
            {
                "type": "array",
                "prefixItems": [{"type": "string"}, {"type": "number"}],
                "items": {"type": "boolean"},
            },
            root_name="TupleValues",
        )
        self.assertIn("z.tuple([z.string(), z.number()], z.boolean())", source)

    def test_empty_array_item_schema_uses_unknown(self):
        source = generate_zod(
            {"type": "array", "items": {}}, root_name="UnknownItems"
        )
        self.assertIn("z.array(z.unknown())", source)
        self.assertNotIn("z.any()", source)

    def test_unique_items_uses_sorted_json_value_canonicalization(self):
        source = generate_zod(
            {
                "type": "array",
                "items": {"type": "object", "additionalProperties": True},
                "uniqueItems": True,
            },
            root_name="UniqueValues",
        )
        self.assertIn("Object.keys(item as Record<string, unknown>).sort()", source)
        self.assertIn("new Set(value.map(canonicalize)).size === value.length", source)
        self.assertNotIn("new Set(value).size", source)

    def test_object_property_count_constraints(self):
        source = generate_zod(
            {
                "type": "object",
                "minProperties": 1,
                "maxProperties": 2,
                "additionalProperties": True,
            },
            root_name="SizedObject",
        )
        self.assertIn("Object.keys(value).length >= 1", source)
        self.assertIn("Object.keys(value).length <= 2", source)

    def test_scalar_root_preserves_pattern_metadata_and_type(self):
        source = generate_zod(
            {
                "title": "Amount",
                "description": "Decimal wire amount",
                "type": "string",
                "pattern": r"^-?[0-9]+(?:\.[0-9]+)?$",
            },
            root_name="AmountDocument",
        )
        self.assertIn('import * as z from "zod";', source)
        self.assertIn("export const AmountDocumentSchema", source)
        self.assertIn("new RegExp(", source)
        self.assertIn('title: "Amount"', source)
        self.assertIn('description: "Decimal wire amount"', source)
        self.assertIn(
            "export type AmountDocument = z.infer<typeof AmountDocumentSchema>;",
            source,
        )
        self.assertTrue(source.endswith("\n"))
        self.assertFalse(source.endswith("\n\n"))

    def test_default_is_metadata_not_a_transform(self):
        source = generate_zod(
            {"type": "string", "default": "draft"}, root_name="StatusDocument"
        )
        self.assertIn('default: "draft"', source)
        self.assertNotIn(".default(", source)

    def test_jsdoc_escapes_comment_terminators_and_multiline_text(self):
        source = generate_zod(
            {"type": "string", "description": "first line\ncloses */ comment"},
            root_name="Documented",
        )
        self.assertIn(" * first line", source)
        self.assertIn(" * closes *\\/ comment", source)

    def test_unknown_keyword_reports_source_and_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/mystery: unsupported schema keyword 'mystery'",
        ):
            generate_zod(
                {"type": "string", "mystery": True},
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_invalid_root_name_reports_source_and_explicit_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/root_name: "
            r"invalid TypeScript export name: 'invalid-name'",
        ):
            generate_zod(
                {"type": "string"},
                root_name="invalid-name",
                source_name="fixture.schema.json",
            )

    def test_invalid_definition_name_reports_escaped_definition_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/\$defs/invalid~1name: "
            r"invalid TypeScript export name: 'invalid/name'",
        ):
            generate_zod(
                {
                    "$defs": {"invalid/name": {"type": "string"}},
                    "type": "string",
                },
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_non_finite_numeric_bounds_report_source_and_pointer(self):
        cases = (
            ("minimum", float("nan")),
            ("exclusiveMaximum", float("inf")),
        )
        for keyword, value in cases:
            with self.subTest(keyword=keyword):
                with self.assertRaisesRegex(
                    GenerationError,
                    rf"fixture\.schema\.json:#/{keyword}: "
                    rf"schema keyword '{keyword}' must be a finite number",
                ):
                    generate_zod(
                        {"type": "number", keyword: value},
                        root_name="Fixture",
                        source_name="fixture.schema.json",
                    )

    def test_cli_requires_root_name(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "schema.json"
            path.write_text(json.dumps({"type": "string"}), encoding="utf-8")
            result = subprocess.run(
                (sys.executable, "tools/gen_zod.py", str(path)),
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
        self.assertNotEqual(0, result.returncode)
        self.assertIn("--root-name", result.stderr)
