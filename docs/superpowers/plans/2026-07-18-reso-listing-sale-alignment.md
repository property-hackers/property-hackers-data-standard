# RESO DD 2.0 Listing & Sale Alignment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Align PHDS's listing and sale model with RESO Data Dictionary 2.0 per `docs/superpowers/specs/2026-07-18-reso-listing-sale-alignment-design.md`: package-wide naming conventions, listing identity/lifecycle, SaleEvidence, media/provenance extensions, MlsObservation envelope, crosswalk doc, fixtures.

**Architecture:** LinkML source of truth in `schema/*.yaml`; all artifacts (JSON Schema, Pydantic, TypeScript, Rust, Elixir wire tests) regenerate via `just gen`. Semantic (cross-record) validation lives in `tools/profile_validation.py`. Positive fixtures in `examples/`, negatives in `counter_examples/schema|semantic/` (each failing exactly one layer).

**Tech Stack:** LinkML, Python (unittest), Rust/serde (generated), just.

## Global Constraints

- **NEVER run `git commit` or `git push`.** The user requires explicit permission in the moment for every commit. End each task by reporting what's ready and a suggested commit message; leave changes in the working tree. This overrides the commit steps normally used by execution skills.
- Schema versions stay `0.2.0` (already bumped in the working tree).
- The nonblank-string pattern used throughout the schema is the exact regex on `Entity.id` in `schema/entities.yaml` (starts `^[^	-...`). Copy it verbatim wherever "NONBLANK" appears below; never retype it.
- Wire format: Money is `{"amount": "<decimal-as-string>", "currency": "<ISO-4217>"}`; CodeableConcept is `{"system": ..., "code": ..., "display": ...}` (all optional strings).
- After any schema change: `just gen` before `just validate` / `just test-generated`. Run `just check` (includes `check-generated` determinism check and `test-rust`) at the end of every task. If `.venv` is missing, run `just venv` first.
- Semantic counter-examples must pass `linkml-validate` and fail `tools/profile_validation.py`; schema counter-examples must fail `linkml-validate`. The `just validate` recipe enforces this automatically.
- Do not edit anything under `schema/generated/` by hand; do not rewrite historical documents under `docs/superpowers/` (this plan and the spec excepted).
- RESO mapping annotations use the existing `reso:` prefix (`https://ddwiki.reso.org/display/DDW20/`).

---

## Normative rename table (Task 1 executes this; later tasks assume it done)

Slot renames (wire-visible — JSON fixtures, tests, docs all follow):

| Class | Old | New |
|---|---|---|
| RecordedInstrument | `recorded_on` | `recorded_date` |
| Parcel | `retired_on` | `retired_date` |
| PropertyParcel | `started_on` / `ended_on` | `start_date` / `end_date` |
| ParcelLineage | `effective_on` | `effective_date` |
| SourceArtifact | `captured_on` | `captured_at` (already `range: datetime` — name/type fix) |
| OwnershipPeriod | `started_on` / `ended_on` | `start_date` / `end_date` |
| Renovation | `completed_on` | `completed_date` |
| TaxInstallment | `due_on` / `paid_on` | `due_date` / `paid_date` |
| Transfer | `effective_on` | `effective_date` |
| Transfer | `transfer_kind` | `transfer_type` |
| Listing | `kind` | `offering_type` |
| ListingEvent | `occurred_on` | `effective_date` |
| ListingEvent | `event_kind` | `event_type` |
| UnitRentObservation | `observed_on` | `observed_date` |
| LoanEvent | `occurred_on` | `effective_date` |
| LoanEvent | `event_kind` | `event_type` |
| Lien | `released_on` | `released_date` |
| ForeclosureCase | `opened_on` / `resolved_on` | `opened_date` / `resolved_date` |
| ForeclosureFiling | `auction_on` | `auction_date` |
| Permit | `applied_on` / `issued_on` / `finaled_on` / `expires_on` | `applied_date` / `issued_date` / `finaled_date` / `expiration_date` |

Enum renames (type names only — permissible values unchanged, so no wire change):

| Old | New |
|---|---|
| `PartyKind` | `PartyType` |
| `ListingKind` | `OfferingType` |
| `ValuationKind` | `ValuationType` |
| `LoanEventKind` | `LoanEventType` |
| `LienKind` | `LienType` |
| `ParcelLineageKind` | `ParcelLineageType` |

Explicit keeps (census-exempt): every bare `kind` slot other than `Listing.kind` keeps its name — `Jurisdiction.kind`, `ParcelLineage.kind`, `Party.kind`, `SourceArtifact.kind`, `PartyContact.kind`, the other-structure `kind`, `AreaMeasure.kind`, `Renovation.kind`, `TaxExemption.kind`, `Lien.kind`, `Permit.kind`, `Valuation.kind`. Also kept: `valid_from`, `valid_to`, and all existing `*_date` / `*_at` slots.

---

### Task 1: Package-wide rename wave + naming census test

**Files:**
- Create: `tests/test_naming_conventions.py`
- Modify: `schema/entities.yaml`, `schema/core.yaml` (enum names), plus every non-generated file that mentions an old name: `examples/*.json`, `examples/standards/*.json`, `counter_examples/**/*.json`, `tests/*.py`, `tests/fixtures/**`, `tools/*.py` (only if they hard-code slot names), `docs/crosswalks/uad36-alignment.md`, `docs/semantics/*.md`, `README.md`
- Regenerate: everything under `schema/generated/` via `just gen`

**Interfaces:**
- Consumes: nothing (first task).
- Produces: the renamed schema slots/enums per the normative table above. Every later task uses the NEW names (`effective_date`, `event_type`, `offering_type`, `OfferingType`, `captured_at`, `transfer_type`, …).

- [x] **Step 1: Write the failing census test**

Create `tests/test_naming_conventions.py`:

```python
"""Naming-convention census: *_date = date, *_at = datetime, no *_on / *_kind."""
from __future__ import annotations

import unittest
from pathlib import Path

import yaml

SCHEMA_DIR = Path(__file__).resolve().parents[1] / "schema"
SOURCES = [
    SCHEMA_DIR / name
    for name in ("core.yaml", "entities.yaml", "profiles.yaml", "capture.yaml")
]


def _iter_attributes():
    for source in SOURCES:
        doc = yaml.safe_load(source.read_text(encoding="utf-8"))
        for class_name, definition in (doc.get("classes") or {}).items():
            for slot_name, slot in ((definition or {}).get("attributes") or {}).items():
                yield source.name, class_name, slot_name, (slot or {})


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
```

Note: `slots: [amount, currency]`-style slot usages (top-level `slots:` section in `core.yaml`) are not attributes; if `core.yaml` defines any top-level `slots:` entries ending in `_on`/`_date`/`_at`, extend `_iter_attributes` to also yield them from the document's `slots:` mapping.

- [x] **Step 2: Run it to verify it fails**

Run: `.venv/bin/python -m unittest tests.test_naming_conventions -v`
Expected: FAIL — `test_no_on_suffixed_slots` lists ~23 offenders (`recorded_on`, `retired_on`, …), `test_no_kind_suffixed_slots` lists 3, `test_no_kind_suffixed_enums` lists 6.

- [x] **Step 3: Apply the schema renames**

