from __future__ import annotations

import json
from decimal import Decimal
from pathlib import Path
import unittest

from pydantic_core import PydanticSerializationError
import yaml

from tests.test_generated_contract import _load_pydantic_module


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema/generated"
PHYSICAL_STATE_EXAMPLE = ROOT / "examples/PropertyProfile-physical-state.json"


class ProfileRoundTripTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.models = _load_pydantic_module()

    def test_every_example_round_trips_without_material_loss(self):
        for path in sorted((ROOT / "examples").glob("PropertyProfile-*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            model = self.models.PropertyProfile.model_validate(payload)
            serialized = model.model_dump(
                mode="json", exclude_unset=True, exclude_none=True
            )
            with self.subTest(path=path.name):
                self.assertEqual(payload, serialized)

    def test_round_trip_examples_exercise_every_profile_collection(self):
        profile = yaml.safe_load((ROOT / "schema/profiles.yaml").read_text())[
            "classes"
        ]["PropertyProfile"]
        expected_collections = {
            name
            for name, slot in profile["attributes"].items()
            if isinstance(slot, dict) and slot.get("multivalued") is True
        }
        exercised_collections = {
            name
            for path in sorted((ROOT / "examples").glob("PropertyProfile-*.json"))
            for name in json.loads(path.read_text(encoding="utf-8"))
        }

        self.assertEqual(set(), expected_collections - exercised_collections)

    def test_omitted_business_classifications_remain_unasserted(self):
        cases = (
            (
                self.models.SaleEvent,
                {"id": "sale-1", "property": "property-1", "close_date": "2026-01-01"},
                ("sale_type",),
            ),
            (
                self.models.UnitRentObservation,
                {
                    "id": "rent-1",
                    "property": "property-1",
                    "unit_type": "studio",
                    "rate": {"amount": "1000", "currency": "CAD"},
                    "observed_date": "2026-01-01",
                },
                ("rate_period", "rate_basis", "rate_type"),
            ),
            (
                self.models.OperatingStatement,
                {
                    "id": "statement-1",
                    "property": "property-1",
                    "statement_year": 2026,
                },
                ("statement_basis",),
            ),
        )

        for model_type, payload, omitted_fields in cases:
            with self.subTest(model=model_type.__name__):
                model = model_type.model_validate(payload)
                serialized = model.model_dump(mode="json", exclude_none=True)
                for field in omitted_fields:
                    self.assertIsNone(getattr(model, field))
                    self.assertNotIn(field, serialized)

    def test_sparse_state_omissions_and_references_survive_round_trip(self):
        payload = json.loads(PHYSICAL_STATE_EXAMPLE.read_text(encoding="utf-8"))
        model = self.models.PropertyProfile.model_validate(payload)
        serialized = model.model_dump(
            mode="json", exclude_unset=True, exclude_none=True
        )

        snapshots = serialized["property_state_snapshots"]
        first_state = snapshots[0]["structure_states"][0]
        second_state = snapshots[1]["structure_states"][0]

        self.assertNotIn("condition_ratings", first_state)
        self.assertNotIn("gross_area", second_state)
        self.assertEqual(
            first_state["subject"], serialized["structures"][0]["id"]
        )
        self.assertEqual(serialized["sales"][0]["property_state"], snapshots[0]["id"])
        self.assertNotEqual(first_state["id"], second_state["id"])

    def test_integral_decimals_remain_json_integers_and_python_decimals(self):
        model = self.models.Area(value=Decimal("12500"), unit="sqft")

        self.assertEqual(model.model_dump(mode="json")["value"], 12500)
        self.assertIs(type(model.model_dump(mode="json")["value"]), int)

        point = self.models.GeoPoint(
            latitude=Decimal("45"), longitude=Decimal("-90")
        )
        python_values = point.model_dump(mode="python")
        self.assertEqual(python_values["latitude"], Decimal("45"))
        self.assertEqual(python_values["longitude"], Decimal("-90"))
        self.assertIs(type(python_values["latitude"]), Decimal)
        self.assertIs(type(python_values["longitude"]), Decimal)

    def test_representative_fixture_decimals_remain_exact_json_numbers(self):
        point = self.models.GeoPoint(
            latitude=Decimal("39.011886133826"),
            longitude=Decimal("-94.6633802"),
        )
        serialized = point.model_dump(mode="json")

        self.assertEqual(
            Decimal(str(serialized["latitude"])), Decimal("39.011886133826")
        )
        self.assertEqual(
            Decimal(str(serialized["longitude"])), Decimal("-94.6633802")
        )
        self.assertIs(type(serialized["latitude"]), float)
        self.assertIs(type(serialized["longitude"]), float)

    def test_high_precision_nonintegral_decimal_fails_loudly(self):
        model = self.models.Area(
            value=Decimal("0.12345678901234567890123456789"), unit="sqft"
        )

        with self.assertRaisesRegex(
            PydanticSerializationError, "cannot be represented exactly as a JSON number"
        ):
            model.model_dump(mode="json")

    def test_extreme_finite_integral_decimal_never_becomes_infinity_or_null(self):
        value = Decimal("1E+999")
        model = self.models.Area(value=value, unit="sqft")
        serialized = model.model_dump(mode="json")["value"]
        wire = model.model_dump_json()

        self.assertIs(type(serialized), int)
        self.assertEqual(Decimal(serialized), value)
        self.assertNotIn("Infinity", wire)
        self.assertNotIn("null", wire)
        self.assertEqual(Decimal(json.loads(wire)["value"]), value)

    def test_jsonld_expanded_graph_preserves_identity_edges_and_literals(self):
        from pyld import jsonld

        # The generated context is an expanded-graph identity conformance
        # surface. Exact profile-shape round trips are covered by Pydantic JSON
        # above; JSON-LD flattening intentionally produces graph nodes.
        payload = json.loads(PHYSICAL_STATE_EXAMPLE.read_text(encoding="utf-8"))
        context = json.loads(
            (GENERATED / "phds.context.jsonld").read_text(encoding="utf-8")
        )
        flattened = jsonld.flatten({"@context": context["@context"], **payload})
        nodes = flattened.get("@graph", []) if isinstance(flattened, dict) else flattened
        ids = [node.get("@id") for node in nodes if node.get("@id")]

        expected_ids = {
            payload["structures"][0]["id"],
            payload["sales"][0]["id"],
            *(snapshot["id"] for snapshot in payload["property_state_snapshots"]),
            *(
                state["id"]
                for snapshot in payload["property_state_snapshots"]
                for state in snapshot["structure_states"]
            ),
        }
        self.assertEqual(len(ids), len(set(ids)))
        self.assertTrue(expected_ids.issubset(ids))

        nodes_by_id = {node["@id"]: node for node in nodes if node.get("@id")}
        vocabulary = context["@context"]["@vocab"]
        structure_id = payload["structures"][0]["id"]
        first_snapshot = payload["property_state_snapshots"][0]
        first_state = first_snapshot["structure_states"][0]
        second_state = payload["property_state_snapshots"][1]["structure_states"][0]
        sale = payload["sales"][0]
        subject_predicate = f"{vocabulary}subject"
        property_state_predicate = f"{vocabulary}property_state"
        gross_area_predicate = f"{vocabulary}gross_area"
        condition_ratings_predicate = f"{vocabulary}condition_ratings"
        value_predicate = f"{vocabulary}value"
        system_predicate = f"{vocabulary}system"
        code_predicate = f"{vocabulary}code"
        self.assertEqual(
            nodes_by_id[first_state["id"]][subject_predicate],
            [{"@id": structure_id}],
        )
        self.assertEqual(
            nodes_by_id[sale["id"]][property_state_predicate],
            [{"@id": first_snapshot["id"]}],
        )
        self.assertNotIn(condition_ratings_predicate, nodes_by_id[first_state["id"]])
        self.assertNotIn(gross_area_predicate, nodes_by_id[second_state["id"]])

        current_area_id = nodes_by_id[structure_id][gross_area_predicate][0]["@id"]
        historical_area_id = nodes_by_id[first_state["id"]][gross_area_predicate][0][
            "@id"
        ]
        current_value = nodes_by_id[current_area_id][value_predicate][0]["@value"]
        historical_value = nodes_by_id[historical_area_id][value_predicate][0][
            "@value"
        ]
        self.assertEqual(current_value, 12500)
        self.assertEqual(historical_value, 12000)
        self.assertIs(type(current_value), int)
        self.assertIs(type(historical_value), int)

        rating_id = nodes_by_id[second_state["id"]][condition_ratings_predicate][0][
            "@id"
        ]
        self.assertEqual(
            nodes_by_id[rating_id][system_predicate],
            [{"@value": "https://example.org/condition-rating"}],
        )
        self.assertEqual(
            nodes_by_id[rating_id][code_predicate], [{"@value": "good"}]
        )

    def test_jsonld_money_and_unit_rate_amounts_expand_as_decimal_literals(self):
        from pyld import jsonld

        payload = json.loads(
            (ROOT / "examples/PropertyProfile-amazon-warehouse-orl.json").read_text(
                encoding="utf-8"
            )
        )
        context = json.loads(
            (GENERATED / "phds.context.jsonld").read_text(encoding="utf-8")
        )
        expanded = jsonld.expand({"@context": context["@context"], **payload})
        vocabulary = context["@context"]["@vocab"]
        decimal_predicate = f"{vocabulary}decimal_amount"
        outer_amount_predicate = f"{vocabulary}amount"
        currency_predicate = f"{vocabulary}currency"

        decimal_values = []
        outer_amount_values = []
        currency_values = []

        def visit(value):
            if isinstance(value, dict):
                decimal_values.extend(value.get(decimal_predicate, []))
                outer_amount_values.extend(value.get(outer_amount_predicate, []))
                currency_values.extend(value.get(currency_predicate, []))
                for child in value.values():
                    visit(child)
            elif isinstance(value, list):
                for child in value:
                    visit(child)

        visit(expanded)
        self.assertTrue(decimal_values, "shipped profile must expose decimal amounts")
        self.assertTrue(outer_amount_values, "Money-valued amount relations must remain")
        self.assertTrue(currency_values, "shipped profile must expose currencies")
        self.assertTrue(all("@value" in value for value in decimal_values))
        self.assertTrue(all("@id" not in value for value in decimal_values))
        self.assertTrue(all("@value" in value for value in currency_values))


if __name__ == "__main__":
    unittest.main()
