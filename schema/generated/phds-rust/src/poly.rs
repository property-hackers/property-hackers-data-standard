#![allow(non_camel_case_types)]

use crate::*;
use crate::poly_containers::*;


pub trait Any   {


}

impl Any for crate::Any {
}


pub trait Money   {

    fn amount<'a>(&'a self) -> &'a str;
    // fn amount_mut(&mut self) -> &mut &'a str;
    // fn set_amount(&mut self, value: String);

    fn currency<'a>(&'a self) -> &'a str;
    // fn currency_mut(&mut self) -> &mut &'a str;
    // fn set_currency(&mut self, value: String);


}

impl Money for crate::Money {
        fn amount<'a>(&'a self) -> &'a str {
        return &self.amount[..];
    }
        fn currency<'a>(&'a self) -> &'a str {
        return &self.currency[..];
    }
}


pub trait Area   {

    fn value(&self) -> f64;
    // fn value_mut(&mut self) -> &mut f64;
    // fn set_value(&mut self, value: f64);

    fn unit<'a>(&'a self) -> &'a crate::AreaUnit;
    // fn unit_mut(&mut self) -> &mut &'a crate::AreaUnit;
    // fn set_unit(&mut self, value: AreaUnit);


}

impl Area for crate::Area {
        fn value(&self) -> f64 {
        return self.value;
    }
        fn unit<'a>(&'a self) -> &'a crate::AreaUnit {
        return &self.unit;
    }
}


pub trait Length   {

    fn value(&self) -> f64;
    // fn value_mut(&mut self) -> &mut f64;
    // fn set_value(&mut self, value: f64);

    fn unit<'a>(&'a self) -> &'a crate::LengthUnit;
    // fn unit_mut(&mut self) -> &mut &'a crate::LengthUnit;
    // fn set_unit(&mut self, value: LengthUnit);


}

impl Length for crate::Length {
        fn value(&self) -> f64 {
        return self.value;
    }
        fn unit<'a>(&'a self) -> &'a crate::LengthUnit {
        return &self.unit;
    }
}


pub trait UnitRate   {

    fn amount<'a>(&'a self) -> &'a str;
    // fn amount_mut(&mut self) -> &mut &'a str;
    // fn set_amount(&mut self, value: String);

    fn currency<'a>(&'a self) -> &'a str;
    // fn currency_mut(&mut self) -> &mut &'a str;
    // fn set_currency(&mut self, value: String);

    fn denominator<'a>(&'a self) -> &'a str;
    // fn denominator_mut(&mut self) -> &mut &'a str;
    // fn set_denominator(&mut self, value: String);


}

impl UnitRate for crate::UnitRate {
        fn amount<'a>(&'a self) -> &'a str {
        return &self.amount[..];
    }
        fn currency<'a>(&'a self) -> &'a str {
        return &self.currency[..];
    }
        fn denominator<'a>(&'a self) -> &'a str {
        return &self.denominator[..];
    }
}


pub trait CodeableConcept   {

    fn system<'a>(&'a self) -> Option<&'a str>;
    // fn system_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_system(&mut self, value: Option<&'a str>);

    fn code<'a>(&'a self) -> Option<&'a str>;
    // fn code_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_code(&mut self, value: Option<&'a str>);

    fn display<'a>(&'a self) -> Option<&'a str>;
    // fn display_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_display(&mut self, value: Option<&'a str>);


}

impl CodeableConcept for crate::CodeableConcept {
        fn system<'a>(&'a self) -> Option<&'a str> {
        return self.system.as_deref();
    }
        fn code<'a>(&'a self) -> Option<&'a str> {
        return self.code.as_deref();
    }
        fn display<'a>(&'a self) -> Option<&'a str> {
        return self.display.as_deref();
    }
}
impl CodeableConcept for crate::Classification {
        fn system<'a>(&'a self) -> Option<&'a str> {
        return Some(&self.system);
    }
        fn code<'a>(&'a self) -> Option<&'a str> {
        return Some(&self.code);
    }
        fn display<'a>(&'a self) -> Option<&'a str> {
        return self.display.as_deref();
    }
}
impl CodeableConcept for crate::Rating {
        fn system<'a>(&'a self) -> Option<&'a str> {
        return Some(&self.system);
    }
        fn code<'a>(&'a self) -> Option<&'a str> {
        return Some(&self.code);
    }
        fn display<'a>(&'a self) -> Option<&'a str> {
        return self.display.as_deref();
    }
}

impl CodeableConcept for crate::CodeableConceptOrSubtype {
        fn system<'a>(&'a self) -> Option<&'a str> {
        match self {
                CodeableConceptOrSubtype::Classification(val) => val.system(),
                CodeableConceptOrSubtype::Rating(val) => val.system(),
                CodeableConceptOrSubtype::CodeableConcept(val) => val.system(),

        }
    }
        fn code<'a>(&'a self) -> Option<&'a str> {
        match self {
                CodeableConceptOrSubtype::Classification(val) => val.code(),
                CodeableConceptOrSubtype::Rating(val) => val.code(),
                CodeableConceptOrSubtype::CodeableConcept(val) => val.code(),

        }
    }
        fn display<'a>(&'a self) -> Option<&'a str> {
        match self {
                CodeableConceptOrSubtype::Classification(val) => val.display(),
                CodeableConceptOrSubtype::Rating(val) => val.display(),
                CodeableConceptOrSubtype::CodeableConcept(val) => val.display(),

        }
    }
}

pub trait Classification : CodeableConcept   {


}

impl Classification for crate::Classification {
}


pub trait Rating : CodeableConcept   {

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn scope<'a>(&'a self) -> Option<&'a crate::RatingScope>;
    // fn scope_mut(&mut self) -> &mut Option<&'a crate::RatingScope>;
    // fn set_scope(&mut self, value: Option<&'a RatingScope>);


}

impl Rating for crate::Rating {
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn scope<'a>(&'a self) -> Option<&'a crate::RatingScope> {
        return self.scope.as_ref();
    }
}


pub trait GeoPoint   {

    fn latitude(&self) -> f64;
    // fn latitude_mut(&mut self) -> &mut f64;
    // fn set_latitude(&mut self, value: f64);

    fn longitude(&self) -> f64;
    // fn longitude_mut(&mut self) -> &mut f64;
    // fn set_longitude(&mut self, value: f64);


}

impl GeoPoint for crate::GeoPoint {
        fn latitude(&self) -> f64 {
        return self.latitude;
    }
        fn longitude(&self) -> f64 {
        return self.longitude;
    }
}


pub trait Geometry   {

    fn type_<'a>(&'a self) -> &'a str;
    // fn type__mut(&mut self) -> &mut &'a str;
    // fn set_type_(&mut self, value: String);

    fn coordinates<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn coordinates_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_coordinates<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl Geometry for crate::Geometry {
        fn type_<'a>(&'a self) -> &'a str {
        return &self.type_[..];
    }
        fn coordinates<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.coordinates.as_ref();
    }
}


pub trait Provenance   {

    fn provider<'a>(&'a self) -> Option<&'a str>;
    // fn provider_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_provider(&mut self, value: Option<&'a str>);

    fn source_url<'a>(&'a self) -> Option<&'a crate::uri>;
    // fn source_url_mut(&mut self) -> &mut Option<&'a crate::uri>;
    // fn set_source_url(&mut self, value: Option<&'a uri>);

    fn retrieved_at<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>>;
    // fn retrieved_at_mut(&mut self) -> &mut Option<&'a crate::DateTime<FixedOffset>>;
    // fn set_retrieved_at(&mut self, value: Option<&'a DateTime<FixedOffset>>);

    fn method<'a>(&'a self) -> Option<&'a crate::CaptureMethod>;
    // fn method_mut(&mut self) -> &mut Option<&'a crate::CaptureMethod>;
    // fn set_method(&mut self, value: Option<&'a CaptureMethod>);

    fn confidence(&self) -> Option<f64>;
    // fn confidence_mut(&mut self) -> &mut Option<f64>;
    // fn set_confidence(&mut self, value: Option<f64>);

    fn verification<'a>(&'a self) -> Option<&'a crate::VerificationStatus>;
    // fn verification_mut(&mut self) -> &mut Option<&'a crate::VerificationStatus>;
    // fn set_verification(&mut self, value: Option<&'a VerificationStatus>);


}

impl Provenance for crate::Provenance {
        fn provider<'a>(&'a self) -> Option<&'a str> {
        return self.provider.as_deref();
    }
        fn source_url<'a>(&'a self) -> Option<&'a crate::uri> {
        return self.source_url.as_ref();
    }
        fn retrieved_at<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>> {
        return self.retrieved_at.as_ref();
    }
        fn method<'a>(&'a self) -> Option<&'a crate::CaptureMethod> {
        return self.method.as_ref();
    }
        fn confidence(&self) -> Option<f64> {
        return self.confidence;
    }
        fn verification<'a>(&'a self) -> Option<&'a crate::VerificationStatus> {
        return self.verification.as_ref();
    }
}


pub trait Entity   {

    fn id<'a>(&'a self) -> &'a str;
    // fn id_mut(&mut self) -> &mut &'a str;
    // fn set_id(&mut self, value: String);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;

    fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>>;
    // fn verifications_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>>;
    // fn set_verifications<E>(&mut self, value: Option<&Vec<E>>) where E: Into<VerificationAttribution>;


}

impl Entity for crate::Entity {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Jurisdiction {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Address {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref().map(|x| poly_containers::ListView::new(x));
    }
}
impl Entity for crate::Property {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Parcel {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyParcel {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::ParcelLineage {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyIdentifier {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Party {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref().map(|x| poly_containers::ListView::new(x));
    }
}
impl Entity for crate::SourceArtifact {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyAddress {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::OwnershipPeriod {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Structure {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Site {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Space {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyState {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::SiteState {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::StructureState {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::SpaceState {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyStateSnapshot {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::PropertyAssociation {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Assessment {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::TaxBill {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Transfer {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::SaleEvent {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Listing {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::LeaseEvent {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::UnitRentObservation {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Loan {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Lien {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::ForeclosureCase {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Permit {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::OperatingStatement {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::RentRoll {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}
impl Entity for crate::Valuation {
        fn id<'a>(&'a self) -> &'a str {
        return &self.id[..];
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        return self.verifications.as_ref();
    }
}

impl Entity for crate::EntityOrSubtype {
        fn id<'a>(&'a self) -> &'a str {
        match self {
                EntityOrSubtype::Jurisdiction(val) => val.id(),
                EntityOrSubtype::Address(val) => val.id(),
                EntityOrSubtype::Property(val) => val.id(),
                EntityOrSubtype::Parcel(val) => val.id(),
                EntityOrSubtype::PropertyParcel(val) => val.id(),
                EntityOrSubtype::ParcelLineage(val) => val.id(),
                EntityOrSubtype::PropertyIdentifier(val) => val.id(),
                EntityOrSubtype::Party(val) => val.id(),
                EntityOrSubtype::SourceArtifact(val) => val.id(),
                EntityOrSubtype::PropertyAddress(val) => val.id(),
                EntityOrSubtype::OwnershipPeriod(val) => val.id(),
                EntityOrSubtype::Structure(val) => val.id(),
                EntityOrSubtype::Site(val) => val.id(),
                EntityOrSubtype::Space(val) => val.id(),
                EntityOrSubtype::PropertyState(val) => val.id(),
                EntityOrSubtype::SiteState(val) => val.id(),
                EntityOrSubtype::StructureState(val) => val.id(),
                EntityOrSubtype::SpaceState(val) => val.id(),
                EntityOrSubtype::PropertyStateSnapshot(val) => val.id(),
                EntityOrSubtype::PropertyAssociation(val) => val.id(),
                EntityOrSubtype::Assessment(val) => val.id(),
                EntityOrSubtype::TaxBill(val) => val.id(),
                EntityOrSubtype::Transfer(val) => val.id(),
                EntityOrSubtype::SaleEvent(val) => val.id(),
                EntityOrSubtype::Listing(val) => val.id(),
                EntityOrSubtype::LeaseEvent(val) => val.id(),
                EntityOrSubtype::UnitRentObservation(val) => val.id(),
                EntityOrSubtype::Loan(val) => val.id(),
                EntityOrSubtype::Lien(val) => val.id(),
                EntityOrSubtype::ForeclosureCase(val) => val.id(),
                EntityOrSubtype::Permit(val) => val.id(),
                EntityOrSubtype::OperatingStatement(val) => val.id(),
                EntityOrSubtype::RentRoll(val) => val.id(),
                EntityOrSubtype::Valuation(val) => val.id(),

        }
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        match self {
                EntityOrSubtype::Jurisdiction(val) => val.extras(),
                EntityOrSubtype::Address(val) => val.extras(),
                EntityOrSubtype::Property(val) => val.extras(),
                EntityOrSubtype::Parcel(val) => val.extras(),
                EntityOrSubtype::PropertyParcel(val) => val.extras(),
                EntityOrSubtype::ParcelLineage(val) => val.extras(),
                EntityOrSubtype::PropertyIdentifier(val) => val.extras(),
                EntityOrSubtype::Party(val) => val.extras(),
                EntityOrSubtype::SourceArtifact(val) => val.extras(),
                EntityOrSubtype::PropertyAddress(val) => val.extras(),
                EntityOrSubtype::OwnershipPeriod(val) => val.extras(),
                EntityOrSubtype::Structure(val) => val.extras(),
                EntityOrSubtype::Site(val) => val.extras(),
                EntityOrSubtype::Space(val) => val.extras(),
                EntityOrSubtype::PropertyState(val) => val.extras(),
                EntityOrSubtype::SiteState(val) => val.extras(),
                EntityOrSubtype::StructureState(val) => val.extras(),
                EntityOrSubtype::SpaceState(val) => val.extras(),
                EntityOrSubtype::PropertyStateSnapshot(val) => val.extras(),
                EntityOrSubtype::PropertyAssociation(val) => val.extras(),
                EntityOrSubtype::Assessment(val) => val.extras(),
                EntityOrSubtype::TaxBill(val) => val.extras(),
                EntityOrSubtype::Transfer(val) => val.extras(),
                EntityOrSubtype::SaleEvent(val) => val.extras(),
                EntityOrSubtype::Listing(val) => val.extras(),
                EntityOrSubtype::LeaseEvent(val) => val.extras(),
                EntityOrSubtype::UnitRentObservation(val) => val.extras(),
                EntityOrSubtype::Loan(val) => val.extras(),
                EntityOrSubtype::Lien(val) => val.extras(),
                EntityOrSubtype::ForeclosureCase(val) => val.extras(),
                EntityOrSubtype::Permit(val) => val.extras(),
                EntityOrSubtype::OperatingStatement(val) => val.extras(),
                EntityOrSubtype::RentRoll(val) => val.extras(),
                EntityOrSubtype::Valuation(val) => val.extras(),

        }
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        match self {
                EntityOrSubtype::Jurisdiction(val) => val.provenance(),
                EntityOrSubtype::Address(val) => val.provenance(),
                EntityOrSubtype::Property(val) => val.provenance(),
                EntityOrSubtype::Parcel(val) => val.provenance(),
                EntityOrSubtype::PropertyParcel(val) => val.provenance(),
                EntityOrSubtype::ParcelLineage(val) => val.provenance(),
                EntityOrSubtype::PropertyIdentifier(val) => val.provenance(),
                EntityOrSubtype::Party(val) => val.provenance(),
                EntityOrSubtype::SourceArtifact(val) => val.provenance(),
                EntityOrSubtype::PropertyAddress(val) => val.provenance(),
                EntityOrSubtype::OwnershipPeriod(val) => val.provenance(),
                EntityOrSubtype::Structure(val) => val.provenance(),
                EntityOrSubtype::Site(val) => val.provenance(),
                EntityOrSubtype::Space(val) => val.provenance(),
                EntityOrSubtype::PropertyState(val) => val.provenance(),
                EntityOrSubtype::SiteState(val) => val.provenance(),
                EntityOrSubtype::StructureState(val) => val.provenance(),
                EntityOrSubtype::SpaceState(val) => val.provenance(),
                EntityOrSubtype::PropertyStateSnapshot(val) => val.provenance(),
                EntityOrSubtype::PropertyAssociation(val) => val.provenance(),
                EntityOrSubtype::Assessment(val) => val.provenance(),
                EntityOrSubtype::TaxBill(val) => val.provenance(),
                EntityOrSubtype::Transfer(val) => val.provenance(),
                EntityOrSubtype::SaleEvent(val) => val.provenance(),
                EntityOrSubtype::Listing(val) => val.provenance(),
                EntityOrSubtype::LeaseEvent(val) => val.provenance(),
                EntityOrSubtype::UnitRentObservation(val) => val.provenance(),
                EntityOrSubtype::Loan(val) => val.provenance(),
                EntityOrSubtype::Lien(val) => val.provenance(),
                EntityOrSubtype::ForeclosureCase(val) => val.provenance(),
                EntityOrSubtype::Permit(val) => val.provenance(),
                EntityOrSubtype::OperatingStatement(val) => val.provenance(),
                EntityOrSubtype::RentRoll(val) => val.provenance(),
                EntityOrSubtype::Valuation(val) => val.provenance(),

        }
    }
        fn verifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::VerificationAttribution>> {
        match self {
                EntityOrSubtype::Jurisdiction(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Address(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Property(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Parcel(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyParcel(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::ParcelLineage(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyIdentifier(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Party(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::SourceArtifact(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyAddress(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::OwnershipPeriod(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Structure(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Site(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Space(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyState(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::SiteState(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::StructureState(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::SpaceState(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyStateSnapshot(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::PropertyAssociation(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Assessment(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::TaxBill(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Transfer(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::SaleEvent(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Listing(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::LeaseEvent(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::UnitRentObservation(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Loan(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Lien(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::ForeclosureCase(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Permit(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::OperatingStatement(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::RentRoll(val) => val.verifications().map(|x| x.to_any()),
                EntityOrSubtype::Valuation(val) => val.verifications().map(|x| x.to_any()),

        }
    }
}

pub trait InstrumentReference   {

    fn relationship_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn relationship_type_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_relationship_type<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn document_number<'a>(&'a self) -> Option<&'a str>;
    // fn document_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_document_number(&mut self, value: Option<&'a str>);

    fn registry_reference<'a>(&'a self) -> Option<&'a str>;
    // fn registry_reference_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_registry_reference(&mut self, value: Option<&'a str>);

    fn recording_authority<'a>(&'a self) -> Option<&'a str>;
    // fn recording_authority_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_recording_authority<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl InstrumentReference for crate::InstrumentReference {
        fn relationship_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.relationship_type.as_ref();
    }
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait RecordedInstrument   {

    fn document_number<'a>(&'a self) -> Option<&'a str>;
    // fn document_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_document_number(&mut self, value: Option<&'a str>);

    fn recording_book<'a>(&'a self) -> Option<&'a str>;
    // fn recording_book_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_recording_book(&mut self, value: Option<&'a str>);

    fn recording_page<'a>(&'a self) -> Option<&'a str>;
    // fn recording_page_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_recording_page(&mut self, value: Option<&'a str>);

    fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn recorded_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_recorded_on(&mut self, value: Option<&'a NaiveDate>);

    fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn instrument_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_instrument_date(&mut self, value: Option<&'a NaiveDate>);

    fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn document_type_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_document_type<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn recording_authority<'a>(&'a self) -> Option<&'a str>;
    // fn recording_authority_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_recording_authority<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn registry_reference<'a>(&'a self) -> Option<&'a str>;
    // fn registry_reference_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_registry_reference(&mut self, value: Option<&'a str>);

    fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>>;
    // fn related_instruments_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>>;
    // fn set_related_instruments<E>(&mut self, value: Option<&Vec<E>>) where E: Into<InstrumentReference>;

    fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifacts<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;


}

impl RecordedInstrument for crate::RecordedInstrument {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}
impl RecordedInstrument for crate::Transfer {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}
impl RecordedInstrument for crate::Loan {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}
impl RecordedInstrument for crate::LoanEvent {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}
impl RecordedInstrument for crate::Lien {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}
impl RecordedInstrument for crate::ForeclosureFiling {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        return self.document_number.as_deref();
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        return self.recording_book.as_deref();
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        return self.recording_page.as_deref();
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.recorded_on.as_ref();
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.instrument_date.as_ref();
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.document_type.as_ref();
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        return self.recording_authority.as_deref();
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        return self.registry_reference.as_deref();
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        return self.related_instruments.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}

impl RecordedInstrument for crate::RecordedInstrumentOrSubtype {
        fn document_number<'a>(&'a self) -> Option<&'a str> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.document_number(),
                RecordedInstrumentOrSubtype::Loan(val) => val.document_number(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.document_number(),
                RecordedInstrumentOrSubtype::Lien(val) => val.document_number(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.document_number(),

        }
    }
        fn recording_book<'a>(&'a self) -> Option<&'a str> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.recording_book(),
                RecordedInstrumentOrSubtype::Loan(val) => val.recording_book(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.recording_book(),
                RecordedInstrumentOrSubtype::Lien(val) => val.recording_book(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.recording_book(),

        }
    }
        fn recording_page<'a>(&'a self) -> Option<&'a str> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.recording_page(),
                RecordedInstrumentOrSubtype::Loan(val) => val.recording_page(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.recording_page(),
                RecordedInstrumentOrSubtype::Lien(val) => val.recording_page(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.recording_page(),

        }
    }
        fn recorded_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.recorded_on(),
                RecordedInstrumentOrSubtype::Loan(val) => val.recorded_on(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.recorded_on(),
                RecordedInstrumentOrSubtype::Lien(val) => val.recorded_on(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.recorded_on(),

        }
    }
        fn instrument_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.instrument_date(),
                RecordedInstrumentOrSubtype::Loan(val) => val.instrument_date(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.instrument_date(),
                RecordedInstrumentOrSubtype::Lien(val) => val.instrument_date(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.instrument_date(),

        }
    }
        fn document_type<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.document_type(),
                RecordedInstrumentOrSubtype::Loan(val) => val.document_type(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.document_type(),
                RecordedInstrumentOrSubtype::Lien(val) => val.document_type(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.document_type(),

        }
    }
        fn recording_authority<'a>(&'a self) -> Option<&'a str> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.recording_authority(),
                RecordedInstrumentOrSubtype::Loan(val) => val.recording_authority(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.recording_authority(),
                RecordedInstrumentOrSubtype::Lien(val) => val.recording_authority(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.recording_authority(),

        }
    }
        fn registry_reference<'a>(&'a self) -> Option<&'a str> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.registry_reference(),
                RecordedInstrumentOrSubtype::Loan(val) => val.registry_reference(),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.registry_reference(),
                RecordedInstrumentOrSubtype::Lien(val) => val.registry_reference(),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.registry_reference(),

        }
    }
        fn related_instruments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::InstrumentReference>> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.related_instruments().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::Loan(val) => val.related_instruments().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.related_instruments().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::Lien(val) => val.related_instruments().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.related_instruments().map(|x| x.to_any()),

        }
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                RecordedInstrumentOrSubtype::Transfer(val) => val.artifacts().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::Loan(val) => val.artifacts().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::LoanEvent(val) => val.artifacts().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::Lien(val) => val.artifacts().map(|x| x.to_any()),
                RecordedInstrumentOrSubtype::ForeclosureFiling(val) => val.artifacts().map(|x| x.to_any()),

        }
    }
}

pub trait TransactionParty   {

    fn role<'a>(&'a self) -> &'a str;
    // fn role_mut(&mut self) -> &mut &'a str;
    // fn set_role(&mut self, value: String);

    fn party<'a>(&'a self) -> &'a str;
    // fn party_mut(&mut self) -> &mut &'a str;
    // fn set_party<E>(&mut self, value: String) where E: Into<String>;

    fn sequence(&self) -> Option<isize>;
    // fn sequence_mut(&mut self) -> &mut Option<isize>;
    // fn set_sequence(&mut self, value: Option<isize>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl TransactionParty for crate::TransactionParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::TransferParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::SaleEventParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::ListingParticipant {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::LeaseEventParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::LoanParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::LienParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
impl TransactionParty for crate::ForeclosureCaseParty {
        fn role<'a>(&'a self) -> &'a str {
        return &self.role[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn sequence(&self) -> Option<isize> {
        return self.sequence;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}

impl TransactionParty for crate::TransactionPartyOrSubtype {
        fn role<'a>(&'a self) -> &'a str {
        match self {
                TransactionPartyOrSubtype::TransferParty(val) => val.role(),
                TransactionPartyOrSubtype::SaleEventParty(val) => val.role(),
                TransactionPartyOrSubtype::ListingParticipant(val) => val.role(),
                TransactionPartyOrSubtype::LeaseEventParty(val) => val.role(),
                TransactionPartyOrSubtype::LoanParty(val) => val.role(),
                TransactionPartyOrSubtype::LienParty(val) => val.role(),
                TransactionPartyOrSubtype::ForeclosureCaseParty(val) => val.role(),

        }
    }
        fn party<'a>(&'a self) -> &'a str {
        match self {
                TransactionPartyOrSubtype::TransferParty(val) => val.party(),
                TransactionPartyOrSubtype::SaleEventParty(val) => val.party(),
                TransactionPartyOrSubtype::ListingParticipant(val) => val.party(),
                TransactionPartyOrSubtype::LeaseEventParty(val) => val.party(),
                TransactionPartyOrSubtype::LoanParty(val) => val.party(),
                TransactionPartyOrSubtype::LienParty(val) => val.party(),
                TransactionPartyOrSubtype::ForeclosureCaseParty(val) => val.party(),

        }
    }
        fn sequence(&self) -> Option<isize> {
        match self {
                TransactionPartyOrSubtype::TransferParty(val) => val.sequence(),
                TransactionPartyOrSubtype::SaleEventParty(val) => val.sequence(),
                TransactionPartyOrSubtype::ListingParticipant(val) => val.sequence(),
                TransactionPartyOrSubtype::LeaseEventParty(val) => val.sequence(),
                TransactionPartyOrSubtype::LoanParty(val) => val.sequence(),
                TransactionPartyOrSubtype::LienParty(val) => val.sequence(),
                TransactionPartyOrSubtype::ForeclosureCaseParty(val) => val.sequence(),

        }
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        match self {
                TransactionPartyOrSubtype::TransferParty(val) => val.extras(),
                TransactionPartyOrSubtype::SaleEventParty(val) => val.extras(),
                TransactionPartyOrSubtype::ListingParticipant(val) => val.extras(),
                TransactionPartyOrSubtype::LeaseEventParty(val) => val.extras(),
                TransactionPartyOrSubtype::LoanParty(val) => val.extras(),
                TransactionPartyOrSubtype::LienParty(val) => val.extras(),
                TransactionPartyOrSubtype::ForeclosureCaseParty(val) => val.extras(),

        }
    }
}

pub trait Jurisdiction : Entity   {

    fn country<'a>(&'a self) -> &'a str;
    // fn country_mut(&mut self) -> &mut &'a str;
    // fn set_country(&mut self, value: String);

    fn region<'a>(&'a self) -> Option<&'a str>;
    // fn region_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_region(&mut self, value: Option<&'a str>);

    fn name<'a>(&'a self) -> &'a str;
    // fn name_mut(&mut self) -> &mut &'a str;
    // fn set_name(&mut self, value: String);

    fn kind<'a>(&'a self) -> &'a str;
    // fn kind_mut(&mut self) -> &mut &'a str;
    // fn set_kind(&mut self, value: String);

    fn authority_code<'a>(&'a self) -> Option<&'a str>;
    // fn authority_code_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_authority_code(&mut self, value: Option<&'a str>);

    fn parent<'a>(&'a self) -> Option<&'a str>;
    // fn parent_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_parent<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn boundary<'a>(&'a self) -> Option<&'a crate::Geometry>;
    // fn boundary_mut(&mut self) -> &mut Option<&'a crate::Geometry>;
    // fn set_boundary<E>(&mut self, value: Option<E>) where E: Into<Geometry>;


}

impl Jurisdiction for crate::Jurisdiction {
        fn country<'a>(&'a self) -> &'a str {
        return &self.country[..];
    }
        fn region<'a>(&'a self) -> Option<&'a str> {
        return self.region.as_deref();
    }
        fn name<'a>(&'a self) -> &'a str {
        return &self.name[..];
    }
        fn kind<'a>(&'a self) -> &'a str {
        return &self.kind[..];
    }
        fn authority_code<'a>(&'a self) -> Option<&'a str> {
        return self.authority_code.as_deref();
    }
        fn parent<'a>(&'a self) -> Option<&'a str> {
        return self.parent.as_deref();
    }
        fn boundary<'a>(&'a self) -> Option<&'a crate::Geometry> {
        return self.boundary.as_ref();
    }
}


pub trait Address : Entity   {

