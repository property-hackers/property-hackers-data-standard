"""Naming-convention census: *_date = date, *_at = datetime, no *_on / *_kind."""
from __future__ import annotations

import unittest
from pathlib import Path

import yaml

SCHEMA_DIR = Path(__file__).resolve().parents[1] / "schema"
SOURCES = [
    SCHEMA_DIR / name
    for name in ("core.yaml", "entities.yaml", "profiles.yaml", "capture.yaml")
] + sorted((SCHEMA_DIR / "standards").glob("*.yaml"))


def _iter_attributes():
    for source in SOURCES:
        doc = yaml.safe_load(source.read_text(encoding="utf-8"))
        for class_name, definition in (doc.get("classes") or {}).items():
            for slot_name, slot in ((definition or {}).get("attributes") or {}).items():
                yield source.name, class_name, slot_name, (slot or {})
        for slot_name, slot in (doc.get("slots") or {}).items():
            yield source.name, "(top-level slots)", slot_name, (slot or {})


def _iter_enums():
    for source in SOURCES:
        doc = yaml.safe_load(source.read_text(encoding="utf-8"))
        for enum_name in doc.get("enums") or {}:
            yield source.name, enum_name


class NamingCensus(unittest.TestCase):
    def test_no_on_suffixed_slots(self):
        offenders = [
            f"{f}:{c}.{s}" for f, c, s, _ in _iter_attributes() if s.endswith("_on")
        ]
        self.assertEqual(offenders, [])

    def test_no_kind_suffixed_slots(self):
        offenders = [
            f"{f}:{c}.{s}" for f, c, s, _ in _iter_attributes() if s.endswith("_kind")
        ]
        self.assertEqual(offenders, [])

    def test_no_kind_suffixed_enums(self):
        offenders = [f"{f}:{e}" for f, e in _iter_enums() if e.endswith("Kind")]
        self.assertEqual(offenders, [])

    def test_date_slots_have_date_range(self):
        for f, c, s, slot in _iter_attributes():
            if s.endswith("_date"):
                self.assertEqual(
                    slot.get("range"), "date", f"{f}:{c}.{s} must be range: date"
                )

    def test_at_slots_have_datetime_range(self):
        for f, c, s, slot in _iter_attributes():
            if s.endswith("_at"):
                self.assertEqual(
                    slot.get("range"), "datetime", f"{f}:{c}.{s} must be range: datetime"
                )


if __name__ == "__main__":
    unittest.main()
