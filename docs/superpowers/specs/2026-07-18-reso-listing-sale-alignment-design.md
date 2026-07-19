# RESO DD 2.0 alignment — listing & sale model design

**Date:** 2026-07-18
**Status:** Approved design, pre-implementation
**Scope:** PHDS v0.2 (uncommitted working tree). Listing identity/lifecycle, sale
evidence, media, MLS capture envelope, provenance source chain, package-wide
naming conventions, RESO DD 2.0 crosswalk doc, fixtures and tests.

## Goals

A PHDS listing or sale payload can **represent, loss-aware,** the identity,
lifecycle, prices, dates, statuses, provenance, and local extensions of
commonly used RESO Data Dictionary 2.0 MLS records, for an enumerated
supported subset (see crosswalk). Alignment targets **RESO DD 2.0** (the
active certification version); DD 2.1 is tracked separately later.

Round-trip (RESO → PHDS → RESO) verification requires the adapter, which is a
non-goal here; this slice proves representability via PHDS-only fixtures and
records unsupported concepts in the crosswalk's loss ledger. Known documented
losses this slice accepts: contingency codes/removal dates (event_type
`contingent` + `extras` only), source Member/Office keys for participants
(Party identity work is a later slice), media permission/dimension metadata.

Non-goals for this slice: a RESO conformance profile in `schema/standards/`
(later slice, following the `uad_3_6.yaml` precedent), a RESO/RCF adapter
implementation, governance/field-catalog machinery, ListingObservation-style
snapshots beyond the `MlsObservation` envelope defined here.

## Architectural decisions (settled during brainstorm)

1. **Slice:** listings + sale/evidence together, plus media and the MLS
   observation envelope. Crosswalk doc ships in the same pass.
2. **Events remain the sole lifecycle truth.** No derived header fields
   reintroduced on `Listing`. The header gains only *agreement facts* that are
   not state changes. RESO flat records import by synthesizing events.
3. **SaleEvidence becomes a canonical entity.** `SaleEvent` is explicitly the
   reconciled layer; per-source claims live on evidence rows.
4. **Media reuses `SourceArtifact`** (extended), not a new MediaAsset entity.
   The MLS envelope is a third instance of the existing capture pattern.
5. **Approach A:** core refinement + LinkML mapping annotations + informative
   crosswalk doc. No RESO validation profile yet.

## Naming policy

Default to the snake_cased RESO field name whenever the concept is generic
real-estate vocabulary. Diverge only where PHDS deliberately means something
different; every divergence gets a crosswalk entry.

Adopted RESO names: `list_price`, `original_list_price` (derivation),
`close_price`, `close_date`, `listing_contract_date`, `on_market_date`
(derivation), `expiration_date`, `cancellation_date` (event), `buyer_financing`,
`concessions_amount`, `concessions_comments`, `special_listing_conditions`,
identifier schemes `listing_key` / `listing_id` / `source_system_id`.

Deliberate divergences:

| PHDS keeps | Not renamed to | Why |
|---|---|---|
| `SaleEvent.sale_price` | `close_price` | Reconciled market-sale conclusion, not the MLS claim; the claim is `SaleEvidence.close_price` |
| `source_status` | `mls_status` | Source-neutral standard; RESO `MlsStatus` maps to it |
| status values `sold` / `leased` | `closed` | More specific than RESO `Closed`; import disambiguates via `offering_type` |

### Package-wide conventions (breaking rename wave, applied everywhere)

- `*_date` = calendar date, `*_at` = datetime. Representative renames:
  `effective_on`→`effective_date`, `recorded_on`→`recorded_date`,
  `observed_on`→`observed_date`, `ListingEvent.occurred_on`→`effective_date`,
  `SourceArtifact.captured_on`→`captured_at` (already a datetime — name/type
  mismatch today). `valid_from`/`valid_to` stay.
- `_kind`→`_type` mechanically: `event_kind`→`event_type`,
  `transfer_kind`→`transfer_type`, `Listing.kind`→`offering_type`
  (enum `ListingKind`→`OfferingType`, values unchanged: `for_sale`/`for_lease`),
  and remaining `*_kind` slots/enums (`LoanEventKind`, `ValuationKind`, …)
  rename in the same wave.
