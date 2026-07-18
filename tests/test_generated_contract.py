"""Cross-generator conformance tests for committed PHDS artifacts."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import unittest

from jsonschema import Draft202012Validator
from pydantic import ValidationError


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema" / "generated"


def _load_pydantic_module():
    path = GENERATED / "phds_pydantic.py"
    spec = importlib.util.spec_from_file_location("phds_generated", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load generated module: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class GeneratedContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(
            (GENERATED / "phds.schema.json").read_text(encoding="utf-8")
        )
        cls.profile_validator = Draft202012Validator(cls.schema)
        cls.money_validator = Draft202012Validator(
            {"$ref": "#/$defs/Money", "$defs": cls.schema["$defs"]}
        )
        cls.assessor_validator = Draft202012Validator(
            {"$ref": "#/$defs/AssessorObservation", "$defs": cls.schema["$defs"]}
        )
        cls.models = _load_pydantic_module()

    def test_documented_assessor_statuses_are_accepted(self):
        for status in (
            "success",
            "not_found",
            "timeout",
            "api_error",
            "parse_error",
            "invalid_address",
            "ambiguous",
        ):
            payload = {"status": status, "provenance": {}}
            with self.subTest(status=status, validator="jsonschema"):
                self.assertEqual([], list(self.assessor_validator.iter_errors(payload)))
            with self.subTest(status=status, validator="pydantic"):
                self.models.AssessorObservation.model_validate(payload)

    def test_unknown_assessor_status_is_rejected(self):
        payload = {"status": "completely_made_up", "provenance": {}}
        self.assertTrue(list(self.assessor_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.AssessorObservation.model_validate(payload)

    def test_canonical_money_is_accepted_by_both_validators(self):
        payload = {"amount": "994250.00", "currency": "USD"}
        self.assertEqual([], list(self.money_validator.iter_errors(payload)))
        self.models.Money.model_validate(payload)

    def test_noncanonical_money_is_rejected_by_both_validators(self):
        payload = {"amount": "994,250.00", "currency": "USD"}
        self.assertTrue(list(self.money_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.Money.model_validate(payload)

    def test_unicode_digit_money_is_rejected_by_both_validators(self):
        # Python's \d matches Unicode decimal digits; ECMA's \d is ASCII-only.
        # Patterns must use [0-9] so every dialect rejects the same values.
        payload = {"amount": "١٢٣.٤٥", "currency": "USD"}
        self.assertTrue(list(self.money_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.Money.model_validate(payload)

    def test_trailing_newline_money_is_rejected_by_both_validators(self):
        # Python's re treats `$` as also matching before a trailing newline,
        # while ECMA regex (AJV et al.) does not — the patterns must reject
        # these in every dialect or clients diverge on the same document.
        for payload in (
            {"amount": "994250.00\n", "currency": "USD"},
            {"amount": "994250.00", "currency": "USD\n"},
        ):
            with self.subTest(payload=payload, validator="jsonschema"):
                self.assertTrue(list(self.money_validator.iter_errors(payload)))
            with self.subTest(payload=payload, validator="pydantic"):
                with self.assertRaises(ValidationError):
                    self.models.Money.model_validate(payload)

    def test_property_profile_fixtures_agree_across_validators(self):
        for path in sorted((ROOT / "examples").glob("*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=path.name, validator="jsonschema"):
                self.assertEqual([], list(self.profile_validator.iter_errors(payload)))
            with self.subTest(path=path.name, validator="pydantic"):
                self.models.PropertyProfile.model_validate(payload)

        for path in sorted((ROOT / "counter_examples").glob("*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=path.name, validator="jsonschema"):
                self.assertTrue(list(self.profile_validator.iter_errors(payload)))
            with self.subTest(path=path.name, validator="pydantic"):
                with self.assertRaises(ValidationError):
                    self.models.PropertyProfile.model_validate(payload)

    def test_every_enum_variant_has_rust_rename_and_round_trip_coverage(self):
        # Census: every permissible value of every vocabulary enum must have a
        # serde rename in the generated crate and an assertion in the Rust
        # wire-format test. Fails when a new enum/value lands without coverage.
        lib_rs = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(encoding="utf-8")
        wire_test = (GENERATED / "phds-rust" / "tests" / "wire_format.rs").read_text(
            encoding="utf-8"
        )
        enums = {
            name: d["enum"]
            for name, d in self.schema["$defs"].items()
            if d.get("type") == "string" and "enum" in d
        }
        self.assertGreaterEqual(len(enums), 23)
        for name, values in enums.items():
            for wire in values:
                variant = "".join(w.capitalize() for w in wire.split("_"))
                with self.subTest(enum=name, value=wire):
                    self.assertIn(f'serde(rename = "{wire}")', lib_rs)
                    self.assertIn(
                        f'assert_round_trip({name}::{variant}, "{wire}");', wire_test
                    )

    def test_typescript_uses_the_closed_assessor_status_enum(self):
        source = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        self.assertIn(
            "export interface AssessorObservation {\n    status: AssessorStatus,",
            source,
        )

    def test_rust_uses_canonical_snake_case_wire_values(self):
        source = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        # gen-rust --serde emits a per-variant rename for every enum; spot-check
        # values whose PascalCase variant name diverges from the wire value.
        for wire in ("not_found", "invalid_address", "llm_extraction", "pending_review"):
            self.assertIn(f'serde(rename = "{wire}")', source)
        self.assertTrue(
            (GENERATED / "phds-rust" / "tests" / "wire_format.rs").exists(),
            "Rust wire-format conformance test must exist outside src/",
        )


if __name__ == "__main__":
    unittest.main()