    fn country<'a>(&'a self) -> &'a str;
    // fn country_mut(&mut self) -> &mut &'a str;
    // fn set_country(&mut self, value: String);

    fn unformatted_address<'a>(&'a self) -> Option<&'a str>;
    // fn unformatted_address_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_unformatted_address(&mut self, value: Option<&'a str>);

    fn street_number<'a>(&'a self) -> Option<&'a str>;
    // fn street_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_street_number(&mut self, value: Option<&'a str>);

    fn street_pre_direction<'a>(&'a self) -> Option<&'a str>;
    // fn street_pre_direction_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_street_pre_direction(&mut self, value: Option<&'a str>);

    fn street_name<'a>(&'a self) -> Option<&'a str>;
    // fn street_name_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_street_name(&mut self, value: Option<&'a str>);

    fn street_suffix<'a>(&'a self) -> Option<&'a str>;
    // fn street_suffix_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_street_suffix(&mut self, value: Option<&'a str>);

    fn street_post_direction<'a>(&'a self) -> Option<&'a str>;
    // fn street_post_direction_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_street_post_direction(&mut self, value: Option<&'a str>);

    fn unit_type<'a>(&'a self) -> Option<&'a str>;
    // fn unit_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_unit_type(&mut self, value: Option<&'a str>);

    fn unit_number<'a>(&'a self) -> Option<&'a str>;
    // fn unit_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_unit_number(&mut self, value: Option<&'a str>);

    fn sublocality<'a>(&'a self) -> Option<&'a str>;
    // fn sublocality_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_sublocality(&mut self, value: Option<&'a str>);

    fn city<'a>(&'a self) -> Option<&'a str>;
    // fn city_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_city(&mut self, value: Option<&'a str>);

    fn region<'a>(&'a self) -> Option<&'a str>;
    // fn region_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_region(&mut self, value: Option<&'a str>);

    fn postal_code<'a>(&'a self) -> Option<&'a str>;
    // fn postal_code_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_postal_code(&mut self, value: Option<&'a str>);

    fn postal_code_suffix<'a>(&'a self) -> Option<&'a str>;
    // fn postal_code_suffix_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_postal_code_suffix(&mut self, value: Option<&'a str>);

    fn admin_area<'a>(&'a self) -> Option<&'a str>;
    // fn admin_area_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_admin_area(&mut self, value: Option<&'a str>);

    fn admin_area_code<'a>(&'a self) -> Option<&'a str>;
    // fn admin_area_code_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_admin_area_code(&mut self, value: Option<&'a str>);

    fn address_hash<'a>(&'a self) -> Option<&'a str>;
    // fn address_hash_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_address_hash(&mut self, value: Option<&'a str>);

    fn address_hash_scheme<'a>(&'a self) -> Option<&'a str>;
    // fn address_hash_scheme_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_address_hash_scheme(&mut self, value: Option<&'a str>);

    fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint>;
    // fn location_mut(&mut self) -> &mut Option<&'a crate::GeoPoint>;
    // fn set_location<E>(&mut self, value: Option<E>) where E: Into<GeoPoint>;

    fn location_accuracy<'a>(&'a self) -> Option<&'a crate::GeocodeAccuracy>;
    // fn location_accuracy_mut(&mut self) -> &mut Option<&'a crate::GeocodeAccuracy>;
    // fn set_location_accuracy(&mut self, value: Option<&'a GeocodeAccuracy>);


}

impl Address for crate::Address {
        fn country<'a>(&'a self) -> &'a str {
        return &self.country[..];
    }
        fn unformatted_address<'a>(&'a self) -> Option<&'a str> {
        return self.unformatted_address.as_deref();
    }
        fn street_number<'a>(&'a self) -> Option<&'a str> {
        return self.street_number.as_deref();
    }
        fn street_pre_direction<'a>(&'a self) -> Option<&'a str> {
        return self.street_pre_direction.as_deref();
    }
        fn street_name<'a>(&'a self) -> Option<&'a str> {
        return self.street_name.as_deref();
    }
        fn street_suffix<'a>(&'a self) -> Option<&'a str> {
        return self.street_suffix.as_deref();
    }
        fn street_post_direction<'a>(&'a self) -> Option<&'a str> {
        return self.street_post_direction.as_deref();
    }
        fn unit_type<'a>(&'a self) -> Option<&'a str> {
        return self.unit_type.as_deref();
    }
        fn unit_number<'a>(&'a self) -> Option<&'a str> {
        return self.unit_number.as_deref();
    }
        fn sublocality<'a>(&'a self) -> Option<&'a str> {
        return self.sublocality.as_deref();
    }
        fn city<'a>(&'a self) -> Option<&'a str> {
        return self.city.as_deref();
    }
        fn region<'a>(&'a self) -> Option<&'a str> {
        return self.region.as_deref();
    }
        fn postal_code<'a>(&'a self) -> Option<&'a str> {
        return self.postal_code.as_deref();
    }
        fn postal_code_suffix<'a>(&'a self) -> Option<&'a str> {
        return self.postal_code_suffix.as_deref();
    }
        fn admin_area<'a>(&'a self) -> Option<&'a str> {
        return self.admin_area.as_deref();
    }
        fn admin_area_code<'a>(&'a self) -> Option<&'a str> {
        return self.admin_area_code.as_deref();
    }
        fn address_hash<'a>(&'a self) -> Option<&'a str> {
        return self.address_hash.as_deref();
    }
        fn address_hash_scheme<'a>(&'a self) -> Option<&'a str> {
        return self.address_hash_scheme.as_deref();
    }
        fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint> {
        return self.location.as_ref();
    }
        fn location_accuracy<'a>(&'a self) -> Option<&'a crate::GeocodeAccuracy> {
        return self.location_accuracy.as_ref();
    }
}


pub trait PropertyFacts   {

    fn name<'a>(&'a self) -> Option<&'a str>;
    // fn name_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_name(&mut self, value: Option<&'a str>);

    fn property_use_class<'a>(&'a self) -> Option<&'a str>;
    // fn property_use_class_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_use_class(&mut self, value: Option<&'a str>);

    fn property_use_type<'a>(&'a self) -> Option<&'a str>;
    // fn property_use_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_use_type(&mut self, value: Option<&'a str>);

    fn property_use_subtype<'a>(&'a self) -> Option<&'a str>;
    // fn property_use_subtype_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_use_subtype(&mut self, value: Option<&'a str>);

    fn property_use_system<'a>(&'a self) -> Option<&'a str>;
    // fn property_use_system_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_use_system(&mut self, value: Option<&'a str>);

    fn estate_type<'a>(&'a self) -> Option<&'a crate::EstateType>;
    // fn estate_type_mut(&mut self) -> &mut Option<&'a crate::EstateType>;
    // fn set_estate_type(&mut self, value: Option<&'a EstateType>);

    fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint>;
    // fn location_mut(&mut self) -> &mut Option<&'a crate::GeoPoint>;
    // fn set_location<E>(&mut self, value: Option<E>) where E: Into<GeoPoint>;

    fn building_count(&self) -> Option<isize>;
    // fn building_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_building_count(&mut self, value: Option<isize>);


}

impl PropertyFacts for crate::PropertyFacts {
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn property_use_class<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_class.as_deref();
    }
        fn property_use_type<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_type.as_deref();
    }
        fn property_use_subtype<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_subtype.as_deref();
    }
        fn property_use_system<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_system.as_deref();
    }
        fn estate_type<'a>(&'a self) -> Option<&'a crate::EstateType> {
        return self.estate_type.as_ref();
    }
        fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint> {
        return self.location.as_ref();
    }
        fn building_count(&self) -> Option<isize> {
        return self.building_count;
    }
}
impl PropertyFacts for crate::Property {
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn property_use_class<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_class.as_deref();
    }
        fn property_use_type<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_type.as_deref();
    }
        fn property_use_subtype<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_subtype.as_deref();
    }
        fn property_use_system<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_system.as_deref();
    }
        fn estate_type<'a>(&'a self) -> Option<&'a crate::EstateType> {
        return self.estate_type.as_ref();
    }
        fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint> {
        return self.location.as_ref();
    }
        fn building_count(&self) -> Option<isize> {
        return self.building_count;
    }
}
impl PropertyFacts for crate::PropertyState {
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn property_use_class<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_class.as_deref();
    }
        fn property_use_type<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_type.as_deref();
    }
        fn property_use_subtype<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_subtype.as_deref();
    }
        fn property_use_system<'a>(&'a self) -> Option<&'a str> {
        return self.property_use_system.as_deref();
    }
        fn estate_type<'a>(&'a self) -> Option<&'a crate::EstateType> {
        return self.estate_type.as_ref();
    }
        fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint> {
        return self.location.as_ref();
    }
        fn building_count(&self) -> Option<isize> {
        return self.building_count;
    }
}

impl PropertyFacts for crate::PropertyFactsOrSubtype {
        fn name<'a>(&'a self) -> Option<&'a str> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.name(),
                PropertyFactsOrSubtype::PropertyState(val) => val.name(),

        }
    }
        fn property_use_class<'a>(&'a self) -> Option<&'a str> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.property_use_class(),
                PropertyFactsOrSubtype::PropertyState(val) => val.property_use_class(),

        }
    }
        fn property_use_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.property_use_type(),
                PropertyFactsOrSubtype::PropertyState(val) => val.property_use_type(),

        }
    }
        fn property_use_subtype<'a>(&'a self) -> Option<&'a str> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.property_use_subtype(),
                PropertyFactsOrSubtype::PropertyState(val) => val.property_use_subtype(),

        }
    }
        fn property_use_system<'a>(&'a self) -> Option<&'a str> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.property_use_system(),
                PropertyFactsOrSubtype::PropertyState(val) => val.property_use_system(),

        }
    }
        fn estate_type<'a>(&'a self) -> Option<&'a crate::EstateType> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.estate_type(),
                PropertyFactsOrSubtype::PropertyState(val) => val.estate_type(),

        }
    }
        fn location<'a>(&'a self) -> Option<&'a crate::GeoPoint> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.location(),
                PropertyFactsOrSubtype::PropertyState(val) => val.location(),

        }
    }
        fn building_count(&self) -> Option<isize> {
        match self {
                PropertyFactsOrSubtype::Property(val) => val.building_count(),
                PropertyFactsOrSubtype::PropertyState(val) => val.building_count(),

        }
    }
}

pub trait Property : Entity  +  PropertyFacts   {


}

impl Property for crate::Property {
}


pub trait Parcel : Entity   {

    fn jurisdiction<'a>(&'a self) -> &'a str;
    // fn jurisdiction_mut(&mut self) -> &mut &'a str;
    // fn set_jurisdiction<E>(&mut self, value: String) where E: Into<String>;

    fn parcel_number<'a>(&'a self) -> &'a str;
    // fn parcel_number_mut(&mut self) -> &mut &'a str;
    // fn set_parcel_number(&mut self, value: String);

    fn normalized_parcel_number<'a>(&'a self) -> Option<&'a str>;
    // fn normalized_parcel_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_normalized_parcel_number(&mut self, value: Option<&'a str>);

    fn unit_designator<'a>(&'a self) -> Option<&'a str>;
    // fn unit_designator_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_unit_designator(&mut self, value: Option<&'a str>);

    fn reso_upi<'a>(&'a self) -> Option<&'a str>;
    // fn reso_upi_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_reso_upi(&mut self, value: Option<&'a str>);

    fn legal_description<'a>(&'a self) -> Option<&'a str>;
    // fn legal_description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_legal_description(&mut self, value: Option<&'a str>);

    fn land_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn land_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_land_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn boundary<'a>(&'a self) -> Option<&'a crate::Geometry>;
    // fn boundary_mut(&mut self) -> &mut Option<&'a crate::Geometry>;
    // fn set_boundary<E>(&mut self, value: Option<E>) where E: Into<Geometry>;

    fn retired_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn retired_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_retired_on(&mut self, value: Option<&'a NaiveDate>);


}

impl Parcel for crate::Parcel {
        fn jurisdiction<'a>(&'a self) -> &'a str {
        return &self.jurisdiction[..];
    }
        fn parcel_number<'a>(&'a self) -> &'a str {
        return &self.parcel_number[..];
    }
        fn normalized_parcel_number<'a>(&'a self) -> Option<&'a str> {
        return self.normalized_parcel_number.as_deref();
    }
        fn unit_designator<'a>(&'a self) -> Option<&'a str> {
        return self.unit_designator.as_deref();
    }
        fn reso_upi<'a>(&'a self) -> Option<&'a str> {
        return self.reso_upi.as_deref();
    }
        fn legal_description<'a>(&'a self) -> Option<&'a str> {
        return self.legal_description.as_deref();
    }
        fn land_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.land_area.as_ref();
    }
        fn boundary<'a>(&'a self) -> Option<&'a crate::Geometry> {
        return self.boundary.as_ref();
    }
        fn retired_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.retired_on.as_ref();
    }
}


pub trait PropertyParcel : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn parcel<'a>(&'a self) -> &'a str;
    // fn parcel_mut(&mut self) -> &mut &'a str;
    // fn set_parcel<E>(&mut self, value: String) where E: Into<String>;

    fn is_primary(&self) -> Option<bool>;
    // fn is_primary_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_primary(&mut self, value: Option<bool>);

    fn started_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn started_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_started_on(&mut self, value: Option<&'a NaiveDate>);

    fn ended_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn ended_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_ended_on(&mut self, value: Option<&'a NaiveDate>);


}

impl PropertyParcel for crate::PropertyParcel {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn parcel<'a>(&'a self) -> &'a str {
        return &self.parcel[..];
    }
        fn is_primary(&self) -> Option<bool> {
        return self.is_primary;
    }
        fn started_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.started_on.as_ref();
    }
        fn ended_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.ended_on.as_ref();
    }
}


pub trait ParcelLineage : Entity   {

    fn predecessor_parcel<'a>(&'a self) -> &'a str;
    // fn predecessor_parcel_mut(&mut self) -> &mut &'a str;
    // fn set_predecessor_parcel<E>(&mut self, value: String) where E: Into<String>;

    fn successor_parcel<'a>(&'a self) -> &'a str;
    // fn successor_parcel_mut(&mut self) -> &mut &'a str;
    // fn set_successor_parcel<E>(&mut self, value: String) where E: Into<String>;

    fn kind<'a>(&'a self) -> &'a crate::ParcelLineageKind;
    // fn kind_mut(&mut self) -> &mut &'a crate::ParcelLineageKind;
    // fn set_kind(&mut self, value: ParcelLineageKind);

    fn effective_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn effective_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_effective_on(&mut self, value: Option<&'a NaiveDate>);


}

impl ParcelLineage for crate::ParcelLineage {
        fn predecessor_parcel<'a>(&'a self) -> &'a str {
        return &self.predecessor_parcel[..];
    }
        fn successor_parcel<'a>(&'a self) -> &'a str {
        return &self.successor_parcel[..];
    }
        fn kind<'a>(&'a self) -> &'a crate::ParcelLineageKind {
        return &self.kind;
    }
        fn effective_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.effective_on.as_ref();
    }
}


pub trait PropertyIdentifier : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn scheme<'a>(&'a self) -> &'a str;
    // fn scheme_mut(&mut self) -> &mut &'a str;
    // fn set_scheme(&mut self, value: String);

    fn namespace<'a>(&'a self) -> Option<&'a str>;
    // fn namespace_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_namespace(&mut self, value: Option<&'a str>);

    fn value<'a>(&'a self) -> &'a str;
    // fn value_mut(&mut self) -> &mut &'a str;
    // fn set_value(&mut self, value: String);


}

impl PropertyIdentifier for crate::PropertyIdentifier {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn scheme<'a>(&'a self) -> &'a str {
        return &self.scheme[..];
    }
        fn namespace<'a>(&'a self) -> Option<&'a str> {
        return self.namespace.as_deref();
    }
        fn value<'a>(&'a self) -> &'a str {
        return &self.value[..];
    }
}


pub trait Party : Entity   {