- **Normative rename table:** the implementation plan must open with an
  exhaustive old-name→new-name inventory covering every `_on`-suffixed slot
  (incl. loan/foreclosure/permit dates), every `_kind` slot, every `*Kind`
  enum, and every bare `kind` slot (each resolved as rename or explicit
  keep — e.g. `SourceArtifact.kind` likely keeps its name), spanning schema,
  validators, generators, fixtures, tests, and current docs (incl. the UAD
  crosswalk). A census test then asserts no unintended `_on`/`_kind`/`*Kind`
  names remain, every `*_at` slot is `datetime`, and every `*_date` slot is
  `date`. Historical specs/plans under `docs/superpowers/` are not rewritten.

## Listing

### Identity

`mls_number` is deleted, replaced by:

```yaml
ListingIdentifier:
  attributes:
    scheme: {required: true}   # listing_key | listing_id | source_system_id | other (open)
    namespace: {}              # issuing system (MLS/aggregator), mirroring PropertyIdentifier
    value: {required: true}    # nonblank pattern like other identifiers
    is_primary: {range: boolean}

Listing:
  identifiers: {range: ListingIdentifier, multivalued: true, inlined: true, inlined_as_list: true}
```

Shape mirrors the existing `PropertyIdentifier` (`scheme + namespace + value`)
so listing and property identity follow one PHDS convention. RESO `ListingKey`
and `ListingId` remain separately recoverable as distinct schemes; originating
and source record keys use `source_system_id` with the system as `namespace`.
Documented identity rule: `(namespace, scheme=listing_key, value)`,
provider-specific fallback when no `listing_key` exists. Handles
cross-listing, syndication, relists.

Semantic rules: identifiers unique on `(scheme, namespace, value)` within a
listing; at most one `is_primary: true` identifier per namespace.

### Agreement facts (new header fields)

- `listing_contract_date: date` (RESO `ListingContractDate`)
- `expiration_date: date` (RESO `ExpirationDate`)
- `listing_agreement_type` — open: `exclusive_right_to_sell | exclusive_agency | open | net | other` (RESO `ListingAgreement`)
- `service_level` — open: `full_service | limited_service | entry_only | other` (RESO `ListingService`)
- `marketing_channel` — open: `mls | private_network | public_portal | owner_direct | auction_platform | other`
- `exposure_type` — open: `public | office_exclusive | private | coming_soon | other`
- `special_listing_conditions` — multivalued `CodeableConcept` (RESO `SpecialListingConditions`; open vocabulary)

`listing_type` is deleted; old values distribute: `mls`→`marketing_channel`,
`pocket`→`exposure_type: office_exclusive|private`, `auction`→
`special_listing_conditions`, `fsbo`→`marketing_channel: owner_direct`.

Invariant preserved: nothing on the header can contradict the event stream —
header fields are only agreement terms events don't carry. Status and prices
stay events-only.

Header semantics for mutable terms: agreement-fact fields are
**latest-observed agreement state** (record-level provenance dates the
observation). When a term changes materially (extended expiration, exposure
change), producers may also record a `status_change`/`extras`-bearing event;
the header always reflects the latest. This is the same
denormalized-convenience stance the old model took for status, restricted to
agreement terms. `coming_soon` in `exposure_type` describes marketing
exposure; the status/event values describe lifecycle state — both may
legitimately appear.

### Status enum

`ListingStatus` expands to: `coming_soon, active, active_under_contract,
pending, hold, sold, leased, withdrawn, canceled, expired, other`.
RESO `Incomplete`/`Delete` are MLS record-management states → `other` +
`source_status` (crosswalk-documented transformation).

## ListingEvent

```yaml
ListingEvent:
  attributes:
    effective_date: {range: date, required: true}   # was occurred_on
    effective_at: {range: datetime}                 # optional precision (RESO *Timestamp)
    observed_at: {range: datetime}                  # producer detection time (backfill-relevant)
    event_type: {required: true}                    # open: listed | price_change | status_change | coming_soon | back_on_market | relisted | contingent | pending | closed | expired | withdrawn | canceled
    status: {range: ListingStatus}
    source_status: {}                               # native MLS status verbatim (RESO MlsStatus)
    list_price: {range: Money}                      # was asking_price (RESO ListPrice; asking rent for leases)
    list_price_low: {range: Money}                  # lower bound for range-priced listings (RESO ListPriceLow)
    rent_period: {range: RentPeriod}
    close_price: {range: Money}                     # on closed events (MLS claim)
    extras: {range: Any}
    provenance: {range: Provenance}
```

