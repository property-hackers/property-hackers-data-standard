"""Project-owned Rust generator override contract."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import re
import tempfile
import unittest

from jinja2 import ChoiceLoader


ROOT = Path(__file__).resolve().parents[1]


class StrictRustGeneratorTests(unittest.TestCase):
    def _generator_module(self, suffix: str):
        tool_path = ROOT / "tools" / "gen_rust.py"
        spec = importlib.util.spec_from_file_location(f"gen_rust_{suffix}", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module

    def _generate_source(self, suffix: str, schema: str) -> tuple[str, str, str]:
        module = self._generator_module(suffix)
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            output = root / "crate"
            module.generate(schema_path, output, force=True, serde=True)
            rust = (output / "src/lib.rs").read_text(encoding="utf-8")
            serde_utils = (output / "src/serde_utils.rs").read_text(encoding="utf-8")
            poly = (output / "src/poly.rs").read_text(encoding="utf-8")
        return rust, serde_utils, poly

    def test_project_override_precedes_linkml_template_fallback(self):
        tool_path = ROOT / "tools" / "gen_rust.py"
        template_path = ROOT / "tools" / "rust_templates" / "struct.rs.jinja"
        self.assertTrue(tool_path.exists(), f"missing Rust wrapper: {tool_path}")
        self.assertTrue(template_path.exists(), f"missing Rust override: {template_path}")

        spec = importlib.util.spec_from_file_location("gen_rust", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        environment = module.template_environment()
        self.assertIsInstance(environment.loader, ChoiceLoader)
        struct_source, _, _ = environment.loader.get_source(
            environment, "struct.rs.jinja"
        )
        self.assertIn("serde(deny_unknown_fields)", struct_source)
        self.assertIsNotNone(environment.get_template("enum.rs.jinja"))

    def test_just_gen_uses_project_rust_wrapper(self):
        justfile = (ROOT / "justfile").read_text(encoding="utf-8")
        self.assertIn(
            ".venv/bin/python tools/gen_rust.py schema/capture.yaml",
            justfile,
        )

    def test_primitive_named_slot_does_not_duplicate_rust_type_alias(self):
        source = (
            ROOT / "schema" / "generated" / "phds-rust" / "src" / "lib.rs"
        ).read_text(encoding="utf-8")
        self.assertEqual(1, source.count("pub type uri ="))
        self.assertNotIn("pub type uri = uri;", source)

    def test_incompatible_primitive_slot_alias_collision_fails_generation(self):
        tool_path = ROOT / "tools" / "gen_rust.py"
        spec = importlib.util.spec_from_file_location("gen_rust_collision", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        schema = """\
id: https://example.org/collision
name: collision
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/collision/
classes:
  Target:
    attributes:
      value: {}
  Host:
    tree_root: true
    attributes:
      uri: {range: Target, inlined: true}
"""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "incompatible Rust alias collision: uri"):
                module.generate(schema_path, root / "crate", force=True, serde=True)

    def test_enum_refinement_does_not_inherit_string_deserializer(self):
        tool_path = ROOT / "tools" / "gen_rust.py"
        spec = importlib.util.spec_from_file_location("gen_rust_enum_refinement", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        schema = f"""\
id: https://example.org/enum-refinement
name: enum_refinement
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/enum-refinement/
enums:
  RatingSystem:
    permissible_values: {{example: {{}}}}
classes:
  Rating:
    attributes:
      system:
        required: true
        pattern: '{module.NONBLANK_TRIMMED_PATTERN}'
  ConstrainedRating:
    is_a: Rating
    slot_usage:
      system: {{range: RatingSystem}}
"""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            module.generate(schema_path, root / "crate", force=True, serde=True)
            rust = (root / "crate/src/lib.rs").read_text(encoding="utf-8")

        rating = re.search(r"pub struct Rating \{(.*?)\n\}", rust, re.DOTALL)
        constrained = re.search(
            r"pub struct ConstrainedRating \{(.*?)\n\}", rust, re.DOTALL
        )
        self.assertIsNotNone(rating)
        self.assertIsNotNone(constrained)
        self.assertIn("deserialize_nonblank_trimmed_string", rating.group(1))
        self.assertNotIn(
            "deserialize_nonblank_trimmed_string", constrained.group(1)
        )
        self.assertIn("pub system: RatingSystem", constrained.group(1))

    def test_uri_enum_wire_value_uses_a_valid_rust_identifier(self):
        tool_path = ROOT / "tools" / "gen_rust.py"
        spec = importlib.util.spec_from_file_location("gen_rust_uri_enum", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        schema = """\
id: https://example.org/uri-enum
name: uri_enum
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/uri-enum/
enums:
  RatingSystem:
    permissible_values:
      "urn:phds:standard:test:1.0": {}
classes:
  Rated:
    attributes:
      system: {range: RatingSystem, required: true}
"""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            module.generate(schema_path, root / "crate", force=True, serde=True)
            rust = (root / "crate/src/lib.rs").read_text(encoding="utf-8")

        self.assertIn('serde(rename = "urn:phds:standard:test:1.0")', rust)
        self.assertIn("UrnPhdsStandardTest10,", rust)
        self.assertNotIn("Urn:phds", rust)

    def test_omit_poly_keeps_concrete_profile_structs_without_broken_adapters(self):
        tool_path = ROOT / "tools" / "gen_rust.py"
        spec = importlib.util.spec_from_file_location("gen_rust_omit_poly", tool_path)
        self.assertIsNotNone(spec)
        self.assertIsNotNone(spec.loader)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)

        schema = """\
