# UAD 3.6 / MISMO Analysis for PHDS

*2026-07-11. Agent analysis of the GSE UAD 3.6 XSD bundle (`docs/references/uad/schema/GSE_UAD_3.6.0_v1.3/`) — the public MISMO v3.6 surface — against the PHDS draft schema.*

**Sources verified:** `GSE_UAD_3.6.0_ComplexTypes_v1.3.xsd` (~330 complexTypes), `GSE_UAD_3.6.0_EnumeratedTypes_v1.3.xsd` (182 enum types), `GSE_UAD_3.6.0_DataTypes_v1.3.xsd`, plus `uad-condition-quality-ratings-definitions.pdf` (C1–C6/Q1–Q6 defined at Overall, Exterior, and Interior levels).

⚠️ **File note:** `uad_appendix_d_field_specific_standardization_requirements.pdf` is actually the **legacy UAD 2.x forms-era** standardization doc — useful only for legacy-form semantics; the XSD is authoritative for 3.6.

⚠️ **Spec correction:** UAD 3.6 **drops `GrossLivingAreaSquareFeetNumber`** (0 hits in XSD) in favor of the `…StandardAboveGradeFinishedAreaMeasure` family. The design spec's example mapping is UAD 2.x vocabulary — keep as *legacy* mapping, add the 3.6 name.

**Container skeleton:** `MESSAGE → DEAL → SERVICES → SERVICE → VALUATION → VALUATION_RESPONSE → VALUATION_ANALYSES → VALUATION_ANALYSIS → {PROPERTIES → PROPERTY, VALUATION_REPORT}`. Under `PROPERTY`: `IMPROVEMENTS → IMPROVEMENT → {STRUCTURE, INTERIOR, SYSTEM, PROPERTY_UNITS → PROPERTY_UNIT → {LEVELS, ROOMS, INTERIOR_COMPONENTS, PROPERTY_UNIT_AREA}}`, plus `SITE`, `CAR_STORAGES`, `MARKET`, `PROJECT`, `PROPERTY_TAXES`, `SALES_HISTORIES`, `SALES_CONTRACTS`, `LISTING_INFORMATIONS`, `ENERGY_EFFICIENCY_AND_GREEN`, `HOMEOWNERS_ASSOCIATIONS`, `ENCUMBRANCES`, `ENVIRONMENTAL_CONDITIONS`. Key 3.6 shift: **ratings and rooms/areas are per-unit and per-level**.

## 1. UAD-aligned `residential_details` proposal (~40 fields)

1:1 with `structures` (⊙ = per-unit in UAD; flattens cleanly for 1-unit, repeats on `spaces` for 2–4 unit/ADU). All enums close with `other`.

**Room counts**

| PHDS field | Type | Source (UAD 3.6) |
|---|---|---|
| `bedrooms_total` ⊙ | int | `BedroomCount` (PROPERTY_UNIT_DETAIL) |
| `bathrooms_full` ⊙ | int | `FullBathroomCount` (no ¾-bath concept in UAD; full/half only) |
| `bathrooms_half` ⊙ | int | `HalfBathroomCount` |
| `rooms_total` ⊙ | int | `TotalRoomCount` (INTERIOR_ROOM_SUMMARY) |
| `room_types` | enum[] | `RoomBase`: Bedroom, BreakfastRoom, Den, DiningRoom, FamilyRoom, FullBathroom, HalfBathroom, Kitchen, LaundryRoom, LivingRoom, Loft, MediaRoom, Mudroom, RecreationRoom, Sunroom, UtilityRoom, WalkInPantry, Workshop, Other |

**Areas** (UAD unit enum `AreaUnitOfMeasureBase`: SquareFeet/SquareMeters/Acres/Hectares — matches our `area_unit` exactly)

