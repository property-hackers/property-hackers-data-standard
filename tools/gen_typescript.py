"""gen-typescript wrapper that types enum-ranged slots with their enums.

Upstream linkml's TypeScriptGenerator emits every enum declaration but then
types enum-ranged slots as plain `string`: its range() has branches for
classes and types only. Override range() with the missing enum branch so
`Provenance.method` is `CaptureMethod`, `AssessorObservation.status` is
`AssessorStatus`, and so on. Drop this wrapper when fixed upstream.
"""

from __future__ import annotations

import sys

from linkml.generators.typescriptgen import TypescriptGenerator
from linkml_runtime.linkml_model.meta import SlotDefinition


class EnumAwareTypescriptGenerator(TypescriptGenerator):
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
        return super().range(slot)


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <schema.yaml>")
    sys.stdout.write(EnumAwareTypescriptGenerator(sys.argv[1]).serialize())


if __name__ == "__main__":
    main()
