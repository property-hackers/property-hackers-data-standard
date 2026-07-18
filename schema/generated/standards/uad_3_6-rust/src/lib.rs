#![allow(non_camel_case_types)]

#[cfg(feature = "serde")]
mod serde_utils;
#[cfg(feature = "stubgen")]
pub mod stub_utils;

#[cfg(feature = "serde")]
use serde_yml as _ ;
use chrono::{DateTime,FixedOffset,NaiveDate};
#[cfg(feature = "pyo3")]
use pyo3::{FromPyObject,prelude::*};
#[cfg(feature = "stubgen")]
use pyo3_stub_gen::{define_stub_info_gatherer,derive::gen_stub_pyclass,derive::gen_stub_pymethods};
#[cfg(feature = "serde")]
use serde::{Deserialize,Serialize,de::IntoDeserializer};
use serde_value::Value;
#[cfg(feature = "serde")]
use serde_path_to_error;
use std::collections::HashMap;
use std::collections::BTreeMap;

// Types

pub type string = String;
pub type integer = String;
pub type boolean = String;
pub type float = f64;
pub type double = f64;
pub type decimal = String;
pub type time = String;
pub type date = String;
pub type datetime = String;
pub type date_or_datetime = String;
pub type uriorcurie = String;
pub type curie = String;
pub type uri = String;
pub type ncname = String;
pub type objectidentifier = String;
pub type nodeidentifier = String;
pub type jsonpointer = String;
pub type jsonpath = String;
pub type sparqlpath = String;
pub type DecimalString = String;

// Slots

pub type amount = String;
pub type currency = String;
pub type system = String;
pub type code = String;
pub type description = String;
pub type scope = String;
pub type display = String;
pub type property = String;
pub type kind = String;
pub type name = String;
pub type structure_number = String;
pub type living_area = String;
pub type gross_area = String;
pub type rentable_area = String;
pub type ground_floor_area = String;
pub type basement_area = String;
pub type basement_finished_area = String;
pub type garage_area = String;
pub type areas = String;
pub type year_built = String;
pub type year_built_estimated = String;
pub type effective_year_built = String;
pub type stories = String;
pub type unit_count = String;
pub type construction_method = String;
pub type construction_status = String;
pub type construction_type = String;
pub type exterior_wall_type = String;
pub type roof_material_type = String;
pub type roof_style_type = String;
pub type foundation_type = String;
pub type foundation_material = String;
pub type condition_ratings = String;
pub type quality_ratings = String;
pub type heating_types = String;
pub type heating_fuel_type = String;
pub type cooling_types = String;
pub type sewer_type = String;
pub type water_type = String;
pub type features = String;
pub type residential = String;
pub type commercial = String;
pub type renovations = String;
pub type id = String;
pub type extras = String;
pub type provenance = String;
pub type verifications = String;
pub type subject = String;
pub type as_of_date = String;
pub type basis = String;
pub type property_state = String;
pub type site_states = String;
pub type structure_states = String;
pub type space_states = String;
pub type parties = String;
pub type artifacts = String;
pub type addresses = String;
pub type property_addresses = String;
pub type identifiers = String;
pub type jurisdictions = String;
pub type parcels = String;
pub type property_parcels = String;
pub type parcel_lineage = String;
pub type site = String;
pub type structures = String;
pub type spaces = String;
pub type property_state_snapshots = String;
pub type associations = String;
pub type assessments = String;
pub type tax_bills = String;
pub type transfers = String;
pub type sales = String;
pub type listings = String;
pub type leases = String;
pub type unit_rents = String;
pub type rent_rolls = String;
pub type loans = String;
pub type liens = String;
pub type foreclosure_cases = String;
pub type permits = String;
pub type ownership = String;
pub type operating_statements = String;
pub type valuations = String;
pub type value = String;
pub type unit = String;
pub type denominator = String;
pub type latitude = f64;
pub type longitude = f64;
pub type type_ = String;
pub type coordinates = Any;
pub type provider = String;
pub type source_url = String;
pub type retrieved_at = DateTime<FixedOffset>;
pub type method = CaptureMethod;
pub type confidence = f64;
pub type verification = VerificationStatus;
pub type relationship_type = CodeableConcept;
pub type document_number = String;
pub type registry_reference = String;
pub type recording_authority = String;
pub type recording_book = String;
pub type recording_page = String;
pub type recorded_on = String;
pub type instrument_date = String;
pub type document_type = String;
pub type related_instruments = String;
pub type role = String;
pub type party = String;
pub type sequence = String;
pub type country = String;
pub type region = String;
pub type authority_code = String;
pub type parent = Jurisdiction;
pub type boundary = String;
pub type unformatted_address = String;
pub type street_number = String;
pub type street_pre_direction = String;
pub type street_name = String;
pub type street_suffix = String;
pub type street_post_direction = String;
pub type unit_type = String;
pub type unit_number = String;
pub type sublocality = String;
pub type city = String;
pub type postal_code = String;
pub type postal_code_suffix = String;
pub type admin_area = String;
pub type admin_area_code = String;
pub type address_hash = String;
pub type address_hash_scheme = String;
pub type location = String;
pub type location_accuracy = GeocodeAccuracy;
pub type property_use_class = String;
pub type property_use_type = String;
pub type property_use_subtype = String;
pub type property_use_system = String;
pub type estate_type = String;
pub type building_count = String;
pub type jurisdiction = String;
pub type parcel_number = String;
pub type normalized_parcel_number = String;
pub type unit_designator = String;
pub type reso_upi = String;
pub type legal_description = String;
pub type land_area = Area;
pub type retired_on = NaiveDate;
pub type parcel = String;
pub type is_primary = String;
pub type started_on = String;
pub type ended_on = String;
pub type predecessor_parcel = Parcel;
pub type successor_parcel = Parcel;
pub type effective_on = String;
pub type scheme = String;
pub type namespace = String;
pub type legal_form = Classification;
pub type normalized_name = String;
pub type name_first = String;
pub type name_middle = String;
pub type name_last = String;
pub type classifications = Vec<Classification>;
pub type contacts = Vec<PartyContact>;
pub type verifier = Party;
pub type verified_at = DateTime<FixedOffset>;
pub type note = String;
pub type storage_reference = String;
pub type media_type = String;
pub type title = String;
pub type original_filename = String;
pub type content_hash = String;
pub type hash_scheme = String;
pub type page_count = isize;
pub type captured_on = DateTime<FixedOffset>;
pub type address = String;
pub type valid_from = String;
pub type valid_to = String;
pub type label = String;
pub type do_not_contact = bool;
pub type vesting_type = String;
pub type mailing_address = Address;
pub type acquired_via_transfer = Transfer;
pub type disposed_via_transfer = Transfer;
pub type interests = Vec<OwnershipInterest>;
pub type interest_pct = f64;
pub type is_owner_occupied = bool;
pub type area = Area;
pub type bedrooms_total = isize;
pub type bathrooms_full = isize;
pub type bathrooms_half = isize;
pub type bathrooms_three_quarter = isize;
pub type rooms_total = isize;
pub type attachment = String;
pub type architectural_design = String;
pub type basement_type = String;
pub type garage_type = String;
pub type garage_attachment = String;
pub type parking_spaces = String;
pub type fireplaces = isize;
pub type has_pool = bool;
pub type has_attic = bool;
pub type has_adu = bool;
pub type adu_legally_rentable = bool;
pub type occupancy = String;
pub type renewable_energy_components = Vec<String>;
pub type market_classification = Rating;
pub type clear_height = Length;
pub type dock_doors = isize;
pub type drive_in_doors = isize;
pub type occupancy_pct = String;
pub type parking_ratio = UnitRate;
pub type tenancy = String;
pub type tenant_count = isize;
pub type parking_types = Vec<String>;
pub type has_sprinkler = bool;
pub type elevators = isize;
pub type submarket = String;
pub type completed_year = isize;
pub type completed_on = NaiveDate;
pub type cost = Money;
pub type lot_size = String;
pub type usable_land_area = String;
pub type usable_land_area_basis = String;
pub type land_use = String;
pub type land_use_category = String;
pub type zoning_code = String;
pub type flood_zone = String;
pub type hazard_zones = String;
pub type view_types = String;
pub type site_influences = String;
pub type easements = String;
pub type restrictions = String;
pub type utilities = String;
pub type frontage = String;
pub type depth = String;
pub type topography = String;
pub type is_corner = String;
pub type entitlement_status = String;
pub type buildable_units = String;
pub type subdivision = String;
pub type lot_number = String;
pub type block = String;
pub type tract_number = String;
pub type phase_number = String;
pub type section_township_range = String;
pub type floor_number = String;
pub type space_use = String;
pub type usable_area = String;
pub type bedrooms = String;
pub type bathrooms = String;
pub type is_adu = String;
pub type is_active = String;
pub type structure = String;
pub type space_identifier = String;
pub type fee = Money;
pub type fee_period = RentPeriod;
pub type tax_year = String;
pub type roll_type = String;
pub type assessed_land_value = Money;
pub type assessed_improvement_value = Money;
pub type assessed_total_value = Money;
pub type market_land_value = Money;
pub type market_improvement_value = Money;
pub type market_total_value = Money;
pub type appraised_land_value = Money;
pub type appraised_improvement_value = Money;
pub type appraised_total_value = Money;
pub type exemptions = Vec<TaxExemption>;
pub type bill_number = String;
pub type amount_billed = Money;
pub type amount_paid = String;
pub type is_delinquent = String;
pub type delinquent_year = isize;
pub type delinquent_amount = Money;
pub type rate_code_area = String;
pub type installments = Vec<TaxInstallment>;
pub type line_items = String;
pub type installment_number = isize;
pub type due_on = NaiveDate;
pub type paid_on = NaiveDate;
pub type rate = String;
pub type transfer_kind = String;
pub type consideration = Money;
pub type transfer_tax = Money;
pub type price_disclosure = String;
pub type price_code = String;
pub type interest_conveyed = CodeableConcept;
pub type partial_interest_pct = f64;
pub type is_inter_family = bool;
pub type is_distressed = bool;
pub type transfer = String;
pub type sale_date = NaiveDate;
pub type sale_price = Money;
pub type sale_type = SaleTypeEnum;
pub type price_per_area = UnitRate;
pub type price_per_unit = Money;
pub type financing = String;
pub type concessions = String;
pub type cap_rate = f64;
pub type noi_at_sale = Money;
pub type opex_at_sale = Money;
pub type occupancy_at_sale_pct = f64;
pub type unit_count_at_sale = isize;
pub type supporting_operating_statement = OperatingStatement;
pub type remarks = String;
pub type listing_type = String;
pub type mls_number = String;
pub type events = String;
pub type participants = Vec<ListingParticipant>;
pub type occurred_on = String;
pub type event_kind = String;
pub type status = String;
pub type asking_price = Money;
pub type rent_period = String;
pub type close_price = Money;
pub type space = String;
pub type lease_type = LeaseTypeEnum;
pub type execution_date = NaiveDate;
pub type commencement_date = NaiveDate;
pub type expiration_date = NaiveDate;
pub type term_months = String;
pub type leased_area = Area;
pub type rent = Money;
pub type starting_rent_per_area = UnitRate;
pub type effective_rent_per_area = UnitRate;
pub type net_effective_rent_per_area = UnitRate;
pub type free_rent_months = f64;
pub type ti_allowance_per_area = UnitRate;
pub type expense_structure = ExpenseStructure;
pub type escalations = Vec<LeaseEscalation>;
pub type taxes_paid_by = String;
pub type insurance_paid_by = String;
pub type cam_paid_by = String;
pub type utilities_paid_by = String;
pub type notes = String;
pub type escalation_type = String;
pub type escalation_value = f64;
pub type frequency_months = isize;
pub type cpi_index = String;
pub type cpi_floor = f64;
pub type cpi_cap = f64;
pub type steps = Vec<RentStep>;
pub type effective_from = NaiveDate;
pub type effective_until = NaiveDate;
pub type from_date = NaiveDate;
pub type concession_type = String;
pub type concession_value = Money;
pub type abatement_months = f64;
pub type abatement_percent = f64;
pub type ti_cap_total = Money;
pub type conditions = Any;
pub type unit_area = Area;
pub type units_available = isize;
pub type rate_period = RentPeriod;
pub type rate_basis = RateBasis;
pub type rate_type = RateType;
pub type observed_on = NaiveDate;
pub type concessions_note = String;
pub type is_purchase_money = bool;
pub type loan_amount = Money;
pub type loan_type = String;
pub type purpose = String;
pub type is_heloc = bool;
pub type is_construction = bool;
pub type is_seller_carryback = bool;
pub type is_assumable = bool;
pub type interest_rate = f64;
pub type is_variable_rate = bool;
pub type due_date = NaiveDate;
pub type lien_position = isize;
pub type to_party = Party;
pub type released_on = NaiveDate;
pub type loan = Loan;
pub type case_number = String;
pub type opened_on = NaiveDate;
pub type resolved_on = NaiveDate;
pub type resolution = String;
pub type past_due_amount = Money;
pub type unpaid_balance = Money;
pub type original_loan_amount = Money;
pub type auction_min_bid = Money;
pub type auction_location = String;
pub type filings = Vec<ForeclosureFiling>;
pub type auction_on = NaiveDate;
pub type auction_at_time = String;
pub type permitting_jurisdiction = Jurisdiction;
pub type permit_number = String;
pub type applied_on = NaiveDate;
pub type issued_on = NaiveDate;
pub type finaled_on = NaiveDate;
pub type expires_on = NaiveDate;
pub type job_value = Money;
pub type fees = Money;
pub type contractor_party = Party;
pub type statement_year = isize;
pub type period_start = NaiveDate;
pub type period_end = NaiveDate;
pub type statement_basis = StatementBasis;
pub type pgi = Money;
pub type vacancy_loss = Money;
pub type vacancy_pct = f64;
pub type egi = Money;
pub type opex_total = Money;
pub type noi = Money;
pub type capex = Money;
pub type reimbursements = Money;
pub type reserves = Money;
pub type reserves_included_in_opex = bool;
pub type ground_lease_expense = Money;
pub type ground_lease_included_in_opex = bool;
pub type category = String;
pub type occupied_unit_count = isize;
pub type total_contract_rent = Money;
pub type total_market_rent = Money;
pub type lines = Vec<RentRollLine>;
pub type tenant = Party;
pub type lease = LeaseEvent;
pub type occupancy_status = CodeableConcept;
pub type reported_area = Area;
pub type contract_rent = Money;
pub type market_rent = Money;
pub type valuation_method = String;
pub type value_type = String;
pub type value_low = Money;
pub type value_high = Money;
pub type value_per_area = UnitRate;
pub type land_value = Money;
pub type confidence_score = isize;
pub type forecast_standard_deviation = f64;
pub type exposure_days = isize;
pub type indicated_value_sales_comparison = Money;
pub type indicated_value_cost = Money;
pub type indicated_value_income = Money;
pub type value_premise = String;
pub type interest = String;
pub type performed_by_party = Party;
pub type report_date = NaiveDate;
pub type query_address = String;
pub type query_parcel_number = String;
pub type assessor_url = uri;
pub type profile = String;
pub type error = String;
pub type artifact_refs = String;
pub type source_category = String;
pub type extracted_at = DateTime<FixedOffset>;
pub type model = String;

// Enums

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Uad36ConditionSystem {
#[cfg_attr(feature = "serde", serde(rename = "urn:phds:standard:uad:3.6:condition"))]
    UrnPhdsStandardUad36Condition,
}

