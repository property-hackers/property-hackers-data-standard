"""Machine-readable index of cross-standard mapping annotations.

Schema slots carry SKOS-derived LinkML mapping annotations
(exact_mappings, close_mappings, related_mappings, ...) whose targets can
be terms from any external data standard — RESO, MISMO, UAD, or anything
else with a prefix declared in the schemas — written as CURIEs, e.g.
reso:ClosePrice. This module exposes those annotations as a queryable
index so importers and tests can route source fields to PHDS slots and
audit coverage. Adding a new standard is just a prefix declaration plus
annotations.
"""

from __future__ import annotations

from dataclasses import dataclass
from functools import lru_cache

from linkml_runtime.utils.schemaview import SchemaView

from tools.profile_validation import SCHEMA_PATH

MAPPING_STRENGTHS = ("exact", "close", "related", "narrow", "broad")


@dataclass(frozen=True, order=True)
class MappingEntry:
    host_class: str
    slot: str
    strength: str
    target: str


@lru_cache(maxsize=1)
def _schema_view() -> SchemaView:
    return SchemaView(str(SCHEMA_PATH))


@lru_cache(maxsize=1)
def mapping_entries() -> tuple[MappingEntry, ...]:
    """Every (class, slot, strength, target) mapping in the schema."""
    view = _schema_view()
    entries: set[MappingEntry] = set()
    for class_name in view.all_classes():
        for slot in view.class_induced_slots(class_name):
            for strength in MAPPING_STRENGTHS:
                for target in getattr(slot, f"{strength}_mappings", ()) or ():
                    entries.add(
                        MappingEntry(str(class_name), str(slot.name), strength, str(target))
                    )
    return tuple(sorted(entries))


def entries_for_prefix(prefix: str) -> tuple[MappingEntry, ...]:
    return tuple(e for e in mapping_entries() if e.target.split(":", 1)[0] == prefix)


def entries_for_target(target: str) -> tuple[MappingEntry, ...]:
    return tuple(e for e in mapping_entries() if e.target == target)