Edit `schema/entities.yaml` applying the normative table exactly (slot keys AND any in-description cross-references, e.g. Transfer.effective_date's description mentions `recorded_on` → `recorded_date`). Edit `schema/core.yaml` renaming the six enums; update every `range: <OldKind>` in `entities.yaml` to the new enum name. Do NOT change permissible values. `Listing.kind` becomes `offering_type: {range: OfferingType, required: true}`.

- [x] **Step 4: Sweep non-schema files**

For each old name in the slot-rename table, find remaining references (generated output excluded — it gets regenerated):

```bash
for name in recorded_on retired_on started_on ended_on effective_on captured_on \
    completed_on due_on paid_on occurred_on observed_on released_on opened_on \
    resolved_on auction_on applied_on issued_on finaled_on expires_on \
    transfer_kind event_kind PartyKind ListingKind ValuationKind LoanEventKind \
    LienKind ParcelLineageKind; do
  echo "== $name"; grep -rln "$name" --include='*.json' --include='*.py' --include='*.md' \
    --include='*.yaml' . | grep -v -e '^\./schema/generated/' -e '^\./docs/superpowers/' -e '^\./\.venv'
done
```

Fix every hit with the table's new name (JSON wire keys, Python string literals, docs prose). `"kind"` on a listing object in JSON becomes `"offering_type"`; other JSON `"kind"` keys stay. Do not mechanically rewrite `docs/superpowers/` history.

- [x] **Step 5: Regenerate and run everything**

```bash
just gen
.venv/bin/python -m unittest tests.test_naming_conventions -v   # expected: PASS (5 tests)
just validate                                                    # expected: all PASS lines
just test-generated                                              # expected: OK
just check                                                       # expected: exit 0
```

If `test_date_slots_have_date_range` flags a pre-existing `*_date` slot missing `range: date` (e.g. a description-only slot), add the correct range — that is the census doing its job.

- [x] **Step 6: Report**

Report the task complete with the file list; suggested commit message: `refactor: standardize temporal and type-discriminator naming package-wide`.

---

### Task 2: ListingStatus expansion

**Files:**
- Modify: `schema/core.yaml` (ListingStatus enum)
- Test: `tests/test_schema_source.py` (only if it asserts enum membership — check first)

**Interfaces:**
- Consumes: Task 1's renames.
- Produces: `ListingStatus` permissible values `coming_soon, active, active_under_contract, pending, hold, sold, leased, withdrawn, canceled, expired, other` — used by Task 4's events and Task 10's fixtures.

- [x] **Step 1: Extend the enum**

In `schema/core.yaml`, replace the `ListingStatus` permissible values with:

```yaml
  ListingStatus:
    description: >-
      PHDS-normalized listing status (close mapping to RESO StandardStatus).
      sold/leased replace RESO's ambiguous Closed; Incomplete and Delete are
      MLS record-management states that normalize to other with the raw value
      preserved in extras or source_status.
    permissible_values:
      coming_soon: {}
      active: {}
      active_under_contract: {}
      pending: {}
      hold: {}
      sold: {}
      leased: {}
      withdrawn: {}
      canceled: {}
      expired: {}
      other: {}
```

- [x] **Step 2: Regenerate and test**

```bash
just gen
grep -rn "active_under_contract" schema/generated/phds.schema.json  # expected: present in ListingStatus enum
just test-generated && just validate                                # expected: pass
```

- [x] **Step 3: Report**

Suggested commit message: `feat: expand ListingStatus toward RESO StandardStatus coverage`.

---

### Task 3: Listing identity and agreement facts

**Files:**
- Modify: `schema/entities.yaml` (Listing, new ListingIdentifier class)
- Modify: `tools/profile_validation.py` (identifier semantic rules)
- Create: `counter_examples/schema/PropertyProfile-blank-listing-identifier-value.json`
- Create: `counter_examples/semantic/PropertyProfile-duplicate-listing-identifier.json`
- Create: `counter_examples/semantic/PropertyProfile-two-primary-listing-identifiers.json`
- Test: `tests/test_profile_validation.py` (add unit tests beside existing patterns)

**Interfaces:**
- Consumes: Task 1's `offering_type` on Listing.
- Produces: `ListingIdentifier` (attributes `scheme` req, `namespace`, `value` req NONBLANK, `is_primary`, `extras`); `Listing.identifiers[]`; Listing header fields `listing_contract_date`, `expiration_date`, `listing_agreement_type`, `service_level`, `marketing_channel`, `exposure_type`, `special_listing_conditions[]`. `mls_number` and `listing_type` are GONE. Validation helper `_validate_listing_identifiers(listings, issues)` in `tools/profile_validation.py`.

- [x] **Step 1: Schema — add ListingIdentifier and rework Listing**

In `schema/entities.yaml`, immediately above `Listing`, add (NONBLANK = the Entity.id pattern, copied verbatim):

```yaml
  ListingIdentifier:
    description: >-
      Namespaced listing identifier mirroring PropertyIdentifier
      (scheme + namespace + value). RESO ListingKey and ListingId stay
      separately recoverable as distinct schemes; identity rule is
      (namespace, scheme=listing_key, value) with provider-specific
      fallback. At most one is_primary identifier per namespace.
    attributes:
      scheme: {required: true, description: "listing_key | listing_id | source_system_id | other (open)"}
      namespace: {description: "Issuing system: MLS, aggregator, or portal"}
      value: {required: true, pattern: NONBLANK}
      is_primary: {range: boolean}
      extras: {range: Any}
```

Rework `Listing` (delete `mls_number` and `listing_type`; keep `property`, `property_state`, `offering_type`, `events`, `participants`, `artifacts`, `remarks`) and add:

```yaml
      identifiers: {range: ListingIdentifier, multivalued: true, inlined: true, inlined_as_list: true}
      listing_contract_date: {range: date, exact_mappings: [reso:ListingContractDate]}
      expiration_date: {range: date, exact_mappings: [reso:ExpirationDate]}
      listing_agreement_type: {description: "exclusive_right_to_sell | exclusive_agency | open | net | other (open)", close_mappings: [reso:ListingAgreement]}
      service_level: {description: "full_service | limited_service | entry_only | other (open)", close_mappings: [reso:ListingService]}
      marketing_channel: {description: "mls | private_network | public_portal | owner_direct | auction_platform | other (open)"}
      exposure_type: {description: "public | office_exclusive | private | coming_soon | other (open)"}
      special_listing_conditions: {range: CodeableConcept, multivalued: true, inlined: true, inlined_as_list: true, close_mappings: [reso:SpecialListingConditions]}
```

Update the `Listing` docstring: header fields are identity + latest-observed agreement terms; lifecycle (status, prices, close) derives from events per the existing derivation rules (keep the existing derivation prose, with renamed fields).

- [x] **Step 2: Semantic rules — write failing tests first**

In `tests/test_profile_validation.py`, following the file's existing test style (import of `validate_profile`, minimal profile dicts), add:

```python
def _listing_profile(identifiers):
    return {
        "property": {"id": "prop-1"},
        "listings": [
            {
                "id": "lst-1",
                "property": "prop-1",
                "offering_type": "for_sale",
                "identifiers": identifiers,
            }
        ],
    }


class ListingIdentifierRules(unittest.TestCase):
    def test_duplicate_identifier_rejected(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1"},
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1"},
                ]
            )
        )
        self.assertTrue(
            any("duplicate listing identifier" in i.message for i in issues), issues
        )

    def test_two_primaries_in_namespace_rejected(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1", "is_primary": True},
                    {"scheme": "listing_id", "namespace": "mls-x", "value": "12345", "is_primary": True},
                ]
            )
        )
        self.assertTrue(
            any("multiple primary" in i.message for i in issues), issues
        )

    def test_distinct_namespaces_each_primary_ok(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1", "is_primary": True},
                    {"scheme": "listing_key", "namespace": "mls-y", "value": "K9", "is_primary": True},
                ]
            )
        )
        self.assertFalse(
            any("identifier" in i.message for i in issues), issues
        )
```

Run: `.venv/bin/python -m unittest tests.test_profile_validation -k ListingIdentifierRules -v`
Expected: FAIL (no such validation yet).

- [x] **Step 3: Implement `_validate_listing_identifiers`**

In `tools/profile_validation.py`, add beside the other `_validate_*` helpers:

```python
def _validate_listing_identifiers(
    listings: list[dict[str, Any]], issues: list[ValidationIssue]
) -> None:
    for listing_index, listing in enumerate(listings):
        seen: Counter[tuple[Any, Any, Any]] = Counter()
        primaries: Counter[str] = Counter()
        for identifier_index, identifier in enumerate(
            _items(listing.get("identifiers"))
        ):
            path = f"listings[{listing_index}].identifiers[{identifier_index}]"
            key = (
                identifier.get("scheme"),
                identifier.get("namespace"),
                identifier.get("value"),
            )
            seen[key] += 1
            if seen[key] > 1:
                issues.append(
                    ValidationIssue(
                        path,
                        "duplicate listing identifier (scheme, namespace, value)",
                    )
                )
            if identifier.get("is_primary"):
                namespace = identifier.get("namespace") or ""
                primaries[namespace] += 1
                if primaries[namespace] > 1:
                    issues.append(
                        ValidationIssue(
                            path,
                            "multiple primary listing identifiers in one namespace",
                        )
                    )
```

Wire it into `validate_profile` after `_validate_rent_rolls(...)`:

```python
    _validate_listing_identifiers(_items(profile.get("listings")), issues)
```

Run the Step 2 tests again. Expected: PASS.

- [x] **Step 4: Counter-example fixtures**

`counter_examples/schema/PropertyProfile-blank-listing-identifier-value.json` (fails linkml on the NONBLANK pattern):

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_sale",
      "identifiers": [{"scheme": "listing_id", "namespace": "mls-x", "value": "  "}]
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-duplicate-listing-identifier.json` (structurally valid, one semantic issue):

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_sale",
      "identifiers": [
        {"scheme": "listing_key", "namespace": "mls-x", "value": "K100"},
        {"scheme": "listing_key", "namespace": "mls-x", "value": "K100"}
      ]
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-two-primary-listing-identifiers.json`:

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_sale",
      "identifiers": [
        {"scheme": "listing_key", "namespace": "mls-x", "value": "K100", "is_primary": true},
        {"scheme": "listing_id", "namespace": "mls-x", "value": "1234567", "is_primary": true}
      ]
    }
  ]
}
```

- [x] **Step 5: Regenerate, validate, full suite**

```bash
just gen && just validate && just test-generated
```
Expected: new counter-examples appear as `PASS structurally rejected` / `PASS semantically rejected`; all tests OK. If `tests/test_profile_validation.py` has an exact-issue census (see its tail — the fixture census precedent), register each new counter-example with its exact expected issue message.

- [x] **Step 6: Report**

Suggested commit message: `feat: namespaced listing identifiers and agreement facts on Listing`.

---

### Task 4: ListingEvent refinement

**Files:**
- Modify: `schema/entities.yaml` (ListingEvent)
- Modify: `tools/profile_validation.py` (event semantic rules)
- Create: `counter_examples/semantic/PropertyProfile-listing-event-effective-at-mismatch.json`
- Create: `counter_examples/semantic/PropertyProfile-close-price-on-active-event.json`
- Create: `counter_examples/semantic/PropertyProfile-lease-list-price-without-rent-period.json`
- Test: `tests/test_profile_validation.py`

**Interfaces:**
- Consumes: Task 1 (`effective_date`, `event_type`), Task 2 (expanded statuses).
- Produces: ListingEvent slots `effective_date` (req), `effective_at`, `observed_at`, `event_type` (req), `status`, `source_status`, `list_price`, `list_price_low`, `rent_period`, `close_price`, `extras`, `provenance`. Helper `_validate_listing_events(listings, issues)`.

- [x] **Step 1: Schema — rework ListingEvent**

Replace the `ListingEvent` block in `schema/entities.yaml` with:

```yaml
  ListingEvent:
    description: >-
      A dated listing assertion. effective_date is when the change took
      effect; effective_at adds optional precision and must fall on
      effective_date (compared on the timestamp's lexical date in its stated
      offset). observed_at is when the producer retrieved or detected the
      event. Order events by effective_date ascending, then effective_at when
      present, then array order. list_price is the asking sale price or
      periodic rent; rent_period states the period when it is rent.
      close_price is asserted only on a closed event and is the source's
      reported closing claim, not the reconciled SaleEvent.sale_price.
    attributes:
      effective_date: {range: date, required: true}
      effective_at: {range: datetime}
      observed_at: {range: datetime}
      event_type: {required: true, description: "listed | price_change | status_change | coming_soon | back_on_market | relisted | contingent | pending | closed | expired | withdrawn | canceled (open)"}
      status: {range: ListingStatus, close_mappings: [reso:StandardStatus]}
      source_status: {exact_mappings: [reso:MlsStatus], description: Native source status verbatim}
      list_price: {range: Money, close_mappings: [reso:ListPrice]}
      list_price_low: {range: Money, close_mappings: [reso:ListPriceLow], description: Lower bound for range-priced listings}
      rent_period: {range: RentPeriod}
      close_price: {range: Money, close_mappings: [reso:ClosePrice]}
      extras: {range: Any}
      provenance: {range: Provenance}
