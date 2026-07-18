from __future__ import annotations

import re
import sys
from datetime import (
    date,
    datetime,
    time
)
from decimal import Decimal
from enum import Enum
from typing import (
    Any,
    ClassVar,
    Literal,
    Optional,
    Union
)

from pydantic import (
    BaseModel,
    ConfigDict,
    Field,
    RootModel,
    SerializationInfo,
    SerializerFunctionWrapHandler,
    field_validator,
    model_serializer
)


metamodel_version = "1.11.0"
version = "0.1.0"


class ConfiguredBaseModel(BaseModel):
    model_config = ConfigDict(
        serialize_by_alias = True,
        validate_by_name = True,
        validate_assignment = True,
        validate_default = True,
        extra = "forbid",
        arbitrary_types_allowed = True,
        use_enum_values = True,
        strict = False,
    )





class LinkMLMeta(RootModel):
    root: dict[str, Any] = {}
    model_config = ConfigDict(frozen=True)

    def __getattr__(self, key:str):
        return getattr(self.root, key)

    def __getitem__(self, key:str):
        return self.root[key]

    def __setitem__(self, key:str, value):
        self.root[key] = value

    def __contains__(self, key:str) -> bool:
        return key in self.root


linkml_meta = LinkMLMeta({'default_prefix': 'phds',
     'default_range': 'string',
     'id': 'https://example.org/phds/profiles',
     'imports': ['linkml:types', 'core', 'entities'],
     'license': 'MIT',
     'name': 'phds_profiles',
     'prefixes': {'linkml': {'prefix_prefix': 'linkml',
                             'prefix_reference': 'https://w3id.org/linkml/'},
                  'phds': {'prefix_prefix': 'phds',
                           'prefix_reference': 'https://example.org/phds/'}},
     'source_file': 'schema/profiles.yaml',
     'title': 'PHDS Profiles'} )

class AreaUnit(str, Enum):
    sqft = "sqft"
    sqm = "sqm"
    acre = "acre"
    hectare = "hectare"


class LengthUnit(str, Enum):
    ft = "ft"
    m = "m"


class CaptureMethod(str, Enum):
    api = "api"
    scrape = "scrape"
    llm_extraction = "llm_extraction"
    manual = "manual"
    bulk = "bulk"


class VerificationStatus(str, Enum):
    unverified = "unverified"
    pending_review = "pending_review"
    verified = "verified"
    disputed = "disputed"
    rejected = "rejected"


class AssessorStatus(str, Enum):
    """
    Outcome of an assessor or public-records lookup.
    """
    success = "success"
    not_found = "not_found"
    timeout = "timeout"
    api_error = "api_error"
    parse_error = "parse_error"
    invalid_address = "invalid_address"
    ambiguous = "ambiguous"


class PartyKind(str, Enum):
    person = "person"
    organization = "organization"


class OrganizationKind(str, Enum):
    llc = "llc"
    corporation = "corporation"
    partnership = "partnership"
    trust = "trust"
    estate = "estate"
    government = "government"
    nonprofit = "nonprofit"
    reit = "reit"
    fund = "fund"
    lender = "lender"
    brokerage = "brokerage"
    hoa = "hoa"
    other = "other"


class SaleTypeEnum(str, Enum):
    arms_length = "arms_length"
    reo = "reo"
    """
    Real-estate-owned / bank sale
    """
    short_sale = "short_sale"
    auction = "auction"
    related_party = "related_party"
    portfolio = "portfolio"
    partial_interest = "partial_interest"
    land_contract = "land_contract"
    new_construction = "new_construction"
    other = "other"


class PriceDisclosure(str, Enum):
    """
    Reliability of a recorded price
    """
    full = "full"
    partial = "partial"
    estimated = "estimated"
    nominal = "nominal"
    non_disclosure = "non_disclosure"
    unknown = "unknown"


class LeaseTypeEnum(str, Enum):
    gross = "gross"
    modified_gross = "modified_gross"
    triple_net = "triple_net"
    double_net = "double_net"
    single_net = "single_net"
    absolute_net = "absolute_net"
    percentage = "percentage"
    ground = "ground"
    residential = "residential"
    other = "other"


class RentPeriod(str, Enum):
    daily = "daily"
    monthly = "monthly"
    annual = "annual"
    per_area_annual = "per_area_annual"
    per_area_monthly = "per_area_monthly"


class RateBasis(str, Enum):
    per_unit = "per_unit"
    per_bed = "per_bed"
    per_area = "per_area"
    per_room = "per_room"
    per_key = "per_key"
    per_slip = "per_slip"
    per_stall = "per_stall"
    per_pad = "per_pad"
    other = "other"


class RateType(str, Enum):
    asking = "asking"
    effective = "effective"
    contract = "contract"


class ListingKind(str, Enum):
    for_sale = "for_sale"
    for_lease = "for_lease"


class ListingStatus(str, Enum):
    active = "active"
    pending = "pending"
    sold = "sold"
    leased = "leased"
    withdrawn = "withdrawn"
    expired = "expired"
    coming_soon = "coming_soon"
    other = "other"


class ValuationKind(str, Enum):
    avm = "avm"
    appraisal = "appraisal"
    bpo = "bpo"
    broker_opinion = "broker_opinion"
    internal = "internal"


class LoanStatus(str, Enum):
    active = "active"
    satisfied = "satisfied"
    assigned = "assigned"
    foreclosure = "foreclosure"
    released = "released"
    unknown = "unknown"


