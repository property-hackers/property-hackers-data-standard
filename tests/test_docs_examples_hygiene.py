import json
import pathlib
import unittest

import yaml


ROOT = pathlib.Path(__file__).resolve().parents[1]


class DocsAndExamplesHygieneTests(unittest.TestCase):
    def test_readme_documents_zod_generator_and_consumer_contract(self):
        source = (ROOT / "README.md").read_text()

        for required_text in (
            "Zod 4",
            "schema/generated/phds.zod.ts",
            "schema/generated/standards/uad_3_6.zod.ts",
            "schema/generated/standards/boma_building_class.metro.zod.ts",
            "schema/generated/standards/boma_building_class.international.zod.ts",
            "Node.js 24+",
            "npm ci",
            "just test-zod",
            'import { PropertyProfileSchema } from "./schema/generated/phds.zod";',
            "PropertyProfileSchema.safeParse(payload)",
            "result.error.issues",
            "## TypeScript and Zod example",
            "examples/zod/PropertyProfile-complex-residential-sale.json",
            "vendor/phds/phds.zod.ts",
            "Pin the PHDS release or commit",
            "Do not edit the generated file",
            "AI coding agent",
        ):
            with self.subTest(required_text=required_text):
                self.assertIn(required_text, source)

        self.assertLess(
            source.index("## TypeScript and Zod example"),
            source.index("## Design highlights"),
        )

        roadmap = source.split("## Roadmap", 1)[1].split("## ", 1)[0]
        self.assertNotIn("Zod generation", roadmap)
        check_line = next(
            line for line in source.splitlines() if line.startswith("just check ")
        )
        self.assertIn("Zod", check_line)

    def test_ci_installs_locked_node_toolchain(self):
        source = (ROOT / ".github/workflows/ci.yml").read_text()
        python_setup = source.index("actions/setup-python@")
        node_setup = source.index(
            "actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020"
        )
        just_venv = source.index("run: just venv")
        npm_install = source.index("run: npm ci")

        self.assertLess(python_setup, node_setup)
        self.assertIn('node-version: "24"', source)
        self.assertIn("cache: npm", source)
        self.assertLess(just_venv, npm_install)
        self.assertIn("name: Install locked Node toolchain", source)
        self.assertIn('test -z "$(git status --porcelain)"', source)

    def test_zod_toolchain_typechecks_every_runtime_suite(self):
        package = json.loads((ROOT / "package.json").read_text())
        config = json.loads((ROOT / "tsconfig.zod.json").read_text())

        self.assertIn("tests/zod_*.test.ts", package["scripts"]["test:zod:runtime"])
        self.assertIn("tests/zod_*.test.ts", config["include"])

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
