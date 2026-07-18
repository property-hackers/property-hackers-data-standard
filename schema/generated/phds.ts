export type EntityId = string;
export type JurisdictionId = string;
export type AddressId = string;
export type PropertyId = string;
export type ParcelId = string;
export type PropertyParcelId = string;
export type ParcelLineageId = string;
export type PropertyIdentifierId = string;
export type PartyId = string;
export type SourceArtifactId = string;
export type PropertyAddressId = string;
export type OwnershipPeriodId = string;
export type StructureId = string;
export type SiteId = string;
export type SpaceId = string;
export type PropertyStateId = string;
export type SiteStateId = string;
export type StructureStateId = string;
export type SpaceStateId = string;
export type PropertyStateSnapshotId = string;
export type PropertyAssociationId = string;
export type AssessmentId = string;
export type TaxBillId = string;
export type TransferId = string;
export type SaleEventId = string;
export type ListingId = string;
export type LeaseEventId = string;
export type UnitRentObservationId = string;
export type LoanId = string;
export type LienId = string;
export type ForeclosureCaseId = string;
export type PermitId = string;
export type OperatingStatementId = string;
export type RentRollId = string;
export type ValuationId = string;

export enum AreaUnit {

    sqft = "sqft",
    sqm = "sqm",
    acre = "acre",
    hectare = "hectare",
};

export enum LengthUnit {

    ft = "ft",
    m = "m",
};

export enum CaptureMethod {

    api = "api",
    scrape = "scrape",
    llm_extraction = "llm_extraction",
    manual = "manual",
    bulk = "bulk",
};

export enum VerificationStatus {

    unverified = "unverified",
    pending_review = "pending_review",
    verified = "verified",
    disputed = "disputed",
    rejected = "rejected",
};

export enum PartyKind {

    person = "person",
    organization = "organization",
};

export enum SaleTypeEnum {

    arms_length = "arms_length",
    /** Real-estate-owned / bank sale */
    reo = "reo",
    short_sale = "short_sale",
    auction = "auction",
    related_party = "related_party",
    portfolio = "portfolio",
    partial_interest = "partial_interest",
    land_contract = "land_contract",
    new_construction = "new_construction",
    other = "other",
};
/**
* Reliability of a recorded price
*/
export enum PriceDisclosure {

    full = "full",
    partial = "partial",
    estimated = "estimated",
    nominal = "nominal",
    non_disclosure = "non_disclosure",
    unknown = "unknown",
};

export enum LeaseTypeEnum {

    gross = "gross",
    modified_gross = "modified_gross",
    triple_net = "triple_net",
    double_net = "double_net",
    single_net = "single_net",
    absolute_net = "absolute_net",
    percentage = "percentage",
    ground = "ground",
    residential = "residential",
    other = "other",
};

export enum RentPeriod {

    daily = "daily",
    monthly = "monthly",
    annual = "annual",
    per_area_annual = "per_area_annual",
    per_area_monthly = "per_area_monthly",
};

export enum RateBasis {

    per_unit = "per_unit",
    per_bed = "per_bed",
    per_area = "per_area",
    per_room = "per_room",
    per_key = "per_key",
    per_slip = "per_slip",
    per_stall = "per_stall",
    per_pad = "per_pad",
    other = "other",
};

export enum RateType {

    asking = "asking",
    market = "market",
    effective = "effective",
    contract = "contract",
};

export enum ListingKind {

    for_sale = "for_sale",
    for_lease = "for_lease",
};

export enum ListingStatus {

    active = "active",
    pending = "pending",
    sold = "sold",
    leased = "leased",
    withdrawn = "withdrawn",
    expired = "expired",
    coming_soon = "coming_soon",
    other = "other",
};

export enum ValuationKind {

    avm = "avm",
    appraisal = "appraisal",
    bpo = "bpo",
    broker_opinion = "broker_opinion",
    internal = "internal",
};

export enum LoanEventKind {

    origination = "origination",
    assignment = "assignment",
    modification = "modification",
    satisfaction = "satisfaction",
    release = "release",
    default = "default",
    reinstatement = "reinstatement",
    other = "other",
};

export enum LienKind {

    tax = "tax",
    judgment = "judgment",
    hoa = "hoa",
    mechanics = "mechanics",
    municipal = "municipal",
    other = "other",
};

export enum ParcelLineageKind {

    split = "split",
    merge = "merge",
    renumber = "renumber",
};

export enum EstateType {

    fee_simple = "fee_simple",
    leased_fee = "leased_fee",
    leasehold = "leasehold",
    life_estate = "life_estate",
    cooperative_shares = "cooperative_shares",
    other = "other",
};

export enum RatingScope {

    overall = "overall",
    exterior = "exterior",
    interior = "interior",
    component = "component",
    other = "other",
};
/**
* What an operating statement represents.
*/
export enum StatementBasis {

    actual = "actual",
    budget = "budget",
    pro_forma = "pro_forma",
    stabilized = "stabilized",
    projected = "projected",
    other = "other",
};

export enum GeocodeAccuracy {

    rooftop = "rooftop",
    parcel = "parcel",
    street = "street",
    postal_centroid = "postal_centroid",
    locality_centroid = "locality_centroid",
    manual = "manual",
    unknown = "unknown",
};
/**
* Outcome of an extraction attempt over already-fetched content. Fetch-level failures (timeout, api_error) belong to the envelope that did the fetching, e.g. AssessorStatus.
*/
export enum ExtractionStatus {