| PHDS field | Source |
|---|---|
| `living_area` (exists) | ⊙ `UnitStandardAboveGradeFinishedAreaMeasure`; whole-property: `TotalFinishedAreaOfAllLivingUnitsIncludingADUAreaMeasure` |
| `finished_area_below_grade` | ⊙ `UnitStandardBelowGradeFinishedAreaMeasure` (+ `UnitNonStandardBelowGradeFinishedAreaMeasure`) |
| `unfinished_area_below_grade` | ⊙ `UnitBelowGradeUnfinishedAreaMeasure` |
| `structure_gross_area` (≈ `gross_area`) | `StructureAreaMeasure`; also `StructureExcludingVehicleStorageAndADUFinishedAreaMeasure`/`…UnfinishedAreaMeasure` |
| `garage_area` (exists) | `CarStorageAreaMeasure` |

**Condition / quality** — rated at three levels

| PHDS field | Source |
|---|---|
| `condition_rating` | `OverallConditionRatingCode` ← `OverallConditionRatingBase` **C1–C6** |
| `quality_rating` | `OverallQualityRatingCode` ← `OverallQualityRatingBase` **Q1–Q6** |
| `exterior_condition_rating` / `exterior_quality_rating` | `ExteriorConditionRatingCode`/`ExteriorQualityRatingCode` (STRUCTURE_DETAIL) |
| `interior_condition_rating` / `interior_quality_rating` ⊙ | `InteriorConditionRatingCode`/`InteriorQualityRatingCode` (PROPERTY_UNIT_DETAIL) |
| `update_status` | `RoomUpdateStatusBase`: FullyUpdated/PartiallyUpdated/NotUpdated + `RoomUpdatedTimeframeBase`: LessThanOneYear/OneToFiveYears/FiveToTenYears/TenOrMoreYears |

**Construction & form**

| PHDS field | Source |
|---|---|
| `year_built` (exists) | `PropertyStructureBuiltYear` |
| `year_built_estimated` bool | `PropertyStructureBuiltYearEstimatedIndicator` |
| `effective_age_years` | `EffectiveAgeYearsCount` (+ low/high range) |
| `remaining_economic_life_years` | `EstimatedRemainingEconomicLifeYearsCount` |
| `stories` (exists) | `StoriesCount` |
| `attachment` | `AttachmentBase`: Attached/Detached; refine with `StructuralDesignBase`: RowhouseTownhouse, SemiDetached, Lowrise, Midrise, Highrise + `RowhouseTownhouseLocationType`/`EndUnitIndicator` |
| `architectural_design` | `ArchitecturalDesignCategoryBase` (30: Ranch, CapeCod, Colonial, Craftsman, Victorian, Contemporary, SplitLevel, AFrame, …) |
| `construction_method` | `ConstructionMethodBase`: SiteBuilt, Manufactured, Modular, OnFrameModular, Container, ThreeDimensionalPrintingTechnology, Other |
| `construction_status` | `ConstructionStatusBase`: Complete/Proposed/UnderConstruction |
| `living_unit_count` | `LivingUnitCount` / `LivingUnitExcludingADUCount` |

**Exterior / systems**

| PHDS field | Source |
|---|---|
| `exterior_wall_material` | `WallMaterialBase` (18: Brick, Stucco, Vinyl, Wood, CementBoard, Log, Adobe, …) |
| `roof_material` | `RoofMaterialBase` (15: Asphalt, Metal, Slate, ClayTile, …, SolarShingles) |
| `roof_age_range` | `RoofEstimatedAgeRangeBase`: LessThanOneYear/OneToTenYears/TenToTwentyYears/TwentyOrMoreYears |
| `foundation_type` (exists) | `FoundationBase`: Basement, CrawlSpace, Slab, PostAndPier, Runner, Other |
| `foundation_material` | `FoundationMaterialBase`: PouredConcrete, ConcreteBlock, Brick, Stone, Wood, Metal, Other |
| `heating_type` (exists) | `HeatingSystemBase`: ForcedWarmAir, Baseboard, Radiant, Radiators, MiniSplit, Stove, Fireplace, GravityAir, PassiveSolar, None, Other |
| `heating_fuel` | `HeatingFuelBase`: NaturalGas, Electric, Oil, Propane, Geothermal, Solar, Wood, Coal, Other |
| `cooling_type` (exists) | `CoolingSystemBase`: Centralized/Individual/Other |
| `sewer_type` / `water_type` (exist) | `SewerSystemBase`: Septic/Cesspool/Other; `WaterSourceBase`: Well/Cistern/LakeOrSpring/HauledWaterHoldingTank/Other; public utilities via SITE_UTILITY (`UtilityBase` + `SiteUtilityOwnershipType`) |