    fn kind<'a>(&'a self) -> &'a crate::PartyKind;
    // fn kind_mut(&mut self) -> &mut &'a crate::PartyKind;
    // fn set_kind(&mut self, value: PartyKind);

    fn legal_form<'a>(&'a self) -> Option<&'a crate::Classification>;
    // fn legal_form_mut(&mut self) -> &mut Option<&'a crate::Classification>;
    // fn set_legal_form<E>(&mut self, value: Option<E>) where E: Into<Classification>;

    fn name<'a>(&'a self) -> &'a str;
    // fn name_mut(&mut self) -> &mut &'a str;
    // fn set_name(&mut self, value: String);

    fn normalized_name<'a>(&'a self) -> Option<&'a str>;
    // fn normalized_name_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_normalized_name(&mut self, value: Option<&'a str>);

    fn name_first<'a>(&'a self) -> Option<&'a str>;
    // fn name_first_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_name_first(&mut self, value: Option<&'a str>);

    fn name_middle<'a>(&'a self) -> Option<&'a str>;
    // fn name_middle_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_name_middle(&mut self, value: Option<&'a str>);

    fn name_last<'a>(&'a self) -> Option<&'a str>;
    // fn name_last_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_name_last(&mut self, value: Option<&'a str>);

    fn classifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Classification>>;
    // fn classifications_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Classification>>;
    // fn set_classifications<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Classification>;

    fn addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PartyAddress>>;
    // fn addresses_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PartyAddress>>;
    // fn set_addresses<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PartyAddress>;

    fn contacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PartyContact>>;
    // fn contacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PartyContact>>;
    // fn set_contacts<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PartyContact>;


}

impl Party for crate::Party {
        fn kind<'a>(&'a self) -> &'a crate::PartyKind {
        return &self.kind;
    }
        fn legal_form<'a>(&'a self) -> Option<&'a crate::Classification> {
        return self.legal_form.as_ref();
    }
        fn name<'a>(&'a self) -> &'a str {
        return &self.name[..];
    }
        fn normalized_name<'a>(&'a self) -> Option<&'a str> {
        return self.normalized_name.as_deref();
    }
        fn name_first<'a>(&'a self) -> Option<&'a str> {
        return self.name_first.as_deref();
    }
        fn name_middle<'a>(&'a self) -> Option<&'a str> {
        return self.name_middle.as_deref();
    }
        fn name_last<'a>(&'a self) -> Option<&'a str> {
        return self.name_last.as_deref();
    }
        fn classifications<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Classification>> {
        return self.classifications.as_ref();
    }
        fn addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PartyAddress>> {
        return self.addresses.as_ref().map(|x| poly_containers::ListView::new(x));
    }
        fn contacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PartyContact>> {
        return self.contacts.as_ref();
    }
}


pub trait VerificationAttribution   {

    fn verifier<'a>(&'a self) -> &'a str;
    // fn verifier_mut(&mut self) -> &mut &'a str;
    // fn set_verifier<E>(&mut self, value: String) where E: Into<String>;

    fn verified_at<'a>(&'a self) -> &'a crate::DateTime<FixedOffset>;
    // fn verified_at_mut(&mut self) -> &mut &'a crate::DateTime<FixedOffset>;
    // fn set_verified_at(&mut self, value: DateTime<FixedOffset>);

    fn note<'a>(&'a self) -> Option<&'a str>;
    // fn note_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_note(&mut self, value: Option<&'a str>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl VerificationAttribution for crate::VerificationAttribution {
        fn verifier<'a>(&'a self) -> &'a str {
        return &self.verifier[..];
    }
        fn verified_at<'a>(&'a self) -> &'a crate::DateTime<FixedOffset> {
        return &self.verified_at;
    }
        fn note<'a>(&'a self) -> Option<&'a str> {
        return self.note.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait SourceArtifact : Entity   {

    fn uri<'a>(&'a self) -> Option<&'a crate::uri>;
    // fn uri_mut(&mut self) -> &mut Option<&'a crate::uri>;
    // fn set_uri(&mut self, value: Option<&'a uri>);

    fn storage_reference<'a>(&'a self) -> Option<&'a str>;
    // fn storage_reference_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_storage_reference(&mut self, value: Option<&'a str>);

    fn media_type<'a>(&'a self) -> Option<&'a str>;
    // fn media_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_media_type(&mut self, value: Option<&'a str>);

    fn kind<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn kind_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_kind<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn title<'a>(&'a self) -> Option<&'a str>;
    // fn title_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_title(&mut self, value: Option<&'a str>);

    fn original_filename<'a>(&'a self) -> Option<&'a str>;
    // fn original_filename_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_original_filename(&mut self, value: Option<&'a str>);

    fn content_hash<'a>(&'a self) -> Option<&'a str>;
    // fn content_hash_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_content_hash(&mut self, value: Option<&'a str>);

    fn hash_scheme<'a>(&'a self) -> Option<&'a str>;
    // fn hash_scheme_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_hash_scheme(&mut self, value: Option<&'a str>);

    fn page_count(&self) -> Option<isize>;
    // fn page_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_page_count(&mut self, value: Option<isize>);

    fn captured_on<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>>;
    // fn captured_on_mut(&mut self) -> &mut Option<&'a crate::DateTime<FixedOffset>>;
    // fn set_captured_on(&mut self, value: Option<&'a DateTime<FixedOffset>>);


}

impl SourceArtifact for crate::SourceArtifact {
        fn uri<'a>(&'a self) -> Option<&'a crate::uri> {
        return self.uri.as_ref();
    }
        fn storage_reference<'a>(&'a self) -> Option<&'a str> {
        return self.storage_reference.as_deref();
    }
        fn media_type<'a>(&'a self) -> Option<&'a str> {
        return self.media_type.as_deref();
    }
        fn kind<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.kind.as_ref();
    }
        fn title<'a>(&'a self) -> Option<&'a str> {
        return self.title.as_deref();
    }
        fn original_filename<'a>(&'a self) -> Option<&'a str> {
        return self.original_filename.as_deref();
    }
        fn content_hash<'a>(&'a self) -> Option<&'a str> {
        return self.content_hash.as_deref();
    }
        fn hash_scheme<'a>(&'a self) -> Option<&'a str> {
        return self.hash_scheme.as_deref();
    }
        fn page_count(&self) -> Option<isize> {
        return self.page_count;
    }
        fn captured_on<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>> {
        return self.captured_on.as_ref();
    }
}


pub trait AddressAssociation   {

    fn address<'a>(&'a self) -> &'a str;
    // fn address_mut(&mut self) -> &mut &'a str;
    // fn set_address<E>(&mut self, value: String) where E: Into<String>;

    fn role<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn role_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_role<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn is_primary(&self) -> Option<bool>;
    // fn is_primary_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_primary(&mut self, value: Option<bool>);

    fn valid_from<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn valid_from_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_valid_from(&mut self, value: Option<&'a NaiveDate>);

    fn valid_to<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn valid_to_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_valid_to(&mut self, value: Option<&'a NaiveDate>);


}

impl AddressAssociation for crate::AddressAssociation {
        fn address<'a>(&'a self) -> &'a str {
        return &self.address[..];
    }
        fn role<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.role.as_ref();
    }
        fn is_primary(&self) -> Option<bool> {
        return self.is_primary;
    }
        fn valid_from<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_from.as_ref();
    }
        fn valid_to<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_to.as_ref();
    }
}
impl AddressAssociation for crate::PartyAddress {
        fn address<'a>(&'a self) -> &'a str {
        return &self.address[..];
    }
        fn role<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.role.as_ref();
    }
        fn is_primary(&self) -> Option<bool> {
        return self.is_primary;
    }
        fn valid_from<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_from.as_ref();
    }
        fn valid_to<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_to.as_ref();
    }
}
impl AddressAssociation for crate::PropertyAddress {
        fn address<'a>(&'a self) -> &'a str {
        return &self.address[..];
    }
        fn role<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.role.as_ref();
    }
        fn is_primary(&self) -> Option<bool> {
        return self.is_primary;
    }
        fn valid_from<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_from.as_ref();
    }
        fn valid_to<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.valid_to.as_ref();
    }
}

impl AddressAssociation for crate::AddressAssociationOrSubtype {
        fn address<'a>(&'a self) -> &'a str {
        match self {
                AddressAssociationOrSubtype::PartyAddress(val) => val.address(),
                AddressAssociationOrSubtype::PropertyAddress(val) => val.address(),

        }
    }
        fn role<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        match self {
                AddressAssociationOrSubtype::PartyAddress(val) => val.role(),
                AddressAssociationOrSubtype::PropertyAddress(val) => val.role(),

        }
    }
        fn is_primary(&self) -> Option<bool> {
        match self {
                AddressAssociationOrSubtype::PartyAddress(val) => val.is_primary(),
                AddressAssociationOrSubtype::PropertyAddress(val) => val.is_primary(),

        }
    }
        fn valid_from<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        match self {
                AddressAssociationOrSubtype::PartyAddress(val) => val.valid_from(),
                AddressAssociationOrSubtype::PropertyAddress(val) => val.valid_from(),

        }
    }
        fn valid_to<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        match self {
                AddressAssociationOrSubtype::PartyAddress(val) => val.valid_to(),
                AddressAssociationOrSubtype::PropertyAddress(val) => val.valid_to(),

        }
    }
}

pub trait PartyAddress : AddressAssociation   {

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl PartyAddress for crate::PartyAddress {
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait PropertyAddress : Entity  +  AddressAssociation   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;


}

impl PropertyAddress for crate::PropertyAddress {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
}


pub trait PartyContact   {

    fn kind<'a>(&'a self) -> &'a str;
    // fn kind_mut(&mut self) -> &mut &'a str;
    // fn set_kind(&mut self, value: String);

    fn value<'a>(&'a self) -> &'a str;
    // fn value_mut(&mut self) -> &mut &'a str;
    // fn set_value(&mut self, value: String);

    fn label<'a>(&'a self) -> Option<&'a str>;
    // fn label_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_label(&mut self, value: Option<&'a str>);

    fn is_primary(&self) -> Option<bool>;
    // fn is_primary_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_primary(&mut self, value: Option<bool>);

    fn do_not_contact(&self) -> Option<bool>;
    // fn do_not_contact_mut(&mut self) -> &mut Option<bool>;
    // fn set_do_not_contact(&mut self, value: Option<bool>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl PartyContact for crate::PartyContact {
        fn kind<'a>(&'a self) -> &'a str {
        return &self.kind[..];
    }
        fn value<'a>(&'a self) -> &'a str {
        return &self.value[..];
    }
        fn label<'a>(&'a self) -> Option<&'a str> {
        return self.label.as_deref();
    }
        fn is_primary(&self) -> Option<bool> {
        return self.is_primary;
    }
        fn do_not_contact(&self) -> Option<bool> {
        return self.do_not_contact;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait OwnershipPeriod : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn started_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn started_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_started_on(&mut self, value: Option<&'a NaiveDate>);

    fn ended_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn ended_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_ended_on(&mut self, value: Option<&'a NaiveDate>);

    fn vesting_type<'a>(&'a self) -> Option<&'a str>;
    // fn vesting_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_vesting_type(&mut self, value: Option<&'a str>);

    fn mailing_address<'a>(&'a self) -> Option<&'a str>;
    // fn mailing_address_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_mailing_address<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn acquired_via_transfer<'a>(&'a self) -> Option<&'a str>;
    // fn acquired_via_transfer_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_acquired_via_transfer<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn disposed_via_transfer<'a>(&'a self) -> Option<&'a str>;
    // fn disposed_via_transfer_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_disposed_via_transfer<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn interests<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OwnershipInterest>>;
    // fn interests_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::OwnershipInterest>>;
    // fn set_interests<E>(&mut self, value: Option<&Vec<E>>) where E: Into<OwnershipInterest>;


}

impl OwnershipPeriod for crate::OwnershipPeriod {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn started_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.started_on.as_ref();
    }
        fn ended_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.ended_on.as_ref();
    }
        fn vesting_type<'a>(&'a self) -> Option<&'a str> {
        return self.vesting_type.as_deref();
    }
        fn mailing_address<'a>(&'a self) -> Option<&'a str> {
        return self.mailing_address.as_deref();
    }
        fn acquired_via_transfer<'a>(&'a self) -> Option<&'a str> {
        return self.acquired_via_transfer.as_deref();
    }
        fn disposed_via_transfer<'a>(&'a self) -> Option<&'a str> {
        return self.disposed_via_transfer.as_deref();
    }
        fn interests<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OwnershipInterest>> {
        return self.interests.as_ref();
    }
}


pub trait OwnershipInterest   {

    fn party<'a>(&'a self) -> &'a str;
    // fn party_mut(&mut self) -> &mut &'a str;
    // fn set_party<E>(&mut self, value: String) where E: Into<String>;

    fn interest_pct(&self) -> Option<f64>;
    // fn interest_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_interest_pct(&mut self, value: Option<f64>);

    fn role<'a>(&'a self) -> Option<&'a str>;
    // fn role_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_role(&mut self, value: Option<&'a str>);

    fn is_owner_occupied(&self) -> Option<bool>;
    // fn is_owner_occupied_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_owner_occupied(&mut self, value: Option<bool>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl OwnershipInterest for crate::OwnershipInterest {
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn interest_pct(&self) -> Option<f64> {
        return self.interest_pct;
    }
        fn role<'a>(&'a self) -> Option<&'a str> {
        return self.role.as_deref();
    }
        fn is_owner_occupied(&self) -> Option<bool> {
        return self.is_owner_occupied;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait StructureFacts   {

    fn kind<'a>(&'a self) -> Option<&'a str>;
    // fn kind_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_kind(&mut self, value: Option<&'a str>);

    fn name<'a>(&'a self) -> Option<&'a str>;
    // fn name_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_name(&mut self, value: Option<&'a str>);

    fn structure_number<'a>(&'a self) -> Option<&'a str>;
    // fn structure_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_structure_number(&mut self, value: Option<&'a str>);

    fn living_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn living_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_living_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn gross_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn gross_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_gross_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn rentable_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_rentable_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn ground_floor_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn ground_floor_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_ground_floor_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn basement_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn basement_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_basement_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn basement_finished_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn basement_finished_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_basement_finished_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn garage_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn garage_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_garage_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn areas<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>>;
    // fn areas_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>>;
    // fn set_areas<E>(&mut self, value: Option<&Vec<E>>) where E: Into<AreaMeasure>;

    fn year_built(&self) -> Option<isize>;
    // fn year_built_mut(&mut self) -> &mut Option<isize>;
    // fn set_year_built(&mut self, value: Option<isize>);

    fn year_built_estimated(&self) -> Option<bool>;
    // fn year_built_estimated_mut(&mut self) -> &mut Option<bool>;
    // fn set_year_built_estimated(&mut self, value: Option<bool>);

    fn effective_year_built(&self) -> Option<isize>;
    // fn effective_year_built_mut(&mut self) -> &mut Option<isize>;
    // fn set_effective_year_built(&mut self, value: Option<isize>);

    fn stories(&self) -> Option<f64>;
    // fn stories_mut(&mut self) -> &mut Option<f64>;
    // fn set_stories(&mut self, value: Option<f64>);

    fn unit_count(&self) -> Option<isize>;
    // fn unit_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_unit_count(&mut self, value: Option<isize>);

    fn construction_method<'a>(&'a self) -> Option<&'a str>;
    // fn construction_method_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_construction_method(&mut self, value: Option<&'a str>);

    fn construction_status<'a>(&'a self) -> Option<&'a str>;
    // fn construction_status_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_construction_status(&mut self, value: Option<&'a str>);

    fn construction_type<'a>(&'a self) -> Option<&'a str>;
    // fn construction_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_construction_type(&mut self, value: Option<&'a str>);

    fn exterior_wall_type<'a>(&'a self) -> Option<&'a str>;
    // fn exterior_wall_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_exterior_wall_type(&mut self, value: Option<&'a str>);

    fn roof_material_type<'a>(&'a self) -> Option<&'a str>;
    // fn roof_material_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_roof_material_type(&mut self, value: Option<&'a str>);

    fn roof_style_type<'a>(&'a self) -> Option<&'a str>;
    // fn roof_style_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_roof_style_type(&mut self, value: Option<&'a str>);

    fn foundation_type<'a>(&'a self) -> Option<&'a str>;
    // fn foundation_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_foundation_type(&mut self, value: Option<&'a str>);

    fn foundation_material<'a>(&'a self) -> Option<&'a str>;
    // fn foundation_material_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_foundation_material(&mut self, value: Option<&'a str>);

    fn condition_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>>;
    // fn condition_ratings_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Rating>>;
    // fn set_condition_ratings<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Rating>;

    fn quality_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>>;
    // fn quality_ratings_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Rating>>;
    // fn set_quality_ratings<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Rating>;

    fn heating_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn heating_types_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_heating_types(&mut self, value: Option<&Vec<String>>);

    fn heating_fuel_type<'a>(&'a self) -> Option<&'a str>;
    // fn heating_fuel_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_heating_fuel_type(&mut self, value: Option<&'a str>);

    fn cooling_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn cooling_types_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_cooling_types(&mut self, value: Option<&Vec<String>>);

    fn sewer_type<'a>(&'a self) -> Option<&'a str>;
    // fn sewer_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_sewer_type(&mut self, value: Option<&'a str>);

    fn water_type<'a>(&'a self) -> Option<&'a str>;
    // fn water_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_water_type(&mut self, value: Option<&'a str>);

    fn features<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn features_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_features(&mut self, value: Option<&Vec<String>>);

    fn residential<'a>(&'a self) -> Option<&'a crate::ResidentialDetails>;
    // fn residential_mut(&mut self) -> &mut Option<&'a crate::ResidentialDetails>;
    // fn set_residential<E>(&mut self, value: Option<E>) where E: Into<ResidentialDetails>;

    fn commercial<'a>(&'a self) -> Option<&'a crate::CommercialDetails>;
    // fn commercial_mut(&mut self) -> &mut Option<&'a crate::CommercialDetails>;
    // fn set_commercial<E>(&mut self, value: Option<E>) where E: Into<CommercialDetails>;

    fn renovations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Renovation>>;
    // fn renovations_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Renovation>>;
    // fn set_renovations<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Renovation>;


}

impl StructureFacts for crate::StructureFacts {
        fn kind<'a>(&'a self) -> Option<&'a str> {
        return self.kind.as_deref();
    }
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn structure_number<'a>(&'a self) -> Option<&'a str> {
        return self.structure_number.as_deref();
    }
        fn living_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.living_area.as_ref();
    }
        fn gross_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.gross_area.as_ref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn ground_floor_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.ground_floor_area.as_ref();
    }
        fn basement_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_area.as_ref();
    }
        fn basement_finished_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_finished_area.as_ref();
    }
        fn garage_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.garage_area.as_ref();
    }
        fn areas<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>> {
        return self.areas.as_ref();
    }
        fn year_built(&self) -> Option<isize> {
        return self.year_built;
    }
        fn year_built_estimated(&self) -> Option<bool> {
        return self.year_built_estimated;
    }
        fn effective_year_built(&self) -> Option<isize> {
        return self.effective_year_built;
    }
        fn stories(&self) -> Option<f64> {
        return self.stories;
    }
        fn unit_count(&self) -> Option<isize> {
        return self.unit_count;
    }
        fn construction_method<'a>(&'a self) -> Option<&'a str> {
        return self.construction_method.as_deref();
    }
        fn construction_status<'a>(&'a self) -> Option<&'a str> {
        return self.construction_status.as_deref();
    }
        fn construction_type<'a>(&'a self) -> Option<&'a str> {
        return self.construction_type.as_deref();
    }
        fn exterior_wall_type<'a>(&'a self) -> Option<&'a str> {
        return self.exterior_wall_type.as_deref();
    }
        fn roof_material_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_material_type.as_deref();
    }
        fn roof_style_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_style_type.as_deref();
    }
        fn foundation_type<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_type.as_deref();
    }
        fn foundation_material<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_material.as_deref();
    }
        fn condition_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.condition_ratings.as_ref();
    }
        fn quality_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.quality_ratings.as_ref();
    }
        fn heating_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.heating_types.as_ref();
    }
        fn heating_fuel_type<'a>(&'a self) -> Option<&'a str> {
        return self.heating_fuel_type.as_deref();
    }
        fn cooling_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.cooling_types.as_ref();
    }
        fn sewer_type<'a>(&'a self) -> Option<&'a str> {
        return self.sewer_type.as_deref();
    }
        fn water_type<'a>(&'a self) -> Option<&'a str> {
        return self.water_type.as_deref();
    }
        fn features<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.features.as_ref();
    }
        fn residential<'a>(&'a self) -> Option<&'a crate::ResidentialDetails> {
        return self.residential.as_ref();
    }
        fn commercial<'a>(&'a self) -> Option<&'a crate::CommercialDetails> {
        return self.commercial.as_ref();
    }
        fn renovations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Renovation>> {
        return self.renovations.as_ref();
    }
}
impl StructureFacts for crate::Structure {
        fn kind<'a>(&'a self) -> Option<&'a str> {
        return self.kind.as_deref();
    }
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn structure_number<'a>(&'a self) -> Option<&'a str> {
        return self.structure_number.as_deref();
    }
        fn living_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.living_area.as_ref();
    }
        fn gross_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.gross_area.as_ref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn ground_floor_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.ground_floor_area.as_ref();
    }
        fn basement_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_area.as_ref();
    }
        fn basement_finished_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_finished_area.as_ref();
    }
        fn garage_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.garage_area.as_ref();
    }
        fn areas<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>> {
        return self.areas.as_ref();
    }
        fn year_built(&self) -> Option<isize> {
        return self.year_built;
    }
        fn year_built_estimated(&self) -> Option<bool> {
        return self.year_built_estimated;
    }
        fn effective_year_built(&self) -> Option<isize> {
        return self.effective_year_built;
    }
        fn stories(&self) -> Option<f64> {
        return self.stories;
    }
        fn unit_count(&self) -> Option<isize> {
        return self.unit_count;
    }
        fn construction_method<'a>(&'a self) -> Option<&'a str> {
        return self.construction_method.as_deref();
    }
        fn construction_status<'a>(&'a self) -> Option<&'a str> {
        return self.construction_status.as_deref();
    }
        fn construction_type<'a>(&'a self) -> Option<&'a str> {
        return self.construction_type.as_deref();
    }
        fn exterior_wall_type<'a>(&'a self) -> Option<&'a str> {
        return self.exterior_wall_type.as_deref();
    }
        fn roof_material_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_material_type.as_deref();
    }
        fn roof_style_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_style_type.as_deref();
    }
        fn foundation_type<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_type.as_deref();
    }
        fn foundation_material<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_material.as_deref();
    }
        fn condition_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.condition_ratings.as_ref();
    }
        fn quality_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.quality_ratings.as_ref();
    }
        fn heating_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.heating_types.as_ref();
    }
        fn heating_fuel_type<'a>(&'a self) -> Option<&'a str> {
        return self.heating_fuel_type.as_deref();
    }
        fn cooling_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.cooling_types.as_ref();
    }
        fn sewer_type<'a>(&'a self) -> Option<&'a str> {
        return self.sewer_type.as_deref();
    }
        fn water_type<'a>(&'a self) -> Option<&'a str> {
        return self.water_type.as_deref();
    }
        fn features<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.features.as_ref();
    }
        fn residential<'a>(&'a self) -> Option<&'a crate::ResidentialDetails> {
        return self.residential.as_ref();
    }
        fn commercial<'a>(&'a self) -> Option<&'a crate::CommercialDetails> {
        return self.commercial.as_ref();
    }
        fn renovations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Renovation>> {
        return self.renovations.as_ref();
    }
}
impl StructureFacts for crate::StructureState {
        fn kind<'a>(&'a self) -> Option<&'a str> {
        return self.kind.as_deref();
    }
        fn name<'a>(&'a self) -> Option<&'a str> {
        return self.name.as_deref();
    }
        fn structure_number<'a>(&'a self) -> Option<&'a str> {
        return self.structure_number.as_deref();
    }
        fn living_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.living_area.as_ref();
    }
        fn gross_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.gross_area.as_ref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn ground_floor_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.ground_floor_area.as_ref();
    }
        fn basement_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_area.as_ref();
    }
        fn basement_finished_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.basement_finished_area.as_ref();
    }
        fn garage_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.garage_area.as_ref();
    }
        fn areas<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>> {
        return self.areas.as_ref();
    }
        fn year_built(&self) -> Option<isize> {
        return self.year_built;
    }
        fn year_built_estimated(&self) -> Option<bool> {
        return self.year_built_estimated;
    }
        fn effective_year_built(&self) -> Option<isize> {
        return self.effective_year_built;
    }
        fn stories(&self) -> Option<f64> {
        return self.stories;
    }
        fn unit_count(&self) -> Option<isize> {
        return self.unit_count;
    }
        fn construction_method<'a>(&'a self) -> Option<&'a str> {
        return self.construction_method.as_deref();
    }
        fn construction_status<'a>(&'a self) -> Option<&'a str> {
        return self.construction_status.as_deref();
    }
        fn construction_type<'a>(&'a self) -> Option<&'a str> {
        return self.construction_type.as_deref();
    }
        fn exterior_wall_type<'a>(&'a self) -> Option<&'a str> {
        return self.exterior_wall_type.as_deref();
    }
        fn roof_material_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_material_type.as_deref();
    }
        fn roof_style_type<'a>(&'a self) -> Option<&'a str> {
        return self.roof_style_type.as_deref();
    }
        fn foundation_type<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_type.as_deref();
    }
        fn foundation_material<'a>(&'a self) -> Option<&'a str> {
        return self.foundation_material.as_deref();
    }
        fn condition_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.condition_ratings.as_ref();
    }
        fn quality_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        return self.quality_ratings.as_ref();
    }
        fn heating_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.heating_types.as_ref();
    }
        fn heating_fuel_type<'a>(&'a self) -> Option<&'a str> {
        return self.heating_fuel_type.as_deref();
    }
        fn cooling_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.cooling_types.as_ref();
    }
        fn sewer_type<'a>(&'a self) -> Option<&'a str> {
        return self.sewer_type.as_deref();
    }
        fn water_type<'a>(&'a self) -> Option<&'a str> {
        return self.water_type.as_deref();
    }
        fn features<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.features.as_ref();
    }
        fn residential<'a>(&'a self) -> Option<&'a crate::ResidentialDetails> {
        return self.residential.as_ref();
    }
        fn commercial<'a>(&'a self) -> Option<&'a crate::CommercialDetails> {
        return self.commercial.as_ref();
    }
        fn renovations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Renovation>> {
        return self.renovations.as_ref();
    }
}

