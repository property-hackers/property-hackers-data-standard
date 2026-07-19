# RESO Data Dictionary 2.0 alignment

Scope: how PHDS v0.2 represents RESO DD 2.0 listing and sale concepts.
Target version: DD 2.0 (active certification version). DD 2.1 is tracked
separately. PHDS remains internationally neutral; RESO vocabulary enters
through mappings and import derivations, never as core defaults.

Parcel- and structure-level RESO fields (for example `ParcelNumber`,
`LivingArea`, `YearBuilt`, `BedroomsTotal`) are out of scope here; the
structure-level physical characteristics are additionally covered by
[`docs/crosswalks/uad36-alignment.md`](uad36-alignment.md), and all of
these attributes carry their own `reso:` mapping annotations directly in
`schema/entities.yaml`. One parcel-level exception is noted
here because it is an identifier, not a characteristic: RESO's Universal
Parcel Identifier (UPI 2.0, `urn:reso:upi:2.0:<Country>:<CountrySubdivision>:<ParcelNumber>[:sub:<ParcelSubcomponent>]`)
maps to a `ParcelIdentifier` row with scheme `upi`, the URN stored
verbatim as the value — PHDS has no dedicated UPI field. UPI 2.0's
data-fidelity rule (parcel number raw as issued, including case, dashes,
and spaces) is the same rule `Parcel.parcel_number` follows; a UPI `:sub:`
subcomponent corresponds to `Parcel.unit_designator` when modeled.

## Mapping model

PHDS listings derive lifecycle state from ListingEvent streams; RESO's
Property resource is a flat latest-state record. Import therefore
synthesizes events from dated RESO fields and never fabricates dates:
undated claims stay in producer-namespaced `extras` keys. Strength column: exact =
same meaning and representation; close = same meaning, PHDS wraps or
narrows; related = associated but deliberately distinct concepts;
transformed = structural or vocabulary conversion documented here.
Exact, close, and related strengths are also carried as machine-readable
LinkML mapping annotations on the schema slots, queryable through
`tools/standard_mappings.py`. `tests/test_standard_mappings.py` pins the
full annotation census, asserts every annotated RESO term is documented
in this file, and exercises a simulated RESO feed import (listing → sale)
against the annotations; per-row strength claims in the tables are
maintained against that census by review.

RESO numeric prices map to PHDS Money as close or transformed, never
exact: Money wraps the raw numeric value and adds an ISO 4217 currency
supplied by feed-level context, not carried by the source field itself.
Which layer a price claim belongs to is stated by the carrying entity and
its value_classification annotation (asserted on SaleEvidence rows,
reconciled on SaleEvent.sale_price), not by mapping strength.

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
| Listing.marketing_channel | (no direct RESO field) | n/a | import | PHDS-only dimension; see note below |
| Listing.exposure_type | (MlsStatus/office exclusive concepts) | transformed | import | PHDS-only dimension; see note below |
| Listing.special_listing_conditions | SpecialListingConditions | close | both | CodeableConcept list |

`marketing_channel` and `exposure_type` are PHDS-only dimensions with no
single RESO DD 2.0 field equivalent. They replace the retired PHDS v0.1
`listing_type` enumeration, whose values now map onto other slots: `mls` →
`marketing_channel`, `fsbo` → `marketing_channel: owner_direct`, `pocket` →
`exposure_type: office_exclusive` / `exposure_type: private`, `auction` →
`special_listing_conditions`.

## Listing events (flat-record derivations)

| RESO DD 2.0 | PHDS destination | Strength | Notes |
|---|---|---|---|
| OnMarketDate | listed event effective_date | transformed | |
| OriginalListPrice | earliest event list_price | transformed | only when a dated event can carry it; else producer-namespaced extras |
| ListPrice | latest event list_price | close | |
| ListPriceLow | event list_price_low | close | |
| PreviousListPrice | derived from event stream | transformed | undated → producer-namespaced extras, never a fabricated event |
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
| Closed | sold or leased | via offering_type; raw value preserved in extras when uncertain |
| Canceled | canceled | |
| Expired | expired | |
| Withdrawn | withdrawn | |
| Incomplete | other | raw value preserved in extras |
| Delete | other | record-management state; raw value preserved in extras |

MlsStatus always lands verbatim in source_status; disagreement between
MlsStatus and StandardStatus is preserved, not resolved. This matrix is
carried as canonical data in `tools/reso_status_matrix.py`;
`tests/test_standard_mappings.py` keeps the data and this table in sync.

## Sale and evidence

| PHDS | RESO DD 2.0 | Strength | Direction | Notes |
|---|---|---|---|---|
| SaleEvent.close_date | CloseDate | close | import | reconciled layer |
| SaleEvent.sale_price | ClosePrice | related | import | reconciled conclusion, deliberately NOT named close_price |
| SaleEvidence.close_date | CloseDate | exact | import | the source's claim |
| SaleEvidence.close_price | ClosePrice | close | import | the source's claim; currency from feed context |
| SaleEvidence.concessions_amount | ConcessionsAmount | close | import | the source's claim; currency from feed context |
| SaleEvent.buyer_financing | BuyerFinancing | close | both | CodeableConcept |
| SaleEvent.concessions_amount | ConcessionsAmount | close | both | |
| SaleEvent.concessions | Concessions | close | both | |
| SaleEvent.concessions_comments | ConcessionsComments | close | both | |
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

## Local fields

Fields the DD 2.0 dictionary defines but this alignment does not otherwise
map — including MLS-local extensions with no DD 2.0 counterpart — ride in
`extras` under producer-namespaced keys, per the general extras convention
([`docs/extensions.md`](../extensions.md)): the producer picks the
namespace and decides how to key the source fields within it, for example
`"extras": {"example-mls.lo1_board_code": "EX"}`. PHDS does not prescribe
a dedicated namespace or key form for unmapped source fields; producers
that need to recover exact source field names can record them in their own
namespace however suits their pipeline.

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
producer-namespaced extras.
Documented losses: contingency codes and removal dates; participant
Member/Office keys; media permission/dimension/deleted-record metadata;
multi-hop syndication lineage. Each rides in extras until its own slice.