**Garage / carport, basement, attic**

| PHDS field | Source |
|---|---|
| `car_storage_type` | `CarStorageBase`: Garage, Carport, CommonCarport, ParkingGarage, Driveway, SharedDriveway, OpenLot, None, Other |
| `car_storage_attachment` | `CarStorageAttachmentBase`: Attached/Detached/BuiltIn |
| `parking_spaces` | `ParkingSpacesCount` |
| `basement_exterior_access` | `BelowGradeExteriorAccessBase`: WalkOut/WalkUp/CellarDoor/Other (LEVEL) |
| `basement_grade` | `GradeLevelBase`: AboveGrade/PartiallyBelowGrade/FullyBelowGrade (LEVEL) |
| `has_attic` | `AtticExistsIndicator` |

**Amenities, view/location, energy, ADU**

| PHDS field | Source |
|---|---|
| `amenities` (extend `features[]`) | `AmenityBase` (55: IngroundPool, IndoorFireplace, Deck, Patio, Porch, Fence, Elevator, GatedCommunity, Sauna, SmartHomeSystem, EVChargingStation, …) + `AmenityAreaMeasure`, `AmenityCount` |
| `pool_features` | `SwimmingPoolFeatureBase`: Heated/Indoor/Caged/Other |
| `waterfront_features` | `WaterfrontFeatureBase`: Dock, Pier, Beach, BoatSlip, BoatLift, BoatRamp, SeawallOrBulkhead, Riprap, None, Other + BODY_OF_WATER |
| `view_type` / `view_range` | `ViewBase` (29: Ocean, Lake, Mountain, GolfCourse, Skyline, Woods, Industrial, …); `ViewRangeBase`: Full/Partial/Seasonal/Other; `ViewPrimaryIndicator`, `ValueMarketabilityImpactType` |
| `site_influences` | `SiteInfluenceBase` (21: BusyRoadway, RailLine, Airport, CommercialArea, GolfCourse, Park, OverheadElectricPowerTransmissionLine, …) |
| `accessibility_features` | `AccessibilityFeatureBase`: Ramps, GrabBars, Doorways, Handrails, …, None/Other |
| `disaster_mitigation_features` | `DisasterMitigationFeatureBase`: FortifiedRoof, StormShutters, FloodVents, ImpactResistantGlass, StormShelter, … |
| `renewable_energy_components` | `RenewableEnergyComponentBase`: Solar/Geothermal/WindTurbine/Other + `…OwnershipType` (owned/leased matters for value) + `…FinancedIndicator` |
| `green_certifications` | GREEN_BUILDING_CERTIFICATION: name, level, year |
| `has_adu` (exists) | `AccessoryDwellingUnitIndicator`; `AccessoryDwellingUnitTotalCount` |
| `adu_legally_rentable` | `AccessoryDwellingUnitLegallyRentableIndicator` |
| `occupancy` ⊙ | `UnitOccupancyBase`: OwnerOccupied/Tenant/Vacant |

## 2. Gap analysis vs the draft

### (a) UAD/MISMO concepts we lack or under-model