impl StructureFacts for crate::StructureFactsOrSubtype {
        fn kind<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.kind(),
                StructureFactsOrSubtype::StructureState(val) => val.kind(),

        }
    }
        fn name<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.name(),
                StructureFactsOrSubtype::StructureState(val) => val.name(),

        }
    }
        fn structure_number<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.structure_number(),
                StructureFactsOrSubtype::StructureState(val) => val.structure_number(),

        }
    }
        fn living_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.living_area(),
                StructureFactsOrSubtype::StructureState(val) => val.living_area(),

        }
    }
        fn gross_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.gross_area(),
                StructureFactsOrSubtype::StructureState(val) => val.gross_area(),

        }
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.rentable_area(),
                StructureFactsOrSubtype::StructureState(val) => val.rentable_area(),

        }
    }
        fn ground_floor_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.ground_floor_area(),
                StructureFactsOrSubtype::StructureState(val) => val.ground_floor_area(),

        }
    }
        fn basement_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.basement_area(),
                StructureFactsOrSubtype::StructureState(val) => val.basement_area(),

        }
    }
        fn basement_finished_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.basement_finished_area(),
                StructureFactsOrSubtype::StructureState(val) => val.basement_finished_area(),

        }
    }
        fn garage_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.garage_area(),
                StructureFactsOrSubtype::StructureState(val) => val.garage_area(),

        }
    }
        fn areas<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::AreaMeasure>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.areas().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.areas().map(|x| x.to_any()),

        }
    }
        fn year_built(&self) -> Option<isize> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.year_built(),
                StructureFactsOrSubtype::StructureState(val) => val.year_built(),

        }
    }
        fn year_built_estimated(&self) -> Option<bool> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.year_built_estimated(),
                StructureFactsOrSubtype::StructureState(val) => val.year_built_estimated(),

        }
    }
        fn effective_year_built(&self) -> Option<isize> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.effective_year_built(),
                StructureFactsOrSubtype::StructureState(val) => val.effective_year_built(),

        }
    }
        fn stories(&self) -> Option<f64> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.stories(),
                StructureFactsOrSubtype::StructureState(val) => val.stories(),

        }
    }
        fn unit_count(&self) -> Option<isize> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.unit_count(),
                StructureFactsOrSubtype::StructureState(val) => val.unit_count(),

        }
    }
        fn construction_method<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.construction_method(),
                StructureFactsOrSubtype::StructureState(val) => val.construction_method(),

        }
    }
        fn construction_status<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.construction_status(),
                StructureFactsOrSubtype::StructureState(val) => val.construction_status(),

        }
    }
        fn construction_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.construction_type(),
                StructureFactsOrSubtype::StructureState(val) => val.construction_type(),

        }
    }
        fn exterior_wall_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.exterior_wall_type(),
                StructureFactsOrSubtype::StructureState(val) => val.exterior_wall_type(),

        }
    }
        fn roof_material_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.roof_material_type(),
                StructureFactsOrSubtype::StructureState(val) => val.roof_material_type(),

        }
    }
        fn roof_style_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.roof_style_type(),
                StructureFactsOrSubtype::StructureState(val) => val.roof_style_type(),

        }
    }
        fn foundation_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.foundation_type(),
                StructureFactsOrSubtype::StructureState(val) => val.foundation_type(),

        }
    }
        fn foundation_material<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.foundation_material(),
                StructureFactsOrSubtype::StructureState(val) => val.foundation_material(),

        }
    }
        fn condition_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.condition_ratings().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.condition_ratings().map(|x| x.to_any()),

        }
    }
        fn quality_ratings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Rating>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.quality_ratings().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.quality_ratings().map(|x| x.to_any()),

        }
    }
        fn heating_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.heating_types().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.heating_types().map(|x| x.to_any()),

        }
    }
        fn heating_fuel_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.heating_fuel_type(),
                StructureFactsOrSubtype::StructureState(val) => val.heating_fuel_type(),

        }
    }
        fn cooling_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.cooling_types().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.cooling_types().map(|x| x.to_any()),

        }
    }
        fn sewer_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.sewer_type(),
                StructureFactsOrSubtype::StructureState(val) => val.sewer_type(),

        }
    }
        fn water_type<'a>(&'a self) -> Option<&'a str> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.water_type(),
                StructureFactsOrSubtype::StructureState(val) => val.water_type(),

        }
    }
        fn features<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.features().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.features().map(|x| x.to_any()),

        }
    }
        fn residential<'a>(&'a self) -> Option<&'a crate::ResidentialDetails> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.residential(),
                StructureFactsOrSubtype::StructureState(val) => val.residential(),

        }
    }
        fn commercial<'a>(&'a self) -> Option<&'a crate::CommercialDetails> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.commercial(),
                StructureFactsOrSubtype::StructureState(val) => val.commercial(),

        }
    }
        fn renovations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Renovation>> {
        match self {
                StructureFactsOrSubtype::Structure(val) => val.renovations().map(|x| x.to_any()),
                StructureFactsOrSubtype::StructureState(val) => val.renovations().map(|x| x.to_any()),

        }
    }
}

pub trait Structure : Entity  +  StructureFacts   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;


}

impl Structure for crate::Structure {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
}


pub trait AreaMeasure   {

    fn kind<'a>(&'a self) -> &'a str;
    // fn kind_mut(&mut self) -> &mut &'a str;
    // fn set_kind(&mut self, value: String);

    fn area<'a>(&'a self) -> &'a crate::Area;
    // fn area_mut(&mut self) -> &mut &'a crate::Area;
    // fn set_area<E>(&mut self, value: E) where E: Into<Area>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl AreaMeasure for crate::AreaMeasure {
        fn kind<'a>(&'a self) -> &'a str {
        return &self.kind[..];
    }
        fn area<'a>(&'a self) -> &'a crate::Area {
        return &self.area;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait ResidentialDetails   {

    fn bedrooms_total(&self) -> Option<isize>;
    // fn bedrooms_total_mut(&mut self) -> &mut Option<isize>;
    // fn set_bedrooms_total(&mut self, value: Option<isize>);

    fn bathrooms_full(&self) -> Option<isize>;
    // fn bathrooms_full_mut(&mut self) -> &mut Option<isize>;
    // fn set_bathrooms_full(&mut self, value: Option<isize>);

    fn bathrooms_half(&self) -> Option<isize>;
    // fn bathrooms_half_mut(&mut self) -> &mut Option<isize>;
    // fn set_bathrooms_half(&mut self, value: Option<isize>);

    fn bathrooms_three_quarter(&self) -> Option<isize>;
    // fn bathrooms_three_quarter_mut(&mut self) -> &mut Option<isize>;
    // fn set_bathrooms_three_quarter(&mut self, value: Option<isize>);

    fn rooms_total(&self) -> Option<isize>;
    // fn rooms_total_mut(&mut self) -> &mut Option<isize>;
    // fn set_rooms_total(&mut self, value: Option<isize>);

    fn attachment<'a>(&'a self) -> Option<&'a str>;
    // fn attachment_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_attachment(&mut self, value: Option<&'a str>);

    fn architectural_design<'a>(&'a self) -> Option<&'a str>;
    // fn architectural_design_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_architectural_design(&mut self, value: Option<&'a str>);

    fn basement_type<'a>(&'a self) -> Option<&'a str>;
    // fn basement_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_basement_type(&mut self, value: Option<&'a str>);

    fn garage_type<'a>(&'a self) -> Option<&'a str>;
    // fn garage_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_garage_type(&mut self, value: Option<&'a str>);

    fn garage_attachment<'a>(&'a self) -> Option<&'a str>;
    // fn garage_attachment_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_garage_attachment(&mut self, value: Option<&'a str>);

    fn parking_spaces(&self) -> Option<isize>;
    // fn parking_spaces_mut(&mut self) -> &mut Option<isize>;
    // fn set_parking_spaces(&mut self, value: Option<isize>);

    fn fireplaces(&self) -> Option<isize>;
    // fn fireplaces_mut(&mut self) -> &mut Option<isize>;
    // fn set_fireplaces(&mut self, value: Option<isize>);

    fn has_pool(&self) -> Option<bool>;
    // fn has_pool_mut(&mut self) -> &mut Option<bool>;
    // fn set_has_pool(&mut self, value: Option<bool>);

    fn has_attic(&self) -> Option<bool>;
    // fn has_attic_mut(&mut self) -> &mut Option<bool>;
    // fn set_has_attic(&mut self, value: Option<bool>);

    fn has_adu(&self) -> Option<bool>;
    // fn has_adu_mut(&mut self) -> &mut Option<bool>;
    // fn set_has_adu(&mut self, value: Option<bool>);

    fn adu_legally_rentable(&self) -> Option<bool>;
    // fn adu_legally_rentable_mut(&mut self) -> &mut Option<bool>;
    // fn set_adu_legally_rentable(&mut self, value: Option<bool>);

    fn occupancy<'a>(&'a self) -> Option<&'a str>;
    // fn occupancy_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_occupancy(&mut self, value: Option<&'a str>);

    fn renewable_energy_components<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn renewable_energy_components_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_renewable_energy_components(&mut self, value: Option<&Vec<String>>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl ResidentialDetails for crate::ResidentialDetails {
        fn bedrooms_total(&self) -> Option<isize> {
        return self.bedrooms_total;
    }
        fn bathrooms_full(&self) -> Option<isize> {
        return self.bathrooms_full;
    }
        fn bathrooms_half(&self) -> Option<isize> {
        return self.bathrooms_half;
    }
        fn bathrooms_three_quarter(&self) -> Option<isize> {
        return self.bathrooms_three_quarter;
    }
        fn rooms_total(&self) -> Option<isize> {
        return self.rooms_total;
    }
        fn attachment<'a>(&'a self) -> Option<&'a str> {
        return self.attachment.as_deref();
    }
        fn architectural_design<'a>(&'a self) -> Option<&'a str> {
        return self.architectural_design.as_deref();
    }
        fn basement_type<'a>(&'a self) -> Option<&'a str> {
        return self.basement_type.as_deref();
    }
        fn garage_type<'a>(&'a self) -> Option<&'a str> {
        return self.garage_type.as_deref();
    }
        fn garage_attachment<'a>(&'a self) -> Option<&'a str> {
        return self.garage_attachment.as_deref();
    }
        fn parking_spaces(&self) -> Option<isize> {
        return self.parking_spaces;
    }
        fn fireplaces(&self) -> Option<isize> {
        return self.fireplaces;
    }
        fn has_pool(&self) -> Option<bool> {
        return self.has_pool;
    }
        fn has_attic(&self) -> Option<bool> {
        return self.has_attic;
    }
        fn has_adu(&self) -> Option<bool> {
        return self.has_adu;
    }
        fn adu_legally_rentable(&self) -> Option<bool> {
        return self.adu_legally_rentable;
    }
        fn occupancy<'a>(&'a self) -> Option<&'a str> {
        return self.occupancy.as_deref();
    }
        fn renewable_energy_components<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.renewable_energy_components.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait CommercialDetails   {

    fn market_classification<'a>(&'a self) -> Option<&'a crate::Rating>;
    // fn market_classification_mut(&mut self) -> &mut Option<&'a crate::Rating>;
    // fn set_market_classification<E>(&mut self, value: Option<E>) where E: Into<Rating>;

    fn clear_height<'a>(&'a self) -> Option<&'a crate::Length>;
    // fn clear_height_mut(&mut self) -> &mut Option<&'a crate::Length>;
    // fn set_clear_height<E>(&mut self, value: Option<E>) where E: Into<Length>;

    fn dock_doors(&self) -> Option<isize>;
    // fn dock_doors_mut(&mut self) -> &mut Option<isize>;
    // fn set_dock_doors(&mut self, value: Option<isize>);

    fn drive_in_doors(&self) -> Option<isize>;
    // fn drive_in_doors_mut(&mut self) -> &mut Option<isize>;
    // fn set_drive_in_doors(&mut self, value: Option<isize>);

    fn occupancy_pct(&self) -> Option<f64>;
    // fn occupancy_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_occupancy_pct(&mut self, value: Option<f64>);

    fn parking_spaces(&self) -> Option<isize>;
    // fn parking_spaces_mut(&mut self) -> &mut Option<isize>;
    // fn set_parking_spaces(&mut self, value: Option<isize>);

    fn parking_ratio<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn parking_ratio_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_parking_ratio<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn tenancy<'a>(&'a self) -> Option<&'a str>;
    // fn tenancy_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_tenancy(&mut self, value: Option<&'a str>);

    fn tenant_count(&self) -> Option<isize>;
    // fn tenant_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_tenant_count(&mut self, value: Option<isize>);

    fn parking_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn parking_types_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_parking_types(&mut self, value: Option<&Vec<String>>);

    fn has_sprinkler(&self) -> Option<bool>;
    // fn has_sprinkler_mut(&mut self) -> &mut Option<bool>;
    // fn set_has_sprinkler(&mut self, value: Option<bool>);

    fn elevators(&self) -> Option<isize>;
    // fn elevators_mut(&mut self) -> &mut Option<isize>;
    // fn set_elevators(&mut self, value: Option<isize>);

    fn submarket<'a>(&'a self) -> Option<&'a str>;
    // fn submarket_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_submarket(&mut self, value: Option<&'a str>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl CommercialDetails for crate::CommercialDetails {
        fn market_classification<'a>(&'a self) -> Option<&'a crate::Rating> {
        return self.market_classification.as_ref();
    }
        fn clear_height<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.clear_height.as_ref();
    }
        fn dock_doors(&self) -> Option<isize> {
        return self.dock_doors;
    }
        fn drive_in_doors(&self) -> Option<isize> {
        return self.drive_in_doors;
    }
        fn occupancy_pct(&self) -> Option<f64> {
        return self.occupancy_pct;
    }
        fn parking_spaces(&self) -> Option<isize> {
        return self.parking_spaces;
    }
        fn parking_ratio<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.parking_ratio.as_ref();
    }
        fn tenancy<'a>(&'a self) -> Option<&'a str> {
        return self.tenancy.as_deref();
    }
        fn tenant_count(&self) -> Option<isize> {
        return self.tenant_count;
    }
        fn parking_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.parking_types.as_ref();
    }
        fn has_sprinkler(&self) -> Option<bool> {
        return self.has_sprinkler;
    }
        fn elevators(&self) -> Option<isize> {
        return self.elevators;
    }
        fn submarket<'a>(&'a self) -> Option<&'a str> {
        return self.submarket.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait Renovation   {

    fn kind<'a>(&'a self) -> Option<&'a str>;
    // fn kind_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_kind(&mut self, value: Option<&'a str>);

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn completed_year(&self) -> Option<isize>;
    // fn completed_year_mut(&mut self) -> &mut Option<isize>;
    // fn set_completed_year(&mut self, value: Option<isize>);

    fn completed_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn completed_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_completed_on(&mut self, value: Option<&'a NaiveDate>);

    fn cost<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn cost_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_cost<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl Renovation for crate::Renovation {
        fn kind<'a>(&'a self) -> Option<&'a str> {
        return self.kind.as_deref();
    }
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn completed_year(&self) -> Option<isize> {
        return self.completed_year;
    }
        fn completed_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.completed_on.as_ref();
    }
        fn cost<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.cost.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait SiteFacts   {

    fn lot_size<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn lot_size_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_lot_size<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn usable_land_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn usable_land_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_usable_land_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn usable_land_area_basis<'a>(&'a self) -> Option<&'a str>;
    // fn usable_land_area_basis_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_usable_land_area_basis(&mut self, value: Option<&'a str>);

    fn land_use<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn land_use_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_land_use<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn land_use_category<'a>(&'a self) -> Option<&'a str>;
    // fn land_use_category_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_land_use_category(&mut self, value: Option<&'a str>);

    fn zoning_code<'a>(&'a self) -> Option<&'a str>;
    // fn zoning_code_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_zoning_code(&mut self, value: Option<&'a str>);

    fn flood_zone<'a>(&'a self) -> Option<&'a str>;
    // fn flood_zone_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_flood_zone(&mut self, value: Option<&'a str>);

    fn hazard_zones<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn hazard_zones_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_hazard_zones(&mut self, value: Option<&Vec<String>>);

    fn view_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn view_types_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_view_types(&mut self, value: Option<&Vec<String>>);

    fn site_influences<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn site_influences_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_site_influences(&mut self, value: Option<&Vec<String>>);

    fn easements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn easements_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_easements(&mut self, value: Option<&Vec<String>>);

    fn restrictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn restrictions_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_restrictions(&mut self, value: Option<&Vec<String>>);

    fn utilities<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn utilities_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_utilities(&mut self, value: Option<&Vec<String>>);

    fn frontage<'a>(&'a self) -> Option<&'a crate::Length>;
    // fn frontage_mut(&mut self) -> &mut Option<&'a crate::Length>;
    // fn set_frontage<E>(&mut self, value: Option<E>) where E: Into<Length>;

    fn depth<'a>(&'a self) -> Option<&'a crate::Length>;
    // fn depth_mut(&mut self) -> &mut Option<&'a crate::Length>;
    // fn set_depth<E>(&mut self, value: Option<E>) where E: Into<Length>;

    fn topography<'a>(&'a self) -> Option<&'a str>;
    // fn topography_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_topography(&mut self, value: Option<&'a str>);

    fn is_corner(&self) -> Option<bool>;
    // fn is_corner_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_corner(&mut self, value: Option<bool>);

    fn entitlement_status<'a>(&'a self) -> Option<&'a str>;
    // fn entitlement_status_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_entitlement_status(&mut self, value: Option<&'a str>);

    fn buildable_units(&self) -> Option<isize>;
    // fn buildable_units_mut(&mut self) -> &mut Option<isize>;
    // fn set_buildable_units(&mut self, value: Option<isize>);

    fn subdivision<'a>(&'a self) -> Option<&'a str>;
    // fn subdivision_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_subdivision(&mut self, value: Option<&'a str>);

    fn lot_number<'a>(&'a self) -> Option<&'a str>;
    // fn lot_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_lot_number(&mut self, value: Option<&'a str>);

    fn block<'a>(&'a self) -> Option<&'a str>;
    // fn block_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_block(&mut self, value: Option<&'a str>);

    fn tract_number<'a>(&'a self) -> Option<&'a str>;
    // fn tract_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_tract_number(&mut self, value: Option<&'a str>);

    fn phase_number<'a>(&'a self) -> Option<&'a str>;
    // fn phase_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_phase_number(&mut self, value: Option<&'a str>);

    fn section_township_range<'a>(&'a self) -> Option<&'a str>;
    // fn section_township_range_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_section_township_range(&mut self, value: Option<&'a str>);


}

