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

import math
from pydantic import field_serializer


metamodel_version = "1.11.0"
version = "0.2.0"


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

    @field_serializer("*", when_used="json", check_fields=False)
    def serialize_decimals_as_json_numbers(self, value: Any) -> Any:
        if isinstance(value, Decimal):
            if not value.is_finite():
                raise ValueError(
                    f"Decimal {value!r} is not a finite JSON number"
                )
            if value == value.to_integral_value():
                return int(value)
            candidate = float(value)
            if not math.isfinite(candidate) or Decimal(str(candidate)) != value:
                raise ValueError(
                    f"Decimal {value!r} cannot be represented exactly as a JSON number"
                )
            return candidate
        return value





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
     'id': 'https://example.org/phds',
     'imports': ['linkml:types', 'profiles'],
     'license': 'MIT',
     'name': 'phds',
     'prefixes': {'linkml': {'prefix_prefix': 'linkml',
                             'prefix_reference': 'https://w3id.org/linkml/'},
                  'phds': {'prefix_prefix': 'phds',
                           'prefix_reference': 'https://example.org/phds/'}},
     'source_file': 'schema/capture.yaml',
     'title': 'PHDS'} )

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


class PartyKind(str, Enum):
    person = "person"
    organization = "organization"


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
    market = "market"
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
    leased_fee = "leased_fee"
    leasehold = "leasehold"
    life_estate = "life_estate"
    cooperative_shares = "cooperative_shares"
    other = "other"


class RatingScope(str, Enum):
    overall = "overall"
    exterior = "exterior"
    interior = "interior"
    component = "component"
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


class ExtractionStatus(str, Enum):
    """
    Outcome of an extraction attempt over already-fetched content. Fetch-level failures (timeout, api_error) belong to the envelope that did the fetching, e.g. AssessorStatus.
    """
    success = "success"
    parse_error = "parse_error"
    """
    Content was retrieved but extraction failed
    """
    irrelevant_page = "irrelevant_page"
    """
    Content contains no extractable property data
    """
    model_error = "model_error"
    """
    The extraction model or tooling failed
    """


