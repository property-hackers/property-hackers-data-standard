// ISO 8601 date string (shim: gen-typescript emits the LinkML type name verbatim)
type date = string;

export type EntityId = string;
export type JurisdictionId = string;
export type AddressId = string;
export type PropertyId = string;
export type ParcelId = string;
export type PropertyParcelId = string;
export type ParcelLineageId = string;
export type PropertyIdentifierId = string;
export type PartyId = string;
export type OwnershipPeriodId = string;
export type StructureId = string;
export type SiteId = string;
export type SpaceId = string;
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

export enum PartyKind {
    
    person = "person",
    organization = "organization",
};

export enum OrganizationKind {
    
    llc = "llc",
    corporation = "corporation",
    partnership = "partnership",
    trust = "trust",
    estate = "estate",
    government = "government",
    nonprofit = "nonprofit",
    reit = "reit",
    fund = "fund",
    lender = "lender",
    brokerage = "brokerage",
    hoa = "hoa",
    other = "other",
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

export enum LoanStatus {
    
    active = "active",
    satisfied = "satisfied",
    assigned = "assigned",
    foreclosure = "foreclosure",
    released = "released",
    unknown = "unknown",
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
    leasehold = "leasehold",
    life_estate = "life_estate",
    cooperative_shares = "cooperative_shares",
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
 * Open map — the `extras` escape hatch on every entity.
 */
export interface Any {
}


/**
 * Monetary amount. JSON wire format is decimal-as-string. ISO-4217 currency.
 */
export interface Money {
    amount: string,
    currency: string,
}


/**
 * Area measurement with explicit unit.
 */
export interface Area {
    value: string,
    unit: string,
}


/**
 * Linear measurement with explicit unit.
 */
export interface Length {
    value: string,
    unit: string,
}


/**
 * Money per explicit denominator (price/sqft, rent/sqm/month, spaces/1000 sqft).
 */
export interface UnitRate {
    amount: string,
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
 * WGS84 coordinate.
 */
export interface GeoPoint {
    latitude: string,
    longitude: string,
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
    method?: string,
    confidence?: number,
    verification?: string,
}



export interface Entity {
    id: string,
    extras?: Any,
    provenance?: Provenance,
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
    recorded_on?: date,
    /** Date executed/signed as dated on the instrument */
    instrument_date?: date,
    document_type?: CodeableConcept,
    /** Authority maintaining the record (optional — parcel context is inference, not identity) */
    recording_authority?: JurisdictionId,
    /** Alternate authority-issued reference: title, dealing, folio, or notarial-act number */
    registry_reference?: string,
    related_instruments?: InstrumentReference[],
}


/**
 * A participant in an event, as recorded on the instrument. `name` is the immutable recorded fact; `party` is optional resolution to a canonical Party. Multi-party is the norm (couples on deeds, co-borrowers).
 */
export interface TransactionParty {
    role: string,
    name: string,
    party?: PartyId,
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
    /** Fallback when components don't parse */
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
    /** authority code (US: county FIPS) */
    admin_area_code?: string,
    /** App-computed dedup key */
    address_hash: string,
    location?: GeoPoint,
    location_accuracy?: string,
}


/**
 * The immutable anchor. Parcel numbers/APNs/vendor IDs are labels about a property, never its key.
 */
export interface Property extends Entity {
    name?: string,
    /** PUCS class when property_use_system = pucs_1_0 */
    property_use_class?: string,
    property_use_type?: string,
    property_use_subtype?: string,
    /** 'pucs_1_0' | local system; REQUIRED when use fields are set (no default) */
    property_use_system?: string,
    estate_type?: string,
    situs_address?: AddressId,
    location?: GeoPoint,
    /** Vendor summary when structures are not enumerated */
    building_count?: number,
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
    retired_on?: date,
}


/**
 * Property ↔ parcel, many-to-many over time (splits/merges/condos). ended_on is end-exclusive.
 */
export interface PropertyParcel extends Entity {
    property: PropertyId,
    parcel: ParcelId,
    is_primary?: boolean,
    started_on?: date,
    ended_on?: date,
}



export interface ParcelLineage extends Entity {
    predecessor_parcel: ParcelId,
    successor_parcel: ParcelId,
    kind: string,
    effective_on?: date,
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
 * One model for every actor — owners, buyers, borrowers, lenders, brokers, trustees, claimants, contractors, HOAs.
 */
export interface Party extends Entity {
    kind: string,
    organization_kind?: string,
    /** Display name as recorded */
    name: string,
    normalized_name?: string,
    name_first?: string,
    name_middle?: string,
    name_last?: string,
    /** Agents / brokers / appraisers */
    license_number?: string,
    addresses?: PartyAddress[],
    contacts?: PartyContact[],
}


/**
 * mailing | physical | registered_agent | previous | other
 */
export interface PartyAddress {
    address: AddressId,
    kind?: string,
    is_primary?: boolean,
    extras?: Any,
    provenance?: Provenance,
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
    started_on?: date,
    ended_on?: date,
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
    interest_pct?: string,
    /** owner | trustee | gp | lp */
    role?: string,
    is_owner_occupied?: boolean,
    extras?: Any,
}



export interface Structure extends Entity {
    property: PropertyId,
    /** building | barn | silo | shed | outbuilding | ... (open) */
    kind?: string,
    name?: string,
    structure_number?: string,
    /** Finished above-grade living area per UAD 3.6 definition */
    living_area?: Area,
    gross_area?: Area,
    /** BOMA definitions */
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
    stories?: string,
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
    condition_rating?: string,
    quality_rating?: string,
    /** 'uad_3_6' (C1–C6/Q1–Q6) | 'local' | ... */
    rating_system?: string,
    heating_types?: string[],
    heating_fuel_type?: string,
    cooling_types?: string[],
    sewer_type?: string,
    water_type?: string,
    /** Open vocab seeded from UAD AmenityBase */
    features?: string[],
    residential?: ResidentialDetails,
    commercial?: CommercialDetails,
    renovations?: Renovation[],
}



export interface AreaMeasure {
    kind: string,
    area: Area,
    extras?: Any,
}


/**
 * International core; the UAD 3.6 profile constrains enums for US appraisal use. A CURRENT-STATE snapshot dated by provenance.retrieved_at (see CommercialDetails note).
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
 * BOMA/OSCRE-cited semantics. Like all detail records, this is a CURRENT-STATE snapshot whose as-of date is provenance.retrieved_at; time-varying fields (occupancy_pct, tenant_count, condition) are not individually dated. Historical/per-period figures belong on dated records: OperatingStatement, UnitRentObservation, or as-of-event snapshots on SaleEvent.
 */
export interface CommercialDetails {
    /** A | B | C */
    building_class?: string,
    clear_height?: Length,
    dock_doors?: number,
    drive_in_doors?: number,
    occupancy_pct?: string,
    parking_spaces?: number,
    /** e.g. spaces per 1000 sqft — denominator explicit */
    parking_ratio?: UnitRate,
    /** single_tenant | multi_tenant */
    tenancy?: string,
    /** Distinct legal tenants (not suites or leases); current-state, dated by provenance */
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
    completed_on?: date,
    cost?: Money,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * 1:1 with property.
 */
export interface Site extends Entity {
    property: PropertyId,
    lot_size?: Area,
    /** Usable portion of the lot; see usable_land_area_basis */
    usable_land_area?: Area,
    /** surveyed | buildable | net_of_constraints | ... (what "usable" means here) */
    usable_land_area_basis?: string,
    /** County code + label, system-tagged */
    land_use?: CodeableConcept,
    /** Standardized (PUCS land classes) */
    land_use_category?: string,
    zoning_code?: string,
    flood_zone?: string,
    hazard_zones?: string[],
    /** UAD ViewBase-seeded */
    view_types?: string[],
    /** UAD SiteInfluenceBase-seeded */
    site_influences?: string[],
    easements?: string[],
    restrictions?: string[],
    utilities?: string[],
    frontage?: Length,
    depth?: Length,
    topography?: string,
    is_corner?: boolean,
    /** raw | entitled | permitted | ... (open; construction/dev) */
    entitlement_status?: string,
    buildable_units?: number,
    subdivision?: string,
    lot_number?: string,
    block?: string,
    tract_number?: string,
    phase_number?: string,
    /** US PLSS (US-specific, optional) */
    section_township_range?: string,
}


/**
 * Leasable suites/units (CRE, multifamily).
 */
export interface Space extends Entity {
    property: PropertyId,
    structure?: StructureId,
    space_identifier: string,
    floor_number?: number,
    space_use?: string,
    rentable_area?: Area,
    usable_area?: Area,
    bedrooms?: number,
    bathrooms?: string,
    /** owner_occupied | tenant | vacant */
    occupancy?: string,
    is_adu?: boolean,
    is_active?: boolean,
}


/**
 * HOA / property association — minimal v1 footprint.
 */
export interface PropertyAssociation extends Entity {
    property: PropertyId,
    /** The association as an organization */
    party?: PartyId,
    name?: string,
    fee?: Money,
    fee_period?: string,
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
    due_on?: date,
    amount?: Money,
    paid_on?: date,
    amount_paid?: Money,
    is_delinquent?: boolean,
    extras?: Any,
}



export interface TaxLineItem {
    /** Taxing authority name or code */
    authority?: string,
    /** Mill rate / levy */
    rate?: string,
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
    effective_on?: date,
    /** Often $0 / nominal */
    consideration?: Money,
    /** Doc stamps; price-inference basis in many places */
    transfer_tax?: Money,
    price_disclosure?: string,
    price_code?: CodeableConcept,
    partial_interest_pct?: string,
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
    transfer?: TransferId,
    sale_date: date,
    sale_price?: Money,
    price_disclosure?: string,
    price_code?: CodeableConcept,
    sale_type?: string,
    price_per_area?: UnitRate,
    price_per_unit?: Money,
    /** cash | conventional | seller | assumption | other (coarse; loans carry detail) */
    financing?: string,
    concessions?: Money,
    cap_rate?: string,
    noi_at_sale?: Money,
    opex_at_sale?: Money,
    occupancy_at_sale_pct?: string,
    unit_count_at_sale?: number,
    /** Traceability: the statement noi_at_sale derives from, when known */
    supporting_operating_statement?: OperatingStatementId,
    parties?: SaleEventParty[],
}


/**
 * role = buyer | seller | buyer_broker | seller_broker
 */
export interface SaleEventParty extends TransactionParty {
}


/**
 * Listing header. Lifecycle lives in events[]; header status/list_price are denormalized conveniences reconstructible from events.
 */
export interface Listing extends Entity {
    property: PropertyId,
    kind: string,
    /** mls | fsbo | auction | pocket */
    listing_type?: string,
    status?: string,
    original_list_price?: Money,
    list_price?: Money,
    list_rent?: Money,
    list_rent_period?: string,
    listed_on?: date,
    closed_on?: date,
    close_price?: Money,
    mls_number?: string,
    events?: ListingEvent[],
    participants?: ListingParticipant[],
}



export interface ListingEvent {
    occurred_on: date,
    /** listed | price_change | status_change | relisted | closed */
    event_kind: string,
    status?: string,
    list_price?: Money,
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
    space?: SpaceId,
    lease_type?: string,
    execution_date?: date,
    commencement_date?: date,
    expiration_date?: date,
    term_months?: number,
    leased_area?: Area,
    rent?: Money,
    rent_period?: string,
    starting_rent_per_area?: UnitRate,
    effective_rent_per_area?: UnitRate,
    net_effective_rent_per_area?: UnitRate,
    free_rent_months?: string,
    ti_allowance_per_area?: UnitRate,
    expense_structure?: ExpenseStructure,
    lessee_industry?: string,
    parties?: LeaseEventParty[],
    escalations?: LeaseEscalation[],
    concessions?: LeaseConcession[],
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
    escalation_value?: string,
    frequency_months?: number,
    cpi_index?: string,
    cpi_floor?: string,
    cpi_cap?: string,
    steps?: RentStep[],
    effective_from?: date,
    effective_until?: date,
    extras?: Any,
}



export interface RentStep {
    from_date: date,
    amount: Money,
}



export interface LeaseConcession {
    /** free_rent | ti_allowance | moving_allowance | ... */
    concession_type: string,
    concession_value?: Money,
    abatement_months?: string,
    abatement_percent?: string,
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
    bathrooms?: string,
    /** Units of this type */
    unit_count?: number,
    units_available?: number,
    rate: Money,
    rate_period?: string,
    rate_basis?: string,
    rate_type?: string,
    observed_on: date,
    concessions_note?: string,
}


/**
 * Recorded debt. lender_name is the immutable ORIGINATING lender; assignments and status changes are dated loan events. status is a derived projection of events.
 */
export interface Loan extends Entity, RecordedInstrument {
    property: PropertyId,
    parcel?: ParcelId,
    /** Purchase-money linkage */
    transfer?: TransferId,
    is_purchase_money?: boolean,
    loan_amount?: Money,
    lender_name?: string,
    /** bank | credit_union | private | seller | ... */
    lender_type?: string,
    /** conventional | fha | va | ... (open) */
    loan_type?: string,
    /** purchase | refinance | construction | heloc | ... */
    purpose?: string,
    is_heloc?: boolean,
    is_construction?: boolean,
    is_seller_carryback?: boolean,
    is_assumable?: boolean,
    interest_rate?: string,
    is_variable_rate?: boolean,
    term_months?: number,
    due_date?: date,
    lien_position?: number,
    status?: string,
    satisfied_on?: date,
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
    event_kind: string,
    occurred_on?: date,
    amount?: Money,
    /** Assignee lender name (assignments) */
    to_name?: string,
    to_party?: PartyId,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * Involuntary encumbrances — tax, judgment, HOA, mechanic's.
 */
export interface Lien extends Entity, RecordedInstrument {
    property: PropertyId,
    kind: string,
    amount?: Money,
    released_on?: date,
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
    opened_on?: date,
    resolved_on?: date,
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
    auction_on?: date,
    /** Time-of-day as published */
    auction_at_time?: string,
    extras?: Any,
    provenance?: Provenance,
}


/**
 * role = lender | trustee | borrower
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
    applied_on?: date,
    issued_on?: date,
    finaled_on?: date,
    expires_on?: date,
    job_value?: Money,
    fees?: Money,
    contractor_name?: string,
    contractor_license?: string,
    contractor_party?: PartyId,
}


/**
 * Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).
 */
export interface OperatingStatement extends Entity {
    property: PropertyId,
    /** The calendar year the statement is for (the year containing period_end for fiscal/trailing periods). */
    statement_year: number,
    /** For fiscal-year, trailing-12, or partial periods. period_start and period_end must be provided together; omit both for calendar-year statements. */
    period_start?: date,
    period_end?: date,
    statement_basis?: string,
    /** Potential gross income */
    pgi?: Money,
    vacancy_loss?: Money,
    vacancy_pct?: string,
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
 * Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.
 */
export interface Valuation extends Entity {
    property: PropertyId,
    kind: string,
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
    confidence_score?: number,
    /** AVM FSD */
    forecast_standard_deviation?: string,
    exposure_days?: number,
    indicated_value_sales_comparison?: Money,
    indicated_value_cost?: Money,
    indicated_value_income?: Money,
    /** as_is | as_completed | as_stabilized */
    value_premise?: string,
    /** fee_simple | leased_fee | leasehold */
    interest?: string,
    performed_by?: string,
    performed_by_party?: PartyId,
    as_of_date: date,
    report_date?: date,
}


/**
 * The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any event (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
 */
export interface PropertyProfile {
    parties?: Party[],
    /** Address bundle referenced by property/parties/ownership */
    addresses?: Address[],
    property: Property,
    identifiers?: PropertyIdentifier[],
    jurisdictions?: Jurisdiction[],
    parcels?: Parcel[],
    property_parcels?: PropertyParcel[],
    parcel_lineage?: ParcelLineage[],
    site?: Site,
    structures?: Structure[],
    spaces?: Space[],
    associations?: PropertyAssociation[],
    assessments?: Assessment[],
    tax_bills?: TaxBill[],
    transfers?: Transfer[],
    sales?: SaleEvent[],
    listings?: Listing[],
    leases?: LeaseEvent[],
    unit_rents?: UnitRentObservation[],
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
 * Thin capture envelope for a county assessor / public-records fetch. The payload is a partial PropertyProfile; status reports the fetch outcome.
 */
export interface AssessorObservation {
    status: AssessorStatus,
    query_address?: string,
    query_parcel_number?: string,
    assessor_url?: string,
    profile?: PropertyProfile,
    error?: string,
    provenance: Provenance,
    extras?: Any,
}


/**
 * Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a partial PropertyProfile; category classifies the source page.
 */
export interface ExtractionObservation {
    /** sales_transaction | sale_listing | lease_listing | in_place_lease | public_data | other */
    category: string,
    source_url?: string,
    extracted_at?: string,
    /** Extraction model identifier */
    model?: string,
    profile?: PropertyProfile,
    provenance: Provenance,
    extras?: Any,
}