    success = "success",
    /** Content was retrieved but extraction failed */
    parse_error = "parse_error",
    /** Content contains no extractable property data */
    irrelevant_page = "irrelevant_page",
    /** The extraction model or tooling failed */
    model_error = "model_error",
};
/**
* What kind of property content an extraction produced. This is a content axis only — where the content came from (public record, vendor, scrape) is Provenance's job. Precedence: when the primary extracted content is a transaction, listing, or lease, use that value even if the page is also a record of property facts. `other` means successfully classified but outside this taxonomy; put the producer's raw label in source_category.
*/
export enum ExtractionCategory {

    sales_transaction = "sales_transaction",
    sale_listing = "sale_listing",
    lease_listing = "lease_listing",
    in_place_lease = "in_place_lease",
    /** Property characteristics, assessment, or tax facts with no primary transaction/listing/lease content */
    property_facts = "property_facts",
    other = "other",
};
/**
* Outcome of an assessor or public-records lookup.
*/
export enum AssessorStatus {

    success = "success",
    not_found = "not_found",
    timeout = "timeout",
    api_error = "api_error",
    parse_error = "parse_error",
    invalid_address = "invalid_address",
    ambiguous = "ambiguous",
};


/**
 * Open map — the `extras` escape hatch on every entity.
 */
export interface Any {
}


/**
 * Monetary amount. JSON wire format is decimal-as-string. ISO-4217 currency.
 */
export interface Money {
    amount: string,
    /** ISO-4217 currency code */
    currency: string,
}


/**
 * Area measurement with explicit unit.
 */
export interface Area {
    value: number,
    unit: AreaUnit,
}


/**
 * Linear measurement with explicit unit.
 */
export interface Length {
    value: number,
    unit: LengthUnit,
}


/**
 * Money per explicit denominator (price/sqft, rent/sqm/month, spaces/1000 sqft).
 */
export interface UnitRate {
    amount: string,
    /** ISO-4217 currency code */
    currency: string,
    /** sqft | sqm | unit | bed | key | month | sqft_month | 1000_sqft | ... */
    denominator: string,
}


/**
 * Source coding preserved alongside canonical values (system + code + display).
 */
export interface CodeableConcept {
    system?: string,
    code?: string,
    display?: string,
}


/**
 * An open-vocabulary classification qualified by the system that defines its code. Codes from different systems are not assumed equivalent.
 */
export interface Classification extends CodeableConcept {
    system: string,
    code: string,
}


/**
 * A rating qualified by the system that defines its code. PHDS does not assume codes from different systems are ordinally or semantically equal.
 */
export interface Rating extends CodeableConcept {
    system: string,
    code: string,
    description?: string,
    scope?: RatingScope,
}


/**
 * WGS84 coordinate.
 */
export interface GeoPoint {
    latitude: number,
    longitude: number,
}


/**
 * GeoJSON Geometry object (typed-in-standard; optional everywhere).
 */
export interface Geometry {
    /** Point | MultiPolygon | ... */
    type: string,
    coordinates?: Any,
}


/**
 * Record-level source envelope. Pure child rows (parties, areas, escalations, filings) inherit the parent envelope unless they carry their own.
 */
export interface Provenance {
    provider?: string,
    source_url?: string,
    retrieved_at?: string,
    method?: CaptureMethod,
    /** Fraction from 0 through 1; 0.8 means 80 percent confidence. */
    confidence?: number,
    verification?: VerificationStatus,
}



export interface Entity {
    /** Canonical identifier; nonblank with no leading or trailing whitespace */
    id: string,
    extras?: Any,
    provenance?: Provenance,
    verifications?: VerificationAttribution[],
}


/**
 * Cross-reference to another recorded instrument (e.g., an assignment citing the original mortgage's recording number) — the target may not exist as a PHDS row.
 */
export interface InstrumentReference {
    /** re_records | corrects | releases | assigns | modifies | subordinates | refers_to */
    relationship_type?: CodeableConcept,
    document_number?: string,
    registry_reference?: string,
    recording_authority?: JurisdictionId,
    extras?: Any,
}


/**
 * This row corresponds to a document recorded/registered with an authority (county recorder, land registry, notarial protocol). Narrowly about document identity and recording — legal effect lives on the host class.
 */
export interface RecordedInstrument {
    /** Primary identifier assigned by the recording/registry authority */
    document_number?: string,
    recording_book?: string,
    recording_page?: string,
    /** Date accepted, recorded, or registered by the authority */
    recorded_on?: string,
    /** Date executed/signed as dated on the instrument */
    instrument_date?: string,
    document_type?: CodeableConcept,
    /** Authority maintaining the record (optional — parcel context is inference, not identity) */
    recording_authority?: JurisdictionId,
    /** Alternate authority-issued reference: title, dealing, folio, or notarial-act number */
    registry_reference?: string,
    related_instruments?: InstrumentReference[],
    artifacts?: SourceArtifactId[],
}


/**
 * A contextual role-bearing relationship from an event or record to a canonical Party. Exact source wording belongs in provenance or source artifacts, not in a second actor-name field. Multi-party is the norm (couples on deeds, co-borrowers).
 */
export interface TransactionParty {
    role: string,
    party: PartyId,
    sequence?: number,
    extras?: Any,
}



