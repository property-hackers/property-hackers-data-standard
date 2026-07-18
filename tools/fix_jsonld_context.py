"""Repair JSON-LD terms whose JSON key has both scalar and object meanings.

LinkML's context generator has one global term per JSON key. PHDS uses
``amount`` both for the exact decimal string inside Money/UnitRate and for
Money-valued entity attributes. Property-scoped JSON-LD 1.1 contexts preserve
both meanings for ordinary, untyped PropertyProfile documents.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from linkml_runtime.utils.schemaview import SchemaView


def repair_context(schema_path: Path, context_path: Path) -> None:
    view = SchemaView(str(schema_path))
    document: dict[str, Any] = json.loads(context_path.read_text(encoding="utf-8"))
    context = document["@context"]

    decimal_holders = set()
    for class_name in view.all_classes():
        for slot in view.class_induced_slots(class_name):
            if slot.name == "amount" and str(slot.slot_uri).endswith("decimal_amount"):
                decimal_holders.add(class_name)

    scoped_slots = set()
    for class_name in view.all_classes():
        for slot in view.class_induced_slots(class_name):
            if slot.range in decimal_holders:
                scoped_slots.add(slot.name)

    if not decimal_holders or not scoped_slots:
        raise RuntimeError("schema exposes no decimal-amount value objects")

    scalar_amount_term = {"@id": "decimal_amount"}
    for slot_name in sorted(scoped_slots):
        existing = context.get(slot_name, {"@id": slot_name})
        term = {"@id": existing} if isinstance(existing, str) else dict(existing)
        term["@context"] = {"amount": scalar_amount_term}
        context[slot_name] = term

    context_path.write_text(
        json.dumps(document, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("schema", type=Path)
    parser.add_argument("context", type=Path)
    args = parser.parse_args()
    repair_context(args.schema, args.context)


if __name__ == "__main__":
    main()