- `effective_date` required; `effective_at`, when present, must fall on
  `effective_date`, compared on the timestamp's lexical date in its stated
  offset (source-local as written; no UTC conversion) — semantic validation.
- Event ordering: `effective_date` ascending; within a date, events carrying
  `effective_at` order by it; array order breaks remaining ties (including
  between timestamped and date-only events on the same date).
- No `previous_list_price` — derivable from history.
- **Events are only synthesized from source-supplied dates.** Flat-record
  derivations (crosswalk-documented): `OnMarketDate`→`listed` event;
  `PurchaseContractDate`→purchase-contract event (normalized `status` set only
  when the source's `StandardStatus` states it — `Pending` vs
  `ActiveUnderContract` is not inferred); `CloseDate`+`ClosePrice`→`closed`
  event; `StatusChangeTimestamp`→`effective_at` on the current-status event
  only; `WithdrawnDate`/`CancellationDate`→respective events. Undated claims
  are NOT manufactured into events: `PreviousListPrice` with no change date,
  and `OriginalListPrice`/`ListPrice` when no dated event can carry them,
  are preserved in `extras.reso_local` (loss-aware, not fabricated history).
  `OriginalEntryTimestamp`/`ModificationTimestamp` are source-system record
  clocks → provenance, never market facts.
- `offering_type` derivation on import: RESO `PropertyType` lease values
  (e.g. Residential Lease, Commercial Lease) → `for_lease`; otherwise
  `for_sale`. Conflicts (lease PropertyType with sale-only fields) keep the
  raw values in `extras.reso_local` and are flagged by the adapter (later
  slice).
- Raw `StandardStatus` is preserved losslessly: values that normalize 1:1 are
  reconstructible from `status`; the non-reconstructible cases
  (`Incomplete`, `Delete`, and `Closed` when `offering_type` is uncertain)
  keep the raw value in `extras.reso_local`. `source_status` remains the
  native `MlsStatus` verbatim.
- `Listing` docstring derivation rules updated for renamed fields (same rules).

## Sale side

### SaleEvent changes

- `sale_date`→`close_date` (required; RESO `CloseDate` is the same real-world
  fact; annotation is `close_mappings`, not exact — reconciled layer).
- `sale_price` keeps its name; remove `exact_mappings: [reso:ClosePrice]`
  (crosswalk: transformed/reconciled mapping).
- `financing`→`buyer_financing`, range becomes `CodeableConcept`.
- `concessions`→`concessions_amount` (`Money`); add `concessions`
  (multivalued `CodeableConcept`) and `concessions_comments` (string).
- New `listings: {range: SaleListingRelationship, multivalued: true, inlined: true, inlined_as_list: true}`.

```yaml
SaleListingRelationship:
  attributes:
    listing: {range: Listing, inlined: false, required: true}
    relationship_type: {required: true}   # resulted_in_sale | prior_listing | other (open)
    extras: {range: Any}
```

Off-market sales: empty/absent list. The vocabulary stays sale-oriented:
listing-to-listing relations (relist chains) are reconstructed from listing
identifiers/events, and "this MLS reported the sale" is what a
`SaleEvidence` row with `listing` set expresses — neither is a
sale-to-listing relationship type.

### SaleEvidence (new entity)

```yaml
SaleEvidence:
  is_a: Entity
  description: One source's claims about a market sale. SaleEvent holds the reconciliation; evidence holds the disagreeing inputs.
  attributes:
    sale: {range: SaleEvent, inlined: false, required: true}
    listing: {range: Listing, inlined: false}
    transfer: {range: Transfer, inlined: false}
    close_date: {range: date}          # RESO CloseDate — exact mapping lives here
    close_price: {range: Money}        # RESO ClosePrice — exact mapping lives here
    concessions_amount: {range: Money}
    document_number: {}
    verification_method: {range: CodeableConcept}  # mls_record | deed | assessor | broker_confirmed | appraiser_verified | ... (open)
    remarks: {}
    artifacts: {range: SourceArtifact, multivalued: true, inlined: false}
    provenance: {range: Provenance, required: true}  # override: required here (Entity has it optional)
```

One evidence row = one source = one provenance — enforced by the required
override, since the entity is meaningless without its source.

Scope note: evidence carries the claims sources most often disagree on
(close date, price, concessions amount, document number). Source-specific
financing/sale-type/party claims are out of scope this slice; when needed
they ride in `extras` and may be promoted later.