export interface Jurisdiction extends Entity {
    /** ISO 3166-1 alpha-2 (no default — i18n) */
    country: string,
    region?: string,
    name: string,
    /** county | municipality | taxing_district | ... (open vocab) */
    kind: string,
    /** US: FIPS/GEOID */
    authority_code?: string,
    parent?: JurisdictionId,
    boundary?: Geometry,
}



export interface Address extends Entity {
    country: string,
    /** Fallback when components do not parse */
    unformatted_address?: string,
    street_number?: string,
    street_pre_direction?: string,
    street_name?: string,
    street_suffix?: string,
    street_post_direction?: string,
    unit_type?: string,
    unit_number?: string,
    /** neighborhood / borough / barrio */
    sublocality?: string,
    city?: string,
    region?: string,
    postal_code?: string,
    postal_code_suffix?: string,
    /** county/district name */
    admin_area?: string,
    /** authority code */
    admin_area_code?: string,
    /** Producer-computed matching key; comparable only under the same scheme */
    address_hash?: string,
    /** Producer-namespaced normalization and hashing scheme */
    address_hash_scheme?: string,
    location?: GeoPoint,
    location_accuracy?: GeocodeAccuracy,
}



export interface PropertyFacts {
    name?: string,
    /** PUCS class when property_use_system identifies PUCS */
    property_use_class?: string,
    property_use_type?: string,
    property_use_subtype?: string,
    /** Required when use fields are set; no default */
    property_use_system?: string,
    estate_type?: EstateType,
    location?: GeoPoint,
    /** Producer summary when structures are not enumerated */
    building_count?: number,
}


/**
 * Stable property identity; mutable descriptive fields come from PropertyFacts.
 */
export interface Property extends Entity, PropertyFacts {
}



export interface Parcel extends Entity {
    jurisdiction: JurisdictionId,
    /** RAW as issued (RESO UPI rule) */
    parcel_number: string,
    /** Matching only, never identity */
    normalized_parcel_number?: string,
    /** Condo sub-parcel discriminator */
    unit_designator?: string,
    /** urn:reso:upi:2.0:... */
    reso_upi?: string,
    legal_description?: string,
    land_area?: Area,
    /** GeoJSON MultiPolygon (optional) */
    boundary?: Geometry,
    /** Set by lineage events */
    retired_on?: string,
}


/**
 * Property ↔ parcel, many-to-many over time (splits/merges/condos). ended_on is end-exclusive.
 */
export interface PropertyParcel extends Entity {
    property: PropertyId,
    parcel: ParcelId,
    is_primary?: boolean,
    started_on?: string,
    ended_on?: string,
}



export interface ParcelLineage extends Entity {
    predecessor_parcel: ParcelId,
    successor_parcel: ParcelId,
    kind: ParcelLineageKind,
    effective_on?: string,
}


/**
 * Namespaced external IDs — MLS keys, vendor IDs, tax account numbers.
 */
export interface PropertyIdentifier extends Entity {
    property: PropertyId,
    scheme: string,
    namespace?: string,
    value: string,
}


/**
 * One canonical model for every actor — owners, buyers, tenants and lessees, borrowers, lenders, brokers, trustees, claimants, contractors, associations, and valuation performers.
 */
export interface Party extends Entity {
    kind: PartyKind,
    /** Optional system-qualified legal form for an organization under an identified jurisdictional or producer vocabulary. This is not an industry classification or a contextual role such as lender, broker, tenant, or association. */
    legal_form?: Classification,
    /** Canonical display name for this Party in the profile; nonblank with no leading or trailing whitespace. Source-specific wording is attributed through provenance or SourceArtifact. */
    name: string,
    /** Producer-derived matching key for the canonical display name. Its normalization algorithm is producer-defined; it is not authoritative display text or a separate identity. */
    normalized_name?: string,
    /** Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems. */
    name_first?: string,
    /** Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems. */
    name_middle?: string,
    /** Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems. */
    name_last?: string,
    /** System-qualified actor classifications from an open vocabulary; use producer-namespaced system and code values */
    classifications?: Classification[],
    addresses?: PartyAddress[],
    contacts?: PartyContact[],
}


/**
 * Verification performed by one canonical party.
 */
export interface VerificationAttribution {
    verifier: PartyId,
    verified_at: string,
    note?: string,
    extras?: Any,
}


/**
 * Source material that supports or preserves an assertion. Semantic validation requires at least one nonblank uri or storage_reference.
 */
export interface SourceArtifact extends Entity {
    uri?: string,
    /** Producer-defined object or document storage reference */
    storage_reference?: string,
    /** MIME media type */
    media_type?: string,
    kind?: CodeableConcept,
    title?: string,
    original_filename?: string,
    content_hash?: string,
    /** Producer-namespaced content hashing scheme */
    hash_scheme?: string,
    page_count?: number,
    captured_on?: string,
}



export interface AddressAssociation {
    address: AddressId,
    /** situs | entrance | alias | address_range | former | mailing | other (open vocabulary) */
    role?: CodeableConcept,
    is_primary?: boolean,
    valid_from?: string,
    valid_to?: string,
}



export interface PartyAddress extends AddressAssociation {
    extras?: Any,
    provenance?: Provenance,
}



export interface PropertyAddress extends Entity, AddressAssociation {
    property: PropertyId,
}