impl SiteFacts for crate::SiteFacts {
        fn lot_size<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.lot_size.as_ref();
    }
        fn usable_land_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_land_area.as_ref();
    }
        fn usable_land_area_basis<'a>(&'a self) -> Option<&'a str> {
        return self.usable_land_area_basis.as_deref();
    }
        fn land_use<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.land_use.as_ref();
    }
        fn land_use_category<'a>(&'a self) -> Option<&'a str> {
        return self.land_use_category.as_deref();
    }
        fn zoning_code<'a>(&'a self) -> Option<&'a str> {
        return self.zoning_code.as_deref();
    }
        fn flood_zone<'a>(&'a self) -> Option<&'a str> {
        return self.flood_zone.as_deref();
    }
        fn hazard_zones<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.hazard_zones.as_ref();
    }
        fn view_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.view_types.as_ref();
    }
        fn site_influences<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.site_influences.as_ref();
    }
        fn easements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.easements.as_ref();
    }
        fn restrictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.restrictions.as_ref();
    }
        fn utilities<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.utilities.as_ref();
    }
        fn frontage<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.frontage.as_ref();
    }
        fn depth<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.depth.as_ref();
    }
        fn topography<'a>(&'a self) -> Option<&'a str> {
        return self.topography.as_deref();
    }
        fn is_corner(&self) -> Option<bool> {
        return self.is_corner;
    }
        fn entitlement_status<'a>(&'a self) -> Option<&'a str> {
        return self.entitlement_status.as_deref();
    }
        fn buildable_units(&self) -> Option<isize> {
        return self.buildable_units;
    }
        fn subdivision<'a>(&'a self) -> Option<&'a str> {
        return self.subdivision.as_deref();
    }
        fn lot_number<'a>(&'a self) -> Option<&'a str> {
        return self.lot_number.as_deref();
    }
        fn block<'a>(&'a self) -> Option<&'a str> {
        return self.block.as_deref();
    }
        fn tract_number<'a>(&'a self) -> Option<&'a str> {
        return self.tract_number.as_deref();
    }
        fn phase_number<'a>(&'a self) -> Option<&'a str> {
        return self.phase_number.as_deref();
    }
        fn section_township_range<'a>(&'a self) -> Option<&'a str> {
        return self.section_township_range.as_deref();
    }
}
impl SiteFacts for crate::Site {
        fn lot_size<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.lot_size.as_ref();
    }
        fn usable_land_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_land_area.as_ref();
    }
        fn usable_land_area_basis<'a>(&'a self) -> Option<&'a str> {
        return self.usable_land_area_basis.as_deref();
    }
        fn land_use<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.land_use.as_ref();
    }
        fn land_use_category<'a>(&'a self) -> Option<&'a str> {
        return self.land_use_category.as_deref();
    }
        fn zoning_code<'a>(&'a self) -> Option<&'a str> {
        return self.zoning_code.as_deref();
    }
        fn flood_zone<'a>(&'a self) -> Option<&'a str> {
        return self.flood_zone.as_deref();
    }
        fn hazard_zones<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.hazard_zones.as_ref();
    }
        fn view_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.view_types.as_ref();
    }
        fn site_influences<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.site_influences.as_ref();
    }
        fn easements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.easements.as_ref();
    }
        fn restrictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.restrictions.as_ref();
    }
        fn utilities<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.utilities.as_ref();
    }
        fn frontage<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.frontage.as_ref();
    }
        fn depth<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.depth.as_ref();
    }
        fn topography<'a>(&'a self) -> Option<&'a str> {
        return self.topography.as_deref();
    }
        fn is_corner(&self) -> Option<bool> {
        return self.is_corner;
    }
        fn entitlement_status<'a>(&'a self) -> Option<&'a str> {
        return self.entitlement_status.as_deref();
    }
        fn buildable_units(&self) -> Option<isize> {
        return self.buildable_units;
    }
        fn subdivision<'a>(&'a self) -> Option<&'a str> {
        return self.subdivision.as_deref();
    }
        fn lot_number<'a>(&'a self) -> Option<&'a str> {
        return self.lot_number.as_deref();
    }
        fn block<'a>(&'a self) -> Option<&'a str> {
        return self.block.as_deref();
    }
        fn tract_number<'a>(&'a self) -> Option<&'a str> {
        return self.tract_number.as_deref();
    }
        fn phase_number<'a>(&'a self) -> Option<&'a str> {
        return self.phase_number.as_deref();
    }
        fn section_township_range<'a>(&'a self) -> Option<&'a str> {
        return self.section_township_range.as_deref();
    }
}
impl SiteFacts for crate::SiteState {
        fn lot_size<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.lot_size.as_ref();
    }
        fn usable_land_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_land_area.as_ref();
    }
        fn usable_land_area_basis<'a>(&'a self) -> Option<&'a str> {
        return self.usable_land_area_basis.as_deref();
    }
        fn land_use<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.land_use.as_ref();
    }
        fn land_use_category<'a>(&'a self) -> Option<&'a str> {
        return self.land_use_category.as_deref();
    }
        fn zoning_code<'a>(&'a self) -> Option<&'a str> {
        return self.zoning_code.as_deref();
    }
        fn flood_zone<'a>(&'a self) -> Option<&'a str> {
        return self.flood_zone.as_deref();
    }
        fn hazard_zones<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.hazard_zones.as_ref();
    }
        fn view_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.view_types.as_ref();
    }
        fn site_influences<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.site_influences.as_ref();
    }
        fn easements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.easements.as_ref();
    }
        fn restrictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.restrictions.as_ref();
    }
        fn utilities<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.utilities.as_ref();
    }
        fn frontage<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.frontage.as_ref();
    }
        fn depth<'a>(&'a self) -> Option<&'a crate::Length> {
        return self.depth.as_ref();
    }
        fn topography<'a>(&'a self) -> Option<&'a str> {
        return self.topography.as_deref();
    }
        fn is_corner(&self) -> Option<bool> {
        return self.is_corner;
    }
        fn entitlement_status<'a>(&'a self) -> Option<&'a str> {
        return self.entitlement_status.as_deref();
    }
        fn buildable_units(&self) -> Option<isize> {
        return self.buildable_units;
    }
        fn subdivision<'a>(&'a self) -> Option<&'a str> {
        return self.subdivision.as_deref();
    }
        fn lot_number<'a>(&'a self) -> Option<&'a str> {
        return self.lot_number.as_deref();
    }
        fn block<'a>(&'a self) -> Option<&'a str> {
        return self.block.as_deref();
    }
        fn tract_number<'a>(&'a self) -> Option<&'a str> {
        return self.tract_number.as_deref();
    }
        fn phase_number<'a>(&'a self) -> Option<&'a str> {
        return self.phase_number.as_deref();
    }
        fn section_township_range<'a>(&'a self) -> Option<&'a str> {
        return self.section_township_range.as_deref();
    }
}

impl SiteFacts for crate::SiteFactsOrSubtype {
        fn lot_size<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.lot_size(),
                SiteFactsOrSubtype::SiteState(val) => val.lot_size(),

        }
    }
        fn usable_land_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.usable_land_area(),
                SiteFactsOrSubtype::SiteState(val) => val.usable_land_area(),

        }
    }
        fn usable_land_area_basis<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.usable_land_area_basis(),
                SiteFactsOrSubtype::SiteState(val) => val.usable_land_area_basis(),

        }
    }
        fn land_use<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.land_use(),
                SiteFactsOrSubtype::SiteState(val) => val.land_use(),

        }
    }
        fn land_use_category<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.land_use_category(),
                SiteFactsOrSubtype::SiteState(val) => val.land_use_category(),

        }
    }
        fn zoning_code<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.zoning_code(),
                SiteFactsOrSubtype::SiteState(val) => val.zoning_code(),

        }
    }
        fn flood_zone<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.flood_zone(),
                SiteFactsOrSubtype::SiteState(val) => val.flood_zone(),

        }
    }
        fn hazard_zones<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.hazard_zones().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.hazard_zones().map(|x| x.to_any()),

        }
    }
        fn view_types<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.view_types().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.view_types().map(|x| x.to_any()),

        }
    }
        fn site_influences<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.site_influences().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.site_influences().map(|x| x.to_any()),

        }
    }
        fn easements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.easements().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.easements().map(|x| x.to_any()),

        }
    }
        fn restrictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.restrictions().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.restrictions().map(|x| x.to_any()),

        }
    }
        fn utilities<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.utilities().map(|x| x.to_any()),
                SiteFactsOrSubtype::SiteState(val) => val.utilities().map(|x| x.to_any()),

        }
    }
        fn frontage<'a>(&'a self) -> Option<&'a crate::Length> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.frontage(),
                SiteFactsOrSubtype::SiteState(val) => val.frontage(),

        }
    }
        fn depth<'a>(&'a self) -> Option<&'a crate::Length> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.depth(),
                SiteFactsOrSubtype::SiteState(val) => val.depth(),

        }
    }
        fn topography<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.topography(),
                SiteFactsOrSubtype::SiteState(val) => val.topography(),

        }
    }
        fn is_corner(&self) -> Option<bool> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.is_corner(),
                SiteFactsOrSubtype::SiteState(val) => val.is_corner(),

        }
    }
        fn entitlement_status<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.entitlement_status(),
                SiteFactsOrSubtype::SiteState(val) => val.entitlement_status(),

        }
    }
        fn buildable_units(&self) -> Option<isize> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.buildable_units(),
                SiteFactsOrSubtype::SiteState(val) => val.buildable_units(),

        }
    }
        fn subdivision<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.subdivision(),
                SiteFactsOrSubtype::SiteState(val) => val.subdivision(),

        }
    }
        fn lot_number<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.lot_number(),
                SiteFactsOrSubtype::SiteState(val) => val.lot_number(),

        }
    }
        fn block<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.block(),
                SiteFactsOrSubtype::SiteState(val) => val.block(),

        }
    }
        fn tract_number<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.tract_number(),
                SiteFactsOrSubtype::SiteState(val) => val.tract_number(),

        }
    }
        fn phase_number<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.phase_number(),
                SiteFactsOrSubtype::SiteState(val) => val.phase_number(),

        }
    }
        fn section_township_range<'a>(&'a self) -> Option<&'a str> {
        match self {
                SiteFactsOrSubtype::Site(val) => val.section_township_range(),
                SiteFactsOrSubtype::SiteState(val) => val.section_township_range(),

        }
    }
}

pub trait Site : Entity  +  SiteFacts   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;


}

impl Site for crate::Site {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
}


pub trait SpaceFacts   {

    fn floor_number(&self) -> Option<isize>;
    // fn floor_number_mut(&mut self) -> &mut Option<isize>;
    // fn set_floor_number(&mut self, value: Option<isize>);

    fn space_use<'a>(&'a self) -> Option<&'a str>;
    // fn space_use_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_space_use(&mut self, value: Option<&'a str>);

    fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn rentable_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_rentable_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn usable_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn usable_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_usable_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn bedrooms(&self) -> Option<isize>;
    // fn bedrooms_mut(&mut self) -> &mut Option<isize>;
    // fn set_bedrooms(&mut self, value: Option<isize>);

    fn bathrooms(&self) -> Option<f64>;
    // fn bathrooms_mut(&mut self) -> &mut Option<f64>;
    // fn set_bathrooms(&mut self, value: Option<f64>);

    fn occupancy<'a>(&'a self) -> Option<&'a str>;
    // fn occupancy_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_occupancy(&mut self, value: Option<&'a str>);

    fn is_adu(&self) -> Option<bool>;
    // fn is_adu_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_adu(&mut self, value: Option<bool>);

    fn is_active(&self) -> Option<bool>;
    // fn is_active_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_active(&mut self, value: Option<bool>);


}

impl SpaceFacts for crate::SpaceFacts {
        fn floor_number(&self) -> Option<isize> {
        return self.floor_number;
    }
        fn space_use<'a>(&'a self) -> Option<&'a str> {
        return self.space_use.as_deref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn usable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_area.as_ref();
    }
        fn bedrooms(&self) -> Option<isize> {
        return self.bedrooms;
    }
        fn bathrooms(&self) -> Option<f64> {
        return self.bathrooms;
    }
        fn occupancy<'a>(&'a self) -> Option<&'a str> {
        return self.occupancy.as_deref();
    }
        fn is_adu(&self) -> Option<bool> {
        return self.is_adu;
    }
        fn is_active(&self) -> Option<bool> {
        return self.is_active;
    }
}
impl SpaceFacts for crate::Space {
        fn floor_number(&self) -> Option<isize> {
        return self.floor_number;
    }
        fn space_use<'a>(&'a self) -> Option<&'a str> {
        return self.space_use.as_deref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn usable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_area.as_ref();
    }
        fn bedrooms(&self) -> Option<isize> {
        return self.bedrooms;
    }
        fn bathrooms(&self) -> Option<f64> {
        return self.bathrooms;
    }
        fn occupancy<'a>(&'a self) -> Option<&'a str> {
        return self.occupancy.as_deref();
    }
        fn is_adu(&self) -> Option<bool> {
        return self.is_adu;
    }
        fn is_active(&self) -> Option<bool> {
        return self.is_active;
    }
}
impl SpaceFacts for crate::SpaceState {
        fn floor_number(&self) -> Option<isize> {
        return self.floor_number;
    }
        fn space_use<'a>(&'a self) -> Option<&'a str> {
        return self.space_use.as_deref();
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.rentable_area.as_ref();
    }
        fn usable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.usable_area.as_ref();
    }
        fn bedrooms(&self) -> Option<isize> {
        return self.bedrooms;
    }
        fn bathrooms(&self) -> Option<f64> {
        return self.bathrooms;
    }
        fn occupancy<'a>(&'a self) -> Option<&'a str> {
        return self.occupancy.as_deref();
    }
        fn is_adu(&self) -> Option<bool> {
        return self.is_adu;
    }
        fn is_active(&self) -> Option<bool> {
        return self.is_active;
    }
}

impl SpaceFacts for crate::SpaceFactsOrSubtype {
        fn floor_number(&self) -> Option<isize> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.floor_number(),
                SpaceFactsOrSubtype::SpaceState(val) => val.floor_number(),

        }
    }
        fn space_use<'a>(&'a self) -> Option<&'a str> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.space_use(),
                SpaceFactsOrSubtype::SpaceState(val) => val.space_use(),

        }
    }
        fn rentable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.rentable_area(),
                SpaceFactsOrSubtype::SpaceState(val) => val.rentable_area(),

        }
    }
        fn usable_area<'a>(&'a self) -> Option<&'a crate::Area> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.usable_area(),
                SpaceFactsOrSubtype::SpaceState(val) => val.usable_area(),

        }
    }
        fn bedrooms(&self) -> Option<isize> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.bedrooms(),
                SpaceFactsOrSubtype::SpaceState(val) => val.bedrooms(),

        }
    }
        fn bathrooms(&self) -> Option<f64> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.bathrooms(),
                SpaceFactsOrSubtype::SpaceState(val) => val.bathrooms(),

        }
    }
        fn occupancy<'a>(&'a self) -> Option<&'a str> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.occupancy(),
                SpaceFactsOrSubtype::SpaceState(val) => val.occupancy(),

        }
    }
        fn is_adu(&self) -> Option<bool> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.is_adu(),
                SpaceFactsOrSubtype::SpaceState(val) => val.is_adu(),

        }
    }
        fn is_active(&self) -> Option<bool> {
        match self {
                SpaceFactsOrSubtype::Space(val) => val.is_active(),
                SpaceFactsOrSubtype::SpaceState(val) => val.is_active(),

        }
    }
}

pub trait Space : Entity  +  SpaceFacts   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn structure<'a>(&'a self) -> Option<&'a str>;
    // fn structure_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_structure<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn space_identifier<'a>(&'a self) -> &'a str;
    // fn space_identifier_mut(&mut self) -> &mut &'a str;
    // fn set_space_identifier(&mut self, value: String);


}

impl Space for crate::Space {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn structure<'a>(&'a self) -> Option<&'a str> {
        return self.structure.as_deref();
    }
        fn space_identifier<'a>(&'a self) -> &'a str {
        return &self.space_identifier[..];
    }
}


pub trait PropertyState : Entity  +  PropertyFacts   {

    fn subject<'a>(&'a self) -> &'a str;
    // fn subject_mut(&mut self) -> &mut &'a str;
    // fn set_subject<E>(&mut self, value: String) where E: Into<String>;


}

impl PropertyState for crate::PropertyState {
        fn subject<'a>(&'a self) -> &'a str {
        return &self.subject[..];
    }
}


pub trait SiteState : Entity  +  SiteFacts   {

    fn subject<'a>(&'a self) -> &'a str;
    // fn subject_mut(&mut self) -> &mut &'a str;
    // fn set_subject<E>(&mut self, value: String) where E: Into<String>;


}

impl SiteState for crate::SiteState {
        fn subject<'a>(&'a self) -> &'a str {
        return &self.subject[..];
    }
}


pub trait StructureState : Entity  +  StructureFacts   {

    fn subject<'a>(&'a self) -> &'a str;
    // fn subject_mut(&mut self) -> &mut &'a str;
    // fn set_subject<E>(&mut self, value: String) where E: Into<String>;


}

impl StructureState for crate::StructureState {
        fn subject<'a>(&'a self) -> &'a str {
        return &self.subject[..];
    }
}


pub trait SpaceState : Entity  +  SpaceFacts   {

    fn subject<'a>(&'a self) -> &'a str;
    // fn subject_mut(&mut self) -> &mut &'a str;
    // fn set_subject<E>(&mut self, value: String) where E: Into<String>;


}

impl SpaceState for crate::SpaceState {
        fn subject<'a>(&'a self) -> &'a str {
        return &self.subject[..];
    }
}


pub trait PropertyStateSnapshot : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn as_of_date_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_as_of_date(&mut self, value: NaiveDate);

    fn basis<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn basis_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_basis<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn property_state<'a>(&'a self) -> Option<&'a crate::PropertyState>;
    // fn property_state_mut(&mut self) -> &mut Option<&'a crate::PropertyState>;
    // fn set_property_state<E>(&mut self, value: Option<E>) where E: Into<PropertyState>;

    fn site_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SiteState>>;
    // fn site_states_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::SiteState>>;
    // fn set_site_states<E>(&mut self, value: Option<&Vec<E>>) where E: Into<SiteState>;

    fn structure_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StructureState>>;
    // fn structure_states_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::StructureState>>;
    // fn set_structure_states<E>(&mut self, value: Option<&Vec<E>>) where E: Into<StructureState>;

    fn space_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SpaceState>>;
    // fn space_states_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::SpaceState>>;
    // fn set_space_states<E>(&mut self, value: Option<&Vec<E>>) where E: Into<SpaceState>;


}

impl PropertyStateSnapshot for crate::PropertyStateSnapshot {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.as_of_date;
    }
        fn basis<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.basis.as_ref();
    }
        fn property_state<'a>(&'a self) -> Option<&'a crate::PropertyState> {
        return self.property_state.as_ref();
    }
        fn site_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SiteState>> {
        return self.site_states.as_ref();
    }
        fn structure_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StructureState>> {
        return self.structure_states.as_ref();
    }
        fn space_states<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SpaceState>> {
        return self.space_states.as_ref();
    }
}


pub trait PropertyAssociation : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn party<'a>(&'a self) -> &'a str;
    // fn party_mut(&mut self) -> &mut &'a str;
    // fn set_party<E>(&mut self, value: String) where E: Into<String>;

    fn fee<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn fee_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_fee<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn fee_period<'a>(&'a self) -> Option<&'a crate::RentPeriod>;
    // fn fee_period_mut(&mut self) -> &mut Option<&'a crate::RentPeriod>;
    // fn set_fee_period(&mut self, value: Option<&'a RentPeriod>);


}

impl PropertyAssociation for crate::PropertyAssociation {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn party<'a>(&'a self) -> &'a str {
        return &self.party[..];
    }
        fn fee<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.fee.as_ref();
    }
        fn fee_period<'a>(&'a self) -> Option<&'a crate::RentPeriod> {
        return self.fee_period.as_ref();
    }
}


pub trait Assessment : Entity   {

    fn parcel<'a>(&'a self) -> &'a str;
    // fn parcel_mut(&mut self) -> &mut &'a str;
    // fn set_parcel<E>(&mut self, value: String) where E: Into<String>;

    fn jurisdiction<'a>(&'a self) -> &'a str;
    // fn jurisdiction_mut(&mut self) -> &mut &'a str;
    // fn set_jurisdiction<E>(&mut self, value: String) where E: Into<String>;

    fn tax_year(&self) -> isize;
    // fn tax_year_mut(&mut self) -> &mut isize;
    // fn set_tax_year(&mut self, value: isize);

    fn roll_type<'a>(&'a self) -> Option<&'a str>;
    // fn roll_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_roll_type(&mut self, value: Option<&'a str>);

    fn assessed_land_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn assessed_land_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_assessed_land_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn assessed_improvement_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn assessed_improvement_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_assessed_improvement_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn assessed_total_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn assessed_total_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_assessed_total_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn market_land_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn market_land_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_market_land_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn market_improvement_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn market_improvement_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_market_improvement_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn market_total_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn market_total_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_market_total_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn appraised_land_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn appraised_land_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_appraised_land_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn appraised_improvement_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn appraised_improvement_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_appraised_improvement_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn appraised_total_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn appraised_total_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_appraised_total_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn exemptions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxExemption>>;
    // fn exemptions_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::TaxExemption>>;
    // fn set_exemptions<E>(&mut self, value: Option<&Vec<E>>) where E: Into<TaxExemption>;


}

impl Assessment for crate::Assessment {
        fn parcel<'a>(&'a self) -> &'a str {
        return &self.parcel[..];
    }
        fn jurisdiction<'a>(&'a self) -> &'a str {
        return &self.jurisdiction[..];
    }
        fn tax_year(&self) -> isize {
        return self.tax_year;
    }
        fn roll_type<'a>(&'a self) -> Option<&'a str> {
        return self.roll_type.as_deref();
    }
        fn assessed_land_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.assessed_land_value.as_ref();
    }
        fn assessed_improvement_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.assessed_improvement_value.as_ref();
    }
        fn assessed_total_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.assessed_total_value.as_ref();
    }
        fn market_land_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.market_land_value.as_ref();
    }
        fn market_improvement_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.market_improvement_value.as_ref();
    }
        fn market_total_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.market_total_value.as_ref();
    }
        fn appraised_land_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.appraised_land_value.as_ref();
    }
        fn appraised_improvement_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.appraised_improvement_value.as_ref();
    }
        fn appraised_total_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.appraised_total_value.as_ref();
    }
        fn exemptions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxExemption>> {
        return self.exemptions.as_ref();
    }
}


pub trait TaxExemption   {