Single-storage-path rule for the MLS close claim: the listing's `closed`
event is the listing-history record; an MLS-sourced `SaleEvidence` row exists
only when a `SaleEvent` reconciliation references that claim, and when such a
row links a `listing`, its `close_date`/`close_price` should equal that
listing's closed-event values. Documented convention, deliberately unenforced
(consistent with the capture envelopes' status/field-pairing stance);
consumers treat the evidence row as authoritative for reconciliation input.

### Value-classification annotations (schema metadata only, no wire change)

- `SaleEvent.sale_price`: `reconciled`
- `SaleEvidence.close_price` etc.: `asserted`
- `price_per_area`, `price_per_unit`, `cap_rate`: `derived`

Expressed as LinkML `annotations: {value_classification: ...}`.

## Media — SourceArtifact extensions

- `order: {range: integer, minimum_value: 0}` (RESO Media `Order`)
- `short_description: {}` (RESO `ShortDescription`)
- `source_modified_at: {range: datetime}` (RESO Media `ModificationTimestamp`)
- `captured_on`→`captured_at` (rename wave)
- `kind` (existing open `CodeableConcept`) carries RESO `MediaCategory`
  values (photo, floor_plan, virtual_tour, document, …) — value map in
  crosswalk.

No new MediaAsset entity.

Documented limitation: `order`/`short_description` are asset-level here; an
artifact shared by multiple listings cannot carry per-listing presentation
order. Acceptable for this slice (profiles bundle artifacts per document); if
shared-artifact presentation becomes real, a listing-media association object
is the follow-up, not a rework of `SourceArtifact`. RESO Media
permission/dimension/deleted-media metadata is a documented loss (rides in
`extras`).

## MlsObservation (capture.yaml)

Third instance of the capture-envelope pattern:

```yaml
MlsObservation:
  attributes:
    status: {range: MlsObservationStatus, required: true}  # success | not_found | api_error | parse_error | ambiguous
    query: {}
    source_record_key: {}                                  # feed ListingKey
    original_entry_at: {range: datetime}                   # RESO OriginalEntryTimestamp
    source_modified_at: {range: datetime}                  # RESO ModificationTimestamp
    profile: {range: PropertyProfile, inlined: true}
    error: {}
    provenance: {range: Provenance, required: true}
    artifact_refs: {range: SourceArtifact, multivalued: true, inlined: false}
    extras: {range: Any}
```

Same convention as siblings: status/field pairing by convention, not enforced.
RESO local fields: `extras.reso_local.<SourceFieldName>` (documented
namespace, no schema change).

## Provenance source chain

`Provenance` gains: `originating_system`, `source_system` (open strings; RESO
`OriginatingSystemName`/`SourceSystemName`), `source_record_id`,
`source_created_at: datetime`, `source_modified_at: datetime`.
`provider` and `retrieved_at` unchanged.

Scoping: `source_record_id`/`source_created_at`/`source_modified_at` describe
the **immediate source's** record; `originating_system` names where the
record was born. This is a two-point lineage, not a full multi-hop chain —
sufficient for the aggregator-republishes-MLS case; deeper syndication chains
are a documented limitation. Where `MlsObservation` and its `provenance`
both carry source clocks, they describe the same feed record and should
agree; the envelope fields exist so the clocks survive when the profile is
detached from the envelope.

## Profile wiring

`profiles.yaml`: `PropertyProfile` gains a `sale_evidence` collection slot
(multivalued `SaleEvidence`, inlined as list), alongside the existing sales
and listings collections. `MlsObservation` lives in `capture.yaml` beside the
other envelopes and is not part of `PropertyProfile`.

The new non-inlined references (`SaleEvidence.sale`, `SaleEvidence.listing`,
`SaleListingRelationship.listing`) require registering `SaleEvent → sales`
and `Listing → listings` in `tools/profile_validation.py`'s
`REFERENCE_TARGET_COLLECTIONS` (the validator fails closed on unregistered
targets), with resolution, duplicate-target, and non-cascading-error tests
following the existing patterns.

Property-coherence semantic check: a `SaleEvidence` row's `sale`, `listing`,
and `transfer` must all resolve to records whose `property` matches the
sale's `property`; same for `SaleListingRelationship.listing`. (Extends the
existing cross-reference consistency checks.)

## Crosswalk doc

