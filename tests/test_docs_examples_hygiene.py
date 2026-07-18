import json
import pathlib
import unittest

import yaml


ROOT = pathlib.Path(__file__).resolve().parents[1]


class DocsAndExamplesHygieneTests(unittest.TestCase):
    def test_uad_crosswalk_describes_current_contract(self):
        source = (ROOT / "docs/crosswalks/uad36-alignment.md").read_text()
        normalized = " ".join(source.split()).casefold()

        for current_term in (
            "phds v0.2",
            "system-qualified `rating`",
            "propertystatesnapshot.as_of_date",
            "optional uad profile",
            "internationally neutral",
        ):
            self.assertIn(current_term, normalized)

        for heading in (
            "mapping model",
            "physical-property mappings",
            "condition and quality ratings",
            "records, events, and valuation",
        ):
            self.assertIn(heading, normalized)

    def test_core_area_and_detail_descriptions_are_standards_neutral(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        classes = entities["classes"]
        facts = classes["StructureFacts"]["attributes"]

        self.assertIn("finished", facts["living_area"]["description"].casefold())
        self.assertNotIn("uad", facts["living_area"]["description"].casefold())
        self.assertIn("occupiable", facts["rentable_area"]["description"].casefold())
        self.assertNotIn("boma", facts["rentable_area"]["description"].casefold())

        for detail in ("ResidentialDetails", "CommercialDetails"):
            description = classes[detail]["description"].casefold()
            self.assertIn("internationally neutral", description)
            self.assertNotIn("boma/oscre-cited semantics", description)

    def test_example_extras_keys_are_namespaced(self):
        for path in sorted((ROOT / "examples").rglob("*.json")):
            payload = json.loads(path.read_text())

            def visit(value):
                if isinstance(value, dict):
                    extras = value.get("extras")
                    if isinstance(extras, dict):
                        for key in extras:
                            with self.subTest(path=path.name, key=key):
                                self.assertRegex(
                                    key,
                                    r"^[a-z0-9][a-z0-9_-]*(?:\.[a-z0-9][a-z0-9_-]*)+$",
                                )
                    for child in value.values():
                        visit(child)
                elif isinstance(value, list):
                    for child in value:
                        visit(child)

            visit(payload)

    def test_flagship_examples_use_synthetic_natural_person_names(self):
        for path in sorted((ROOT / "examples").glob("*.json")):
            payload = json.loads(path.read_text())
            names = [
                party.get("name", "")
                for party in payload.get("parties", [])
                if party.get("kind") == "person"
            ]
            for name in names:
                with self.subTest(path=path.name, name=name):
                    self.assertTrue(name.startswith("Example "))

            person_ids = {
                party["id"]
                for party in payload.get("parties", [])
                if party.get("kind") == "person"
            }
            for party_id in person_ids:
                with self.subTest(path=path.name, party_id=party_id):
                    self.assertTrue(party_id.startswith("party-example-"))
            for ownership in payload.get("ownership", []):
                represented_parties = {
                    interest.get("party")
                    for interest in ownership.get("interests", [])
                }
                if represented_parties & person_ids:
                    with self.subTest(path=path.name, ownership=ownership["id"]):
                        self.assertTrue(
                            ownership["id"].startswith("ownership-example-")
                        )

    def test_money_counterexamples_are_minimal(self):
        names = (
            "PropertyProfile-bad-money-amount.json",
            "PropertyProfile-money-trailing-newline.json",
            "PropertyProfile-money-unicode-digits.json",
        )
        for name in names:
            path = ROOT / "counter_examples/schema" / name
            payload = json.loads(path.read_text())
            with self.subTest(path=name):
                self.assertLessEqual(len(path.read_text().splitlines()), 35)
                self.assertEqual(
                    {"property", "jurisdictions", "parcels", "assessments"},
                    set(payload),
                )


if __name__ == "__main__":
    unittest.main()