class LoanEventKind(str, Enum):
    origination = "origination"
    assignment = "assignment"
    modification = "modification"
    satisfaction = "satisfaction"
    release = "release"
    default = "default"
    reinstatement = "reinstatement"
    other = "other"


class LienKind(str, Enum):
    tax = "tax"
    judgment = "judgment"
    hoa = "hoa"
    mechanics = "mechanics"
    municipal = "municipal"
    other = "other"


class ParcelLineageKind(str, Enum):
    split = "split"
    merge = "merge"
    renumber = "renumber"


class EstateType(str, Enum):
    fee_simple = "fee_simple"
    leasehold = "leasehold"
    life_estate = "life_estate"
    cooperative_shares = "cooperative_shares"
    other = "other"


class StatementBasis(str, Enum):
    """
    What an operating statement represents.
    """
    actual = "actual"
    budget = "budget"
    pro_forma = "pro_forma"
    stabilized = "stabilized"
    projected = "projected"
    other = "other"


class GeocodeAccuracy(str, Enum):
    rooftop = "rooftop"
    parcel = "parcel"
    street = "street"
    postal_centroid = "postal_centroid"
    locality_centroid = "locality_centroid"
    manual = "manual"
    unknown = "unknown"



class Money(ConfiguredBaseModel):
    """
    Monetary amount. JSON wire format is decimal-as-string. ISO-4217 currency.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    amount: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    currency: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money', 'UnitRate']} })

    @field_validator('amount')
    def pattern_amount(cls, v):
        pattern=re.compile(r"^-?\d+(\.\d+)?$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid amount format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid amount format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('currency')
    def pattern_currency(cls, v):
        pattern=re.compile(r"^[A-Z]{3}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid currency format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid currency format: {v}"
            raise ValueError(err_msg)
        return v


class Area(ConfiguredBaseModel):
    """
    Area measurement with explicit unit.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    value: Decimal = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    unit: AreaUnit = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area', 'Length']} })


class Length(ConfiguredBaseModel):
    """
    Linear measurement with explicit unit.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    value: Decimal = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    unit: LengthUnit = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area', 'Length']} })


class UnitRate(ConfiguredBaseModel):
    """
    Money per explicit denominator (price/sqft, rent/sqm/month, spaces/1000 sqft).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    amount: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    currency: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money', 'UnitRate']} })
    denominator: str = Field(default=..., description="""sqft | sqm | unit | bed | key | month | sqft_month | 1000_sqft | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRate']} })

    @field_validator('amount')
    def pattern_amount(cls, v):
        pattern=re.compile(r"^-?\d+(\.\d+)?$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid amount format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid amount format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('currency')
    def pattern_currency(cls, v):
        pattern=re.compile(r"^[A-Z]{3}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid currency format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid currency format: {v}"
            raise ValueError(err_msg)
        return v


class CodeableConcept(ConfiguredBaseModel):
    """
    Source coding preserved alongside canonical values (system + code + display).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    system: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })
    code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })
    display: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })


class GeoPoint(ConfiguredBaseModel):
    """
    WGS84 coordinate.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    latitude: Decimal = Field(default=..., ge=-90, le=90, json_schema_extra = { "linkml_meta": {'domain_of': ['GeoPoint']} })
    longitude: Decimal = Field(default=..., ge=-180, le=180, json_schema_extra = { "linkml_meta": {'domain_of': ['GeoPoint']} })


class Geometry(ConfiguredBaseModel):
    """
    GeoJSON Geometry object (typed-in-standard; optional everywhere).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    type: str = Field(default=..., description="""Point | MultiPolygon | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Geometry']} })
    coordinates: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Geometry']} })


class Provenance(ConfiguredBaseModel):
    """
    Record-level source envelope. Pure child rows (parties, areas, escalations, filings) inherit the parent envelope unless they carry their own.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    provider: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })
    source_url: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance', 'ExtractionObservation']} })
    retrieved_at: Optional[datetime ] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })
    method: Optional[CaptureMethod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })
    confidence: Optional[float] = Field(default=None, ge=0, le=1, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })
    verification: Optional[VerificationStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })


