from __future__ import annotations

import argparse
from collections import Counter
from dataclasses import dataclass
from datetime import date
from functools import lru_cache
import json
from pathlib import Path
import sys
from typing import Any

from linkml_runtime.utils.schemaview import SchemaView


@dataclass(frozen=True, order=True)
class ValidationIssue:
    path: str
    message: str


@dataclass(frozen=True, order=True)
class ReferenceRule:
    host_class: str
    slot: str
    target_class: str
    multivalued: bool


SCHEMA_PATH = Path(__file__).resolve().parents[1] / "schema" / "capture.yaml"

# Every canonical target of an explicit, non-inlined LinkML reference exposed
# by PropertyProfile must have a bundle declared here. Undeclared reference
# ranges fail closed.
REFERENCE_TARGET_COLLECTIONS: dict[str, str] = {
    "Address": "addresses",
    "Jurisdiction": "jurisdictions",
    "LeaseEvent": "leases",
    "Loan": "loans",
    "OperatingStatement": "operating_statements",
    "Parcel": "parcels",
    "Party": "parties",
    "Property": "property",
    "PropertyStateSnapshot": "property_state_snapshots",
    "Site": "site",
    "SourceArtifact": "artifacts",
    "Space": "spaces",
    "Structure": "structures",
    "Transfer": "transfers",
}

REFERENCE_LABELS = {
    "OperatingStatement": "operating statement",
    "PropertyStateSnapshot": "property_state",
    "SourceArtifact": "artifact",
}

DUPLICATE_LABELS = {
    "PropertyStateSnapshot": "property_state_snapshot",
    "RentRoll": "rent_roll",
}

DATE_INTERVAL_RULES: dict[str, tuple[tuple[str, str], ...]] = {
    "PropertyParcel": (("started_on", "ended_on"),),
    "OwnershipPeriod": (("started_on", "ended_on"),),
    "LeaseEvent": (("commencement_date", "expiration_date"),),
    "LeaseEscalation": (("effective_from", "effective_until"),),
    "ForeclosureCase": (("opened_on", "resolved_on"),),
    "Permit": (
        ("applied_on", "issued_on"),
        ("applied_on", "finaled_on"),
        ("applied_on", "expires_on"),
        ("issued_on", "finaled_on"),
        ("issued_on", "expires_on"),
    ),
    "OperatingStatement": (("period_start", "period_end"),),
}

PAIRED_DATE_RULES: dict[str, tuple[tuple[str, str], ...]] = {
    "OperatingStatement": (("period_start", "period_end"),),
}


@lru_cache(maxsize=1)
def _schema_view() -> SchemaView:
    return SchemaView(str(SCHEMA_PATH))


@lru_cache(maxsize=1)
def reachable_reference_rules() -> tuple[ReferenceRule, ...]:
    """Discover references reachable through inlined profile shapes."""
    view = _schema_view()
    queue = ["PropertyProfile"]
    visited: set[str] = set()
    rules: set[ReferenceRule] = set()
    while queue:
        class_name = queue.pop(0)
        if class_name in visited:
            continue
        visited.add(class_name)
        for slot in view.class_induced_slots(class_name):
            if slot.range not in view.all_classes():
                continue
            if view.is_inlined(slot):
                queue.append(slot.range)
            else:
                rules.add(
                    ReferenceRule(
                        class_name,
                        slot.name,
                        slot.range,
                        bool(slot.multivalued),
                    )
                )
    return tuple(sorted(rules))


def reference_target_collections() -> dict[str, str]:
    return dict(REFERENCE_TARGET_COLLECTIONS)


def _typed_nodes(profile: dict[str, Any]):
    """Yield breadth-first `(path, class_name, object)` tuples for inlined data."""
    view = _schema_view()
    queue: list[tuple[str, str, dict[str, Any]]] = [("", "PropertyProfile", profile)]
    while queue:
        path, class_name, value = queue.pop(0)
        yield path, class_name, value
        slots = list(view.class_induced_slots(class_name))
        if class_name == "PropertyProfile":
            slots.sort(key=lambda slot: (slot.name != "property", slot.name))
        for slot in slots:
            if slot.range not in view.all_classes():
                continue
            if not view.is_inlined(slot):
                continue
            child = value.get(slot.name)
            child_path = f"{path}.{slot.name}" if path else slot.name
            if isinstance(child, dict):
                queue.append((child_path, slot.range, child))
            elif isinstance(child, list):
                for index, item in enumerate(child):
                    if isinstance(item, dict):
                        queue.append((f"{child_path}[{index}]", slot.range, item))