| Concept | Verdict |
|---|---|
| Approach-level valuation detail (see §3) | **v1**: applicability/exclusion flags + cost-approach land value on `valuations`; full detail → appraisal extension |
| Reconciliation (`OpinionOfValueAmount`, `MarketingOrExposureDaysCount`) | v1 partial: `exposure_days` cheap+useful; rest → extension |
| Comparables & adjustment grids (COMPARABLE_ADJUSTMENT, weights, proximity) | Appraisal **extension** (work product, not property fact) |
| Market condition analysis (MARKET_INVENTORY, MARKET_TREND) | Analytics **extension** |
| Project / condo / co-op (PROJECT_DETAIL: `ProjectLegalStructureBase` Condominium/Condop/Cooperative, unit counts, `ProjectCommercialSpacePercent`, co-op shares) | **v1 (light)**: `project_legal_structure` + minimal fields; full project entity → extension. Condos too common for zero footprint |
| HOA / association charges | Named **extension** (as spec already says) |
| Energy efficiency & green | v1: `renewable_energy_components` + ownership; certifications → extension |
| Estate/interest & ground rent (`PropertyEstateBase` FeeSimple/Leasehold, PROPERTY_GROUND_RENT) | **v1**: `estate_type` on property (we only have `interest` on valuations); ground-rent detail → extension |
| Site encumbrances (EASEMENT, ENCROACHMENT, `RestrictionBase`: Age/HistoricPreservation/Income/LandUse/Rental/SalePrice) | v1 (light): `sites.restrictions[]`/`easements[]` open vocab; detail → extension |
| Hazard zones beyond flood (`HazardZoneBase`, AIRPORT_ZONE, manufactured wind/thermal/roof-load zones). UAD has **no** granular FEMA zone — ours is richer | v1: keep `flood_zone`, consider `hazard_zones[]`; environmental conditions (Radon, UST, Superfund) → extension |
| **View & site influences** | **v1**: draft has no view fields at all; core comp vocabulary |
| Unit/level/room hierarchy | v1: per-unit residential fields on `spaces` (add occupancy, ADU flag, unit areas); level/room granularity → extension |
| Listing detail (`InitialListPriceAmount` vs `FinalListPriceAmount`, `DaysOnMarketCount`, `ListingType` MLS/FSBO/Auction) | **v1**: cheap adds to `listings` |
| Rent schedule / rent control (`UnitMonthlyActualRentAmount` vs `…MarketRentAmount`, `PropertyRentControlStatusType`) | v1: `unit_rent_observations.rate_type` nearly covers; add `rent_control_status` |
| Defects & inspections | Extension |
| Manufactured home detail (HUD plate/serial) | Extension; v1 = `construction_method = manufactured` |
| Highest & best use | Out of scope |
| Appraiser/licensing/scope-of-work/ROV/certifications | Out of scope (mortgage workflow) |
| Sales contract detail (`ArmsLengthIndicator`, `SalesConcessionAmount`) | v1 mostly covered by `sale_events` |

**Independent validations:** UAD SALES_HISTORY distinguishes `OwnershipTransferTransactionBase` = **DeedTransferOnly | Sale** — direct validation of our transfers/sale_events split. PARCEL_IDENTIFICATION + `NonContiguousParcelSeparator` validates multi-parcel properties.

### (b) Our entities with no UAD counterpart

- **Confirmed absent:** `loans` (no recorded-mortgage entity), `liens`, `foreclosure_records`, `tax_bills` (PROPERTY_TAX has no billed/paid amounts), `assessments` (no assessed land/improvement values anywhere), `owners`/`ownership_periods`, `jurisdictions`, `parcel_lineage`, `addresses` as entity.
- **Correction:** `unit_rent_observations`/`lease_events` are NOT absent — UAD has RENTAL_INFORMATION / UNIT_RENT_SCHEDULE (actual vs market monthly rent, lease start, furnished, concessions) and RENTAL_COMPARISON. It lacks commercial lease structure (no lease types, escalations, TI). Record crosswalks (e.g., `unit_rent_observations.rate ← UnitMonthlyActualRentAmount`).

## 3. Valuation approaches & reconciliation in the XSD