class ExtractionCategory(str, Enum):
    """
    What kind of property content an extraction produced. This is a content axis only — where the content came from (public record, vendor, scrape) is Provenance's job. Precedence: when the primary extracted content is a transaction, listing, or lease, use that value even if the page is also a record of property facts. `other` means successfully classified but outside this taxonomy; put the producer's raw label in source_category.
    """
    sales_transaction = "sales_transaction"
    sale_listing = "sale_listing"
    lease_listing = "lease_listing"
    in_place_lease = "in_place_lease"
    property_facts = "property_facts"
    """
    Property characteristics, assessment, or tax facts with no primary transaction/listing/lease content
    """
    other = "other"


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
                       'StatementLineItem'],
         'slot_uri': 'phds:decimal_amount'} })
    currency: str = Field(default=..., description="""ISO-4217 currency code""", json_schema_extra = { "linkml_meta": {'domain_of': ['Money', 'UnitRate']} })

    @field_validator('amount')
    def pattern_amount(cls, v):
        pattern=re.compile(r"^-?[0-9]+(\.[0-9]+)?(?![\s\S])")
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
        pattern=re.compile(r"^[A-Z]{3}(?![\s\S])")
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
                       'StatementLineItem'],
         'slot_uri': 'phds:decimal_amount'} })
    currency: str = Field(default=..., description="""ISO-4217 currency code""", json_schema_extra = { "linkml_meta": {'domain_of': ['Money', 'UnitRate']} })
    denominator: str = Field(default=..., description="""sqft | sqm | unit | bed | key | month | sqft_month | 1000_sqft | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRate']} })

    @field_validator('amount')
    def pattern_amount(cls, v):
        pattern=re.compile(r"^-?[0-9]+(\.[0-9]+)?(?![\s\S])")
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
        pattern=re.compile(r"^[A-Z]{3}(?![\s\S])")
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

    system: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    display: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })


class Classification(CodeableConcept):
    """
    An open-vocabulary classification qualified by the system that defines its code. Codes from different systems are not assumed equivalent.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    system: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    code: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    display: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })

    @field_validator('system')
    def pattern_system(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid system format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid system format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('code')
    def pattern_code(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid code format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid code format: {v}"
            raise ValueError(err_msg)
        return v


class Rating(CodeableConcept):
    """
    A rating qualified by the system that defines its code. PHDS does not assume codes from different systems are ordinally or semantically equal.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/core'})

    system: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    code: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept', 'Classification', 'Rating']} })
    description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Rating', 'Renovation', 'Permit']} })
    scope: Optional[RatingScope] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Rating']} })
    display: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CodeableConcept']} })

    @field_validator('system')
    def pattern_system(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid system format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid system format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('code')
    def pattern_code(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid code format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid code format: {v}"
            raise ValueError(err_msg)
        return v


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
    confidence: Optional[float] = Field(default=None, description="""Fraction from 0 through 1; 0.8 means 80 percent confidence.""", ge=0, le=1, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })
    verification: Optional[VerificationStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Provenance']} })


class Entity(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'abstract': True, 'from_schema': 'https://example.org/phds/entities'})

    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })


class TransactionParty(ConfiguredBaseModel):
    """
    A contextual role-bearing relationship from an event or record to a canonical Party. Exact source wording belongs in provenance or source artifacts, not in a second actor-name field. Multi-party is the norm (couples on deeds, co-borrowers).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'abstract': True, 'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class Jurisdiction(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    country: str = Field(default=..., description="""ISO 3166-1 alpha-2 (no default — i18n)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    region: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    name: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    kind: str = Field(default=..., description="""county | municipality | taxing_district | ... (open vocab)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('country')
    def pattern_country(cls, v):
        pattern=re.compile(r"^[A-Z]{2}(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid country format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid country format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class Address(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    country: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Address']} })
    unformatted_address: Optional[str] = Field(default=None, description="""Fallback when components do not parse""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
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
    admin_area_code: Optional[str] = Field(default=None, description="""authority code""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    address_hash: Optional[str] = Field(default=None, description="""Producer-computed matching key; comparable only under the same scheme""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    address_hash_scheme: Optional[str] = Field(default=None, description="""Producer-namespaced normalization and hashing scheme""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'PropertyFacts']} })
    location_accuracy: Optional[GeocodeAccuracy] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('country')
    def pattern_country(cls, v):
        pattern=re.compile(r"^[A-Z]{2}(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid country format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid country format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('address_hash')
    def pattern_address_hash(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid address_hash format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid address_hash format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('address_hash_scheme')
    def pattern_address_hash_scheme(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid address_hash_scheme format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid address_hash_scheme format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyFacts(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    property_use_class: Optional[str] = Field(default=None, description="""PUCS class when property_use_system identifies PUCS""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_subtype: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_system: Optional[str] = Field(default=None, description="""Required when use fields are set; no default""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    estate_type: Optional[EstateType] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'PropertyFacts']} })
    building_count: Optional[int] = Field(default=None, description="""Producer summary when structures are not enumerated""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })


class Property(PropertyFacts, Entity):
    """
    Stable property identity; mutable descriptive fields come from PropertyFacts.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['PropertyFacts']})

    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    property_use_class: Optional[str] = Field(default=None, description="""PUCS class when property_use_system identifies PUCS""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_subtype: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_system: Optional[str] = Field(default=None, description="""Required when use fields are set; no default""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    estate_type: Optional[EstateType] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'PropertyFacts']} })
    building_count: Optional[int] = Field(default=None, description="""Producer summary when structures are not enumerated""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class Parcel(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'unique_keys': {'parcel_identity': {'unique_key_name': 'parcel_identity',
                                             'unique_key_slots': ['jurisdiction',
                                                                  'parcel_number',
                                                                  'unit_designator',
                                                                  'retired_on']}}})

    jurisdiction: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill', 'TaxLineItem']} })
    parcel_number: str = Field(default=..., description="""RAW as issued (RESO UPI rule)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel'], 'exact_mappings': ['reso:ParcelNumber']} })
    normalized_parcel_number: Optional[str] = Field(default=None, description="""Matching only, never identity""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    unit_designator: Optional[str] = Field(default=None, description="""Condo sub-parcel discriminator""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    reso_upi: Optional[str] = Field(default=None, description="""urn:reso:upi:2.0:...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    legal_description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    land_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    boundary: Optional[Geometry] = Field(default=None, description="""GeoJSON MultiPolygon (optional)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'Parcel']} })
    retired_on: Optional[date] = Field(default=None, description="""Set by lineage events""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'AddressAssociation', 'PartyContact']} })
    started_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    ended_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class ParcelLineage(Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    predecessor_parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage']} })
    successor_parcel: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage']} })
    kind: ParcelLineageKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    effective_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage', 'Transfer']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    scheme: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyIdentifier']} })
    namespace: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyIdentifier']} })
    value: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Area',
                       'Length',
                       'PropertyIdentifier',
                       'PartyContact',
                       'Valuation']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class Party(Entity):
    """
    One canonical model for every actor — owners, buyers, tenants and lessees, borrowers, lenders, brokers, trustees, claimants, contractors, associations, and valuation performers.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: PartyKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    legal_form: Optional[Classification] = Field(default=None, description="""Optional system-qualified legal form for an organization under an identified jurisdictional or producer vocabulary. This is not an industry classification or a contextual role such as lender, broker, tenant, or association.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name: str = Field(default=..., description="""Canonical display name for this Party in the profile; nonblank with no leading or trailing whitespace. Source-specific wording is attributed through provenance or SourceArtifact.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    normalized_name: Optional[str] = Field(default=None, description="""Producer-derived matching key for the canonical display name. Its normalization algorithm is producer-defined; it is not authoritative display text or a separate identity.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_first: Optional[str] = Field(default=None, description="""Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_middle: Optional[str] = Field(default=None, description="""Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    name_last: Optional[str] = Field(default=None, description="""Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    classifications: Optional[list[Classification]] = Field(default=None, description="""System-qualified actor classifications from an open vocabulary; use producer-namespaced system and code values""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    addresses: Optional[list[PartyAddress]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party', 'PropertyProfile']} })
    contacts: Optional[list[PartyContact]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Party']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('name')
    def pattern_name(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid name format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid name format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class VerificationAttribution(ConfiguredBaseModel):
    """
    Verification performed by one canonical party.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    verifier: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['VerificationAttribution']} })
    verified_at: datetime  = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['VerificationAttribution']} })
    note: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['VerificationAttribution']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('verifier')
    def pattern_verifier(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid verifier format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid verifier format: {v}"
            raise ValueError(err_msg)
        return v


class SourceArtifact(Entity):
    """
    Source material that supports or preserves an assertion. Semantic validation requires at least one nonblank uri or storage_reference.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    uri: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    storage_reference: Optional[str] = Field(default=None, description="""Producer-defined object or document storage reference""", json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    media_type: Optional[str] = Field(default=None, description="""MIME media type""", json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    kind: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    title: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    original_filename: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    content_hash: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    hash_scheme: Optional[str] = Field(default=None, description="""Producer-namespaced content hashing scheme""", json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    page_count: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    captured_on: Optional[datetime ] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SourceArtifact']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('uri')
    def pattern_uri(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid uri format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid uri format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('storage_reference')
    def pattern_storage_reference(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid storage_reference format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid storage_reference format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('content_hash')
    def pattern_content_hash(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid content_hash format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid content_hash format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('hash_scheme')
    def pattern_hash_scheme(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid hash_scheme format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid hash_scheme format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class AddressAssociation(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    address: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    role: Optional[CodeableConcept] = Field(default=None, description="""situs | entrance | alias | address_range | former | mailing | other (open vocabulary)""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'AddressAssociation', 'PartyContact']} })
    valid_from: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    valid_to: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })

    @field_validator('address')
    def pattern_address(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid address format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid address format: {v}"
            raise ValueError(err_msg)
        return v


class PartyAddress(AddressAssociation):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['AddressAssociation']})

    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    address: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    role: Optional[CodeableConcept] = Field(default=None, description="""situs | entrance | alias | address_range | former | mailing | other (open vocabulary)""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'AddressAssociation', 'PartyContact']} })
    valid_from: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    valid_to: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })

    @field_validator('address')
    def pattern_address(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid address format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid address format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyAddress(AddressAssociation, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['AddressAssociation']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    address: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    role: Optional[CodeableConcept] = Field(default=None, description="""situs | entrance | alias | address_range | former | mailing | other (open vocabulary)""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'AddressAssociation', 'PartyContact']} })
    valid_from: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    valid_to: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AddressAssociation']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('address')
    def pattern_address(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid address format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid address format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PartyContact(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., description="""phone | email | website | other""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
    is_primary: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'AddressAssociation', 'PartyContact']} })
    do_not_contact: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PartyContact']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    started_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    ended_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'OwnershipPeriod']} })
    vesting_type: Optional[str] = Field(default=None, description="""joint_tenants | tenants_in_common | community_property | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    mailing_address: Optional[str] = Field(default=None, description="""Owner's mailing address during THIS period""", json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    acquired_via_transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    disposed_via_transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    interests: Optional[list[OwnershipInterest]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipPeriod']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('mailing_address')
    def pattern_mailing_address(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid mailing_address format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid mailing_address format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class OwnershipInterest(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    interest_pct: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 75 means 75 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipInterest']} })
    role: Optional[str] = Field(default=None, description="""owner | trustee | gp | lp""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    is_owner_occupied: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OwnershipInterest']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class StructureFacts(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    kind: Optional[str] = Field(default=None, description="""building | barn | silo | shed | outbuilding | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    structure_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    living_area: Optional[Area] = Field(default=None, description="""Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:LivingArea']} })
    gross_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    rentable_area: Optional[Area] = Field(default=None, description="""Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    ground_floor_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_finished_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    garage_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    areas: Optional[list[AreaMeasure]] = Field(default=None, description="""Long-tail area kinds; canonical kinds forbidden here (validator-enforced)""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    year_built: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:YearBuilt']} })
    year_built_estimated: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    effective_year_built: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    stories: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    unit_count: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'UnitRentObservation', 'RentRoll']} })
    construction_method: Optional[str] = Field(default=None, description="""site_built | manufactured | modular | container | 3d_printed | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_status: Optional[str] = Field(default=None, description="""complete | proposed | under_construction""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    exterior_wall_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_material_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_style_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_material: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    condition_ratings: Optional[list[Rating]] = Field(default=None, description="""Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    quality_ratings: Optional[list[Rating]] = Field(default=None, description="""Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_fuel_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    cooling_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    sewer_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    water_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    features: Optional[list[str]] = Field(default=None, description="""Open vocabulary for physical features and amenities""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    residential: Optional[ResidentialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    commercial: Optional[CommercialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    renovations: Optional[list[Renovation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })


class Structure(StructureFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['StructureFacts']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    kind: Optional[str] = Field(default=None, description="""building | barn | silo | shed | outbuilding | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    structure_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    living_area: Optional[Area] = Field(default=None, description="""Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:LivingArea']} })
    gross_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    rentable_area: Optional[Area] = Field(default=None, description="""Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    ground_floor_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_finished_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    garage_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    areas: Optional[list[AreaMeasure]] = Field(default=None, description="""Long-tail area kinds; canonical kinds forbidden here (validator-enforced)""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    year_built: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:YearBuilt']} })
    year_built_estimated: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    effective_year_built: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    stories: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    unit_count: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'UnitRentObservation', 'RentRoll']} })
    construction_method: Optional[str] = Field(default=None, description="""site_built | manufactured | modular | container | 3d_printed | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_status: Optional[str] = Field(default=None, description="""complete | proposed | under_construction""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    exterior_wall_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_material_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_style_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_material: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    condition_ratings: Optional[list[Rating]] = Field(default=None, description="""Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    quality_ratings: Optional[list[Rating]] = Field(default=None, description="""Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_fuel_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    cooling_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    sewer_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    water_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    features: Optional[list[str]] = Field(default=None, description="""Open vocabulary for physical features and amenities""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    residential: Optional[ResidentialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    commercial: Optional[CommercialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    renovations: Optional[list[Renovation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class AreaMeasure(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ResidentialDetails(ConfiguredBaseModel):
    """
    Internationally neutral residential facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
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
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'SpaceFacts']} })
    renewable_energy_components: Optional[list[str]] = Field(default=None, description="""solar | geothermal | wind""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    Internationally neutral commercial facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Historical/per-period financial figures belong on dated records such as OperatingStatement and UnitRentObservation. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    market_classification: Optional[Rating] = Field(default=None, description="""Competitive market positioning under the named rating system; distinct from physical condition and construction quality.""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    clear_height: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    dock_doors: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    drive_in_doors: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    occupancy_pct: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 95 means 95 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails', 'RentRoll']} })
    parking_spaces: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'CommercialDetails']} })
    parking_ratio: Optional[UnitRate] = Field(default=None, description="""e.g. spaces per 1000 sqft — denominator explicit""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    tenancy: Optional[str] = Field(default=None, description="""single_tenant | multi_tenant""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    tenant_count: Optional[int] = Field(default=None, description="""Distinct legal tenants (not suites or leases); timing follows the containing Structure or StructureState context""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    parking_types: Optional[list[str]] = Field(default=None, description="""surface | structured | underground | covered | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    has_sprinkler: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    elevators: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    submarket: Optional[str] = Field(default=None, description="""CRE market geography""", json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Rating', 'Renovation', 'Permit']} })
    completed_year: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    completed_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    cost: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Renovation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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


class SiteFacts(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    lot_size: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area_basis: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use_category: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    zoning_code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    flood_zone: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    hazard_zones: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    view_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    site_influences: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    easements: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    restrictions: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    utilities: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    frontage: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    depth: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    topography: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    is_corner: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    entitlement_status: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    buildable_units: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    subdivision: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    lot_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    block: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    tract_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    phase_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    section_township_range: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })


class Site(SiteFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixins': ['SiteFacts']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    lot_size: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area_basis: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use_category: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    zoning_code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    flood_zone: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    hazard_zones: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    view_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    site_influences: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    easements: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    restrictions: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    utilities: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    frontage: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    depth: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    topography: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    is_corner: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    entitlement_status: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    buildable_units: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    subdivision: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    lot_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    block: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    tract_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    phase_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    section_township_range: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class SpaceFacts(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixin': True})

    floor_number: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    space_use: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    rentable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    usable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'SpaceFacts']} })
    is_adu: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    is_active: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })


class Space(SpaceFacts, Entity):
    """
    Leasable suites/units (CRE, multifamily).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['SpaceFacts'],
         'unique_keys': {'space_identity': {'unique_key_name': 'space_identity',
                                            'unique_key_slots': ['property',
                                                                 'space_identifier']}}})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    structure: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'Permit']} })
    space_identifier: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Space']} })
    floor_number: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    space_use: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    rentable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    usable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'SpaceFacts']} })
    is_adu: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    is_active: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyState(PropertyFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['PropertyFacts']})

    subject: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyState', 'SiteState', 'StructureState', 'SpaceState']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    property_use_class: Optional[str] = Field(default=None, description="""PUCS class when property_use_system identifies PUCS""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_subtype: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    property_use_system: Optional[str] = Field(default=None, description="""Required when use fields are set; no default""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    estate_type: Optional[EstateType] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    location: Optional[GeoPoint] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'PropertyFacts']} })
    building_count: Optional[int] = Field(default=None, description="""Producer summary when structures are not enumerated""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class SiteState(SiteFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixins': ['SiteFacts']})

    subject: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyState', 'SiteState', 'StructureState', 'SpaceState']} })
    lot_size: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    usable_land_area_basis: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    land_use_category: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    zoning_code: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    flood_zone: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    hazard_zones: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    view_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    site_influences: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    easements: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    restrictions: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    utilities: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    frontage: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    depth: Optional[Length] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    topography: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    is_corner: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    entitlement_status: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    buildable_units: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    subdivision: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    lot_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    block: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    tract_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    phase_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    section_township_range: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SiteFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class StructureState(StructureFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['StructureFacts']})

    subject: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyState', 'SiteState', 'StructureState', 'SpaceState']} })
    kind: Optional[str] = Field(default=None, description="""building | barn | silo | shed | outbuilding | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    name: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction', 'PropertyFacts', 'Party', 'StructureFacts']} })
    structure_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    living_area: Optional[Area] = Field(default=None, description="""Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:LivingArea']} })
    gross_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    rentable_area: Optional[Area] = Field(default=None, description="""Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    ground_floor_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    basement_finished_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    garage_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    areas: Optional[list[AreaMeasure]] = Field(default=None, description="""Long-tail area kinds; canonical kinds forbidden here (validator-enforced)""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    year_built: Optional[int] = Field(default=None, ge=1, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts'], 'exact_mappings': ['reso:YearBuilt']} })
    year_built_estimated: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    effective_year_built: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    stories: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    unit_count: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'UnitRentObservation', 'RentRoll']} })
    construction_method: Optional[str] = Field(default=None, description="""site_built | manufactured | modular | container | 3d_printed | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_status: Optional[str] = Field(default=None, description="""complete | proposed | under_construction""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    construction_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    exterior_wall_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_material_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    roof_style_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    foundation_material: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    condition_ratings: Optional[list[Rating]] = Field(default=None, description="""Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    quality_ratings: Optional[list[Rating]] = Field(default=None, description="""Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair.""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    heating_fuel_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    cooling_types: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    sewer_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    water_type: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    features: Optional[list[str]] = Field(default=None, description="""Open vocabulary for physical features and amenities""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    residential: Optional[ResidentialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    commercial: Optional[CommercialDetails] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    renovations: Optional[list[Renovation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class SpaceState(SpaceFacts, Entity):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities', 'mixins': ['SpaceFacts']})

    subject: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyState', 'SiteState', 'StructureState', 'SpaceState']} })
    floor_number: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    space_use: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    rentable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'SpaceFacts']} })
    usable_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    occupancy: Optional[str] = Field(default=None, description="""owner_occupied | tenant | vacant""", json_schema_extra = { "linkml_meta": {'domain_of': ['ResidentialDetails', 'SpaceFacts']} })
    is_adu: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    is_active: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyStateSnapshot(Entity):
    """
    Sparse asserted physical state effective on as_of_date.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    as_of_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot', 'RentRoll', 'Valuation']} })
    basis: Optional[CodeableConcept] = Field(default=None, description="""at_sale | at_listing | at_lease | inspection | reported | inferred (open vocabulary)""", json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot']} })
    property_state: Optional[PropertyState] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'Valuation']} })
    site_states: Optional[list[SiteState]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot']} })
    structure_states: Optional[list[StructureState]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot']} })
    space_states: Optional[list[SpaceState]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyAssociation(Entity):
    """
    HOA or property-association relationship; identity and classification live on the canonical Party.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    party: str = Field(default=..., description="""The canonical association organization""", json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    fee: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyAssociation']} })
    fee_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyAssociation']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
    jurisdiction: str = Field(default=..., description="""The ASSESSING authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill', 'TaxLineItem']} })
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
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class TaxExemption(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    kind: str = Field(default=..., description="""homestead | senior | veteran | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    jurisdiction: str = Field(default=..., description="""Issuing authority""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill', 'TaxLineItem']} })
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
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class TaxLineItem(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    jurisdiction: Optional[str] = Field(default=None, description="""Optional canonical taxing jurisdiction responsible for this line item""", json_schema_extra = { "linkml_meta": {'domain_of': ['Parcel', 'Assessment', 'TaxBill', 'TaxLineItem']} })
    rate: Optional[Decimal] = Field(default=None, description="""Source-defined tax rate or levy value; not governed by the _pct percentage-points convention.""", json_schema_extra = { "linkml_meta": {'domain_of': ['TaxLineItem', 'UnitRentObservation']} })
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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('jurisdiction')
    def pattern_jurisdiction(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid jurisdiction format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid jurisdiction format: {v}"
            raise ValueError(err_msg)
        return v


class Transfer(RecordedInstrument, Entity):
    """
    Every recorded deed/instrument — the ownership ledger, NOT comps.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    transfer_kind: str = Field(default=..., description="""warranty_deed | quitclaim | foreclosure | tax_deed | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    effective_on: Optional[date] = Field(default=None, description="""Legal/economic effectiveness — may differ from instrument_date and recorded_on""", json_schema_extra = { "linkml_meta": {'domain_of': ['ParcelLineage', 'Transfer']} })
    consideration: Optional[Money] = Field(default=None, description="""Often $0 / nominal""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    transfer_tax: Optional[Money] = Field(default=None, description="""Doc stamps; price-inference basis in many places""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    price_disclosure: Optional[PriceDisclosure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    price_code: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    interest_conveyed: Optional[CodeableConcept] = Field(default=None, description="""The legal or beneficial interest conveyed by this transfer.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
    partial_interest_pct: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 25 means 25 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer']} })
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class TransferParty(TransactionParty):
    """
    role = grantor | grantee
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class SaleEvent(Entity):
    """
    A market sale referencing its recorded transfer. A quitclaim never becomes a comp.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    property_state: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'Valuation']} })
    transfer: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Loan']} })
    sale_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent'], 'exact_mappings': ['reso:CloseDate']} })
    sale_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent'], 'exact_mappings': ['reso:ClosePrice']} })
    price_disclosure: Optional[PriceDisclosure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    price_code: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer', 'SaleEvent']} })
    sale_type: Optional[SaleTypeEnum] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    price_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    price_per_unit: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    financing: Optional[str] = Field(default=None, description="""cash | conventional | seller | assumption | other (coarse; loans carry detail)""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    concessions: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'LeaseEvent']} })
    cap_rate: Optional[Decimal] = Field(default=None, description="""Capitalization rate in percentage points; 5.75 means 5.75 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    noi_at_sale: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    opex_at_sale: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    occupancy_at_sale_pct: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 90 means 90 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    unit_count_at_sale: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    supporting_operating_statement: Optional[str] = Field(default=None, description="""Traceability: the statement noi_at_sale derives from, when known""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent']} })
    parties: Optional[list[SaleEventParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    remarks: Optional[str] = Field(default=None, description="""Source- or vendor-authored narrative interpreted through provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Listing', 'LeaseEvent']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class SaleEventParty(TransactionParty):
    """
    role = buyer | seller | buyer_broker | seller_broker
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class Listing(Entity):
    """
    Listing identity and non-lifecycle facts. Process events by occurred_on ascending; array order breaks same-date ties. Carry status, asking_price, and rent_period forward independently from the latest event that supplies each field. Original asking terms come from the earliest event supplying them. close_price comes from the latest closed event supplying it.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    property_state: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'Valuation']} })
    kind: ListingKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    listing_type: Optional[str] = Field(default=None, description="""mls | fsbo | auction | pocket""", json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    mls_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    events: Optional[list[ListingEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing', 'Loan'], 'list_elements_ordered': True} })
    participants: Optional[list[ListingParticipant]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Listing']} })
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    remarks: Optional[str] = Field(default=None, description="""Source- or vendor-authored narrative interpreted through provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Listing', 'LeaseEvent']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class ListingEvent(ConfiguredBaseModel):
    """
    A dated listing assertion. asking_price is a sale price or periodic rent; rent_period states the period when asking_price is rent. close_price is asserted on a closed event.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    occurred_on: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    event_kind: str = Field(default=..., description="""listed | price_change | status_change | relisted | closed""", json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LoanEvent']} })
    status: Optional[ListingStatus] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    asking_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent']} })
    rent_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LeaseEvent', 'RentRoll']} })
    close_price: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class LeaseEvent(Entity):
    """
    Executed leases (not asking rents — see UnitRentObservation).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    property_state: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'Valuation']} })
    space: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'RentRollLine']} })
    lease_type: Optional[LeaseTypeEnum] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    execution_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    commencement_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    expiration_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    term_months: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'Loan']} })
    leased_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    rent_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LeaseEvent', 'RentRoll']} })
    starting_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    effective_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    net_effective_rent_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    free_rent_months: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    ti_allowance_per_area: Optional[UnitRate] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    expense_structure: Optional[ExpenseStructure] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    parties: Optional[list[LeaseEventParty]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    escalations: Optional[list[LeaseEscalation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent']} })
    concessions: Optional[list[LeaseConcession]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'LeaseEvent']} })
    remarks: Optional[str] = Field(default=None, description="""Source- or vendor-authored narrative interpreted through provenance.""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Listing', 'LeaseEvent']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class LeaseEventParty(TransactionParty):
    """
    role = lessee | landlord | guarantor | lessee_broker | landlord_broker
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class LeaseEscalation(ConfiguredBaseModel):
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    escalation_type: str = Field(default=..., description="""fixed_amount | fixed_percent | cpi | step_schedule | fmv | none""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    escalation_value: Optional[Decimal] = Field(default=None, description="""For escalation_type=fixed_percent, a value from 0-100 percentage points; 3 means 3 percent. For escalation_type=fixed_amount, the increment in the currency of the parent LeaseEvent.rent and the period specified by LeaseEvent.rent_period.""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    frequency_months: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_index: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_floor: Optional[Decimal] = Field(default=None, description="""CPI escalation floor in percentage points; 2 means 2 percent.""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    cpi_cap: Optional[Decimal] = Field(default=None, description="""CPI escalation cap in percentage points; 5 means 5 percent.""", json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    steps: Optional[list[RentStep]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    effective_from: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    effective_until: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEscalation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    abatement_percent: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 10 means 10 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    ti_cap_total: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    conditions: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseConcession']} })
    notes: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ExpenseStructure', 'LeaseConcession']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    unit_type: str = Field(default=..., description="""'1BR/1BA', '10x10'""", json_schema_extra = { "linkml_meta": {'domain_of': ['Address', 'UnitRentObservation']} })
    unit_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    bedrooms: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    bathrooms: Optional[Decimal] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['SpaceFacts', 'UnitRentObservation']} })
    unit_count: Optional[int] = Field(default=None, description="""Units of this type""", json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'UnitRentObservation', 'RentRoll']} })
    units_available: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    rate: Money = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TaxLineItem', 'UnitRentObservation']} })
    rate_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    rate_basis: Optional[RateBasis] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    rate_type: Optional[RateType] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    observed_on: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    concessions_note: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['UnitRentObservation']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class Loan(RecordedInstrument, Entity):
    """
    Recorded debt. Originating lender identity and classification live on the canonical Party referenced by a LoanParty. Assignments, modifications, satisfactions, and other lifecycle assertions are dated events; consumers derive current status and satisfaction dates.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    parcel: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel', 'Assessment', 'TaxBill', 'Transfer', 'Loan']} })
    transfer: Optional[str] = Field(default=None, description="""Purchase-money linkage""", json_schema_extra = { "linkml_meta": {'domain_of': ['SaleEvent', 'Loan']} })
    is_purchase_money: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    loan_amount: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    loan_type: Optional[str] = Field(default=None, description="""conventional | fha | va | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    purpose: Optional[str] = Field(default=None, description="""purchase | refinance | construction | heloc | ...""", json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_heloc: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_construction: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_seller_carryback: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_assumable: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    interest_rate: Optional[Decimal] = Field(default=None, description="""Nominal interest rate in percentage points; 6.5 means 6.5 percent.""", ge=0, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    is_variable_rate: Optional[bool] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    term_months: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'Loan']} })
    due_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
    lien_position: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['Loan']} })
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class LoanParty(TransactionParty):
    """
    role = borrower | lender | beneficiary | trustee (assignees live in loan events)
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


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
    to_party: Optional[str] = Field(default=None, description="""Canonical assignee for assignment-like events""", json_schema_extra = { "linkml_meta": {'domain_of': ['LoanEvent']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })

    @field_validator('to_party')
    def pattern_to_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid to_party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid to_party format: {v}"
            raise ValueError(err_msg)
        return v


class Lien(RecordedInstrument, Entity):
    """
    Involuntary encumbrances — tax, judgment, HOA, mechanic's.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    kind: LienKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class LienParty(TransactionParty):
    """
    role = claimant | debtor
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class ForeclosureCase(Entity):
    """
    A foreclosure PROCEEDING; its recorded filings append over time.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
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
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class ForeclosureFiling(RecordedInstrument):
    """
    One recorded filing; postponements append as new filings.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities',
         'mixins': ['RecordedInstrument']})

    status: str = Field(default=..., description="""nod | lis_pendens | notice_of_sale | auction_scheduled | postponement | ... (open; US-seeded)""", json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    auction_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureFiling']} })
    auction_at_time: Optional[str] = Field(default=None, description="""Time-of-day as published""", json_schema_extra = { "linkml_meta": {'domain_of': ['ForeclosureFiling']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })


class ForeclosureCaseParty(TransactionParty):
    """
    Open contextual role. Mortgage and deed-of-trust proceedings commonly use lender | trustee | borrower; lien foreclosures may instead use claimant | debtor.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    role: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'AddressAssociation', 'OwnershipInterest']} })
    party: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty', 'OwnershipInterest', 'PropertyAssociation']} })
    sequence: Optional[int] = Field(default=None, ge=1, json_schema_extra = { "linkml_meta": {'domain_of': ['TransactionParty']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('party')
    def pattern_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid party format: {v}"
            raise ValueError(err_msg)
        return v


class Permit(Entity):
    """
    Building permits — header + lifecycle dates. Inspections/projects are extension.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    structure: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Space', 'Permit']} })
    permitting_jurisdiction: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    permit_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    kind: Optional[str] = Field(default=None, description="""roofing | solar | hvac | adu | pool | new_construction | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
                       'AreaMeasure',
                       'Renovation',
                       'TaxExemption',
                       'Listing',
                       'Lien',
                       'Permit',
                       'Valuation']} })
    status: Optional[str] = Field(default=None, description="""issued | finaled | expired | ... (open)""", json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    description: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Rating', 'Renovation', 'Permit']} })
    applied_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    issued_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    finaled_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    expires_on: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    job_value: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    fees: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    contractor_party: Optional[str] = Field(default=None, description="""Canonical contractor Party reference; credential records are outside core v0.2""", json_schema_extra = { "linkml_meta": {'domain_of': ['Permit']} })
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('contractor_party')
    def pattern_contractor_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid contractor_party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid contractor_party format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class OperatingStatement(Entity):
    """
    Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    statement_year: int = Field(default=..., description="""The calendar year the statement is for (the year containing period_end for fiscal/trailing periods).""", ge=1000, le=2200, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    period_start: Optional[date] = Field(default=None, description="""For fiscal-year, trailing-12, or partial periods. period_start and period_end must be provided together; omit both for calendar-year statements.""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    period_end: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    statement_basis: Optional[StatementBasis] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    pgi: Optional[Money] = Field(default=None, description="""Potential gross income""", json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    vacancy_loss: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
    vacancy_pct: Optional[Decimal] = Field(default=None, description="""0–100 percentage points; 5 means 5 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['OperatingStatement']} })
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
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class RentRoll(Entity):
    """
    Dated rent and occupancy observation applicable to any property use. Header totals are authoritative reported values; lines are supporting detail and are not required to sum to the totals. All Money values on one rent roll MUST use one currency (validator-enforced). For its as_of_date, this record governs reported occupancy and rent facts; current-state fields such as Space.occupancy and CommercialDetails.occupancy_pct do not override it. A line preserves dated rent-roll assertions and does not by itself create a canonical Space, Party, or LeaseEvent.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    as_of_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot', 'RentRoll', 'Valuation']} })
    unit_count: Optional[int] = Field(default=None, ge=0, json_schema_extra = { "linkml_meta": {'domain_of': ['StructureFacts', 'UnitRentObservation', 'RentRoll']} })
    occupied_unit_count: Optional[int] = Field(default=None, ge=0, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRoll']} })
    occupancy_pct: Optional[Decimal] = Field(default=None, description="""Occupancy in 0–100 percentage points; 95 means 95 percent.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['CommercialDetails', 'RentRoll']} })
    total_contract_rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRoll']} })
    total_market_rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRoll']} })
    rent_period: Optional[RentPeriod] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent', 'LeaseEvent', 'RentRoll']} })
    lines: Optional[list[RentRollLine]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRoll']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class RentRollLine(ConfiguredBaseModel):
    """
    Supporting rent-roll detail. space resolves to canonical Space. tenant is the canonical legal lessee Party when present, and lease resolves to canonical LeaseEvent when known. When both tenant and lease are present, tenant must match a party with role: lessee declared by that lease, if the lease declares any lessee. References remain optional for aggregate, vacant, unleased, or unresolved lines; the line does not duplicate canonical space identity, tenant names, or lease dates. When a source tenant cannot be resolved to a canonical Party, omit tenant and preserve the source evidence through the RentRoll provenance and profile-level SourceArtifact records; do not mint a placeholder Party or copy the source name into extras.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    space: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['LeaseEvent', 'RentRollLine']} })
    tenant: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    lease: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    occupancy_status: Optional[CodeableConcept] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    reported_area: Optional[Area] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    contract_rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    market_rent: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RentRollLine']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })

    @field_validator('tenant')
    def pattern_tenant(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid tenant format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid tenant format: {v}"
            raise ValueError(err_msg)
        return v


class Valuation(Entity):
    """
    Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/entities'})

    property: str = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    property_state: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot',
                       'SaleEvent',
                       'Listing',
                       'LeaseEvent',
                       'Valuation']} })
    kind: ValuationKind = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['Jurisdiction',
                       'ParcelLineage',
                       'Party',
                       'SourceArtifact',
                       'PartyContact',
                       'StructureFacts',
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
    confidence_score: Optional[int] = Field(default=None, description="""Source-defined confidence score; not governed by the _pct percentage-points convention.""", ge=0, le=100, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    forecast_standard_deviation: Optional[Decimal] = Field(default=None, description="""Source-defined AVM forecast standard deviation; not governed by the _pct percentage-points convention.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    exposure_days: Optional[int] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_sales_comparison: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_cost: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    indicated_value_income: Optional[Money] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    value_premise: Optional[str] = Field(default=None, description="""as_is | as_completed | as_stabilized""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    interest: Optional[str] = Field(default=None, description="""Interest valued for this opinion; independent of Property.estate_type and Transfer.interest_conveyed.""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    performed_by_party: Optional[str] = Field(default=None, description="""Canonical person or organization that performed the valuation""", json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    as_of_date: date = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyStateSnapshot', 'RentRoll', 'Valuation']} })
    report_date: Optional[date] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Valuation']} })
    artifacts: Optional[list[str]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    id: str = Field(default=..., description="""Canonical identifier; nonblank with no leading or trailing whitespace""", json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
    verifications: Optional[list[VerificationAttribution]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity']} })

    @field_validator('performed_by_party')
    def pattern_performed_by_party(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid performed_by_party format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid performed_by_party format: {v}"
            raise ValueError(err_msg)
        return v

    @field_validator('id')
    def pattern_id(cls, v):
        pattern=re.compile(r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF](?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])")
        if isinstance(v, list):
            for element in v:
                if isinstance(element, str) and not pattern.match(element):
                    err_msg = f"Invalid id format: {element}"
                    raise ValueError(err_msg)
        elif isinstance(v, str) and not pattern.match(v):
            err_msg = f"Invalid id format: {v}"
            raise ValueError(err_msg)
        return v


class PropertyProfile(ConfiguredBaseModel):
    """
    The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds/profiles', 'tree_root': True})

    parties: Optional[list[Party]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Transfer',
                       'SaleEvent',
                       'LeaseEvent',
                       'Loan',
                       'Lien',
                       'ForeclosureCase',
                       'PropertyProfile']} })
    artifacts: Optional[list[SourceArtifact]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['RecordedInstrument',
                       'Listing',
                       'Permit',
                       'Valuation',
                       'PropertyProfile']} })
    addresses: Optional[list[Address]] = Field(default=None, description="""Address bundle referenced by property/parties/ownership""", json_schema_extra = { "linkml_meta": {'domain_of': ['Party', 'PropertyProfile']} })
    property: Property = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyParcel',
                       'PropertyIdentifier',
                       'PropertyAddress',
                       'OwnershipPeriod',
                       'Structure',
                       'Site',
                       'Space',
                       'PropertyStateSnapshot',
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
                       'RentRoll',
                       'Valuation',
                       'PropertyProfile']} })
    property_addresses: Optional[list[PropertyAddress]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    identifiers: Optional[list[PropertyIdentifier]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    jurisdictions: Optional[list[Jurisdiction]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    parcels: Optional[list[Parcel]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    property_parcels: Optional[list[PropertyParcel]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    parcel_lineage: Optional[list[ParcelLineage]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    site: Optional[Site] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    structures: Optional[list[Structure]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    spaces: Optional[list[Space]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    property_state_snapshots: Optional[list[PropertyStateSnapshot]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    associations: Optional[list[PropertyAssociation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    assessments: Optional[list[Assessment]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    tax_bills: Optional[list[TaxBill]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    transfers: Optional[list[Transfer]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    sales: Optional[list[SaleEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    listings: Optional[list[Listing]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    leases: Optional[list[LeaseEvent]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    unit_rents: Optional[list[UnitRentObservation]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
    rent_rolls: Optional[list[RentRoll]] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['PropertyProfile']} })
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
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class AssessorObservation(ConfiguredBaseModel):
    """
    Thin capture envelope for a county assessor / public-records fetch. The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the fetch outcome. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds'})

    status: AssessorStatus = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    query_address: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    query_parcel_number: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    assessor_url: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation']} })
    profile: Optional[PropertyProfile] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
    error: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
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
    artifact_refs: Optional[list[str]] = Field(default=None, description="""References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
                       'PropertyProfile',
                       'AssessorObservation',
                       'ExtractionObservation']} })


class ExtractionObservation(ConfiguredBaseModel):
    """
    Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the extraction outcome. By convention profile and category accompany success and error accompanies failure statuses — validators deliberately do not enforce this pairing (LinkML rules would materialize in only some generated validators, diverging the contracts), so consumers must branch on status, not on field presence.
    """
    linkml_meta: ClassVar[LinkMLMeta] = LinkMLMeta({'from_schema': 'https://example.org/phds'})

    status: ExtractionStatus = Field(default=..., json_schema_extra = { "linkml_meta": {'domain_of': ['ListingEvent',
                       'ForeclosureFiling',
                       'Permit',
                       'AssessorObservation',
                       'ExtractionObservation']} })
    category: Optional[ExtractionCategory] = Field(default=None, description="""Content classification; expected when status is success""", json_schema_extra = { "linkml_meta": {'domain_of': ['StatementLineItem', 'ExtractionObservation']} })
    source_category: Optional[str] = Field(default=None, description="""Raw or more specific producer classification, especially when category is `other`""", json_schema_extra = { "linkml_meta": {'domain_of': ['ExtractionObservation']} })
    error: Optional[str] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
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
    artifact_refs: Optional[list[str]] = Field(default=None, description="""References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent.""", json_schema_extra = { "linkml_meta": {'domain_of': ['AssessorObservation', 'ExtractionObservation']} })
    extras: Optional[Any] = Field(default=None, json_schema_extra = { "linkml_meta": {'domain_of': ['Entity',
                       'InstrumentReference',
                       'TransactionParty',
                       'VerificationAttribution',
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
                       'RentRollLine',
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
Classification.model_rebuild()
Rating.model_rebuild()
GeoPoint.model_rebuild()
Geometry.model_rebuild()
Provenance.model_rebuild()
Entity.model_rebuild()
InstrumentReference.model_rebuild()
RecordedInstrument.model_rebuild()
TransactionParty.model_rebuild()
Jurisdiction.model_rebuild()
Address.model_rebuild()
PropertyFacts.model_rebuild()
Property.model_rebuild()
Parcel.model_rebuild()
PropertyParcel.model_rebuild()
ParcelLineage.model_rebuild()
PropertyIdentifier.model_rebuild()
Party.model_rebuild()
VerificationAttribution.model_rebuild()
SourceArtifact.model_rebuild()
AddressAssociation.model_rebuild()
PartyAddress.model_rebuild()
PropertyAddress.model_rebuild()
PartyContact.model_rebuild()
OwnershipPeriod.model_rebuild()
OwnershipInterest.model_rebuild()
StructureFacts.model_rebuild()
Structure.model_rebuild()
AreaMeasure.model_rebuild()
ResidentialDetails.model_rebuild()
CommercialDetails.model_rebuild()
Renovation.model_rebuild()
SiteFacts.model_rebuild()
Site.model_rebuild()
SpaceFacts.model_rebuild()
Space.model_rebuild()
PropertyState.model_rebuild()
SiteState.model_rebuild()
StructureState.model_rebuild()
SpaceState.model_rebuild()
PropertyStateSnapshot.model_rebuild()
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
RentRoll.model_rebuild()
RentRollLine.model_rebuild()
Valuation.model_rebuild()
PropertyProfile.model_rebuild()
AssessorObservation.model_rebuild()
ExtractionObservation.model_rebuild()