    fn kind<'a>(&'a self) -> &'a str;
    // fn kind_mut(&mut self) -> &mut &'a str;
    // fn set_kind(&mut self, value: String);

    fn amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl TaxExemption for crate::TaxExemption {
        fn kind<'a>(&'a self) -> &'a str {
        return &self.kind[..];
    }
        fn amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait TaxBill : Entity   {

    fn parcel<'a>(&'a self) -> &'a str;
    // fn parcel_mut(&mut self) -> &mut &'a str;
    // fn set_parcel<E>(&mut self, value: String) where E: Into<String>;

    fn jurisdiction<'a>(&'a self) -> &'a str;
    // fn jurisdiction_mut(&mut self) -> &mut &'a str;
    // fn set_jurisdiction<E>(&mut self, value: String) where E: Into<String>;

    fn tax_year(&self) -> isize;
    // fn tax_year_mut(&mut self) -> &mut isize;
    // fn set_tax_year(&mut self, value: isize);

    fn bill_number<'a>(&'a self) -> Option<&'a str>;
    // fn bill_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_bill_number(&mut self, value: Option<&'a str>);

    fn amount_billed<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_billed_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount_billed<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn amount_paid<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_paid_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount_paid<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn is_delinquent(&self) -> Option<bool>;
    // fn is_delinquent_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_delinquent(&mut self, value: Option<bool>);

    fn delinquent_year(&self) -> Option<isize>;
    // fn delinquent_year_mut(&mut self) -> &mut Option<isize>;
    // fn set_delinquent_year(&mut self, value: Option<isize>);

    fn delinquent_amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn delinquent_amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_delinquent_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn rate_code_area<'a>(&'a self) -> Option<&'a str>;
    // fn rate_code_area_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_rate_code_area(&mut self, value: Option<&'a str>);

    fn installments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxInstallment>>;
    // fn installments_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::TaxInstallment>>;
    // fn set_installments<E>(&mut self, value: Option<&Vec<E>>) where E: Into<TaxInstallment>;

    fn line_items<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxLineItem>>;
    // fn line_items_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::TaxLineItem>>;
    // fn set_line_items<E>(&mut self, value: Option<&Vec<E>>) where E: Into<TaxLineItem>;


}

impl TaxBill for crate::TaxBill {
        fn parcel<'a>(&'a self) -> &'a str {
        return &self.parcel[..];
    }
        fn jurisdiction<'a>(&'a self) -> &'a str {
        return &self.jurisdiction[..];
    }
        fn tax_year(&self) -> isize {
        return self.tax_year;
    }
        fn bill_number<'a>(&'a self) -> Option<&'a str> {
        return self.bill_number.as_deref();
    }
        fn amount_billed<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount_billed.as_ref();
    }
        fn amount_paid<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount_paid.as_ref();
    }
        fn is_delinquent(&self) -> Option<bool> {
        return self.is_delinquent;
    }
        fn delinquent_year(&self) -> Option<isize> {
        return self.delinquent_year;
    }
        fn delinquent_amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.delinquent_amount.as_ref();
    }
        fn rate_code_area<'a>(&'a self) -> Option<&'a str> {
        return self.rate_code_area.as_deref();
    }
        fn installments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxInstallment>> {
        return self.installments.as_ref();
    }
        fn line_items<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxLineItem>> {
        return self.line_items.as_ref();
    }
}


pub trait TaxInstallment   {

    fn installment_number(&self) -> Option<isize>;
    // fn installment_number_mut(&mut self) -> &mut Option<isize>;
    // fn set_installment_number(&mut self, value: Option<isize>);

    fn due_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn due_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_due_on(&mut self, value: Option<&'a NaiveDate>);

    fn amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn paid_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn paid_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_paid_on(&mut self, value: Option<&'a NaiveDate>);

    fn amount_paid<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_paid_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount_paid<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn is_delinquent(&self) -> Option<bool>;
    // fn is_delinquent_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_delinquent(&mut self, value: Option<bool>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl TaxInstallment for crate::TaxInstallment {
        fn installment_number(&self) -> Option<isize> {
        return self.installment_number;
    }
        fn due_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.due_on.as_ref();
    }
        fn amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount.as_ref();
    }
        fn paid_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.paid_on.as_ref();
    }
        fn amount_paid<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount_paid.as_ref();
    }
        fn is_delinquent(&self) -> Option<bool> {
        return self.is_delinquent;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait TaxLineItem   {

    fn jurisdiction<'a>(&'a self) -> Option<&'a str>;
    // fn jurisdiction_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_jurisdiction<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn rate(&self) -> Option<f64>;
    // fn rate_mut(&mut self) -> &mut Option<f64>;
    // fn set_rate(&mut self, value: Option<f64>);

    fn amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl TaxLineItem for crate::TaxLineItem {
        fn jurisdiction<'a>(&'a self) -> Option<&'a str> {
        return self.jurisdiction.as_deref();
    }
        fn rate(&self) -> Option<f64> {
        return self.rate;
    }
        fn amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait Transfer : Entity  +  RecordedInstrument   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn parcel<'a>(&'a self) -> Option<&'a str>;
    // fn parcel_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_parcel<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn transfer_kind<'a>(&'a self) -> &'a str;
    // fn transfer_kind_mut(&mut self) -> &mut &'a str;
    // fn set_transfer_kind(&mut self, value: String);

    fn effective_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn effective_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_effective_on(&mut self, value: Option<&'a NaiveDate>);

    fn consideration<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn consideration_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_consideration<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn transfer_tax<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn transfer_tax_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_transfer_tax<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn price_disclosure<'a>(&'a self) -> Option<&'a crate::PriceDisclosure>;
    // fn price_disclosure_mut(&mut self) -> &mut Option<&'a crate::PriceDisclosure>;
    // fn set_price_disclosure(&mut self, value: Option<&'a PriceDisclosure>);

    fn price_code<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn price_code_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_price_code<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn interest_conveyed<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn interest_conveyed_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_interest_conveyed<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn partial_interest_pct(&self) -> Option<f64>;
    // fn partial_interest_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_partial_interest_pct(&mut self, value: Option<f64>);

    fn is_inter_family(&self) -> Option<bool>;
    // fn is_inter_family_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_inter_family(&mut self, value: Option<bool>);

    fn is_distressed(&self) -> Option<bool>;
    // fn is_distressed_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_distressed(&mut self, value: Option<bool>);

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TransferParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::TransferParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<TransferParty>;


}

impl Transfer for crate::Transfer {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn parcel<'a>(&'a self) -> Option<&'a str> {
        return self.parcel.as_deref();
    }
        fn transfer_kind<'a>(&'a self) -> &'a str {
        return &self.transfer_kind[..];
    }
        fn effective_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.effective_on.as_ref();
    }
        fn consideration<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.consideration.as_ref();
    }
        fn transfer_tax<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.transfer_tax.as_ref();
    }
        fn price_disclosure<'a>(&'a self) -> Option<&'a crate::PriceDisclosure> {
        return self.price_disclosure.as_ref();
    }
        fn price_code<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.price_code.as_ref();
    }
        fn interest_conveyed<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.interest_conveyed.as_ref();
    }
        fn partial_interest_pct(&self) -> Option<f64> {
        return self.partial_interest_pct;
    }
        fn is_inter_family(&self) -> Option<bool> {
        return self.is_inter_family;
    }
        fn is_distressed(&self) -> Option<bool> {
        return self.is_distressed;
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TransferParty>> {
        return self.parties.as_ref();
    }
}


pub trait TransferParty : TransactionParty   {


}

impl TransferParty for crate::TransferParty {
}


pub trait SaleEvent : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn property_state<'a>(&'a self) -> Option<&'a str>;
    // fn property_state_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_state<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn transfer<'a>(&'a self) -> Option<&'a str>;
    // fn transfer_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_transfer<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn sale_date<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn sale_date_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_sale_date(&mut self, value: NaiveDate);

    fn sale_price<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn sale_price_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_sale_price<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn price_disclosure<'a>(&'a self) -> Option<&'a crate::PriceDisclosure>;
    // fn price_disclosure_mut(&mut self) -> &mut Option<&'a crate::PriceDisclosure>;
    // fn set_price_disclosure(&mut self, value: Option<&'a PriceDisclosure>);

    fn price_code<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn price_code_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_price_code<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn sale_type<'a>(&'a self) -> Option<&'a crate::SaleTypeEnum>;
    // fn sale_type_mut(&mut self) -> &mut Option<&'a crate::SaleTypeEnum>;
    // fn set_sale_type(&mut self, value: Option<&'a SaleTypeEnum>);

    fn price_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn price_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_price_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn price_per_unit<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn price_per_unit_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_price_per_unit<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn financing<'a>(&'a self) -> Option<&'a str>;
    // fn financing_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_financing(&mut self, value: Option<&'a str>);

    fn concessions<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn concessions_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_concessions<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn cap_rate(&self) -> Option<f64>;
    // fn cap_rate_mut(&mut self) -> &mut Option<f64>;
    // fn set_cap_rate(&mut self, value: Option<f64>);

    fn noi_at_sale<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn noi_at_sale_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_noi_at_sale<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn opex_at_sale<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn opex_at_sale_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_opex_at_sale<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn occupancy_at_sale_pct(&self) -> Option<f64>;
    // fn occupancy_at_sale_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_occupancy_at_sale_pct(&mut self, value: Option<f64>);

    fn unit_count_at_sale(&self) -> Option<isize>;
    // fn unit_count_at_sale_mut(&mut self) -> &mut Option<isize>;
    // fn set_unit_count_at_sale(&mut self, value: Option<isize>);

    fn supporting_operating_statement<'a>(&'a self) -> Option<&'a str>;
    // fn supporting_operating_statement_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_supporting_operating_statement<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SaleEventParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::SaleEventParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<SaleEventParty>;

    fn remarks<'a>(&'a self) -> Option<&'a str>;
    // fn remarks_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_remarks(&mut self, value: Option<&'a str>);


}

impl SaleEvent for crate::SaleEvent {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn property_state<'a>(&'a self) -> Option<&'a str> {
        return self.property_state.as_deref();
    }
        fn transfer<'a>(&'a self) -> Option<&'a str> {
        return self.transfer.as_deref();
    }
        fn sale_date<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.sale_date;
    }
        fn sale_price<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.sale_price.as_ref();
    }
        fn price_disclosure<'a>(&'a self) -> Option<&'a crate::PriceDisclosure> {
        return self.price_disclosure.as_ref();
    }
        fn price_code<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.price_code.as_ref();
    }
        fn sale_type<'a>(&'a self) -> Option<&'a crate::SaleTypeEnum> {
        return self.sale_type.as_ref();
    }
        fn price_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.price_per_area.as_ref();
    }
        fn price_per_unit<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.price_per_unit.as_ref();
    }
        fn financing<'a>(&'a self) -> Option<&'a str> {
        return self.financing.as_deref();
    }
        fn concessions<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.concessions.as_ref();
    }
        fn cap_rate(&self) -> Option<f64> {
        return self.cap_rate;
    }
        fn noi_at_sale<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.noi_at_sale.as_ref();
    }
        fn opex_at_sale<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.opex_at_sale.as_ref();
    }
        fn occupancy_at_sale_pct(&self) -> Option<f64> {
        return self.occupancy_at_sale_pct;
    }
        fn unit_count_at_sale(&self) -> Option<isize> {
        return self.unit_count_at_sale;
    }
        fn supporting_operating_statement<'a>(&'a self) -> Option<&'a str> {
        return self.supporting_operating_statement.as_deref();
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SaleEventParty>> {
        return self.parties.as_ref();
    }
        fn remarks<'a>(&'a self) -> Option<&'a str> {
        return self.remarks.as_deref();
    }
}


pub trait SaleEventParty : TransactionParty   {


}

impl SaleEventParty for crate::SaleEventParty {
}


pub trait Listing : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn property_state<'a>(&'a self) -> Option<&'a str>;
    // fn property_state_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_state<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn kind<'a>(&'a self) -> &'a crate::ListingKind;
    // fn kind_mut(&mut self) -> &mut &'a crate::ListingKind;
    // fn set_kind(&mut self, value: ListingKind);

    fn listing_type<'a>(&'a self) -> Option<&'a str>;
    // fn listing_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_listing_type(&mut self, value: Option<&'a str>);

    fn mls_number<'a>(&'a self) -> Option<&'a str>;
    // fn mls_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_mls_number(&mut self, value: Option<&'a str>);

    fn events<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ListingEvent>>;
    // fn events_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ListingEvent>>;
    // fn set_events<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ListingEvent>;

    fn participants<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ListingParticipant>>;
    // fn participants_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ListingParticipant>>;
    // fn set_participants<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ListingParticipant>;

    fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifacts<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;

    fn remarks<'a>(&'a self) -> Option<&'a str>;
    // fn remarks_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_remarks(&mut self, value: Option<&'a str>);


}

impl Listing for crate::Listing {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn property_state<'a>(&'a self) -> Option<&'a str> {
        return self.property_state.as_deref();
    }
        fn kind<'a>(&'a self) -> &'a crate::ListingKind {
        return &self.kind;
    }
        fn listing_type<'a>(&'a self) -> Option<&'a str> {
        return self.listing_type.as_deref();
    }
        fn mls_number<'a>(&'a self) -> Option<&'a str> {
        return self.mls_number.as_deref();
    }
        fn events<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ListingEvent>> {
        return self.events.as_ref();
    }
        fn participants<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ListingParticipant>> {
        return self.participants.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
        fn remarks<'a>(&'a self) -> Option<&'a str> {
        return self.remarks.as_deref();
    }
}


pub trait ListingEvent   {

    fn occurred_on<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn occurred_on_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_occurred_on(&mut self, value: NaiveDate);

    fn event_kind<'a>(&'a self) -> &'a str;
    // fn event_kind_mut(&mut self) -> &mut &'a str;
    // fn set_event_kind(&mut self, value: String);

    fn status<'a>(&'a self) -> Option<&'a crate::ListingStatus>;
    // fn status_mut(&mut self) -> &mut Option<&'a crate::ListingStatus>;
    // fn set_status(&mut self, value: Option<&'a ListingStatus>);

    fn asking_price<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn asking_price_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_asking_price<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod>;
    // fn rent_period_mut(&mut self) -> &mut Option<&'a crate::RentPeriod>;
    // fn set_rent_period(&mut self, value: Option<&'a RentPeriod>);

    fn close_price<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn close_price_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_close_price<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl ListingEvent for crate::ListingEvent {
        fn occurred_on<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.occurred_on;
    }
        fn event_kind<'a>(&'a self) -> &'a str {
        return &self.event_kind[..];
    }
        fn status<'a>(&'a self) -> Option<&'a crate::ListingStatus> {
        return self.status.as_ref();
    }
        fn asking_price<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.asking_price.as_ref();
    }
        fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod> {
        return self.rent_period.as_ref();
    }
        fn close_price<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.close_price.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait ListingParticipant : TransactionParty   {


}

impl ListingParticipant for crate::ListingParticipant {
}


pub trait LeaseEvent : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn property_state<'a>(&'a self) -> Option<&'a str>;
    // fn property_state_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_state<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn space<'a>(&'a self) -> Option<&'a str>;
    // fn space_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_space<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn lease_type<'a>(&'a self) -> Option<&'a crate::LeaseTypeEnum>;
    // fn lease_type_mut(&mut self) -> &mut Option<&'a crate::LeaseTypeEnum>;
    // fn set_lease_type(&mut self, value: Option<&'a LeaseTypeEnum>);

    fn execution_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn execution_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_execution_date(&mut self, value: Option<&'a NaiveDate>);

    fn commencement_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn commencement_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_commencement_date(&mut self, value: Option<&'a NaiveDate>);

    fn expiration_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn expiration_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_expiration_date(&mut self, value: Option<&'a NaiveDate>);

    fn term_months(&self) -> Option<isize>;
    // fn term_months_mut(&mut self) -> &mut Option<isize>;
    // fn set_term_months(&mut self, value: Option<isize>);

    fn leased_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn leased_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_leased_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn rent<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn rent_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_rent<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod>;
    // fn rent_period_mut(&mut self) -> &mut Option<&'a crate::RentPeriod>;
    // fn set_rent_period(&mut self, value: Option<&'a RentPeriod>);

    fn starting_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn starting_rent_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_starting_rent_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn effective_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn effective_rent_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_effective_rent_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn net_effective_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn net_effective_rent_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_net_effective_rent_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn free_rent_months(&self) -> Option<f64>;
    // fn free_rent_months_mut(&mut self) -> &mut Option<f64>;
    // fn set_free_rent_months(&mut self, value: Option<f64>);

    fn ti_allowance_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn ti_allowance_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_ti_allowance_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn expense_structure<'a>(&'a self) -> Option<&'a crate::ExpenseStructure>;
    // fn expense_structure_mut(&mut self) -> &mut Option<&'a crate::ExpenseStructure>;
    // fn set_expense_structure<E>(&mut self, value: Option<E>) where E: Into<ExpenseStructure>;

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEventParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LeaseEventParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LeaseEventParty>;

    fn escalations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEscalation>>;
    // fn escalations_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LeaseEscalation>>;
    // fn set_escalations<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LeaseEscalation>;

    fn concessions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseConcession>>;
    // fn concessions_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LeaseConcession>>;
    // fn set_concessions<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LeaseConcession>;

    fn remarks<'a>(&'a self) -> Option<&'a str>;
    // fn remarks_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_remarks(&mut self, value: Option<&'a str>);


}

impl LeaseEvent for crate::LeaseEvent {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn property_state<'a>(&'a self) -> Option<&'a str> {
        return self.property_state.as_deref();
    }
        fn space<'a>(&'a self) -> Option<&'a str> {
        return self.space.as_deref();
    }
        fn lease_type<'a>(&'a self) -> Option<&'a crate::LeaseTypeEnum> {
        return self.lease_type.as_ref();
    }
        fn execution_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.execution_date.as_ref();
    }
        fn commencement_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.commencement_date.as_ref();
    }
        fn expiration_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.expiration_date.as_ref();
    }
        fn term_months(&self) -> Option<isize> {
        return self.term_months;
    }
        fn leased_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.leased_area.as_ref();
    }
        fn rent<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.rent.as_ref();
    }
        fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod> {
        return self.rent_period.as_ref();
    }
        fn starting_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.starting_rent_per_area.as_ref();
    }
        fn effective_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.effective_rent_per_area.as_ref();
    }
        fn net_effective_rent_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.net_effective_rent_per_area.as_ref();
    }
        fn free_rent_months(&self) -> Option<f64> {
        return self.free_rent_months;
    }
        fn ti_allowance_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.ti_allowance_per_area.as_ref();
    }
        fn expense_structure<'a>(&'a self) -> Option<&'a crate::ExpenseStructure> {
        return self.expense_structure.as_ref();
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEventParty>> {
        return self.parties.as_ref();
    }
        fn escalations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEscalation>> {
        return self.escalations.as_ref();
    }
        fn concessions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseConcession>> {
        return self.concessions.as_ref();
    }
        fn remarks<'a>(&'a self) -> Option<&'a str> {
        return self.remarks.as_deref();
    }
}


pub trait LeaseEventParty : TransactionParty   {


}

impl LeaseEventParty for crate::LeaseEventParty {
}


pub trait ExpenseStructure   {

    fn taxes_paid_by<'a>(&'a self) -> Option<&'a str>;
    // fn taxes_paid_by_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_taxes_paid_by(&mut self, value: Option<&'a str>);

    fn insurance_paid_by<'a>(&'a self) -> Option<&'a str>;
    // fn insurance_paid_by_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_insurance_paid_by(&mut self, value: Option<&'a str>);

    fn cam_paid_by<'a>(&'a self) -> Option<&'a str>;
    // fn cam_paid_by_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_cam_paid_by(&mut self, value: Option<&'a str>);

    fn utilities_paid_by<'a>(&'a self) -> Option<&'a str>;
    // fn utilities_paid_by_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_utilities_paid_by(&mut self, value: Option<&'a str>);

    fn notes<'a>(&'a self) -> Option<&'a str>;
    // fn notes_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_notes(&mut self, value: Option<&'a str>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl ExpenseStructure for crate::ExpenseStructure {
        fn taxes_paid_by<'a>(&'a self) -> Option<&'a str> {
        return self.taxes_paid_by.as_deref();
    }
        fn insurance_paid_by<'a>(&'a self) -> Option<&'a str> {
        return self.insurance_paid_by.as_deref();
    }
        fn cam_paid_by<'a>(&'a self) -> Option<&'a str> {
        return self.cam_paid_by.as_deref();
    }
        fn utilities_paid_by<'a>(&'a self) -> Option<&'a str> {
        return self.utilities_paid_by.as_deref();
    }
        fn notes<'a>(&'a self) -> Option<&'a str> {
        return self.notes.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait LeaseEscalation   {

    fn escalation_type<'a>(&'a self) -> &'a str;
    // fn escalation_type_mut(&mut self) -> &mut &'a str;
    // fn set_escalation_type(&mut self, value: String);

    fn escalation_value(&self) -> Option<f64>;
    // fn escalation_value_mut(&mut self) -> &mut Option<f64>;
    // fn set_escalation_value(&mut self, value: Option<f64>);

    fn frequency_months(&self) -> Option<isize>;
    // fn frequency_months_mut(&mut self) -> &mut Option<isize>;
    // fn set_frequency_months(&mut self, value: Option<isize>);

    fn cpi_index<'a>(&'a self) -> Option<&'a str>;
    // fn cpi_index_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_cpi_index(&mut self, value: Option<&'a str>);

    fn cpi_floor(&self) -> Option<f64>;
    // fn cpi_floor_mut(&mut self) -> &mut Option<f64>;
    // fn set_cpi_floor(&mut self, value: Option<f64>);

    fn cpi_cap(&self) -> Option<f64>;
    // fn cpi_cap_mut(&mut self) -> &mut Option<f64>;
    // fn set_cpi_cap(&mut self, value: Option<f64>);

    fn steps<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentStep>>;
    // fn steps_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::RentStep>>;
    // fn set_steps<E>(&mut self, value: Option<&Vec<E>>) where E: Into<RentStep>;

    fn effective_from<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn effective_from_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_effective_from(&mut self, value: Option<&'a NaiveDate>);

    fn effective_until<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn effective_until_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_effective_until(&mut self, value: Option<&'a NaiveDate>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl LeaseEscalation for crate::LeaseEscalation {
        fn escalation_type<'a>(&'a self) -> &'a str {
        return &self.escalation_type[..];
    }
        fn escalation_value(&self) -> Option<f64> {
        return self.escalation_value;
    }
        fn frequency_months(&self) -> Option<isize> {
        return self.frequency_months;
    }
        fn cpi_index<'a>(&'a self) -> Option<&'a str> {
        return self.cpi_index.as_deref();
    }
        fn cpi_floor(&self) -> Option<f64> {
        return self.cpi_floor;
    }
        fn cpi_cap(&self) -> Option<f64> {
        return self.cpi_cap;
    }
        fn steps<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentStep>> {
        return self.steps.as_ref();
    }
        fn effective_from<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.effective_from.as_ref();
    }
        fn effective_until<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.effective_until.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait RentStep   {

    fn from_date<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn from_date_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_from_date(&mut self, value: NaiveDate);

    fn amount<'a>(&'a self) -> &'a crate::Money;
    // fn amount_mut(&mut self) -> &mut &'a crate::Money;
    // fn set_amount<E>(&mut self, value: E) where E: Into<Money>;


}