`VALUATION_REPORT → APPROACH_TO_VALUE → {SALES_COMPARISON_APPROACH, COST_APPROACH, INCOME_APPROACH}` + `VALUATION_RECONCILIATION`:

- **Exclusion mechanism per approach:** `ValuationApproachExclusionReasonBase`: InsufficientData, LackOfLandSales, DifficultyEstimatingDepreciation, NotNecessaryForCredibleResults, Other. SCOPE_OF_WORK_DETAIL carries applicability flags: `SalesComparisonApproachIndicator`, `CostApproachIndicator`, `IncomeApproachIndicator`, `RentScheduleIndicator`, `SiteValueIndicator`.
- **Sales comparison:** `ValueIndicatedBySalesComparisonAmount`; comps in COMPARABLE/COMPARABLE_ADJUSTMENT.
- **Cost:** `ValueIndicatedByCostApproachAmount`, **`SiteEstimatedValueAmount` (land value)** + `SiteValuationMethodType` (Allocation/Extraction/SalesComparison), `CostAnalysisType` (Replacement/Reproduction), `CostMethodType`, `DepreciationMethodType`, IMPROVEMENT_DEPRECIATION (physical/functional/external amounts+percents), REPLACEMENT_REPRODUCTION_COST (`PricePerAreaAmount`).
- **Income:** `ValueIndicatedByIncomeApproachAmount`, `GrossRentMultiplierFactorNumber` (residential income = GRM, not cap-rate DCF).
- **Reconciliation:** `OpinionOfValueAmount` (final), `AppraisalReportEffectiveDate` (= our `as_of_date`), `PropertyAsIsConditionRatingCode`, `MarketingOrExposureDaysCount` (+range); VALUATION_CONDITION → `PropertyValuationConditionalConclusionType` (≈ our `value_premise`) + SUBJECT_TO_COMPLETION_ITEMS.

**Verdict on `valuations`:** `indicated_value_*` + `value_premise` map cleanly. Misses to consider for v1: (1) per-approach applicability/exclusion status; (2) **`land_value`** ← `SiteEstimatedValueAmount`; (3) `exposure_days`; (4) `valuation_method` ← `PropertyValuationMethodBase`: Desktop/Exterior/Hybrid/Traditional (distinct from `kind`); (5) optionally `gross_rent_multiplier`. Depreciation/cost-method detail → appraisal extension.

## 4. Commercial note

Confirmed residential-only (ImprovementBase = Dwelling/Outbuilding; GRM income analysis; condo/co-op projects). For the commercial analog: **BOMA measurement standards** (rentable/usable area — the commercial counterpart of ANSI Z765/UAD GLA semantics), **OSCRE IDM** (CRE industry data model — spaces, leases, valuations), secondarily MISMO's commercial workgroup specs and Appraisal Institute conventions. CRE fields should cite BOMA/OSCRE definitions in the glossary the same way residential fields cite UAD 3.6.

---

## Final alignment verification (2026-07-11)

Machine-checked against the LinkML schema before removal of the UAD reference
downloads: **25/25 v1-verdict items present** (estate type; view/site-influence/
easement/restriction/hazard vocabularies; renewable energy + ADU flags; per-unit
occupancy; initial-vs-final list price + listing type; valuation land value,
exposure days, method, three approach values, premise; construction method/
status; year-built-estimated; foundation material; heating fuel; rating-system
discriminator; architectural design; basement type; three-quarter baths;
transfer-vs-sale split). Consciously deferred to the UAD profile / extensions:
room_types[], update_status, roof_age_range, effective_age /
remaining_economic_life, green certifications, rent_control_status,
project_legal_structure (open item). UAD 3.6 source materials are public
downloads from Fannie Mae / Freddie Mac (singlefamily.fanniemae.com,
sf.freddiemac.com — spec, Appendix A/D, condition-quality definitions, the
GSE_UAD_3.6.0 XSD bundle, sample scenarios); not redistributed here.
