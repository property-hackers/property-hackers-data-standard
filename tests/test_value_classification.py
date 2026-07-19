"""value_classification annotations must survive in the LinkML source."""
from __future__ import annotations

import unittest
from pathlib import Path

from linkml_runtime.utils.schemaview import SchemaView

SCHEMA = Path(__file__).resolve().parents[1] / "schema" / "capture.yaml"

EXPECTED = {
    ("SaleEvent", "sale_price"): "reconciled",
    ("SaleEvent", "price_per_area"): "derived",
    ("SaleEvent", "price_per_unit"): "derived",
    ("SaleEvent", "cap_rate"): "derived",
    ("SaleEvidence", "close_date"): "asserted",
    ("SaleEvidence", "close_price"): "asserted",
    ("SaleEvidence", "concessions_amount"): "asserted",
}


class ValueClassification(unittest.TestCase):
    def test_annotations_present(self):
        view = SchemaView(str(SCHEMA))
        for (class_name, slot_name), expected in EXPECTED.items():
            slot = view.induced_slot(slot_name, class_name)
            annotation = slot.annotations._get("value_classification")
            self.assertIsNotNone(
                annotation, f"{class_name}.{slot_name} missing value_classification"
            )
            self.assertEqual(annotation.value, expected, f"{class_name}.{slot_name}")


if __name__ == "__main__":
    unittest.main()