def _target_indexes(profile: dict[str, Any]) -> dict[str, Counter[str]]:
    indexes: dict[str, Counter[str]] = {}
    for target_class, collection in REFERENCE_TARGET_COLLECTIONS.items():
        value = profile.get(collection)
        if isinstance(value, dict):
            items = [value]
        else:
            items = _items(value)
        indexes[target_class] = Counter(
            item.get("id")
            for item in items
            if isinstance(item, dict) and item.get("id") is not None
        )
    return indexes


def _validate_entity_ids_and_references(
    profile: dict[str, Any], issues: list[ValidationIssue]
) -> dict[str, Counter[str]]:
    view = _schema_view()
    rules_by_host: dict[str, list[ReferenceRule]] = {}
    for rule in reachable_reference_rules():
        rules_by_host.setdefault(rule.host_class, []).append(rule)
    indexes = _target_indexes(profile)

    first_declaration_by_id: dict[str, tuple[str, str]] = {}
    for path, class_name, value in _typed_nodes(profile):
        if "Entity" in view.class_ancestors(class_name):
            identifier = value.get("id")
            if identifier is not None:
                id_path = f"{path}.id" if path else "id"
                first_path, first_class = first_declaration_by_id.setdefault(
                    identifier, (id_path, class_name)
                )
                if first_path != id_path:
                    if first_class == class_name:
                        label = DUPLICATE_LABELS.get(
                            class_name,
                            REFERENCE_LABELS.get(
                                class_name, class_name.replace("Event", "").lower()
                            ),
                        )
                        message = f"duplicate {label} id"
                    else:
                        message = (
                            f"duplicate Entity id; first declared at {first_path}"
                        )
                    issues.append(
                        ValidationIssue(
                            id_path,
                            message,
                        )
                    )

        for rule in rules_by_host.get(class_name, []):
            if rule.target_class not in indexes:
                raise RuntimeError(
                    f"no PropertyProfile target bundle for {rule.target_class} "
                    f"({rule.host_class}.{rule.slot})"
                )
            raw = value.get(rule.slot)
            references = raw if rule.multivalued and isinstance(raw, list) else [raw]
            for index, reference in enumerate(references):
                if reference is None:
                    continue
                reference_path = f"{path}.{rule.slot}" if path else rule.slot
                if rule.multivalued:
                    reference_path += f"[{index}]"
                if indexes[rule.target_class][reference] == 0:
                    label = REFERENCE_LABELS.get(
                        rule.target_class,
                        rule.target_class.replace("Event", "").lower(),
                    )
                    issues.append(
                        ValidationIssue(
                            reference_path,
                            f"{label} reference does not resolve",
                        )
                    )
    return indexes


def _date(value: str | None) -> date | None:
    return date.fromisoformat(value) if value else None


def _role_code(value: Any) -> str:
    if isinstance(value, dict):
        return str(value.get("code", ""))
    return str(value or "")


def _items(value: list[dict[str, Any]] | None) -> list[dict[str, Any]]:
    """Normalize optional LinkML collections after structural validation."""
    return value or []


def _overlaps(left: dict[str, Any], right: dict[str, Any]) -> bool:
    left_start, left_end = _date(left.get("valid_from")), _date(left.get("valid_to"))
    right_start, right_end = _date(right.get("valid_from")), _date(
        right.get("valid_to")
    )
    return (left_end is None or right_start is None or right_start <= left_end) and (
        right_end is None or left_start is None or left_start <= right_end
    )


def _validate_primary_address_overlaps(
    associations: list[dict[str, Any]],
    path_prefix: str,
    issues: list[ValidationIssue],
) -> None:
    primaries: dict[str, list[tuple[int, dict[str, Any]]]] = {}
    for index, association in enumerate(associations):
        if association.get("is_primary"):
            primaries.setdefault(_role_code(association.get("role")), []).append(
                (index, association)
            )

    for role, values in primaries.items():
        for left_index, left in values:
            for right_index, right in values:
                if left_index < right_index and _overlaps(left, right):
                    issues.append(
                        ValidationIssue(
                            f"{path_prefix}[{right_index}]",
                            f"primary address validity overlaps for role {role}",
                        )
                    )