export interface PartyContact {
    /** phone | email | website | other */
    kind: string,
    value: string,
    label?: string,
    is_primary?: boolean,
    do_not_contact?: boolean,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * One ownership regime; ended_on is end-exclusive, NULL = current.
 */
export interface OwnershipPeriod extends Entity {
    property: PropertyId,
    started_on?: string,
    ended_on?: string,
    /** joint_tenants | tenants_in_common | community_property | ... (open) */
    vesting_type?: string,
    /** Owner's mailing address during THIS period */
    mailing_address?: AddressId,
    acquired_via_transfer?: TransferId,
    disposed_via_transfer?: TransferId,
    interests?: OwnershipInterest[],
}



export interface OwnershipInterest {
    party: PartyId,
    /** 0–100 percentage points; 75 means 75 percent. */
    interest_pct?: number,
    /** owner | trustee | gp | lp */
    role?: string,
    is_owner_occupied?: boolean,
    extras?: Any,
}



export interface StructureFacts {
    /** building | barn | silo | shed | outbuilding | ... (open) */
    kind?: string,
    name?: string,
    structure_number?: string,
    /** Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance. */
    living_area?: Area,
    gross_area?: Area,
    /** Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method. */
    rentable_area?: Area,
    ground_floor_area?: Area,
    basement_area?: Area,
    basement_finished_area?: Area,
    garage_area?: Area,
    /** Long-tail area kinds; canonical kinds forbidden here (validator-enforced) */
    areas?: AreaMeasure[],
    year_built?: number,
    year_built_estimated?: boolean,
    effective_year_built?: number,
    stories?: number,
    unit_count?: number,
    /** site_built | manufactured | modular | container | 3d_printed | ... */
    construction_method?: string,
    /** complete | proposed | under_construction */
    construction_status?: string,
    construction_type?: string,
    exterior_wall_type?: string,
    roof_material_type?: string,
    roof_style_type?: string,
    foundation_type?: string,
    foundation_material?: string,
    /** Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair. */
    condition_ratings?: Rating[],
    /** Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair. */
    quality_ratings?: Rating[],
    heating_types?: string[],
    heating_fuel_type?: string,
    cooling_types?: string[],
    sewer_type?: string,
    water_type?: string,
    /** Open vocabulary for physical features and amenities */
    features?: string[],
    residential?: ResidentialDetails,
    commercial?: CommercialDetails,
    renovations?: Renovation[],
}



export interface Structure extends Entity, StructureFacts {
    property: PropertyId,
}



export interface AreaMeasure {
    kind: string,
    area: Area,
    extras?: Any,
}


/**
 * Internationally neutral residential facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
 */
export interface ResidentialDetails {
    bedrooms_total?: number,
    bathrooms_full?: number,
    bathrooms_half?: number,
    bathrooms_three_quarter?: number,
    rooms_total?: number,
    /** detached | attached | semi_detached | rowhouse_townhouse | ... */
    attachment?: string,
    /** ranch | cape_cod | colonial | craftsman | ... */
    architectural_design?: string,
    /** none | full | partial | walkout | cellar */
    basement_type?: string,
    /** garage | carport | parking_garage | driveway | none | ... */
    garage_type?: string,
    /** attached | detached | built_in */
    garage_attachment?: string,
    parking_spaces?: number,
    fireplaces?: number,
    has_pool?: boolean,
    has_attic?: boolean,
    has_adu?: boolean,
    adu_legally_rentable?: boolean,
    /** owner_occupied | tenant | vacant */
    occupancy?: string,
    /** solar | geothermal | wind */
    renewable_energy_components?: string[],
    extras?: Any,
    provenance?: Provenance,
}


/**
 * Internationally neutral commercial facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Historical/per-period financial figures belong on dated records such as OperatingStatement and UnitRentObservation. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
 */
export interface CommercialDetails {
    /** Competitive market positioning under the named rating system; distinct from physical condition and construction quality. */
    market_classification?: Rating,
    clear_height?: Length,
    dock_doors?: number,
    drive_in_doors?: number,
    /** 0–100 percentage points; 95 means 95 percent. */
    occupancy_pct?: number,
    parking_spaces?: number,
    /** e.g. spaces per 1000 sqft — denominator explicit */
    parking_ratio?: UnitRate,
    /** single_tenant | multi_tenant */
    tenancy?: string,
    /** Distinct legal tenants (not suites or leases); timing follows the containing Structure or StructureState context */
    tenant_count?: number,
    /** surface | structured | underground | covered | ... */
    parking_types?: string[],
    has_sprinkler?: boolean,
    elevators?: number,
    /** CRE market geography */
    submarket?: string,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * Repeatable events — never a flat year on the structure.
 */
export interface Renovation {
    /** kitchen | bath | roof | addition | gut | ... (open) */
    kind?: string,
    description?: string,
    completed_year?: number,
    completed_on?: string,
    cost?: Money,
    extras?: Any,
    provenance?: Provenance,
}



export interface SiteFacts {
    lot_size?: Area,
    usable_land_area?: Area,
    usable_land_area_basis?: string,
    land_use?: CodeableConcept,
    land_use_category?: string,
    zoning_code?: string,
    flood_zone?: string,
    hazard_zones?: string[],
    view_types?: string[],
    site_influences?: string[],
    easements?: string[],
    restrictions?: string[],
    utilities?: string[],
    frontage?: Length,
    depth?: Length,
    topography?: string,
    is_corner?: boolean,
    entitlement_status?: string,
    buildable_units?: number,
    subdivision?: string,
    lot_number?: string,
    block?: string,
    tract_number?: string,
    phase_number?: string,
    section_township_range?: string,
}



export interface Site extends Entity, SiteFacts {
    property: PropertyId,
}



export interface SpaceFacts {
    floor_number?: number,
    space_use?: string,
    rentable_area?: Area,
    usable_area?: Area,
    bedrooms?: number,
    bathrooms?: number,
    /** owner_occupied | tenant | vacant */
    occupancy?: string,
    is_adu?: boolean,
    is_active?: boolean,
}


/**
 * Leasable suites/units (CRE, multifamily).
 */
export interface Space extends Entity, SpaceFacts {
    property: PropertyId,
    structure?: StructureId,
    space_identifier: string,
}



export interface PropertyState extends Entity, PropertyFacts {
    subject: PropertyId,
}



export interface SiteState extends Entity, SiteFacts {
    subject: SiteId,
}



export interface StructureState extends Entity, StructureFacts {
    subject: StructureId,
}



export interface SpaceState extends Entity, SpaceFacts {
    subject: SpaceId,
}


/**
 * Sparse asserted physical state effective on as_of_date.
 */
export interface PropertyStateSnapshot extends Entity {
    property: PropertyId,
    as_of_date: string,
    /** at_sale | at_listing | at_lease | inspection | reported | inferred (open vocabulary) */
    basis?: CodeableConcept,
    property_state?: PropertyState,
    site_states?: SiteState[],
    structure_states?: StructureState[],
    space_states?: SpaceState[],
}


/**
 * HOA or property-association relationship; identity and classification live on the canonical Party.
 */
export interface PropertyAssociation extends Entity {
    property: PropertyId,
    /** The canonical association organization */
    party: PartyId,
    fee?: Money,
    fee_period?: RentPeriod,
}


/**
 * Parcel-first tax-roll values; identity = parcel + assessing jurisdiction + year + roll type.
 */
export interface Assessment extends Entity {
    parcel: ParcelId,
    /** The ASSESSING authority */
    jurisdiction: JurisdictionId,
    tax_year: number,
    /** original | corrected | supplemental | appeal | ... */
    roll_type?: string,
    assessed_land_value?: Money,
    assessed_improvement_value?: Money,
    assessed_total_value?: Money,
    market_land_value?: Money,
    market_improvement_value?: Money,
    market_total_value?: Money,
    appraised_land_value?: Money,
    appraised_improvement_value?: Money,
    appraised_total_value?: Money,
    exemptions?: TaxExemption[],
}



export interface TaxExemption {
    /** homestead | senior | veteran | ... (open) */
    kind: string,
    amount?: Money,
    extras?: Any,
}



export interface TaxBill extends Entity {
    parcel: ParcelId,
    /** Issuing authority */
    jurisdiction: JurisdictionId,
    tax_year: number,
    bill_number?: string,
    amount_billed?: Money,
    amount_paid?: Money,
    is_delinquent?: boolean,
    delinquent_year?: number,
    delinquent_amount?: Money,
    rate_code_area?: string,
    installments?: TaxInstallment[],
    line_items?: TaxLineItem[],
}



export interface TaxInstallment {
    installment_number?: number,
    due_on?: string,
    amount?: Money,
    paid_on?: string,
    amount_paid?: Money,
    is_delinquent?: boolean,
    extras?: Any,
}



export interface TaxLineItem {
    /** Optional canonical taxing jurisdiction responsible for this line item */
    jurisdiction?: JurisdictionId,
    /** Source-defined tax rate or levy value; not governed by the _pct percentage-points convention. */
    rate?: number,
    amount?: Money,
    extras?: Any,
}


/**
 * Every recorded deed/instrument — the ownership ledger, NOT comps.
 */
export interface Transfer extends Entity, RecordedInstrument {
    property: PropertyId,
    parcel?: ParcelId,
    /** warranty_deed | quitclaim | foreclosure | tax_deed | ... (open) */
    transfer_kind: string,
    /** Legal/economic effectiveness — may differ from instrument_date and recorded_on */
    effective_on?: string,
    /** Often $0 / nominal */
    consideration?: Money,
    /** Doc stamps; price-inference basis in many places */
    transfer_tax?: Money,
    price_disclosure?: PriceDisclosure,
    price_code?: CodeableConcept,
    /** The legal or beneficial interest conveyed by this transfer. */
    interest_conveyed?: CodeableConcept,
    /** 0–100 percentage points; 25 means 25 percent. */
    partial_interest_pct?: number,
    is_inter_family?: boolean,
    is_distressed?: boolean,
    parties?: TransferParty[],
}


/**
 * role = grantor | grantee
 */
export interface TransferParty extends TransactionParty {
}


/**
 * A market sale referencing its recorded transfer. A quitclaim never becomes a comp.
 */
export interface SaleEvent extends Entity {
    property: PropertyId,
    property_state?: PropertyStateSnapshotId,
    transfer?: TransferId,
    sale_date: string,
    sale_price?: Money,
    price_disclosure?: PriceDisclosure,
    price_code?: CodeableConcept,
    sale_type?: SaleTypeEnum,
    price_per_area?: UnitRate,
    price_per_unit?: Money,
    /** cash | conventional | seller | assumption | other (coarse; loans carry detail) */
    financing?: string,
    concessions?: Money,
    /** Capitalization rate in percentage points; 5.75 means 5.75 percent. */
    cap_rate?: number,
    noi_at_sale?: Money,
    opex_at_sale?: Money,
    /** 0–100 percentage points; 90 means 90 percent. */
    occupancy_at_sale_pct?: number,
    unit_count_at_sale?: number,
    /** Traceability: the statement noi_at_sale derives from, when known */
    supporting_operating_statement?: OperatingStatementId,
    parties?: SaleEventParty[],
    /** Source- or vendor-authored narrative interpreted through provenance. */
    remarks?: string,
}


/**
 * role = buyer | seller | buyer_broker | seller_broker
 */
export interface SaleEventParty extends TransactionParty {
}


/**
 * Listing identity and non-lifecycle facts. Process events by occurred_on ascending; array order breaks same-date ties. Carry status, asking_price, and rent_period forward independently from the latest event that supplies each field. Original asking terms come from the earliest event supplying them. close_price comes from the latest closed event supplying it.
 */
export interface Listing extends Entity {
    property: PropertyId,
    property_state?: PropertyStateSnapshotId,
    kind: ListingKind,
    /** mls | fsbo | auction | pocket */
    listing_type?: string,
    mls_number?: string,
    events?: ListingEvent[],
    participants?: ListingParticipant[],
    artifacts?: SourceArtifactId[],
    /** Source- or vendor-authored narrative interpreted through provenance. */
    remarks?: string,
}


/**
 * A dated listing assertion. asking_price is a sale price or periodic rent; rent_period states the period when asking_price is rent. close_price is asserted on a closed event.
 */
export interface ListingEvent {
    occurred_on: string,
    /** listed | price_change | status_change | relisted | closed */
    event_kind: string,
    status?: ListingStatus,
    asking_price?: Money,
    rent_period?: RentPeriod,
    close_price?: Money,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * role = listing_agent | co_listing_agent | buyer_agent | listing_brokerage | selling_brokerage
 */
export interface ListingParticipant extends TransactionParty {
}


/**
 * Executed leases (not asking rents — see UnitRentObservation).
 */
export interface LeaseEvent extends Entity {
    property: PropertyId,
    property_state?: PropertyStateSnapshotId,
    space?: SpaceId,
    lease_type?: LeaseTypeEnum,
    execution_date?: string,
    commencement_date?: string,
    expiration_date?: string,
    term_months?: number,
    leased_area?: Area,
    rent?: Money,
    rent_period?: RentPeriod,
    starting_rent_per_area?: UnitRate,
    effective_rent_per_area?: UnitRate,
    net_effective_rent_per_area?: UnitRate,
    free_rent_months?: number,
    ti_allowance_per_area?: UnitRate,
    expense_structure?: ExpenseStructure,
    parties?: LeaseEventParty[],
    escalations?: LeaseEscalation[],
    concessions?: LeaseConcession[],
    /** Source- or vendor-authored narrative interpreted through provenance. */
    remarks?: string,
}


/**
 * role = lessee | landlord | guarantor | lessee_broker | landlord_broker
 */
export interface LeaseEventParty extends TransactionParty {
}


/**
 * Who pays what (taxes/insurance/CAM/utilities).
 */
export interface ExpenseStructure {
    /** landlord | tenant | shared */
    taxes_paid_by?: string,
    insurance_paid_by?: string,
    cam_paid_by?: string,
    utilities_paid_by?: string,
    notes?: string,
    extras?: Any,
}



export interface LeaseEscalation {
    /** fixed_amount | fixed_percent | cpi | step_schedule | fmv | none */
    escalation_type: string,
    /** For escalation_type=fixed_percent, a value from 0-100 percentage points; 3 means 3 percent. For escalation_type=fixed_amount, the increment in the currency of the parent LeaseEvent.rent and the period specified by LeaseEvent.rent_period. */
    escalation_value?: number,
    frequency_months?: number,
    cpi_index?: string,
    /** CPI escalation floor in percentage points; 2 means 2 percent. */
    cpi_floor?: number,
    /** CPI escalation cap in percentage points; 5 means 5 percent. */
    cpi_cap?: number,
    steps?: RentStep[],
    effective_from?: string,
    effective_until?: string,
    extras?: Any,
}



export interface RentStep {
    from_date: string,
    amount: Money,
}



export interface LeaseConcession {
    /** free_rent | ti_allowance | moving_allowance | ... */
    concession_type: string,
    concession_value?: Money,
    abatement_months?: number,
    /** 0–100 percentage points; 10 means 10 percent. */
    abatement_percent?: number,
    ti_cap_total?: Money,
    conditions?: Any,
    notes?: string,
    extras?: Any,
}


/**
 * Advertised/going rates — floorplans, storage units, slips. NOT executed leases.
 */
export interface UnitRentObservation extends Entity {
    property: PropertyId,
    /** '1BR/1BA', '10x10' */
    unit_type: string,
    unit_area?: Area,
    bedrooms?: number,
    bathrooms?: number,
    /** Units of this type */
    unit_count?: number,
    units_available?: number,
    rate: Money,
    rate_period?: RentPeriod,
    rate_basis?: RateBasis,
    rate_type?: RateType,
    observed_on: string,
    concessions_note?: string,
}


/**
 * Recorded debt. Originating lender identity and classification live on the canonical Party referenced by a LoanParty. Assignments, modifications, satisfactions, and other lifecycle assertions are dated events; consumers derive current status and satisfaction dates.
 */
export interface Loan extends Entity, RecordedInstrument {
    property: PropertyId,
    parcel?: ParcelId,
    /** Purchase-money linkage */
    transfer?: TransferId,
    is_purchase_money?: boolean,
    loan_amount?: Money,
    /** conventional | fha | va | ... (open) */
    loan_type?: string,
    /** purchase | refinance | construction | heloc | ... */
    purpose?: string,
    is_heloc?: boolean,
    is_construction?: boolean,
    is_seller_carryback?: boolean,
    is_assumable?: boolean,
    /** Nominal interest rate in percentage points; 6.5 means 6.5 percent. */
    interest_rate?: number,
    is_variable_rate?: boolean,
    term_months?: number,
    due_date?: string,
    lien_position?: number,
    parties?: LoanParty[],
    events?: LoanEvent[],
}


/**
 * role = borrower | lender | beneficiary | trustee (assignees live in loan events)
 */
export interface LoanParty extends TransactionParty {
}


/**
 * Dated loan lifecycle — assignments, modifications, satisfactions. Recording fields apply when the event is a recorded instrument; they stay null for unrecorded servicing events.
 */
export interface LoanEvent extends RecordedInstrument {
    event_kind: LoanEventKind,
    occurred_on?: string,
    amount?: Money,
    /** Canonical assignee for assignment-like events */
    to_party?: PartyId,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * Involuntary encumbrances — tax, judgment, HOA, mechanic's.
 */
export interface Lien extends Entity, RecordedInstrument {
    property: PropertyId,
    kind: LienKind,
    amount?: Money,
    released_on?: string,
    parties?: LienParty[],
}


/**
 * role = claimant | debtor
 */
export interface LienParty extends TransactionParty {
}


/**
 * A foreclosure PROCEEDING; its recorded filings append over time.
 */
export interface ForeclosureCase extends Entity {
    property: PropertyId,
    /** The defaulted loan, when known */
    loan?: LoanId,
    case_number?: string,
    opened_on?: string,
    resolved_on?: string,
    /** sold_at_auction | cured | dismissed | reo */
    resolution?: string,
    past_due_amount?: Money,
    unpaid_balance?: Money,
    original_loan_amount?: Money,
    auction_min_bid?: Money,
    auction_location?: string,
    filings?: ForeclosureFiling[],
    parties?: ForeclosureCaseParty[],
}


/**
 * One recorded filing; postponements append as new filings.
 */
export interface ForeclosureFiling extends RecordedInstrument {
    /** nod | lis_pendens | notice_of_sale | auction_scheduled | postponement | ... (open; US-seeded) */
    status: string,
    auction_on?: string,
    /** Time-of-day as published */
    auction_at_time?: string,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * Open contextual role. Mortgage and deed-of-trust proceedings commonly use lender | trustee | borrower; lien foreclosures may instead use claimant | debtor.
 */
export interface ForeclosureCaseParty extends TransactionParty {
}


/**
 * Building permits — header + lifecycle dates. Inspections/projects are extension.
 */
export interface Permit extends Entity {
    property: PropertyId,
    structure?: StructureId,
    permitting_jurisdiction?: JurisdictionId,
    permit_number?: string,
    /** roofing | solar | hvac | adu | pool | new_construction | ... (open) */
    kind?: string,
    /** issued | finaled | expired | ... (open) */
    status?: string,
    description?: string,
    applied_on?: string,
    issued_on?: string,
    finaled_on?: string,
    expires_on?: string,
    job_value?: Money,
    fees?: Money,
    /** Canonical contractor Party reference; credential records are outside core v0.2 */
    contractor_party?: PartyId,
    artifacts?: SourceArtifactId[],
}


/**
 * Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).
 */
export interface OperatingStatement extends Entity {
    property: PropertyId,
    /** The calendar year the statement is for (the year containing period_end for fiscal/trailing periods). */
    statement_year: number,
    /** For fiscal-year, trailing-12, or partial periods. period_start and period_end must be provided together; omit both for calendar-year statements. */
    period_start?: string,
    period_end?: string,
    statement_basis?: StatementBasis,
    /** Potential gross income */
    pgi?: Money,
    vacancy_loss?: Money,
    /** 0–100 percentage points; 5 means 5 percent. */
    vacancy_pct?: number,
    /** Effective gross income */
    egi?: Money,
    opex_total?: Money,
    /** Net operating income; may be negative */
    noi?: Money,
    capex?: Money,
    /** Expense recoveries from tenants (CRE) */
    reimbursements?: Money,
    /** Reserves for replacement */
    reserves?: Money,
    reserves_included_in_opex?: boolean,
    ground_lease_expense?: Money,
    ground_lease_included_in_opex?: boolean,
    line_items?: StatementLineItem[],
}



export interface StatementLineItem {
    /** income | expense | capital | other (open vocab) */
    category: string,
    label: string,
    amount: Money,
    extras?: Any,
}


/**
 * Dated rent and occupancy observation applicable to any property use. Header totals are authoritative reported values; lines are supporting detail and are not required to sum to the totals. All Money values on one rent roll MUST use one currency (validator-enforced). For its as_of_date, this record governs reported occupancy and rent facts; current-state fields such as Space.occupancy and CommercialDetails.occupancy_pct do not override it. A line preserves dated rent-roll assertions and does not by itself create a canonical Space, Party, or LeaseEvent.
 */
export interface RentRoll extends Entity {
    property: PropertyId,
    as_of_date: string,
    unit_count?: number,
    occupied_unit_count?: number,
    /** Occupancy in 0–100 percentage points; 95 means 95 percent. */
    occupancy_pct?: number,
    total_contract_rent?: Money,
    total_market_rent?: Money,
    rent_period?: RentPeriod,
    lines?: RentRollLine[],
}


/**
 * Supporting rent-roll detail. space resolves to canonical Space. tenant is the canonical legal lessee Party when present, and lease resolves to canonical LeaseEvent when known. When both tenant and lease are present, tenant must match a party with role: lessee declared by that lease, if the lease declares any lessee. References remain optional for aggregate, vacant, unleased, or unresolved lines; the line does not duplicate canonical space identity, tenant names, or lease dates. When a source tenant cannot be resolved to a canonical Party, omit tenant and preserve the source evidence through the RentRoll provenance and profile-level SourceArtifact records; do not mint a placeholder Party or copy the source name into extras.
 */
export interface RentRollLine {
    space?: SpaceId,
    tenant?: PartyId,
    lease?: LeaseEventId,
    occupancy_status?: CodeableConcept,
    reported_area?: Area,
    contract_rent?: Money,
    market_rent?: Money,
    extras?: Any,
}


/**
 * Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.
 */
export interface Valuation extends Entity {
    property: PropertyId,
    property_state?: PropertyStateSnapshotId,
    kind: ValuationKind,
    /** desktop | exterior | hybrid | traditional */
    valuation_method?: string,
    /** market_value | liquidation | insurable | land | ... (open) */
    value_type?: string,
    value: Money,
    value_low?: Money,
    value_high?: Money,
    value_per_area?: UnitRate,
    /** Cost-approach site value */
    land_value?: Money,
    /** Source-defined confidence score; not governed by the _pct percentage-points convention. */
    confidence_score?: number,
    /** Source-defined AVM forecast standard deviation; not governed by the _pct percentage-points convention. */
    forecast_standard_deviation?: number,
    exposure_days?: number,
    indicated_value_sales_comparison?: Money,
    indicated_value_cost?: Money,
    indicated_value_income?: Money,
    /** as_is | as_completed | as_stabilized */
    value_premise?: string,
    /** Interest valued for this opinion; independent of Property.estate_type and Transfer.interest_conveyed. */
    interest?: string,
    /** Canonical person or organization that performed the valuation */
    performed_by_party?: PartyId,
    as_of_date: string,
    report_date?: string,
    artifacts?: SourceArtifactId[],
}


/**
 * The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
 */
export interface PropertyProfile {
    parties?: Party[],
    artifacts?: SourceArtifact[],
    /** Address bundle referenced by property/parties/ownership */
    addresses?: Address[],
    property: Property,
    property_addresses?: PropertyAddress[],
    identifiers?: PropertyIdentifier[],
    jurisdictions?: Jurisdiction[],
    parcels?: Parcel[],
    property_parcels?: PropertyParcel[],
    parcel_lineage?: ParcelLineage[],
    site?: Site,
    structures?: Structure[],
    spaces?: Space[],
    property_state_snapshots?: PropertyStateSnapshot[],
    associations?: PropertyAssociation[],
    assessments?: Assessment[],
    tax_bills?: TaxBill[],
    transfers?: Transfer[],
    sales?: SaleEvent[],
    listings?: Listing[],
    leases?: LeaseEvent[],
    unit_rents?: UnitRentObservation[],
    rent_rolls?: RentRoll[],
    loans?: Loan[],
    liens?: Lien[],
    foreclosure_cases?: ForeclosureCase[],
    permits?: Permit[],
    ownership?: OwnershipPeriod[],
    operating_statements?: OperatingStatement[],
    valuations?: Valuation[],
    provenance?: Provenance,
    extras?: Any,
}


/**
 * Thin capture envelope for a county assessor / public-records fetch. The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the fetch outcome. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence.
 */
export interface AssessorObservation {
    status: AssessorStatus,
    query_address?: string,
    query_parcel_number?: string,
    assessor_url?: string,
    profile?: PropertyProfile,
    error?: string,
    provenance: Provenance,
    /** References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent. */
    artifact_refs?: SourceArtifactId[],
    extras?: Any,
}


/**
 * Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the extraction outcome. By convention profile and category accompany success and error accompanies failure statuses — validators deliberately do not enforce this pairing (LinkML rules would materialize in only some generated validators, diverging the contracts), so consumers must branch on status, not on field presence.
 */
export interface ExtractionObservation {
    status: ExtractionStatus,
    /** Content classification; expected when status is success */
    category?: ExtractionCategory,
    /** Raw or more specific producer classification, especially when category is `other` */
    source_category?: string,
    error?: string,
    source_url?: string,
    extracted_at?: string,
    /** Extraction model identifier */
    model?: string,
    profile?: PropertyProfile,
    provenance: Provenance,
    /** References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent. */
    artifact_refs?: SourceArtifactId[],
    extras?: Any,
}