`docs/crosswalks/reso-dd20-alignment.md`, following the UAD doc's format:
mapping-model prose, then tables with columns
**PHDS | RESO DD 2.0 | strength (exact/close/transformed) | direction | notes/loss**.
Covers: listing header, identifiers, events + flat-record derivations, sale,
evidence, media, provenance, and the deliberate naming divergences. Includes
a **complete DD 2.0 StandardStatus matrix** (every value incl. `Incomplete`,
`Delete`, `Closed`-sale vs `Closed`-lease, and the `MlsStatus`-disagrees
case) and the loss ledger for the enumerated supported subset. RESO numeric
price fields map to PHDS `Money` as **transformed** mappings (never exact):
`Money` requires an ISO 4217 currency the flat record doesn't carry, so the
crosswalk documents the currency-context rule (feed/system-level currency
declaration).

## Fixtures & tests

Follow existing repo patterns (`examples/`, `counter_examples/schema/`,
`counter_examples/semantic/`, pytest suites):

Positive fixtures:
- Closed residential listing with full event history
- Active listing (incl. a range-priced case using `list_price_low`)
- Lease listing (list_price + rent_period) closing as `leased`
- Relist with price-change events and a second identifier namespace
- Sale with disagreeing MLS + deed evidence rows
- Off-market sale (no listing relationships)
- `MlsObservation` success envelope — capture fixtures live in a dedicated
  directory (e.g. `examples/capture/`) and are validated explicitly as
  `MlsObservation`; the existing `examples/PropertyProfile-*` harness and
  `just validate` glob don't pick them up, so the plan adds a capture
  validation path

Counter-examples (each failing for exactly one intended reason, per the
existing exact-issue precedent):
- `effective_at` not on `effective_date` (semantic)
- `SaleEvidence.artifacts` referencing an artifact ID absent from the
  profile's artifact bundle (semantic, matching the existing
  unresolved-artifact counter-example pattern)
- `SaleEvidence.sale` referencing a SaleEvent absent from the profile (semantic)
- `SaleEvidence` missing `provenance` (schema)
- Property-coherence violation: evidence `listing` resolving to a listing on
  a different property (semantic)
- Blank `ListingIdentifier.value` (schema)
- Duplicate `(scheme, namespace, value)` identifier (semantic)
- Two `is_primary` identifiers in one namespace (semantic)
- `close_price` on a non-closed event (semantic)
- Lease-listing `list_price` without `rent_period` on a `for_lease` listing's
  priced event (semantic)

Also: a generated-contract test asserting `value_classification` annotations
survive into schema metadata (wire-format tests alone can't detect their
loss), plus the rename-census test from the naming section.

All generators re-run via existing `justfile` targets so JSON Schema, Pydantic,
TypeScript, Rust, and Elixir artifacts stay in sync. Existing fixtures,
examples, docs, and tests updated for the rename wave.

## Impact / migration notes

Breaking changes are confined to uncommitted v0.2: `_on`→`_date`/`_at` renames,
`_kind`→`_type` renames, `Listing.kind`→`offering_type`, `mls_number` and
`listing_type` deleted (replaced), `ListingEvent.asking_price`→`list_price`,
`SaleEvent.sale_date`→`close_date`, `financing`/`concessions` renames.
No deprecation shims needed pre-release; the crosswalk and this spec are the
migration record. The UAD crosswalk (`docs/crosswalks/uad36-alignment.md`)
and other current docs are updated for renamed fields (e.g.
`SaleEvent.concessions`→`concessions_amount`); historical specs/plans under
`docs/superpowers/` are left as written.

## Independent review

Reviewed by Codex (gpt-5 via MCP, read-only) against the spec and schema
source on 2026-07-18. Accepted findings folded in above: no fabricated event
dates on import; lossless StandardStatus preservation rules; identifier shape
aligned to `PropertyIdentifier` with namespace; narrowed loss-aware claim +
loss ledger; required `SaleEvidence.provenance`; single-storage-path rule for
the MLS close claim; sale-oriented relationship vocabulary;
`REFERENCE_TARGET_COLLECTIONS` registrations + property-coherence checks;
normative rename table + census test; capture-fixture harness placement;
`Money` mappings as transformed; timezone rule for `effective_at`; event
ordering; media/provenance documented limitations; expanded fixture matrix.
Deferred (with rationale in place): participant Member/Office identity and
dated participant assignments (Party slice), contingency detail, evidence
party/financing claims, multi-hop provenance chains, listing-media
association object.