```

- [x] **Step 2: Failing tests for the three event rules**

Add to `tests/test_profile_validation.py`:

```python
def _event_profile(offering_type, event):
    return {
        "property": {"id": "prop-1"},
        "listings": [
            {
                "id": "lst-1",
                "property": "prop-1",
                "offering_type": offering_type,
                "events": [event],
            }
        ],
    }


class ListingEventRules(unittest.TestCase):
    def test_effective_at_must_match_effective_date(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "effective_at": "2026-03-02T09:00:00-05:00",
                    "event_type": "listed",
                },
            )
        )
        self.assertTrue(
            any("effective_at" in i.message for i in issues), issues
        )

    def test_close_price_only_on_closed_event(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "event_type": "price_change",
                    "close_price": {"amount": "400000", "currency": "USD"},
                },
            )
        )
        self.assertTrue(
            any("close_price" in i.message for i in issues), issues
        )

    def test_lease_list_price_requires_rent_period(self):
        issues = validate_profile(
            _event_profile(
                "for_lease",
                {
                    "effective_date": "2026-03-01",
                    "event_type": "listed",
                    "list_price": {"amount": "2500", "currency": "USD"},
                },
            )
        )
        self.assertTrue(
            any("rent_period" in i.message for i in issues), issues
        )

    def test_valid_closed_event_passes(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "effective_at": "2026-03-01T16:30:00-05:00",
                    "event_type": "closed",
                    "status": "sold",
                    "close_price": {"amount": "400000", "currency": "USD"},
                },
            )
        )
        self.assertEqual(issues, [])
```

Run: `.venv/bin/python -m unittest tests.test_profile_validation -k ListingEventRules -v`
Expected: first three FAIL, last passes trivially only after implementation — run again post-Step 3.

- [x] **Step 3: Implement `_validate_listing_events`**

In `tools/profile_validation.py`:

```python
def _validate_listing_events(
    listings: list[dict[str, Any]], issues: list[ValidationIssue]
) -> None:
    for listing_index, listing in enumerate(listings):
        offering_type = listing.get("offering_type")
        for event_index, event in enumerate(_items(listing.get("events"))):
            path = f"listings[{listing_index}].events[{event_index}]"
            effective_at = event.get("effective_at")
            effective_date = event.get("effective_date")
            if effective_at and effective_date:
                if str(effective_at)[:10] != str(effective_date):
                    issues.append(
                        ValidationIssue(
                            path,
                            "effective_at lexical date does not match effective_date",
                        )
                    )
            if (
                event.get("close_price") is not None
                and event.get("event_type") != "closed"
            ):
                issues.append(
                    ValidationIssue(path, "close_price is only valid on a closed event")
                )
            if (
                offering_type == "for_lease"
                and event.get("list_price") is not None
                and event.get("rent_period") is None
            ):
                issues.append(
                    ValidationIssue(
                        path,
                        "for_lease list_price requires rent_period",
                    )
                )
```

Wire into `validate_profile` next to `_validate_listing_identifiers`:

```python
    _validate_listing_events(_items(profile.get("listings")), issues)
