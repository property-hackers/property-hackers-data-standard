"""Generate TypeScript types that match PHDS's JSON wire contract.

Upstream linkml's TypeScriptGenerator emits every enum declaration but then
types enum-ranged slots as plain `string`: its range() has branches for
classes and types only. Override range() with the missing enum branch so
`Provenance.method` is `CaptureMethod`, `AssessorObservation.status` is
`AssessorStatus`, and so on. It also maps LinkML decimals to JSON numbers and
dates to ISO 8601 strings.
"""

from __future__ import annotations

import sys

from linkml.generators.typescriptgen import TypescriptGenerator
from linkml_runtime.linkml_model.meta import SlotDefinition


class EnumAwareTypescriptGenerator(TypescriptGenerator):
    def serialize(self, output=None) -> str:
        modified = False
        for cls in self.schemaview.all_classes().values():
            for slot_name in cls.slot_usage:
                if slot_name not in cls.slots:
                    cls.slots.append(slot_name)
                    modified = True
        if modified:
            self.schemaview.set_modified()
        generated = super().serialize(output)
        cleaned = "\n".join(
            line.rstrip() for line in generated.splitlines()
        ).rstrip("\n")
        return f"{cleaned}\n" if cleaned else ""

    def range(self, slot: SlotDefinition) -> str:
        # Upstream expands subproperty_of before considering range; keep that
        # precedence so the enum branch only handles what upstream would have
        # typed as plain `string`.
        if self.expand_subproperty_of and slot.subproperty_of:
            return super().range(slot)
        r = slot.range
        if r is not None and r in self.schemaview.all_enums():
            name = self.name(self.schemaview.get_enum(r))
            return f"{name}[]" if slot.multivalued else name
        if r is not None and r in self.schemaview.all_types():
            linkml_type = self.schemaview.get_type(r)
            if linkml_type.base == "Decimal":
                return "number[]" if slot.multivalued else "number"
            if linkml_type.base == "XSDDate":
                return "string[]" if slot.multivalued else "string"
        return super().range(slot)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <schema.yaml>")
    sys.stdout.write(EnumAwareTypescriptGenerator(sys.argv[1]).serialize())


if __name__ == "__main__":
    main()