class Entity(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'abstract': True, 'from_schema': 'https://example.org/phds/entities'})

    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class InstrumentReference(ConfiguredBaseModel):
    """
    Cross-reference to another recorded instrument (e.g., an assignment citing the original mortgage's recording number) — the target may not exist as a PHDS row.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    relationship_type: Optional[CodeableConcept] = Field(default=None, description="""re_records | corrects | releases | assigns | modifies | subordinates | refers_to""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference']} })
    document_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class RecordedInstrument(ConfiguredBaseModel):
    """
    This row corresponds to a document recorded/registered with an authority (county recorder, land registry, notarial protocol). Narrowly about document identity and recording — legal effect lives on the host class.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })


class TransactionParty(ConfiguredBaseModel):
    """
    A participant in an event, as recorded on the instrument. `name` is the immutable recorded fact; `party` is optional resolution to a canonical Party. Multi-party is the norm (couples on deeds, co-borrowers).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'abstract': True, 'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Jurisdiction(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    country: str = Field(default=..., description="""ISO 3166-1 alpha-2 (no default — i18n)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    region: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    kind: str = Field(default=..., description="""county | municipality | taxing_district | ... (open vocab)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    authority_code: Optional[str] = Field(default=None, description="""US: FIPS/GEOID""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction']} })
    parent: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction']} })
    boundary: Optional[Geometry] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Parcel']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('country')
    def pattern_country(cls, v):
        pattern=re.compile(r"^[A-Z]{2}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid country format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid country format: {v}"
            raise ValueError(err_msg)
        return v


class Address(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    country: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    unformatted_address: Optional[str] = Field(default=None, description="""Fallback when components don't parse""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    street_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    street_pre_direction: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    street_name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    street_suffix: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    street_post_direction: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    unit_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'UnitRentObservation']} })
    unit_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    sublocality: Optional[str] = Field(default=None, description="""neighborhood / borough / barrio""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    city: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    region: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    postal_code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    postal_code_suffix: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    admin_area: Optional[str] = Field(default=None, description="""county/district name""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    admin_area_code: Optional[str] = Field(default=None, description="""authority code (US: county FIPS)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    address_hash: str = Field(default=..., description="""App-computed dedup key""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'Property']} })
    location_accuracy: Optional[GeocodeAccuracy] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('country')
    def pattern_country(cls, v):
        pattern=re.compile(r"^[A-Z]{2}$")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid country format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid country format: {v}"
            raise ValueError(err_msg)
        return v


class Property(Entity):
    """
    The immutable anchor. Parcel numbers/APNs/vendor IDs are labels about a property, never its key.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    property_use_class: Optional[str] = Field(default=None, description="""PUCS class when property_use_system = pucs_1_0""", json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    property_use_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    property_use_subtype: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    property_use_system: Optional[str] = Field(default=None, description="""'pucs_1_0' | local system; REQUIRED when use fields are set (no default)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    estate_type: Optional[EstateType] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    situs_address: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'Property']} })
    building_count: Optional[int] = Field(default=None, description="""Vendor summary when structures are not enumerated""", json_schema_extra = { "linkml_meta": {'domain_of': ['Property']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Parcel(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'parcel_identity': {'unique_key_name': 'parcel_identity',
                                             'unique_key_slots': ['jurisdiction',
                                                                  'parcel_number',
                                                                  'unit_designator',
                                                                  'retired_on']}}})

    jurisdiction: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill']} })
    parcel_number: str = Field(default=..., description="""RAW as issued (RESO UPI rule)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel'], 'exact_mappings': ['reso:ParcelNumber']} })
    normalized_parcel_number: Optional[str] = Field(default=None, description="""Matching only, never identity""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    unit_designator: Optional[str] = Field(default=None, description="""Condo sub-parcel discriminator""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    reso_upi: Optional[str] = Field(default=None, description="""urn:reso:upi:2.0:...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    legal_description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    land_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    boundary: Optional[Geometry] = Field(default=None, description="""GeoJSON MultiPolygon (optional)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Parcel']} })
    retired_on: Optional[date] = Field(default=None, description="""Set by lineage events""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PropertyParcel(Entity):
    """
    Property ↔ parcel, many-to-many over time (splits/merges/condos). ended_on is end-exclusive.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'property_parcel_pair': {'unique_key_name': 'property_parcel_pair',
                                                  'unique_key_slots': ['property',
                                                                       'parcel']}}})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'PartyAddress', 'PartyContact']} })
    started_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    ended_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ParcelLineage(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    predecessor_parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage']} })
    successor_parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage']} })
    kind: ParcelLineageKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    effective_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage', 'Transfer']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PropertyIdentifier(Entity):
    """
    Namespaced external IDs — MLS keys, vendor IDs, tax account numbers.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'identifier_identity': {'unique_key_name': 'identifier_identity',
                                                 'unique_key_slots': ['scheme',
                                                                      'namespace',
                                                                      'value']}}})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    scheme: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyIdentifier']} })
    namespace: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyIdentifier']} })
    value: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Party(Entity):
    """
    One model for every actor — owners, buyers, borrowers, lenders, brokers, trustees, claimants, contractors, HOAs.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: PartyKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    organization_kind: Optional[OrganizationKind] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name: str = Field(default=..., description="""Display name as recorded""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    normalized_name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_first: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_middle: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_last: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    license_number: Optional[str] = Field(default=None, description="""Agents / brokers / appraisers""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    addresses: Optional[list[PartyAddress]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party', 'PropertyProfile']} })
    contacts: Optional[list[PartyContact]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PartyAddress(ConfiguredBaseModel):
    """
    mailing | physical | registered_agent | previous | other
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    address: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PartyAddress']} })
    kind: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'PartyAddress', 'PartyContact']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PartyContact(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., description="""phone | email | website | other""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    value: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    label: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PartyContact', 'StatementLineItem']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'PartyAddress', 'PartyContact']} })
    do_not_contact: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PartyContact']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class OwnershipPeriod(Entity):
    """
    One ownership regime; ended_on is end-exclusive, NULL = current.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    started_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    ended_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    vesting_type: Optional[str] = Field(default=None, description="""joint_tenants | tenants_in_common | community_property | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    mailing_address: Optional[str] = Field(default=None, description="""Owner's mailing address during THIS period""", json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    acquired_via_transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    disposed_via_transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    interests: Optional[list[OwnershipInterest]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class OwnershipInterest(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    interest_pct: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipInterest']} })
    role: Optional[str] = Field(default=None, description="""owner | trustee | gp | lp""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    is_owner_occupied: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipInterest']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Structure(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    kind: Optional[str] = Field(default=None, description="""building | barn | silo | shed | outbuilding | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    structure_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    living_area: Optional[Area] = Field(default=None, description="""Finished above-grade living area per UAD 3.6 definition""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure'], 'exact_mappings': ['reso:LivingArea']} })
    gross_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    rentable_area: Optional[Area] = Field(default=None, description="""BOMA definitions""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure', 'Space']} })
    ground_floor_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    basement_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    basement_finished_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    garage_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    areas: Optional[list[AreaMeasure]] = Field(default=None, description="""Long-tail area kinds; canonical kinds forbidden here (validator-enforced)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    year_built: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure'], 'exact_mappings': ['reso:YearBuilt']} })
    year_built_estimated: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    effective_year_built: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    stories: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    unit_count: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure', 'UnitRentObservation']} })
    construction_method: Optional[str] = Field(default=None, description="""site_built | manufactured | modular | container | 3d_printed | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    construction_status: Optional[str] = Field(default=None, description="""complete | proposed | under_construction""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    construction_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    exterior_wall_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    roof_material_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    roof_style_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    foundation_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    foundation_material: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    condition_rating: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    quality_rating: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    rating_system: Optional[str] = Field(default=None, description="""'uad_3_6' (C1–C6/Q1–Q6) | 'local' | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    heating_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    heating_fuel_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    cooling_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    sewer_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    water_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    features: Optional[list[str]] = Field(default=None, description="""Open vocab seeded from UAD AmenityBase""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    residential: Optional[ResidentialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    commercial: Optional[CommercialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    renovations: Optional[list[Renovation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class AreaMeasure(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    area: Area = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['AreaMeasure']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ResidentialDetails(ConfiguredBaseModel):
    """
    International core; the UAD 3.6 profile constrains enums for US appraisal use. A CURRENT-STATE snapshot dated by provenance.retrieved_at (see CommercialDetails note).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    bedrooms_total: Optional[int] = Field(default=None, ge=0, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails'], 'exact_mappings': ['reso:BedroomsTotal']} })
    bathrooms_full: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    bathrooms_half: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    bathrooms_three_quarter: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    rooms_total: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    attachment: Optional[str] = Field(default=None, description="""detached | attached | semi_detached | rowhouse_townhouse | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    architectural_design: Optional[str] = Field(default=None, description="""ranch | cape_cod | colonial | craftsman | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    basement_type: Optional[str] = Field(default=None, description="""none | full | partial | walkout | cellar""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    garage_type: Optional[str] = Field(default=None, description="""garage | carport | parking_garage | driveway | none | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    garage_attachment: Optional[str] = Field(default=None, description="""attached | detached | built_in""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    parking_spaces: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'CommercialDetails']} })
    fireplaces: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    has_pool: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    has_attic: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    has_adu: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    adu_legally_rentable: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'Space']} })
    renewable_energy_components: Optional[list[str]] = Field(default=None, description="""solar | geothermal | wind""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class CommercialDetails(ConfiguredBaseModel):
    """
    BOMA/OSCRE-cited semantics. Like all detail records, this is a CURRENT-STATE snapshot whose as-of date is provenance.retrieved_at; time-varying fields (occupancy_pct, tenant_count, condition) are not individually dated. Historical/per-period figures belong on dated records: OperatingStatement, UnitRentObservation, or as-of-event snapshots on SaleEvent.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    building_class: Optional[str] = Field(default=None, description="""A | B | C""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    clear_height: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    dock_doors: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    drive_in_doors: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    occupancy_pct: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    parking_spaces: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'CommercialDetails']} })
    parking_ratio: Optional[UnitRate] = Field(default=None, description="""e.g. spaces per 1000 sqft — denominator explicit""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    tenancy: Optional[str] = Field(default=None, description="""single_tenant | multi_tenant""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    tenant_count: Optional[int] = Field(default=None, description="""Distinct legal tenants (not suites or leases); current-state, dated by provenance""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    parking_types: Optional[list[str]] = Field(default=None, description="""surface | structured | underground | covered | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    has_sprinkler: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    elevators: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    submarket: Optional[str] = Field(default=None, description="""CRE market geography""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Renovation(ConfiguredBaseModel):
    """
    Repeatable events — never a flat year on the structure.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: Optional[str] = Field(default=None, description="""kitchen | bath | roof | addition | gut | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation', 'Permit']} })
    completed_year: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    completed_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    cost: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Site(Entity):
    """
    1:1 with property.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    lot_size: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    usable_land_area: Optional[Area] = Field(default=None, description="""Usable portion of the lot; see usable_land_area_basis""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    usable_land_area_basis: Optional[str] = Field(default=None, description="""surveyed | buildable | net_of_constraints | ... (what \"usable\" means here)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    land_use: Optional[CodeableConcept] = Field(default=None, description="""County code + label, system-tagged""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    land_use_category: Optional[str] = Field(default=None, description="""Standardized (PUCS land classes)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    zoning_code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    flood_zone: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    hazard_zones: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    view_types: Optional[list[str]] = Field(default=None, description="""UAD ViewBase-seeded""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    site_influences: Optional[list[str]] = Field(default=None, description="""UAD SiteInfluenceBase-seeded""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    easements: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    restrictions: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    utilities: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    frontage: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    depth: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    topography: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    is_corner: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    entitlement_status: Optional[str] = Field(default=None, description="""raw | entitled | permitted | ... (open; construction/dev)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    buildable_units: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    subdivision: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    lot_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    block: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    tract_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    phase_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    section_township_range: Optional[str] = Field(default=None, description="""US PLSS (US-specific, optional)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Site']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Space(Entity):
    """
    Leasable suites/units (CRE, multifamily).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'space_identity': {'unique_key_name': 'space_identity',
                                            'unique_key_slots': ['property',
                                                                 'space_identifier']}}})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    structure: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'Permit']} })
    space_identifier: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    floor_number: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    space_use: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    rentable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Structure', 'Space']} })
    usable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'UnitRentObservation']} })
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'Space']} })
    is_adu: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    is_active: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PropertyAssociation(Entity):
    """
    HOA / property association — minimal v1 footprint.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    party: Optional[str] = Field(default=None, description="""The association as an organization""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    fee: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyAssociation']} })
    fee_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyAssociation']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Assessment(Entity):
    """
    Parcel-first tax-roll values; identity = parcel + assessing jurisdiction + year + roll type.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'assessment_identity': {'unique_key_name': 'assessment_identity',
                                                 'unique_key_slots': ['parcel',
                                                                      'jurisdiction',
                                                                      'tax_year',
                                                                      'roll_type']}}})

    parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    jurisdiction: str = Field(default=..., description="""The ASSESSING authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill']} })
    tax_year: int = Field(default=..., ge=1000, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment', 'TaxBill']} })
    roll_type: Optional[str] = Field(default=None, description="""original | corrected | supplemental | appeal | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    assessed_land_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    assessed_improvement_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    assessed_total_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    market_land_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    market_improvement_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    market_total_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    appraised_land_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    appraised_improvement_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    appraised_total_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    exemptions: Optional[list[TaxExemption]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TaxExemption(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., description="""homestead | senior | veteran | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TaxBill(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'tax_bill_identity': {'unique_key_name': 'tax_bill_identity',
                                               'unique_key_slots': ['parcel',
                                                                    'jurisdiction',
                                                                    'tax_year',
                                                                    'bill_number']}}})

    parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    jurisdiction: str = Field(default=..., description="""Issuing authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill']} })
    tax_year: int = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Assessment', 'TaxBill']} })
    bill_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    amount_billed: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    amount_paid: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'TaxInstallment']} })
    is_delinquent: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'TaxInstallment']} })
    delinquent_year: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    delinquent_amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    rate_code_area: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    installments: Optional[list[TaxInstallment]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill']} })
    line_items: Optional[list[TaxLineItem]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'OperatingStatement']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TaxInstallment(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    installment_number: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxInstallment']} })
    due_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxInstallment']} })
    amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    paid_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxInstallment']} })
    amount_paid: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'TaxInstallment']} })
    is_delinquent: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'TaxInstallment']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TaxLineItem(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    authority: Optional[str] = Field(default=None, description="""Taxing authority name or code""", json_schema_extra = { "linkml_meta": {'domain_of': ['TaxLineItem']} })
    rate: Optional[Decimal] = Field(default=None, description="""Mill rate / levy""", json_schema_extra = { "linkml_meta": {'domain_of': ['TaxLineItem', 'UnitRentObservation']} })
    amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Transfer(RecordedInstrument, Entity):
    """
    Every recorded deed/instrument — the ownership ledger, NOT comps.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    transfer_kind: str = Field(default=..., description="""warranty_deed | quitclaim | foreclosure | tax_deed | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    effective_on: Optional[date] = Field(default=None, description="""Legal/economic effectiveness — may differ from instrument_date and recorded_on""", json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage', 'Transfer']} })
    consideration: Optional[Money] = Field(default=None, description="""Often $0 / nominal""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    transfer_tax: Optional[Money] = Field(default=None, description="""Doc stamps; price-inference basis in many places""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    price_disclosure: Optional[PriceDisclosure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    price_code: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    partial_interest_pct: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    is_inter_family: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    is_distressed: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    parties: Optional[list[TransferParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TransferParty(TransactionParty):
    """
    role = grantor | grantee
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class SaleEvent(Entity):
    """
    A market sale referencing its recorded transfer. A quitclaim never becomes a comp.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Loan']} })
    sale_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent'], 'exact_mappings': ['reso:CloseDate']} })
    sale_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent'], 'exact_mappings': ['reso:ClosePrice']} })
    price_disclosure: Optional[PriceDisclosure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    price_code: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    sale_type: Optional[SaleTypeEnum] = Field(default=SaleTypeEnum.arms_length, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent'], 'ifabsent': 'string(arms_length)'} })
    price_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    price_per_unit: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    financing: Optional[str] = Field(default=None, description="""cash | conventional | seller | assumption | other (coarse; loans carry detail)""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    concessions: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'LeaseEvent']} })
    cap_rate: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    noi_at_sale: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    opex_at_sale: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    occupancy_at_sale_pct: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    unit_count_at_sale: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    supporting_operating_statement: Optional[str] = Field(default=None, description="""Traceability: the statement noi_at_sale derives from, when known""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    parties: Optional[list[SaleEventParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class SaleEventParty(TransactionParty):
    """
    role = buyer | seller | buyer_broker | seller_broker
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Listing(Entity):
    """
    Listing header. Lifecycle lives in events[]; header status/list_price are denormalized conveniences reconstructible from events.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    kind: ListingKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    listing_type: Optional[str] = Field(default=None, description="""mls | fsbo | auction | pocket""", json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    status: Optional[ListingStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    original_list_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    list_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing', 'ListingEvent']} })
    list_rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    list_rent_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    listed_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    closed_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    close_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    mls_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    events: Optional[list[ListingEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing', 'Loan']} })
    participants: Optional[list[ListingParticipant]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ListingEvent(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    occurred_on: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    event_kind: str = Field(default=..., description="""listed | price_change | status_change | relisted | closed""", json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    status: Optional[ListingStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    list_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing', 'ListingEvent']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ListingParticipant(TransactionParty):
    """
    role = listing_agent | co_listing_agent | buyer_agent | listing_brokerage | selling_brokerage
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LeaseEvent(Entity):
    """
    Executed leases (not asking rents — see UnitRentObservation).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    space: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    lease_type: Optional[LeaseTypeEnum] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    execution_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    commencement_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    expiration_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    term_months: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'Loan']} })
    leased_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    rent_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    starting_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    effective_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    net_effective_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    free_rent_months: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    ti_allowance_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    expense_structure: Optional[ExpenseStructure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    lessee_industry: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    parties: Optional[list[LeaseEventParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    escalations: Optional[list[LeaseEscalation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    concessions: Optional[list[LeaseConcession]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'LeaseEvent']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LeaseEventParty(TransactionParty):
    """
    role = lessee | landlord | guarantor | lessee_broker | landlord_broker
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ExpenseStructure(ConfiguredBaseModel):
    """
    Who pays what (taxes/insurance/CAM/utilities).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    taxes_paid_by: Optional[str] = Field(default=None, description="""landlord | tenant | shared""", json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure']} })
    insurance_paid_by: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure']} })
    cam_paid_by: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure']} })
    utilities_paid_by: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure']} })
    notes: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure', 'LeaseConcession']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LeaseEscalation(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    escalation_type: str = Field(default=..., description="""fixed_amount | fixed_percent | cpi | step_schedule | fmv | none""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    escalation_value: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    frequency_months: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_index: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_floor: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_cap: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    steps: Optional[list[RentStep]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    effective_from: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    effective_until: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class RentStep(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    from_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['RentStep']} })
    amount: Money = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })


class LeaseConcession(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    concession_type: str = Field(default=..., description="""free_rent | ti_allowance | moving_allowance | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    concession_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    abatement_months: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    abatement_percent: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    ti_cap_total: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    conditions: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    notes: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure', 'LeaseConcession']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class UnitRentObservation(Entity):
    """
    Advertised/going rates — floorplans, storage units, slips. NOT executed leases.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    unit_type: str = Field(default=..., description="""'1BR/1BA', '10x10'""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'UnitRentObservation']} })
    unit_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'UnitRentObservation']} })
    unit_count: Optional[int] = Field(default=None, description="""Units of this type""", json_schema_extra = { "linkml_meta": {'domain_of': ['Structure', 'UnitRentObservation']} })
    units_available: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    rate: Money = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TaxLineItem', 'UnitRentObservation']} })
    rate_period: Optional[RentPeriod] = Field(default=RentPeriod.monthly, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation'], 'ifabsent': 'string(monthly)'} })
    rate_basis: Optional[RateBasis] = Field(default=RateBasis.per_unit, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation'], 'ifabsent': 'string(per_unit)'} })
    rate_type: Optional[RateType] = Field(default=RateType.asking, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation'], 'ifabsent': 'string(asking)'} })
    observed_on: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    concessions_note: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Loan(RecordedInstrument, Entity):
    """
    Recorded debt. lender_name is the immutable ORIGINATING lender; assignments and status changes are dated loan events. status is a derived projection of events.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    transfer: Optional[str] = Field(default=None, description="""Purchase-money linkage""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Loan']} })
    is_purchase_money: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    loan_amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    lender_name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    lender_type: Optional[str] = Field(default=None, description="""bank | credit_union | private | seller | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    loan_type: Optional[str] = Field(default=None, description="""conventional | fha | va | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    purpose: Optional[str] = Field(default=None, description="""purchase | refinance | construction | heloc | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_heloc: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_construction: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_seller_carryback: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_assumable: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    interest_rate: Optional[Decimal] = Field(default=None, ge=0, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_variable_rate: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    term_months: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'Loan']} })
    due_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    lien_position: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    status: Optional[LoanStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    satisfied_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    parties: Optional[list[LoanParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    events: Optional[list[LoanEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing', 'Loan']} })
    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LoanParty(TransactionParty):
    """
    role = borrower | lender | beneficiary | trustee (assignees live in loan events)
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LoanEvent(RecordedInstrument):
    """
    Dated loan lifecycle — assignments, modifications, satisfactions. Recording fields apply when the event is a recorded instrument; they stay null for unrecorded servicing events.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    event_kind: LoanEventKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    occurred_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    to_name: Optional[str] = Field(default=None, description="""Assignee lender name (assignments)""", json_schema_extra = { "linkml_meta": {'domain_of': ['LoanEvent']} })
    to_party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LoanEvent']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })


class Lien(RecordedInstrument, Entity):
    """
    Involuntary encumbrances — tax, judgment, HOA, mechanic's.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    kind: LienKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    released_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Lien']} })
    parties: Optional[list[LienParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LienParty(TransactionParty):
    """
    role = claimant | debtor
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ForeclosureCase(Entity):
    """
    A foreclosure PROCEEDING; its recorded filings append over time.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    loan: Optional[str] = Field(default=None, description="""The defaulted loan, when known""", json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    case_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    opened_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    resolved_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    resolution: Optional[str] = Field(default=None, description="""sold_at_auction | cured | dismissed | reo""", json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    past_due_amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    unpaid_balance: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    original_loan_amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    auction_min_bid: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    auction_location: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    filings: Optional[list[ForeclosureFiling]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureCase']} })
    parties: Optional[list[ForeclosureCaseParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ForeclosureFiling(RecordedInstrument):
    """
    One recorded filing; postponements append as new filings.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    status: str = Field(default=..., description="""nod | lis_pendens | notice_of_sale | auction_scheduled | postponement | ... (open; US-seeded)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    auction_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureFiling']} })
    auction_at_time: Optional[str] = Field(default=None, description="""Time-of-day as published""", json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureFiling']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    document_number: Optional[str] = Field(default=None, description="""Primary identifier assigned by the recording/registry authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    recording_book: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_page: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recorded_on: Optional[date] = Field(default=None, description="""Date accepted, recorded, or registered by the authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    instrument_date: Optional[date] = Field(default=None, description="""Date executed/signed as dated on the instrument""", json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    document_type: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })
    recording_authority: Optional[str] = Field(default=None, description="""Authority maintaining the record (optional — parcel context is inference, not identity)""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    registry_reference: Optional[str] = Field(default=None, description="""Alternate authority-issued reference: title, dealing, folio, or notarial-act number""", json_schema_extra = { "linkml_meta": {'domain_of': ['InstrumentReference', 'RecordedInstrument']} })
    related_instruments: Optional[list[InstrumentReference]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument']} })


class ForeclosureCaseParty(TransactionParty):
    """
    role = lender | trustee | borrower
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty',
                       'Jurisdiction',
                       'Property',
                       'Party',
                       'Structure',
                       'PropertyAssociation']} })
    party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Permit(Entity):
    """
    Building permits — header + lifecycle dates. Inspections/projects are extension.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    structure: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'Permit']} })
    permitting_jurisdiction: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    permit_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    kind: Optional[str] = Field(default=None, description="""roofing | solar | hvac | adu | pool | new_construction | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    status: Optional[str] = Field(default=None, description="""issued | finaled | expired | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation', 'Permit']} })
    applied_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    issued_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    finaled_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    expires_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    job_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    fees: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    contractor_name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    contractor_license: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    contractor_party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class OperatingStatement(Entity):
    """
    Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    statement_year: int = Field(default=..., description="""The calendar year the statement is for (the year containing period_end for fiscal/trailing periods).""", ge=1000, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    period_start: Optional[date] = Field(default=None, description="""For fiscal-year, trailing-12, or partial periods. period_start and period_end must be provided together; omit both for calendar-year statements.""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    period_end: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    statement_basis: Optional[StatementBasis] = Field(default=StatementBasis.actual, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement'], 'ifabsent': 'string(actual)'} })
    pgi: Optional[Money] = Field(default=None, description="""Potential gross income""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    vacancy_loss: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    vacancy_pct: Optional[Decimal] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    egi: Optional[Money] = Field(default=None, description="""Effective gross income""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    opex_total: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    noi: Optional[Money] = Field(default=None, description="""Net operating income; may be negative""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    capex: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    reimbursements: Optional[Money] = Field(default=None, description="""Expense recoveries from tenants (CRE)""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    reserves: Optional[Money] = Field(default=None, description="""Reserves for replacement""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    reserves_included_in_opex: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    ground_lease_expense: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    ground_lease_included_in_opex: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    line_items: Optional[list[StatementLineItem]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['TaxBill', 'OperatingStatement']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class StatementLineItem(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    category: str = Field(default=..., description="""income | expense | capital | other (open vocab)""", json_schema_extra = { "linkml_meta": {'domain_of': ['StatementLineItem', 'ExtractionObservation']} })
    label: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PartyContact', 'StatementLineItem']} })
    amount: Money = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Money',
                       'UnitRate',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'RentStep',
                       'LoanEvent',
                       'Lien',
                       'StatementLineItem']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class Valuation(Entity):
    """
    Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    kind: ValuationKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'PartyAddress',
                       'PartyContact',
                       'Structure',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    valuation_method: Optional[str] = Field(default=None, description="""desktop | exterior | hybrid | traditional""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value_type: Optional[str] = Field(default=None, description="""market_value | liquidation | insurable | land | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value: Money = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    value_low: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value_high: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    land_value: Optional[Money] = Field(default=None, description="""Cost-approach site value""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    confidence_score: Optional[int] = Field(default=None, ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    forecast_standard_deviation: Optional[Decimal] = Field(default=None, description="""AVM FSD""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    exposure_days: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_sales_comparison: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_cost: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_income: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value_premise: Optional[str] = Field(default=None, description="""as_is | as_completed | as_stabilized""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    interest: Optional[str] = Field(default=None, description="""fee_simple | leased_fee | leasehold""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    performed_by: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    performed_by_party: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    as_of_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    report_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    id: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class PropertyProfile(ConfiguredBaseModel):
    """
    The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any event (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/profiles', 'tree_root': True})

    parties: Optional[list[Party]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    addresses: Optional[list[Address]] = Field(default=None, description="""Address bundle referenced by property/parties/ownership""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party', 'PropertyProfile']} })
    property: Property = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyAssociation',
                       'Transfer',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'UnitRentObservation',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'Permit',
                       'OperatingStatement',
                       'Valuation',
                       'PropertyProfile']} })
    identifiers: Optional[list[PropertyIdentifier]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    jurisdictions: Optional[list[Jurisdiction]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    parcels: Optional[list[Parcel]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    property_parcels: Optional[list[PropertyParcel]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    parcel_lineage: Optional[list[ParcelLineage]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    site: Optional[Site] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    structures: Optional[list[Structure]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    spaces: Optional[list[Space]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    associations: Optional[list[PropertyAssociation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    assessments: Optional[list[Assessment]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    tax_bills: Optional[list[TaxBill]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    transfers: Optional[list[Transfer]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    sales: Optional[list[SaleEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    listings: Optional[list[Listing]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    leases: Optional[list[LeaseEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    unit_rents: Optional[list[UnitRentObservation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    loans: Optional[list[Loan]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    liens: Optional[list[Lien]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    foreclosure_cases: Optional[list[ForeclosureCase]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    permits: Optional[list[Permit]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    ownership: Optional[list[OwnershipPeriod]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    operating_statements: Optional[list[OperatingStatement]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    valuations: Optional[list[Valuation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    provenance: Optional[Provenance] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class AssessorObservation(ConfiguredBaseModel):
    """
    Thin capture envelope for a county assessor / public-records fetch. The payload is a partial PropertyProfile; status reports the fetch outcome.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/profiles'})

    status: AssessorStatus = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Listing',
                       'ListingEvent',
                       'Loan',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation']} })
    query_address: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    query_parcel_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    assessor_url: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    profile: Optional[PropertyProfile] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
    error: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    provenance: Provenance = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ExtractionObservation(ConfiguredBaseModel):
    """
    Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a partial PropertyProfile; category classifies the source page.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/profiles'})

    category: str = Field(default=..., description="""sales_transaction | sale_listing | lease_listing | in_place_lease | public_data | other""", json_schema_extra = { "linkml_meta": {'domain_of': ['StatementLineItem', 'ExtractionObservation']} })
    source_url: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance', 'ExtractionObservation']} })
    extracted_at: Optional[datetime ] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExtractionObservation']} })
    model: Optional[str] = Field(default=None, description="""Extraction model identifier""", json_schema_extra = { "linkml_meta": {'domain_of': ['ExtractionObservation']} })
    profile: Optional[PropertyProfile] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
    provenance: Provenance = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'PartyAddress',
                       'PartyContact',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'ListingEvent',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'PartyAddress',
                       'PartyContact',
                       'OwnershipInterest',
                       'AreaMeasure',
                       'ResidentialDetails',
                       'CommercialDetails',
                       'Renovation',
                       'TaxExemption',
                       'TaxInstallment',
                       'TaxLineItem',
                       'ListingEvent',
                       'ExpenseStructure',
                       'LeaseEscalation',
                       'LeaseConcession',
                       'LoanEvent',
                       'ForeclosureFiling',
                       'StatementLineItem',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


# Model rebuild
# see https://pydantic-docs.helpmanual.io/usage/models/#rebuilding-a-model
Money.model_rebuild()
Area.model_rebuild()
Length.model_rebuild()
UnitRate.model_rebuild()
CodeableConcept.model_rebuild()
GeoPoint.model_rebuild()
Geometry.model_rebuild()
Provenance.model_rebuild()
Entity.model_rebuild()
InstrumentReference.model_rebuild()
RecordedInstrument.model_rebuild()
TransactionParty.model_rebuild()
Jurisdiction.model_rebuild()
Address.model_rebuild()
Property.model_rebuild()
Parcel.model_rebuild()
PropertyParcel.model_rebuild()
ParcelLineage.model_rebuild()
PropertyIdentifier.model_rebuild()
Party.model_rebuild()
PartyAddress.model_rebuild()
PartyContact.model_rebuild()
OwnershipPeriod.model_rebuild()
OwnershipInterest.model_rebuild()
Structure.model_rebuild()
AreaMeasure.model_rebuild()
ResidentialDetails.model_rebuild()
CommercialDetails.model_rebuild()
Renovation.model_rebuild()
Site.model_rebuild()
Space.model_rebuild()
PropertyAssociation.model_rebuild()
Assessment.model_rebuild()
TaxExemption.model_rebuild()
TaxBill.model_rebuild()
TaxInstallment.model_rebuild()
TaxLineItem.model_rebuild()
Transfer.model_rebuild()
TransferParty.model_rebuild()
SaleEvent.model_rebuild()
SaleEventParty.model_rebuild()
Listing.model_rebuild()
ListingEvent.model_rebuild()
ListingParticipant.model_rebuild()
LeaseEvent.model_rebuild()
LeaseEventParty.model_rebuild()
ExpenseStructure.model_rebuild()
LeaseEscalation.model_rebuild()
RentStep.model_rebuild()
LeaseConcession.model_rebuild()
UnitRentObservation.model_rebuild()
Loan.model_rebuild()
LoanParty.model_rebuild()
LoanEvent.model_rebuild()
Lien.model_rebuild()
LienParty.model_rebuild()
ForeclosureCase.model_rebuild()
ForeclosureFiling.model_rebuild()
ForeclosureCaseParty.model_rebuild()
Permit.model_rebuild()
OperatingStatement.model_rebuild()
StatementLineItem.model_rebuild()
Valuation.model_rebuild()
PropertyProfile.model_rebuild()
AssessorObservation.model_rebuild()
ExtractionObservation.model_rebuild()