id: https://example.org/narrowed-profile
name: narrowed_profile
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/narrowed-profile/
classes:
  BaseValue:
    attributes:
      value: {}
  NarrowValue:
    is_a: BaseValue
  BaseProfile:
    attributes:
      item: {range: BaseValue}
  NarrowProfile:
    is_a: BaseProfile
    tree_root: true
    slot_usage:
      item: {range: NarrowValue}
"""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            output = root / "crate"
            (output / "src").mkdir(parents=True)
            (output / "src/poly.rs").write_text("stale", encoding="utf-8")
            (output / "src/poly_containers.rs").write_text(
                "stale", encoding="utf-8"
            )
            module.generate(
                schema_path,
                output,
                force=True,
                serde=True,
                omit_poly=True,
            )
            rust = (root / "crate/src/lib.rs").read_text(encoding="utf-8")
            self.assertFalse((root / "crate/src/poly.rs").exists())
            self.assertFalse((root / "crate/src/poly_containers.rs").exists())

        self.assertIn("pub struct NarrowProfile", rust)
        self.assertIn("pub item: Option<NarrowValue>", rust)
        self.assertNotIn("pub mod poly;", rust)

    def test_subtype_union_keeps_codeable_concept_base_as_last_variant(self):
        rust, _, poly = self._generate_source(
            "codeable_base",
            """\
id: https://example.org/codeable-base
name: codeable_base
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/codeable-base/
classes:
  CodeableConcept:
    attributes:
      system: {}
      code: {}
      display: {}
  Classification:
    is_a: CodeableConcept
    attributes:
      system: {required: true}
      code: {required: true}
  Rating:
    is_a: CodeableConcept
    attributes:
      system: {required: true}
      code: {required: true}
      scope: {}
  Document:
    tree_root: true
    attributes:
      kind: {range: CodeableConcept, inlined: true}
""",
        )
        union = re.search(
            r"pub enum CodeableConceptOrSubtype \{(.*?)\}", rust, re.DOTALL
        )
        self.assertIsNotNone(union)
        self.assertRegex(
            re.sub(r"\s+", " ", union.group(1)),
            r"Classification\(Classification\).*Rating\(Rating\).*CodeableConcept\(CodeableConcept\)\s*$",
        )
        self.assertIn(
            "CodeableConceptOrSubtype::CodeableConcept(val) => val.system()",
            poly,
        )

    def test_datetime_fields_use_fixed_offset_rfc3339_type(self):
        rust, _, _ = self._generate_source(
            "datetime",
            """\
id: https://example.org/datetime
name: datetime_contract
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/datetime/
classes:
  Observation:
    tree_root: true
    attributes:
      observed_at: {range: datetime, required: true}
""",
        )
        observation = re.search(r"pub struct Observation \{(.*?)\n\}", rust, re.DOTALL)
        self.assertIsNotNone(observation)
        self.assertIn("pub observed_at: DateTime<FixedOffset>", observation.group(1))
        self.assertIn("use chrono::{DateTime,FixedOffset}", rust)
        self.assertNotIn("NaiveDateTime", rust)

    def test_protected_rust_field_serializes_with_original_wire_name(self):
        rust, _, _ = self._generate_source(
            "protected_name",
            """\
id: https://example.org/protected-name
name: protected_name
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/protected-name/
classes:
  Geometry:
    tree_root: true
    attributes:
      type: {required: true}
""",
        )
        geometry = re.search(r"pub struct Geometry \{(.*?)\n\}", rust, re.DOTALL)
        self.assertIsNotNone(geometry)
        self.assertIn('serde(rename = "type")', geometry.group(1))
        self.assertIn("pub type_: String", geometry.group(1))

    def test_all_supported_string_patterns_get_rust_deserializers(self):
        nonblank_pattern = (
            r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A"
            r"\u2028\u2029\u202F\u205F\u3000\uFEFF]"
            r"(?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-"
            r"\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])"
        )
        rust, serde_utils, _ = self._generate_source(
            "patterns_source",
            f"""\
id: https://example.org/patterns
name: pattern_contract
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/patterns/
classes:
  Values:
    tree_root: true
    attributes:
      identifier: {{required: true, pattern: '{nonblank_pattern}'}}
      amount: {{required: true, pattern: '^-?[0-9]+(\\.[0-9]+)?(?![\\s\\S])'}}
      currency: {{required: true, pattern: '^[A-Z]{{3}}(?![\\s\\S])'}}
      country: {{required: true, pattern: '^[A-Z]{{2}}(?![\\s\\S])'}}
""",
        )
        for validator in (
            "nonblank_trimmed_string",
            "decimal_string",
            "iso_currency",
            "iso_country",
        ):
            with self.subTest(validator=validator):
                self.assertIn(
                    f"serde_utils::deserialize_{validator}",
                    rust,
                )
                self.assertIn(f"fn validate_{validator}", serde_utils)

    def test_unknown_string_pattern_fails_rust_generation(self):
        module = self._generator_module("unsupported_pattern")
        schema = """\
id: https://example.org/unsupported-pattern
name: unsupported_pattern
imports: [linkml:types]
prefixes:
  linkml: https://w3id.org/linkml/
default_prefix: https://example.org/unsupported-pattern/
classes:
  Value:
    tree_root: true
    attributes:
      code: {required: true, pattern: '^custom$'}
"""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            schema_path = root / "schema.yaml"
            schema_path.write_text(schema, encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "unsupported Rust string pattern"):
                module.generate(schema_path, root / "crate", force=True, serde=True)


if __name__ == "__main__":
    unittest.main()