impl core::fmt::Display for Uad36ConditionSystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Uad36ConditionSystem::UrnPhdsStandardUad36Condition => f.write_str("urn:phds:standard:uad:3.6:condition"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Uad36ConditionSystem {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            Uad36ConditionSystem::UrnPhdsStandardUad36Condition => "urn:phds:standard:uad:3.6:condition",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Uad36ConditionSystem {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "urn:phds:standard:uad:3.6:condition" | "UrnPhdsStandardUad36Condition" => Ok(Uad36ConditionSystem::UrnPhdsStandardUad36Condition),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for Uad36ConditionSystem: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(Uad36ConditionSystem)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Uad36ConditionSystem {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['urn:phds:standard:uad:3.6:condition']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Uad36QualitySystem {
#[cfg_attr(feature = "serde", serde(rename = "urn:phds:standard:uad:3.6:quality"))]
    UrnPhdsStandardUad36Quality,
}

impl core::fmt::Display for Uad36QualitySystem {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Uad36QualitySystem::UrnPhdsStandardUad36Quality => f.write_str("urn:phds:standard:uad:3.6:quality"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Uad36QualitySystem {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            Uad36QualitySystem::UrnPhdsStandardUad36Quality => "urn:phds:standard:uad:3.6:quality",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Uad36QualitySystem {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "urn:phds:standard:uad:3.6:quality" | "UrnPhdsStandardUad36Quality" => Ok(Uad36QualitySystem::UrnPhdsStandardUad36Quality),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for Uad36QualitySystem: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(Uad36QualitySystem)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Uad36QualitySystem {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['urn:phds:standard:uad:3.6:quality']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Uad36ConditionCode {
#[cfg_attr(feature = "serde", serde(rename = "C1"))]
    C1,
#[cfg_attr(feature = "serde", serde(rename = "C2"))]
    C2,
#[cfg_attr(feature = "serde", serde(rename = "C3"))]
    C3,
#[cfg_attr(feature = "serde", serde(rename = "C4"))]
    C4,
#[cfg_attr(feature = "serde", serde(rename = "C5"))]
    C5,
#[cfg_attr(feature = "serde", serde(rename = "C6"))]
    C6,
}

impl core::fmt::Display for Uad36ConditionCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Uad36ConditionCode::C1 => f.write_str("C1"),
            Uad36ConditionCode::C2 => f.write_str("C2"),
            Uad36ConditionCode::C3 => f.write_str("C3"),
            Uad36ConditionCode::C4 => f.write_str("C4"),
            Uad36ConditionCode::C5 => f.write_str("C5"),
            Uad36ConditionCode::C6 => f.write_str("C6"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Uad36ConditionCode {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            Uad36ConditionCode::C1 => "C1",
            Uad36ConditionCode::C2 => "C2",
            Uad36ConditionCode::C3 => "C3",
            Uad36ConditionCode::C4 => "C4",
            Uad36ConditionCode::C5 => "C5",
            Uad36ConditionCode::C6 => "C6",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Uad36ConditionCode {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "C1" => Ok(Uad36ConditionCode::C1),
                "C2" => Ok(Uad36ConditionCode::C2),
                "C3" => Ok(Uad36ConditionCode::C3),
                "C4" => Ok(Uad36ConditionCode::C4),
                "C5" => Ok(Uad36ConditionCode::C5),
                "C6" => Ok(Uad36ConditionCode::C6),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for Uad36ConditionCode: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(Uad36ConditionCode)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Uad36ConditionCode {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['C1', 'C2', 'C3', 'C4', 'C5', 'C6']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Uad36QualityCode {
#[cfg_attr(feature = "serde", serde(rename = "Q1"))]
    Q1,
#[cfg_attr(feature = "serde", serde(rename = "Q2"))]
    Q2,
#[cfg_attr(feature = "serde", serde(rename = "Q3"))]
    Q3,
#[cfg_attr(feature = "serde", serde(rename = "Q4"))]
    Q4,
#[cfg_attr(feature = "serde", serde(rename = "Q5"))]
    Q5,
#[cfg_attr(feature = "serde", serde(rename = "Q6"))]
    Q6,
}

impl core::fmt::Display for Uad36QualityCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Uad36QualityCode::Q1 => f.write_str("Q1"),
            Uad36QualityCode::Q2 => f.write_str("Q2"),
            Uad36QualityCode::Q3 => f.write_str("Q3"),
            Uad36QualityCode::Q4 => f.write_str("Q4"),
            Uad36QualityCode::Q5 => f.write_str("Q5"),
            Uad36QualityCode::Q6 => f.write_str("Q6"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Uad36QualityCode {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            Uad36QualityCode::Q1 => "Q1",
            Uad36QualityCode::Q2 => "Q2",
            Uad36QualityCode::Q3 => "Q3",
            Uad36QualityCode::Q4 => "Q4",
            Uad36QualityCode::Q5 => "Q5",
            Uad36QualityCode::Q6 => "Q6",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Uad36QualityCode {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "Q1" => Ok(Uad36QualityCode::Q1),
                "Q2" => Ok(Uad36QualityCode::Q2),
                "Q3" => Ok(Uad36QualityCode::Q3),
                "Q4" => Ok(Uad36QualityCode::Q4),
                "Q5" => Ok(Uad36QualityCode::Q5),
                "Q6" => Ok(Uad36QualityCode::Q6),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for Uad36QualityCode: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(Uad36QualityCode)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Uad36QualityCode {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['Q1', 'Q2', 'Q3', 'Q4', 'Q5', 'Q6']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Uad36RatingScope {
#[cfg_attr(feature = "serde", serde(rename = "overall"))]
    Overall,
#[cfg_attr(feature = "serde", serde(rename = "exterior"))]
    Exterior,
#[cfg_attr(feature = "serde", serde(rename = "interior"))]
    Interior,
}

impl core::fmt::Display for Uad36RatingScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Uad36RatingScope::Overall => f.write_str("overall"),
            Uad36RatingScope::Exterior => f.write_str("exterior"),
            Uad36RatingScope::Interior => f.write_str("interior"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Uad36RatingScope {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            Uad36RatingScope::Overall => "overall",
            Uad36RatingScope::Exterior => "exterior",
            Uad36RatingScope::Interior => "interior",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Uad36RatingScope {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "overall" | "Overall" => Ok(Uad36RatingScope::Overall),
                "exterior" | "Exterior" => Ok(Uad36RatingScope::Exterior),
                "interior" | "Interior" => Ok(Uad36RatingScope::Interior),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for Uad36RatingScope: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(Uad36RatingScope)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Uad36RatingScope {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['overall', 'exterior', 'interior']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AreaUnit {
#[cfg_attr(feature = "serde", serde(rename = "sqft"))]
    Sqft,
#[cfg_attr(feature = "serde", serde(rename = "sqm"))]
    Sqm,
#[cfg_attr(feature = "serde", serde(rename = "acre"))]
    Acre,
#[cfg_attr(feature = "serde", serde(rename = "hectare"))]
    Hectare,
}

impl core::fmt::Display for AreaUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AreaUnit::Sqft => f.write_str("sqft"),
            AreaUnit::Sqm => f.write_str("sqm"),
            AreaUnit::Acre => f.write_str("acre"),
            AreaUnit::Hectare => f.write_str("hectare"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for AreaUnit {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            AreaUnit::Sqft => "sqft",
            AreaUnit::Sqm => "sqm",
            AreaUnit::Acre => "acre",
            AreaUnit::Hectare => "hectare",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for AreaUnit {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "sqft" | "Sqft" => Ok(AreaUnit::Sqft),
                "sqm" | "Sqm" => Ok(AreaUnit::Sqm),
                "acre" | "Acre" => Ok(AreaUnit::Acre),
                "hectare" | "Hectare" => Ok(AreaUnit::Hectare),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for AreaUnit: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(AreaUnit)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for AreaUnit {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['sqft', 'sqm', 'acre', 'hectare']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LengthUnit {
#[cfg_attr(feature = "serde", serde(rename = "ft"))]
    Ft,
#[cfg_attr(feature = "serde", serde(rename = "m"))]
    M,
}

impl core::fmt::Display for LengthUnit {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LengthUnit::Ft => f.write_str("ft"),
            LengthUnit::M => f.write_str("m"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for LengthUnit {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            LengthUnit::Ft => "ft",
            LengthUnit::M => "m",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for LengthUnit {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "ft" | "Ft" => Ok(LengthUnit::Ft),
                "m" | "M" => Ok(LengthUnit::M),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for LengthUnit: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(LengthUnit)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for LengthUnit {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['ft', 'm']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CaptureMethod {
#[cfg_attr(feature = "serde", serde(rename = "api"))]
    Api,
#[cfg_attr(feature = "serde", serde(rename = "scrape"))]
    Scrape,
#[cfg_attr(feature = "serde", serde(rename = "llm_extraction"))]
    LlmExtraction,
#[cfg_attr(feature = "serde", serde(rename = "manual"))]
    Manual,
#[cfg_attr(feature = "serde", serde(rename = "bulk"))]
    Bulk,
}

impl core::fmt::Display for CaptureMethod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            CaptureMethod::Api => f.write_str("api"),
            CaptureMethod::Scrape => f.write_str("scrape"),
            CaptureMethod::LlmExtraction => f.write_str("llm_extraction"),
            CaptureMethod::Manual => f.write_str("manual"),
            CaptureMethod::Bulk => f.write_str("bulk"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for CaptureMethod {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            CaptureMethod::Api => "api",
            CaptureMethod::Scrape => "scrape",
            CaptureMethod::LlmExtraction => "llm_extraction",
            CaptureMethod::Manual => "manual",
            CaptureMethod::Bulk => "bulk",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for CaptureMethod {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "api" | "Api" => Ok(CaptureMethod::Api),
                "scrape" | "Scrape" => Ok(CaptureMethod::Scrape),
                "llm_extraction" | "LlmExtraction" => Ok(CaptureMethod::LlmExtraction),
                "manual" | "Manual" => Ok(CaptureMethod::Manual),
                "bulk" | "Bulk" => Ok(CaptureMethod::Bulk),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for CaptureMethod: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(CaptureMethod)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for CaptureMethod {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['api', 'scrape', 'llm_extraction', 'manual', 'bulk']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VerificationStatus {
#[cfg_attr(feature = "serde", serde(rename = "unverified"))]
    Unverified,
#[cfg_attr(feature = "serde", serde(rename = "pending_review"))]
    PendingReview,
#[cfg_attr(feature = "serde", serde(rename = "verified"))]
    Verified,
#[cfg_attr(feature = "serde", serde(rename = "disputed"))]
    Disputed,
#[cfg_attr(feature = "serde", serde(rename = "rejected"))]
    Rejected,
}

impl core::fmt::Display for VerificationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VerificationStatus::Unverified => f.write_str("unverified"),
            VerificationStatus::PendingReview => f.write_str("pending_review"),
            VerificationStatus::Verified => f.write_str("verified"),
            VerificationStatus::Disputed => f.write_str("disputed"),
            VerificationStatus::Rejected => f.write_str("rejected"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for VerificationStatus {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            VerificationStatus::Unverified => "unverified",
            VerificationStatus::PendingReview => "pending_review",
            VerificationStatus::Verified => "verified",
            VerificationStatus::Disputed => "disputed",
            VerificationStatus::Rejected => "rejected",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for VerificationStatus {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "unverified" | "Unverified" => Ok(VerificationStatus::Unverified),
                "pending_review" | "PendingReview" => Ok(VerificationStatus::PendingReview),
                "verified" | "Verified" => Ok(VerificationStatus::Verified),
                "disputed" | "Disputed" => Ok(VerificationStatus::Disputed),
                "rejected" | "Rejected" => Ok(VerificationStatus::Rejected),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for VerificationStatus: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(VerificationStatus)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for VerificationStatus {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['unverified', 'pending_review', 'verified', 'disputed', 'rejected']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PartyKind {
#[cfg_attr(feature = "serde", serde(rename = "person"))]
    Person,
#[cfg_attr(feature = "serde", serde(rename = "organization"))]
    Organization,
}

impl core::fmt::Display for PartyKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PartyKind::Person => f.write_str("person"),
            PartyKind::Organization => f.write_str("organization"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for PartyKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            PartyKind::Person => "person",
            PartyKind::Organization => "organization",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for PartyKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "person" | "Person" => Ok(PartyKind::Person),
                "organization" | "Organization" => Ok(PartyKind::Organization),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for PartyKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(PartyKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for PartyKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['person', 'organization']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SaleTypeEnum {
#[cfg_attr(feature = "serde", serde(rename = "arms_length"))]
    ArmsLength,
#[cfg_attr(feature = "serde", serde(rename = "reo"))]
    Reo,
#[cfg_attr(feature = "serde", serde(rename = "short_sale"))]
    ShortSale,
#[cfg_attr(feature = "serde", serde(rename = "auction"))]
    Auction,
#[cfg_attr(feature = "serde", serde(rename = "related_party"))]
    RelatedParty,
#[cfg_attr(feature = "serde", serde(rename = "portfolio"))]
    Portfolio,
#[cfg_attr(feature = "serde", serde(rename = "partial_interest"))]
    PartialInterest,
#[cfg_attr(feature = "serde", serde(rename = "land_contract"))]
    LandContract,
#[cfg_attr(feature = "serde", serde(rename = "new_construction"))]
    NewConstruction,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for SaleTypeEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SaleTypeEnum::ArmsLength => f.write_str("arms_length"),
            SaleTypeEnum::Reo => f.write_str("reo"),
            SaleTypeEnum::ShortSale => f.write_str("short_sale"),
            SaleTypeEnum::Auction => f.write_str("auction"),
            SaleTypeEnum::RelatedParty => f.write_str("related_party"),
            SaleTypeEnum::Portfolio => f.write_str("portfolio"),
            SaleTypeEnum::PartialInterest => f.write_str("partial_interest"),
            SaleTypeEnum::LandContract => f.write_str("land_contract"),
            SaleTypeEnum::NewConstruction => f.write_str("new_construction"),
            SaleTypeEnum::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for SaleTypeEnum {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            SaleTypeEnum::ArmsLength => "arms_length",
            SaleTypeEnum::Reo => "reo",
            SaleTypeEnum::ShortSale => "short_sale",
            SaleTypeEnum::Auction => "auction",
            SaleTypeEnum::RelatedParty => "related_party",
            SaleTypeEnum::Portfolio => "portfolio",
            SaleTypeEnum::PartialInterest => "partial_interest",
            SaleTypeEnum::LandContract => "land_contract",
            SaleTypeEnum::NewConstruction => "new_construction",
            SaleTypeEnum::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for SaleTypeEnum {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "arms_length" | "ArmsLength" => Ok(SaleTypeEnum::ArmsLength),
                "reo" | "Reo" => Ok(SaleTypeEnum::Reo),
                "short_sale" | "ShortSale" => Ok(SaleTypeEnum::ShortSale),
                "auction" | "Auction" => Ok(SaleTypeEnum::Auction),
                "related_party" | "RelatedParty" => Ok(SaleTypeEnum::RelatedParty),
                "portfolio" | "Portfolio" => Ok(SaleTypeEnum::Portfolio),
                "partial_interest" | "PartialInterest" => Ok(SaleTypeEnum::PartialInterest),
                "land_contract" | "LandContract" => Ok(SaleTypeEnum::LandContract),
                "new_construction" | "NewConstruction" => Ok(SaleTypeEnum::NewConstruction),
                "other" | "Other" => Ok(SaleTypeEnum::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for SaleTypeEnum: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(SaleTypeEnum)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for SaleTypeEnum {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['arms_length', 'reo', 'short_sale', 'auction', 'related_party', 'portfolio', 'partial_interest', 'land_contract', 'new_construction', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PriceDisclosure {
#[cfg_attr(feature = "serde", serde(rename = "full"))]
    Full,
#[cfg_attr(feature = "serde", serde(rename = "partial"))]
    Partial,
#[cfg_attr(feature = "serde", serde(rename = "estimated"))]
    Estimated,
#[cfg_attr(feature = "serde", serde(rename = "nominal"))]
    Nominal,
#[cfg_attr(feature = "serde", serde(rename = "non_disclosure"))]
    NonDisclosure,
#[cfg_attr(feature = "serde", serde(rename = "unknown"))]
    Unknown,
}

impl core::fmt::Display for PriceDisclosure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PriceDisclosure::Full => f.write_str("full"),
            PriceDisclosure::Partial => f.write_str("partial"),
            PriceDisclosure::Estimated => f.write_str("estimated"),
            PriceDisclosure::Nominal => f.write_str("nominal"),
            PriceDisclosure::NonDisclosure => f.write_str("non_disclosure"),
            PriceDisclosure::Unknown => f.write_str("unknown"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for PriceDisclosure {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            PriceDisclosure::Full => "full",
            PriceDisclosure::Partial => "partial",
            PriceDisclosure::Estimated => "estimated",
            PriceDisclosure::Nominal => "nominal",
            PriceDisclosure::NonDisclosure => "non_disclosure",
            PriceDisclosure::Unknown => "unknown",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for PriceDisclosure {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "full" | "Full" => Ok(PriceDisclosure::Full),
                "partial" | "Partial" => Ok(PriceDisclosure::Partial),
                "estimated" | "Estimated" => Ok(PriceDisclosure::Estimated),
                "nominal" | "Nominal" => Ok(PriceDisclosure::Nominal),
                "non_disclosure" | "NonDisclosure" => Ok(PriceDisclosure::NonDisclosure),
                "unknown" | "Unknown" => Ok(PriceDisclosure::Unknown),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for PriceDisclosure: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(PriceDisclosure)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for PriceDisclosure {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['full', 'partial', 'estimated', 'nominal', 'non_disclosure', 'unknown']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LeaseTypeEnum {
#[cfg_attr(feature = "serde", serde(rename = "gross"))]
    Gross,
#[cfg_attr(feature = "serde", serde(rename = "modified_gross"))]
    ModifiedGross,
#[cfg_attr(feature = "serde", serde(rename = "triple_net"))]
    TripleNet,
#[cfg_attr(feature = "serde", serde(rename = "double_net"))]
    DoubleNet,
#[cfg_attr(feature = "serde", serde(rename = "single_net"))]
    SingleNet,
#[cfg_attr(feature = "serde", serde(rename = "absolute_net"))]
    AbsoluteNet,
#[cfg_attr(feature = "serde", serde(rename = "percentage"))]
    Percentage,
#[cfg_attr(feature = "serde", serde(rename = "ground"))]
    Ground,
#[cfg_attr(feature = "serde", serde(rename = "residential"))]
    Residential,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for LeaseTypeEnum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LeaseTypeEnum::Gross => f.write_str("gross"),
            LeaseTypeEnum::ModifiedGross => f.write_str("modified_gross"),
            LeaseTypeEnum::TripleNet => f.write_str("triple_net"),
            LeaseTypeEnum::DoubleNet => f.write_str("double_net"),
            LeaseTypeEnum::SingleNet => f.write_str("single_net"),
            LeaseTypeEnum::AbsoluteNet => f.write_str("absolute_net"),
            LeaseTypeEnum::Percentage => f.write_str("percentage"),
            LeaseTypeEnum::Ground => f.write_str("ground"),
            LeaseTypeEnum::Residential => f.write_str("residential"),
            LeaseTypeEnum::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for LeaseTypeEnum {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            LeaseTypeEnum::Gross => "gross",
            LeaseTypeEnum::ModifiedGross => "modified_gross",
            LeaseTypeEnum::TripleNet => "triple_net",
            LeaseTypeEnum::DoubleNet => "double_net",
            LeaseTypeEnum::SingleNet => "single_net",
            LeaseTypeEnum::AbsoluteNet => "absolute_net",
            LeaseTypeEnum::Percentage => "percentage",
            LeaseTypeEnum::Ground => "ground",
            LeaseTypeEnum::Residential => "residential",
            LeaseTypeEnum::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for LeaseTypeEnum {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "gross" | "Gross" => Ok(LeaseTypeEnum::Gross),
                "modified_gross" | "ModifiedGross" => Ok(LeaseTypeEnum::ModifiedGross),
                "triple_net" | "TripleNet" => Ok(LeaseTypeEnum::TripleNet),
                "double_net" | "DoubleNet" => Ok(LeaseTypeEnum::DoubleNet),
                "single_net" | "SingleNet" => Ok(LeaseTypeEnum::SingleNet),
                "absolute_net" | "AbsoluteNet" => Ok(LeaseTypeEnum::AbsoluteNet),
                "percentage" | "Percentage" => Ok(LeaseTypeEnum::Percentage),
                "ground" | "Ground" => Ok(LeaseTypeEnum::Ground),
                "residential" | "Residential" => Ok(LeaseTypeEnum::Residential),
                "other" | "Other" => Ok(LeaseTypeEnum::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for LeaseTypeEnum: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(LeaseTypeEnum)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for LeaseTypeEnum {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['gross', 'modified_gross', 'triple_net', 'double_net', 'single_net', 'absolute_net', 'percentage', 'ground', 'residential', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RentPeriod {
#[cfg_attr(feature = "serde", serde(rename = "daily"))]
    Daily,
#[cfg_attr(feature = "serde", serde(rename = "monthly"))]
    Monthly,
#[cfg_attr(feature = "serde", serde(rename = "annual"))]
    Annual,
#[cfg_attr(feature = "serde", serde(rename = "per_area_annual"))]
    PerAreaAnnual,
#[cfg_attr(feature = "serde", serde(rename = "per_area_monthly"))]
    PerAreaMonthly,
}

impl core::fmt::Display for RentPeriod {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RentPeriod::Daily => f.write_str("daily"),
            RentPeriod::Monthly => f.write_str("monthly"),
            RentPeriod::Annual => f.write_str("annual"),
            RentPeriod::PerAreaAnnual => f.write_str("per_area_annual"),
            RentPeriod::PerAreaMonthly => f.write_str("per_area_monthly"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RentPeriod {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            RentPeriod::Daily => "daily",
            RentPeriod::Monthly => "monthly",
            RentPeriod::Annual => "annual",
            RentPeriod::PerAreaAnnual => "per_area_annual",
            RentPeriod::PerAreaMonthly => "per_area_monthly",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RentPeriod {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "daily" | "Daily" => Ok(RentPeriod::Daily),
                "monthly" | "Monthly" => Ok(RentPeriod::Monthly),
                "annual" | "Annual" => Ok(RentPeriod::Annual),
                "per_area_annual" | "PerAreaAnnual" => Ok(RentPeriod::PerAreaAnnual),
                "per_area_monthly" | "PerAreaMonthly" => Ok(RentPeriod::PerAreaMonthly),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for RentPeriod: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(RentPeriod)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for RentPeriod {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['daily', 'monthly', 'annual', 'per_area_annual', 'per_area_monthly']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RateBasis {
#[cfg_attr(feature = "serde", serde(rename = "per_unit"))]
    PerUnit,
#[cfg_attr(feature = "serde", serde(rename = "per_bed"))]
    PerBed,
#[cfg_attr(feature = "serde", serde(rename = "per_area"))]
    PerArea,
#[cfg_attr(feature = "serde", serde(rename = "per_room"))]
    PerRoom,
#[cfg_attr(feature = "serde", serde(rename = "per_key"))]
    PerKey,
#[cfg_attr(feature = "serde", serde(rename = "per_slip"))]
    PerSlip,
#[cfg_attr(feature = "serde", serde(rename = "per_stall"))]
    PerStall,
#[cfg_attr(feature = "serde", serde(rename = "per_pad"))]
    PerPad,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for RateBasis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RateBasis::PerUnit => f.write_str("per_unit"),
            RateBasis::PerBed => f.write_str("per_bed"),
            RateBasis::PerArea => f.write_str("per_area"),
            RateBasis::PerRoom => f.write_str("per_room"),
            RateBasis::PerKey => f.write_str("per_key"),
            RateBasis::PerSlip => f.write_str("per_slip"),
            RateBasis::PerStall => f.write_str("per_stall"),
            RateBasis::PerPad => f.write_str("per_pad"),
            RateBasis::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RateBasis {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            RateBasis::PerUnit => "per_unit",
            RateBasis::PerBed => "per_bed",
            RateBasis::PerArea => "per_area",
            RateBasis::PerRoom => "per_room",
            RateBasis::PerKey => "per_key",
            RateBasis::PerSlip => "per_slip",
            RateBasis::PerStall => "per_stall",
            RateBasis::PerPad => "per_pad",
            RateBasis::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RateBasis {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "per_unit" | "PerUnit" => Ok(RateBasis::PerUnit),
                "per_bed" | "PerBed" => Ok(RateBasis::PerBed),
                "per_area" | "PerArea" => Ok(RateBasis::PerArea),
                "per_room" | "PerRoom" => Ok(RateBasis::PerRoom),
                "per_key" | "PerKey" => Ok(RateBasis::PerKey),
                "per_slip" | "PerSlip" => Ok(RateBasis::PerSlip),
                "per_stall" | "PerStall" => Ok(RateBasis::PerStall),
                "per_pad" | "PerPad" => Ok(RateBasis::PerPad),
                "other" | "Other" => Ok(RateBasis::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for RateBasis: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(RateBasis)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for RateBasis {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['per_unit', 'per_bed', 'per_area', 'per_room', 'per_key', 'per_slip', 'per_stall', 'per_pad', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RateType {
#[cfg_attr(feature = "serde", serde(rename = "asking"))]
    Asking,
#[cfg_attr(feature = "serde", serde(rename = "market"))]
    Market,
#[cfg_attr(feature = "serde", serde(rename = "effective"))]
    Effective,
#[cfg_attr(feature = "serde", serde(rename = "contract"))]
    Contract,
}

impl core::fmt::Display for RateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RateType::Asking => f.write_str("asking"),
            RateType::Market => f.write_str("market"),
            RateType::Effective => f.write_str("effective"),
            RateType::Contract => f.write_str("contract"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RateType {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            RateType::Asking => "asking",
            RateType::Market => "market",
            RateType::Effective => "effective",
            RateType::Contract => "contract",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RateType {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "asking" | "Asking" => Ok(RateType::Asking),
                "market" | "Market" => Ok(RateType::Market),
                "effective" | "Effective" => Ok(RateType::Effective),
                "contract" | "Contract" => Ok(RateType::Contract),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for RateType: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(RateType)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for RateType {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['asking', 'market', 'effective', 'contract']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ListingKind {
#[cfg_attr(feature = "serde", serde(rename = "for_sale"))]
    ForSale,
#[cfg_attr(feature = "serde", serde(rename = "for_lease"))]
    ForLease,
}

impl core::fmt::Display for ListingKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ListingKind::ForSale => f.write_str("for_sale"),
            ListingKind::ForLease => f.write_str("for_lease"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ListingKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ListingKind::ForSale => "for_sale",
            ListingKind::ForLease => "for_lease",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ListingKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "for_sale" | "ForSale" => Ok(ListingKind::ForSale),
                "for_lease" | "ForLease" => Ok(ListingKind::ForLease),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ListingKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ListingKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ListingKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['for_sale', 'for_lease']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ListingStatus {
#[cfg_attr(feature = "serde", serde(rename = "active"))]
    Active,
#[cfg_attr(feature = "serde", serde(rename = "pending"))]
    Pending,
#[cfg_attr(feature = "serde", serde(rename = "sold"))]
    Sold,
#[cfg_attr(feature = "serde", serde(rename = "leased"))]
    Leased,
#[cfg_attr(feature = "serde", serde(rename = "withdrawn"))]
    Withdrawn,
#[cfg_attr(feature = "serde", serde(rename = "expired"))]
    Expired,
#[cfg_attr(feature = "serde", serde(rename = "coming_soon"))]
    ComingSoon,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for ListingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ListingStatus::Active => f.write_str("active"),
            ListingStatus::Pending => f.write_str("pending"),
            ListingStatus::Sold => f.write_str("sold"),
            ListingStatus::Leased => f.write_str("leased"),
            ListingStatus::Withdrawn => f.write_str("withdrawn"),
            ListingStatus::Expired => f.write_str("expired"),
            ListingStatus::ComingSoon => f.write_str("coming_soon"),
            ListingStatus::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ListingStatus {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ListingStatus::Active => "active",
            ListingStatus::Pending => "pending",
            ListingStatus::Sold => "sold",
            ListingStatus::Leased => "leased",
            ListingStatus::Withdrawn => "withdrawn",
            ListingStatus::Expired => "expired",
            ListingStatus::ComingSoon => "coming_soon",
            ListingStatus::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ListingStatus {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "active" | "Active" => Ok(ListingStatus::Active),
                "pending" | "Pending" => Ok(ListingStatus::Pending),
                "sold" | "Sold" => Ok(ListingStatus::Sold),
                "leased" | "Leased" => Ok(ListingStatus::Leased),
                "withdrawn" | "Withdrawn" => Ok(ListingStatus::Withdrawn),
                "expired" | "Expired" => Ok(ListingStatus::Expired),
                "coming_soon" | "ComingSoon" => Ok(ListingStatus::ComingSoon),
                "other" | "Other" => Ok(ListingStatus::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ListingStatus: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ListingStatus)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ListingStatus {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['active', 'pending', 'sold', 'leased', 'withdrawn', 'expired', 'coming_soon', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValuationKind {
#[cfg_attr(feature = "serde", serde(rename = "avm"))]
    Avm,
#[cfg_attr(feature = "serde", serde(rename = "appraisal"))]
    Appraisal,
#[cfg_attr(feature = "serde", serde(rename = "bpo"))]
    Bpo,
#[cfg_attr(feature = "serde", serde(rename = "broker_opinion"))]
    BrokerOpinion,
#[cfg_attr(feature = "serde", serde(rename = "internal"))]
    Internal,
}

impl core::fmt::Display for ValuationKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ValuationKind::Avm => f.write_str("avm"),
            ValuationKind::Appraisal => f.write_str("appraisal"),
            ValuationKind::Bpo => f.write_str("bpo"),
            ValuationKind::BrokerOpinion => f.write_str("broker_opinion"),
            ValuationKind::Internal => f.write_str("internal"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ValuationKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ValuationKind::Avm => "avm",
            ValuationKind::Appraisal => "appraisal",
            ValuationKind::Bpo => "bpo",
            ValuationKind::BrokerOpinion => "broker_opinion",
            ValuationKind::Internal => "internal",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ValuationKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "avm" | "Avm" => Ok(ValuationKind::Avm),
                "appraisal" | "Appraisal" => Ok(ValuationKind::Appraisal),
                "bpo" | "Bpo" => Ok(ValuationKind::Bpo),
                "broker_opinion" | "BrokerOpinion" => Ok(ValuationKind::BrokerOpinion),
                "internal" | "Internal" => Ok(ValuationKind::Internal),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ValuationKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ValuationKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ValuationKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['avm', 'appraisal', 'bpo', 'broker_opinion', 'internal']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LoanEventKind {
#[cfg_attr(feature = "serde", serde(rename = "origination"))]
    Origination,
#[cfg_attr(feature = "serde", serde(rename = "assignment"))]
    Assignment,
#[cfg_attr(feature = "serde", serde(rename = "modification"))]
    Modification,
#[cfg_attr(feature = "serde", serde(rename = "satisfaction"))]
    Satisfaction,
#[cfg_attr(feature = "serde", serde(rename = "release"))]
    Release,
#[cfg_attr(feature = "serde", serde(rename = "default"))]
    Default,
#[cfg_attr(feature = "serde", serde(rename = "reinstatement"))]
    Reinstatement,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for LoanEventKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoanEventKind::Origination => f.write_str("origination"),
            LoanEventKind::Assignment => f.write_str("assignment"),
            LoanEventKind::Modification => f.write_str("modification"),
            LoanEventKind::Satisfaction => f.write_str("satisfaction"),
            LoanEventKind::Release => f.write_str("release"),
            LoanEventKind::Default => f.write_str("default"),
            LoanEventKind::Reinstatement => f.write_str("reinstatement"),
            LoanEventKind::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for LoanEventKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            LoanEventKind::Origination => "origination",
            LoanEventKind::Assignment => "assignment",
            LoanEventKind::Modification => "modification",
            LoanEventKind::Satisfaction => "satisfaction",
            LoanEventKind::Release => "release",
            LoanEventKind::Default => "default",
            LoanEventKind::Reinstatement => "reinstatement",
            LoanEventKind::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for LoanEventKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "origination" | "Origination" => Ok(LoanEventKind::Origination),
                "assignment" | "Assignment" => Ok(LoanEventKind::Assignment),
                "modification" | "Modification" => Ok(LoanEventKind::Modification),
                "satisfaction" | "Satisfaction" => Ok(LoanEventKind::Satisfaction),
                "release" | "Release" => Ok(LoanEventKind::Release),
                "default" | "Default" => Ok(LoanEventKind::Default),
                "reinstatement" | "Reinstatement" => Ok(LoanEventKind::Reinstatement),
                "other" | "Other" => Ok(LoanEventKind::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for LoanEventKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(LoanEventKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for LoanEventKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['origination', 'assignment', 'modification', 'satisfaction', 'release', 'default', 'reinstatement', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LienKind {
#[cfg_attr(feature = "serde", serde(rename = "tax"))]
    Tax,
#[cfg_attr(feature = "serde", serde(rename = "judgment"))]
    Judgment,
#[cfg_attr(feature = "serde", serde(rename = "hoa"))]
    Hoa,
#[cfg_attr(feature = "serde", serde(rename = "mechanics"))]
    Mechanics,
#[cfg_attr(feature = "serde", serde(rename = "municipal"))]
    Municipal,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for LienKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LienKind::Tax => f.write_str("tax"),
            LienKind::Judgment => f.write_str("judgment"),
            LienKind::Hoa => f.write_str("hoa"),
            LienKind::Mechanics => f.write_str("mechanics"),
            LienKind::Municipal => f.write_str("municipal"),
            LienKind::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for LienKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            LienKind::Tax => "tax",
            LienKind::Judgment => "judgment",
            LienKind::Hoa => "hoa",
            LienKind::Mechanics => "mechanics",
            LienKind::Municipal => "municipal",
            LienKind::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for LienKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "tax" | "Tax" => Ok(LienKind::Tax),
                "judgment" | "Judgment" => Ok(LienKind::Judgment),
                "hoa" | "Hoa" => Ok(LienKind::Hoa),
                "mechanics" | "Mechanics" => Ok(LienKind::Mechanics),
                "municipal" | "Municipal" => Ok(LienKind::Municipal),
                "other" | "Other" => Ok(LienKind::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for LienKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(LienKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for LienKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['tax', 'judgment', 'hoa', 'mechanics', 'municipal', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ParcelLineageKind {
#[cfg_attr(feature = "serde", serde(rename = "split"))]
    Split,
#[cfg_attr(feature = "serde", serde(rename = "merge"))]
    Merge,
#[cfg_attr(feature = "serde", serde(rename = "renumber"))]
    Renumber,
}

impl core::fmt::Display for ParcelLineageKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ParcelLineageKind::Split => f.write_str("split"),
            ParcelLineageKind::Merge => f.write_str("merge"),
            ParcelLineageKind::Renumber => f.write_str("renumber"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ParcelLineageKind {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ParcelLineageKind::Split => "split",
            ParcelLineageKind::Merge => "merge",
            ParcelLineageKind::Renumber => "renumber",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ParcelLineageKind {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "split" | "Split" => Ok(ParcelLineageKind::Split),
                "merge" | "Merge" => Ok(ParcelLineageKind::Merge),
                "renumber" | "Renumber" => Ok(ParcelLineageKind::Renumber),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ParcelLineageKind: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ParcelLineageKind)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ParcelLineageKind {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['split', 'merge', 'renumber']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EstateType {
#[cfg_attr(feature = "serde", serde(rename = "fee_simple"))]
    FeeSimple,
#[cfg_attr(feature = "serde", serde(rename = "leased_fee"))]
    LeasedFee,
#[cfg_attr(feature = "serde", serde(rename = "leasehold"))]
    Leasehold,
#[cfg_attr(feature = "serde", serde(rename = "life_estate"))]
    LifeEstate,
#[cfg_attr(feature = "serde", serde(rename = "cooperative_shares"))]
    CooperativeShares,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for EstateType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EstateType::FeeSimple => f.write_str("fee_simple"),
            EstateType::LeasedFee => f.write_str("leased_fee"),
            EstateType::Leasehold => f.write_str("leasehold"),
            EstateType::LifeEstate => f.write_str("life_estate"),
            EstateType::CooperativeShares => f.write_str("cooperative_shares"),
            EstateType::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for EstateType {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            EstateType::FeeSimple => "fee_simple",
            EstateType::LeasedFee => "leased_fee",
            EstateType::Leasehold => "leasehold",
            EstateType::LifeEstate => "life_estate",
            EstateType::CooperativeShares => "cooperative_shares",
            EstateType::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for EstateType {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "fee_simple" | "FeeSimple" => Ok(EstateType::FeeSimple),
                "leased_fee" | "LeasedFee" => Ok(EstateType::LeasedFee),
                "leasehold" | "Leasehold" => Ok(EstateType::Leasehold),
                "life_estate" | "LifeEstate" => Ok(EstateType::LifeEstate),
                "cooperative_shares" | "CooperativeShares" => Ok(EstateType::CooperativeShares),
                "other" | "Other" => Ok(EstateType::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for EstateType: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(EstateType)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for EstateType {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['fee_simple', 'leased_fee', 'leasehold', 'life_estate', 'cooperative_shares', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RatingScope {
#[cfg_attr(feature = "serde", serde(rename = "overall"))]
    Overall,
#[cfg_attr(feature = "serde", serde(rename = "exterior"))]
    Exterior,
#[cfg_attr(feature = "serde", serde(rename = "interior"))]
    Interior,
#[cfg_attr(feature = "serde", serde(rename = "component"))]
    Component,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for RatingScope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            RatingScope::Overall => f.write_str("overall"),
            RatingScope::Exterior => f.write_str("exterior"),
            RatingScope::Interior => f.write_str("interior"),
            RatingScope::Component => f.write_str("component"),
            RatingScope::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RatingScope {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            RatingScope::Overall => "overall",
            RatingScope::Exterior => "exterior",
            RatingScope::Interior => "interior",
            RatingScope::Component => "component",
            RatingScope::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RatingScope {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "overall" | "Overall" => Ok(RatingScope::Overall),
                "exterior" | "Exterior" => Ok(RatingScope::Exterior),
                "interior" | "Interior" => Ok(RatingScope::Interior),
                "component" | "Component" => Ok(RatingScope::Component),
                "other" | "Other" => Ok(RatingScope::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for RatingScope: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(RatingScope)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for RatingScope {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['overall', 'exterior', 'interior', 'component', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StatementBasis {
#[cfg_attr(feature = "serde", serde(rename = "actual"))]
    Actual,
#[cfg_attr(feature = "serde", serde(rename = "budget"))]
    Budget,
#[cfg_attr(feature = "serde", serde(rename = "pro_forma"))]
    ProForma,
#[cfg_attr(feature = "serde", serde(rename = "stabilized"))]
    Stabilized,
#[cfg_attr(feature = "serde", serde(rename = "projected"))]
    Projected,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for StatementBasis {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            StatementBasis::Actual => f.write_str("actual"),
            StatementBasis::Budget => f.write_str("budget"),
            StatementBasis::ProForma => f.write_str("pro_forma"),
            StatementBasis::Stabilized => f.write_str("stabilized"),
            StatementBasis::Projected => f.write_str("projected"),
            StatementBasis::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for StatementBasis {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            StatementBasis::Actual => "actual",
            StatementBasis::Budget => "budget",
            StatementBasis::ProForma => "pro_forma",
            StatementBasis::Stabilized => "stabilized",
            StatementBasis::Projected => "projected",
            StatementBasis::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for StatementBasis {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "actual" | "Actual" => Ok(StatementBasis::Actual),
                "budget" | "Budget" => Ok(StatementBasis::Budget),
                "pro_forma" | "ProForma" => Ok(StatementBasis::ProForma),
                "stabilized" | "Stabilized" => Ok(StatementBasis::Stabilized),
                "projected" | "Projected" => Ok(StatementBasis::Projected),
                "other" | "Other" => Ok(StatementBasis::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for StatementBasis: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(StatementBasis)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for StatementBasis {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['actual', 'budget', 'pro_forma', 'stabilized', 'projected', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GeocodeAccuracy {
#[cfg_attr(feature = "serde", serde(rename = "rooftop"))]
    Rooftop,
#[cfg_attr(feature = "serde", serde(rename = "parcel"))]
    Parcel,
#[cfg_attr(feature = "serde", serde(rename = "street"))]
    Street,
#[cfg_attr(feature = "serde", serde(rename = "postal_centroid"))]
    PostalCentroid,
#[cfg_attr(feature = "serde", serde(rename = "locality_centroid"))]
    LocalityCentroid,
#[cfg_attr(feature = "serde", serde(rename = "manual"))]
    Manual,
#[cfg_attr(feature = "serde", serde(rename = "unknown"))]
    Unknown,
}

impl core::fmt::Display for GeocodeAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            GeocodeAccuracy::Rooftop => f.write_str("rooftop"),
            GeocodeAccuracy::Parcel => f.write_str("parcel"),
            GeocodeAccuracy::Street => f.write_str("street"),
            GeocodeAccuracy::PostalCentroid => f.write_str("postal_centroid"),
            GeocodeAccuracy::LocalityCentroid => f.write_str("locality_centroid"),
            GeocodeAccuracy::Manual => f.write_str("manual"),
            GeocodeAccuracy::Unknown => f.write_str("unknown"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for GeocodeAccuracy {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            GeocodeAccuracy::Rooftop => "rooftop",
            GeocodeAccuracy::Parcel => "parcel",
            GeocodeAccuracy::Street => "street",
            GeocodeAccuracy::PostalCentroid => "postal_centroid",
            GeocodeAccuracy::LocalityCentroid => "locality_centroid",
            GeocodeAccuracy::Manual => "manual",
            GeocodeAccuracy::Unknown => "unknown",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for GeocodeAccuracy {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "rooftop" | "Rooftop" => Ok(GeocodeAccuracy::Rooftop),
                "parcel" | "Parcel" => Ok(GeocodeAccuracy::Parcel),
                "street" | "Street" => Ok(GeocodeAccuracy::Street),
                "postal_centroid" | "PostalCentroid" => Ok(GeocodeAccuracy::PostalCentroid),
                "locality_centroid" | "LocalityCentroid" => Ok(GeocodeAccuracy::LocalityCentroid),
                "manual" | "Manual" => Ok(GeocodeAccuracy::Manual),
                "unknown" | "Unknown" => Ok(GeocodeAccuracy::Unknown),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for GeocodeAccuracy: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(GeocodeAccuracy)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for GeocodeAccuracy {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['rooftop', 'parcel', 'street', 'postal_centroid', 'locality_centroid', 'manual', 'unknown']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExtractionStatus {
#[cfg_attr(feature = "serde", serde(rename = "success"))]
    Success,
#[cfg_attr(feature = "serde", serde(rename = "parse_error"))]
    ParseError,
#[cfg_attr(feature = "serde", serde(rename = "irrelevant_page"))]
    IrrelevantPage,
#[cfg_attr(feature = "serde", serde(rename = "model_error"))]
    ModelError,
}

impl core::fmt::Display for ExtractionStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExtractionStatus::Success => f.write_str("success"),
            ExtractionStatus::ParseError => f.write_str("parse_error"),
            ExtractionStatus::IrrelevantPage => f.write_str("irrelevant_page"),
            ExtractionStatus::ModelError => f.write_str("model_error"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ExtractionStatus {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ExtractionStatus::Success => "success",
            ExtractionStatus::ParseError => "parse_error",
            ExtractionStatus::IrrelevantPage => "irrelevant_page",
            ExtractionStatus::ModelError => "model_error",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ExtractionStatus {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "success" | "Success" => Ok(ExtractionStatus::Success),
                "parse_error" | "ParseError" => Ok(ExtractionStatus::ParseError),
                "irrelevant_page" | "IrrelevantPage" => Ok(ExtractionStatus::IrrelevantPage),
                "model_error" | "ModelError" => Ok(ExtractionStatus::ModelError),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ExtractionStatus: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ExtractionStatus)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ExtractionStatus {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['success', 'parse_error', 'irrelevant_page', 'model_error']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ExtractionCategory {
#[cfg_attr(feature = "serde", serde(rename = "sales_transaction"))]
    SalesTransaction,
#[cfg_attr(feature = "serde", serde(rename = "sale_listing"))]
    SaleListing,
#[cfg_attr(feature = "serde", serde(rename = "lease_listing"))]
    LeaseListing,
#[cfg_attr(feature = "serde", serde(rename = "in_place_lease"))]
    InPlaceLease,
#[cfg_attr(feature = "serde", serde(rename = "property_facts"))]
    PropertyFacts,
#[cfg_attr(feature = "serde", serde(rename = "other"))]
    Other,
}

impl core::fmt::Display for ExtractionCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExtractionCategory::SalesTransaction => f.write_str("sales_transaction"),
            ExtractionCategory::SaleListing => f.write_str("sale_listing"),
            ExtractionCategory::LeaseListing => f.write_str("lease_listing"),
            ExtractionCategory::InPlaceLease => f.write_str("in_place_lease"),
            ExtractionCategory::PropertyFacts => f.write_str("property_facts"),
            ExtractionCategory::Other => f.write_str("other"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for ExtractionCategory {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            ExtractionCategory::SalesTransaction => "sales_transaction",
            ExtractionCategory::SaleListing => "sale_listing",
            ExtractionCategory::LeaseListing => "lease_listing",
            ExtractionCategory::InPlaceLease => "in_place_lease",
            ExtractionCategory::PropertyFacts => "property_facts",
            ExtractionCategory::Other => "other",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for ExtractionCategory {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "sales_transaction" | "SalesTransaction" => Ok(ExtractionCategory::SalesTransaction),
                "sale_listing" | "SaleListing" => Ok(ExtractionCategory::SaleListing),
                "lease_listing" | "LeaseListing" => Ok(ExtractionCategory::LeaseListing),
                "in_place_lease" | "InPlaceLease" => Ok(ExtractionCategory::InPlaceLease),
                "property_facts" | "PropertyFacts" => Ok(ExtractionCategory::PropertyFacts),
                "other" | "Other" => Ok(ExtractionCategory::Other),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for ExtractionCategory: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(ExtractionCategory)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for ExtractionCategory {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['sales_transaction', 'sale_listing', 'lease_listing', 'in_place_lease', 'property_facts', 'other']",
            "typing".into(),
        )
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssessorStatus {
#[cfg_attr(feature = "serde", serde(rename = "success"))]
    Success,
#[cfg_attr(feature = "serde", serde(rename = "not_found"))]
    NotFound,
#[cfg_attr(feature = "serde", serde(rename = "timeout"))]
    Timeout,
#[cfg_attr(feature = "serde", serde(rename = "api_error"))]
    ApiError,
#[cfg_attr(feature = "serde", serde(rename = "parse_error"))]
    ParseError,
#[cfg_attr(feature = "serde", serde(rename = "invalid_address"))]
    InvalidAddress,
#[cfg_attr(feature = "serde", serde(rename = "ambiguous"))]
    Ambiguous,
}

impl core::fmt::Display for AssessorStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AssessorStatus::Success => f.write_str("success"),
            AssessorStatus::NotFound => f.write_str("not_found"),
            AssessorStatus::Timeout => f.write_str("timeout"),
            AssessorStatus::ApiError => f.write_str("api_error"),
            AssessorStatus::ParseError => f.write_str("parse_error"),
            AssessorStatus::InvalidAddress => f.write_str("invalid_address"),
            AssessorStatus::Ambiguous => f.write_str("ambiguous"),
        }
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for AssessorStatus {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let s: &str = match self {
            AssessorStatus::Success => "success",
            AssessorStatus::NotFound => "not_found",
            AssessorStatus::Timeout => "timeout",
            AssessorStatus::ApiError => "api_error",
            AssessorStatus::ParseError => "parse_error",
            AssessorStatus::InvalidAddress => "invalid_address",
            AssessorStatus::Ambiguous => "ambiguous",
        };
        Ok(pyo3::types::PyString::new(py, s).into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for AssessorStatus {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(s) = ob.extract::<&str>() {
            match s {
                "success" | "Success" => Ok(AssessorStatus::Success),
                "not_found" | "NotFound" => Ok(AssessorStatus::NotFound),
                "timeout" | "Timeout" => Ok(AssessorStatus::Timeout),
                "api_error" | "ApiError" => Ok(AssessorStatus::ApiError),
                "parse_error" | "ParseError" => Ok(AssessorStatus::ParseError),
                "invalid_address" | "InvalidAddress" => Ok(AssessorStatus::InvalidAddress),
                "ambiguous" | "Ambiguous" => Ok(AssessorStatus::Ambiguous),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("invalid value for AssessorStatus: {}", s),
                )),
            }
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                concat!("expected str for ", stringify!(AssessorStatus)),
            ))
        }
    }
}

#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for AssessorStatus {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::with_module(
            "typing.Literal['success', 'not_found', 'timeout', 'api_error', 'parse_error', 'invalid_address', 'ambiguous']",
            "typing".into(),
        )
    }
}

// Classes

#[derive(Clone, PartialEq)]
pub struct Anything(
    #[cfg(feature = "serde")] pub serde_value::Value,
    #[cfg(not(feature = "serde"))] pub (),
);


#[cfg(feature = "stubgen")]
impl ::pyo3_stub_gen::PyStubType for Anything {
    fn type_output() -> ::pyo3_stub_gen::TypeInfo {
        ::pyo3_stub_gen::TypeInfo::any()
    }
}


#[cfg(feature = "serde")]
impl Serialize for Anything {
    fn serialize<S>(&self, to_ser: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.0.serialize(to_ser)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Anything {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        <serde_value::Value as Deserialize>::deserialize(de).map(Anything)
    }
}

#[cfg(all(feature = "pyo3", feature = "serde"))]
impl<'py> FromPyObject<'py> for Anything {
    fn extract_bound(obj: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        use pyo3::types::{PyAny, PyDict, PyList, PyTuple};
        use serde_value::Value;

        fn py_to_value<'py>(o: &pyo3::Bound<'py, PyAny>) -> pyo3::PyResult<Value> {
            // None -> Unit
            if o.is_none() {
                return Ok(Value::Unit);
            }

            // Try simple primitives first
            if let Ok(s) = o.extract::<&str>() {
                return Ok(Value::String(s.to_string()));
            }
            if let Ok(b) = o.extract::<bool>() {
                return Ok(Value::Bool(b));
            }

            // Sequences (list/tuple)
            if let Ok(list) = o.downcast::<PyList>() {
                let mut out = Vec::with_capacity(list.len());
                for item in list.iter() {
                    out.push(py_to_value(&item)?);
                }
                return Ok(Value::Seq(out));
            }
            if let Ok(t) = o.downcast::<PyTuple>() {
                let mut out = Vec::with_capacity(t.len());
                for item in t.iter() {
                    out.push(py_to_value(&item)?);
                }
                return Ok(Value::Seq(out));
            }

            // Mappings (dict with string-like keys)
            if let Ok(d) = o.downcast::<PyDict>() {
                let mut map = std::collections::BTreeMap::<Value, Value>::new();
                for (k, v) in d.iter() {
                    // Only accept string-like keys for deterministic ordering
                    if let Ok(ks) = k.extract::<&str>() {
                        map.insert(Value::String(ks.to_string()), py_to_value(&v)?);
                    } else {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            "dict keys for Anything must be str",
                        ));
                    }
                }
                return Ok(Value::Map(map));
            }

            // Fallback: stringify unknown types
            let s = format!("{}", o.str()?);
            Ok(Value::String(s))
        }

        Ok(Anything(py_to_value(obj)?))
    }
}

/* ---------- getter side ---------- */
#[cfg(all(feature = "pyo3", feature = "serde"))]
impl<'py> IntoPyObject<'py> for Anything {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        use pyo3::types::{PyAny, PyDict, PyList, PyString};
        use serde_value::Value;

        fn value_to_py<'py>(py: Python<'py>, v: &Value) -> pyo3::PyResult<Bound<'py, PyAny>> {
            match v {
                Value::Unit => Ok(py.None().into_bound(py)),
                Value::Bool(b) => Ok(pyo3::types::PyBool::new(py, *b).to_owned().into_any()),
                Value::String(s) => Ok(PyString::new(py, s).into_any()),
                Value::Seq(seq) => {
                    let list = PyList::empty(py);
                    for item in seq.iter() {
                        let ob = value_to_py(py, item)?;
                        list.append(ob)?;
                    }
                    Ok(list.into_any())
                }
                Value::Map(map) => {
                    let dict = PyDict::new(py);
                    for (k, v) in map.iter() {
                        let pk = value_to_py(py, k)?;
                        let pv = value_to_py(py, v)?;
                        dict.set_item(pk, pv)?;
                    }
                    Ok(dict.into_any())
                }
                // Best-effort for other serde_value variants
                // (numbers, bytes, chars, etc.)
                other => {
                    // Try common cases without bringing extra deps
                    // Numbers are converted via string if not covered above
                    let s = format!("{:?}", other);
                    Ok(PyString::new(py, &s).into_any())
                }
            }
        }

        value_to_py(py, &self.0)
    }
}

impl std::fmt::Debug for Anything {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "serde")]
        return write!(f, "Anything({:?})", self.0);

        #[cfg(not(feature = "serde"))]
        return f.write_str("Anything(<opaque>)");
    }
}pub type Any = Anything;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Money {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_decimal_string"))]
    pub amount: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_iso_currency"))]
    pub currency: String
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Money {
    #[new]
    #[pyo3(signature = (amount, currency))]
    pub fn new(amount: String, currency: String) -> Self {
        Money{amount, currency}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Money>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Money> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Money>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Money",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Area {
    pub value: f64,
    pub unit: AreaUnit
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Area {
    #[new]
    #[pyo3(signature = (value, unit))]
    pub fn new(value: f64, unit: AreaUnit) -> Self {
        Area{value, unit}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Area>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Area> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Area>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Area",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Length {
    pub value: f64,
    pub unit: LengthUnit
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Length {
    #[new]
    #[pyo3(signature = (value, unit))]
    pub fn new(value: f64, unit: LengthUnit) -> Self {
        Length{value, unit}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Length>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Length> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Length>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Length",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct UnitRate {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_decimal_string"))]
    pub amount: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_iso_currency"))]
    pub currency: String,
    pub denominator: String
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl UnitRate {
    #[new]
    #[pyo3(signature = (amount, currency, denominator))]
    pub fn new(amount: String, currency: String, denominator: String) -> Self {
        UnitRate{amount, currency, denominator}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<UnitRate>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<UnitRate> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<UnitRate>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid UnitRate",
        ))
    }
}



pub mod codeable_concept_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum system_range {
        String(String),
        Uad36ConditionSystem(Uad36ConditionSystem),
        Uad36QualitySystem(Uad36QualitySystem)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for system_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<String>() {
                return Ok(system_range::String(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionSystem>() {
                return Ok(system_range::Uad36ConditionSystem(val));
            }            if let Ok(val) = ob.extract::<Uad36QualitySystem>() {
                return Ok(system_range::Uad36QualitySystem(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid system",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for system_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                system_range::String(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                system_range::Uad36ConditionSystem(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                system_range::Uad36QualitySystem(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<system_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<system_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<system_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid system",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(system_range = String | Uad36ConditionSystem | Uad36QualitySystem);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum code_range {
        String(String),
        Uad36ConditionCode(Uad36ConditionCode),
        Uad36QualityCode(Uad36QualityCode)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for code_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<String>() {
                return Ok(code_range::String(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionCode>() {
                return Ok(code_range::Uad36ConditionCode(val));
            }            if let Ok(val) = ob.extract::<Uad36QualityCode>() {
                return Ok(code_range::Uad36QualityCode(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid code",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for code_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                code_range::String(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                code_range::Uad36ConditionCode(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                code_range::Uad36QualityCode(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<code_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<code_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<code_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid code",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(code_range = String | Uad36ConditionCode | Uad36QualityCode);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct CodeableConcept {
    #[cfg_attr(feature = "serde", serde(default))]
    pub system: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl CodeableConcept {
    #[new]
    #[pyo3(signature = (system=None, code=None, display=None))]
    pub fn new(system: Option<String>, code: Option<String>, display: Option<String>) -> Self {
        CodeableConcept{system, code, display}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CodeableConcept>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CodeableConcept> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CodeableConcept>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CodeableConcept",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum CodeableConceptOrSubtype {    Classification(Classification),     Rating(Rating),     Uad36ConditionRating(Uad36ConditionRating),     Uad36QualityRating(Uad36QualityRating),     CodeableConcept(CodeableConcept)}

impl From<Classification>   for CodeableConceptOrSubtype { fn from(x: Classification)   -> Self { Self::Classification(x) } }
impl From<Rating>   for CodeableConceptOrSubtype { fn from(x: Rating)   -> Self { Self::Rating(x) } }
impl From<Uad36ConditionRating>   for CodeableConceptOrSubtype { fn from(x: Uad36ConditionRating)   -> Self { Self::Uad36ConditionRating(x) } }
impl From<Uad36QualityRating>   for CodeableConceptOrSubtype { fn from(x: Uad36QualityRating)   -> Self { Self::Uad36QualityRating(x) } }
impl From<CodeableConcept>   for CodeableConceptOrSubtype { fn from(x: CodeableConcept)   -> Self { Self::CodeableConcept(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for CodeableConceptOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Classification>() {
            return Ok(CodeableConceptOrSubtype::Classification(val));
        }        if let Ok(val) = ob.extract::<Rating>() {
            return Ok(CodeableConceptOrSubtype::Rating(val));
        }        if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
            return Ok(CodeableConceptOrSubtype::Uad36ConditionRating(val));
        }        if let Ok(val) = ob.extract::<Uad36QualityRating>() {
            return Ok(CodeableConceptOrSubtype::Uad36QualityRating(val));
        }        if let Ok(val) = ob.extract::<CodeableConcept>() {
            return Ok(CodeableConceptOrSubtype::CodeableConcept(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CodeableConceptOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for CodeableConceptOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            CodeableConceptOrSubtype::Classification(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CodeableConceptOrSubtype::Rating(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CodeableConceptOrSubtype::Uad36ConditionRating(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CodeableConceptOrSubtype::Uad36QualityRating(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            CodeableConceptOrSubtype::CodeableConcept(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CodeableConceptOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CodeableConceptOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CodeableConceptOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CodeableConceptOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(CodeableConceptOrSubtype = Classification | Rating | Uad36ConditionRating | Uad36QualityRating | CodeableConcept);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Classification {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub system: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub code: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Classification {
    #[new]
    #[pyo3(signature = (system, code, display=None))]
    pub fn new(system: String, code: String, display: Option<String>) -> Self {
        Classification{system, code, display}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Classification>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Classification> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Classification>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Classification",
        ))
    }
}



pub mod rating_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum system_range {
        String(String),
        Uad36ConditionSystem(Uad36ConditionSystem),
        Uad36QualitySystem(Uad36QualitySystem)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for system_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<String>() {
                return Ok(system_range::String(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionSystem>() {
                return Ok(system_range::Uad36ConditionSystem(val));
            }            if let Ok(val) = ob.extract::<Uad36QualitySystem>() {
                return Ok(system_range::Uad36QualitySystem(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid system",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for system_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                system_range::String(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                system_range::Uad36ConditionSystem(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                system_range::Uad36QualitySystem(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<system_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<system_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<system_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid system",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(system_range = String | Uad36ConditionSystem | Uad36QualitySystem);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum code_range {
        String(String),
        Uad36ConditionCode(Uad36ConditionCode),
        Uad36QualityCode(Uad36QualityCode)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for code_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<String>() {
                return Ok(code_range::String(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionCode>() {
                return Ok(code_range::Uad36ConditionCode(val));
            }            if let Ok(val) = ob.extract::<Uad36QualityCode>() {
                return Ok(code_range::Uad36QualityCode(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid code",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for code_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                code_range::String(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                code_range::Uad36ConditionCode(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                code_range::Uad36QualityCode(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<code_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<code_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<code_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid code",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(code_range = String | Uad36ConditionCode | Uad36QualityCode);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum scope_range {
        RatingScope(RatingScope),
        Uad36RatingScope(Uad36RatingScope)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for scope_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<RatingScope>() {
                return Ok(scope_range::RatingScope(val));
            }            if let Ok(val) = ob.extract::<Uad36RatingScope>() {
                return Ok(scope_range::Uad36RatingScope(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid scope",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for scope_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                scope_range::RatingScope(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                scope_range::Uad36RatingScope(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<scope_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<scope_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<scope_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid scope",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(scope_range = RatingScope | Uad36RatingScope);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Rating {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub system: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub code: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub scope: Option<RatingScope>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Rating {
    #[new]
    #[pyo3(signature = (system, code, description=None, scope=None, display=None))]
    pub fn new(system: String, code: String, description: Option<String>, scope: Option<RatingScope>, display: Option<String>) -> Self {
        Rating{system, code, description, scope, display}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Rating>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Rating> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Rating>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Rating",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum RatingOrSubtype {    Uad36ConditionRating(Uad36ConditionRating),     Uad36QualityRating(Uad36QualityRating)}

impl From<Uad36ConditionRating>   for RatingOrSubtype { fn from(x: Uad36ConditionRating)   -> Self { Self::Uad36ConditionRating(x) } }
impl From<Uad36QualityRating>   for RatingOrSubtype { fn from(x: Uad36QualityRating)   -> Self { Self::Uad36QualityRating(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RatingOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
            return Ok(RatingOrSubtype::Uad36ConditionRating(val));
        }        if let Ok(val) = ob.extract::<Uad36QualityRating>() {
            return Ok(RatingOrSubtype::Uad36QualityRating(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RatingOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RatingOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            RatingOrSubtype::Uad36ConditionRating(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            RatingOrSubtype::Uad36QualityRating(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RatingOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RatingOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RatingOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RatingOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(RatingOrSubtype = Uad36ConditionRating | Uad36QualityRating);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36ConditionRating {
    pub system: Uad36ConditionSystem,
    pub code: Uad36ConditionCode,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    pub scope: Uad36RatingScope,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36ConditionRating {
    #[new]
    #[pyo3(signature = (system, code, scope, description=None, display=None))]
    pub fn new(system: Uad36ConditionSystem, code: Uad36ConditionCode, scope: Uad36RatingScope, description: Option<String>, display: Option<String>) -> Self {
        Uad36ConditionRating{system, code, scope, description, display}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36ConditionRating>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36ConditionRating> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36ConditionRating",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36QualityRating {
    pub system: Uad36QualitySystem,
    pub code: Uad36QualityCode,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    pub scope: Uad36RatingScope,
    #[cfg_attr(feature = "serde", serde(default))]
    pub display: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36QualityRating {
    #[new]
    #[pyo3(signature = (system, code, scope, description=None, display=None))]
    pub fn new(system: Uad36QualitySystem, code: Uad36QualityCode, scope: Uad36RatingScope, description: Option<String>, display: Option<String>) -> Self {
        Uad36QualityRating{system, code, scope, description, display}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36QualityRating>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36QualityRating> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36QualityRating>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36QualityRating",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl GeoPoint {
    #[new]
    #[pyo3(signature = (latitude, longitude))]
    pub fn new(latitude: f64, longitude: f64) -> Self {
        GeoPoint{latitude, longitude}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<GeoPoint>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<GeoPoint> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<GeoPoint>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid GeoPoint",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Geometry {
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub type_: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub coordinates: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Geometry {
    #[new]
    #[pyo3(signature = (type_, coordinates=None))]
    pub fn new(type_: String, coordinates: Option<serde_utils::PyValue<Any>>) -> Self {
        let coordinates = coordinates.map(|v| v.into_inner());
        Geometry{type_, coordinates}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Geometry>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Geometry> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Geometry>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Geometry",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Provenance {
    #[cfg_attr(feature = "serde", serde(default))]
    pub provider: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_url: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub retrieved_at: Option<DateTime<FixedOffset>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub method: Option<CaptureMethod>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub confidence: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verification: Option<VerificationStatus>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Provenance {
    #[new]
    #[pyo3(signature = (provider=None, source_url=None, retrieved_at=None, method=None, confidence=None, verification=None))]
    pub fn new(provider: Option<String>, source_url: Option<uri>, retrieved_at: Option<DateTime<FixedOffset>>, method: Option<CaptureMethod>, confidence: Option<f64>, verification: Option<VerificationStatus>) -> Self {
        Provenance{provider, source_url, retrieved_at, method, confidence, verification}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Provenance>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Provenance> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Provenance>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Provenance",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Entity {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Entity {
    #[new]
    #[pyo3(signature = (id, extras=None, provenance=None, verifications=None))]
    pub fn new(id: String, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Entity{id, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Entity>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Entity> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Entity>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Entity",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Entity {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }


    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum EntityOrSubtype {    Jurisdiction(Jurisdiction),     Address(Address),     Property(Property),     Parcel(Parcel),     PropertyParcel(PropertyParcel),     ParcelLineage(ParcelLineage),     PropertyIdentifier(PropertyIdentifier),     Party(Party),     SourceArtifact(SourceArtifact),     PropertyAddress(PropertyAddress),     OwnershipPeriod(OwnershipPeriod),     Structure(Structure),     Site(Site),     Space(Space),     PropertyState(PropertyState),     SiteState(SiteState),     StructureState(StructureState),     SpaceState(SpaceState),     PropertyStateSnapshot(PropertyStateSnapshot),     PropertyAssociation(PropertyAssociation),     Assessment(Assessment),     TaxBill(TaxBill),     Transfer(Transfer),     SaleEvent(SaleEvent),     Listing(Listing),     LeaseEvent(LeaseEvent),     UnitRentObservation(UnitRentObservation),     Loan(Loan),     Lien(Lien),     ForeclosureCase(ForeclosureCase),     Permit(Permit),     OperatingStatement(OperatingStatement),     RentRoll(RentRoll),     Valuation(Valuation),     Uad36PropertyStateSnapshot(Uad36PropertyStateSnapshot),     Uad36StructureState(Uad36StructureState),     Uad36Structure(Uad36Structure)}

impl From<Jurisdiction>   for EntityOrSubtype { fn from(x: Jurisdiction)   -> Self { Self::Jurisdiction(x) } }
impl From<Address>   for EntityOrSubtype { fn from(x: Address)   -> Self { Self::Address(x) } }
impl From<Property>   for EntityOrSubtype { fn from(x: Property)   -> Self { Self::Property(x) } }
impl From<Parcel>   for EntityOrSubtype { fn from(x: Parcel)   -> Self { Self::Parcel(x) } }
impl From<PropertyParcel>   for EntityOrSubtype { fn from(x: PropertyParcel)   -> Self { Self::PropertyParcel(x) } }
impl From<ParcelLineage>   for EntityOrSubtype { fn from(x: ParcelLineage)   -> Self { Self::ParcelLineage(x) } }
impl From<PropertyIdentifier>   for EntityOrSubtype { fn from(x: PropertyIdentifier)   -> Self { Self::PropertyIdentifier(x) } }
impl From<Party>   for EntityOrSubtype { fn from(x: Party)   -> Self { Self::Party(x) } }
impl From<SourceArtifact>   for EntityOrSubtype { fn from(x: SourceArtifact)   -> Self { Self::SourceArtifact(x) } }
impl From<PropertyAddress>   for EntityOrSubtype { fn from(x: PropertyAddress)   -> Self { Self::PropertyAddress(x) } }
impl From<OwnershipPeriod>   for EntityOrSubtype { fn from(x: OwnershipPeriod)   -> Self { Self::OwnershipPeriod(x) } }
impl From<Structure>   for EntityOrSubtype { fn from(x: Structure)   -> Self { Self::Structure(x) } }
impl From<Site>   for EntityOrSubtype { fn from(x: Site)   -> Self { Self::Site(x) } }
impl From<Space>   for EntityOrSubtype { fn from(x: Space)   -> Self { Self::Space(x) } }
impl From<PropertyState>   for EntityOrSubtype { fn from(x: PropertyState)   -> Self { Self::PropertyState(x) } }
impl From<SiteState>   for EntityOrSubtype { fn from(x: SiteState)   -> Self { Self::SiteState(x) } }
impl From<StructureState>   for EntityOrSubtype { fn from(x: StructureState)   -> Self { Self::StructureState(x) } }
impl From<SpaceState>   for EntityOrSubtype { fn from(x: SpaceState)   -> Self { Self::SpaceState(x) } }
impl From<PropertyStateSnapshot>   for EntityOrSubtype { fn from(x: PropertyStateSnapshot)   -> Self { Self::PropertyStateSnapshot(x) } }
impl From<PropertyAssociation>   for EntityOrSubtype { fn from(x: PropertyAssociation)   -> Self { Self::PropertyAssociation(x) } }
impl From<Assessment>   for EntityOrSubtype { fn from(x: Assessment)   -> Self { Self::Assessment(x) } }
impl From<TaxBill>   for EntityOrSubtype { fn from(x: TaxBill)   -> Self { Self::TaxBill(x) } }
impl From<Transfer>   for EntityOrSubtype { fn from(x: Transfer)   -> Self { Self::Transfer(x) } }
impl From<SaleEvent>   for EntityOrSubtype { fn from(x: SaleEvent)   -> Self { Self::SaleEvent(x) } }
impl From<Listing>   for EntityOrSubtype { fn from(x: Listing)   -> Self { Self::Listing(x) } }
impl From<LeaseEvent>   for EntityOrSubtype { fn from(x: LeaseEvent)   -> Self { Self::LeaseEvent(x) } }
impl From<UnitRentObservation>   for EntityOrSubtype { fn from(x: UnitRentObservation)   -> Self { Self::UnitRentObservation(x) } }
impl From<Loan>   for EntityOrSubtype { fn from(x: Loan)   -> Self { Self::Loan(x) } }
impl From<Lien>   for EntityOrSubtype { fn from(x: Lien)   -> Self { Self::Lien(x) } }
impl From<ForeclosureCase>   for EntityOrSubtype { fn from(x: ForeclosureCase)   -> Self { Self::ForeclosureCase(x) } }
impl From<Permit>   for EntityOrSubtype { fn from(x: Permit)   -> Self { Self::Permit(x) } }
impl From<OperatingStatement>   for EntityOrSubtype { fn from(x: OperatingStatement)   -> Self { Self::OperatingStatement(x) } }
impl From<RentRoll>   for EntityOrSubtype { fn from(x: RentRoll)   -> Self { Self::RentRoll(x) } }
impl From<Valuation>   for EntityOrSubtype { fn from(x: Valuation)   -> Self { Self::Valuation(x) } }
impl From<Uad36PropertyStateSnapshot>   for EntityOrSubtype { fn from(x: Uad36PropertyStateSnapshot)   -> Self { Self::Uad36PropertyStateSnapshot(x) } }
impl From<Uad36StructureState>   for EntityOrSubtype { fn from(x: Uad36StructureState)   -> Self { Self::Uad36StructureState(x) } }
impl From<Uad36Structure>   for EntityOrSubtype { fn from(x: Uad36Structure)   -> Self { Self::Uad36Structure(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for EntityOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Jurisdiction>() {
            return Ok(EntityOrSubtype::Jurisdiction(val));
        }        if let Ok(val) = ob.extract::<Address>() {
            return Ok(EntityOrSubtype::Address(val));
        }        if let Ok(val) = ob.extract::<Property>() {
            return Ok(EntityOrSubtype::Property(val));
        }        if let Ok(val) = ob.extract::<Parcel>() {
            return Ok(EntityOrSubtype::Parcel(val));
        }        if let Ok(val) = ob.extract::<PropertyParcel>() {
            return Ok(EntityOrSubtype::PropertyParcel(val));
        }        if let Ok(val) = ob.extract::<ParcelLineage>() {
            return Ok(EntityOrSubtype::ParcelLineage(val));
        }        if let Ok(val) = ob.extract::<PropertyIdentifier>() {
            return Ok(EntityOrSubtype::PropertyIdentifier(val));
        }        if let Ok(val) = ob.extract::<Party>() {
            return Ok(EntityOrSubtype::Party(val));
        }        if let Ok(val) = ob.extract::<SourceArtifact>() {
            return Ok(EntityOrSubtype::SourceArtifact(val));
        }        if let Ok(val) = ob.extract::<PropertyAddress>() {
            return Ok(EntityOrSubtype::PropertyAddress(val));
        }        if let Ok(val) = ob.extract::<OwnershipPeriod>() {
            return Ok(EntityOrSubtype::OwnershipPeriod(val));
        }        if let Ok(val) = ob.extract::<Structure>() {
            return Ok(EntityOrSubtype::Structure(val));
        }        if let Ok(val) = ob.extract::<Site>() {
            return Ok(EntityOrSubtype::Site(val));
        }        if let Ok(val) = ob.extract::<Space>() {
            return Ok(EntityOrSubtype::Space(val));
        }        if let Ok(val) = ob.extract::<PropertyState>() {
            return Ok(EntityOrSubtype::PropertyState(val));
        }        if let Ok(val) = ob.extract::<SiteState>() {
            return Ok(EntityOrSubtype::SiteState(val));
        }        if let Ok(val) = ob.extract::<StructureState>() {
            return Ok(EntityOrSubtype::StructureState(val));
        }        if let Ok(val) = ob.extract::<SpaceState>() {
            return Ok(EntityOrSubtype::SpaceState(val));
        }        if let Ok(val) = ob.extract::<PropertyStateSnapshot>() {
            return Ok(EntityOrSubtype::PropertyStateSnapshot(val));
        }        if let Ok(val) = ob.extract::<PropertyAssociation>() {
            return Ok(EntityOrSubtype::PropertyAssociation(val));
        }        if let Ok(val) = ob.extract::<Assessment>() {
            return Ok(EntityOrSubtype::Assessment(val));
        }        if let Ok(val) = ob.extract::<TaxBill>() {
            return Ok(EntityOrSubtype::TaxBill(val));
        }        if let Ok(val) = ob.extract::<Transfer>() {
            return Ok(EntityOrSubtype::Transfer(val));
        }        if let Ok(val) = ob.extract::<SaleEvent>() {
            return Ok(EntityOrSubtype::SaleEvent(val));
        }        if let Ok(val) = ob.extract::<Listing>() {
            return Ok(EntityOrSubtype::Listing(val));
        }        if let Ok(val) = ob.extract::<LeaseEvent>() {
            return Ok(EntityOrSubtype::LeaseEvent(val));
        }        if let Ok(val) = ob.extract::<UnitRentObservation>() {
            return Ok(EntityOrSubtype::UnitRentObservation(val));
        }        if let Ok(val) = ob.extract::<Loan>() {
            return Ok(EntityOrSubtype::Loan(val));
        }        if let Ok(val) = ob.extract::<Lien>() {
            return Ok(EntityOrSubtype::Lien(val));
        }        if let Ok(val) = ob.extract::<ForeclosureCase>() {
            return Ok(EntityOrSubtype::ForeclosureCase(val));
        }        if let Ok(val) = ob.extract::<Permit>() {
            return Ok(EntityOrSubtype::Permit(val));
        }        if let Ok(val) = ob.extract::<OperatingStatement>() {
            return Ok(EntityOrSubtype::OperatingStatement(val));
        }        if let Ok(val) = ob.extract::<RentRoll>() {
            return Ok(EntityOrSubtype::RentRoll(val));
        }        if let Ok(val) = ob.extract::<Valuation>() {
            return Ok(EntityOrSubtype::Valuation(val));
        }        if let Ok(val) = ob.extract::<Uad36PropertyStateSnapshot>() {
            return Ok(EntityOrSubtype::Uad36PropertyStateSnapshot(val));
        }        if let Ok(val) = ob.extract::<Uad36StructureState>() {
            return Ok(EntityOrSubtype::Uad36StructureState(val));
        }        if let Ok(val) = ob.extract::<Uad36Structure>() {
            return Ok(EntityOrSubtype::Uad36Structure(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EntityOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for EntityOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            EntityOrSubtype::Jurisdiction(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Address(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Property(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Parcel(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyParcel(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::ParcelLineage(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyIdentifier(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Party(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::SourceArtifact(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyAddress(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::OwnershipPeriod(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Structure(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Site(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Space(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::SiteState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::StructureState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::SpaceState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyStateSnapshot(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::PropertyAssociation(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Assessment(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::TaxBill(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Transfer(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::SaleEvent(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Listing(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::LeaseEvent(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::UnitRentObservation(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Loan(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Lien(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::ForeclosureCase(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Permit(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::OperatingStatement(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::RentRoll(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Valuation(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Uad36PropertyStateSnapshot(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Uad36StructureState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            EntityOrSubtype::Uad36Structure(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<EntityOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<EntityOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<EntityOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid EntityOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for EntityOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Jurisdiction::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Jurisdiction(x));
        }
        if let Ok(x) = Address::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Address(x));
        }
        if let Ok(x) = Property::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Property(x));
        }
        if let Ok(x) = Parcel::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Parcel(x));
        }
        if let Ok(x) = PropertyParcel::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyParcel(x));
        }
        if let Ok(x) = ParcelLineage::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::ParcelLineage(x));
        }
        if let Ok(x) = PropertyIdentifier::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyIdentifier(x));
        }
        if let Ok(x) = Party::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Party(x));
        }
        if let Ok(x) = SourceArtifact::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SourceArtifact(x));
        }
        if let Ok(x) = PropertyAddress::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyAddress(x));
        }
        if let Ok(x) = OwnershipPeriod::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::OwnershipPeriod(x));
        }
        if let Ok(x) = Structure::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Structure(x));
        }
        if let Ok(x) = Site::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Site(x));
        }
        if let Ok(x) = Space::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Space(x));
        }
        if let Ok(x) = PropertyState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyState(x));
        }
        if let Ok(x) = SiteState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SiteState(x));
        }
        if let Ok(x) = StructureState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::StructureState(x));
        }
        if let Ok(x) = SpaceState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SpaceState(x));
        }
        if let Ok(x) = PropertyStateSnapshot::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyStateSnapshot(x));
        }
        if let Ok(x) = PropertyAssociation::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyAssociation(x));
        }
        if let Ok(x) = Assessment::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Assessment(x));
        }
        if let Ok(x) = TaxBill::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::TaxBill(x));
        }
        if let Ok(x) = Transfer::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Transfer(x));
        }
        if let Ok(x) = SaleEvent::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SaleEvent(x));
        }
        if let Ok(x) = Listing::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Listing(x));
        }
        if let Ok(x) = LeaseEvent::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::LeaseEvent(x));
        }
        if let Ok(x) = UnitRentObservation::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::UnitRentObservation(x));
        }
        if let Ok(x) = Loan::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Loan(x));
        }
        if let Ok(x) = Lien::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Lien(x));
        }
        if let Ok(x) = ForeclosureCase::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::ForeclosureCase(x));
        }
        if let Ok(x) = Permit::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Permit(x));
        }
        if let Ok(x) = OperatingStatement::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::OperatingStatement(x));
        }
        if let Ok(x) = RentRoll::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::RentRoll(x));
        }
        if let Ok(x) = Valuation::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Valuation(x));
        }
        if let Ok(x) = Uad36PropertyStateSnapshot::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36PropertyStateSnapshot(x));
        }
        if let Ok(x) = Uad36StructureState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36StructureState(x));
        }
        if let Ok(x) = Uad36Structure::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36Structure(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Jurisdiction::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Jurisdiction(x));
        }
        if let Ok(x) = Address::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Address(x));
        }
        if let Ok(x) = Property::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Property(x));
        }
        if let Ok(x) = Parcel::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Parcel(x));
        }
        if let Ok(x) = PropertyParcel::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyParcel(x));
        }
        if let Ok(x) = ParcelLineage::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::ParcelLineage(x));
        }
        if let Ok(x) = PropertyIdentifier::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyIdentifier(x));
        }
        if let Ok(x) = Party::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Party(x));
        }
        if let Ok(x) = SourceArtifact::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SourceArtifact(x));
        }
        if let Ok(x) = PropertyAddress::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyAddress(x));
        }
        if let Ok(x) = OwnershipPeriod::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::OwnershipPeriod(x));
        }
        if let Ok(x) = Structure::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Structure(x));
        }
        if let Ok(x) = Site::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Site(x));
        }
        if let Ok(x) = Space::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Space(x));
        }
        if let Ok(x) = PropertyState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyState(x));
        }
        if let Ok(x) = SiteState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SiteState(x));
        }
        if let Ok(x) = StructureState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::StructureState(x));
        }
        if let Ok(x) = SpaceState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SpaceState(x));
        }
        if let Ok(x) = PropertyStateSnapshot::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyStateSnapshot(x));
        }
        if let Ok(x) = PropertyAssociation::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::PropertyAssociation(x));
        }
        if let Ok(x) = Assessment::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Assessment(x));
        }
        if let Ok(x) = TaxBill::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::TaxBill(x));
        }
        if let Ok(x) = Transfer::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Transfer(x));
        }
        if let Ok(x) = SaleEvent::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::SaleEvent(x));
        }
        if let Ok(x) = Listing::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Listing(x));
        }
        if let Ok(x) = LeaseEvent::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::LeaseEvent(x));
        }
        if let Ok(x) = UnitRentObservation::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::UnitRentObservation(x));
        }
        if let Ok(x) = Loan::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Loan(x));
        }
        if let Ok(x) = Lien::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Lien(x));
        }
        if let Ok(x) = ForeclosureCase::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::ForeclosureCase(x));
        }
        if let Ok(x) = Permit::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Permit(x));
        }
        if let Ok(x) = OperatingStatement::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::OperatingStatement(x));
        }
        if let Ok(x) = RentRoll::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::RentRoll(x));
        }
        if let Ok(x) = Valuation::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Valuation(x));
        }
        if let Ok(x) = Uad36PropertyStateSnapshot::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36PropertyStateSnapshot(x));
        }
        if let Ok(x) = Uad36StructureState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36StructureState(x));
        }
        if let Ok(x) = Uad36Structure::from_pair_simple(k.clone(), v.clone()) {
            return Ok(EntityOrSubtype::Uad36Structure(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            EntityOrSubtype::Jurisdiction(inner) => inner.extract_key(),
            EntityOrSubtype::Address(inner) => inner.extract_key(),
            EntityOrSubtype::Property(inner) => inner.extract_key(),
            EntityOrSubtype::Parcel(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyParcel(inner) => inner.extract_key(),
            EntityOrSubtype::ParcelLineage(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyIdentifier(inner) => inner.extract_key(),
            EntityOrSubtype::Party(inner) => inner.extract_key(),
            EntityOrSubtype::SourceArtifact(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyAddress(inner) => inner.extract_key(),
            EntityOrSubtype::OwnershipPeriod(inner) => inner.extract_key(),
            EntityOrSubtype::Structure(inner) => inner.extract_key(),
            EntityOrSubtype::Site(inner) => inner.extract_key(),
            EntityOrSubtype::Space(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyState(inner) => inner.extract_key(),
            EntityOrSubtype::SiteState(inner) => inner.extract_key(),
            EntityOrSubtype::StructureState(inner) => inner.extract_key(),
            EntityOrSubtype::SpaceState(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyStateSnapshot(inner) => inner.extract_key(),
            EntityOrSubtype::PropertyAssociation(inner) => inner.extract_key(),
            EntityOrSubtype::Assessment(inner) => inner.extract_key(),
            EntityOrSubtype::TaxBill(inner) => inner.extract_key(),
            EntityOrSubtype::Transfer(inner) => inner.extract_key(),
            EntityOrSubtype::SaleEvent(inner) => inner.extract_key(),
            EntityOrSubtype::Listing(inner) => inner.extract_key(),
            EntityOrSubtype::LeaseEvent(inner) => inner.extract_key(),
            EntityOrSubtype::UnitRentObservation(inner) => inner.extract_key(),
            EntityOrSubtype::Loan(inner) => inner.extract_key(),
            EntityOrSubtype::Lien(inner) => inner.extract_key(),
            EntityOrSubtype::ForeclosureCase(inner) => inner.extract_key(),
            EntityOrSubtype::Permit(inner) => inner.extract_key(),
            EntityOrSubtype::OperatingStatement(inner) => inner.extract_key(),
            EntityOrSubtype::RentRoll(inner) => inner.extract_key(),
            EntityOrSubtype::Valuation(inner) => inner.extract_key(),
            EntityOrSubtype::Uad36PropertyStateSnapshot(inner) => inner.extract_key(),
            EntityOrSubtype::Uad36StructureState(inner) => inner.extract_key(),
            EntityOrSubtype::Uad36Structure(inner) => inner.extract_key(),
        }
    }
}

#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(EntityOrSubtype = Jurisdiction | Address | Property | Parcel | PropertyParcel | ParcelLineage | PropertyIdentifier | Party | SourceArtifact | PropertyAddress | OwnershipPeriod | Structure | Site | Space | PropertyState | SiteState | StructureState | SpaceState | PropertyStateSnapshot | PropertyAssociation | Assessment | TaxBill | Transfer | SaleEvent | Listing | LeaseEvent | UnitRentObservation | Loan | Lien | ForeclosureCase | Permit | OperatingStatement | RentRoll | Valuation | Uad36PropertyStateSnapshot | Uad36StructureState | Uad36Structure);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct InstrumentReference {
    #[cfg_attr(feature = "serde", serde(default))]
    pub relationship_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl InstrumentReference {
    #[new]
    #[pyo3(signature = (relationship_type=None, document_number=None, registry_reference=None, recording_authority=None, extras=None))]
    pub fn new(relationship_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, document_number: Option<String>, registry_reference: Option<String>, recording_authority: Option<String>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let relationship_type = relationship_type.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        InstrumentReference{relationship_type, document_number, registry_reference, recording_authority, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<InstrumentReference>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<InstrumentReference> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<InstrumentReference>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid InstrumentReference",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct RecordedInstrument {
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl RecordedInstrument {
    #[new]
    #[pyo3(signature = (document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None))]
    pub fn new(document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>) -> Self {
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        RecordedInstrument{document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RecordedInstrument>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RecordedInstrument> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RecordedInstrument>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RecordedInstrument",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum RecordedInstrumentOrSubtype {    Transfer(Transfer),     Loan(Loan),     LoanEvent(LoanEvent),     Lien(Lien),     ForeclosureFiling(ForeclosureFiling)}

impl From<Transfer>   for RecordedInstrumentOrSubtype { fn from(x: Transfer)   -> Self { Self::Transfer(x) } }
impl From<Loan>   for RecordedInstrumentOrSubtype { fn from(x: Loan)   -> Self { Self::Loan(x) } }
impl From<LoanEvent>   for RecordedInstrumentOrSubtype { fn from(x: LoanEvent)   -> Self { Self::LoanEvent(x) } }
impl From<Lien>   for RecordedInstrumentOrSubtype { fn from(x: Lien)   -> Self { Self::Lien(x) } }
impl From<ForeclosureFiling>   for RecordedInstrumentOrSubtype { fn from(x: ForeclosureFiling)   -> Self { Self::ForeclosureFiling(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for RecordedInstrumentOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Transfer>() {
            return Ok(RecordedInstrumentOrSubtype::Transfer(val));
        }        if let Ok(val) = ob.extract::<Loan>() {
            return Ok(RecordedInstrumentOrSubtype::Loan(val));
        }        if let Ok(val) = ob.extract::<LoanEvent>() {
            return Ok(RecordedInstrumentOrSubtype::LoanEvent(val));
        }        if let Ok(val) = ob.extract::<Lien>() {
            return Ok(RecordedInstrumentOrSubtype::Lien(val));
        }        if let Ok(val) = ob.extract::<ForeclosureFiling>() {
            return Ok(RecordedInstrumentOrSubtype::ForeclosureFiling(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RecordedInstrumentOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for RecordedInstrumentOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            RecordedInstrumentOrSubtype::Transfer(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            RecordedInstrumentOrSubtype::Loan(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            RecordedInstrumentOrSubtype::LoanEvent(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            RecordedInstrumentOrSubtype::Lien(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RecordedInstrumentOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RecordedInstrumentOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RecordedInstrumentOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RecordedInstrumentOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(RecordedInstrumentOrSubtype = Transfer | Loan | LoanEvent | Lien | ForeclosureFiling);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TransactionParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TransactionParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        TransactionParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TransactionParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TransactionParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TransactionParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TransactionParty",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum TransactionPartyOrSubtype {    TransferParty(TransferParty),     SaleEventParty(SaleEventParty),     ListingParticipant(ListingParticipant),     LeaseEventParty(LeaseEventParty),     LoanParty(LoanParty),     LienParty(LienParty),     ForeclosureCaseParty(ForeclosureCaseParty)}

impl From<TransferParty>   for TransactionPartyOrSubtype { fn from(x: TransferParty)   -> Self { Self::TransferParty(x) } }
impl From<SaleEventParty>   for TransactionPartyOrSubtype { fn from(x: SaleEventParty)   -> Self { Self::SaleEventParty(x) } }
impl From<ListingParticipant>   for TransactionPartyOrSubtype { fn from(x: ListingParticipant)   -> Self { Self::ListingParticipant(x) } }
impl From<LeaseEventParty>   for TransactionPartyOrSubtype { fn from(x: LeaseEventParty)   -> Self { Self::LeaseEventParty(x) } }
impl From<LoanParty>   for TransactionPartyOrSubtype { fn from(x: LoanParty)   -> Self { Self::LoanParty(x) } }
impl From<LienParty>   for TransactionPartyOrSubtype { fn from(x: LienParty)   -> Self { Self::LienParty(x) } }
impl From<ForeclosureCaseParty>   for TransactionPartyOrSubtype { fn from(x: ForeclosureCaseParty)   -> Self { Self::ForeclosureCaseParty(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for TransactionPartyOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TransferParty>() {
            return Ok(TransactionPartyOrSubtype::TransferParty(val));
        }        if let Ok(val) = ob.extract::<SaleEventParty>() {
            return Ok(TransactionPartyOrSubtype::SaleEventParty(val));
        }        if let Ok(val) = ob.extract::<ListingParticipant>() {
            return Ok(TransactionPartyOrSubtype::ListingParticipant(val));
        }        if let Ok(val) = ob.extract::<LeaseEventParty>() {
            return Ok(TransactionPartyOrSubtype::LeaseEventParty(val));
        }        if let Ok(val) = ob.extract::<LoanParty>() {
            return Ok(TransactionPartyOrSubtype::LoanParty(val));
        }        if let Ok(val) = ob.extract::<LienParty>() {
            return Ok(TransactionPartyOrSubtype::LienParty(val));
        }        if let Ok(val) = ob.extract::<ForeclosureCaseParty>() {
            return Ok(TransactionPartyOrSubtype::ForeclosureCaseParty(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TransactionPartyOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for TransactionPartyOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            TransactionPartyOrSubtype::TransferParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::SaleEventParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::ListingParticipant(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::LeaseEventParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::LoanParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::LienParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            TransactionPartyOrSubtype::ForeclosureCaseParty(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TransactionPartyOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TransactionPartyOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TransactionPartyOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TransactionPartyOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(TransactionPartyOrSubtype = TransferParty | SaleEventParty | ListingParticipant | LeaseEventParty | LoanParty | LienParty | ForeclosureCaseParty);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Jurisdiction {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_iso_country"))]
    pub country: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub region: Option<String>,
    pub name: String,
    pub kind: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub authority_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parent: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub boundary: Option<Geometry>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Jurisdiction {
    #[new]
    #[pyo3(signature = (country, name, kind, id, region=None, authority_code=None, parent=None, boundary=None, extras=None, provenance=None, verifications=None))]
    pub fn new(country: String, name: String, kind: String, id: String, region: Option<String>, authority_code: Option<String>, parent: Option<String>, boundary: Option<serde_utils::PyValue<Geometry>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let boundary = boundary.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Jurisdiction{country, name, kind, id, region, authority_code, parent, boundary, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Jurisdiction>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Jurisdiction> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Jurisdiction>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Jurisdiction",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Jurisdiction {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Jurisdiction from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Address {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_iso_country"))]
    pub country: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unformatted_address: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub street_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub street_pre_direction: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub street_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub street_suffix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub street_post_direction: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sublocality: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub city: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub region: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub postal_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub postal_code_suffix: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub admin_area: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub admin_area_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub address_hash: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub address_hash_scheme: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub location: Option<GeoPoint>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub location_accuracy: Option<GeocodeAccuracy>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<Box<VerificationAttribution>>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Address {
    #[new]
    #[pyo3(signature = (country, id, unformatted_address=None, street_number=None, street_pre_direction=None, street_name=None, street_suffix=None, street_post_direction=None, unit_type=None, unit_number=None, sublocality=None, city=None, region=None, postal_code=None, postal_code_suffix=None, admin_area=None, admin_area_code=None, address_hash=None, address_hash_scheme=None, location=None, location_accuracy=None, extras=None, provenance=None, verifications=None))]
    pub fn new(country: String, id: String, unformatted_address: Option<String>, street_number: Option<String>, street_pre_direction: Option<String>, street_name: Option<String>, street_suffix: Option<String>, street_post_direction: Option<String>, unit_type: Option<String>, unit_number: Option<String>, sublocality: Option<String>, city: Option<String>, region: Option<String>, postal_code: Option<String>, postal_code_suffix: Option<String>, admin_area: Option<String>, admin_area_code: Option<String>, address_hash: Option<String>, address_hash_scheme: Option<String>, location: Option<serde_utils::PyValue<GeoPoint>>, location_accuracy: Option<GeocodeAccuracy>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<Box<VerificationAttribution>>>>) -> Self {
        let location = location.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Address{country, id, unformatted_address, street_number, street_pre_direction, street_name, street_suffix, street_post_direction, unit_type, unit_number, sublocality, city, region, postal_code, postal_code_suffix, admin_area, admin_area_code, address_hash, address_hash_scheme, location, location_accuracy, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Address>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Address> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Address>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Address",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Address {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Address from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyFacts {
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_class: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_subtype: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_system: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub estate_type: Option<EstateType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub location: Option<GeoPoint>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub building_count: Option<isize>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyFacts {
    #[new]
    #[pyo3(signature = (name=None, property_use_class=None, property_use_type=None, property_use_subtype=None, property_use_system=None, estate_type=None, location=None, building_count=None))]
    pub fn new(name: Option<String>, property_use_class: Option<String>, property_use_type: Option<String>, property_use_subtype: Option<String>, property_use_system: Option<String>, estate_type: Option<EstateType>, location: Option<serde_utils::PyValue<GeoPoint>>, building_count: Option<isize>) -> Self {
        let location = location.map(|v| v.into_inner());
        PropertyFacts{name, property_use_class, property_use_type, property_use_subtype, property_use_system, estate_type, location, building_count}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyFacts>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyFacts> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyFacts>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyFacts",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum PropertyFactsOrSubtype {    Property(Property),     PropertyState(PropertyState)}

impl From<Property>   for PropertyFactsOrSubtype { fn from(x: Property)   -> Self { Self::Property(x) } }
impl From<PropertyState>   for PropertyFactsOrSubtype { fn from(x: PropertyState)   -> Self { Self::PropertyState(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for PropertyFactsOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Property>() {
            return Ok(PropertyFactsOrSubtype::Property(val));
        }        if let Ok(val) = ob.extract::<PropertyState>() {
            return Ok(PropertyFactsOrSubtype::PropertyState(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyFactsOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for PropertyFactsOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            PropertyFactsOrSubtype::Property(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            PropertyFactsOrSubtype::PropertyState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyFactsOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyFactsOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyFactsOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyFactsOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(PropertyFactsOrSubtype = Property | PropertyState);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Property {
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_class: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_subtype: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_system: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub estate_type: Option<EstateType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub location: Option<GeoPoint>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub building_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Property {
    #[new]
    #[pyo3(signature = (id, name=None, property_use_class=None, property_use_type=None, property_use_subtype=None, property_use_system=None, estate_type=None, location=None, building_count=None, extras=None, provenance=None, verifications=None))]
    pub fn new(id: String, name: Option<String>, property_use_class: Option<String>, property_use_type: Option<String>, property_use_subtype: Option<String>, property_use_system: Option<String>, estate_type: Option<EstateType>, location: Option<serde_utils::PyValue<GeoPoint>>, building_count: Option<isize>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let location = location.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Property{id, name, property_use_class, property_use_type, property_use_subtype, property_use_system, estate_type, location, building_count, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Property>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Property> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Property>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Property",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Property {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }


    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Parcel {
    pub jurisdiction: String,
    pub parcel_number: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub normalized_parcel_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_designator: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reso_upi: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub legal_description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub boundary: Option<Geometry>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub retired_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Parcel {
    #[new]
    #[pyo3(signature = (jurisdiction, parcel_number, id, normalized_parcel_number=None, unit_designator=None, reso_upi=None, legal_description=None, land_area=None, boundary=None, retired_on=None, extras=None, provenance=None, verifications=None))]
    pub fn new(jurisdiction: String, parcel_number: String, id: String, normalized_parcel_number: Option<String>, unit_designator: Option<String>, reso_upi: Option<String>, legal_description: Option<String>, land_area: Option<serde_utils::PyValue<Area>>, boundary: Option<serde_utils::PyValue<Geometry>>, retired_on: Option<NaiveDate>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let land_area = land_area.map(|v| v.into_inner());
        let boundary = boundary.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Parcel{jurisdiction, parcel_number, id, normalized_parcel_number, unit_designator, reso_upi, legal_description, land_area, boundary, retired_on, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Parcel>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Parcel> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Parcel>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Parcel",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Parcel {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Parcel from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyParcel {
    pub property: String,
    pub parcel: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_primary: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub started_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ended_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyParcel {
    #[new]
    #[pyo3(signature = (property, parcel, id, is_primary=None, started_on=None, ended_on=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, parcel: String, id: String, is_primary: Option<bool>, started_on: Option<NaiveDate>, ended_on: Option<NaiveDate>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyParcel{property, parcel, id, is_primary, started_on, ended_on, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyParcel>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyParcel> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyParcel>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyParcel",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyParcel {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyParcel from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ParcelLineage {
    pub predecessor_parcel: String,
    pub successor_parcel: String,
    pub kind: ParcelLineageKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ParcelLineage {
    #[new]
    #[pyo3(signature = (predecessor_parcel, successor_parcel, kind, id, effective_on=None, extras=None, provenance=None, verifications=None))]
    pub fn new(predecessor_parcel: String, successor_parcel: String, kind: ParcelLineageKind, id: String, effective_on: Option<NaiveDate>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        ParcelLineage{predecessor_parcel, successor_parcel, kind, id, effective_on, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ParcelLineage>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ParcelLineage> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ParcelLineage>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ParcelLineage",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for ParcelLineage {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a ParcelLineage from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyIdentifier {
    pub property: String,
    pub scheme: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub namespace: Option<String>,
    pub value: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyIdentifier {
    #[new]
    #[pyo3(signature = (property, scheme, value, id, namespace=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, scheme: String, value: String, id: String, namespace: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyIdentifier{property, scheme, value, id, namespace, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyIdentifier>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyIdentifier> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyIdentifier>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyIdentifier",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyIdentifier {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyIdentifier from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Party {
    pub kind: PartyKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub legal_form: Option<Classification>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub name: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub normalized_name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_first: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_middle: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name_last: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub classifications: Option<Vec<Classification>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub addresses: Option<Vec<Box<PartyAddress>>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub contacts: Option<Vec<PartyContact>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<Box<VerificationAttribution>>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Party {
    #[new]
    #[pyo3(signature = (kind, name, id, legal_form=None, normalized_name=None, name_first=None, name_middle=None, name_last=None, classifications=None, addresses=None, contacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(kind: PartyKind, name: String, id: String, legal_form: Option<serde_utils::PyValue<Classification>>, normalized_name: Option<String>, name_first: Option<String>, name_middle: Option<String>, name_last: Option<String>, classifications: Option<serde_utils::PyValue<Vec<Classification>>>, addresses: Option<serde_utils::PyValue<Vec<Box<PartyAddress>>>>, contacts: Option<serde_utils::PyValue<Vec<PartyContact>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<Box<VerificationAttribution>>>>) -> Self {
        let legal_form = legal_form.map(|v| v.into_inner());
        let classifications = classifications.map(|v| v.into_inner());
        let addresses = addresses.map(|v| v.into_inner());
        let contacts = contacts.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Party{kind, name, id, legal_form, normalized_name, name_first, name_middle, name_last, classifications, addresses, contacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Party>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Party> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Party>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Party",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Party {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Party from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct VerificationAttribution {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub verifier: String,
    pub verified_at: DateTime<FixedOffset>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub note: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl VerificationAttribution {
    #[new]
    #[pyo3(signature = (verifier, verified_at, note=None, extras=None))]
    pub fn new(verifier: String, verified_at: DateTime<FixedOffset>, note: Option<String>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        VerificationAttribution{verifier, verified_at, note, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<VerificationAttribution>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<VerificationAttribution> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<VerificationAttribution>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid VerificationAttribution",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SourceArtifact {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub uri: Option<uri>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub storage_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub media_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub title: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub original_filename: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub content_hash: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub hash_scheme: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub page_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub captured_on: Option<DateTime<FixedOffset>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SourceArtifact {
    #[new]
    #[pyo3(signature = (id, uri=None, storage_reference=None, media_type=None, kind=None, title=None, original_filename=None, content_hash=None, hash_scheme=None, page_count=None, captured_on=None, extras=None, provenance=None, verifications=None))]
    pub fn new(id: String, uri: Option<uri>, storage_reference: Option<String>, media_type: Option<String>, kind: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, title: Option<String>, original_filename: Option<String>, content_hash: Option<String>, hash_scheme: Option<String>, page_count: Option<isize>, captured_on: Option<DateTime<FixedOffset>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let kind = kind.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        SourceArtifact{id, uri, storage_reference, media_type, kind, title, original_filename, content_hash, hash_scheme, page_count, captured_on, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SourceArtifact>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SourceArtifact> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SourceArtifact>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SourceArtifact",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SourceArtifact {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        let mut map:  BTreeMap<Value, Value> = BTreeMap::new();
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }


    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AddressAssociation {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub address: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_primary: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_from: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_to: Option<NaiveDate>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl AddressAssociation {
    #[new]
    #[pyo3(signature = (address, role=None, is_primary=None, valid_from=None, valid_to=None))]
    pub fn new(address: String, role: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, is_primary: Option<bool>, valid_from: Option<NaiveDate>, valid_to: Option<NaiveDate>) -> Self {
        let role = role.map(|v| v.into_inner());
        AddressAssociation{address, role, is_primary, valid_from, valid_to}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AddressAssociation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AddressAssociation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AddressAssociation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AddressAssociation",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum AddressAssociationOrSubtype {    PartyAddress(PartyAddress),     PropertyAddress(PropertyAddress)}

impl From<PartyAddress>   for AddressAssociationOrSubtype { fn from(x: PartyAddress)   -> Self { Self::PartyAddress(x) } }
impl From<PropertyAddress>   for AddressAssociationOrSubtype { fn from(x: PropertyAddress)   -> Self { Self::PropertyAddress(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for AddressAssociationOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PartyAddress>() {
            return Ok(AddressAssociationOrSubtype::PartyAddress(val));
        }        if let Ok(val) = ob.extract::<PropertyAddress>() {
            return Ok(AddressAssociationOrSubtype::PropertyAddress(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AddressAssociationOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for AddressAssociationOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            AddressAssociationOrSubtype::PartyAddress(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            AddressAssociationOrSubtype::PropertyAddress(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AddressAssociationOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AddressAssociationOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AddressAssociationOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AddressAssociationOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(AddressAssociationOrSubtype = PartyAddress | PropertyAddress);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PartyAddress {
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub address: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_primary: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_from: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_to: Option<NaiveDate>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PartyAddress {
    #[new]
    #[pyo3(signature = (address, extras=None, provenance=None, role=None, is_primary=None, valid_from=None, valid_to=None))]
    pub fn new(address: String, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, role: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, is_primary: Option<bool>, valid_from: Option<NaiveDate>, valid_to: Option<NaiveDate>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let role = role.map(|v| v.into_inner());
        PartyAddress{address, extras, provenance, role, is_primary, valid_from, valid_to}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PartyAddress>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PartyAddress> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PartyAddress>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PartyAddress",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyAddress {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub address: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_primary: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_from: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valid_to: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyAddress {
    #[new]
    #[pyo3(signature = (property, address, id, role=None, is_primary=None, valid_from=None, valid_to=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, address: String, id: String, role: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, is_primary: Option<bool>, valid_from: Option<NaiveDate>, valid_to: Option<NaiveDate>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let role = role.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyAddress{property, address, id, role, is_primary, valid_from, valid_to, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyAddress>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyAddress> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyAddress>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyAddress",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyAddress {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyAddress from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PartyContact {
    pub kind: String,
    pub value: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub label: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_primary: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub do_not_contact: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PartyContact {
    #[new]
    #[pyo3(signature = (kind, value, label=None, is_primary=None, do_not_contact=None, extras=None, provenance=None))]
    pub fn new(kind: String, value: String, label: Option<String>, is_primary: Option<bool>, do_not_contact: Option<bool>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        PartyContact{kind, value, label, is_primary, do_not_contact, extras, provenance}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PartyContact>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PartyContact> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PartyContact>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PartyContact",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct OwnershipPeriod {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub started_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ended_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub vesting_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub mailing_address: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub acquired_via_transfer: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub disposed_via_transfer: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interests: Option<Vec<OwnershipInterest>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl OwnershipPeriod {
    #[new]
    #[pyo3(signature = (property, id, started_on=None, ended_on=None, vesting_type=None, mailing_address=None, acquired_via_transfer=None, disposed_via_transfer=None, interests=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, started_on: Option<NaiveDate>, ended_on: Option<NaiveDate>, vesting_type: Option<String>, mailing_address: Option<String>, acquired_via_transfer: Option<String>, disposed_via_transfer: Option<String>, interests: Option<serde_utils::PyValue<Vec<OwnershipInterest>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let interests = interests.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        OwnershipPeriod{property, id, started_on, ended_on, vesting_type, mailing_address, acquired_via_transfer, disposed_via_transfer, interests, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<OwnershipPeriod>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<OwnershipPeriod> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<OwnershipPeriod>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid OwnershipPeriod",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for OwnershipPeriod {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a OwnershipPeriod from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct OwnershipInterest {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interest_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub role: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_owner_occupied: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl OwnershipInterest {
    #[new]
    #[pyo3(signature = (party, interest_pct=None, role=None, is_owner_occupied=None, extras=None))]
    pub fn new(party: String, interest_pct: Option<f64>, role: Option<String>, is_owner_occupied: Option<bool>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        OwnershipInterest{party, interest_pct, role, is_owner_occupied, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<OwnershipInterest>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<OwnershipInterest> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<OwnershipInterest>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid OwnershipInterest",
        ))
    }
}



pub mod structure_facts_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum condition_ratings_range {
        Rating(Rating),
        Uad36ConditionRating(Uad36ConditionRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for condition_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(condition_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
                return Ok(condition_ratings_range::Uad36ConditionRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for condition_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                condition_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                condition_ratings_range::Uad36ConditionRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<condition_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<condition_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<condition_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(condition_ratings_range = Rating | Uad36ConditionRating);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum quality_ratings_range {
        Rating(Rating),
        Uad36QualityRating(Uad36QualityRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for quality_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(quality_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36QualityRating>() {
                return Ok(quality_ratings_range::Uad36QualityRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for quality_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                quality_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                quality_ratings_range::Uad36QualityRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<quality_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<quality_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<quality_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(quality_ratings_range = Rating | Uad36QualityRating);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct StructureFacts {
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub living_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub gross_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_floor_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_finished_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub areas: Option<Vec<AreaMeasure>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built_estimated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stories: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exterior_wall_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_material_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_style_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_material: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub condition_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub quality_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_fuel_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub cooling_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sewer_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub water_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub residential: Option<ResidentialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commercial: Option<CommercialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub renovations: Option<Vec<Renovation>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl StructureFacts {
    #[new]
    #[pyo3(signature = (kind=None, name=None, structure_number=None, living_area=None, gross_area=None, rentable_area=None, ground_floor_area=None, basement_area=None, basement_finished_area=None, garage_area=None, areas=None, year_built=None, year_built_estimated=None, effective_year_built=None, stories=None, unit_count=None, construction_method=None, construction_status=None, construction_type=None, exterior_wall_type=None, roof_material_type=None, roof_style_type=None, foundation_type=None, foundation_material=None, condition_ratings=None, quality_ratings=None, heating_types=None, heating_fuel_type=None, cooling_types=None, sewer_type=None, water_type=None, features=None, residential=None, commercial=None, renovations=None))]
    pub fn new(kind: Option<String>, name: Option<String>, structure_number: Option<String>, living_area: Option<serde_utils::PyValue<Area>>, gross_area: Option<serde_utils::PyValue<Area>>, rentable_area: Option<serde_utils::PyValue<Area>>, ground_floor_area: Option<serde_utils::PyValue<Area>>, basement_area: Option<serde_utils::PyValue<Area>>, basement_finished_area: Option<serde_utils::PyValue<Area>>, garage_area: Option<serde_utils::PyValue<Area>>, areas: Option<serde_utils::PyValue<Vec<AreaMeasure>>>, year_built: Option<isize>, year_built_estimated: Option<bool>, effective_year_built: Option<isize>, stories: Option<f64>, unit_count: Option<isize>, construction_method: Option<String>, construction_status: Option<String>, construction_type: Option<String>, exterior_wall_type: Option<String>, roof_material_type: Option<String>, roof_style_type: Option<String>, foundation_type: Option<String>, foundation_material: Option<String>, condition_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, quality_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, heating_types: Option<Vec<String>>, heating_fuel_type: Option<String>, cooling_types: Option<Vec<String>>, sewer_type: Option<String>, water_type: Option<String>, features: Option<Vec<String>>, residential: Option<serde_utils::PyValue<ResidentialDetails>>, commercial: Option<serde_utils::PyValue<CommercialDetails>>, renovations: Option<serde_utils::PyValue<Vec<Renovation>>>) -> Self {
        let living_area = living_area.map(|v| v.into_inner());
        let gross_area = gross_area.map(|v| v.into_inner());
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let ground_floor_area = ground_floor_area.map(|v| v.into_inner());
        let basement_area = basement_area.map(|v| v.into_inner());
        let basement_finished_area = basement_finished_area.map(|v| v.into_inner());
        let garage_area = garage_area.map(|v| v.into_inner());
        let areas = areas.map(|v| v.into_inner());
        let condition_ratings = condition_ratings.map(|v| v.into_inner());
        let quality_ratings = quality_ratings.map(|v| v.into_inner());
        let residential = residential.map(|v| v.into_inner());
        let commercial = commercial.map(|v| v.into_inner());
        let renovations = renovations.map(|v| v.into_inner());
        StructureFacts{kind, name, structure_number, living_area, gross_area, rentable_area, ground_floor_area, basement_area, basement_finished_area, garage_area, areas, year_built, year_built_estimated, effective_year_built, stories, unit_count, construction_method, construction_status, construction_type, exterior_wall_type, roof_material_type, roof_style_type, foundation_type, foundation_material, condition_ratings, quality_ratings, heating_types, heating_fuel_type, cooling_types, sewer_type, water_type, features, residential, commercial, renovations}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructureFacts>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructureFacts> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructureFacts>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureFacts",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum StructureFactsOrSubtype {    Structure(Structure),     StructureState(StructureState),     Uad36StructureState(Uad36StructureState),     Uad36Structure(Uad36Structure)}

impl From<Structure>   for StructureFactsOrSubtype { fn from(x: Structure)   -> Self { Self::Structure(x) } }
impl From<StructureState>   for StructureFactsOrSubtype { fn from(x: StructureState)   -> Self { Self::StructureState(x) } }
impl From<Uad36StructureState>   for StructureFactsOrSubtype { fn from(x: Uad36StructureState)   -> Self { Self::Uad36StructureState(x) } }
impl From<Uad36Structure>   for StructureFactsOrSubtype { fn from(x: Uad36Structure)   -> Self { Self::Uad36Structure(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for StructureFactsOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Structure>() {
            return Ok(StructureFactsOrSubtype::Structure(val));
        }        if let Ok(val) = ob.extract::<StructureState>() {
            return Ok(StructureFactsOrSubtype::StructureState(val));
        }        if let Ok(val) = ob.extract::<Uad36StructureState>() {
            return Ok(StructureFactsOrSubtype::Uad36StructureState(val));
        }        if let Ok(val) = ob.extract::<Uad36Structure>() {
            return Ok(StructureFactsOrSubtype::Uad36Structure(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureFactsOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for StructureFactsOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            StructureFactsOrSubtype::Structure(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            StructureFactsOrSubtype::StructureState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            StructureFactsOrSubtype::Uad36StructureState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            StructureFactsOrSubtype::Uad36Structure(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructureFactsOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructureFactsOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructureFactsOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureFactsOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(StructureFactsOrSubtype = Structure | StructureState | Uad36StructureState | Uad36Structure);

pub mod structure_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum condition_ratings_range {
        Rating(Rating),
        Uad36ConditionRating(Uad36ConditionRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for condition_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(condition_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
                return Ok(condition_ratings_range::Uad36ConditionRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for condition_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                condition_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                condition_ratings_range::Uad36ConditionRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<condition_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<condition_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<condition_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(condition_ratings_range = Rating | Uad36ConditionRating);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum quality_ratings_range {
        Rating(Rating),
        Uad36QualityRating(Uad36QualityRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for quality_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(quality_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36QualityRating>() {
                return Ok(quality_ratings_range::Uad36QualityRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for quality_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                quality_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                quality_ratings_range::Uad36QualityRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<quality_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<quality_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<quality_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(quality_ratings_range = Rating | Uad36QualityRating);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Structure {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub living_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub gross_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_floor_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_finished_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub areas: Option<Vec<AreaMeasure>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built_estimated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stories: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exterior_wall_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_material_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_style_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_material: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub condition_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub quality_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_fuel_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub cooling_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sewer_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub water_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub residential: Option<ResidentialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commercial: Option<CommercialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub renovations: Option<Vec<Renovation>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Structure {
    #[new]
    #[pyo3(signature = (property, id, kind=None, name=None, structure_number=None, living_area=None, gross_area=None, rentable_area=None, ground_floor_area=None, basement_area=None, basement_finished_area=None, garage_area=None, areas=None, year_built=None, year_built_estimated=None, effective_year_built=None, stories=None, unit_count=None, construction_method=None, construction_status=None, construction_type=None, exterior_wall_type=None, roof_material_type=None, roof_style_type=None, foundation_type=None, foundation_material=None, condition_ratings=None, quality_ratings=None, heating_types=None, heating_fuel_type=None, cooling_types=None, sewer_type=None, water_type=None, features=None, residential=None, commercial=None, renovations=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, kind: Option<String>, name: Option<String>, structure_number: Option<String>, living_area: Option<serde_utils::PyValue<Area>>, gross_area: Option<serde_utils::PyValue<Area>>, rentable_area: Option<serde_utils::PyValue<Area>>, ground_floor_area: Option<serde_utils::PyValue<Area>>, basement_area: Option<serde_utils::PyValue<Area>>, basement_finished_area: Option<serde_utils::PyValue<Area>>, garage_area: Option<serde_utils::PyValue<Area>>, areas: Option<serde_utils::PyValue<Vec<AreaMeasure>>>, year_built: Option<isize>, year_built_estimated: Option<bool>, effective_year_built: Option<isize>, stories: Option<f64>, unit_count: Option<isize>, construction_method: Option<String>, construction_status: Option<String>, construction_type: Option<String>, exterior_wall_type: Option<String>, roof_material_type: Option<String>, roof_style_type: Option<String>, foundation_type: Option<String>, foundation_material: Option<String>, condition_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, quality_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, heating_types: Option<Vec<String>>, heating_fuel_type: Option<String>, cooling_types: Option<Vec<String>>, sewer_type: Option<String>, water_type: Option<String>, features: Option<Vec<String>>, residential: Option<serde_utils::PyValue<ResidentialDetails>>, commercial: Option<serde_utils::PyValue<CommercialDetails>>, renovations: Option<serde_utils::PyValue<Vec<Renovation>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let living_area = living_area.map(|v| v.into_inner());
        let gross_area = gross_area.map(|v| v.into_inner());
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let ground_floor_area = ground_floor_area.map(|v| v.into_inner());
        let basement_area = basement_area.map(|v| v.into_inner());
        let basement_finished_area = basement_finished_area.map(|v| v.into_inner());
        let garage_area = garage_area.map(|v| v.into_inner());
        let areas = areas.map(|v| v.into_inner());
        let condition_ratings = condition_ratings.map(|v| v.into_inner());
        let quality_ratings = quality_ratings.map(|v| v.into_inner());
        let residential = residential.map(|v| v.into_inner());
        let commercial = commercial.map(|v| v.into_inner());
        let renovations = renovations.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Structure{property, id, kind, name, structure_number, living_area, gross_area, rentable_area, ground_floor_area, basement_area, basement_finished_area, garage_area, areas, year_built, year_built_estimated, effective_year_built, stories, unit_count, construction_method, construction_status, construction_type, exterior_wall_type, roof_material_type, roof_style_type, foundation_type, foundation_material, condition_ratings, quality_ratings, heating_types, heating_fuel_type, cooling_types, sewer_type, water_type, features, residential, commercial, renovations, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Structure>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Structure> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Structure>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Structure",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Structure {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Structure from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum StructureOrSubtype {    Uad36Structure(Uad36Structure)}

impl From<Uad36Structure>   for StructureOrSubtype { fn from(x: Uad36Structure)   -> Self { Self::Uad36Structure(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for StructureOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36Structure>() {
            return Ok(StructureOrSubtype::Uad36Structure(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for StructureOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            StructureOrSubtype::Uad36Structure(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructureOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructureOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructureOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for StructureOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36Structure::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(StructureOrSubtype::Uad36Structure(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36Structure::from_pair_simple(k.clone(), v.clone()) {
            return Ok(StructureOrSubtype::Uad36Structure(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            StructureOrSubtype::Uad36Structure(inner) => inner.extract_key(),
        }
    }
}

#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(StructureOrSubtype = Uad36Structure);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36Structure {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub living_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub gross_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_floor_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_finished_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub areas: Option<Vec<AreaMeasure>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built_estimated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stories: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exterior_wall_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_material_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_style_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_material: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub condition_ratings: Option<Vec<Uad36ConditionRating>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub quality_ratings: Option<Vec<Uad36QualityRating>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_fuel_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub cooling_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sewer_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub water_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub residential: Option<ResidentialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commercial: Option<CommercialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub renovations: Option<Vec<Renovation>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36Structure {
    #[new]
    #[pyo3(signature = (property, id, kind=None, name=None, structure_number=None, living_area=None, gross_area=None, rentable_area=None, ground_floor_area=None, basement_area=None, basement_finished_area=None, garage_area=None, areas=None, year_built=None, year_built_estimated=None, effective_year_built=None, stories=None, unit_count=None, construction_method=None, construction_status=None, construction_type=None, exterior_wall_type=None, roof_material_type=None, roof_style_type=None, foundation_type=None, foundation_material=None, condition_ratings=None, quality_ratings=None, heating_types=None, heating_fuel_type=None, cooling_types=None, sewer_type=None, water_type=None, features=None, residential=None, commercial=None, renovations=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, kind: Option<String>, name: Option<String>, structure_number: Option<String>, living_area: Option<serde_utils::PyValue<Area>>, gross_area: Option<serde_utils::PyValue<Area>>, rentable_area: Option<serde_utils::PyValue<Area>>, ground_floor_area: Option<serde_utils::PyValue<Area>>, basement_area: Option<serde_utils::PyValue<Area>>, basement_finished_area: Option<serde_utils::PyValue<Area>>, garage_area: Option<serde_utils::PyValue<Area>>, areas: Option<serde_utils::PyValue<Vec<AreaMeasure>>>, year_built: Option<isize>, year_built_estimated: Option<bool>, effective_year_built: Option<isize>, stories: Option<f64>, unit_count: Option<isize>, construction_method: Option<String>, construction_status: Option<String>, construction_type: Option<String>, exterior_wall_type: Option<String>, roof_material_type: Option<String>, roof_style_type: Option<String>, foundation_type: Option<String>, foundation_material: Option<String>, condition_ratings: Option<serde_utils::PyValue<Vec<Uad36ConditionRating>>>, quality_ratings: Option<serde_utils::PyValue<Vec<Uad36QualityRating>>>, heating_types: Option<Vec<String>>, heating_fuel_type: Option<String>, cooling_types: Option<Vec<String>>, sewer_type: Option<String>, water_type: Option<String>, features: Option<Vec<String>>, residential: Option<serde_utils::PyValue<ResidentialDetails>>, commercial: Option<serde_utils::PyValue<CommercialDetails>>, renovations: Option<serde_utils::PyValue<Vec<Renovation>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let living_area = living_area.map(|v| v.into_inner());
        let gross_area = gross_area.map(|v| v.into_inner());
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let ground_floor_area = ground_floor_area.map(|v| v.into_inner());
        let basement_area = basement_area.map(|v| v.into_inner());
        let basement_finished_area = basement_finished_area.map(|v| v.into_inner());
        let garage_area = garage_area.map(|v| v.into_inner());
        let areas = areas.map(|v| v.into_inner());
        let condition_ratings = condition_ratings.map(|v| v.into_inner());
        let quality_ratings = quality_ratings.map(|v| v.into_inner());
        let residential = residential.map(|v| v.into_inner());
        let commercial = commercial.map(|v| v.into_inner());
        let renovations = renovations.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Uad36Structure{property, id, kind, name, structure_number, living_area, gross_area, rentable_area, ground_floor_area, basement_area, basement_finished_area, garage_area, areas, year_built, year_built_estimated, effective_year_built, stories, unit_count, construction_method, construction_status, construction_type, exterior_wall_type, roof_material_type, roof_style_type, foundation_type, foundation_material, condition_ratings, quality_ratings, heating_types, heating_fuel_type, cooling_types, sewer_type, water_type, features, residential, commercial, renovations, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36Structure>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36Structure> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36Structure>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36Structure",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Uad36Structure {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Uad36Structure from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AreaMeasure {
    pub kind: String,
    pub area: Area,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl AreaMeasure {
    #[new]
    #[pyo3(signature = (kind, area, extras=None))]
    pub fn new(kind: String, area: serde_utils::PyValue<Area>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let area = area.into_inner();
        let extras = extras.map(|v| v.into_inner());
        AreaMeasure{kind, area, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AreaMeasure>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AreaMeasure> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AreaMeasure>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AreaMeasure",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ResidentialDetails {
    #[cfg_attr(feature = "serde", serde(default))]
    pub bedrooms_total: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms_full: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms_half: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms_three_quarter: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rooms_total: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub attachment: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub architectural_design: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_attachment: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parking_spaces: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fireplaces: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_pool: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_attic: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_adu: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub adu_legally_rentable: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub renewable_energy_components: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ResidentialDetails {
    #[new]
    #[pyo3(signature = (bedrooms_total=None, bathrooms_full=None, bathrooms_half=None, bathrooms_three_quarter=None, rooms_total=None, attachment=None, architectural_design=None, basement_type=None, garage_type=None, garage_attachment=None, parking_spaces=None, fireplaces=None, has_pool=None, has_attic=None, has_adu=None, adu_legally_rentable=None, occupancy=None, renewable_energy_components=None, extras=None, provenance=None))]
    pub fn new(bedrooms_total: Option<isize>, bathrooms_full: Option<isize>, bathrooms_half: Option<isize>, bathrooms_three_quarter: Option<isize>, rooms_total: Option<isize>, attachment: Option<String>, architectural_design: Option<String>, basement_type: Option<String>, garage_type: Option<String>, garage_attachment: Option<String>, parking_spaces: Option<isize>, fireplaces: Option<isize>, has_pool: Option<bool>, has_attic: Option<bool>, has_adu: Option<bool>, adu_legally_rentable: Option<bool>, occupancy: Option<String>, renewable_energy_components: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        ResidentialDetails{bedrooms_total, bathrooms_full, bathrooms_half, bathrooms_three_quarter, rooms_total, attachment, architectural_design, basement_type, garage_type, garage_attachment, parking_spaces, fireplaces, has_pool, has_attic, has_adu, adu_legally_rentable, occupancy, renewable_energy_components, extras, provenance}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ResidentialDetails>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ResidentialDetails> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ResidentialDetails>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ResidentialDetails",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct CommercialDetails {
    #[cfg_attr(feature = "serde", serde(default))]
    pub market_classification: Option<RatingOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub clear_height: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub dock_doors: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub drive_in_doors: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parking_spaces: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parking_ratio: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenancy: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenant_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parking_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub has_sprinkler: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub elevators: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub submarket: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl CommercialDetails {
    #[new]
    #[pyo3(signature = (market_classification=None, clear_height=None, dock_doors=None, drive_in_doors=None, occupancy_pct=None, parking_spaces=None, parking_ratio=None, tenancy=None, tenant_count=None, parking_types=None, has_sprinkler=None, elevators=None, submarket=None, extras=None, provenance=None))]
    pub fn new(market_classification: Option<serde_utils::PyValue<RatingOrSubtype>>, clear_height: Option<serde_utils::PyValue<Length>>, dock_doors: Option<isize>, drive_in_doors: Option<isize>, occupancy_pct: Option<f64>, parking_spaces: Option<isize>, parking_ratio: Option<serde_utils::PyValue<UnitRate>>, tenancy: Option<String>, tenant_count: Option<isize>, parking_types: Option<Vec<String>>, has_sprinkler: Option<bool>, elevators: Option<isize>, submarket: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>) -> Self {
        let market_classification = market_classification.map(|v| v.into_inner());
        let clear_height = clear_height.map(|v| v.into_inner());
        let parking_ratio = parking_ratio.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        CommercialDetails{market_classification, clear_height, dock_doors, drive_in_doors, occupancy_pct, parking_spaces, parking_ratio, tenancy, tenant_count, parking_types, has_sprinkler, elevators, submarket, extras, provenance}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<CommercialDetails>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<CommercialDetails> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<CommercialDetails>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid CommercialDetails",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Renovation {
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub completed_year: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub completed_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cost: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Renovation {
    #[new]
    #[pyo3(signature = (kind=None, description=None, completed_year=None, completed_on=None, cost=None, extras=None, provenance=None))]
    pub fn new(kind: Option<String>, description: Option<String>, completed_year: Option<isize>, completed_on: Option<NaiveDate>, cost: Option<serde_utils::PyValue<Money>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>) -> Self {
        let cost = cost.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        Renovation{kind, description, completed_year, completed_on, cost, extras, provenance}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Renovation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Renovation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Renovation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Renovation",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SiteFacts {
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_size: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area_basis: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use_category: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub zoning_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub flood_zone: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub hazard_zones: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub view_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub site_influences: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub easements: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub restrictions: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub utilities: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub frontage: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub topography: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_corner: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub entitlement_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub buildable_units: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subdivision: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub block: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tract_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub phase_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub section_township_range: Option<String>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SiteFacts {
    #[new]
    #[pyo3(signature = (lot_size=None, usable_land_area=None, usable_land_area_basis=None, land_use=None, land_use_category=None, zoning_code=None, flood_zone=None, hazard_zones=None, view_types=None, site_influences=None, easements=None, restrictions=None, utilities=None, frontage=None, depth=None, topography=None, is_corner=None, entitlement_status=None, buildable_units=None, subdivision=None, lot_number=None, block=None, tract_number=None, phase_number=None, section_township_range=None))]
    pub fn new(lot_size: Option<serde_utils::PyValue<Area>>, usable_land_area: Option<serde_utils::PyValue<Area>>, usable_land_area_basis: Option<String>, land_use: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, land_use_category: Option<String>, zoning_code: Option<String>, flood_zone: Option<String>, hazard_zones: Option<Vec<String>>, view_types: Option<Vec<String>>, site_influences: Option<Vec<String>>, easements: Option<Vec<String>>, restrictions: Option<Vec<String>>, utilities: Option<Vec<String>>, frontage: Option<serde_utils::PyValue<Length>>, depth: Option<serde_utils::PyValue<Length>>, topography: Option<String>, is_corner: Option<bool>, entitlement_status: Option<String>, buildable_units: Option<isize>, subdivision: Option<String>, lot_number: Option<String>, block: Option<String>, tract_number: Option<String>, phase_number: Option<String>, section_township_range: Option<String>) -> Self {
        let lot_size = lot_size.map(|v| v.into_inner());
        let usable_land_area = usable_land_area.map(|v| v.into_inner());
        let land_use = land_use.map(|v| v.into_inner());
        let frontage = frontage.map(|v| v.into_inner());
        let depth = depth.map(|v| v.into_inner());
        SiteFacts{lot_size, usable_land_area, usable_land_area_basis, land_use, land_use_category, zoning_code, flood_zone, hazard_zones, view_types, site_influences, easements, restrictions, utilities, frontage, depth, topography, is_corner, entitlement_status, buildable_units, subdivision, lot_number, block, tract_number, phase_number, section_township_range}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SiteFacts>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SiteFacts> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SiteFacts>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SiteFacts",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum SiteFactsOrSubtype {    Site(Site),     SiteState(SiteState)}

impl From<Site>   for SiteFactsOrSubtype { fn from(x: Site)   -> Self { Self::Site(x) } }
impl From<SiteState>   for SiteFactsOrSubtype { fn from(x: SiteState)   -> Self { Self::SiteState(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for SiteFactsOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Site>() {
            return Ok(SiteFactsOrSubtype::Site(val));
        }        if let Ok(val) = ob.extract::<SiteState>() {
            return Ok(SiteFactsOrSubtype::SiteState(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SiteFactsOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for SiteFactsOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SiteFactsOrSubtype::Site(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            SiteFactsOrSubtype::SiteState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SiteFactsOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SiteFactsOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SiteFactsOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SiteFactsOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(SiteFactsOrSubtype = Site | SiteState);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Site {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_size: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area_basis: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use_category: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub zoning_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub flood_zone: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub hazard_zones: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub view_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub site_influences: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub easements: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub restrictions: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub utilities: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub frontage: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub topography: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_corner: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub entitlement_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub buildable_units: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subdivision: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub block: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tract_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub phase_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub section_township_range: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Site {
    #[new]
    #[pyo3(signature = (property, id, lot_size=None, usable_land_area=None, usable_land_area_basis=None, land_use=None, land_use_category=None, zoning_code=None, flood_zone=None, hazard_zones=None, view_types=None, site_influences=None, easements=None, restrictions=None, utilities=None, frontage=None, depth=None, topography=None, is_corner=None, entitlement_status=None, buildable_units=None, subdivision=None, lot_number=None, block=None, tract_number=None, phase_number=None, section_township_range=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, lot_size: Option<serde_utils::PyValue<Area>>, usable_land_area: Option<serde_utils::PyValue<Area>>, usable_land_area_basis: Option<String>, land_use: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, land_use_category: Option<String>, zoning_code: Option<String>, flood_zone: Option<String>, hazard_zones: Option<Vec<String>>, view_types: Option<Vec<String>>, site_influences: Option<Vec<String>>, easements: Option<Vec<String>>, restrictions: Option<Vec<String>>, utilities: Option<Vec<String>>, frontage: Option<serde_utils::PyValue<Length>>, depth: Option<serde_utils::PyValue<Length>>, topography: Option<String>, is_corner: Option<bool>, entitlement_status: Option<String>, buildable_units: Option<isize>, subdivision: Option<String>, lot_number: Option<String>, block: Option<String>, tract_number: Option<String>, phase_number: Option<String>, section_township_range: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let lot_size = lot_size.map(|v| v.into_inner());
        let usable_land_area = usable_land_area.map(|v| v.into_inner());
        let land_use = land_use.map(|v| v.into_inner());
        let frontage = frontage.map(|v| v.into_inner());
        let depth = depth.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Site{property, id, lot_size, usable_land_area, usable_land_area_basis, land_use, land_use_category, zoning_code, flood_zone, hazard_zones, view_types, site_influences, easements, restrictions, utilities, frontage, depth, topography, is_corner, entitlement_status, buildable_units, subdivision, lot_number, block, tract_number, phase_number, section_township_range, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Site>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Site> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Site>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Site",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Site {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Site from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SpaceFacts {
    #[cfg_attr(feature = "serde", serde(default))]
    pub floor_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub space_use: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bedrooms: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_adu: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_active: Option<bool>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SpaceFacts {
    #[new]
    #[pyo3(signature = (floor_number=None, space_use=None, rentable_area=None, usable_area=None, bedrooms=None, bathrooms=None, occupancy=None, is_adu=None, is_active=None))]
    pub fn new(floor_number: Option<isize>, space_use: Option<String>, rentable_area: Option<serde_utils::PyValue<Area>>, usable_area: Option<serde_utils::PyValue<Area>>, bedrooms: Option<isize>, bathrooms: Option<f64>, occupancy: Option<String>, is_adu: Option<bool>, is_active: Option<bool>) -> Self {
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let usable_area = usable_area.map(|v| v.into_inner());
        SpaceFacts{floor_number, space_use, rentable_area, usable_area, bedrooms, bathrooms, occupancy, is_adu, is_active}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SpaceFacts>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SpaceFacts> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SpaceFacts>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SpaceFacts",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum SpaceFactsOrSubtype {    Space(Space),     SpaceState(SpaceState)}

impl From<Space>   for SpaceFactsOrSubtype { fn from(x: Space)   -> Self { Self::Space(x) } }
impl From<SpaceState>   for SpaceFactsOrSubtype { fn from(x: SpaceState)   -> Self { Self::SpaceState(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for SpaceFactsOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Space>() {
            return Ok(SpaceFactsOrSubtype::Space(val));
        }        if let Ok(val) = ob.extract::<SpaceState>() {
            return Ok(SpaceFactsOrSubtype::SpaceState(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SpaceFactsOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for SpaceFactsOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            SpaceFactsOrSubtype::Space(val) => val.into_pyobject(py).map(move |b| b.into_any()),
            SpaceFactsOrSubtype::SpaceState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SpaceFactsOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SpaceFactsOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SpaceFactsOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SpaceFactsOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(SpaceFactsOrSubtype = Space | SpaceState);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Space {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure: Option<String>,
    pub space_identifier: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub floor_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub space_use: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bedrooms: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_adu: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_active: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Space {
    #[new]
    #[pyo3(signature = (property, space_identifier, id, structure=None, floor_number=None, space_use=None, rentable_area=None, usable_area=None, bedrooms=None, bathrooms=None, occupancy=None, is_adu=None, is_active=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, space_identifier: String, id: String, structure: Option<String>, floor_number: Option<isize>, space_use: Option<String>, rentable_area: Option<serde_utils::PyValue<Area>>, usable_area: Option<serde_utils::PyValue<Area>>, bedrooms: Option<isize>, bathrooms: Option<f64>, occupancy: Option<String>, is_adu: Option<bool>, is_active: Option<bool>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let usable_area = usable_area.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Space{property, space_identifier, id, structure, floor_number, space_use, rentable_area, usable_area, bedrooms, bathrooms, occupancy, is_adu, is_active, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Space>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Space> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Space>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Space",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Space {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Space from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyState {
    pub subject: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_class: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_subtype: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_use_system: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub estate_type: Option<EstateType>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub location: Option<GeoPoint>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub building_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyState {
    #[new]
    #[pyo3(signature = (subject, id, name=None, property_use_class=None, property_use_type=None, property_use_subtype=None, property_use_system=None, estate_type=None, location=None, building_count=None, extras=None, provenance=None, verifications=None))]
    pub fn new(subject: String, id: String, name: Option<String>, property_use_class: Option<String>, property_use_type: Option<String>, property_use_subtype: Option<String>, property_use_system: Option<String>, estate_type: Option<EstateType>, location: Option<serde_utils::PyValue<GeoPoint>>, building_count: Option<isize>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let location = location.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyState{subject, id, name, property_use_class, property_use_type, property_use_subtype, property_use_system, estate_type, location, building_count, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyState>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyState> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyState>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyState",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyState {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyState from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SiteState {
    pub subject: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_size: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_land_area_basis: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_use_category: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub zoning_code: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub flood_zone: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub hazard_zones: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub view_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub site_influences: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub easements: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub restrictions: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub utilities: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub frontage: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub depth: Option<Length>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub topography: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_corner: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub entitlement_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub buildable_units: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub subdivision: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lot_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub block: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub tract_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub phase_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub section_township_range: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SiteState {
    #[new]
    #[pyo3(signature = (subject, id, lot_size=None, usable_land_area=None, usable_land_area_basis=None, land_use=None, land_use_category=None, zoning_code=None, flood_zone=None, hazard_zones=None, view_types=None, site_influences=None, easements=None, restrictions=None, utilities=None, frontage=None, depth=None, topography=None, is_corner=None, entitlement_status=None, buildable_units=None, subdivision=None, lot_number=None, block=None, tract_number=None, phase_number=None, section_township_range=None, extras=None, provenance=None, verifications=None))]
    pub fn new(subject: String, id: String, lot_size: Option<serde_utils::PyValue<Area>>, usable_land_area: Option<serde_utils::PyValue<Area>>, usable_land_area_basis: Option<String>, land_use: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, land_use_category: Option<String>, zoning_code: Option<String>, flood_zone: Option<String>, hazard_zones: Option<Vec<String>>, view_types: Option<Vec<String>>, site_influences: Option<Vec<String>>, easements: Option<Vec<String>>, restrictions: Option<Vec<String>>, utilities: Option<Vec<String>>, frontage: Option<serde_utils::PyValue<Length>>, depth: Option<serde_utils::PyValue<Length>>, topography: Option<String>, is_corner: Option<bool>, entitlement_status: Option<String>, buildable_units: Option<isize>, subdivision: Option<String>, lot_number: Option<String>, block: Option<String>, tract_number: Option<String>, phase_number: Option<String>, section_township_range: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let lot_size = lot_size.map(|v| v.into_inner());
        let usable_land_area = usable_land_area.map(|v| v.into_inner());
        let land_use = land_use.map(|v| v.into_inner());
        let frontage = frontage.map(|v| v.into_inner());
        let depth = depth.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        SiteState{subject, id, lot_size, usable_land_area, usable_land_area_basis, land_use, land_use_category, zoning_code, flood_zone, hazard_zones, view_types, site_influences, easements, restrictions, utilities, frontage, depth, topography, is_corner, entitlement_status, buildable_units, subdivision, lot_number, block, tract_number, phase_number, section_township_range, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SiteState>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SiteState> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SiteState>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SiteState",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SiteState {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a SiteState from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

pub mod structure_state_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum condition_ratings_range {
        Rating(Rating),
        Uad36ConditionRating(Uad36ConditionRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for condition_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(condition_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36ConditionRating>() {
                return Ok(condition_ratings_range::Uad36ConditionRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for condition_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                condition_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                condition_ratings_range::Uad36ConditionRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<condition_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<condition_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<condition_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid condition_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(condition_ratings_range = Rating | Uad36ConditionRating);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum quality_ratings_range {
        Rating(Rating),
        Uad36QualityRating(Uad36QualityRating)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for quality_ratings_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Rating>() {
                return Ok(quality_ratings_range::Rating(val));
            }            if let Ok(val) = ob.extract::<Uad36QualityRating>() {
                return Ok(quality_ratings_range::Uad36QualityRating(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for quality_ratings_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                quality_ratings_range::Rating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                quality_ratings_range::Uad36QualityRating(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<quality_ratings_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<quality_ratings_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<quality_ratings_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid quality_ratings",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(quality_ratings_range = Rating | Uad36QualityRating);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct StructureState {
    pub subject: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub living_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub gross_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_floor_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_finished_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub areas: Option<Vec<AreaMeasure>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built_estimated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stories: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exterior_wall_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_material_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_style_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_material: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub condition_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub quality_ratings: Option<Vec<RatingOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_fuel_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub cooling_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sewer_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub water_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub residential: Option<ResidentialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commercial: Option<CommercialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub renovations: Option<Vec<Renovation>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl StructureState {
    #[new]
    #[pyo3(signature = (subject, id, kind=None, name=None, structure_number=None, living_area=None, gross_area=None, rentable_area=None, ground_floor_area=None, basement_area=None, basement_finished_area=None, garage_area=None, areas=None, year_built=None, year_built_estimated=None, effective_year_built=None, stories=None, unit_count=None, construction_method=None, construction_status=None, construction_type=None, exterior_wall_type=None, roof_material_type=None, roof_style_type=None, foundation_type=None, foundation_material=None, condition_ratings=None, quality_ratings=None, heating_types=None, heating_fuel_type=None, cooling_types=None, sewer_type=None, water_type=None, features=None, residential=None, commercial=None, renovations=None, extras=None, provenance=None, verifications=None))]
    pub fn new(subject: String, id: String, kind: Option<String>, name: Option<String>, structure_number: Option<String>, living_area: Option<serde_utils::PyValue<Area>>, gross_area: Option<serde_utils::PyValue<Area>>, rentable_area: Option<serde_utils::PyValue<Area>>, ground_floor_area: Option<serde_utils::PyValue<Area>>, basement_area: Option<serde_utils::PyValue<Area>>, basement_finished_area: Option<serde_utils::PyValue<Area>>, garage_area: Option<serde_utils::PyValue<Area>>, areas: Option<serde_utils::PyValue<Vec<AreaMeasure>>>, year_built: Option<isize>, year_built_estimated: Option<bool>, effective_year_built: Option<isize>, stories: Option<f64>, unit_count: Option<isize>, construction_method: Option<String>, construction_status: Option<String>, construction_type: Option<String>, exterior_wall_type: Option<String>, roof_material_type: Option<String>, roof_style_type: Option<String>, foundation_type: Option<String>, foundation_material: Option<String>, condition_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, quality_ratings: Option<serde_utils::PyValue<Vec<RatingOrSubtype>>>, heating_types: Option<Vec<String>>, heating_fuel_type: Option<String>, cooling_types: Option<Vec<String>>, sewer_type: Option<String>, water_type: Option<String>, features: Option<Vec<String>>, residential: Option<serde_utils::PyValue<ResidentialDetails>>, commercial: Option<serde_utils::PyValue<CommercialDetails>>, renovations: Option<serde_utils::PyValue<Vec<Renovation>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let living_area = living_area.map(|v| v.into_inner());
        let gross_area = gross_area.map(|v| v.into_inner());
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let ground_floor_area = ground_floor_area.map(|v| v.into_inner());
        let basement_area = basement_area.map(|v| v.into_inner());
        let basement_finished_area = basement_finished_area.map(|v| v.into_inner());
        let garage_area = garage_area.map(|v| v.into_inner());
        let areas = areas.map(|v| v.into_inner());
        let condition_ratings = condition_ratings.map(|v| v.into_inner());
        let quality_ratings = quality_ratings.map(|v| v.into_inner());
        let residential = residential.map(|v| v.into_inner());
        let commercial = commercial.map(|v| v.into_inner());
        let renovations = renovations.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        StructureState{subject, id, kind, name, structure_number, living_area, gross_area, rentable_area, ground_floor_area, basement_area, basement_finished_area, garage_area, areas, year_built, year_built_estimated, effective_year_built, stories, unit_count, construction_method, construction_status, construction_type, exterior_wall_type, roof_material_type, roof_style_type, foundation_type, foundation_material, condition_ratings, quality_ratings, heating_types, heating_fuel_type, cooling_types, sewer_type, water_type, features, residential, commercial, renovations, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructureState>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructureState> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructureState>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureState",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for StructureState {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a StructureState from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum StructureStateOrSubtype {    Uad36StructureState(Uad36StructureState)}

impl From<Uad36StructureState>   for StructureStateOrSubtype { fn from(x: Uad36StructureState)   -> Self { Self::Uad36StructureState(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for StructureStateOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36StructureState>() {
            return Ok(StructureStateOrSubtype::Uad36StructureState(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureStateOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for StructureStateOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            StructureStateOrSubtype::Uad36StructureState(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StructureStateOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StructureStateOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StructureStateOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StructureStateOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for StructureStateOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36StructureState::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(StructureStateOrSubtype::Uad36StructureState(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36StructureState::from_pair_simple(k.clone(), v.clone()) {
            return Ok(StructureStateOrSubtype::Uad36StructureState(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            StructureStateOrSubtype::Uad36StructureState(inner) => inner.extract_key(),
        }
    }
}

#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(StructureStateOrSubtype = Uad36StructureState);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36StructureState {
    pub subject: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub name: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub living_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub gross_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_floor_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basement_finished_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub garage_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub areas: Option<Vec<AreaMeasure>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub year_built_estimated: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_year_built: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stories: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub construction_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exterior_wall_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_material_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roof_style_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub foundation_material: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub condition_ratings: Option<Vec<Uad36ConditionRating>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub quality_ratings: Option<Vec<Uad36QualityRating>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub heating_fuel_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub cooling_types: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sewer_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub water_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_primitive_list_or_single_value_optional",
        serialize_with = "serde_utils::serialize_primitive_list_or_single_value_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub features: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub residential: Option<ResidentialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commercial: Option<CommercialDetails>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub renovations: Option<Vec<Renovation>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36StructureState {
    #[new]
    #[pyo3(signature = (subject, id, kind=None, name=None, structure_number=None, living_area=None, gross_area=None, rentable_area=None, ground_floor_area=None, basement_area=None, basement_finished_area=None, garage_area=None, areas=None, year_built=None, year_built_estimated=None, effective_year_built=None, stories=None, unit_count=None, construction_method=None, construction_status=None, construction_type=None, exterior_wall_type=None, roof_material_type=None, roof_style_type=None, foundation_type=None, foundation_material=None, condition_ratings=None, quality_ratings=None, heating_types=None, heating_fuel_type=None, cooling_types=None, sewer_type=None, water_type=None, features=None, residential=None, commercial=None, renovations=None, extras=None, provenance=None, verifications=None))]
    pub fn new(subject: String, id: String, kind: Option<String>, name: Option<String>, structure_number: Option<String>, living_area: Option<serde_utils::PyValue<Area>>, gross_area: Option<serde_utils::PyValue<Area>>, rentable_area: Option<serde_utils::PyValue<Area>>, ground_floor_area: Option<serde_utils::PyValue<Area>>, basement_area: Option<serde_utils::PyValue<Area>>, basement_finished_area: Option<serde_utils::PyValue<Area>>, garage_area: Option<serde_utils::PyValue<Area>>, areas: Option<serde_utils::PyValue<Vec<AreaMeasure>>>, year_built: Option<isize>, year_built_estimated: Option<bool>, effective_year_built: Option<isize>, stories: Option<f64>, unit_count: Option<isize>, construction_method: Option<String>, construction_status: Option<String>, construction_type: Option<String>, exterior_wall_type: Option<String>, roof_material_type: Option<String>, roof_style_type: Option<String>, foundation_type: Option<String>, foundation_material: Option<String>, condition_ratings: Option<serde_utils::PyValue<Vec<Uad36ConditionRating>>>, quality_ratings: Option<serde_utils::PyValue<Vec<Uad36QualityRating>>>, heating_types: Option<Vec<String>>, heating_fuel_type: Option<String>, cooling_types: Option<Vec<String>>, sewer_type: Option<String>, water_type: Option<String>, features: Option<Vec<String>>, residential: Option<serde_utils::PyValue<ResidentialDetails>>, commercial: Option<serde_utils::PyValue<CommercialDetails>>, renovations: Option<serde_utils::PyValue<Vec<Renovation>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let living_area = living_area.map(|v| v.into_inner());
        let gross_area = gross_area.map(|v| v.into_inner());
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let ground_floor_area = ground_floor_area.map(|v| v.into_inner());
        let basement_area = basement_area.map(|v| v.into_inner());
        let basement_finished_area = basement_finished_area.map(|v| v.into_inner());
        let garage_area = garage_area.map(|v| v.into_inner());
        let areas = areas.map(|v| v.into_inner());
        let condition_ratings = condition_ratings.map(|v| v.into_inner());
        let quality_ratings = quality_ratings.map(|v| v.into_inner());
        let residential = residential.map(|v| v.into_inner());
        let commercial = commercial.map(|v| v.into_inner());
        let renovations = renovations.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Uad36StructureState{subject, id, kind, name, structure_number, living_area, gross_area, rentable_area, ground_floor_area, basement_area, basement_finished_area, garage_area, areas, year_built, year_built_estimated, effective_year_built, stories, unit_count, construction_method, construction_status, construction_type, exterior_wall_type, roof_material_type, roof_style_type, foundation_type, foundation_material, condition_ratings, quality_ratings, heating_types, heating_fuel_type, cooling_types, sewer_type, water_type, features, residential, commercial, renovations, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36StructureState>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36StructureState> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36StructureState>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36StructureState",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Uad36StructureState {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Uad36StructureState from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SpaceState {
    pub subject: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub floor_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub space_use: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rentable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub usable_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bedrooms: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_adu: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_active: Option<bool>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SpaceState {
    #[new]
    #[pyo3(signature = (subject, id, floor_number=None, space_use=None, rentable_area=None, usable_area=None, bedrooms=None, bathrooms=None, occupancy=None, is_adu=None, is_active=None, extras=None, provenance=None, verifications=None))]
    pub fn new(subject: String, id: String, floor_number: Option<isize>, space_use: Option<String>, rentable_area: Option<serde_utils::PyValue<Area>>, usable_area: Option<serde_utils::PyValue<Area>>, bedrooms: Option<isize>, bathrooms: Option<f64>, occupancy: Option<String>, is_adu: Option<bool>, is_active: Option<bool>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let rentable_area = rentable_area.map(|v| v.into_inner());
        let usable_area = usable_area.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        SpaceState{subject, id, floor_number, space_use, rentable_area, usable_area, bedrooms, bathrooms, occupancy, is_adu, is_active, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SpaceState>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SpaceState> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SpaceState>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SpaceState",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SpaceState {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a SpaceState from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

pub mod property_state_snapshot_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum structure_states_range {
        StructureState(StructureState),
        Uad36StructureState(Uad36StructureState)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for structure_states_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<StructureState>() {
                return Ok(structure_states_range::StructureState(val));
            }            if let Ok(val) = ob.extract::<Uad36StructureState>() {
                return Ok(structure_states_range::Uad36StructureState(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid structure_states",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for structure_states_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                structure_states_range::StructureState(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                structure_states_range::Uad36StructureState(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<structure_states_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<structure_states_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<structure_states_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid structure_states",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(structure_states_range = StructureState | Uad36StructureState);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyStateSnapshot {
    pub property: String,
    pub as_of_date: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basis: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<PropertyState>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub site_states: Option<Vec<SiteState>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_states: Option<Vec<StructureStateOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub space_states: Option<Vec<SpaceState>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyStateSnapshot {
    #[new]
    #[pyo3(signature = (property, as_of_date, id, basis=None, property_state=None, site_states=None, structure_states=None, space_states=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, as_of_date: NaiveDate, id: String, basis: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, property_state: Option<serde_utils::PyValue<PropertyState>>, site_states: Option<serde_utils::PyValue<Vec<SiteState>>>, structure_states: Option<serde_utils::PyValue<Vec<StructureStateOrSubtype>>>, space_states: Option<serde_utils::PyValue<Vec<SpaceState>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let basis = basis.map(|v| v.into_inner());
        let property_state = property_state.map(|v| v.into_inner());
        let site_states = site_states.map(|v| v.into_inner());
        let structure_states = structure_states.map(|v| v.into_inner());
        let space_states = space_states.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyStateSnapshot{property, as_of_date, id, basis, property_state, site_states, structure_states, space_states, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyStateSnapshot>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyStateSnapshot> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyStateSnapshot>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyStateSnapshot",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyStateSnapshot {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyStateSnapshot from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum PropertyStateSnapshotOrSubtype {    Uad36PropertyStateSnapshot(Uad36PropertyStateSnapshot)}

impl From<Uad36PropertyStateSnapshot>   for PropertyStateSnapshotOrSubtype { fn from(x: Uad36PropertyStateSnapshot)   -> Self { Self::Uad36PropertyStateSnapshot(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for PropertyStateSnapshotOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36PropertyStateSnapshot>() {
            return Ok(PropertyStateSnapshotOrSubtype::Uad36PropertyStateSnapshot(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyStateSnapshotOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for PropertyStateSnapshotOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            PropertyStateSnapshotOrSubtype::Uad36PropertyStateSnapshot(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyStateSnapshotOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyStateSnapshotOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyStateSnapshotOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyStateSnapshotOrSubtype",
        ))
    }
}

#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyStateSnapshotOrSubtype {
    type Key       = String;
    type Value     = serde_value::Value;
    type Error     = String;

    fn from_pair_mapping(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36PropertyStateSnapshot::from_pair_mapping(k.clone(), v.clone()) {
            return Ok(PropertyStateSnapshotOrSubtype::Uad36PropertyStateSnapshot(x));
        }
        Err("none of the variants matched the mapping form".into())
    }

    fn from_pair_simple(k: Self::Key, v: Self::Value) -> Result<Self, Self::Error> {
        if let Ok(x) = Uad36PropertyStateSnapshot::from_pair_simple(k.clone(), v.clone()) {
            return Ok(PropertyStateSnapshotOrSubtype::Uad36PropertyStateSnapshot(x));
        }
        Err("none of the variants support the primitive form".into())
    }

    fn extract_key(&self) -> &Self::Key {
        match self {
            PropertyStateSnapshotOrSubtype::Uad36PropertyStateSnapshot(inner) => inner.extract_key(),
        }
    }
}

#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(PropertyStateSnapshotOrSubtype = Uad36PropertyStateSnapshot);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36PropertyStateSnapshot {
    pub property: String,
    pub as_of_date: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub basis: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<PropertyState>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub site_states: Option<Vec<SiteState>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure_states: Option<Vec<Uad36StructureState>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub space_states: Option<Vec<SpaceState>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36PropertyStateSnapshot {
    #[new]
    #[pyo3(signature = (property, as_of_date, id, basis=None, property_state=None, site_states=None, structure_states=None, space_states=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, as_of_date: NaiveDate, id: String, basis: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, property_state: Option<serde_utils::PyValue<PropertyState>>, site_states: Option<serde_utils::PyValue<Vec<SiteState>>>, structure_states: Option<serde_utils::PyValue<Vec<Uad36StructureState>>>, space_states: Option<serde_utils::PyValue<Vec<SpaceState>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let basis = basis.map(|v| v.into_inner());
        let property_state = property_state.map(|v| v.into_inner());
        let site_states = site_states.map(|v| v.into_inner());
        let structure_states = structure_states.map(|v| v.into_inner());
        let space_states = space_states.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Uad36PropertyStateSnapshot{property, as_of_date, id, basis, property_state, site_states, structure_states, space_states, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36PropertyStateSnapshot>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36PropertyStateSnapshot> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36PropertyStateSnapshot>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36PropertyStateSnapshot",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Uad36PropertyStateSnapshot {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Uad36PropertyStateSnapshot from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyAssociation {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fee: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fee_period: Option<RentPeriod>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyAssociation {
    #[new]
    #[pyo3(signature = (property, party, id, fee=None, fee_period=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, party: String, id: String, fee: Option<serde_utils::PyValue<Money>>, fee_period: Option<RentPeriod>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let fee = fee.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        PropertyAssociation{property, party, id, fee, fee_period, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyAssociation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyAssociation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyAssociation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyAssociation",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for PropertyAssociation {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a PropertyAssociation from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Assessment {
    pub parcel: String,
    pub jurisdiction: String,
    pub tax_year: isize,
    #[cfg_attr(feature = "serde", serde(default))]
    pub roll_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessed_land_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessed_improvement_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessed_total_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub market_land_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub market_improvement_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub market_total_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub appraised_land_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub appraised_improvement_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub appraised_total_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exemptions: Option<Vec<TaxExemption>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Assessment {
    #[new]
    #[pyo3(signature = (parcel, jurisdiction, tax_year, id, roll_type=None, assessed_land_value=None, assessed_improvement_value=None, assessed_total_value=None, market_land_value=None, market_improvement_value=None, market_total_value=None, appraised_land_value=None, appraised_improvement_value=None, appraised_total_value=None, exemptions=None, extras=None, provenance=None, verifications=None))]
    pub fn new(parcel: String, jurisdiction: String, tax_year: isize, id: String, roll_type: Option<String>, assessed_land_value: Option<serde_utils::PyValue<Money>>, assessed_improvement_value: Option<serde_utils::PyValue<Money>>, assessed_total_value: Option<serde_utils::PyValue<Money>>, market_land_value: Option<serde_utils::PyValue<Money>>, market_improvement_value: Option<serde_utils::PyValue<Money>>, market_total_value: Option<serde_utils::PyValue<Money>>, appraised_land_value: Option<serde_utils::PyValue<Money>>, appraised_improvement_value: Option<serde_utils::PyValue<Money>>, appraised_total_value: Option<serde_utils::PyValue<Money>>, exemptions: Option<serde_utils::PyValue<Vec<TaxExemption>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let assessed_land_value = assessed_land_value.map(|v| v.into_inner());
        let assessed_improvement_value = assessed_improvement_value.map(|v| v.into_inner());
        let assessed_total_value = assessed_total_value.map(|v| v.into_inner());
        let market_land_value = market_land_value.map(|v| v.into_inner());
        let market_improvement_value = market_improvement_value.map(|v| v.into_inner());
        let market_total_value = market_total_value.map(|v| v.into_inner());
        let appraised_land_value = appraised_land_value.map(|v| v.into_inner());
        let appraised_improvement_value = appraised_improvement_value.map(|v| v.into_inner());
        let appraised_total_value = appraised_total_value.map(|v| v.into_inner());
        let exemptions = exemptions.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Assessment{parcel, jurisdiction, tax_year, id, roll_type, assessed_land_value, assessed_improvement_value, assessed_total_value, market_land_value, market_improvement_value, market_total_value, appraised_land_value, appraised_improvement_value, appraised_total_value, exemptions, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Assessment>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Assessment> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Assessment>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Assessment",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Assessment {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Assessment from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TaxExemption {
    pub kind: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TaxExemption {
    #[new]
    #[pyo3(signature = (kind, amount=None, extras=None))]
    pub fn new(kind: String, amount: Option<serde_utils::PyValue<Money>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let amount = amount.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        TaxExemption{kind, amount, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TaxExemption>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TaxExemption> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TaxExemption>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TaxExemption",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TaxBill {
    pub parcel: String,
    pub jurisdiction: String,
    pub tax_year: isize,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bill_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount_billed: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount_paid: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_delinquent: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub delinquent_year: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub delinquent_amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rate_code_area: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub installments: Option<Vec<TaxInstallment>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub line_items: Option<Vec<TaxLineItem>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TaxBill {
    #[new]
    #[pyo3(signature = (parcel, jurisdiction, tax_year, id, bill_number=None, amount_billed=None, amount_paid=None, is_delinquent=None, delinquent_year=None, delinquent_amount=None, rate_code_area=None, installments=None, line_items=None, extras=None, provenance=None, verifications=None))]
    pub fn new(parcel: String, jurisdiction: String, tax_year: isize, id: String, bill_number: Option<String>, amount_billed: Option<serde_utils::PyValue<Money>>, amount_paid: Option<serde_utils::PyValue<Money>>, is_delinquent: Option<bool>, delinquent_year: Option<isize>, delinquent_amount: Option<serde_utils::PyValue<Money>>, rate_code_area: Option<String>, installments: Option<serde_utils::PyValue<Vec<TaxInstallment>>>, line_items: Option<serde_utils::PyValue<Vec<TaxLineItem>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let amount_billed = amount_billed.map(|v| v.into_inner());
        let amount_paid = amount_paid.map(|v| v.into_inner());
        let delinquent_amount = delinquent_amount.map(|v| v.into_inner());
        let installments = installments.map(|v| v.into_inner());
        let line_items = line_items.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        TaxBill{parcel, jurisdiction, tax_year, id, bill_number, amount_billed, amount_paid, is_delinquent, delinquent_year, delinquent_amount, rate_code_area, installments, line_items, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TaxBill>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TaxBill> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TaxBill>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TaxBill",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for TaxBill {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a TaxBill from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TaxInstallment {
    #[cfg_attr(feature = "serde", serde(default))]
    pub installment_number: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub due_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub paid_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount_paid: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_delinquent: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TaxInstallment {
    #[new]
    #[pyo3(signature = (installment_number=None, due_on=None, amount=None, paid_on=None, amount_paid=None, is_delinquent=None, extras=None))]
    pub fn new(installment_number: Option<isize>, due_on: Option<NaiveDate>, amount: Option<serde_utils::PyValue<Money>>, paid_on: Option<NaiveDate>, amount_paid: Option<serde_utils::PyValue<Money>>, is_delinquent: Option<bool>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let amount = amount.map(|v| v.into_inner());
        let amount_paid = amount_paid.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        TaxInstallment{installment_number, due_on, amount, paid_on, amount_paid, is_delinquent, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TaxInstallment>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TaxInstallment> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TaxInstallment>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TaxInstallment",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TaxLineItem {
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub jurisdiction: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rate: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TaxLineItem {
    #[new]
    #[pyo3(signature = (jurisdiction=None, rate=None, amount=None, extras=None))]
    pub fn new(jurisdiction: Option<String>, rate: Option<f64>, amount: Option<serde_utils::PyValue<Money>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let amount = amount.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        TaxLineItem{jurisdiction, rate, amount, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TaxLineItem>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TaxLineItem> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TaxLineItem>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TaxLineItem",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Transfer {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcel: Option<String>,
    pub transfer_kind: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub consideration: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub transfer_tax: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_disclosure: Option<PriceDisclosure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_code: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interest_conveyed: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub partial_interest_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_inter_family: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_distressed: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<TransferParty>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Transfer {
    #[new]
    #[pyo3(signature = (property, transfer_kind, id, parcel=None, effective_on=None, consideration=None, transfer_tax=None, price_disclosure=None, price_code=None, interest_conveyed=None, partial_interest_pct=None, is_inter_family=None, is_distressed=None, parties=None, document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, transfer_kind: String, id: String, parcel: Option<String>, effective_on: Option<NaiveDate>, consideration: Option<serde_utils::PyValue<Money>>, transfer_tax: Option<serde_utils::PyValue<Money>>, price_disclosure: Option<PriceDisclosure>, price_code: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, interest_conveyed: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, partial_interest_pct: Option<f64>, is_inter_family: Option<bool>, is_distressed: Option<bool>, parties: Option<serde_utils::PyValue<Vec<TransferParty>>>, document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let consideration = consideration.map(|v| v.into_inner());
        let transfer_tax = transfer_tax.map(|v| v.into_inner());
        let price_code = price_code.map(|v| v.into_inner());
        let interest_conveyed = interest_conveyed.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Transfer{property, transfer_kind, id, parcel, effective_on, consideration, transfer_tax, price_disclosure, price_code, interest_conveyed, partial_interest_pct, is_inter_family, is_distressed, parties, document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Transfer>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Transfer> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Transfer>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Transfer",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Transfer {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Transfer from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct TransferParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl TransferParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        TransferParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<TransferParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<TransferParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<TransferParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid TransferParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SaleEvent {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub transfer: Option<String>,
    pub sale_date: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sale_price: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_disclosure: Option<PriceDisclosure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_code: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sale_type: Option<SaleTypeEnum>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub price_per_unit: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub financing: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub concessions: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cap_rate: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub noi_at_sale: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub opex_at_sale: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy_at_sale_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count_at_sale: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub supporting_operating_statement: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<SaleEventParty>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub remarks: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SaleEvent {
    #[new]
    #[pyo3(signature = (property, sale_date, id, property_state=None, transfer=None, sale_price=None, price_disclosure=None, price_code=None, sale_type=None, price_per_area=None, price_per_unit=None, financing=None, concessions=None, cap_rate=None, noi_at_sale=None, opex_at_sale=None, occupancy_at_sale_pct=None, unit_count_at_sale=None, supporting_operating_statement=None, parties=None, remarks=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, sale_date: NaiveDate, id: String, property_state: Option<String>, transfer: Option<String>, sale_price: Option<serde_utils::PyValue<Money>>, price_disclosure: Option<PriceDisclosure>, price_code: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, sale_type: Option<SaleTypeEnum>, price_per_area: Option<serde_utils::PyValue<UnitRate>>, price_per_unit: Option<serde_utils::PyValue<Money>>, financing: Option<String>, concessions: Option<serde_utils::PyValue<Money>>, cap_rate: Option<f64>, noi_at_sale: Option<serde_utils::PyValue<Money>>, opex_at_sale: Option<serde_utils::PyValue<Money>>, occupancy_at_sale_pct: Option<f64>, unit_count_at_sale: Option<isize>, supporting_operating_statement: Option<String>, parties: Option<serde_utils::PyValue<Vec<SaleEventParty>>>, remarks: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let sale_price = sale_price.map(|v| v.into_inner());
        let price_code = price_code.map(|v| v.into_inner());
        let price_per_area = price_per_area.map(|v| v.into_inner());
        let price_per_unit = price_per_unit.map(|v| v.into_inner());
        let concessions = concessions.map(|v| v.into_inner());
        let noi_at_sale = noi_at_sale.map(|v| v.into_inner());
        let opex_at_sale = opex_at_sale.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        SaleEvent{property, sale_date, id, property_state, transfer, sale_price, price_disclosure, price_code, sale_type, price_per_area, price_per_unit, financing, concessions, cap_rate, noi_at_sale, opex_at_sale, occupancy_at_sale_pct, unit_count_at_sale, supporting_operating_statement, parties, remarks, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SaleEvent>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SaleEvent> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SaleEvent>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SaleEvent",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for SaleEvent {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a SaleEvent from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct SaleEventParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl SaleEventParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        SaleEventParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<SaleEventParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<SaleEventParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<SaleEventParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid SaleEventParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Listing {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<String>,
    pub kind: ListingKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub listing_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub mls_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub events: Option<Vec<ListingEvent>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub participants: Option<Vec<ListingParticipant>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub remarks: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Listing {
    #[new]
    #[pyo3(signature = (property, kind, id, property_state=None, listing_type=None, mls_number=None, events=None, participants=None, artifacts=None, remarks=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, kind: ListingKind, id: String, property_state: Option<String>, listing_type: Option<String>, mls_number: Option<String>, events: Option<serde_utils::PyValue<Vec<ListingEvent>>>, participants: Option<serde_utils::PyValue<Vec<ListingParticipant>>>, artifacts: Option<Vec<String>>, remarks: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let events = events.map(|v| v.into_inner());
        let participants = participants.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Listing{property, kind, id, property_state, listing_type, mls_number, events, participants, artifacts, remarks, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Listing>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Listing> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Listing>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Listing",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Listing {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Listing from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ListingEvent {
    pub occurred_on: NaiveDate,
    pub event_kind: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<ListingStatus>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub asking_price: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent_period: Option<RentPeriod>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub close_price: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ListingEvent {
    #[new]
    #[pyo3(signature = (occurred_on, event_kind, status=None, asking_price=None, rent_period=None, close_price=None, extras=None, provenance=None))]
    pub fn new(occurred_on: NaiveDate, event_kind: String, status: Option<ListingStatus>, asking_price: Option<serde_utils::PyValue<Money>>, rent_period: Option<RentPeriod>, close_price: Option<serde_utils::PyValue<Money>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>) -> Self {
        let asking_price = asking_price.map(|v| v.into_inner());
        let close_price = close_price.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        ListingEvent{occurred_on, event_kind, status, asking_price, rent_period, close_price, extras, provenance}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ListingEvent>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ListingEvent> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ListingEvent>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ListingEvent",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ListingParticipant {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ListingParticipant {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        ListingParticipant{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ListingParticipant>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ListingParticipant> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ListingParticipant>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ListingParticipant",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LeaseEvent {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub space: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lease_type: Option<LeaseTypeEnum>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub execution_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub commencement_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub expiration_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub term_months: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub leased_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent_period: Option<RentPeriod>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub starting_rent_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_rent_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub net_effective_rent_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub free_rent_months: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ti_allowance_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub expense_structure: Option<ExpenseStructure>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<LeaseEventParty>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub escalations: Option<Vec<LeaseEscalation>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub concessions: Option<Vec<LeaseConcession>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub remarks: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LeaseEvent {
    #[new]
    #[pyo3(signature = (property, id, property_state=None, space=None, lease_type=None, execution_date=None, commencement_date=None, expiration_date=None, term_months=None, leased_area=None, rent=None, rent_period=None, starting_rent_per_area=None, effective_rent_per_area=None, net_effective_rent_per_area=None, free_rent_months=None, ti_allowance_per_area=None, expense_structure=None, parties=None, escalations=None, concessions=None, remarks=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, property_state: Option<String>, space: Option<String>, lease_type: Option<LeaseTypeEnum>, execution_date: Option<NaiveDate>, commencement_date: Option<NaiveDate>, expiration_date: Option<NaiveDate>, term_months: Option<isize>, leased_area: Option<serde_utils::PyValue<Area>>, rent: Option<serde_utils::PyValue<Money>>, rent_period: Option<RentPeriod>, starting_rent_per_area: Option<serde_utils::PyValue<UnitRate>>, effective_rent_per_area: Option<serde_utils::PyValue<UnitRate>>, net_effective_rent_per_area: Option<serde_utils::PyValue<UnitRate>>, free_rent_months: Option<f64>, ti_allowance_per_area: Option<serde_utils::PyValue<UnitRate>>, expense_structure: Option<serde_utils::PyValue<ExpenseStructure>>, parties: Option<serde_utils::PyValue<Vec<LeaseEventParty>>>, escalations: Option<serde_utils::PyValue<Vec<LeaseEscalation>>>, concessions: Option<serde_utils::PyValue<Vec<LeaseConcession>>>, remarks: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let leased_area = leased_area.map(|v| v.into_inner());
        let rent = rent.map(|v| v.into_inner());
        let starting_rent_per_area = starting_rent_per_area.map(|v| v.into_inner());
        let effective_rent_per_area = effective_rent_per_area.map(|v| v.into_inner());
        let net_effective_rent_per_area = net_effective_rent_per_area.map(|v| v.into_inner());
        let ti_allowance_per_area = ti_allowance_per_area.map(|v| v.into_inner());
        let expense_structure = expense_structure.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let escalations = escalations.map(|v| v.into_inner());
        let concessions = concessions.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        LeaseEvent{property, id, property_state, space, lease_type, execution_date, commencement_date, expiration_date, term_months, leased_area, rent, rent_period, starting_rent_per_area, effective_rent_per_area, net_effective_rent_per_area, free_rent_months, ti_allowance_per_area, expense_structure, parties, escalations, concessions, remarks, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LeaseEvent>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LeaseEvent> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LeaseEvent>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LeaseEvent",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for LeaseEvent {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a LeaseEvent from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LeaseEventParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LeaseEventParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        LeaseEventParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LeaseEventParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LeaseEventParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LeaseEventParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LeaseEventParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ExpenseStructure {
    #[cfg_attr(feature = "serde", serde(default))]
    pub taxes_paid_by: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub insurance_paid_by: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cam_paid_by: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub utilities_paid_by: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub notes: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ExpenseStructure {
    #[new]
    #[pyo3(signature = (taxes_paid_by=None, insurance_paid_by=None, cam_paid_by=None, utilities_paid_by=None, notes=None, extras=None))]
    pub fn new(taxes_paid_by: Option<String>, insurance_paid_by: Option<String>, cam_paid_by: Option<String>, utilities_paid_by: Option<String>, notes: Option<String>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        ExpenseStructure{taxes_paid_by, insurance_paid_by, cam_paid_by, utilities_paid_by, notes, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ExpenseStructure>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ExpenseStructure> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ExpenseStructure>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExpenseStructure",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LeaseEscalation {
    pub escalation_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub escalation_value: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub frequency_months: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cpi_index: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cpi_floor: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub cpi_cap: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub steps: Option<Vec<RentStep>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_from: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub effective_until: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LeaseEscalation {
    #[new]
    #[pyo3(signature = (escalation_type, escalation_value=None, frequency_months=None, cpi_index=None, cpi_floor=None, cpi_cap=None, steps=None, effective_from=None, effective_until=None, extras=None))]
    pub fn new(escalation_type: String, escalation_value: Option<f64>, frequency_months: Option<isize>, cpi_index: Option<String>, cpi_floor: Option<f64>, cpi_cap: Option<f64>, steps: Option<serde_utils::PyValue<Vec<RentStep>>>, effective_from: Option<NaiveDate>, effective_until: Option<NaiveDate>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let steps = steps.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        LeaseEscalation{escalation_type, escalation_value, frequency_months, cpi_index, cpi_floor, cpi_cap, steps, effective_from, effective_until, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LeaseEscalation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LeaseEscalation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LeaseEscalation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LeaseEscalation",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct RentStep {
    pub from_date: NaiveDate,
    pub amount: Money
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl RentStep {
    #[new]
    #[pyo3(signature = (from_date, amount))]
    pub fn new(from_date: NaiveDate, amount: serde_utils::PyValue<Money>) -> Self {
        let amount = amount.into_inner();
        RentStep{from_date, amount}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RentStep>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RentStep> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RentStep>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RentStep",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LeaseConcession {
    pub concession_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub concession_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abatement_months: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub abatement_percent: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ti_cap_total: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub conditions: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub notes: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LeaseConcession {
    #[new]
    #[pyo3(signature = (concession_type, concession_value=None, abatement_months=None, abatement_percent=None, ti_cap_total=None, conditions=None, notes=None, extras=None))]
    pub fn new(concession_type: String, concession_value: Option<serde_utils::PyValue<Money>>, abatement_months: Option<f64>, abatement_percent: Option<f64>, ti_cap_total: Option<serde_utils::PyValue<Money>>, conditions: Option<serde_utils::PyValue<Any>>, notes: Option<String>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let concession_value = concession_value.map(|v| v.into_inner());
        let ti_cap_total = ti_cap_total.map(|v| v.into_inner());
        let conditions = conditions.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        LeaseConcession{concession_type, concession_value, abatement_months, abatement_percent, ti_cap_total, conditions, notes, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LeaseConcession>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LeaseConcession> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LeaseConcession>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LeaseConcession",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct UnitRentObservation {
    pub property: String,
    pub unit_type: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bedrooms: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub bathrooms: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub units_available: Option<isize>,
    pub rate: Money,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rate_period: Option<RentPeriod>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rate_basis: Option<RateBasis>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rate_type: Option<RateType>,
    pub observed_on: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub concessions_note: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl UnitRentObservation {
    #[new]
    #[pyo3(signature = (property, unit_type, rate, observed_on, id, unit_area=None, bedrooms=None, bathrooms=None, unit_count=None, units_available=None, rate_period=None, rate_basis=None, rate_type=None, concessions_note=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, unit_type: String, rate: serde_utils::PyValue<Money>, observed_on: NaiveDate, id: String, unit_area: Option<serde_utils::PyValue<Area>>, bedrooms: Option<isize>, bathrooms: Option<f64>, unit_count: Option<isize>, units_available: Option<isize>, rate_period: Option<RentPeriod>, rate_basis: Option<RateBasis>, rate_type: Option<RateType>, concessions_note: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let rate = rate.into_inner();
        let unit_area = unit_area.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        UnitRentObservation{property, unit_type, rate, observed_on, id, unit_area, bedrooms, bathrooms, unit_count, units_available, rate_period, rate_basis, rate_type, concessions_note, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<UnitRentObservation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<UnitRentObservation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<UnitRentObservation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid UnitRentObservation",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for UnitRentObservation {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a UnitRentObservation from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Loan {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcel: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub transfer: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_purchase_money: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub loan_amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub loan_type: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub purpose: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_heloc: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_construction: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_seller_carryback: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_assumable: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interest_rate: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub is_variable_rate: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub term_months: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub due_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lien_position: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<LoanParty>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub events: Option<Vec<LoanEvent>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Loan {
    #[new]
    #[pyo3(signature = (property, id, parcel=None, transfer=None, is_purchase_money=None, loan_amount=None, loan_type=None, purpose=None, is_heloc=None, is_construction=None, is_seller_carryback=None, is_assumable=None, interest_rate=None, is_variable_rate=None, term_months=None, due_date=None, lien_position=None, parties=None, events=None, document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, parcel: Option<String>, transfer: Option<String>, is_purchase_money: Option<bool>, loan_amount: Option<serde_utils::PyValue<Money>>, loan_type: Option<String>, purpose: Option<String>, is_heloc: Option<bool>, is_construction: Option<bool>, is_seller_carryback: Option<bool>, is_assumable: Option<bool>, interest_rate: Option<f64>, is_variable_rate: Option<bool>, term_months: Option<isize>, due_date: Option<NaiveDate>, lien_position: Option<isize>, parties: Option<serde_utils::PyValue<Vec<LoanParty>>>, events: Option<serde_utils::PyValue<Vec<LoanEvent>>>, document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let loan_amount = loan_amount.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let events = events.map(|v| v.into_inner());
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Loan{property, id, parcel, transfer, is_purchase_money, loan_amount, loan_type, purpose, is_heloc, is_construction, is_seller_carryback, is_assumable, interest_rate, is_variable_rate, term_months, due_date, lien_position, parties, events, document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Loan>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Loan> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Loan>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Loan",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Loan {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Loan from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LoanParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LoanParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        LoanParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LoanParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LoanParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LoanParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LoanParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LoanEvent {
    pub event_kind: LoanEventKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occurred_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub to_party: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LoanEvent {
    #[new]
    #[pyo3(signature = (event_kind, occurred_on=None, amount=None, to_party=None, extras=None, provenance=None, document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None))]
    pub fn new(event_kind: LoanEventKind, occurred_on: Option<NaiveDate>, amount: Option<serde_utils::PyValue<Money>>, to_party: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>) -> Self {
        let amount = amount.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        LoanEvent{event_kind, occurred_on, amount, to_party, extras, provenance, document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LoanEvent>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LoanEvent> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LoanEvent>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LoanEvent",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Lien {
    pub property: String,
    pub kind: LienKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub released_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<LienParty>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Lien {
    #[new]
    #[pyo3(signature = (property, kind, id, amount=None, released_on=None, parties=None, document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, kind: LienKind, id: String, amount: Option<serde_utils::PyValue<Money>>, released_on: Option<NaiveDate>, parties: Option<serde_utils::PyValue<Vec<LienParty>>>, document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let amount = amount.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Lien{property, kind, id, amount, released_on, parties, document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Lien>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Lien> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Lien>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Lien",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Lien {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Lien from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct LienParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl LienParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        LienParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<LienParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<LienParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<LienParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid LienParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ForeclosureCase {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub loan: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub case_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub opened_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub resolved_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub resolution: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub past_due_amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unpaid_balance: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub original_loan_amount: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub auction_min_bid: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub auction_location: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub filings: Option<Vec<ForeclosureFiling>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<ForeclosureCaseParty>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ForeclosureCase {
    #[new]
    #[pyo3(signature = (property, id, loan=None, case_number=None, opened_on=None, resolved_on=None, resolution=None, past_due_amount=None, unpaid_balance=None, original_loan_amount=None, auction_min_bid=None, auction_location=None, filings=None, parties=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, loan: Option<String>, case_number: Option<String>, opened_on: Option<NaiveDate>, resolved_on: Option<NaiveDate>, resolution: Option<String>, past_due_amount: Option<serde_utils::PyValue<Money>>, unpaid_balance: Option<serde_utils::PyValue<Money>>, original_loan_amount: Option<serde_utils::PyValue<Money>>, auction_min_bid: Option<serde_utils::PyValue<Money>>, auction_location: Option<String>, filings: Option<serde_utils::PyValue<Vec<ForeclosureFiling>>>, parties: Option<serde_utils::PyValue<Vec<ForeclosureCaseParty>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let past_due_amount = past_due_amount.map(|v| v.into_inner());
        let unpaid_balance = unpaid_balance.map(|v| v.into_inner());
        let original_loan_amount = original_loan_amount.map(|v| v.into_inner());
        let auction_min_bid = auction_min_bid.map(|v| v.into_inner());
        let filings = filings.map(|v| v.into_inner());
        let parties = parties.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        ForeclosureCase{property, id, loan, case_number, opened_on, resolved_on, resolution, past_due_amount, unpaid_balance, original_loan_amount, auction_min_bid, auction_location, filings, parties, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ForeclosureCase>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ForeclosureCase> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ForeclosureCase>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ForeclosureCase",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for ForeclosureCase {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a ForeclosureCase from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ForeclosureFiling {
    pub status: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub auction_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub auction_at_time: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_book: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_page: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recorded_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub instrument_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub document_type: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub recording_authority: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub registry_reference: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub related_instruments: Option<Vec<InstrumentReference>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ForeclosureFiling {
    #[new]
    #[pyo3(signature = (status, auction_on=None, auction_at_time=None, extras=None, provenance=None, document_number=None, recording_book=None, recording_page=None, recorded_on=None, instrument_date=None, document_type=None, recording_authority=None, registry_reference=None, related_instruments=None, artifacts=None))]
    pub fn new(status: String, auction_on: Option<NaiveDate>, auction_at_time: Option<String>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, document_number: Option<String>, recording_book: Option<String>, recording_page: Option<String>, recorded_on: Option<NaiveDate>, instrument_date: Option<NaiveDate>, document_type: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, recording_authority: Option<String>, registry_reference: Option<String>, related_instruments: Option<serde_utils::PyValue<Vec<InstrumentReference>>>, artifacts: Option<Vec<String>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let document_type = document_type.map(|v| v.into_inner());
        let related_instruments = related_instruments.map(|v| v.into_inner());
        ForeclosureFiling{status, auction_on, auction_at_time, extras, provenance, document_number, recording_book, recording_page, recorded_on, instrument_date, document_type, recording_authority, registry_reference, related_instruments, artifacts}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ForeclosureFiling>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ForeclosureFiling> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ForeclosureFiling>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ForeclosureFiling",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ForeclosureCaseParty {
    pub role: String,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub party: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub sequence: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ForeclosureCaseParty {
    #[new]
    #[pyo3(signature = (role, party, sequence=None, extras=None))]
    pub fn new(role: String, party: String, sequence: Option<isize>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let extras = extras.map(|v| v.into_inner());
        ForeclosureCaseParty{role, party, sequence, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ForeclosureCaseParty>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ForeclosureCaseParty> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ForeclosureCaseParty>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ForeclosureCaseParty",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Permit {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub structure: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub permitting_jurisdiction: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub permit_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub kind: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub status: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub description: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub applied_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub issued_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub finaled_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub expires_on: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub job_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub fees: Option<Money>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub contractor_party: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Permit {
    #[new]
    #[pyo3(signature = (property, id, structure=None, permitting_jurisdiction=None, permit_number=None, kind=None, status=None, description=None, applied_on=None, issued_on=None, finaled_on=None, expires_on=None, job_value=None, fees=None, contractor_party=None, artifacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, id: String, structure: Option<String>, permitting_jurisdiction: Option<String>, permit_number: Option<String>, kind: Option<String>, status: Option<String>, description: Option<String>, applied_on: Option<NaiveDate>, issued_on: Option<NaiveDate>, finaled_on: Option<NaiveDate>, expires_on: Option<NaiveDate>, job_value: Option<serde_utils::PyValue<Money>>, fees: Option<serde_utils::PyValue<Money>>, contractor_party: Option<String>, artifacts: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let job_value = job_value.map(|v| v.into_inner());
        let fees = fees.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Permit{property, id, structure, permitting_jurisdiction, permit_number, kind, status, description, applied_on, issued_on, finaled_on, expires_on, job_value, fees, contractor_party, artifacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Permit>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Permit> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Permit>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Permit",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Permit {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Permit from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct OperatingStatement {
    pub property: String,
    pub statement_year: isize,
    #[cfg_attr(feature = "serde", serde(default))]
    pub period_start: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub period_end: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub statement_basis: Option<StatementBasis>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub pgi: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub vacancy_loss: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub vacancy_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub egi: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub opex_total: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub noi: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub capex: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reimbursements: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reserves: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reserves_included_in_opex: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_lease_expense: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub ground_lease_included_in_opex: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub line_items: Option<Vec<StatementLineItem>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl OperatingStatement {
    #[new]
    #[pyo3(signature = (property, statement_year, id, period_start=None, period_end=None, statement_basis=None, pgi=None, vacancy_loss=None, vacancy_pct=None, egi=None, opex_total=None, noi=None, capex=None, reimbursements=None, reserves=None, reserves_included_in_opex=None, ground_lease_expense=None, ground_lease_included_in_opex=None, line_items=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, statement_year: isize, id: String, period_start: Option<NaiveDate>, period_end: Option<NaiveDate>, statement_basis: Option<StatementBasis>, pgi: Option<serde_utils::PyValue<Money>>, vacancy_loss: Option<serde_utils::PyValue<Money>>, vacancy_pct: Option<f64>, egi: Option<serde_utils::PyValue<Money>>, opex_total: Option<serde_utils::PyValue<Money>>, noi: Option<serde_utils::PyValue<Money>>, capex: Option<serde_utils::PyValue<Money>>, reimbursements: Option<serde_utils::PyValue<Money>>, reserves: Option<serde_utils::PyValue<Money>>, reserves_included_in_opex: Option<bool>, ground_lease_expense: Option<serde_utils::PyValue<Money>>, ground_lease_included_in_opex: Option<bool>, line_items: Option<serde_utils::PyValue<Vec<StatementLineItem>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let pgi = pgi.map(|v| v.into_inner());
        let vacancy_loss = vacancy_loss.map(|v| v.into_inner());
        let egi = egi.map(|v| v.into_inner());
        let opex_total = opex_total.map(|v| v.into_inner());
        let noi = noi.map(|v| v.into_inner());
        let capex = capex.map(|v| v.into_inner());
        let reimbursements = reimbursements.map(|v| v.into_inner());
        let reserves = reserves.map(|v| v.into_inner());
        let ground_lease_expense = ground_lease_expense.map(|v| v.into_inner());
        let line_items = line_items.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        OperatingStatement{property, statement_year, id, period_start, period_end, statement_basis, pgi, vacancy_loss, vacancy_pct, egi, opex_total, noi, capex, reimbursements, reserves, reserves_included_in_opex, ground_lease_expense, ground_lease_included_in_opex, line_items, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<OperatingStatement>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<OperatingStatement> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<OperatingStatement>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid OperatingStatement",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for OperatingStatement {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a OperatingStatement from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct StatementLineItem {
    pub category: String,
    pub label: String,
    pub amount: Money,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl StatementLineItem {
    #[new]
    #[pyo3(signature = (category, label, amount, extras=None))]
    pub fn new(category: String, label: String, amount: serde_utils::PyValue<Money>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let amount = amount.into_inner();
        let extras = extras.map(|v| v.into_inner());
        StatementLineItem{category, label, amount, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<StatementLineItem>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<StatementLineItem> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<StatementLineItem>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid StatementLineItem",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct RentRoll {
    pub property: String,
    pub as_of_date: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupied_unit_count: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy_pct: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub total_contract_rent: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub total_market_rent: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent_period: Option<RentPeriod>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lines: Option<Vec<RentRollLine>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl RentRoll {
    #[new]
    #[pyo3(signature = (property, as_of_date, id, unit_count=None, occupied_unit_count=None, occupancy_pct=None, total_contract_rent=None, total_market_rent=None, rent_period=None, lines=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, as_of_date: NaiveDate, id: String, unit_count: Option<isize>, occupied_unit_count: Option<isize>, occupancy_pct: Option<f64>, total_contract_rent: Option<serde_utils::PyValue<Money>>, total_market_rent: Option<serde_utils::PyValue<Money>>, rent_period: Option<RentPeriod>, lines: Option<serde_utils::PyValue<Vec<RentRollLine>>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let total_contract_rent = total_contract_rent.map(|v| v.into_inner());
        let total_market_rent = total_market_rent.map(|v| v.into_inner());
        let lines = lines.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        RentRoll{property, as_of_date, id, unit_count, occupied_unit_count, occupancy_pct, total_contract_rent, total_market_rent, rent_period, lines, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RentRoll>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RentRoll> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RentRoll>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RentRoll",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for RentRoll {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a RentRoll from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct RentRollLine {
    #[cfg_attr(feature = "serde", serde(default))]
    pub space: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tenant: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub lease: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub occupancy_status: Option<CodeableConceptOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub reported_area: Option<Area>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub contract_rent: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub market_rent: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl RentRollLine {
    #[new]
    #[pyo3(signature = (space=None, tenant=None, lease=None, occupancy_status=None, reported_area=None, contract_rent=None, market_rent=None, extras=None))]
    pub fn new(space: Option<String>, tenant: Option<String>, lease: Option<String>, occupancy_status: Option<serde_utils::PyValue<CodeableConceptOrSubtype>>, reported_area: Option<serde_utils::PyValue<Area>>, contract_rent: Option<serde_utils::PyValue<Money>>, market_rent: Option<serde_utils::PyValue<Money>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let occupancy_status = occupancy_status.map(|v| v.into_inner());
        let reported_area = reported_area.map(|v| v.into_inner());
        let contract_rent = contract_rent.map(|v| v.into_inner());
        let market_rent = market_rent.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        RentRollLine{space, tenant, lease, occupancy_status, reported_area, contract_rent, market_rent, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<RentRollLine>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<RentRollLine> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<RentRollLine>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid RentRollLine",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Valuation {
    pub property: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state: Option<String>,
    pub kind: ValuationKind,
    #[cfg_attr(feature = "serde", serde(default))]
    pub valuation_method: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_type: Option<String>,
    pub value: Money,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_low: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_high: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_per_area: Option<UnitRate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub land_value: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub confidence_score: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub forecast_standard_deviation: Option<f64>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub exposure_days: Option<isize>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub indicated_value_sales_comparison: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub indicated_value_cost: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub indicated_value_income: Option<Money>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub value_premise: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub interest: Option<String>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string_optional"))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub performed_by_party: Option<String>,
    pub as_of_date: NaiveDate,
    #[cfg_attr(feature = "serde", serde(default))]
    pub report_date: Option<NaiveDate>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(deserialize_with = "serde_utils::deserialize_nonblank_trimmed_string"))]
    pub id: String,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub verifications: Option<Vec<VerificationAttribution>>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Valuation {
    #[new]
    #[pyo3(signature = (property, kind, value, as_of_date, id, property_state=None, valuation_method=None, value_type=None, value_low=None, value_high=None, value_per_area=None, land_value=None, confidence_score=None, forecast_standard_deviation=None, exposure_days=None, indicated_value_sales_comparison=None, indicated_value_cost=None, indicated_value_income=None, value_premise=None, interest=None, performed_by_party=None, report_date=None, artifacts=None, extras=None, provenance=None, verifications=None))]
    pub fn new(property: String, kind: ValuationKind, value: serde_utils::PyValue<Money>, as_of_date: NaiveDate, id: String, property_state: Option<String>, valuation_method: Option<String>, value_type: Option<String>, value_low: Option<serde_utils::PyValue<Money>>, value_high: Option<serde_utils::PyValue<Money>>, value_per_area: Option<serde_utils::PyValue<UnitRate>>, land_value: Option<serde_utils::PyValue<Money>>, confidence_score: Option<isize>, forecast_standard_deviation: Option<f64>, exposure_days: Option<isize>, indicated_value_sales_comparison: Option<serde_utils::PyValue<Money>>, indicated_value_cost: Option<serde_utils::PyValue<Money>>, indicated_value_income: Option<serde_utils::PyValue<Money>>, value_premise: Option<String>, interest: Option<String>, performed_by_party: Option<String>, report_date: Option<NaiveDate>, artifacts: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>, provenance: Option<serde_utils::PyValue<Provenance>>, verifications: Option<serde_utils::PyValue<Vec<VerificationAttribution>>>) -> Self {
        let value = value.into_inner();
        let value_low = value_low.map(|v| v.into_inner());
        let value_high = value_high.map(|v| v.into_inner());
        let value_per_area = value_per_area.map(|v| v.into_inner());
        let land_value = land_value.map(|v| v.into_inner());
        let indicated_value_sales_comparison = indicated_value_sales_comparison.map(|v| v.into_inner());
        let indicated_value_cost = indicated_value_cost.map(|v| v.into_inner());
        let indicated_value_income = indicated_value_income.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let verifications = verifications.map(|v| v.into_inner());
        Valuation{property, kind, value, as_of_date, id, property_state, valuation_method, value_type, value_low, value_high, value_per_area, land_value, confidence_score, forecast_standard_deviation, exposure_days, indicated_value_sales_comparison, indicated_value_cost, indicated_value_income, value_premise, interest, performed_by_party, report_date, artifacts, extras, provenance, verifications}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Valuation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Valuation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Valuation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Valuation",
        ))
    }
}


#[cfg(feature = "serde")]
impl serde_utils::InlinedPair for Valuation {
    type Key   = String;
    type Value = Value;
    type Error = String;

    fn extract_key(&self) -> &Self::Key {
        return &self.id;
    }

    fn from_pair_mapping(k: Self::Key, v: Value) -> Result<Self,Self::Error> {
        let mut map = match v {
            Value::Map(m) => m,
            _ => return Err("ClassDefinition must be a mapping".into()),
        };
        let key_value = serde_value::to_value(k.clone())
            .map_err(|e| format!("unable to serialize key: {}", e))?;
        map.insert(Value::String("id".into()), key_value);
        let de          = Value::Map(map).into_deserializer();
        match serde_path_to_error::deserialize(de) {
            Ok(ok)  => Ok(ok),
            Err(e)  => Err(format!("at `{}`: {}", e.path(), e.inner())),
        }
    }


    fn from_pair_simple(_k: Self::Key, _v: Value) -> Result<Self,Self::Error> {
        Err("Cannot create a Valuation from a primitive value!".into())
    }


    fn compact_value(&self) -> Option<Value> {
        let value = match serde_value::to_value(self) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match value {
            Value::Map(mut map) => {
                map.remove(&Value::String("id".into()));
                Some(Value::Map(map))
            }
            _ => None,
        }
    }
}

pub mod property_profile_utl {
    use super::*;
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum structures_range {
        Structure(Structure),
        Uad36Structure(Uad36Structure)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for structures_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<Structure>() {
                return Ok(structures_range::Structure(val));
            }            if let Ok(val) = ob.extract::<Uad36Structure>() {
                return Ok(structures_range::Uad36Structure(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid structures",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for structures_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                structures_range::Structure(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                structures_range::Uad36Structure(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<structures_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<structures_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<structures_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid structures",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(structures_range = Structure | Uad36Structure);
    #[derive(Debug, Clone, PartialEq)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub enum property_state_snapshots_range {
        PropertyStateSnapshot(PropertyStateSnapshot),
        Uad36PropertyStateSnapshot(Uad36PropertyStateSnapshot)    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for property_state_snapshots_range {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<PropertyStateSnapshot>() {
                return Ok(property_state_snapshots_range::PropertyStateSnapshot(val));
            }            if let Ok(val) = ob.extract::<Uad36PropertyStateSnapshot>() {
                return Ok(property_state_snapshots_range::Uad36PropertyStateSnapshot(val));
            }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid property_state_snapshots",
            ))
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for property_state_snapshots_range {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;

        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            match self {
                property_state_snapshots_range::PropertyStateSnapshot(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
                property_state_snapshots_range::Uad36PropertyStateSnapshot(val) => Ok(val.into_pyobject(py).map(move |b| <pyo3::Bound<'_, _> as Clone>::clone(&b).into_any())?),
            }
        }
    }


    #[cfg(feature = "pyo3")]
    impl<'py> IntoPyObject<'py> for Box<property_state_snapshots_range>
    {
        type Target = PyAny;
        type Output = Bound<'py, Self::Target>;
        type Error = PyErr;
        fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
            (*self).into_pyobject(py).map(move |x| x.into_any())
        }
    }

    #[cfg(feature = "pyo3")]
    impl<'py> FromPyObject<'py> for Box<property_state_snapshots_range> {
        fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
            if let Ok(val) = ob.extract::<property_state_snapshots_range>() {
                return Ok(Box::new(val));
            }
            Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                "invalid property_state_snapshots",
            ))
        }
    }

    #[cfg(feature = "stubgen")]
    ::pyo3_stub_gen::impl_stub_type!(property_state_snapshots_range = PropertyStateSnapshot | Uad36PropertyStateSnapshot);
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct PropertyProfile {
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<Party>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<SourceArtifact>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub addresses: Option<Vec<Address>>,
    pub property: Property,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_addresses: Option<Vec<PropertyAddress>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub identifiers: Option<Vec<PropertyIdentifier>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub jurisdictions: Option<Vec<Jurisdiction>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcels: Option<Vec<Parcel>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_parcels: Option<Vec<PropertyParcel>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcel_lineage: Option<Vec<ParcelLineage>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub site: Option<Site>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub structures: Option<Vec<StructureOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub spaces: Option<Vec<Space>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state_snapshots: Option<Vec<PropertyStateSnapshotOrSubtype>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub associations: Option<Vec<PropertyAssociation>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessments: Option<Vec<Assessment>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tax_bills: Option<Vec<TaxBill>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub transfers: Option<Vec<Transfer>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub sales: Option<Vec<SaleEvent>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub listings: Option<Vec<Listing>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub leases: Option<Vec<LeaseEvent>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_rents: Option<Vec<UnitRentObservation>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent_rolls: Option<Vec<RentRoll>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub loans: Option<Vec<Loan>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub liens: Option<Vec<Lien>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub foreclosure_cases: Option<Vec<ForeclosureCase>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub permits: Option<Vec<Permit>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub ownership: Option<Vec<OwnershipPeriod>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub operating_statements: Option<Vec<OperatingStatement>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub valuations: Option<Vec<Valuation>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl PropertyProfile {
    #[new]
    #[pyo3(signature = (property, parties=None, artifacts=None, addresses=None, property_addresses=None, identifiers=None, jurisdictions=None, parcels=None, property_parcels=None, parcel_lineage=None, site=None, structures=None, spaces=None, property_state_snapshots=None, associations=None, assessments=None, tax_bills=None, transfers=None, sales=None, listings=None, leases=None, unit_rents=None, rent_rolls=None, loans=None, liens=None, foreclosure_cases=None, permits=None, ownership=None, operating_statements=None, valuations=None, provenance=None, extras=None))]
    pub fn new(property: serde_utils::PyValue<Property>, parties: Option<serde_utils::PyValue<Vec<Party>>>, artifacts: Option<serde_utils::PyValue<Vec<SourceArtifact>>>, addresses: Option<serde_utils::PyValue<Vec<Address>>>, property_addresses: Option<serde_utils::PyValue<Vec<PropertyAddress>>>, identifiers: Option<serde_utils::PyValue<Vec<PropertyIdentifier>>>, jurisdictions: Option<serde_utils::PyValue<Vec<Jurisdiction>>>, parcels: Option<serde_utils::PyValue<Vec<Parcel>>>, property_parcels: Option<serde_utils::PyValue<Vec<PropertyParcel>>>, parcel_lineage: Option<serde_utils::PyValue<Vec<ParcelLineage>>>, site: Option<serde_utils::PyValue<Site>>, structures: Option<serde_utils::PyValue<Vec<StructureOrSubtype>>>, spaces: Option<serde_utils::PyValue<Vec<Space>>>, property_state_snapshots: Option<serde_utils::PyValue<Vec<PropertyStateSnapshotOrSubtype>>>, associations: Option<serde_utils::PyValue<Vec<PropertyAssociation>>>, assessments: Option<serde_utils::PyValue<Vec<Assessment>>>, tax_bills: Option<serde_utils::PyValue<Vec<TaxBill>>>, transfers: Option<serde_utils::PyValue<Vec<Transfer>>>, sales: Option<serde_utils::PyValue<Vec<SaleEvent>>>, listings: Option<serde_utils::PyValue<Vec<Listing>>>, leases: Option<serde_utils::PyValue<Vec<LeaseEvent>>>, unit_rents: Option<serde_utils::PyValue<Vec<UnitRentObservation>>>, rent_rolls: Option<serde_utils::PyValue<Vec<RentRoll>>>, loans: Option<serde_utils::PyValue<Vec<Loan>>>, liens: Option<serde_utils::PyValue<Vec<Lien>>>, foreclosure_cases: Option<serde_utils::PyValue<Vec<ForeclosureCase>>>, permits: Option<serde_utils::PyValue<Vec<Permit>>>, ownership: Option<serde_utils::PyValue<Vec<OwnershipPeriod>>>, operating_statements: Option<serde_utils::PyValue<Vec<OperatingStatement>>>, valuations: Option<serde_utils::PyValue<Vec<Valuation>>>, provenance: Option<serde_utils::PyValue<Provenance>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let property = property.into_inner();
        let parties = parties.map(|v| v.into_inner());
        let artifacts = artifacts.map(|v| v.into_inner());
        let addresses = addresses.map(|v| v.into_inner());
        let property_addresses = property_addresses.map(|v| v.into_inner());
        let identifiers = identifiers.map(|v| v.into_inner());
        let jurisdictions = jurisdictions.map(|v| v.into_inner());
        let parcels = parcels.map(|v| v.into_inner());
        let property_parcels = property_parcels.map(|v| v.into_inner());
        let parcel_lineage = parcel_lineage.map(|v| v.into_inner());
        let site = site.map(|v| v.into_inner());
        let structures = structures.map(|v| v.into_inner());
        let spaces = spaces.map(|v| v.into_inner());
        let property_state_snapshots = property_state_snapshots.map(|v| v.into_inner());
        let associations = associations.map(|v| v.into_inner());
        let assessments = assessments.map(|v| v.into_inner());
        let tax_bills = tax_bills.map(|v| v.into_inner());
        let transfers = transfers.map(|v| v.into_inner());
        let sales = sales.map(|v| v.into_inner());
        let listings = listings.map(|v| v.into_inner());
        let leases = leases.map(|v| v.into_inner());
        let unit_rents = unit_rents.map(|v| v.into_inner());
        let rent_rolls = rent_rolls.map(|v| v.into_inner());
        let loans = loans.map(|v| v.into_inner());
        let liens = liens.map(|v| v.into_inner());
        let foreclosure_cases = foreclosure_cases.map(|v| v.into_inner());
        let permits = permits.map(|v| v.into_inner());
        let ownership = ownership.map(|v| v.into_inner());
        let operating_statements = operating_statements.map(|v| v.into_inner());
        let valuations = valuations.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        PropertyProfile{property, parties, artifacts, addresses, property_addresses, identifiers, jurisdictions, parcels, property_parcels, parcel_lineage, site, structures, spaces, property_state_snapshots, associations, assessments, tax_bills, transfers, sales, listings, leases, unit_rents, rent_rolls, loans, liens, foreclosure_cases, permits, ownership, operating_statements, valuations, provenance, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyProfile>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyProfile> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyProfile>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyProfile",
        ))
    }
}


#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature="serde", serde(untagged))]
pub enum PropertyProfileOrSubtype {    Uad36PropertyProfile(Uad36PropertyProfile)}

impl From<Uad36PropertyProfile>   for PropertyProfileOrSubtype { fn from(x: Uad36PropertyProfile)   -> Self { Self::Uad36PropertyProfile(x) } }

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for PropertyProfileOrSubtype {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36PropertyProfile>() {
            return Ok(PropertyProfileOrSubtype::Uad36PropertyProfile(val));
        }Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyProfileOrSubtype",
        ))
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for PropertyProfileOrSubtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            PropertyProfileOrSubtype::Uad36PropertyProfile(val) => val.into_pyobject(py).map(move |b| b.into_any()),
        }
    }
}


#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<PropertyProfileOrSubtype>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<PropertyProfileOrSubtype> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<PropertyProfileOrSubtype>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid PropertyProfileOrSubtype",
        ))
    }
}


#[cfg(feature = "stubgen")]
::pyo3_stub_gen::impl_stub_type!(PropertyProfileOrSubtype = Uad36PropertyProfile);

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct Uad36PropertyProfile {
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parties: Option<Vec<Party>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifacts: Option<Vec<SourceArtifact>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub addresses: Option<Vec<Address>>,
    pub property: Property,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_addresses: Option<Vec<PropertyAddress>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub identifiers: Option<Vec<PropertyIdentifier>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub jurisdictions: Option<Vec<Jurisdiction>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcels: Option<Vec<Parcel>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_parcels: Option<Vec<PropertyParcel>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub parcel_lineage: Option<Vec<ParcelLineage>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub site: Option<Site>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub structures: Option<Vec<Uad36Structure>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub spaces: Option<Vec<Space>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub property_state_snapshots: Option<Vec<Uad36PropertyStateSnapshot>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub associations: Option<Vec<PropertyAssociation>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessments: Option<Vec<Assessment>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub tax_bills: Option<Vec<TaxBill>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub transfers: Option<Vec<Transfer>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub sales: Option<Vec<SaleEvent>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub listings: Option<Vec<Listing>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub leases: Option<Vec<LeaseEvent>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub unit_rents: Option<Vec<UnitRentObservation>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub rent_rolls: Option<Vec<RentRoll>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub loans: Option<Vec<Loan>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub liens: Option<Vec<Lien>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub foreclosure_cases: Option<Vec<ForeclosureCase>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub permits: Option<Vec<Permit>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub ownership: Option<Vec<OwnershipPeriod>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub operating_statements: Option<Vec<OperatingStatement>>,
    #[cfg_attr(feature = "serde", serde(
        deserialize_with = "serde_utils::deserialize_inlined_dict_list_optional",
        serialize_with = "serde_utils::serialize_inlined_dict_list_optional"
    ))]
    #[cfg_attr(feature = "serde", serde(default))]
    pub valuations: Option<Vec<Valuation>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub provenance: Option<Provenance>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl Uad36PropertyProfile {
    #[new]
    #[pyo3(signature = (property, parties=None, artifacts=None, addresses=None, property_addresses=None, identifiers=None, jurisdictions=None, parcels=None, property_parcels=None, parcel_lineage=None, site=None, structures=None, spaces=None, property_state_snapshots=None, associations=None, assessments=None, tax_bills=None, transfers=None, sales=None, listings=None, leases=None, unit_rents=None, rent_rolls=None, loans=None, liens=None, foreclosure_cases=None, permits=None, ownership=None, operating_statements=None, valuations=None, provenance=None, extras=None))]
    pub fn new(property: serde_utils::PyValue<Property>, parties: Option<serde_utils::PyValue<Vec<Party>>>, artifacts: Option<serde_utils::PyValue<Vec<SourceArtifact>>>, addresses: Option<serde_utils::PyValue<Vec<Address>>>, property_addresses: Option<serde_utils::PyValue<Vec<PropertyAddress>>>, identifiers: Option<serde_utils::PyValue<Vec<PropertyIdentifier>>>, jurisdictions: Option<serde_utils::PyValue<Vec<Jurisdiction>>>, parcels: Option<serde_utils::PyValue<Vec<Parcel>>>, property_parcels: Option<serde_utils::PyValue<Vec<PropertyParcel>>>, parcel_lineage: Option<serde_utils::PyValue<Vec<ParcelLineage>>>, site: Option<serde_utils::PyValue<Site>>, structures: Option<serde_utils::PyValue<Vec<Uad36Structure>>>, spaces: Option<serde_utils::PyValue<Vec<Space>>>, property_state_snapshots: Option<serde_utils::PyValue<Vec<Uad36PropertyStateSnapshot>>>, associations: Option<serde_utils::PyValue<Vec<PropertyAssociation>>>, assessments: Option<serde_utils::PyValue<Vec<Assessment>>>, tax_bills: Option<serde_utils::PyValue<Vec<TaxBill>>>, transfers: Option<serde_utils::PyValue<Vec<Transfer>>>, sales: Option<serde_utils::PyValue<Vec<SaleEvent>>>, listings: Option<serde_utils::PyValue<Vec<Listing>>>, leases: Option<serde_utils::PyValue<Vec<LeaseEvent>>>, unit_rents: Option<serde_utils::PyValue<Vec<UnitRentObservation>>>, rent_rolls: Option<serde_utils::PyValue<Vec<RentRoll>>>, loans: Option<serde_utils::PyValue<Vec<Loan>>>, liens: Option<serde_utils::PyValue<Vec<Lien>>>, foreclosure_cases: Option<serde_utils::PyValue<Vec<ForeclosureCase>>>, permits: Option<serde_utils::PyValue<Vec<Permit>>>, ownership: Option<serde_utils::PyValue<Vec<OwnershipPeriod>>>, operating_statements: Option<serde_utils::PyValue<Vec<OperatingStatement>>>, valuations: Option<serde_utils::PyValue<Vec<Valuation>>>, provenance: Option<serde_utils::PyValue<Provenance>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let property = property.into_inner();
        let parties = parties.map(|v| v.into_inner());
        let artifacts = artifacts.map(|v| v.into_inner());
        let addresses = addresses.map(|v| v.into_inner());
        let property_addresses = property_addresses.map(|v| v.into_inner());
        let identifiers = identifiers.map(|v| v.into_inner());
        let jurisdictions = jurisdictions.map(|v| v.into_inner());
        let parcels = parcels.map(|v| v.into_inner());
        let property_parcels = property_parcels.map(|v| v.into_inner());
        let parcel_lineage = parcel_lineage.map(|v| v.into_inner());
        let site = site.map(|v| v.into_inner());
        let structures = structures.map(|v| v.into_inner());
        let spaces = spaces.map(|v| v.into_inner());
        let property_state_snapshots = property_state_snapshots.map(|v| v.into_inner());
        let associations = associations.map(|v| v.into_inner());
        let assessments = assessments.map(|v| v.into_inner());
        let tax_bills = tax_bills.map(|v| v.into_inner());
        let transfers = transfers.map(|v| v.into_inner());
        let sales = sales.map(|v| v.into_inner());
        let listings = listings.map(|v| v.into_inner());
        let leases = leases.map(|v| v.into_inner());
        let unit_rents = unit_rents.map(|v| v.into_inner());
        let rent_rolls = rent_rolls.map(|v| v.into_inner());
        let loans = loans.map(|v| v.into_inner());
        let liens = liens.map(|v| v.into_inner());
        let foreclosure_cases = foreclosure_cases.map(|v| v.into_inner());
        let permits = permits.map(|v| v.into_inner());
        let ownership = ownership.map(|v| v.into_inner());
        let operating_statements = operating_statements.map(|v| v.into_inner());
        let valuations = valuations.map(|v| v.into_inner());
        let provenance = provenance.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        Uad36PropertyProfile{property, parties, artifacts, addresses, property_addresses, identifiers, jurisdictions, parcels, property_parcels, parcel_lineage, site, structures, spaces, property_state_snapshots, associations, assessments, tax_bills, transfers, sales, listings, leases, unit_rents, rent_rolls, loans, liens, foreclosure_cases, permits, ownership, operating_statements, valuations, provenance, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<Uad36PropertyProfile>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<Uad36PropertyProfile> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<Uad36PropertyProfile>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid Uad36PropertyProfile",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct AssessorObservation {
    pub status: AssessorStatus,
    #[cfg_attr(feature = "serde", serde(default))]
    pub query_address: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub query_parcel_number: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub assessor_url: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub profile: Option<PropertyProfileOrSubtype>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub error: Option<String>,
    pub provenance: Provenance,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifact_refs: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl AssessorObservation {
    #[new]
    #[pyo3(signature = (status, provenance, query_address=None, query_parcel_number=None, assessor_url=None, profile=None, error=None, artifact_refs=None, extras=None))]
    pub fn new(status: AssessorStatus, provenance: serde_utils::PyValue<Provenance>, query_address: Option<String>, query_parcel_number: Option<String>, assessor_url: Option<uri>, profile: Option<serde_utils::PyValue<PropertyProfileOrSubtype>>, error: Option<String>, artifact_refs: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let provenance = provenance.into_inner();
        let profile = profile.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        AssessorObservation{status, provenance, query_address, query_parcel_number, assessor_url, profile, error, artifact_refs, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<AssessorObservation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<AssessorObservation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<AssessorObservation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid AssessorObservation",
        ))
    }
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
#[cfg_attr(feature = "stubgen", gen_stub_pyclass)]
#[cfg_attr(feature = "pyo3", pyclass(subclass, get_all, set_all))]
pub struct ExtractionObservation {
    pub status: ExtractionStatus,
    #[cfg_attr(feature = "serde", serde(default))]
    pub category: Option<ExtractionCategory>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_category: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub error: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub source_url: Option<uri>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extracted_at: Option<DateTime<FixedOffset>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub model: Option<String>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub profile: Option<PropertyProfileOrSubtype>,
    pub provenance: Provenance,
    #[cfg_attr(feature = "serde", serde(default))]
    pub artifact_refs: Option<Vec<String>>,
    #[cfg_attr(feature = "serde", serde(default))]
    pub extras: Option<Any>
}
#[cfg(feature = "pyo3")]
#[cfg_attr(feature = "stubgen", gen_stub_pymethods)]
#[pymethods]
impl ExtractionObservation {
    #[new]
    #[pyo3(signature = (status, provenance, category=None, source_category=None, error=None, source_url=None, extracted_at=None, model=None, profile=None, artifact_refs=None, extras=None))]
    pub fn new(status: ExtractionStatus, provenance: serde_utils::PyValue<Provenance>, category: Option<ExtractionCategory>, source_category: Option<String>, error: Option<String>, source_url: Option<uri>, extracted_at: Option<DateTime<FixedOffset>>, model: Option<String>, profile: Option<serde_utils::PyValue<PropertyProfileOrSubtype>>, artifact_refs: Option<Vec<String>>, extras: Option<serde_utils::PyValue<Any>>) -> Self {
        let provenance = provenance.into_inner();
        let profile = profile.map(|v| v.into_inner());
        let extras = extras.map(|v| v.into_inner());
        ExtractionObservation{status, provenance, category, source_category, error, source_url, extracted_at, model, profile, artifact_refs, extras}
    }
}

#[cfg(feature = "pyo3")]
impl<'py> IntoPyObject<'py> for Box<ExtractionObservation>
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (*self).into_pyobject(py).map(move |x| x.into_any())
    }
}

#[cfg(feature = "pyo3")]
impl<'py> FromPyObject<'py> for Box<ExtractionObservation> {
    fn extract_bound(ob: &pyo3::Bound<'py, pyo3::types::PyAny>) -> pyo3::PyResult<Self> {
        if let Ok(val) = ob.extract::<ExtractionObservation>() {
            return Ok(Box::new(val));
        }
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "invalid ExtractionObservation",
        ))
    }
}






#[cfg(feature = "stubgen")]
define_stub_info_gatherer!(stub_info);