```

Run Step 2 tests. Expected: all 4 PASS.

- [x] **Step 4: Counter-example fixtures**

`counter_examples/semantic/PropertyProfile-listing-event-effective-at-mismatch.json`:

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_sale",
      "events": [
        {
          "effective_date": "2026-03-01",
          "effective_at": "2026-03-02T09:00:00-05:00",
          "event_type": "listed",
          "status": "active",
          "list_price": {"amount": "400000", "currency": "USD"}
        }
      ]
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-close-price-on-active-event.json`:

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_sale",
      "events": [
        {
          "effective_date": "2026-03-01",
          "event_type": "price_change",
          "status": "active",
          "close_price": {"amount": "400000", "currency": "USD"}
        }
      ]
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-lease-list-price-without-rent-period.json`:

```json
{
  "property": {"id": "prop-1"},
  "listings": [
    {
      "id": "lst-1",
      "property": "prop-1",
      "offering_type": "for_lease",
      "events": [
        {
          "effective_date": "2026-03-01",
          "event_type": "listed",
          "status": "active",
          "list_price": {"amount": "2500", "currency": "USD"}
        }
      ]
    }
  ]
}
```

- [x] **Step 5: Regenerate, validate, full suite**

```bash
just gen && just validate && just test-generated
```
Expected: all pass, new semantic counter-examples rejected. Register exact expected messages in the fixture census if present.

- [x] **Step 6: Report**

Suggested commit message: `feat: RESO-aligned listing events with effective/observed times and status pair`.

---

### Task 5: SaleEvent refinement + value-classification annotations

**Files:**
- Modify: `schema/entities.yaml` (SaleEvent)
- Modify: `examples/PropertyProfile-sfr-example-ga.json` and any other fixture using `sale_date` / `financing` (Task 1's grep already renamed nothing here — these are semantic changes)
- Create: `tests/test_value_classification.py`

**Interfaces:**
- Consumes: Task 1.
- Produces: SaleEvent slots `close_date` (req, was `sale_date`), `sale_price` (annotation `reconciled`, no reso exact mapping), `buyer_financing` (CodeableConcept), `concessions_amount` (Money), `concessions[]` (CodeableConcept), `concessions_comments`, unchanged others. Wire change: `"sale_date"` → `"close_date"`, `"financing": "cash"` → `"buyer_financing": {"code": "cash"}`.

- [x] **Step 1: Write the failing annotation test**

Create `tests/test_value_classification.py`:

```python
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
```

Run: `.venv/bin/python -m unittest tests.test_value_classification -v`
Expected: FAIL (annotations absent). If `annotations._get` is unavailable in the installed linkml-runtime, use `slot.annotations["value_classification"]` inside a `KeyError`-tolerant helper — assert equivalently.

- [x] **Step 2: Rework SaleEvent in the schema**

Apply to the `SaleEvent` block:

```yaml
      close_date: {range: date, required: true, close_mappings: [reso:CloseDate], description: Closing date of the market sale (reconciled layer)}
      sale_price:
        range: Money
        description: Reconciled market-sale amount — NOT the raw MLS ClosePrice claim (that is SaleEvidence.close_price)
        related_mappings: [reso:ClosePrice]
        annotations: {value_classification: reconciled}
      buyer_financing: {range: CodeableConcept, close_mappings: [reso:BuyerFinancing], description: Financing the purchaser used to close}
      concessions_amount: {range: Money, close_mappings: [reso:ConcessionsAmount]}
      concessions: {range: CodeableConcept, multivalued: true, inlined: true, inlined_as_list: true, close_mappings: [reso:Concessions]}
      concessions_comments: {close_mappings: [reso:ConcessionsComments]}
```

replacing `sale_date`, `sale_price` (old form with `exact_mappings`), `financing`, and `concessions`. Add `annotations: {value_classification: derived}` to `price_per_area`, `price_per_unit`, and `cap_rate`.

- [x] **Step 3: Update fixtures**

In every fixture with sales (at minimum `examples/PropertyProfile-sfr-example-ga.json`): `"sale_date"` → `"close_date"`; `"financing": "conventional"` → `"buyer_financing": {"code": "conventional"}`. Find them:

```bash
grep -rln '"sale_date"\|"financing"' examples counter_examples tests
```

- [x] **Step 4: Regenerate and test**

```bash
just gen
.venv/bin/python -m unittest tests.test_value_classification -v  # expected: PASS
just validate && just test-generated                             # expected: pass
```

- [x] **Step 5: Report**

Suggested commit message: `feat: reconciled-layer SaleEvent with RESO-named financing and concessions`.

---

### Task 6: SaleListingRelationship + reference registry + property coherence

**Files:**
- Modify: `schema/entities.yaml` (new class; SaleEvent.listings)
- Modify: `tools/profile_validation.py` (registry + coherence checks)
- Create: `counter_examples/semantic/PropertyProfile-unresolved-sale-listing-relationship.json`
- Create: `counter_examples/semantic/PropertyProfile-sale-listing-cross-property.json`
- Test: `tests/test_profile_validation.py`

**Interfaces:**
- Consumes: Tasks 1, 5 (`close_date`).
- Produces: `SaleListingRelationship` (`listing` req non-inlined ref, `relationship_type` req, `extras`); `SaleEvent.listings[]` (inlined list); `REFERENCE_TARGET_COLLECTIONS` gains `"Listing": "listings"`; helper `_validate_sale_coherence(profile, issues)` (extended by Task 7).

- [x] **Step 1: Schema**

Below `SaleEventParty` add:

```yaml
  SaleListingRelationship:
    description: >-
      Sale-to-listing relation. Vocabulary stays sale-oriented; relist chains
      are listing-to-listing facts and "this MLS reported the sale" is a
      SaleEvidence row with listing set — neither belongs here.
    attributes:
      listing: {range: Listing, inlined: false, required: true, pattern: NONBLANK}
      relationship_type: {required: true, description: "resulted_in_sale | prior_listing | other (open)"}
      extras: {range: Any}
```

On `SaleEvent`, add:

```yaml
      listings: {range: SaleListingRelationship, multivalued: true, inlined: true, inlined_as_list: true, description: Absent/empty for off-market sales}
```

- [x] **Step 2: Failing tests — registry and coherence**

Add to `tests/test_profile_validation.py`:

```python
class SaleListingRelationshipRules(unittest.TestCase):
    @staticmethod
    def _profile(listing_property="prop-1"):
        return {
            "property": {"id": "prop-1"},
            "listings": [
                {"id": "lst-1", "property": listing_property, "offering_type": "for_sale"}
            ],
            "sales": [
                {
                    "id": "sale-1",
                    "property": "prop-1",
                    "close_date": "2026-04-01",
                    "listings": [
                        {"listing": "lst-1", "relationship_type": "resulted_in_sale"}
                    ],
                }
            ],
        }

    def test_resolved_same_property_ok(self):
        self.assertEqual(validate_profile(self._profile()), [])

    def test_unresolved_listing_reference(self):
        profile = self._profile()
        profile["sales"][0]["listings"][0]["listing"] = "lst-missing"
        issues = validate_profile(profile)
        self.assertTrue(
            any("does not resolve" in i.message for i in issues), issues
        )

    def test_cross_property_listing_rejected(self):
        profile = self._profile(listing_property="prop-other")
        # prop-other must exist somewhere for the listing itself to resolve;
        # the point is the sale and listing disagree about the property.
        issues = validate_profile(profile)
        self.assertTrue(
            any("different property" in i.message for i in issues), issues
        )
```

Note: in `test_cross_property_listing_rejected` the listing's `property: "prop-other"` will ALSO raise an unresolved-property issue (profile only bundles `prop-1`) — assert specifically on the `different property` message, which must be present regardless.

Run: `.venv/bin/python -m unittest tests.test_profile_validation -k SaleListingRelationshipRules -v`
Expected: `test_resolved_same_property_ok` ERRORS with `RuntimeError: no PropertyProfile target bundle for Listing` — that's the fail-closed registry doing its job.

- [x] **Step 3: Register targets and implement coherence**

In `tools/profile_validation.py`, add to `REFERENCE_TARGET_COLLECTIONS` (alphabetical position):

```python
    "Listing": "listings",
    "SaleEvent": "sales",
