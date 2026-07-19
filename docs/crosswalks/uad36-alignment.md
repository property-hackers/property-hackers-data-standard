# UAD 3.6 alignment

This crosswalk describes how PHDS v0.2 represents concepts from the GSE
Uniform Appraisal Dataset (UAD) 3.6 and its MISMO 3.6 message model. UAD is a
United States residential-mortgage standard. PHDS remains internationally
neutral: the core schema does not import UAD, and UAD constraints apply only
through the optional UAD profile in [`schema/standards/uad_3_6.yaml`](../../schema/standards/uad_3_6.yaml).

## Mapping model

PHDS represents condition and construction quality with system-qualified
`Rating` objects. Each rating supplies its own `system`, `code`, optional
`display`, and `scope`. The optional UAD profile constrains UAD rating systems,
codes, and scopes without making them the PHDS-wide scale.

Stable `Property`, `Site`, `Structure`, and `Space` entities contain
observation-derived current facts. Sparse state records under a
`PropertyStateSnapshot` contain asserted historical facts, and
`PropertyStateSnapshot.as_of_date` is their effective date. Capture and
provenance timestamps record when information was obtained, not when the
asserted property state was effective.

UAD often places interior facts on a `PROPERTY_UNIT`. In PHDS, a rating's
`scope: interior` identifies the part being rated; it does not move the rating
to `Space`. The containing `Structure` or `StructureState` remains the subject.

## Physical-property mappings

| PHDS v0.2 | UAD 3.6 concept | Notes |
|---|---|---|
| `Structure.living_area` | `UnitStandardAboveGradeFinishedAreaMeasure` | PHDS records the value and unit; the applicable profile or provenance identifies the measurement method. |
| `Structure.gross_area` | `StructureAreaMeasure` | Broader structure area; it is distinct from living and rentable area. |
| `Structure.garage_area` | `CarStorageAreaMeasure` | Parking count remains separate. |
| `Structure.year_built` | `PropertyStructureBuiltYear` | `year_built_estimated` records whether the year is estimated. |
| `Structure.stories` | `StoriesCount` | Decimal values support partial levels where a source reports them. |
| `Structure.construction_method` | `ConstructionMethodBase` | Open PHDS vocabulary; UAD values can be carried directly when the source system is identified. |
| `Structure.construction_status` | `ConstructionStatusBase` | Open PHDS vocabulary. |
| `ResidentialDetails.bedrooms_total` | `BedroomCount` | Residential detail nested in the containing structure or structure state. |
| `ResidentialDetails.bathrooms_full` | `FullBathroomCount` | PHDS also has separate half- and three-quarter-bath counts. |
| `ResidentialDetails.bathrooms_half` | `HalfBathroomCount` | — |
| `ResidentialDetails.rooms_total` | `TotalRoomCount` | — |
| `ResidentialDetails.attachment` | `AttachmentBase` and structural-design concepts | Open PHDS vocabulary. |
| `ResidentialDetails.architectural_design` | `ArchitecturalDesignCategoryBase` | Open PHDS vocabulary. |
| `ResidentialDetails.occupancy` | `UnitOccupancyBase` | Current or historical meaning follows the containing structure context. |
| `ResidentialDetails.has_adu` | `AccessoryDwellingUnitIndicator` | Legal rentability maps to `adu_legally_rentable`. |

PHDS fields such as `foundation_type`, `foundation_material`,
`heating_types`, `heating_fuel_type`, `cooling_types`, `sewer_type`,
`water_type`, `features`, `site_influences`, `view_types`, `easements`,
`restrictions`, and `hazard_zones` carry corresponding UAD concepts as open
vocabularies. Producers identify the source vocabulary through provenance or a
constrained profile rather than assuming UAD semantics in core.

## Condition and quality ratings

| PHDS rating | UAD 3.6 source |
|---|---|
| `condition_ratings[]` with `scope: overall` | `OverallConditionRatingCode` (`C1`–`C6`) |
| `condition_ratings[]` with `scope: exterior` | `ExteriorConditionRatingCode` |
| `condition_ratings[]` with `scope: interior` | `InteriorConditionRatingCode` |
| `quality_ratings[]` with `scope: overall` | `OverallQualityRatingCode` (`Q1`–`Q6`) |
| `quality_ratings[]` with `scope: exterior` | `ExteriorQualityRatingCode` |
| `quality_ratings[]` with `scope: interior` | `InteriorQualityRatingCode` |

Semantic validation permits at most one rating for each `(system, scope)` pair
within a rating collection. The optional UAD profile also prevents condition
codes from appearing as quality ratings and quality codes from appearing as
condition ratings.

## Records, events, and valuation

| PHDS v0.2 | UAD 3.6 concept | Notes |
|---|---|---|
| `Transfer` | deed-transfer sales history | PHDS separates recorded conveyance from a comparable `SaleEvent`. |
| `SaleEvent.sale_type` | arms-length and sale-characteristic assertions | Absence means the producer did not assert a sale type. |
| `SaleEvent.concessions_amount` | sales concession amount | Exact money remains a decimal string on the wire. |
| `Listing.events` | listing-information status and price observations | Each event carries its own effective date and event-level economics. |
| `UnitRentObservation` | unit actual or market rent observation | `rate_type` distinguishes contract, market, asking, and effective rent. |
| `LeaseEvent` | lease and rental information | PHDS also models commercial lease structure not supplied by UAD. |
| `Valuation.as_of_date` | `AppraisalReportEffectiveDate` | The value's effective date. |
| `Valuation.indicated_value_sales_comparison` | sales-comparison indicated value | — |
| `Valuation.indicated_value_cost` | cost-approach indicated value | — |
| `Valuation.indicated_value_income` | income-approach indicated value | — |
| `Valuation.land_value` | `SiteEstimatedValueAmount` | — |
| `Valuation.exposure_days` | `MarketingOrExposureDaysCount` | — |
| `Valuation.value_premise` | valuation conditional conclusion | Open PHDS vocabulary. |

UAD comparable grids, detailed approach calculations, reconciliation work
papers, scope-of-work declarations, appraiser workflow, and report
certifications are appraisal work product rather than canonical property
facts. They belong in separate appraisal-oriented extensions.

## PHDS concepts outside UAD's scope

PHDS also represents canonical parties, addresses, jurisdictions, parcels,
parcel lineage, ownership periods, assessments, tax bills, recorded loans,
liens, foreclosure cases, permits, operating statements, commercial rent
rolls, source artifacts, and multi-authority identifiers. These concepts do
not need a UAD counterpart to be valid PHDS data.

Authoritative UAD definitions, schemas, and licensing terms remain with
[Fannie Mae](https://singlefamily.fanniemae.com/applications-technology/uniform-mortgage-data-program/uniform-appraisal-dataset) and
[Freddie Mac](https://sf.freddiemac.com/tools-learning/uniform-mortgage-data-program/uad).
