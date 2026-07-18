// Canonical wire-format conformance for generated enums. Lives outside src/
// so `gen-rust --force` (which rewrites src/, Cargo.toml, pyproject.toml)
// never touches it.
//
// The exhaustive test below enumerates every permissible value of every
// vocabulary enum in phds.schema.json (137 variants across 23 enums);
// tests/test_generated_contract.py census-checks that this stays in sync
// with the schema.
#![cfg(feature = "serde")]

use phds_profiles::*;

fn assert_round_trip<T>(value: T, wire: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug,
{
    let encoded = serde_yml::to_string(&value).expect("serialize");
    assert_eq!(encoded.trim(), wire, "serialize {value:?}");
    let decoded: T = serde_yml::from_str(wire).expect("deserialize");
    assert_eq!(decoded, value, "deserialize {wire:?}");
}

#[test]
fn assessor_status_round_trips_canonical_wire_values() {
    assert_round_trip(AssessorStatus::Success, "success");
    assert_round_trip(AssessorStatus::NotFound, "not_found");
    assert_round_trip(AssessorStatus::Timeout, "timeout");
    assert_round_trip(AssessorStatus::ApiError, "api_error");
    assert_round_trip(AssessorStatus::ParseError, "parse_error");
    assert_round_trip(AssessorStatus::InvalidAddress, "invalid_address");
    assert_round_trip(AssessorStatus::Ambiguous, "ambiguous");
}

#[test]
fn all_enum_variants_round_trip_canonical_wire_values() {
    assert_round_trip(AreaUnit::Sqft, "sqft");
    assert_round_trip(AreaUnit::Sqm, "sqm");
    assert_round_trip(AreaUnit::Acre, "acre");
    assert_round_trip(AreaUnit::Hectare, "hectare");
    assert_round_trip(AssessorStatus::Success, "success");
    assert_round_trip(AssessorStatus::NotFound, "not_found");
    assert_round_trip(AssessorStatus::Timeout, "timeout");
    assert_round_trip(AssessorStatus::ApiError, "api_error");
    assert_round_trip(AssessorStatus::ParseError, "parse_error");
    assert_round_trip(AssessorStatus::InvalidAddress, "invalid_address");
    assert_round_trip(AssessorStatus::Ambiguous, "ambiguous");
    assert_round_trip(CaptureMethod::Api, "api");
    assert_round_trip(CaptureMethod::Scrape, "scrape");
    assert_round_trip(CaptureMethod::LlmExtraction, "llm_extraction");
    assert_round_trip(CaptureMethod::Manual, "manual");
    assert_round_trip(CaptureMethod::Bulk, "bulk");
    assert_round_trip(EstateType::FeeSimple, "fee_simple");
    assert_round_trip(EstateType::Leasehold, "leasehold");
    assert_round_trip(EstateType::LifeEstate, "life_estate");
    assert_round_trip(EstateType::CooperativeShares, "cooperative_shares");
    assert_round_trip(EstateType::Other, "other");
    assert_round_trip(GeocodeAccuracy::Rooftop, "rooftop");
    assert_round_trip(GeocodeAccuracy::Parcel, "parcel");
    assert_round_trip(GeocodeAccuracy::Street, "street");
    assert_round_trip(GeocodeAccuracy::PostalCentroid, "postal_centroid");
    assert_round_trip(GeocodeAccuracy::LocalityCentroid, "locality_centroid");
    assert_round_trip(GeocodeAccuracy::Manual, "manual");
    assert_round_trip(GeocodeAccuracy::Unknown, "unknown");
    assert_round_trip(LeaseTypeEnum::Gross, "gross");
    assert_round_trip(LeaseTypeEnum::ModifiedGross, "modified_gross");
    assert_round_trip(LeaseTypeEnum::TripleNet, "triple_net");
    assert_round_trip(LeaseTypeEnum::DoubleNet, "double_net");
    assert_round_trip(LeaseTypeEnum::SingleNet, "single_net");
    assert_round_trip(LeaseTypeEnum::AbsoluteNet, "absolute_net");
    assert_round_trip(LeaseTypeEnum::Percentage, "percentage");
    assert_round_trip(LeaseTypeEnum::Ground, "ground");
    assert_round_trip(LeaseTypeEnum::Residential, "residential");
    assert_round_trip(LeaseTypeEnum::Other, "other");
    assert_round_trip(LengthUnit::Ft, "ft");
    assert_round_trip(LengthUnit::M, "m");
    assert_round_trip(LienKind::Tax, "tax");
    assert_round_trip(LienKind::Judgment, "judgment");
    assert_round_trip(LienKind::Hoa, "hoa");
    assert_round_trip(LienKind::Mechanics, "mechanics");
    assert_round_trip(LienKind::Municipal, "municipal");
    assert_round_trip(LienKind::Other, "other");
    assert_round_trip(ListingKind::ForSale, "for_sale");
    assert_round_trip(ListingKind::ForLease, "for_lease");
    assert_round_trip(ListingStatus::Active, "active");
    assert_round_trip(ListingStatus::Pending, "pending");
    assert_round_trip(ListingStatus::Sold, "sold");
    assert_round_trip(ListingStatus::Leased, "leased");
    assert_round_trip(ListingStatus::Withdrawn, "withdrawn");
    assert_round_trip(ListingStatus::Expired, "expired");
    assert_round_trip(ListingStatus::ComingSoon, "coming_soon");
    assert_round_trip(ListingStatus::Other, "other");
    assert_round_trip(LoanEventKind::Origination, "origination");
    assert_round_trip(LoanEventKind::Assignment, "assignment");
    assert_round_trip(LoanEventKind::Modification, "modification");
    assert_round_trip(LoanEventKind::Satisfaction, "satisfaction");
    assert_round_trip(LoanEventKind::Release, "release");
    assert_round_trip(LoanEventKind::Default, "default");
    assert_round_trip(LoanEventKind::Reinstatement, "reinstatement");
    assert_round_trip(LoanEventKind::Other, "other");
    assert_round_trip(LoanStatus::Active, "active");
    assert_round_trip(LoanStatus::Satisfied, "satisfied");
    assert_round_trip(LoanStatus::Assigned, "assigned");
    assert_round_trip(LoanStatus::Foreclosure, "foreclosure");
    assert_round_trip(LoanStatus::Released, "released");
    assert_round_trip(LoanStatus::Unknown, "unknown");
    assert_round_trip(OrganizationKind::Llc, "llc");
    assert_round_trip(OrganizationKind::Corporation, "corporation");
    assert_round_trip(OrganizationKind::Partnership, "partnership");
    assert_round_trip(OrganizationKind::Trust, "trust");
    assert_round_trip(OrganizationKind::Estate, "estate");
    assert_round_trip(OrganizationKind::Government, "government");
    assert_round_trip(OrganizationKind::Nonprofit, "nonprofit");
    assert_round_trip(OrganizationKind::Reit, "reit");
    assert_round_trip(OrganizationKind::Fund, "fund");
    assert_round_trip(OrganizationKind::Lender, "lender");
    assert_round_trip(OrganizationKind::Brokerage, "brokerage");
    assert_round_trip(OrganizationKind::Hoa, "hoa");
    assert_round_trip(OrganizationKind::Other, "other");
    assert_round_trip(ParcelLineageKind::Split, "split");
    assert_round_trip(ParcelLineageKind::Merge, "merge");
    assert_round_trip(ParcelLineageKind::Renumber, "renumber");
    assert_round_trip(PartyKind::Person, "person");
    assert_round_trip(PartyKind::Organization, "organization");
    assert_round_trip(PriceDisclosure::Full, "full");
    assert_round_trip(PriceDisclosure::Partial, "partial");
    assert_round_trip(PriceDisclosure::Estimated, "estimated");
    assert_round_trip(PriceDisclosure::Nominal, "nominal");
    assert_round_trip(PriceDisclosure::NonDisclosure, "non_disclosure");
    assert_round_trip(PriceDisclosure::Unknown, "unknown");
    assert_round_trip(RateBasis::PerUnit, "per_unit");
    assert_round_trip(RateBasis::PerBed, "per_bed");
    assert_round_trip(RateBasis::PerArea, "per_area");
    assert_round_trip(RateBasis::PerRoom, "per_room");
    assert_round_trip(RateBasis::PerKey, "per_key");
    assert_round_trip(RateBasis::PerSlip, "per_slip");
    assert_round_trip(RateBasis::PerStall, "per_stall");
    assert_round_trip(RateBasis::PerPad, "per_pad");
    assert_round_trip(RateBasis::Other, "other");
    assert_round_trip(RateType::Asking, "asking");
    assert_round_trip(RateType::Effective, "effective");
    assert_round_trip(RateType::Contract, "contract");
    assert_round_trip(RentPeriod::Daily, "daily");
    assert_round_trip(RentPeriod::Monthly, "monthly");
    assert_round_trip(RentPeriod::Annual, "annual");
    assert_round_trip(RentPeriod::PerAreaAnnual, "per_area_annual");
    assert_round_trip(RentPeriod::PerAreaMonthly, "per_area_monthly");
    assert_round_trip(SaleTypeEnum::ArmsLength, "arms_length");
    assert_round_trip(SaleTypeEnum::Reo, "reo");
    assert_round_trip(SaleTypeEnum::ShortSale, "short_sale");
    assert_round_trip(SaleTypeEnum::Auction, "auction");
    assert_round_trip(SaleTypeEnum::RelatedParty, "related_party");
    assert_round_trip(SaleTypeEnum::Portfolio, "portfolio");
    assert_round_trip(SaleTypeEnum::PartialInterest, "partial_interest");
    assert_round_trip(SaleTypeEnum::LandContract, "land_contract");
    assert_round_trip(SaleTypeEnum::NewConstruction, "new_construction");
    assert_round_trip(SaleTypeEnum::Other, "other");
    assert_round_trip(StatementBasis::Actual, "actual");
    assert_round_trip(StatementBasis::Budget, "budget");
    assert_round_trip(StatementBasis::ProForma, "pro_forma");
    assert_round_trip(StatementBasis::Stabilized, "stabilized");
    assert_round_trip(StatementBasis::Projected, "projected");
    assert_round_trip(StatementBasis::Other, "other");
    assert_round_trip(ValuationKind::Avm, "avm");
    assert_round_trip(ValuationKind::Appraisal, "appraisal");
    assert_round_trip(ValuationKind::Bpo, "bpo");
    assert_round_trip(ValuationKind::BrokerOpinion, "broker_opinion");
    assert_round_trip(ValuationKind::Internal, "internal");
    assert_round_trip(VerificationStatus::Unverified, "unverified");
    assert_round_trip(VerificationStatus::PendingReview, "pending_review");
    assert_round_trip(VerificationStatus::Verified, "verified");
    assert_round_trip(VerificationStatus::Disputed, "disputed");
    assert_round_trip(VerificationStatus::Rejected, "rejected");
}