```

(`SaleEvent` is needed by Task 7; registering both now is harmless — the registry only requires that referenced targets have bundles.)

Add the coherence helper:

```python
def _validate_sale_coherence(
    profile: dict[str, Any], issues: list[ValidationIssue]
) -> None:
    listings_by_id = {
        listing.get("id"): listing for listing in _items(profile.get("listings"))
    }
    transfers_by_id = {
        transfer.get("id"): transfer for transfer in _items(profile.get("transfers"))
    }
    sales_by_id = {sale.get("id"): sale for sale in _items(profile.get("sales"))}

    for sale_index, sale in enumerate(_items(profile.get("sales"))):
        for relationship_index, relationship in enumerate(_items(sale.get("listings"))):
            target = listings_by_id.get(relationship.get("listing"))
            if target and target.get("property") != sale.get("property"):
                issues.append(
                    ValidationIssue(
                        f"sales[{sale_index}].listings[{relationship_index}].listing",
                        "listing refers to a different property than the sale",
                    )
                )

    for evidence_index, evidence in enumerate(_items(profile.get("sale_evidence"))):
        sale = sales_by_id.get(evidence.get("sale"))
        if not sale:
            continue
        sale_property = sale.get("property")
        for slot, index in (("listing", listings_by_id), ("transfer", transfers_by_id)):
            target = index.get(evidence.get(slot))
            if target and target.get("property") != sale_property:
                issues.append(
                    ValidationIssue(
                        f"sale_evidence[{evidence_index}].{slot}",
                        f"{slot} refers to a different property than the evidenced sale",
                    )
                )
```

(The `sale_evidence` loop is inert until Task 7 adds the bundle — it iterates an absent key.) Wire into `validate_profile`:

```python
    _validate_sale_coherence(profile, issues)
