"""The enum-aware TS generator must only add the missing enum branch —
every other upstream behavior (notably subproperty_of expansion, which
upstream checks before range) must be preserved verbatim."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]

SYNTHETIC_SCHEMA = """\
id: https://example.org/tstest
name: tstest
prefixes:
  linkml: https://w3id.org/linkml/
  ex: https://example.org/tstest/
default_prefix: ex
default_range: string
imports:
  - linkml:types
types:
  DecimalString:
    typeof: string
enums:
  Color:
    permissible_values: {red: {}, deep_blue: {}}
slots:
  measurement: {}
  height: {is_a: measurement}
classes:
  Reading:
    attributes:
      color: {range: Color}
      colors: {range: Color, multivalued: true}
      kind: {range: Color, subproperty_of: measurement, multivalued: true}
      ratio: {range: decimal}
      ratios: {range: decimal, multivalued: true}
      exact_amount: {range: DecimalString}
      observed_date: {range: date}
  RefinedReading:
    is_a: Reading
    slot_usage:
      color: {range: Color, required: true}
"""


def _load_tool():
    path = ROOT / "tools" / "gen_typescript.py"
    spec = importlib.util.spec_from_file_location("gen_typescript", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def _declaration(source: str, field: str) -> str:
    for line in source.splitlines():
        stripped = line.strip()
        if stripped.startswith(f"{field}:") or stripped.startswith(f"{field}?:"):
            return stripped
    raise AssertionError(f"no declaration for {field!r} in generated TypeScript")


def _interface_body(source: str, name: str) -> str:
    declaration = f"export interface {name}"
    start = source.index(declaration)
    body_start = source.index("{", start) + 1
    body_end = source.index("}", body_start)
    return source[body_start:body_end]


class EnumAwareTypescriptGeneratorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        from linkml.generators.typescriptgen import TypescriptGenerator

        cls.tool = _load_tool()
        cls.tmp = tempfile.TemporaryDirectory()
        schema_path = Path(cls.tmp.name) / "tstest.yaml"
        schema_path.write_text(SYNTHETIC_SCHEMA, encoding="utf-8")
        cls.upstream = TypescriptGenerator(str(schema_path)).serialize()
        cls.wrapped = cls.tool.EnumAwareTypescriptGenerator(str(schema_path)).serialize()

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def test_enum_slot_is_enum_typed(self):
        self.assertEqual("color?: Color,", _declaration(self.wrapped, "color"))

    def test_multivalued_enum_slot_is_enum_array_typed(self):
        self.assertEqual("colors?: Color[],", _declaration(self.wrapped, "colors"))

    def test_subproperty_of_precedence_matches_upstream(self):
        # Upstream expands subproperty_of before looking at range; the enum
        # branch must not shadow that.
        self.assertEqual(
            _declaration(self.upstream, "kind"),
            _declaration(self.wrapped, "kind"),
        )

    def test_decimal_slots_are_json_numbers_but_decimal_strings_remain_strings(self):
        self.assertEqual("ratio?: number,", _declaration(self.wrapped, "ratio"))
        self.assertEqual("ratios?: number[],", _declaration(self.wrapped, "ratios"))
        self.assertEqual(
            "exact_amount?: string,",
            _declaration(self.wrapped, "exact_amount"),
        )

    def test_linkml_dates_are_iso_8601_strings(self):
        self.assertEqual(
            "observed_date?: string,",
            _declaration(self.wrapped, "observed_date"),
        )
        self.assertNotIn("type date =", self.wrapped)

    def test_slot_usage_refinement_is_direct_without_flattening_inherited_slots(self):
        body = _interface_body(self.wrapped, "RefinedReading")
        self.assertIn("color: Color,", body)
        self.assertNotIn("colors", body)
        self.assertNotIn("kind", body)

    def test_generated_source_has_no_trailing_whitespace(self):
        self.assertTrue(self.wrapped.endswith("\n"))
        self.assertFalse(self.wrapped.endswith("\n\n"))
        self.assertEqual(
            [],
            [
                (line_number, line)
                for line_number, line in enumerate(
                    self.wrapped.splitlines(), start=1
                )
                if line != line.rstrip()
            ],
        )


if __name__ == "__main__":
    unittest.main()
