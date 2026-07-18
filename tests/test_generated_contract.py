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
        self.assertIn(
            '#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]\n'
            "pub enum AssessorStatus {",
            source,
        )
        self.assertIn("fn canonical_wire_values_round_trip()", source)


if __name__ == "__main__":
    unittest.main()