```

Run Step 2 tests. Expected: all 3 PASS.

- [x] **Step 4: Counter-example fixtures**

`counter_examples/semantic/PropertyProfile-unresolved-sale-listing-relationship.json`:

```json
{
  "property": {"id": "prop-1"},
  "sales": [
    {
      "id": "sale-1",
      "property": "prop-1",
      "close_date": "2026-04-01",
      "listings": [{"listing": "lst-missing", "relationship_type": "resulted_in_sale"}]
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-sale-listing-cross-property.json` — two properties can't share one profile, so express the mismatch via a listing whose `property` is a dangling id; the counter-example must fail for the coherence reason. To keep it single-issue, bundle both properties' ids is impossible (profile has one `property`); instead make the LISTING's property reference the profile property and the SALE's property a second entity? Also impossible. Single-issue form: listing resolves, sale resolves, but listing.property ≠ sale.property with listing.property dangling produces TWO issues — acceptable only if the fixture census allows multiple; if the census demands exactly one issue, drop this fixture file and rely on the unit test `test_cross_property_listing_rejected` for coverage. Decide per the census's actual behavior (check `tests/test_profile_validation.py` tail) and record the choice in the task report.

- [x] **Step 5: Regenerate, validate, full suite**

```bash
just gen && just validate && just test-generated
```
Expected: pass.

- [x] **Step 6: Report**

Suggested commit message: `feat: sale-to-listing relationships with property coherence validation`.

---

### Task 7: SaleEvidence entity + profile wiring

**Files:**
- Modify: `schema/entities.yaml` (SaleEvidence class)
- Modify: `schema/profiles.yaml` (sale_evidence bundle)
- Modify: `tests/test_value_classification.py` (extend EXPECTED)
- Create: `counter_examples/schema/PropertyProfile-sale-evidence-missing-provenance.json`
- Create: `counter_examples/semantic/PropertyProfile-unresolved-sale-evidence-sale.json`
- Test: `tests/test_profile_validation.py`

**Interfaces:**
- Consumes: Task 5 (`close_date` naming), Task 6 (registry entries + `_validate_sale_coherence` evidence loop).
- Produces: `SaleEvidence` entity (slots below); `PropertyProfile.sale_evidence[]`. Task 10 fixtures use it.

- [x] **Step 1: Schema — SaleEvidence**

Below `SaleListingRelationship` in `schema/entities.yaml`:

```yaml
  SaleEvidence:
    is_a: Entity
    description: >-
      One source's claims about a market sale. SaleEvent holds the
      reconciliation; evidence rows hold the disagreeing inputs (MLS close
      record, deed, assessor, broker or appraiser verification). One row =
      one source = one provenance. When an MLS-sourced row links a listing,
      its close_date/close_price should equal that listing's closed-event
      values (convention, deliberately unenforced).
    attributes:
      sale: {range: SaleEvent, inlined: false, required: true, pattern: NONBLANK}
      listing: {range: Listing, inlined: false}
      transfer: {range: Transfer, inlined: false}
      close_date: {range: date, exact_mappings: [reso:CloseDate], annotations: {value_classification: asserted}}
      close_price: {range: Money, exact_mappings: [reso:ClosePrice], annotations: {value_classification: asserted}}
      concessions_amount: {range: Money, exact_mappings: [reso:ConcessionsAmount], annotations: {value_classification: asserted}}
      document_number: {}
      verification_method: {range: CodeableConcept, description: "mls_record | deed | assessor | broker_confirmed | appraiser_verified | ... (open)"}
      remarks: {}
      artifacts: {range: SourceArtifact, multivalued: true, inlined: false}
      provenance: {range: Provenance, required: true}
```

(`provenance` here overrides the optional `Entity.provenance` as required.)

In `schema/profiles.yaml`, after the `sales:` line add:

```yaml
      sale_evidence: {range: SaleEvidence, multivalued: true, inlined: true, inlined_as_list: true}
```

- [x] **Step 2: Extend the annotation test**

In `tests/test_value_classification.py` add to `EXPECTED`:

```python
    ("SaleEvidence", "close_date"): "asserted",
    ("SaleEvidence", "close_price"): "asserted",
    ("SaleEvidence", "concessions_amount"): "asserted",
```

Run: `.venv/bin/python -m unittest tests.test_value_classification -v`
Expected: PASS (annotations were added with the class).

- [x] **Step 3: Reference-resolution tests**

Add to `tests/test_profile_validation.py`:

```python
class SaleEvidenceRules(unittest.TestCase):
    @staticmethod
    def _profile():
        return {
            "property": {"id": "prop-1"},
            "sales": [
                {"id": "sale-1", "property": "prop-1", "close_date": "2026-04-01"}
            ],
            "sale_evidence": [
                {
                    "id": "ev-1",
                    "sale": "sale-1",
                    "close_date": "2026-04-01",
                    "close_price": {"amount": "400000", "currency": "USD"},
                    "verification_method": {"code": "mls_record"},
                    "provenance": {"provider": "MLS X"},
                }
            ],
        }

    def test_valid_evidence_passes(self):
        self.assertEqual(validate_profile(self._profile()), [])

    def test_unresolved_sale_rejected(self):
        profile = self._profile()
        profile["sale_evidence"][0]["sale"] = "sale-missing"
        issues = validate_profile(profile)
        self.assertTrue(
            any("does not resolve" in i.message for i in issues), issues
        )
```

Run: `.venv/bin/python -m unittest tests.test_profile_validation -k SaleEvidenceRules -v`
Expected: PASS out of the box — the schema-derived reference rules and Task 6's registry entries cover it. If `test_valid_evidence_passes` errors with a RuntimeError about an unregistered target, a reference slot is missing its registry entry; fix the registry, not the test.

- [x] **Step 4: Counter-example fixtures**

`counter_examples/schema/PropertyProfile-sale-evidence-missing-provenance.json` (fails linkml: required `provenance` absent):

```json
{
  "property": {"id": "prop-1"},
  "sales": [{"id": "sale-1", "property": "prop-1", "close_date": "2026-04-01"}],
  "sale_evidence": [
    {
      "id": "ev-1",
      "sale": "sale-1",
      "close_price": {"amount": "400000", "currency": "USD"}
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-unresolved-sale-evidence-sale.json`:

```json
{
  "property": {"id": "prop-1"},
  "sales": [{"id": "sale-1", "property": "prop-1", "close_date": "2026-04-01"}],
  "sale_evidence": [
    {
      "id": "ev-1",
      "sale": "sale-missing",
      "close_price": {"amount": "400000", "currency": "USD"},
      "provenance": {"provider": "MLS X"}
    }
  ]
}
```

`counter_examples/semantic/PropertyProfile-unresolved-sale-evidence-artifact.json` (artifact ref absent from the profile bundle — matches the existing unresolved-artifact pattern):

```json
{
  "property": {"id": "prop-1"},
  "sales": [{"id": "sale-1", "property": "prop-1", "close_date": "2026-04-01"}],
  "sale_evidence": [
    {
      "id": "ev-1",
      "sale": "sale-1",
      "close_price": {"amount": "400000", "currency": "USD"},
      "artifacts": ["art-missing"],
      "provenance": {"provider": "MLS X"}
    }
  ]
}
```

- [x] **Step 5: Regenerate, validate, full suite**

```bash
just gen && just validate && just test-generated
```
Expected: pass; both counter-examples rejected at their intended layer.

- [x] **Step 6: Report**

Suggested commit message: `feat: SaleEvidence entity for per-source sale claims`.

---

### Task 8: SourceArtifact media extensions + Provenance source chain

**Files:**
- Modify: `schema/entities.yaml` (SourceArtifact)
- Modify: `schema/core.yaml` (Provenance)

**Interfaces:**
- Consumes: Task 1 (`captured_at`).
- Produces: SourceArtifact adds `order`, `short_description`, `source_modified_at`; Provenance adds `originating_system`, `source_system`, `source_record_id`, `source_created_at`, `source_modified_at`. Task 9's envelope and Task 10's fixtures use these.

- [x] **Step 1: SourceArtifact additions**

Add to `SourceArtifact` attributes in `schema/entities.yaml`:

```yaml
      order: {range: integer, minimum_value: 0, close_mappings: [reso:Order], description: Presentation order within the owning record's artifact list}
      short_description: {close_mappings: [reso:ShortDescription]}
      source_modified_at: {range: datetime, description: When the source system last modified this media/document record}
```

- [x] **Step 2: Provenance additions**

Add to `Provenance` attributes in `schema/core.yaml`:

```yaml
      originating_system: {close_mappings: [reso:OriginatingSystemName], description: System where the record was born}
      source_system: {close_mappings: [reso:SourceSystemName], description: Immediate system that supplied the record}
      source_record_id: {description: The immediate source's record identifier}
      source_created_at: {range: datetime, description: When the immediate source created the record}
      source_modified_at: {range: datetime, description: When the immediate source last modified the record}
```

- [x] **Step 3: Regenerate and test**

```bash
just gen
grep -n "originating_system\|short_description" schema/generated/phds.schema.json | head  # expected: present
just validate && just test-generated                                                       # expected: pass
.venv/bin/python -m unittest tests.test_naming_conventions -v                              # expected: PASS (new *_at slots are datetime)
```

- [x] **Step 4: Report**

Suggested commit message: `feat: media presentation fields and two-point provenance source lineage`.

---

### Task 9: MlsObservation capture envelope

**Files:**
- Modify: `schema/capture.yaml` (enum + class)
- Modify: `justfile` (capture example validation)
- Create: `examples/capture/MlsObservation-closed-sale.json`
- Test: existing `just validate` loop (extended)

**Interfaces:**
- Consumes: Tasks 3, 4, 8 (profile payload fields), existing `validate_capture_envelope` + `--document-type capture-envelope` in `tools/profile_validation.py`.
- Produces: `MlsObservationStatus` enum; `MlsObservation` class; `examples/capture/` directory convention (`<Class>-<name>.json`).

- [x] **Step 1: Schema — enum and class**

In `schema/capture.yaml` add to `enums`:

```yaml
  MlsObservationStatus:
    description: Outcome of an MLS / listing-feed record capture.
    permissible_values:
      success: {}
      not_found: {}
      api_error: {}
      parse_error: {}
      ambiguous: {}
```

and to `classes` (beside the sibling envelopes):

```yaml
  MlsObservation:
    description: >-
      Capture envelope for an MLS / RESO Web API / feed record. The payload
      is a sparse PropertyProfile (a valid profile still requires its
      `property` section); listing-centric property characteristics belong on
      the profile's state snapshots, not on timeless entities. By convention
      profile accompanies success and error accompanies failure statuses —
      validators deliberately do not enforce this pairing, so consumers must
      branch on status, not on field presence. RESO local fields ride in
      extras.reso_local keyed by source field name. original_entry_at and
      source_modified_at are the feed record's clocks (RESO
      OriginalEntryTimestamp / ModificationTimestamp); they duplicate
      provenance source clocks so they survive when the profile is detached
      from the envelope, and should agree with them.
    attributes:
      status: {range: MlsObservationStatus, required: true}
      query: {description: Listing id/key or address queried}
      source_record_key: {description: The feed's ListingKey for this record}
      original_entry_at: {range: datetime, close_mappings: [reso:OriginalEntryTimestamp]}
      source_modified_at: {range: datetime, close_mappings: [reso:ModificationTimestamp]}
      profile: {range: PropertyProfile, inlined: true}
      error: {}
      provenance: {range: Provenance, required: true}
      artifact_refs: {range: SourceArtifact, multivalued: true, inlined: false, description: "References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent."}
      extras: {range: Any}
```

- [x] **Step 2: Example fixture**

Create `examples/capture/MlsObservation-closed-sale.json`:

```json
{
  "status": "success",
  "query": "K2400031",
  "source_record_key": "K2400031",
  "original_entry_at": "2026-01-05T14:02:11Z",
  "source_modified_at": "2026-04-02T08:15:00Z",
  "provenance": {
    "provider": "Example Aggregator",
    "originating_system": "Example MLS",
    "source_system": "Example Aggregator Feed",
    "source_record_id": "K2400031",
    "source_modified_at": "2026-04-02T08:15:00Z",
    "retrieved_at": "2026-04-02T09:00:00Z",
    "method": "api"
  },
  "artifact_refs": ["art-photo-1"],
  "profile": {
    "property": {"id": "prop-mls-1"},
    "artifacts": [
      {
        "id": "art-photo-1",
        "uri": "https://photos.example.com/K2400031/1.jpg",
        "media_type": "image/jpeg",
        "kind": {"code": "photo"},
        "order": 1,
        "short_description": "Front elevation"
      }
    ],
    "listings": [
      {
        "id": "lst-mls-1",
        "property": "prop-mls-1",
        "offering_type": "for_sale",
        "identifiers": [
          {"scheme": "listing_key", "namespace": "example-mls", "value": "K2400031", "is_primary": true},
          {"scheme": "listing_id", "namespace": "example-mls", "value": "7654321"}
        ],
        "listing_contract_date": "2026-01-04",
        "marketing_channel": "mls",
        "artifacts": ["art-photo-1"],
        "events": [
          {"effective_date": "2026-01-05", "event_type": "listed", "status": "active", "source_status": "A", "list_price": {"amount": "415000", "currency": "USD"}},
          {"effective_date": "2026-03-01", "event_type": "pending", "status": "pending", "source_status": "P"},
          {"effective_date": "2026-04-01", "event_type": "closed", "status": "sold", "source_status": "C", "close_price": {"amount": "407500", "currency": "USD"}}
        ]
      }
    ],
    "extras": {"reso_local": {"LO1_BoardCode": "EX"}}
  }
}
```

Check `method: "api"` against the `CaptureMethod` enum in `schema/core.yaml` first; use an actual permissible value.

- [x] **Step 3: Extend `just validate`**

In `justfile`, add to the `validate` recipe after the PropertyProfile examples loop:

```make
    for f in examples/capture/MlsObservation-*.json; do \
      .venv/bin/linkml-validate -s schema/capture.yaml -C MlsObservation "$f" || exit 1; \
      .venv/bin/python tools/profile_validation.py --document-type capture-envelope "$f" || exit 1; \
      echo "PASS $f"; \
    done
```

- [x] **Step 4: Regenerate and validate**

```bash
just gen && just validate
```
Expected: `PASS examples/capture/MlsObservation-closed-sale.json` appears; everything else still green. Then `just test-generated`.

- [x] **Step 5: Report**

Suggested commit message: `feat: MlsObservation capture envelope for MLS feed records`.

---

### Task 10: Integration fixtures

**Files:**
- Create: `examples/PropertyProfile-listing-lifecycle.json`
- Create: `examples/PropertyProfile-sale-evidence.json`

**Interfaces:**
- Consumes: everything from Tasks 1–8.
- Produces: the spec's positive-fixture matrix (closed listing with history; active range-priced listing; lease listing closing leased; relist with second namespace; disagreeing MLS+deed evidence; off-market sale).

- [x] **Step 1: Listing-lifecycle fixture**

`examples/PropertyProfile-listing-lifecycle.json` — one property, three listings covering closed-with-history, active-range-priced, lease-closing-leased, plus a relist chain (`lst-2024` withdrawn, `lst-2025` relisted under a second namespace):

```json
{
  "property": {"id": "prop-ll-1"},
  "listings": [
    {
      "id": "lst-2024",
      "property": "prop-ll-1",
      "offering_type": "for_sale",
      "identifiers": [
        {"scheme": "listing_key", "namespace": "mls-alpha", "value": "A1001", "is_primary": true}
      ],
      "listing_contract_date": "2024-02-01",
      "expiration_date": "2024-08-01",
      "listing_agreement_type": "exclusive_right_to_sell",
      "service_level": "full_service",
      "marketing_channel": "mls",
      "exposure_type": "public",
      "events": [
        {"effective_date": "2024-02-03", "event_type": "listed", "status": "active", "source_status": "ACT", "list_price": {"amount": "450000", "currency": "USD"}},
        {"effective_date": "2024-04-15", "event_type": "price_change", "status": "active", "list_price": {"amount": "435000", "currency": "USD"}},
        {"effective_date": "2024-07-20", "event_type": "withdrawn", "status": "withdrawn", "source_status": "WTH"}
      ]
    },
    {
      "id": "lst-2025",
      "property": "prop-ll-1",
      "offering_type": "for_sale",
      "identifiers": [
        {"scheme": "listing_key", "namespace": "mls-beta", "value": "B2002", "is_primary": true}
      ],
      "listing_contract_date": "2025-01-10",
      "marketing_channel": "mls",
      "events": [
        {"effective_date": "2025-01-12", "event_type": "relisted", "status": "active", "list_price": {"amount": "429000", "currency": "USD"}, "list_price_low": {"amount": "415000", "currency": "USD"}},
        {"effective_date": "2025-02-20", "effective_at": "2025-02-20T15:04:00-05:00", "event_type": "pending", "status": "pending"},
        {"effective_date": "2025-03-28", "event_type": "closed", "status": "sold", "close_price": {"amount": "421000", "currency": "USD"}}
      ]
    },
    {
      "id": "lst-lease",
      "property": "prop-ll-1",
      "offering_type": "for_lease",
      "identifiers": [
        {"scheme": "listing_id", "namespace": "mls-alpha", "value": "L-88", "is_primary": true}
      ],
      "events": [
        {"effective_date": "2025-06-01", "event_type": "listed", "status": "active", "list_price": {"amount": "2600", "currency": "USD"}, "rent_period": "monthly"},
        {"effective_date": "2025-07-04", "event_type": "closed", "status": "leased", "close_price": {"amount": "2500", "currency": "USD"}, "rent_period": "monthly"}
      ]
    }
  ],
  "provenance": {"provider": "example", "retrieved_at": "2026-07-18T12:00:00Z"}
}
```

- [x] **Step 2: Sale-evidence fixture**

`examples/PropertyProfile-sale-evidence.json` — a sale with disagreeing MLS and deed evidence plus its listing relationship, and a second off-market sale with no listings:

```json
{
  "property": {"id": "prop-se-1"},
  "parties": [
    {"id": "party-b", "kind": "person", "name": "Example Buyer"},
    {"id": "party-s", "kind": "person", "name": "Example Seller"}
  ],
  "listings": [
    {
      "id": "lst-se-1",
      "property": "prop-se-1",
      "offering_type": "for_sale",
      "identifiers": [
        {"scheme": "listing_key", "namespace": "mls-alpha", "value": "A9001", "is_primary": true}
      ],
      "events": [
        {"effective_date": "2026-01-05", "event_type": "listed", "status": "active", "list_price": {"amount": "415000", "currency": "USD"}},
        {"effective_date": "2026-04-01", "event_type": "closed", "status": "sold", "close_price": {"amount": "407500", "currency": "USD"}}
      ]
    }
  ],
  "transfers": [
    {
      "id": "xfer-se-1",
      "property": "prop-se-1",
      "transfer_type": "warranty_deed",
      "instrument_date": "2026-04-01",
      "recorded_date": "2026-04-09",
      "consideration": {"amount": "407000", "currency": "USD"},
      "parties": [
        {"role": "grantee", "party": "party-b", "sequence": 1},
        {"role": "grantor", "party": "party-s", "sequence": 1}
      ]
    }
  ],
  "sales": [
    {
      "id": "sale-se-1",
      "property": "prop-se-1",
      "transfer": "xfer-se-1",
      "close_date": "2026-04-01",
      "sale_price": {"amount": "407500", "currency": "USD"},
      "sale_type": "arms_length",
      "buyer_financing": {"code": "conventional"},
      "concessions_amount": {"amount": "5000", "currency": "USD"},
      "concessions": [{"code": "closing_costs"}],
      "concessions_comments": "Seller paid closing costs",
      "listings": [{"listing": "lst-se-1", "relationship_type": "resulted_in_sale"}],
      "parties": [
        {"role": "buyer", "party": "party-b", "sequence": 1},
        {"role": "seller", "party": "party-s", "sequence": 1}
      ]
    },
    {
      "id": "sale-se-offmarket",
      "property": "prop-se-1",
      "close_date": "2020-09-15",
      "sale_price": {"amount": "310000", "currency": "USD"},
      "sale_type": "arms_length"
    }
  ],
  "sale_evidence": [
    {
      "id": "ev-mls",
      "sale": "sale-se-1",
      "listing": "lst-se-1",
      "close_date": "2026-04-01",
      "close_price": {"amount": "407500", "currency": "USD"},
      "concessions_amount": {"amount": "5000", "currency": "USD"},
      "verification_method": {"code": "mls_record"},
      "provenance": {"provider": "MLS Alpha", "source_system": "MLS Alpha", "retrieved_at": "2026-04-02T09:00:00Z"}
    },
    {
      "id": "ev-deed",
      "sale": "sale-se-1",
      "transfer": "xfer-se-1",
      "close_date": "2026-04-01",
      "close_price": {"amount": "407000", "currency": "USD"},
      "document_number": "2026-041234",
      "verification_method": {"code": "deed"},
      "provenance": {"provider": "Example County Recorder", "retrieved_at": "2026-04-20T09:00:00Z"}
    }
  ],
  "provenance": {"provider": "example", "retrieved_at": "2026-07-18T12:00:00Z"}
}
```

Verify `Party` requires `kind` + `name` shapes against the schema before finalizing (adjust to actual required Party slots — check `Party` in `schema/entities.yaml`).

- [x] **Step 3: Validate**

```bash
just validate
```
Expected: both new example files print `PASS`. Fix any shape errors against the schema (never the other way around — these fixtures follow the schema, not vice versa).

- [x] **Step 4: Full suite**

```bash
just check
```
Expected: exit 0.

- [x] **Step 5: Report**

Suggested commit message: `test: listing lifecycle and sale evidence integration fixtures`.

---

### Task 11: RESO DD 2.0 crosswalk doc + doc updates

**Files:**
- Create: `docs/crosswalks/reso-dd20-alignment.md`
- Modify: `README.md` (add crosswalk link beside the UAD one, if the README lists crosswalks)
- Verify: `docs/crosswalks/uad36-alignment.md` reflects Task 1/5 renames (Task 1 already swept it; confirm `concessions_amount` if it appears)

**Interfaces:**
- Consumes: all prior tasks (documents the final shapes).
- Produces: the crosswalk document; no code.

- [x] **Step 1: Write the crosswalk**

Create `docs/crosswalks/reso-dd20-alignment.md` with this structure and content (prose follows the UAD doc's register; tables must include at least the rows below — extend where the schema has more mapped slots):

```markdown
# RESO Data Dictionary 2.0 alignment

Scope: how PHDS v0.2 represents RESO DD 2.0 listing and sale concepts.
Target version: DD 2.0 (active certification version). DD 2.1 is tracked
separately. PHDS remains internationally neutral; RESO vocabulary enters
through mappings and import derivations, never as core defaults.

## Mapping model

PHDS listings derive lifecycle state from ListingEvent streams; RESO's
Property resource is a flat latest-state record. Import therefore
synthesizes events from dated RESO fields and never fabricates dates:
undated claims stay in extras.reso_local. Strength column: exact =
same meaning and representation; close = same meaning, PHDS wraps or
narrows; transformed = structural or vocabulary conversion documented here.
All RESO numeric prices map to PHDS Money as transformed-or-close, never
exact: Money requires an ISO 4217 currency supplied by feed-level context.

## Listing identity

| PHDS | RESO DD 2.0 | Strength | Direction | Notes |
|---|---|---|---|---|
| ListingIdentifier scheme=listing_key | ListingKey | exact | both | namespace = issuing system |
| ListingIdentifier scheme=listing_id | ListingId | exact | both | human-facing MLS number |
| ListingIdentifier scheme=source_system_id | SourceSystemID / OriginatingSystemID | transformed | import | namespace disambiguates which system |
| Provenance.originating_system | OriginatingSystemName | close | import | |
| Provenance.source_system | SourceSystemName | close | import | |

## Listing header

| PHDS | RESO DD 2.0 | Strength | Direction | Notes |
|---|---|---|---|---|
| Listing.offering_type | PropertyType (lease values) | transformed | import | Residential/Commercial Lease → for_lease, else for_sale |
| Listing.listing_contract_date | ListingContractDate | exact | both | |
| Listing.expiration_date | ExpirationDate | exact | both | |
| Listing.listing_agreement_type | ListingAgreement | close | both | snake_case lookup values |
| Listing.service_level | ListingService | close | both | |
| Listing.exposure_type | (MlsStatus/office exclusive concepts) | transformed | import | |
| Listing.special_listing_conditions | SpecialListingConditions | close | both | CodeableConcept list |

## Listing events (flat-record derivations)

| RESO DD 2.0 | PHDS destination | Strength | Notes |
|---|---|---|---|
| OnMarketDate | listed event effective_date | transformed | |
| OriginalListPrice | earliest event list_price | transformed | only when a dated event can carry it; else extras.reso_local |
| ListPrice | latest event list_price | close | |
| ListPriceLow | event list_price_low | close | |
| PreviousListPrice | derived from event stream | transformed | undated → extras.reso_local, never a fabricated event |
| PurchaseContractDate | purchase-contract event | transformed | status only when StandardStatus states it |
| CloseDate + ClosePrice | closed event | close | the MLS claim; reconciled value is SaleEvent |
| StatusChangeTimestamp | current-status event effective_at | transformed | |
| WithdrawnDate / CancellationDate | withdrawn / canceled events | transformed | |
| OriginalEntryTimestamp | MlsObservation.original_entry_at + provenance | close | record clock, not a market fact |
| ModificationTimestamp | MlsObservation.source_modified_at + provenance | close | record clock, not a market fact |
| MlsStatus | event source_status | exact | |

## Status matrix (StandardStatus ↔ ListingStatus)

| StandardStatus | PHDS status | Notes |
|---|---|---|
| Coming Soon | coming_soon | |
| Active | active | |
| Active Under Contract | active_under_contract | |
| Pending | pending | |
| Hold | hold | |
| Closed | sold or leased | via offering_type; raw value in extras.reso_local when uncertain |
| Canceled | canceled | |
| Expired | expired | |
| Withdrawn | withdrawn | |
| Incomplete | other | raw value in extras.reso_local |
| Delete | other | record-management state; raw value in extras.reso_local |

MlsStatus always lands verbatim in source_status; disagreement between
MlsStatus and StandardStatus is preserved, not resolved.

## Sale and evidence

| PHDS | RESO DD 2.0 | Strength | Direction | Notes |
|---|---|---|---|---|
| SaleEvent.close_date | CloseDate | close | import | reconciled layer |
| SaleEvent.sale_price | ClosePrice | transformed | import | reconciled conclusion, deliberately NOT named close_price |
| SaleEvidence.close_date | CloseDate | exact | import | the source's claim |
| SaleEvidence.close_price | ClosePrice | exact | import | currency from feed context |
| SaleEvent.buyer_financing | BuyerFinancing | close | both | CodeableConcept |
| SaleEvent.concessions_amount | ConcessionsAmount | close | both | |
| SaleEvent.concessions | Concessions | close | both | |
| SaleEvent.concessions_comments | ConcessionsComments | exact | both | |
| Transfer.consideration | (none) | — | — | recorded instrument amount, distinct from ClosePrice |

## Media

| PHDS | RESO DD 2.0 (Media) | Strength | Notes |
|---|---|---|---|
| SourceArtifact.uri | MediaURL | close | |
| SourceArtifact.media_type | MimeType | close | |
| SourceArtifact.kind | MediaCategory | transformed | open CodeableConcept: photo, floor_plan, virtual_tour, document |
| SourceArtifact.order | Order | close | asset-level; per-listing reorder unsupported (documented limitation) |
| SourceArtifact.short_description | ShortDescription | close | |
| SourceArtifact.source_modified_at | ModificationTimestamp | close | |

## Deliberate divergences

- SaleEvent.sale_price stays sale_price (reconciled layer; the claim is
  SaleEvidence.close_price).
- source_status not mls_status (source-neutral standard).
- sold/leased instead of Closed (disambiguated by offering_type;
  transformation documented above).

## Loss ledger (this alignment's supported subset)

Preserved: identity keys, lifecycle events/dates/statuses/prices, agreement
facts, buyer financing, concessions trio, media references and order,
source clocks and two-point source lineage, local fields via
extras.reso_local.
Documented losses: contingency codes and removal dates; participant
Member/Office keys; media permission/dimension/deleted-record metadata;
multi-hop syndication lineage. Each rides in extras until its own slice.
```

- [x] **Step 2: Cross-check against the schema**

For every PHDS slot named in the doc, `grep -n` it in `schema/entities.yaml`/`core.yaml`/`capture.yaml` to confirm the name is real. Fix doc or (if the doc reveals a schema omission) flag it in the task report.

- [x] **Step 3: Hygiene test**

```bash
just test-generated
```
Expected: pass — `tests/test_docs_examples_hygiene.py` checks docs/examples consistency; fix anything it flags about the new doc.

- [x] **Step 4: Report**

Suggested commit message: `docs: RESO DD 2.0 crosswalk for listings, sales, and evidence`.

---

## Final verification (after all tasks)

- [x] `just check` — full suite green (determinism, validation, generated contracts, Rust round-trips).
- [x] `grep -rn "mls_number\|listing_type\|asking_price\|sale_date\|event_kind\|occurred_on" --include='*.yaml' --include='*.json' --include='*.py' . | grep -v -e schema/generated -e docs/superpowers -e '\.venv'` — expected: no hits (old vocabulary fully retired).
- [x] Spot-check `schema/generated/phds.ts` and `schema/generated/phds-rust/src/` for `OfferingType`, `SaleEvidence`, `MlsObservation` — confirm generators picked everything up.
- [x] Report to the user: full change summary + suggested commit message(s); await explicit commit permission.