def _validate_date_intervals(
    profile: dict[str, Any], issues: list[ValidationIssue]
) -> None:
    for path, class_name, value in _typed_nodes(profile):
        for start_field, end_field in DATE_INTERVAL_RULES.get(class_name, ()):
            start = _date(value.get(start_field))
            end = _date(value.get(end_field))
            if start is not None and end is not None and end < start:
                issues.append(
                    ValidationIssue(
                        path,
                        f"{end_field} precedes {start_field}",
                    )
                )
        for left_field, right_field in PAIRED_DATE_RULES.get(class_name, ()):
            if (value.get(left_field) is None) != (value.get(right_field) is None):
                issues.append(
                    ValidationIssue(
                        path,
                        f"{left_field} and {right_field} must be provided together",
                    )
                )


def _validate_structure_ratings(
    profile: dict[str, Any], issues: list[ValidationIssue]
) -> None:
    for path, class_name, value in _typed_nodes(profile):
        if class_name not in {"Structure", "StructureState"}:
            continue
        for field in ("condition_ratings", "quality_ratings"):
            seen: set[tuple[str | None, str | None]] = set()
            for index, rating in enumerate(_items(value.get(field))):
                key = (rating.get("system"), rating.get("scope"))
                if key in seen:
                    issues.append(
                        ValidationIssue(
                            f"{path}.{field}[{index}]",
                            f"duplicate rating system and scope in {field}",
                        )
                    )
                else:
                    seen.add(key)



def _validate_rent_rolls(
    profile: dict[str, Any],
    issues: list[ValidationIssue],
    space_ids: Counter[str],
    lease_ids: Counter[str],
) -> None:
    spaces_by_id = {
        item["id"]: item
        for item in _items(profile.get("spaces"))
        if item.get("id") is not None and space_ids[item["id"]] == 1
    }
    leases_by_id = {
        item["id"]: item
        for item in _items(profile.get("leases"))
        if item.get("id") is not None and lease_ids[item["id"]] == 1
    }
    money_fields = ("total_contract_rent", "total_market_rent")
    line_money_fields = ("contract_rent", "market_rent")

    for roll_index, roll in enumerate(_items(profile.get("rent_rolls"))):
        path = f"rent_rolls[{roll_index}]"
        if (
            roll.get("unit_count") is not None
            and roll.get("occupied_unit_count") is not None
            and roll["occupied_unit_count"] > roll["unit_count"]
        ):
            issues.append(
                ValidationIssue(path, "occupied_unit_count exceeds unit_count")
            )

        currencies: set[str] = set()
        for field in money_fields:
            value = roll.get(field)
            if isinstance(value, dict) and value.get("currency"):
                currencies.add(value["currency"])

        for line_index, line in enumerate(_items(roll.get("lines"))):
            line_path = f"{path}.lines[{line_index}]"
            for field in line_money_fields:
                value = line.get(field)
                if isinstance(value, dict) and value.get("currency"):
                    currencies.add(value["currency"])
            space_reference = line.get("space")
            lease_reference = line.get("lease")
            resolved_space = spaces_by_id.get(space_reference)
            resolved_lease = leases_by_id.get(lease_reference)
            tenant_reference = line.get("tenant")
            if tenant_reference is not None and resolved_lease is not None:
                lessees = {
                    relationship.get("party")
                    for relationship in _items(resolved_lease.get("parties"))
                    if _role_code(relationship.get("role")) == "lessee"
                    and relationship.get("party") is not None
                }
                if lessees and tenant_reference not in lessees:
                    issues.append(
                        ValidationIssue(
                            f"{line_path}.tenant",
                            "tenant does not match a lessee declared by the "
                            "referenced lease",
                        )
                    )
            if (
                resolved_space is not None
                and resolved_lease is not None
                and resolved_lease.get("space")
                and resolved_lease["space"] != space_reference
            ):
                issues.append(
                    ValidationIssue(
                        f"{line_path}.lease",
                        "lease space does not match rent roll line space",
                    )
                )

        if len(currencies) > 1:
            issues.append(
                ValidationIssue(
                    path,
                    "all rent roll money must use one currency",
                )
            )


def _validate_artifacts(
    profile: dict[str, Any], issues: list[ValidationIssue]
) -> None:
    artifacts = _items(profile.get("artifacts"))
    for index, artifact in enumerate(artifacts):
        if not artifact.get("uri") and not artifact.get("storage_reference"):
            issues.append(
                ValidationIssue(
                    f"artifacts[{index}]",
                    "artifact requires uri or storage_reference",
                )
            )
        has_hash = bool(artifact.get("content_hash"))
        has_scheme = bool(artifact.get("hash_scheme"))
        if has_hash != has_scheme:
            issues.append(
                ValidationIssue(
                    f"artifacts[{index}]",
                    "content_hash requires hash_scheme",
                )
            )