impl RentStep for crate::RentStep {
        fn from_date<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.from_date;
    }
        fn amount<'a>(&'a self) -> &'a crate::Money {
        return &self.amount;
    }
}


pub trait LeaseConcession   {

    fn concession_type<'a>(&'a self) -> &'a str;
    // fn concession_type_mut(&mut self) -> &mut &'a str;
    // fn set_concession_type(&mut self, value: String);

    fn concession_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn concession_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_concession_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn abatement_months(&self) -> Option<f64>;
    // fn abatement_months_mut(&mut self) -> &mut Option<f64>;
    // fn set_abatement_months(&mut self, value: Option<f64>);

    fn abatement_percent(&self) -> Option<f64>;
    // fn abatement_percent_mut(&mut self) -> &mut Option<f64>;
    // fn set_abatement_percent(&mut self, value: Option<f64>);

    fn ti_cap_total<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn ti_cap_total_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_ti_cap_total<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn conditions<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn conditions_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_conditions<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn notes<'a>(&'a self) -> Option<&'a str>;
    // fn notes_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_notes(&mut self, value: Option<&'a str>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl LeaseConcession for crate::LeaseConcession {
        fn concession_type<'a>(&'a self) -> &'a str {
        return &self.concession_type[..];
    }
        fn concession_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.concession_value.as_ref();
    }
        fn abatement_months(&self) -> Option<f64> {
        return self.abatement_months;
    }
        fn abatement_percent(&self) -> Option<f64> {
        return self.abatement_percent;
    }
        fn ti_cap_total<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.ti_cap_total.as_ref();
    }
        fn conditions<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.conditions.as_ref();
    }
        fn notes<'a>(&'a self) -> Option<&'a str> {
        return self.notes.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait UnitRentObservation : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn unit_type<'a>(&'a self) -> &'a str;
    // fn unit_type_mut(&mut self) -> &mut &'a str;
    // fn set_unit_type(&mut self, value: String);

    fn unit_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn unit_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_unit_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn bedrooms(&self) -> Option<isize>;
    // fn bedrooms_mut(&mut self) -> &mut Option<isize>;
    // fn set_bedrooms(&mut self, value: Option<isize>);

    fn bathrooms(&self) -> Option<f64>;
    // fn bathrooms_mut(&mut self) -> &mut Option<f64>;
    // fn set_bathrooms(&mut self, value: Option<f64>);

    fn unit_count(&self) -> Option<isize>;
    // fn unit_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_unit_count(&mut self, value: Option<isize>);

    fn units_available(&self) -> Option<isize>;
    // fn units_available_mut(&mut self) -> &mut Option<isize>;
    // fn set_units_available(&mut self, value: Option<isize>);

    fn rate<'a>(&'a self) -> &'a crate::Money;
    // fn rate_mut(&mut self) -> &mut &'a crate::Money;
    // fn set_rate<E>(&mut self, value: E) where E: Into<Money>;

    fn rate_period<'a>(&'a self) -> Option<&'a crate::RentPeriod>;
    // fn rate_period_mut(&mut self) -> &mut Option<&'a crate::RentPeriod>;
    // fn set_rate_period(&mut self, value: Option<&'a RentPeriod>);

    fn rate_basis<'a>(&'a self) -> Option<&'a crate::RateBasis>;
    // fn rate_basis_mut(&mut self) -> &mut Option<&'a crate::RateBasis>;
    // fn set_rate_basis(&mut self, value: Option<&'a RateBasis>);

    fn rate_type<'a>(&'a self) -> Option<&'a crate::RateType>;
    // fn rate_type_mut(&mut self) -> &mut Option<&'a crate::RateType>;
    // fn set_rate_type(&mut self, value: Option<&'a RateType>);

    fn observed_on<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn observed_on_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_observed_on(&mut self, value: NaiveDate);

    fn concessions_note<'a>(&'a self) -> Option<&'a str>;
    // fn concessions_note_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_concessions_note(&mut self, value: Option<&'a str>);


}

impl UnitRentObservation for crate::UnitRentObservation {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn unit_type<'a>(&'a self) -> &'a str {
        return &self.unit_type[..];
    }
        fn unit_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.unit_area.as_ref();
    }
        fn bedrooms(&self) -> Option<isize> {
        return self.bedrooms;
    }
        fn bathrooms(&self) -> Option<f64> {
        return self.bathrooms;
    }
        fn unit_count(&self) -> Option<isize> {
        return self.unit_count;
    }
        fn units_available(&self) -> Option<isize> {
        return self.units_available;
    }
        fn rate<'a>(&'a self) -> &'a crate::Money {
        return &self.rate;
    }
        fn rate_period<'a>(&'a self) -> Option<&'a crate::RentPeriod> {
        return self.rate_period.as_ref();
    }
        fn rate_basis<'a>(&'a self) -> Option<&'a crate::RateBasis> {
        return self.rate_basis.as_ref();
    }
        fn rate_type<'a>(&'a self) -> Option<&'a crate::RateType> {
        return self.rate_type.as_ref();
    }
        fn observed_on<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.observed_on;
    }
        fn concessions_note<'a>(&'a self) -> Option<&'a str> {
        return self.concessions_note.as_deref();
    }
}


pub trait Loan : Entity  +  RecordedInstrument   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn parcel<'a>(&'a self) -> Option<&'a str>;
    // fn parcel_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_parcel<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn transfer<'a>(&'a self) -> Option<&'a str>;
    // fn transfer_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_transfer<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn is_purchase_money(&self) -> Option<bool>;
    // fn is_purchase_money_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_purchase_money(&mut self, value: Option<bool>);

    fn loan_amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn loan_amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_loan_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn loan_type<'a>(&'a self) -> Option<&'a str>;
    // fn loan_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_loan_type(&mut self, value: Option<&'a str>);

    fn purpose<'a>(&'a self) -> Option<&'a str>;
    // fn purpose_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_purpose(&mut self, value: Option<&'a str>);

    fn is_heloc(&self) -> Option<bool>;
    // fn is_heloc_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_heloc(&mut self, value: Option<bool>);

    fn is_construction(&self) -> Option<bool>;
    // fn is_construction_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_construction(&mut self, value: Option<bool>);

    fn is_seller_carryback(&self) -> Option<bool>;
    // fn is_seller_carryback_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_seller_carryback(&mut self, value: Option<bool>);

    fn is_assumable(&self) -> Option<bool>;
    // fn is_assumable_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_assumable(&mut self, value: Option<bool>);

    fn interest_rate(&self) -> Option<f64>;
    // fn interest_rate_mut(&mut self) -> &mut Option<f64>;
    // fn set_interest_rate(&mut self, value: Option<f64>);

    fn is_variable_rate(&self) -> Option<bool>;
    // fn is_variable_rate_mut(&mut self) -> &mut Option<bool>;
    // fn set_is_variable_rate(&mut self, value: Option<bool>);

    fn term_months(&self) -> Option<isize>;
    // fn term_months_mut(&mut self) -> &mut Option<isize>;
    // fn set_term_months(&mut self, value: Option<isize>);

    fn due_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn due_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_due_date(&mut self, value: Option<&'a NaiveDate>);

    fn lien_position(&self) -> Option<isize>;
    // fn lien_position_mut(&mut self) -> &mut Option<isize>;
    // fn set_lien_position(&mut self, value: Option<isize>);

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LoanParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LoanParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LoanParty>;

    fn events<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LoanEvent>>;
    // fn events_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LoanEvent>>;
    // fn set_events<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LoanEvent>;


}

impl Loan for crate::Loan {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn parcel<'a>(&'a self) -> Option<&'a str> {
        return self.parcel.as_deref();
    }
        fn transfer<'a>(&'a self) -> Option<&'a str> {
        return self.transfer.as_deref();
    }
        fn is_purchase_money(&self) -> Option<bool> {
        return self.is_purchase_money;
    }
        fn loan_amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.loan_amount.as_ref();
    }
        fn loan_type<'a>(&'a self) -> Option<&'a str> {
        return self.loan_type.as_deref();
    }
        fn purpose<'a>(&'a self) -> Option<&'a str> {
        return self.purpose.as_deref();
    }
        fn is_heloc(&self) -> Option<bool> {
        return self.is_heloc;
    }
        fn is_construction(&self) -> Option<bool> {
        return self.is_construction;
    }
        fn is_seller_carryback(&self) -> Option<bool> {
        return self.is_seller_carryback;
    }
        fn is_assumable(&self) -> Option<bool> {
        return self.is_assumable;
    }
        fn interest_rate(&self) -> Option<f64> {
        return self.interest_rate;
    }
        fn is_variable_rate(&self) -> Option<bool> {
        return self.is_variable_rate;
    }
        fn term_months(&self) -> Option<isize> {
        return self.term_months;
    }
        fn due_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.due_date.as_ref();
    }
        fn lien_position(&self) -> Option<isize> {
        return self.lien_position;
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LoanParty>> {
        return self.parties.as_ref();
    }
        fn events<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LoanEvent>> {
        return self.events.as_ref();
    }
}


pub trait LoanParty : TransactionParty   {


}

impl LoanParty for crate::LoanParty {
}


pub trait LoanEvent : RecordedInstrument   {

    fn event_kind<'a>(&'a self) -> &'a crate::LoanEventKind;
    // fn event_kind_mut(&mut self) -> &mut &'a crate::LoanEventKind;
    // fn set_event_kind(&mut self, value: LoanEventKind);

    fn occurred_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn occurred_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_occurred_on(&mut self, value: Option<&'a NaiveDate>);

    fn amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn to_party<'a>(&'a self) -> Option<&'a str>;
    // fn to_party_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_to_party<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl LoanEvent for crate::LoanEvent {
        fn event_kind<'a>(&'a self) -> &'a crate::LoanEventKind {
        return &self.event_kind;
    }
        fn occurred_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.occurred_on.as_ref();
    }
        fn amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount.as_ref();
    }
        fn to_party<'a>(&'a self) -> Option<&'a str> {
        return self.to_party.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait Lien : Entity  +  RecordedInstrument   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn kind<'a>(&'a self) -> &'a crate::LienKind;
    // fn kind_mut(&mut self) -> &mut &'a crate::LienKind;
    // fn set_kind(&mut self, value: LienKind);

    fn amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn released_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn released_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_released_on(&mut self, value: Option<&'a NaiveDate>);

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LienParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LienParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LienParty>;


}

impl Lien for crate::Lien {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn kind<'a>(&'a self) -> &'a crate::LienKind {
        return &self.kind;
    }
        fn amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.amount.as_ref();
    }
        fn released_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.released_on.as_ref();
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LienParty>> {
        return self.parties.as_ref();
    }
}


pub trait LienParty : TransactionParty   {


}

impl LienParty for crate::LienParty {
}


pub trait ForeclosureCase : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn loan<'a>(&'a self) -> Option<&'a str>;
    // fn loan_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_loan<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn case_number<'a>(&'a self) -> Option<&'a str>;
    // fn case_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_case_number(&mut self, value: Option<&'a str>);

    fn opened_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn opened_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_opened_on(&mut self, value: Option<&'a NaiveDate>);

    fn resolved_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn resolved_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_resolved_on(&mut self, value: Option<&'a NaiveDate>);

    fn resolution<'a>(&'a self) -> Option<&'a str>;
    // fn resolution_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_resolution(&mut self, value: Option<&'a str>);

    fn past_due_amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn past_due_amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_past_due_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn unpaid_balance<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn unpaid_balance_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_unpaid_balance<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn original_loan_amount<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn original_loan_amount_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_original_loan_amount<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn auction_min_bid<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn auction_min_bid_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_auction_min_bid<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn auction_location<'a>(&'a self) -> Option<&'a str>;
    // fn auction_location_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_auction_location(&mut self, value: Option<&'a str>);

    fn filings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureFiling>>;
    // fn filings_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ForeclosureFiling>>;
    // fn set_filings<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ForeclosureFiling>;

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCaseParty>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCaseParty>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ForeclosureCaseParty>;


}

impl ForeclosureCase for crate::ForeclosureCase {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn loan<'a>(&'a self) -> Option<&'a str> {
        return self.loan.as_deref();
    }
        fn case_number<'a>(&'a self) -> Option<&'a str> {
        return self.case_number.as_deref();
    }
        fn opened_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.opened_on.as_ref();
    }
        fn resolved_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.resolved_on.as_ref();
    }
        fn resolution<'a>(&'a self) -> Option<&'a str> {
        return self.resolution.as_deref();
    }
        fn past_due_amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.past_due_amount.as_ref();
    }
        fn unpaid_balance<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.unpaid_balance.as_ref();
    }
        fn original_loan_amount<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.original_loan_amount.as_ref();
    }
        fn auction_min_bid<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.auction_min_bid.as_ref();
    }
        fn auction_location<'a>(&'a self) -> Option<&'a str> {
        return self.auction_location.as_deref();
    }
        fn filings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureFiling>> {
        return self.filings.as_ref();
    }
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCaseParty>> {
        return self.parties.as_ref();
    }
}


pub trait ForeclosureFiling : RecordedInstrument   {

    fn status<'a>(&'a self) -> &'a str;
    // fn status_mut(&mut self) -> &mut &'a str;
    // fn set_status(&mut self, value: String);

    fn auction_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn auction_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_auction_on(&mut self, value: Option<&'a NaiveDate>);

    fn auction_at_time<'a>(&'a self) -> Option<&'a str>;
    // fn auction_at_time_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_auction_at_time(&mut self, value: Option<&'a str>);

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;


}

impl ForeclosureFiling for crate::ForeclosureFiling {
        fn status<'a>(&'a self) -> &'a str {
        return &self.status[..];
    }
        fn auction_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.auction_on.as_ref();
    }
        fn auction_at_time<'a>(&'a self) -> Option<&'a str> {
        return self.auction_at_time.as_deref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
}


pub trait ForeclosureCaseParty : TransactionParty   {


}

impl ForeclosureCaseParty for crate::ForeclosureCaseParty {
}


pub trait Permit : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn structure<'a>(&'a self) -> Option<&'a str>;
    // fn structure_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_structure<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn permitting_jurisdiction<'a>(&'a self) -> Option<&'a str>;
    // fn permitting_jurisdiction_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_permitting_jurisdiction<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn permit_number<'a>(&'a self) -> Option<&'a str>;
    // fn permit_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_permit_number(&mut self, value: Option<&'a str>);

    fn kind<'a>(&'a self) -> Option<&'a str>;
    // fn kind_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_kind(&mut self, value: Option<&'a str>);

    fn status<'a>(&'a self) -> Option<&'a str>;
    // fn status_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_status(&mut self, value: Option<&'a str>);

    fn description<'a>(&'a self) -> Option<&'a str>;
    // fn description_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_description(&mut self, value: Option<&'a str>);

    fn applied_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn applied_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_applied_on(&mut self, value: Option<&'a NaiveDate>);

    fn issued_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn issued_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_issued_on(&mut self, value: Option<&'a NaiveDate>);

    fn finaled_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn finaled_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_finaled_on(&mut self, value: Option<&'a NaiveDate>);

    fn expires_on<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn expires_on_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_expires_on(&mut self, value: Option<&'a NaiveDate>);

    fn job_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn job_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_job_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn fees<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn fees_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_fees<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn contractor_party<'a>(&'a self) -> Option<&'a str>;
    // fn contractor_party_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_contractor_party<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifacts<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;


}

impl Permit for crate::Permit {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn structure<'a>(&'a self) -> Option<&'a str> {
        return self.structure.as_deref();
    }
        fn permitting_jurisdiction<'a>(&'a self) -> Option<&'a str> {
        return self.permitting_jurisdiction.as_deref();
    }
        fn permit_number<'a>(&'a self) -> Option<&'a str> {
        return self.permit_number.as_deref();
    }
        fn kind<'a>(&'a self) -> Option<&'a str> {
        return self.kind.as_deref();
    }
        fn status<'a>(&'a self) -> Option<&'a str> {
        return self.status.as_deref();
    }
        fn description<'a>(&'a self) -> Option<&'a str> {
        return self.description.as_deref();
    }
        fn applied_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.applied_on.as_ref();
    }
        fn issued_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.issued_on.as_ref();
    }
        fn finaled_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.finaled_on.as_ref();
    }
        fn expires_on<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.expires_on.as_ref();
    }
        fn job_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.job_value.as_ref();
    }
        fn fees<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.fees.as_ref();
    }
        fn contractor_party<'a>(&'a self) -> Option<&'a str> {
        return self.contractor_party.as_deref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}


pub trait OperatingStatement : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn statement_year(&self) -> isize;
    // fn statement_year_mut(&mut self) -> &mut isize;
    // fn set_statement_year(&mut self, value: isize);

    fn period_start<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn period_start_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_period_start(&mut self, value: Option<&'a NaiveDate>);

    fn period_end<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn period_end_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_period_end(&mut self, value: Option<&'a NaiveDate>);

    fn statement_basis<'a>(&'a self) -> Option<&'a crate::StatementBasis>;
    // fn statement_basis_mut(&mut self) -> &mut Option<&'a crate::StatementBasis>;
    // fn set_statement_basis(&mut self, value: Option<&'a StatementBasis>);

    fn pgi<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn pgi_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_pgi<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn vacancy_loss<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn vacancy_loss_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_vacancy_loss<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn vacancy_pct(&self) -> Option<f64>;
    // fn vacancy_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_vacancy_pct(&mut self, value: Option<f64>);

    fn egi<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn egi_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_egi<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn opex_total<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn opex_total_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_opex_total<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn noi<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn noi_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_noi<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn capex<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn capex_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_capex<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn reimbursements<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn reimbursements_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_reimbursements<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn reserves<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn reserves_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_reserves<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn reserves_included_in_opex(&self) -> Option<bool>;
    // fn reserves_included_in_opex_mut(&mut self) -> &mut Option<bool>;
    // fn set_reserves_included_in_opex(&mut self, value: Option<bool>);

    fn ground_lease_expense<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn ground_lease_expense_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_ground_lease_expense<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn ground_lease_included_in_opex(&self) -> Option<bool>;
    // fn ground_lease_included_in_opex_mut(&mut self) -> &mut Option<bool>;
    // fn set_ground_lease_included_in_opex(&mut self, value: Option<bool>);

    fn line_items<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StatementLineItem>>;
    // fn line_items_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::StatementLineItem>>;
    // fn set_line_items<E>(&mut self, value: Option<&Vec<E>>) where E: Into<StatementLineItem>;


}

impl OperatingStatement for crate::OperatingStatement {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn statement_year(&self) -> isize {
        return self.statement_year;
    }
        fn period_start<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.period_start.as_ref();
    }
        fn period_end<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.period_end.as_ref();
    }
        fn statement_basis<'a>(&'a self) -> Option<&'a crate::StatementBasis> {
        return self.statement_basis.as_ref();
    }
        fn pgi<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.pgi.as_ref();
    }
        fn vacancy_loss<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.vacancy_loss.as_ref();
    }
        fn vacancy_pct(&self) -> Option<f64> {
        return self.vacancy_pct;
    }
        fn egi<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.egi.as_ref();
    }
        fn opex_total<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.opex_total.as_ref();
    }
        fn noi<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.noi.as_ref();
    }
        fn capex<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.capex.as_ref();
    }
        fn reimbursements<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.reimbursements.as_ref();
    }
        fn reserves<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.reserves.as_ref();
    }
        fn reserves_included_in_opex(&self) -> Option<bool> {
        return self.reserves_included_in_opex;
    }
        fn ground_lease_expense<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.ground_lease_expense.as_ref();
    }
        fn ground_lease_included_in_opex(&self) -> Option<bool> {
        return self.ground_lease_included_in_opex;
    }
        fn line_items<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::StatementLineItem>> {
        return self.line_items.as_ref();
    }
}


pub trait StatementLineItem   {

    fn category<'a>(&'a self) -> &'a str;
    // fn category_mut(&mut self) -> &mut &'a str;
    // fn set_category(&mut self, value: String);

    fn label<'a>(&'a self) -> &'a str;
    // fn label_mut(&mut self) -> &mut &'a str;
    // fn set_label(&mut self, value: String);

    fn amount<'a>(&'a self) -> &'a crate::Money;
    // fn amount_mut(&mut self) -> &mut &'a crate::Money;
    // fn set_amount<E>(&mut self, value: E) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl StatementLineItem for crate::StatementLineItem {
        fn category<'a>(&'a self) -> &'a str {
        return &self.category[..];
    }
        fn label<'a>(&'a self) -> &'a str {
        return &self.label[..];
    }
        fn amount<'a>(&'a self) -> &'a crate::Money {
        return &self.amount;
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait RentRoll : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn as_of_date_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_as_of_date(&mut self, value: NaiveDate);

    fn unit_count(&self) -> Option<isize>;
    // fn unit_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_unit_count(&mut self, value: Option<isize>);

    fn occupied_unit_count(&self) -> Option<isize>;
    // fn occupied_unit_count_mut(&mut self) -> &mut Option<isize>;
    // fn set_occupied_unit_count(&mut self, value: Option<isize>);

    fn occupancy_pct(&self) -> Option<f64>;
    // fn occupancy_pct_mut(&mut self) -> &mut Option<f64>;
    // fn set_occupancy_pct(&mut self, value: Option<f64>);

    fn total_contract_rent<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn total_contract_rent_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_total_contract_rent<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn total_market_rent<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn total_market_rent_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_total_market_rent<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod>;
    // fn rent_period_mut(&mut self) -> &mut Option<&'a crate::RentPeriod>;
    // fn set_rent_period(&mut self, value: Option<&'a RentPeriod>);

    fn lines<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentRollLine>>;
    // fn lines_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::RentRollLine>>;
    // fn set_lines<E>(&mut self, value: Option<&Vec<E>>) where E: Into<RentRollLine>;


}

