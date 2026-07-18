from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import subprocess
import tempfile
import unittest

from jsonschema import Draft202012Validator
from linkml_runtime.utils.schemaview import SchemaView
from pydantic import ValidationError
import yaml

from tools.gen_wire_format_test import rust_enum_variants
from tools.profile_validation import ValidationIssue, validate_profile


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema/generated"
STANDARDS = GENERATED / "standards"


def _load_module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load generated module: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _definition_validator(schema: dict, class_name: str):
    return Draft202012Validator(
        {"$ref": f"#/$defs/{class_name}", "$defs": schema["$defs"]}
    )


def _ref(value: dict) -> str:
    if "$ref" in value:
        return value["$ref"]
    branches = [branch for branch in value.get("anyOf", []) if "$ref" in branch]
    if len(branches) != 1:
        raise AssertionError(f"expected one class reference: {value}")
    return branches[0]["$ref"]


class StandardProfileTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.uad_schema = json.loads(
            (STANDARDS / "uad_3_6.schema.json").read_text(encoding="utf-8")
        )
        cls.boma_metro_schema = json.loads(
            (STANDARDS / "boma_building_class.metro.schema.json").read_text(
                encoding="utf-8"
            )
        )
        cls.boma_international_schema = json.loads(
            (
                STANDARDS
                / "boma_building_class.international.schema.json"
            ).read_text(encoding="utf-8")
        )
        cls.boma_schema = cls.boma_metro_schema
        cls.uad_models = _load_module(
            "phds_uad_3_6_generated", STANDARDS / "uad_3_6_pydantic.py"
        )
        cls.boma_models = _load_module(
            "phds_boma_generated",
            STANDARDS / "boma_building_class_pydantic.py",
        )

    def test_core_does_not_import_optional_rating_definitions(self):
        core = json.loads((GENERATED / "phds.schema.json").read_text())
        optional_names = (
            "Uad36ConditionRating",
            "Uad36QualityRating",
            "Uad36PropertyProfile",
            "BomaMetroPropertyProfile",
            "BomaInternationalPropertyProfile",
        )
        for name in optional_names:
            self.assertNotIn(name, core["$defs"])

        for name in ("core.yaml", "entities.yaml", "profiles.yaml", "capture.yaml"):
            source = (ROOT / "schema" / name).read_text(encoding="utf-8")
            with self.subTest(schema=name):
                self.assertNotIn("standards/", source)
                self.assertNotIn("uad_3_6", source)
                self.assertNotIn("boma_building_class", source)

    def test_optional_contracts_are_generated_separately(self):
        self.assertIn("Uad36PropertyProfile", self.uad_schema["$defs"])
        self.assertNotIn("BomaMetroPropertyProfile", self.uad_schema["$defs"])
        self.assertIn("BomaMetroPropertyProfile", self.boma_schema["$defs"])
        self.assertIn("BomaInternationalPropertyProfile", self.boma_schema["$defs"])
        self.assertNotIn("Uad36PropertyProfile", self.boma_schema["$defs"])

    def test_emitted_standard_schema_identities_are_unique_and_root_specific(self):
        expected = (
            (
                self.uad_schema,
                "https://example.org/phds/standards/uad-3-6",
                "phds_uad_3_6",
                "Uad36PropertyProfile",
            ),
            (
                self.boma_metro_schema,
                "https://example.org/phds/standards/boma-building-class/metro",
                "phds_boma_building_class_metro",
                "BomaMetroBuildingClassPropertyProfile",
            ),
            (
                self.boma_international_schema,
                "https://example.org/phds/standards/boma-building-class/international",
                "phds_boma_building_class_international",
                "BomaInternationalBuildingClassPropertyProfile",
            ),
        )
        self.assertEqual(len(expected), len({schema["$id"] for schema, *_ in expected}))
        for schema, schema_id, name, root_class in expected:
            with self.subTest(root_class=root_class):
                self.assertEqual(schema_id, schema["$id"])
                self.assertEqual(name, schema["title"])
                self.assertIn(root_class, schema["$defs"])
                self.assertEqual(
                    schema["$defs"][root_class]["description"],
                    schema["description"],
                )

    def test_boma_root_schemas_have_one_way_authored_imports(self):
        standards = ROOT / "schema/standards"
        definitions = yaml.safe_load(
            (standards / "boma_building_class.yaml").read_text(encoding="utf-8")
        )
        root_files = (
            (
                "boma_building_class_metro.yaml",
                "https://example.org/phds/standards/boma-building-class/metro",
                "phds_boma_building_class_metro",
                "PHDS BOMA Metro Office Building Class Profile",
                "BomaMetroBuildingClassPropertyProfile",
                "BomaMetroPropertyProfile",
            ),
            (
                "boma_building_class_international.yaml",
                "https://example.org/phds/standards/boma-building-class/international",
                "phds_boma_building_class_international",
                "PHDS BOMA International Office Building Class Profile",
                "BomaInternationalBuildingClassPropertyProfile",
                "BomaInternationalPropertyProfile",
            ),
        )
        for filename, schema_id, name, title, root_class, parent_class in root_files:
            source = yaml.safe_load((standards / filename).read_text(encoding="utf-8"))
            with self.subTest(filename=filename):
                self.assertEqual(schema_id, source["id"])
                self.assertEqual(name, source["name"])
                self.assertEqual(title, source["title"])
                self.assertEqual(["boma_building_class"], source["imports"])
                local_roots = [
                    name
                    for name, definition in source["classes"].items()
                    if definition.get("tree_root")
                ]
                self.assertEqual([root_class], local_roots)
                self.assertEqual(parent_class, source["classes"][root_class]["is_a"])

        self.assertNotIn("boma_building_class_metro", definitions["imports"])
        self.assertNotIn("boma_building_class_international", definitions["imports"])

    def test_emitted_root_schemas_validate_whole_documents_directly(self):
        valid_cases = (
            (self.uad_schema, "uad-property-profile.json"),
            (self.boma_metro_schema, "boma-metro-property-profile.json"),
            (
                self.boma_international_schema,
                "boma-international-property-profile.json",
            ),
        )
        for schema, fixture in valid_cases:
            payload = json.loads(
                (ROOT / "examples/standards" / fixture).read_text(encoding="utf-8")
            )
            with self.subTest(fixture=fixture):
                self.assertEqual([], list(Draft202012Validator(schema).iter_errors(payload)))

        invalid_cases = (
            (self.uad_schema, "uad-invalid-condition-code.json"),
            (self.uad_schema, "uad-missing-scope.json"),
            (self.boma_metro_schema, "boma-metro-invalid-code.json"),
            (
                self.boma_international_schema,
                "boma-international-invalid-code.json",
            ),
            (self.boma_metro_schema, "boma-system-code-mismatch.json"),
        )
        for schema, fixture in invalid_cases:
            payload = json.loads(
                (ROOT / "counter_examples/standards" / fixture).read_text(
                    encoding="utf-8"
                )
            )
            with self.subTest(fixture=fixture):
                self.assertTrue(list(Draft202012Validator(schema).iter_errors(payload)))

        generic_uad = {
            "property": {"id": "p"},
            "structures": [
                {
                    "id": "s",
                    "property": "p",
                    "condition_ratings": [
                        {
                            "system": "urn:vendor:condition",
                            "code": "average",
                        }
                    ],
                }
            ],
        }
        generic_boma = {
            "property": {"id": "p"},
            "structures": [
                {
                    "id": "s",
                    "property": "p",
                    "commercial": {
                        "market_classification": {
                            "system": "urn:vendor:market-class",
                            "code": "premium",
                        }
                    },
                }
            ],
        }
        self.assertTrue(
            list(Draft202012Validator(self.uad_schema).iter_errors(generic_uad))
        )
        self.assertTrue(
            list(
                Draft202012Validator(self.boma_metro_schema).iter_errors(
                    generic_boma
                )
            )
        )
        self.assertTrue(
            list(
                Draft202012Validator(self.boma_international_schema).iter_errors(
                    generic_boma
                )
            )
        )

    def test_standard_systems_and_codes_are_exact_and_coupled(self):
        expected = {
            "Uad36ConditionSystem": ["urn:phds:standard:uad:3.6:condition"],
            "Uad36QualitySystem": ["urn:phds:standard:uad:3.6:quality"],
            "Uad36ConditionCode": ["C1", "C2", "C3", "C4", "C5", "C6"],
            "Uad36QualityCode": ["Q1", "Q2", "Q3", "Q4", "Q5", "Q6"],
            "BomaMetroSystem": ["urn:phds:standard:boma:office:metro"],
            "BomaInternationalSystem": [
                "urn:phds:standard:boma:office:international"
            ],
            "BomaMetroCode": ["A", "B", "C"],
            "BomaInternationalCode": [
                "investment",
                "institutional",
                "speculative",
            ],
        }
        for name, values in expected.items():
            schema = self.uad_schema if name.startswith("Uad") else self.boma_schema
            with self.subTest(enum=name):
                self.assertEqual(values, schema["$defs"][name]["enum"])

        for schema, class_name, system, code in (
            (
                self.uad_schema,
                "Uad36ConditionRating",
                "Uad36ConditionSystem",
                "Uad36ConditionCode",
            ),
            (
                self.uad_schema,
                "Uad36QualityRating",
                "Uad36QualitySystem",
                "Uad36QualityCode",
            ),
            (
                self.boma_schema,
                "BomaMetroOfficeMarketClassification",
                "BomaMetroSystem",
                "BomaMetroCode",
            ),
            (
                self.boma_schema,
                "BomaInternationalOfficeMarketClassification",
                "BomaInternationalSystem",
                "BomaInternationalCode",
            ),
        ):
            properties = schema["$defs"][class_name]["properties"]
            required = schema["$defs"][class_name]["required"]
            with self.subTest(class_name=class_name):
                self.assertEqual(f"#/$defs/{system}", _ref(properties["system"]))
                self.assertEqual(f"#/$defs/{code}", _ref(properties["code"]))
                self.assertIn("system", required)
                self.assertIn("code", required)

        boma_source = yaml.safe_load(
            (ROOT / "schema/standards/boma_building_class.yaml").read_text(
                encoding="utf-8"
            )
        )
        international = boma_source["enums"]["BomaInternationalCode"][
            "permissible_values"
        ]
        self.assertEqual(
            {
                "investment": "Investment",
                "institutional": "Institutional",
                "speculative": "Speculative",
            },
            {code: value["description"] for code, value in international.items()},
        )
        self.assertEqual(
            ["https://boma.org/boma-standards/building-class-definitions"],
            boma_source["see_also"],
        )

    def test_uad_scope_is_required_and_documents_interior_context(self):
        for class_name in ("Uad36ConditionRating", "Uad36QualityRating"):
            definition = self.uad_schema["$defs"][class_name]
            with self.subTest(class_name=class_name):
                self.assertIn("scope", definition["required"])
                self.assertEqual(
                    "#/$defs/Uad36RatingScope",
                    _ref(definition["properties"]["scope"]),
                )
        source = (ROOT / "schema/standards/uad_3_6.yaml").read_text()
        self.assertIn("appraised dwelling or unit", source)
        self.assertIn("Space-level UAD mapping is not implied", source)

    def test_whole_profile_ranges_are_constrained(self):
        cases = (
            (
                self.uad_schema,
                "Uad36Structure",
                "condition_ratings",
                "Uad36ConditionRating",
            ),
            (
                self.uad_schema,
                "Uad36StructureState",
                "quality_ratings",
                "Uad36QualityRating",
            ),
            (
                self.uad_schema,
                "Uad36PropertyStateSnapshot",
                "structure_states",
                "Uad36StructureState",
            ),
            (
                self.uad_schema,
                "Uad36PropertyProfile",
                "structures",
                "Uad36Structure",
            ),
            (
                self.uad_schema,
                "Uad36PropertyProfile",
                "property_state_snapshots",
                "Uad36PropertyStateSnapshot",
            ),
            (
                self.boma_schema,
                "BomaMetroCommercialDetails",
                "market_classification",
                "BomaMetroOfficeMarketClassification",
            ),
            (
                self.boma_schema,
                "BomaInternationalCommercialDetails",
                "market_classification",
                "BomaInternationalOfficeMarketClassification",
            ),
            (
                self.boma_schema,
                "BomaMetroStructure",
                "commercial",
                "BomaMetroCommercialDetails",
            ),
            (
                self.boma_schema,
                "BomaInternationalStructureState",
                "commercial",
                "BomaInternationalCommercialDetails",
            ),
            (
                self.boma_schema,
                "BomaMetroPropertyProfile",
                "structures",
                "BomaMetroStructure",
            ),
            (
                self.boma_schema,
                "BomaMetroPropertyProfile",
                "property_state_snapshots",
                "BomaMetroPropertyStateSnapshot",
            ),
            (
                self.boma_schema,
                "BomaMetroPropertyStateSnapshot",
                "structure_states",
                "BomaMetroStructureState",
            ),
            (
                self.boma_schema,
                "BomaInternationalPropertyProfile",
                "structures",
                "BomaInternationalStructure",
            ),
            (
                self.boma_schema,
                "BomaInternationalPropertyProfile",
                "property_state_snapshots",
                "BomaInternationalPropertyStateSnapshot",
            ),
            (
                self.boma_schema,
                "BomaInternationalPropertyStateSnapshot",
                "structure_states",
                "BomaInternationalStructureState",
            ),
        )
        for schema, class_name, field, target in cases:
            with self.subTest(class_name=class_name, field=field):
                prop = schema["$defs"][class_name]["properties"][field]
                self.assertEqual(f"#/$defs/{target}", _ref(prop.get("items", prop)))

    def test_valid_whole_profiles_pass_json_schema_and_pydantic(self):
        cases = (
            (
                self.uad_schema,
                self.uad_models,
                "Uad36PropertyProfile",
                "uad-property-profile.json",
            ),
            (
                self.boma_schema,
                self.boma_models,
                "BomaMetroPropertyProfile",
                "boma-metro-property-profile.json",
            ),
            (
                self.boma_schema,
                self.boma_models,
                "BomaInternationalPropertyProfile",
                "boma-international-property-profile.json",
            ),
        )
        for schema, models, class_name, fixture in cases:
            payload = json.loads(
                (ROOT / "examples/standards" / fixture).read_text(encoding="utf-8")
            )
            with self.subTest(class_name=class_name, validator="jsonschema"):
                self.assertEqual(
                    [], list(_definition_validator(schema, class_name).iter_errors(payload))
                )
            with self.subTest(class_name=class_name, validator="pydantic"):
                getattr(models, class_name).model_validate(payload)

    def test_structural_counterexamples_fail_json_schema_and_pydantic(self):
        cases = (
            (self.uad_schema, self.uad_models, "Uad36PropertyProfile", "uad-missing-scope.json"),
            (self.uad_schema, self.uad_models, "Uad36PropertyProfile", "uad-invalid-condition-code.json"),
            (self.uad_schema, self.uad_models, "Uad36PropertyProfile", "uad-quality-rating-in-condition.json"),
            (self.boma_schema, self.boma_models, "BomaMetroPropertyProfile", "boma-metro-invalid-code.json"),
            (self.boma_schema, self.boma_models, "BomaInternationalPropertyProfile", "boma-international-invalid-code.json"),
            (self.boma_schema, self.boma_models, "BomaMetroPropertyProfile", "boma-system-code-mismatch.json"),
        )
        for schema, models, class_name, fixture in cases:
            payload = json.loads(
                (ROOT / "counter_examples/standards" / fixture).read_text(
                    encoding="utf-8"
                )
            )
            with self.subTest(fixture=fixture, validator="jsonschema"):
                self.assertTrue(
                    list(_definition_validator(schema, class_name).iter_errors(payload))
                )
            with self.subTest(fixture=fixture, validator="pydantic"):
                with self.assertRaises(ValidationError):
                    getattr(models, class_name).model_validate(payload)

    def test_boma_negative_placement_semantics(self):
        valid = ROOT / "examples/standards/boma-metro-property-profile.json"
        invalid = ROOT / "counter_examples/standards/boma-market-rating-in-condition.json"
        tool = ROOT / "tools/standards_validation.py"
        accepted = subprocess.run(
            [str(ROOT / ".venv/bin/python"), str(tool), str(valid)],
            cwd=ROOT,
            capture_output=True,
            text=True,
        )
        rejected = subprocess.run(
            [str(ROOT / ".venv/bin/python"), str(tool), str(invalid)],
            cwd=ROOT,
            capture_output=True,
            text=True,
        )
        self.assertEqual(0, accepted.returncode, accepted.stderr)
        self.assertNotEqual(0, rejected.returncode)
        self.assertIn("condition_ratings", rejected.stdout + rejected.stderr)

        validation = _load_module(
            "phds_standards_validation", ROOT / "tools/standards_validation.py"
        )
        nested = {
            "structures": [
                {
                    "quality_ratings": [
                        {
                            "system": "urn:phds:standard:boma:office:international",
                            "code": "investment",
                        }
                    ]
                }
            ],
            "property_state_snapshots": [
                {
                    "structure_states": [
                        {
                            "condition_ratings": [
                                {
                                    "system": "urn:phds:standard:boma:office:metro",
                                    "code": "A",
                                }
                            ]
                        }
                    ]
                }
            ],
        }
        issues = validation.boma_placement_issues(nested)
        self.assertEqual(2, len(issues))
        self.assertIn("structures[0].quality_ratings[0]", issues[0])
        self.assertIn(
            "property_state_snapshots[0].structure_states[0].condition_ratings[0]",
            issues[1],
        )

    def test_standard_example_recipe_runs_core_semantics_before_profile_semantics(self):
        recipe = (ROOT / "justfile").read_text(encoding="utf-8")
        for fixture in (
            "uad-property-profile.json",
            "boma-metro-property-profile.json",
            "boma-international-property-profile.json",
        ):
            with self.subTest(fixture=fixture):
                core = f"tools/profile_validation.py examples/standards/{fixture}"
                profile = f"tools/standards_validation.py examples/standards/{fixture}"
                self.assertIn(core, recipe)
                self.assertIn(profile, recipe)
                self.assertLess(recipe.index(core), recipe.index(profile))

    def test_standard_fixture_can_be_structurally_valid_but_core_semantically_invalid(self):
        recipe = (ROOT / "justfile").read_text(encoding="utf-8")
        self.assertIn("uad-core-semantic-invalid.json", recipe)
        self.assertIn(
            "tools/profile_validation.py counter_examples/standards/"
            "uad-core-semantic-invalid.json",
            recipe,
        )
        payload = json.loads(
            (ROOT / "counter_examples/standards/uad-core-semantic-invalid.json").read_text(
                encoding="utf-8"
            )
        )
        self.assertEqual(
            [],
            list(
                _definition_validator(
                    self.uad_schema, "Uad36PropertyProfile"
                ).iter_errors(payload)
            ),
        )
        self.assertEqual(
            [
                ValidationIssue(
                    "structures[0].id",
                    "duplicate Entity id; first declared at property.id",
                )
            ],
            validate_profile(payload),
        )

    def test_typescript_and_rust_preserve_profile_ranges_and_wire_values(self):
        uad_ts = (STANDARDS / "uad_3_6.ts").read_text(encoding="utf-8")
        boma_ts = (STANDARDS / "boma_building_class.ts").read_text(encoding="utf-8")
        uad_rust = (STANDARDS / "uad_3_6-rust/src/lib.rs").read_text(
            encoding="utf-8"
        )
        boma_rust = (
            STANDARDS / "boma_building_class-rust/src/lib.rs"
        ).read_text(encoding="utf-8")
        uad_wire = (
            STANDARDS / "uad_3_6-rust/tests/wire_format.rs"
        ).read_text(encoding="utf-8")
        boma_wire = (
            STANDARDS / "boma_building_class-rust/tests/wire_format.rs"
        ).read_text(encoding="utf-8")

        for source, snippets in (
            (
                uad_ts,
                (
                    "condition_ratings?: Uad36ConditionRating[]",
                    "quality_ratings?: Uad36QualityRating[]",
                    "structures?: Uad36Structure[]",
                    "property_state_snapshots?: Uad36PropertyStateSnapshot[]",
                ),
            ),
            (
                boma_ts,
                (
                    "market_classification?: BomaMetroOfficeMarketClassification",
                    "market_classification?: BomaInternationalOfficeMarketClassification",
                    "structures?: BomaMetroStructure[]",
                    "structures?: BomaInternationalStructure[]",
                ),
            ),
            (
                uad_rust,
                (
                    "pub condition_ratings: Option<Vec<Uad36ConditionRating>>",
                    "pub quality_ratings: Option<Vec<Uad36QualityRating>>",
                    "pub structures: Option<Vec<Uad36Structure>>",
                ),
            ),
            (
                boma_rust,
                (
                    "pub market_classification: Option<BomaMetroOfficeMarketClassification>",
                    "pub market_classification: Option<BomaInternationalOfficeMarketClassification>",
                    "pub structures: Option<Vec<BomaMetroStructure>>",
                    "pub structures: Option<Vec<BomaInternationalStructure>>",
                ),
            ),
        ):
            for snippet in snippets:
                with self.subTest(snippet=snippet):
                    self.assertIn(snippet, source)

        for wire, values in (
            (
                uad_wire,
                (
                    "urn:phds:standard:uad:3.6:condition",
                    "urn:phds:standard:uad:3.6:quality",
                    '"C1"',
                    '"Q6"',
                ),
            ),
            (
                boma_wire,
                (
                    "urn:phds:standard:boma:office:metro",
                    "urn:phds:standard:boma:office:international",
                    '"A"',
                    '"investment"',
                    '"institutional"',
                    '"speculative"',
                ),
            ),
        ):
            for value in values:
                with self.subTest(wire_value=value):
                    self.assertIn(value, wire)

        self.assertIn("uad_profile_rejects_invalid_ratings", uad_wire)
        self.assertIn("missing_scope", uad_wire)
        self.assertIn("quality_rating_in_condition", uad_wire)
        self.assertIn("boma_profiles_reject_invalid_ratings", boma_wire)
        self.assertIn("metro_invalid_code", boma_wire)
        self.assertIn("international_invalid_code", boma_wire)
        self.assertIn("system_code_mismatch", boma_wire)
        for wire in (uad_wire, boma_wire):
            with self.subTest(contract="generic standards strictness"):
                self.assertIn("generic_unknown_fields_are_rejected", wire)
                self.assertIn("generic_patterned_strings_are_rejected", wire)

    def test_wire_generator_supports_alternate_contract_paths_and_crate(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "standard.schema.json"
            rust_path = root / "lib.rs"
            output_path = root / "wire_format.rs"
            schema_path.write_text(
                json.dumps(
                    {
                        "$defs": {
                            "ExampleCode": {
                                "type": "string",
                                "enum": ["canonical_value"],
                            }
                        }
                    }
                ),
                encoding="utf-8",
            )
            rust_path.write_text(
                """\
pub enum ExampleCode {
    #[cfg_attr(feature = "serde", serde(rename = "canonical_value"))]
    CanonicalValue,
}
""",
                encoding="utf-8",
            )

            subprocess.run(
                [
                    str(ROOT / ".venv/bin/python"),
                    str(ROOT / "tools/gen_wire_format_test.py"),
                    "--json-schema",
                    str(schema_path),
                    "--rust-lib",
                    str(rust_path),
                    "--output",
                    str(output_path),
                    "--crate",
                    "example_standard",
                ],
                check=True,
                cwd=ROOT,
                capture_output=True,
                text=True,
            )

            output = output_path.read_text(encoding="utf-8")
            self.assertIn("use example_standard::*;", output)
            self.assertIn(
                'assert_round_trip(ExampleCode::CanonicalValue, "canonical_value");',
                output,
            )

    def test_wire_generator_rejects_rust_only_enum_type(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "standard.schema.json"
            rust_path = root / "lib.rs"
            output_path = root / "wire_format.rs"
            schema_path.write_text(
                json.dumps(
                    {
                        "$defs": {
                            "ExampleCode": {
                                "type": "string",
                                "enum": ["canonical_value"],
                            }
                        }
                    }
                ),
                encoding="utf-8",
            )
            rust_path.write_text(
                """\
pub enum ExampleCode {
    #[cfg_attr(feature = "serde", serde(rename = "canonical_value"))]
    CanonicalValue,
}
pub enum RustOnlyCode {
    #[cfg_attr(feature = "serde", serde(rename = "extra"))]
    Extra,
}
""",
                encoding="utf-8",
            )
            result = subprocess.run(
                [
                    str(ROOT / ".venv/bin/python"),
                    str(ROOT / "tools/gen_wire_format_test.py"),
                    "--json-schema",
                    str(schema_path),
                    "--rust-lib",
                    str(rust_path),
                    "--output",
                    str(output_path),
                    "--crate",
                    "example_standard",
                ],
                check=False,
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(0, result.returncode)
            self.assertIn("RustOnlyCode", result.stderr + result.stdout)

    def test_wire_generator_rejects_enum_missing_from_both_generated_contracts(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            linkml_path = root / "standard.yaml"
            schema_path = root / "standard.schema.json"
            rust_path = root / "lib.rs"
            output_path = root / "wire_format.rs"
            linkml_path.write_text(
                """\
id: https://example.org/test
name: test
prefixes:
  test: https://example.org/test/
default_prefix: test
enums:
  ExampleCode:
    permissible_values:
      canonical_value: {}
  MissingCode:
    permissible_values:
      missing_value: {}
""",
                encoding="utf-8",
            )
            schema_path.write_text(
                json.dumps(
                    {
                        "$defs": {
                            "ExampleCode": {
                                "type": "string",
                                "enum": ["canonical_value"],
                            }
                        }
                    }
                ),
                encoding="utf-8",
            )
            rust_path.write_text(
                """\
pub enum ExampleCode {
    #[cfg_attr(feature = "serde", serde(rename = "canonical_value"))]
    CanonicalValue,
}
""",
                encoding="utf-8",
            )

            result = subprocess.run(
                [
                    str(ROOT / ".venv/bin/python"),
                    str(ROOT / "tools/gen_wire_format_test.py"),
                    "--linkml-schema",
                    str(linkml_path),
                    "--json-schema",
                    str(schema_path),
                    "--rust-lib",
                    str(rust_path),
                    "--output",
                    str(output_path),
                    "--crate",
                    "example_standard",
                ],
                check=False,
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
            self.assertNotEqual(0, result.returncode)
            self.assertIn("MissingCode", result.stderr + result.stdout)

    def test_generated_enum_census_matches_linkml_import_closure(self):
        contracts = (
            (
                ROOT / "schema/standards/uad_3_6.yaml",
                STANDARDS / "uad_3_6.schema.json",
                STANDARDS / "uad_3_6-rust/src/lib.rs",
            ),
            (
                ROOT / "schema/standards/boma_building_class.yaml",
                STANDARDS / "boma_building_class.metro.schema.json",
                STANDARDS / "boma_building_class-rust/src/lib.rs",
            ),
        )
        for linkml_path, schema_path, rust_path in contracts:
            with self.subTest(schema=linkml_path.name):
                expected = set(
                    SchemaView(str(linkml_path)).all_enums(imports=True)
                )
                schema = json.loads(schema_path.read_text(encoding="utf-8"))
                json_schema_enums = {
                    name
                    for name, definition in schema["$defs"].items()
                    if definition.get("type") == "string"
                    and "enum" in definition
                }
                rust_enums = set(
                    rust_enum_variants(rust_path.read_text(encoding="utf-8"))
                )
                self.assertEqual(expected, json_schema_enums)
                self.assertEqual(expected, rust_enums)

    def test_standard_wire_tests_round_trip_each_shipped_example(self):
        contracts = (
            (
                "phds_uad_3_6",
                STANDARDS / "uad_3_6.schema.json",
                STANDARDS / "uad_3_6-rust/src/lib.rs",
            ),
            (
                "phds_boma_building_class",
                STANDARDS / "boma_building_class.metro.schema.json",
                STANDARDS / "boma_building_class-rust/src/lib.rs",
            ),
        )
        generated = {}
        with tempfile.TemporaryDirectory() as directory:
            for crate, schema_path, rust_path in contracts:
                output_path = Path(directory) / f"{crate}.rs"
                subprocess.run(
                    [
                        str(ROOT / ".venv/bin/python"),
                        str(ROOT / "tools/gen_wire_format_test.py"),
                        "--json-schema",
                        str(schema_path),
                        "--rust-lib",
                        str(rust_path),
                        "--output",
                        str(output_path),
                        "--crate",
                        crate,
                    ],
                    check=True,
                    cwd=ROOT,
                    capture_output=True,
                    text=True,
                )
                generated[crate] = output_path.read_text(encoding="utf-8")

        uad_wire = generated["phds_uad_3_6"]
        boma_wire = generated["phds_boma_building_class"]

        self.assertIn("every_shipped_uad_example_round_trips", uad_wire)
        self.assertIn("uad-property-profile.json", uad_wire)
        self.assertIn("Uad36PropertyProfile", uad_wire)
        self.assertIn("every_shipped_boma_example_round_trips", boma_wire)
        self.assertIn("boma-metro-property-profile.json", boma_wire)
        self.assertIn("BomaMetroPropertyProfile", boma_wire)
        self.assertIn("boma-international-property-profile.json", boma_wire)
        self.assertIn("BomaInternationalPropertyProfile", boma_wire)
        self.assertIn('"condition_ratings":[{', uad_wire)
        self.assertIn('"quality_ratings":[{', uad_wire)
        self.assertNotIn('"condition_rating":', uad_wire)
        self.assertNotIn('"quality_rating":', uad_wire)
        for wire in (uad_wire, boma_wire):
            self.assertIn('env!("CARGO_MANIFEST_DIR")', wire)
            self.assertNotIn(str(ROOT), wire)

    def test_wire_generation_recipes_pin_linkml_enum_sources(self):
        recipe = (ROOT / "justfile").read_text(encoding="utf-8")
        expected_invocations = (
            "tools/gen_wire_format_test.py --linkml-schema schema/capture.yaml",
            "tools/gen_wire_format_test.py --linkml-schema "
            "schema/standards/uad_3_6.yaml",
            "tools/gen_wire_format_test.py --linkml-schema "
            "schema/standards/boma_building_class.yaml",
        )
        for invocation in expected_invocations:
            with self.subTest(invocation=invocation):
                self.assertIn(invocation, recipe)

    def test_wire_generator_keeps_default_core_contract(self):
        checked_out = GENERATED / "phds-rust/tests/wire_format.rs"
        original = checked_out.read_bytes()
        with tempfile.TemporaryDirectory() as directory:
            output_path = Path(directory) / "wire_format.rs"
            subprocess.run(
                [
                    str(ROOT / ".venv/bin/python"),
                    str(ROOT / "tools/gen_wire_format_test.py"),
                    "--json-schema",
                    str(GENERATED / "phds.schema.json"),
                    "--rust-lib",
                    str(GENERATED / "phds-rust/src/lib.rs"),
                    "--output",
                    str(output_path),
                ],
                check=True,
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
            output = output_path.read_text(encoding="utf-8")

        self.assertEqual(original, checked_out.read_bytes())
        self.assertIn("use phds::*;", output)
        self.assertIn("generic_unknown_fields_are_rejected", output)
        self.assertIn("every_shipped_core_example_round_trips", output)
        self.assertIn('env!("CARGO_MANIFEST_DIR")', output)
        self.assertNotIn(str(ROOT), output)


if __name__ == "__main__":
    unittest.main()