def validate_capture_envelope(envelope: dict[str, Any]) -> list[ValidationIssue]:
    """Validate capture semantics over one optional nested PropertyProfile."""
    issues: list[ValidationIssue] = []
    profile = envelope.get("profile")
    artifact_ids: Counter[str] = Counter()
    if isinstance(profile, dict):
        issues.extend(
            ValidationIssue(f"profile.{issue.path}", issue.message)
            for issue in validate_profile(profile)
        )
        artifact_ids = Counter(
            artifact.get("id")
            for artifact in _items(profile.get("artifacts"))
            if artifact.get("id") is not None
        )

    for index, reference in enumerate(_items(envelope.get("artifact_refs"))):
        if artifact_ids[reference] == 0:
            issues.append(
                ValidationIssue(
                    f"artifact_refs[{index}]",
                    "artifact reference does not resolve against profile.artifacts",
                )
            )
    return sorted(issues)


def validate_profile(profile: dict[str, Any]) -> list[ValidationIssue]:
    """Validate aggregate invariants on a structurally valid PropertyProfile.

    Callers must first validate field types and required fields with a generated
    PHDS contract. Optional LinkML collections may be absent or explicitly
    null; both representations are accepted here.
    """
    issues: list[ValidationIssue] = []
    indexes = _validate_entity_ids_and_references(profile, issues)
    addresses = _items(profile.get("addresses"))
    parties = _items(profile.get("parties"))
    for index, party in enumerate(parties):
        if party.get("legal_form") is not None and party.get("kind") != "organization":
            issues.append(
                ValidationIssue(
                    f"parties[{index}].legal_form",
                    "legal_form is only valid for an organization Party",
                )
            )

    for index, address in enumerate(addresses):
        has_hash = bool(address.get("address_hash"))
        has_scheme = bool(address.get("address_hash_scheme"))
        if has_hash != has_scheme:
            issues.append(
                ValidationIssue(
                    f"addresses[{index}]",
                    "address_hash requires address_hash_scheme",
                )
            )

    property_addresses = _items(profile.get("property_addresses"))
    for index, association in enumerate(property_addresses):
        path = f"property_addresses[{index}]"
        if (
            association.get("valid_from")
            and association.get("valid_to")
            and _date(association["valid_to"]) < _date(association["valid_from"])
        ):
            issues.append(ValidationIssue(path, "valid_to precedes valid_from"))
    _validate_primary_address_overlaps(property_addresses, "property_addresses", issues)

    for party_index, party in enumerate(parties):
        party_addresses = _items(party.get("addresses"))
        for address_index, association in enumerate(party_addresses):
            if (
                association.get("valid_from")
                and association.get("valid_to")
                and _date(association["valid_to"]) < _date(association["valid_from"])
            ):
                issues.append(
                    ValidationIssue(
                        f"parties[{party_index}].addresses[{address_index}]",
                        "valid_to precedes valid_from",
                    )
                )
        _validate_primary_address_overlaps(
            party_addresses,
            f"parties[{party_index}].addresses",
            issues,
        )

    _validate_date_intervals(profile, issues)
    _validate_structure_ratings(profile, issues)
    _validate_rent_rolls(profile, issues, indexes["Space"], indexes["LeaseEvent"])
    _validate_artifacts(profile, issues)
    return sorted(issues)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--document-type",
        choices=("property-profile", "capture-envelope"),
        default="property-profile",
    )
    parser.add_argument("profile", type=Path)
    args = parser.parse_args()
    try:
        profile = json.loads(args.profile.read_text(encoding="utf-8"))
    except OSError as error:
        print(f"profile: cannot read JSON: {error}", file=sys.stderr)
        return 1
    except json.JSONDecodeError as error:
        print(f"profile: invalid JSON: {error}", file=sys.stderr)
        return 1
    if not isinstance(profile, dict):
        print("profile: profile root must be a JSON object", file=sys.stderr)
        return 1
    if args.document_type == "capture-envelope":
        issues = validate_capture_envelope(profile)
    else:
        issues = validate_profile(profile)
    for issue in issues:
        print(f"{issue.path}: {issue.message}", file=sys.stderr)
    return 1 if issues else 0


if __name__ == "__main__":
    raise SystemExit(main())