impl RentRoll for crate::RentRoll {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.as_of_date;
    }
        fn unit_count(&self) -> Option<isize> {
        return self.unit_count;
    }
        fn occupied_unit_count(&self) -> Option<isize> {
        return self.occupied_unit_count;
    }
        fn occupancy_pct(&self) -> Option<f64> {
        return self.occupancy_pct;
    }
        fn total_contract_rent<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.total_contract_rent.as_ref();
    }
        fn total_market_rent<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.total_market_rent.as_ref();
    }
        fn rent_period<'a>(&'a self) -> Option<&'a crate::RentPeriod> {
        return self.rent_period.as_ref();
    }
        fn lines<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentRollLine>> {
        return self.lines.as_ref();
    }
}


pub trait RentRollLine   {

    fn space<'a>(&'a self) -> Option<&'a str>;
    // fn space_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_space<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn tenant<'a>(&'a self) -> Option<&'a str>;
    // fn tenant_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_tenant<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn lease<'a>(&'a self) -> Option<&'a str>;
    // fn lease_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_lease<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn occupancy_status<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype>;
    // fn occupancy_status_mut(&mut self) -> &mut Option<&'a CodeableConceptOrSubtype>;
    // fn set_occupancy_status<E>(&mut self, value: Option<E>) where E: Into<CodeableConcept>;

    fn reported_area<'a>(&'a self) -> Option<&'a crate::Area>;
    // fn reported_area_mut(&mut self) -> &mut Option<&'a crate::Area>;
    // fn set_reported_area<E>(&mut self, value: Option<E>) where E: Into<Area>;

    fn contract_rent<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn contract_rent_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_contract_rent<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn market_rent<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn market_rent_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_market_rent<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl RentRollLine for crate::RentRollLine {
        fn space<'a>(&'a self) -> Option<&'a str> {
        return self.space.as_deref();
    }
        fn tenant<'a>(&'a self) -> Option<&'a str> {
        return self.tenant.as_deref();
    }
        fn lease<'a>(&'a self) -> Option<&'a str> {
        return self.lease.as_deref();
    }
        fn occupancy_status<'a>(&'a self) -> Option<&'a CodeableConceptOrSubtype> {
        return self.occupancy_status.as_ref();
    }
        fn reported_area<'a>(&'a self) -> Option<&'a crate::Area> {
        return self.reported_area.as_ref();
    }
        fn contract_rent<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.contract_rent.as_ref();
    }
        fn market_rent<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.market_rent.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait Valuation : Entity   {

    fn property<'a>(&'a self) -> &'a str;
    // fn property_mut(&mut self) -> &mut &'a str;
    // fn set_property<E>(&mut self, value: String) where E: Into<String>;

    fn property_state<'a>(&'a self) -> Option<&'a str>;
    // fn property_state_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_property_state<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn kind<'a>(&'a self) -> &'a crate::ValuationKind;
    // fn kind_mut(&mut self) -> &mut &'a crate::ValuationKind;
    // fn set_kind(&mut self, value: ValuationKind);

    fn valuation_method<'a>(&'a self) -> Option<&'a str>;
    // fn valuation_method_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_valuation_method(&mut self, value: Option<&'a str>);

    fn value_type<'a>(&'a self) -> Option<&'a str>;
    // fn value_type_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_value_type(&mut self, value: Option<&'a str>);

    fn value<'a>(&'a self) -> &'a crate::Money;
    // fn value_mut(&mut self) -> &mut &'a crate::Money;
    // fn set_value<E>(&mut self, value: E) where E: Into<Money>;

    fn value_low<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn value_low_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_value_low<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn value_high<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn value_high_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_value_high<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn value_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate>;
    // fn value_per_area_mut(&mut self) -> &mut Option<&'a crate::UnitRate>;
    // fn set_value_per_area<E>(&mut self, value: Option<E>) where E: Into<UnitRate>;

    fn land_value<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn land_value_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_land_value<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn confidence_score(&self) -> Option<isize>;
    // fn confidence_score_mut(&mut self) -> &mut Option<isize>;
    // fn set_confidence_score(&mut self, value: Option<isize>);

    fn forecast_standard_deviation(&self) -> Option<f64>;
    // fn forecast_standard_deviation_mut(&mut self) -> &mut Option<f64>;
    // fn set_forecast_standard_deviation(&mut self, value: Option<f64>);

    fn exposure_days(&self) -> Option<isize>;
    // fn exposure_days_mut(&mut self) -> &mut Option<isize>;
    // fn set_exposure_days(&mut self, value: Option<isize>);

    fn indicated_value_sales_comparison<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn indicated_value_sales_comparison_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_indicated_value_sales_comparison<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn indicated_value_cost<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn indicated_value_cost_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_indicated_value_cost<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn indicated_value_income<'a>(&'a self) -> Option<&'a crate::Money>;
    // fn indicated_value_income_mut(&mut self) -> &mut Option<&'a crate::Money>;
    // fn set_indicated_value_income<E>(&mut self, value: Option<E>) where E: Into<Money>;

    fn value_premise<'a>(&'a self) -> Option<&'a str>;
    // fn value_premise_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_value_premise(&mut self, value: Option<&'a str>);

    fn interest<'a>(&'a self) -> Option<&'a str>;
    // fn interest_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_interest(&mut self, value: Option<&'a str>);

    fn performed_by_party<'a>(&'a self) -> Option<&'a str>;
    // fn performed_by_party_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_performed_by_party<E>(&mut self, value: Option<&'a str>) where E: Into<String>;

    fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate;
    // fn as_of_date_mut(&mut self) -> &mut &'a crate::NaiveDate;
    // fn set_as_of_date(&mut self, value: NaiveDate);

    fn report_date<'a>(&'a self) -> Option<&'a crate::NaiveDate>;
    // fn report_date_mut(&mut self) -> &mut Option<&'a crate::NaiveDate>;
    // fn set_report_date(&mut self, value: Option<&'a NaiveDate>);

    fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifacts<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;


}

impl Valuation for crate::Valuation {
        fn property<'a>(&'a self) -> &'a str {
        return &self.property[..];
    }
        fn property_state<'a>(&'a self) -> Option<&'a str> {
        return self.property_state.as_deref();
    }
        fn kind<'a>(&'a self) -> &'a crate::ValuationKind {
        return &self.kind;
    }
        fn valuation_method<'a>(&'a self) -> Option<&'a str> {
        return self.valuation_method.as_deref();
    }
        fn value_type<'a>(&'a self) -> Option<&'a str> {
        return self.value_type.as_deref();
    }
        fn value<'a>(&'a self) -> &'a crate::Money {
        return &self.value;
    }
        fn value_low<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.value_low.as_ref();
    }
        fn value_high<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.value_high.as_ref();
    }
        fn value_per_area<'a>(&'a self) -> Option<&'a crate::UnitRate> {
        return self.value_per_area.as_ref();
    }
        fn land_value<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.land_value.as_ref();
    }
        fn confidence_score(&self) -> Option<isize> {
        return self.confidence_score;
    }
        fn forecast_standard_deviation(&self) -> Option<f64> {
        return self.forecast_standard_deviation;
    }
        fn exposure_days(&self) -> Option<isize> {
        return self.exposure_days;
    }
        fn indicated_value_sales_comparison<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.indicated_value_sales_comparison.as_ref();
    }
        fn indicated_value_cost<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.indicated_value_cost.as_ref();
    }
        fn indicated_value_income<'a>(&'a self) -> Option<&'a crate::Money> {
        return self.indicated_value_income.as_ref();
    }
        fn value_premise<'a>(&'a self) -> Option<&'a str> {
        return self.value_premise.as_deref();
    }
        fn interest<'a>(&'a self) -> Option<&'a str> {
        return self.interest.as_deref();
    }
        fn performed_by_party<'a>(&'a self) -> Option<&'a str> {
        return self.performed_by_party.as_deref();
    }
        fn as_of_date<'a>(&'a self) -> &'a crate::NaiveDate {
        return &self.as_of_date;
    }
        fn report_date<'a>(&'a self) -> Option<&'a crate::NaiveDate> {
        return self.report_date.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifacts.as_ref();
    }
}


pub trait PropertyProfile   {

    fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Party>>;
    // fn parties_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Party>>;
    // fn set_parties<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Party>;

    fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SourceArtifact>>;
    // fn artifacts_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::SourceArtifact>>;
    // fn set_artifacts<E>(&mut self, value: Option<&Vec<E>>) where E: Into<SourceArtifact>;

    fn addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Address>>;
    // fn addresses_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Address>>;
    // fn set_addresses<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Address>;

    fn property<'a>(&'a self) -> &'a crate::Property;
    // fn property_mut(&mut self) -> &mut &'a crate::Property;
    // fn set_property<E>(&mut self, value: E) where E: Into<Property>;

    fn property_addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyAddress>>;
    // fn property_addresses_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PropertyAddress>>;
    // fn set_property_addresses<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PropertyAddress>;

    fn identifiers<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyIdentifier>>;
    // fn identifiers_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PropertyIdentifier>>;
    // fn set_identifiers<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PropertyIdentifier>;

    fn jurisdictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Jurisdiction>>;
    // fn jurisdictions_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Jurisdiction>>;
    // fn set_jurisdictions<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Jurisdiction>;

    fn parcels<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Parcel>>;
    // fn parcels_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Parcel>>;
    // fn set_parcels<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Parcel>;

    fn property_parcels<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyParcel>>;
    // fn property_parcels_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PropertyParcel>>;
    // fn set_property_parcels<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PropertyParcel>;

    fn parcel_lineage<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ParcelLineage>>;
    // fn parcel_lineage_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ParcelLineage>>;
    // fn set_parcel_lineage<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ParcelLineage>;

    fn site<'a>(&'a self) -> Option<&'a crate::Site>;
    // fn site_mut(&mut self) -> &mut Option<&'a crate::Site>;
    // fn set_site<E>(&mut self, value: Option<E>) where E: Into<Site>;

    fn structures<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Structure>>;
    // fn structures_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Structure>>;
    // fn set_structures<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Structure>;

    fn spaces<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Space>>;
    // fn spaces_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Space>>;
    // fn set_spaces<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Space>;

    fn property_state_snapshots<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyStateSnapshot>>;
    // fn property_state_snapshots_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PropertyStateSnapshot>>;
    // fn set_property_state_snapshots<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PropertyStateSnapshot>;

    fn associations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyAssociation>>;
    // fn associations_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::PropertyAssociation>>;
    // fn set_associations<E>(&mut self, value: Option<&Vec<E>>) where E: Into<PropertyAssociation>;

    fn assessments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Assessment>>;
    // fn assessments_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Assessment>>;
    // fn set_assessments<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Assessment>;

    fn tax_bills<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxBill>>;
    // fn tax_bills_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::TaxBill>>;
    // fn set_tax_bills<E>(&mut self, value: Option<&Vec<E>>) where E: Into<TaxBill>;

    fn transfers<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Transfer>>;
    // fn transfers_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Transfer>>;
    // fn set_transfers<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Transfer>;

    fn sales<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SaleEvent>>;
    // fn sales_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::SaleEvent>>;
    // fn set_sales<E>(&mut self, value: Option<&Vec<E>>) where E: Into<SaleEvent>;

    fn listings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Listing>>;
    // fn listings_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Listing>>;
    // fn set_listings<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Listing>;

    fn leases<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEvent>>;
    // fn leases_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::LeaseEvent>>;
    // fn set_leases<E>(&mut self, value: Option<&Vec<E>>) where E: Into<LeaseEvent>;

    fn unit_rents<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::UnitRentObservation>>;
    // fn unit_rents_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::UnitRentObservation>>;
    // fn set_unit_rents<E>(&mut self, value: Option<&Vec<E>>) where E: Into<UnitRentObservation>;

    fn rent_rolls<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentRoll>>;
    // fn rent_rolls_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::RentRoll>>;
    // fn set_rent_rolls<E>(&mut self, value: Option<&Vec<E>>) where E: Into<RentRoll>;

    fn loans<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Loan>>;
    // fn loans_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Loan>>;
    // fn set_loans<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Loan>;

    fn liens<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Lien>>;
    // fn liens_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Lien>>;
    // fn set_liens<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Lien>;

    fn foreclosure_cases<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCase>>;
    // fn foreclosure_cases_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCase>>;
    // fn set_foreclosure_cases<E>(&mut self, value: Option<&Vec<E>>) where E: Into<ForeclosureCase>;

    fn permits<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Permit>>;
    // fn permits_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Permit>>;
    // fn set_permits<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Permit>;

    fn ownership<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OwnershipPeriod>>;
    // fn ownership_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::OwnershipPeriod>>;
    // fn set_ownership<E>(&mut self, value: Option<&Vec<E>>) where E: Into<OwnershipPeriod>;

    fn operating_statements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OperatingStatement>>;
    // fn operating_statements_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::OperatingStatement>>;
    // fn set_operating_statements<E>(&mut self, value: Option<&Vec<E>>) where E: Into<OperatingStatement>;

    fn valuations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Valuation>>;
    // fn valuations_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, crate::Valuation>>;
    // fn set_valuations<E>(&mut self, value: Option<&Vec<E>>) where E: Into<Valuation>;

    fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance>;
    // fn provenance_mut(&mut self) -> &mut Option<&'a crate::Provenance>;
    // fn set_provenance<E>(&mut self, value: Option<E>) where E: Into<Provenance>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl PropertyProfile for crate::PropertyProfile {
        fn parties<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Party>> {
        return self.parties.as_ref();
    }
        fn artifacts<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SourceArtifact>> {
        return self.artifacts.as_ref();
    }
        fn addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Address>> {
        return self.addresses.as_ref();
    }
        fn property<'a>(&'a self) -> &'a crate::Property {
        return &self.property;
    }
        fn property_addresses<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyAddress>> {
        return self.property_addresses.as_ref();
    }
        fn identifiers<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyIdentifier>> {
        return self.identifiers.as_ref();
    }
        fn jurisdictions<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Jurisdiction>> {
        return self.jurisdictions.as_ref();
    }
        fn parcels<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Parcel>> {
        return self.parcels.as_ref();
    }
        fn property_parcels<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyParcel>> {
        return self.property_parcels.as_ref();
    }
        fn parcel_lineage<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ParcelLineage>> {
        return self.parcel_lineage.as_ref();
    }
        fn site<'a>(&'a self) -> Option<&'a crate::Site> {
        return self.site.as_ref();
    }
        fn structures<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Structure>> {
        return self.structures.as_ref();
    }
        fn spaces<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Space>> {
        return self.spaces.as_ref();
    }
        fn property_state_snapshots<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyStateSnapshot>> {
        return self.property_state_snapshots.as_ref();
    }
        fn associations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::PropertyAssociation>> {
        return self.associations.as_ref();
    }
        fn assessments<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Assessment>> {
        return self.assessments.as_ref();
    }
        fn tax_bills<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::TaxBill>> {
        return self.tax_bills.as_ref();
    }
        fn transfers<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Transfer>> {
        return self.transfers.as_ref();
    }
        fn sales<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::SaleEvent>> {
        return self.sales.as_ref();
    }
        fn listings<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Listing>> {
        return self.listings.as_ref();
    }
        fn leases<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::LeaseEvent>> {
        return self.leases.as_ref();
    }
        fn unit_rents<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::UnitRentObservation>> {
        return self.unit_rents.as_ref();
    }
        fn rent_rolls<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::RentRoll>> {
        return self.rent_rolls.as_ref();
    }
        fn loans<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Loan>> {
        return self.loans.as_ref();
    }
        fn liens<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Lien>> {
        return self.liens.as_ref();
    }
        fn foreclosure_cases<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::ForeclosureCase>> {
        return self.foreclosure_cases.as_ref();
    }
        fn permits<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Permit>> {
        return self.permits.as_ref();
    }
        fn ownership<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OwnershipPeriod>> {
        return self.ownership.as_ref();
    }
        fn operating_statements<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::OperatingStatement>> {
        return self.operating_statements.as_ref();
    }
        fn valuations<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, crate::Valuation>> {
        return self.valuations.as_ref();
    }
        fn provenance<'a>(&'a self) -> Option<&'a crate::Provenance> {
        return self.provenance.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait AssessorObservation   {

    fn status<'a>(&'a self) -> &'a crate::AssessorStatus;
    // fn status_mut(&mut self) -> &mut &'a crate::AssessorStatus;
    // fn set_status(&mut self, value: AssessorStatus);

    fn query_address<'a>(&'a self) -> Option<&'a str>;
    // fn query_address_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_query_address(&mut self, value: Option<&'a str>);

    fn query_parcel_number<'a>(&'a self) -> Option<&'a str>;
    // fn query_parcel_number_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_query_parcel_number(&mut self, value: Option<&'a str>);

    fn assessor_url<'a>(&'a self) -> Option<&'a crate::uri>;
    // fn assessor_url_mut(&mut self) -> &mut Option<&'a crate::uri>;
    // fn set_assessor_url(&mut self, value: Option<&'a uri>);

    fn profile<'a>(&'a self) -> Option<&'a crate::PropertyProfile>;
    // fn profile_mut(&mut self) -> &mut Option<&'a crate::PropertyProfile>;
    // fn set_profile<E>(&mut self, value: Option<E>) where E: Into<PropertyProfile>;

    fn error<'a>(&'a self) -> Option<&'a str>;
    // fn error_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_error(&mut self, value: Option<&'a str>);

    fn provenance<'a>(&'a self) -> &'a crate::Provenance;
    // fn provenance_mut(&mut self) -> &mut &'a crate::Provenance;
    // fn set_provenance<E>(&mut self, value: E) where E: Into<Provenance>;

    fn artifact_refs<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifact_refs_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifact_refs<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl AssessorObservation for crate::AssessorObservation {
        fn status<'a>(&'a self) -> &'a crate::AssessorStatus {
        return &self.status;
    }
        fn query_address<'a>(&'a self) -> Option<&'a str> {
        return self.query_address.as_deref();
    }
        fn query_parcel_number<'a>(&'a self) -> Option<&'a str> {
        return self.query_parcel_number.as_deref();
    }
        fn assessor_url<'a>(&'a self) -> Option<&'a crate::uri> {
        return self.assessor_url.as_ref();
    }
        fn profile<'a>(&'a self) -> Option<&'a crate::PropertyProfile> {
        return self.profile.as_ref();
    }
        fn error<'a>(&'a self) -> Option<&'a str> {
        return self.error.as_deref();
    }
        fn provenance<'a>(&'a self) -> &'a crate::Provenance {
        return &self.provenance;
    }
        fn artifact_refs<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifact_refs.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}


pub trait ExtractionObservation   {

    fn status<'a>(&'a self) -> &'a crate::ExtractionStatus;
    // fn status_mut(&mut self) -> &mut &'a crate::ExtractionStatus;
    // fn set_status(&mut self, value: ExtractionStatus);

    fn category<'a>(&'a self) -> Option<&'a crate::ExtractionCategory>;
    // fn category_mut(&mut self) -> &mut Option<&'a crate::ExtractionCategory>;
    // fn set_category(&mut self, value: Option<&'a ExtractionCategory>);

    fn source_category<'a>(&'a self) -> Option<&'a str>;
    // fn source_category_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_source_category(&mut self, value: Option<&'a str>);

    fn error<'a>(&'a self) -> Option<&'a str>;
    // fn error_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_error(&mut self, value: Option<&'a str>);

    fn source_url<'a>(&'a self) -> Option<&'a crate::uri>;
    // fn source_url_mut(&mut self) -> &mut Option<&'a crate::uri>;
    // fn set_source_url(&mut self, value: Option<&'a uri>);

    fn extracted_at<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>>;
    // fn extracted_at_mut(&mut self) -> &mut Option<&'a crate::DateTime<FixedOffset>>;
    // fn set_extracted_at(&mut self, value: Option<&'a DateTime<FixedOffset>>);

    fn model<'a>(&'a self) -> Option<&'a str>;
    // fn model_mut(&mut self) -> &mut Option<&'a str>;
    // fn set_model(&mut self, value: Option<&'a str>);

    fn profile<'a>(&'a self) -> Option<&'a crate::PropertyProfile>;
    // fn profile_mut(&mut self) -> &mut Option<&'a crate::PropertyProfile>;
    // fn set_profile<E>(&mut self, value: Option<E>) where E: Into<PropertyProfile>;

    fn provenance<'a>(&'a self) -> &'a crate::Provenance;
    // fn provenance_mut(&mut self) -> &mut &'a crate::Provenance;
    // fn set_provenance<E>(&mut self, value: E) where E: Into<Provenance>;

    fn artifact_refs<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>>;
    // fn artifact_refs_mut(&mut self) -> &mut Option<impl poly_containers::SeqRef<'a, String>>;
    // fn set_artifact_refs<E>(&mut self, value: Option<&Vec<String>>) where E: Into<String>;

    fn extras<'a>(&'a self) -> Option<&'a crate::Any>;
    // fn extras_mut(&mut self) -> &mut Option<&'a crate::Any>;
    // fn set_extras<E>(&mut self, value: Option<E>) where E: Into<Any>;


}

impl ExtractionObservation for crate::ExtractionObservation {
        fn status<'a>(&'a self) -> &'a crate::ExtractionStatus {
        return &self.status;
    }
        fn category<'a>(&'a self) -> Option<&'a crate::ExtractionCategory> {
        return self.category.as_ref();
    }
        fn source_category<'a>(&'a self) -> Option<&'a str> {
        return self.source_category.as_deref();
    }
        fn error<'a>(&'a self) -> Option<&'a str> {
        return self.error.as_deref();
    }
        fn source_url<'a>(&'a self) -> Option<&'a crate::uri> {
        return self.source_url.as_ref();
    }
        fn extracted_at<'a>(&'a self) -> Option<&'a crate::DateTime<FixedOffset>> {
        return self.extracted_at.as_ref();
    }
        fn model<'a>(&'a self) -> Option<&'a str> {
        return self.model.as_deref();
    }
        fn profile<'a>(&'a self) -> Option<&'a crate::PropertyProfile> {
        return self.profile.as_ref();
    }
        fn provenance<'a>(&'a self) -> &'a crate::Provenance {
        return &self.provenance;
    }
        fn artifact_refs<'a>(&'a self) -> Option<impl poly_containers::SeqRef<'a, String>> {
        return self.artifact_refs.as_ref();
    }
        fn extras<'a>(&'a self) -> Option<&'a crate::Any> {
        return self.extras.as_ref();
    }
}
