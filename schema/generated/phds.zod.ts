// Generated from schema/generated/phds.schema.json by tools/gen_zod.py. DO NOT EDIT.
import * as z from "zod";

const rfc3986Path = /^(?:[A-Za-z0-9._~!$&'()*+,;=:@/-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986QueryOrFragment = /^(?:[A-Za-z0-9._~!$&'()*+,;=:@/?-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986UserInfo = /^(?:[A-Za-z0-9._~!$&'()*+,;=:-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986RegisteredName = /^(?:[A-Za-z0-9._~!$&'()*+,;=-]|%[0-9A-Fa-f]{2})*$/;
const rfc3986IpvFuture = /^[Vv][0-9A-Fa-f]+\.[A-Za-z0-9._~!$&'()*+,;=:-]+$/;

const isRfc3986Authority = (value: string): boolean => {
  const at = value.lastIndexOf("@");
  if (at >= 0 && !rfc3986UserInfo.test(value.slice(0, at))) return false;
  const hostAndPort = value.slice(at + 1);
  if (hostAndPort.startsWith("[")) {
    const close = hostAndPort.indexOf("]");
    if (close < 0 || !/^(?::[0-9]*)?$/.test(hostAndPort.slice(close + 1))) return false;
    const literal = hostAndPort.slice(1, close);
    if (rfc3986IpvFuture.test(literal)) return true;
    try {
      new URL(`http://[${literal}]/`);
      return true;
    } catch {
      return false;
    }
  }
  const colon = hostAndPort.lastIndexOf(":");
  const hasPort = colon >= 0;
  const host = hasPort ? hostAndPort.slice(0, colon) : hostAndPort;
  const port = hasPort ? hostAndPort.slice(colon + 1) : "";
  return !host.includes(":") && rfc3986RegisteredName.test(host) && (!hasPort || /^[0-9]*$/.test(port));
};

const isRfc3986Uri = (value: string): boolean => {
  const match = /^([A-Za-z][A-Za-z0-9+.-]*):(?:\/\/([^/?#]*))?([^?#]*)(?:\?([^#]*))?(?:#(.*))?$/.exec(value);
  if (match === null) return false;
  const [, , authority, path, query, fragment] = match;
  return (authority === undefined || isRfc3986Authority(authority))
    && rfc3986Path.test(path)
    && (query === undefined || rfc3986QueryOrFragment.test(query))
    && (fragment === undefined || rfc3986QueryOrFragment.test(fragment));
};

const rfc3986UriSchema = z.string().refine(isRfc3986Uri, { message: "Invalid RFC 3986 URI" });

const publishedLeapSecondDates = new Set([
  "1972-06-30", "1972-12-31", "1973-12-31", "1974-12-31", "1975-12-31",
  "1976-12-31", "1977-12-31", "1978-12-31", "1979-12-31", "1981-06-30",
  "1982-06-30", "1983-06-30", "1985-06-30", "1987-12-31", "1989-12-31",
  "1990-12-31", "1992-06-30", "1993-06-30", "1994-06-30", "1995-12-31",
  "1997-06-30", "1998-12-31", "2005-12-31", "2008-12-31", "2012-06-30",
  "2015-06-30", "2016-12-31",
]);

const isRfc3339DateTime = (value: string): boolean => {
  const match = /^(\d{4})-(\d{2})-(\d{2})[Tt](\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(?:[Zz]|([+-])(\d{2}):(\d{2}))$/.exec(value);
  if (match === null) return false;
  const [, yearText, monthText, dayText, hourText, minuteText, secondText, sign, offsetHourText = "00", offsetMinuteText = "00"] = match;
  const [year, month, day, hour, minute, second, offsetHour, offsetMinute] = [yearText, monthText, dayText, hourText, minuteText, secondText, offsetHourText, offsetMinuteText].map(Number);
  const leapYear = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  const daysInMonth = [31, leapYear ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month - 1];
  if (daysInMonth === undefined || day < 1 || day > daysInMonth || hour > 23 || minute > 59 || second > 60 || offsetHour > 23 || offsetMinute > 59) return false;
  if (second < 60) return true;
  const instant = new Date(0);
  instant.setUTCFullYear(year, month - 1, day);
  instant.setUTCHours(hour, minute, 59, 0);
  const signedOffset = sign === "+" ? offsetHour * 60 + offsetMinute : sign === "-" ? -(offsetHour * 60 + offsetMinute) : 0;
  instant.setUTCMinutes(instant.getUTCMinutes() - signedOffset);
  const utcDate = `${String(instant.getUTCFullYear()).padStart(4, "0")}-${String(instant.getUTCMonth() + 1).padStart(2, "0")}-${String(instant.getUTCDate()).padStart(2, "0")}`;
  return instant.getUTCHours() === 23 && instant.getUTCMinutes() === 59 && publishedLeapSecondDates.has(utcDate);
};

const rfc3339DateTimeSchema = z.string().refine(isRfc3339DateTime, { message: "Invalid RFC 3339 date-time" });

/**
 * Open map — the `extras` escape hatch on every entity.
 */
export const AnySchema = z.union([z.boolean(), z.looseObject({}), z.number(), z.string()]).nullable().meta({description: "Open map — the `extras` escape hatch on every entity.", title: "Any"});

export type Any = z.infer<typeof AnySchema>;

/**
 * WGS84 coordinate.
 */
export const GeoPointSchema = z.strictObject({"latitude": z.number().min(-90).max(90), "longitude": z.number().min(-180).max(180)}).meta({description: "WGS84 coordinate.", title: "GeoPoint"});

export type GeoPoint = z.infer<typeof GeoPointSchema>;

/**
 */
export const GeocodeAccuracySchema = z.intersection(z.enum(["rooftop", "parcel", "street", "postal_centroid", "locality_centroid", "manual", "unknown"]), z.string()).meta({description: "", title: "GeocodeAccuracy"});

export type GeocodeAccuracy = z.infer<typeof GeocodeAccuracySchema>;

/**
 */
export const CaptureMethodSchema = z.intersection(z.enum(["api", "scrape", "llm_extraction", "manual", "bulk"]), z.string()).meta({description: "", title: "CaptureMethod"});

export type CaptureMethod = z.infer<typeof CaptureMethodSchema>;

/**
 */
export const VerificationStatusSchema = z.intersection(z.enum(["unverified", "pending_review", "verified", "disputed", "rejected"]), z.string()).meta({description: "", title: "VerificationStatus"});

export type VerificationStatus = z.infer<typeof VerificationStatusSchema>;

/**
 * Record-level source envelope. Pure child rows (parties, areas, escalations, filings) inherit the parent envelope unless they carry their own.
 */
export const ProvenanceSchema = z.strictObject({"confidence": z.number().min(0).max(1).nullable().optional().meta({description: "Fraction from 0 through 1; 0.8 means 80 percent confidence."}), "method": CaptureMethodSchema.optional(), "originating_system": z.string().nullable().optional().meta({description: "System where the record was born"}), "provider": z.string().nullable().optional(), "retrieved_at": rfc3339DateTimeSchema.nullable().optional(), "source_created_at": rfc3339DateTimeSchema.nullable().optional().meta({description: "When the immediate source created the record"}), "source_modified_at": rfc3339DateTimeSchema.nullable().optional().meta({description: "When the immediate source last modified the record"}), "source_record_id": z.string().nullable().optional().meta({description: "The immediate source's record identifier"}), "source_system": z.string().nullable().optional().meta({description: "Immediate system that supplied the record"}), "source_url": rfc3986UriSchema.nullable().optional(), "verification": VerificationStatusSchema.optional()}).meta({description: "Record-level source envelope. Pure child rows (parties, areas, escalations, filings) inherit the parent envelope unless they carry their own.", title: "Provenance"});

export type Provenance = z.infer<typeof ProvenanceSchema>;

/**
 * Verification performed by one canonical party.
 */
export const VerificationAttributionSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "note": z.string().nullable().optional(), "verified_at": rfc3339DateTimeSchema, "verifier": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])"))}).meta({description: "Verification performed by one canonical party.", title: "VerificationAttribution"});

export type VerificationAttribution = z.infer<typeof VerificationAttributionSchema>;

/**
 */
export const AddressSchema = z.strictObject({"address_hash": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Producer-computed matching key; comparable only under the same scheme"}), "address_hash_scheme": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Producer-namespaced normalization and hashing scheme"}), "admin_area": z.string().nullable().optional().meta({description: "county/district name"}), "admin_area_code": z.string().nullable().optional().meta({description: "authority code"}), "city": z.string().nullable().optional(), "country": z.string().regex(new RegExp("^[A-Z]{2}(?![\\s\\S])")), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "location": z.union([GeoPointSchema, z.null()]).optional(), "location_accuracy": GeocodeAccuracySchema.optional(), "postal_code": z.string().nullable().optional(), "postal_code_suffix": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "region": z.string().nullable().optional(), "street_name": z.string().nullable().optional(), "street_number": z.string().nullable().optional(), "street_post_direction": z.string().nullable().optional(), "street_pre_direction": z.string().nullable().optional(), "street_suffix": z.string().nullable().optional(), "sublocality": z.string().nullable().optional().meta({description: "neighborhood / borough / barrio"}), "unformatted_address": z.string().nullable().optional().meta({description: "Fallback when components do not parse"}), "unit_number": z.string().nullable().optional(), "unit_type": z.string().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "Address"});

export type Address = z.infer<typeof AddressSchema>;

/**
 * Source coding preserved alongside canonical values (system + code + display).
 */
export const CodeableConceptSchema = z.strictObject({"code": z.string().nullable().optional(), "display": z.string().nullable().optional(), "system": z.string().nullable().optional()}).meta({description: "Source coding preserved alongside canonical values (system + code + display).", title: "CodeableConcept"});

export type CodeableConcept = z.infer<typeof CodeableConceptSchema>;

/**
 */
export const AddressAssociationSchema = z.strictObject({"address": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "is_primary": z.boolean().nullable().optional(), "role": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "situs | entrance | alias | address_range | former | mailing | other (open vocabulary)"}), "valid_from": z.iso.date().nullable().optional(), "valid_to": z.iso.date().nullable().optional()}).meta({description: "", title: "AddressAssociation"});

export type AddressAssociation = z.infer<typeof AddressAssociationSchema>;

/**
 */
export const AreaUnitSchema = z.intersection(z.enum(["sqft", "sqm", "acre", "hectare"]), z.string()).meta({description: "", title: "AreaUnit"});

export type AreaUnit = z.infer<typeof AreaUnitSchema>;

/**
 * Area measurement with explicit unit.
 */
export const AreaSchema = z.strictObject({"unit": AreaUnitSchema, "value": z.number()}).meta({description: "Area measurement with explicit unit.", title: "Area"});

export type Area = z.infer<typeof AreaSchema>;

/**
 */
export const AreaMeasureSchema = z.strictObject({"area": AreaSchema, "extras": z.union([AnySchema, z.null()]).optional(), "kind": z.string()}).meta({description: "", title: "AreaMeasure"});

export type AreaMeasure = z.infer<typeof AreaMeasureSchema>;

/**
 * Monetary amount. JSON wire format is decimal-as-string. ISO-4217 currency.
 */
export const MoneySchema = z.strictObject({"amount": z.string().regex(new RegExp("^-?[0-9]+(\\.[0-9]+)?(?![\\s\\S])")), "currency": z.string().regex(new RegExp("^[A-Z]{3}(?![\\s\\S])")).meta({description: "ISO-4217 currency code"})}).meta({description: "Monetary amount. JSON wire format is decimal-as-string. ISO-4217 currency.", title: "Money"});

export type Money = z.infer<typeof MoneySchema>;

/**
 */
export const TaxExemptionSchema = z.strictObject({"amount": z.union([MoneySchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "kind": z.string().meta({description: "homestead | senior | veteran | ... (open)"})}).meta({description: "", title: "TaxExemption"});

export type TaxExemption = z.infer<typeof TaxExemptionSchema>;

/**
 * Parcel-first tax-roll values; identity = parcel + assessing jurisdiction + year + roll type.
 */
export const AssessmentSchema = z.strictObject({"appraised_improvement_value": z.union([MoneySchema, z.null()]).optional(), "appraised_land_value": z.union([MoneySchema, z.null()]).optional(), "appraised_total_value": z.union([MoneySchema, z.null()]).optional(), "assessed_improvement_value": z.union([MoneySchema, z.null()]).optional(), "assessed_land_value": z.union([MoneySchema, z.null()]).optional(), "assessed_total_value": z.union([MoneySchema, z.null()]).optional(), "exemptions": z.array(TaxExemptionSchema).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "jurisdiction": z.string().meta({description: "The ASSESSING authority"}), "market_improvement_value": z.union([MoneySchema, z.null()]).optional(), "market_land_value": z.union([MoneySchema, z.null()]).optional(), "market_total_value": z.union([MoneySchema, z.null()]).optional(), "parcel": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "roll_type": z.string().nullable().optional().meta({description: "original | corrected | supplemental | appeal | ..."}), "tax_year": z.number().int().min(1000).max(2200), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Parcel-first tax-roll values; identity = parcel + assessing jurisdiction + year + roll type.", title: "Assessment"});

export type Assessment = z.infer<typeof AssessmentSchema>;

/**
 * Outcome of an assessor or public-records lookup.
 */
export const AssessorStatusSchema = z.intersection(z.enum(["success", "not_found", "timeout", "api_error", "parse_error", "invalid_address", "ambiguous"]), z.string()).meta({description: "Outcome of an assessor or public-records lookup.", title: "AssessorStatus"});

export type AssessorStatus = z.infer<typeof AssessorStatusSchema>;

/**
 * Open contextual role. Mortgage and deed-of-trust proceedings commonly use lender | trustee | borrower; lien foreclosures may instead use claimant | debtor.
 */
export const ForeclosureCasePartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "Open contextual role. Mortgage and deed-of-trust proceedings commonly use lender | trustee | borrower; lien foreclosures may instead use claimant | debtor.", title: "ForeclosureCaseParty"});

export type ForeclosureCaseParty = z.infer<typeof ForeclosureCasePartySchema>;

/**
 * Cross-reference to another recorded instrument (e.g., an assignment citing the original mortgage's recording number) — the target may not exist as a PHDS row.
 */
export const InstrumentReferenceSchema = z.strictObject({"document_number": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "recording_authority": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional(), "relationship_type": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "re_records | corrects | releases | assigns | modifies | subordinates | refers_to"})}).meta({description: "Cross-reference to another recorded instrument (e.g., an assignment citing the original mortgage's recording number) — the target may not exist as a PHDS row.", title: "InstrumentReference"});

export type InstrumentReference = z.infer<typeof InstrumentReferenceSchema>;

/**
 * One recorded filing; postponements append as new filings.
 */
export const ForeclosureFilingSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "auction_at_time": z.string().nullable().optional().meta({description: "Time-of-day as published"}), "auction_date": z.iso.date().nullable().optional(), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional(), "status": z.string().meta({description: "nod | lis_pendens | notice_of_sale | auction_scheduled | postponement | ... (open; US-seeded)"})}).meta({description: "One recorded filing; postponements append as new filings.", title: "ForeclosureFiling"});

export type ForeclosureFiling = z.infer<typeof ForeclosureFilingSchema>;

/**
 * A foreclosure PROCEEDING; its recorded filings append over time.
 */
export const ForeclosureCaseSchema = z.strictObject({"auction_location": z.string().nullable().optional(), "auction_min_bid": z.union([MoneySchema, z.null()]).optional(), "case_number": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "filings": z.array(ForeclosureFilingSchema).nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "loan": z.string().nullable().optional().meta({description: "The defaulted loan, when known"}), "opened_date": z.iso.date().nullable().optional(), "original_loan_amount": z.union([MoneySchema, z.null()]).optional(), "parties": z.array(ForeclosureCasePartySchema).nullable().optional(), "past_due_amount": z.union([MoneySchema, z.null()]).optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "resolution": z.string().nullable().optional().meta({description: "sold_at_auction | cured | dismissed | reo"}), "resolved_date": z.iso.date().nullable().optional(), "unpaid_balance": z.union([MoneySchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "A foreclosure PROCEEDING; its recorded filings append over time.", title: "ForeclosureCase"});

export type ForeclosureCase = z.infer<typeof ForeclosureCaseSchema>;

/**
 * GeoJSON Geometry object (typed-in-standard; optional everywhere).
 */
export const GeometrySchema = z.strictObject({"coordinates": z.union([AnySchema, z.null()]).optional(), "type": z.string().meta({description: "Point | MultiPolygon | ..."})}).meta({description: "GeoJSON Geometry object (typed-in-standard; optional everywhere).", title: "Geometry"});

export type Geometry = z.infer<typeof GeometrySchema>;

/**
 */
export const JurisdictionSchema = z.strictObject({"authority_code": z.string().nullable().optional().meta({description: "US: FIPS/GEOID"}), "boundary": z.union([GeometrySchema, z.null()]).optional(), "country": z.string().regex(new RegExp("^[A-Z]{2}(?![\\s\\S])")).meta({description: "ISO 3166-1 alpha-2 (no default — i18n)"}), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": z.string().meta({description: "county | municipality | taxing_district | ... (open vocab)"}), "name": z.string(), "parent": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "region": z.string().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "Jurisdiction"});

export type Jurisdiction = z.infer<typeof JurisdictionSchema>;

/**
 * Who pays what (taxes/insurance/CAM/utilities).
 */
export const ExpenseStructureSchema = z.strictObject({"cam_paid_by": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "insurance_paid_by": z.string().nullable().optional(), "notes": z.string().nullable().optional(), "taxes_paid_by": z.string().nullable().optional().meta({description: "landlord | tenant | shared"}), "utilities_paid_by": z.string().nullable().optional()}).meta({description: "Who pays what (taxes/insurance/CAM/utilities).", title: "ExpenseStructure"});

export type ExpenseStructure = z.infer<typeof ExpenseStructureSchema>;

/**
 */
export const LeaseConcessionSchema = z.strictObject({"abatement_months": z.number().nullable().optional(), "abatement_percent": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 10 means 10 percent."}), "concession_type": z.string().meta({description: "free_rent | ti_allowance | moving_allowance | ..."}), "concession_value": z.union([MoneySchema, z.null()]).optional(), "conditions": z.union([AnySchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "notes": z.string().nullable().optional(), "ti_cap_total": z.union([MoneySchema, z.null()]).optional()}).meta({description: "", title: "LeaseConcession"});

export type LeaseConcession = z.infer<typeof LeaseConcessionSchema>;

/**
 */
export const RentStepSchema = z.strictObject({"amount": MoneySchema, "from_date": z.iso.date()}).meta({description: "", title: "RentStep"});

export type RentStep = z.infer<typeof RentStepSchema>;

/**
 */
export const LeaseEscalationSchema = z.strictObject({"cpi_cap": z.number().nullable().optional().meta({description: "CPI escalation cap in percentage points; 5 means 5 percent."}), "cpi_floor": z.number().nullable().optional().meta({description: "CPI escalation floor in percentage points; 2 means 2 percent."}), "cpi_index": z.string().nullable().optional(), "effective_from": z.iso.date().nullable().optional(), "effective_until": z.iso.date().nullable().optional(), "escalation_type": z.string().meta({description: "fixed_amount | fixed_percent | cpi | step_schedule | fmv | none"}), "escalation_value": z.number().nullable().optional().meta({description: "For escalation_type=fixed_percent, a value from 0-100 percentage points; 3 means 3 percent. For escalation_type=fixed_amount, the increment in the currency of the parent LeaseEvent.rent and the period specified by LeaseEvent.rent_period."}), "extras": z.union([AnySchema, z.null()]).optional(), "frequency_months": z.number().int().nullable().optional(), "steps": z.array(RentStepSchema).nullable().optional()}).meta({description: "", title: "LeaseEscalation"});

export type LeaseEscalation = z.infer<typeof LeaseEscalationSchema>;

/**
 * role = lessee | landlord | guarantor | lessee_broker | landlord_broker
 */
export const LeaseEventPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = lessee | landlord | guarantor | lessee_broker | landlord_broker", title: "LeaseEventParty"});

export type LeaseEventParty = z.infer<typeof LeaseEventPartySchema>;

/**
 */
export const LeaseTypeEnumSchema = z.intersection(z.enum(["gross", "modified_gross", "triple_net", "double_net", "single_net", "absolute_net", "percentage", "ground", "residential", "other"]), z.string()).meta({description: "", title: "LeaseTypeEnum"});

export type LeaseTypeEnum = z.infer<typeof LeaseTypeEnumSchema>;

/**
 */
export const RentPeriodSchema = z.intersection(z.enum(["daily", "monthly", "annual", "per_area_annual", "per_area_monthly"]), z.string()).meta({description: "", title: "RentPeriod"});

export type RentPeriod = z.infer<typeof RentPeriodSchema>;

/**
 * Money per explicit denominator (price/sqft, rent/sqm/month, spaces/1000 sqft).
 */
export const UnitRateSchema = z.strictObject({"amount": z.string().regex(new RegExp("^-?[0-9]+(\\.[0-9]+)?(?![\\s\\S])")), "currency": z.string().regex(new RegExp("^[A-Z]{3}(?![\\s\\S])")).meta({description: "ISO-4217 currency code"}), "denominator": z.string().meta({description: "sqft | sqm | unit | bed | key | month | sqft_month | 1000_sqft | ..."})}).meta({description: "Money per explicit denominator (price/sqft, rent/sqm/month, spaces/1000 sqft).", title: "UnitRate"});

export type UnitRate = z.infer<typeof UnitRateSchema>;

/**
 * Executed leases (not asking rents — see UnitRentObservation).
 */
export const LeaseEventSchema = z.strictObject({"commencement_date": z.iso.date().nullable().optional(), "concessions": z.array(LeaseConcessionSchema).nullable().optional(), "effective_rent_per_area": z.union([UnitRateSchema, z.null()]).optional(), "escalations": z.array(LeaseEscalationSchema).nullable().optional(), "execution_date": z.iso.date().nullable().optional(), "expense_structure": z.union([ExpenseStructureSchema, z.null()]).optional(), "expiration_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "free_rent_months": z.number().nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "lease_type": LeaseTypeEnumSchema.optional(), "leased_area": z.union([AreaSchema, z.null()]).optional(), "net_effective_rent_per_area": z.union([UnitRateSchema, z.null()]).optional(), "parties": z.array(LeaseEventPartySchema).nullable().optional(), "property": z.string(), "property_state": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "remarks": z.string().nullable().optional().meta({description: "Source- or vendor-authored narrative interpreted through provenance."}), "rent": z.union([MoneySchema, z.null()]).optional(), "rent_period": RentPeriodSchema.optional(), "space": z.string().nullable().optional(), "starting_rent_per_area": z.union([UnitRateSchema, z.null()]).optional(), "term_months": z.number().int().min(1).nullable().optional(), "ti_allowance_per_area": z.union([UnitRateSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Executed leases (not asking rents — see UnitRentObservation).", title: "LeaseEvent"});

export type LeaseEvent = z.infer<typeof LeaseEventSchema>;

/**
 * role = claimant | debtor
 */
export const LienPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = claimant | debtor", title: "LienParty"});

export type LienParty = z.infer<typeof LienPartySchema>;

/**
 */
export const LienTypeSchema = z.intersection(z.enum(["tax", "judgment", "hoa", "mechanics", "municipal", "other"]), z.string()).meta({description: "", title: "LienType"});

export type LienType = z.infer<typeof LienTypeSchema>;

/**
 * Involuntary encumbrances — tax, judgment, HOA, mechanic's.
 */
export const LienSchema = z.strictObject({"amount": z.union([MoneySchema, z.null()]).optional(), "artifacts": z.array(z.string()).nullable().optional(), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "kind": LienTypeSchema, "parties": z.array(LienPartySchema).nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional(), "released_date": z.iso.date().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Involuntary encumbrances — tax, judgment, HOA, mechanic's.", title: "Lien"});

export type Lien = z.infer<typeof LienSchema>;

/**
 * PHDS-normalized listing status (close mapping to RESO StandardStatus). sold/leased replace RESO's ambiguous Closed; Incomplete and Delete are MLS record-management states that normalize to other with the raw value preserved in extras.
 */
export const ListingStatusSchema = z.intersection(z.enum(["coming_soon", "active", "active_under_contract", "pending", "hold", "sold", "leased", "withdrawn", "canceled", "expired", "other"]), z.string()).meta({description: "PHDS-normalized listing status (close mapping to RESO StandardStatus). sold/leased replace RESO's ambiguous Closed; Incomplete and Delete are MLS record-management states that normalize to other with the raw value preserved in extras.", title: "ListingStatus"});

export type ListingStatus = z.infer<typeof ListingStatusSchema>;

/**
 * A dated listing assertion. effective_date is when the change took effect; effective_at adds optional precision and must fall on effective_date (compared on the timestamp's lexical date in its stated offset). observed_at is when the producer retrieved or detected the event. Order events by effective_date ascending, then effective_at when present, then array order. list_price is the asking sale price or periodic rent; rent_period states the period when it is rent. close_price is asserted only on a closed event and is the source's reported closing claim, not the reconciled SaleEvent.sale_price.
 */
export const ListingEventSchema = z.strictObject({"close_price": z.union([MoneySchema, z.null()]).optional(), "effective_at": rfc3339DateTimeSchema.nullable().optional(), "effective_date": z.iso.date(), "event_type": z.string().meta({description: "listed | price_change | status_change | coming_soon | back_on_market | relisted | contingent | pending | closed | expired | withdrawn | canceled (open)"}), "extras": z.union([AnySchema, z.null()]).optional(), "list_price": z.union([MoneySchema, z.null()]).optional(), "list_price_low": z.union([MoneySchema, z.null()]).optional().meta({description: "Lower bound for range-priced listings"}), "observed_at": rfc3339DateTimeSchema.nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rent_period": RentPeriodSchema.optional(), "source_status": z.string().nullable().optional().meta({description: "Native source status verbatim"}), "status": ListingStatusSchema.optional()}).meta({description: "A dated listing assertion. effective_date is when the change took effect; effective_at adds optional precision and must fall on effective_date (compared on the timestamp's lexical date in its stated offset). observed_at is when the producer retrieved or detected the event. Order events by effective_date ascending, then effective_at when present, then array order. list_price is the asking sale price or periodic rent; rent_period states the period when it is rent. close_price is asserted only on a closed event and is the source's reported closing claim, not the reconciled SaleEvent.sale_price.", title: "ListingEvent"});

export type ListingEvent = z.infer<typeof ListingEventSchema>;

/**
 * Namespaced listing identifier mirroring PropertyIdentifier (scheme + namespace + value). RESO ListingKey and ListingId stay separately recoverable as distinct schemes; identity rule is (namespace, scheme=listing_key, value) with provider-specific fallback. At most one is_primary identifier per namespace.
 */
export const ListingIdentifierSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "is_primary": z.boolean().nullable().optional(), "namespace": z.string().nullable().optional().meta({description: "Issuing system: MLS, aggregator, or portal"}), "scheme": z.string().meta({description: "listing_key | listing_id | source_system_id | other (open)"}), "value": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])"))}).meta({description: "Namespaced listing identifier mirroring PropertyIdentifier (scheme + namespace + value). RESO ListingKey and ListingId stay separately recoverable as distinct schemes; identity rule is (namespace, scheme=listing_key, value) with provider-specific fallback. At most one is_primary identifier per namespace.", title: "ListingIdentifier"});

export type ListingIdentifier = z.infer<typeof ListingIdentifierSchema>;

/**
 * role = listing_agent | co_listing_agent | buyer_agent | listing_brokerage | selling_brokerage
 */
export const ListingParticipantSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = listing_agent | co_listing_agent | buyer_agent | listing_brokerage | selling_brokerage", title: "ListingParticipant"});

export type ListingParticipant = z.infer<typeof ListingParticipantSchema>;

/**
 */
export const OfferingTypeSchema = z.intersection(z.enum(["for_sale", "for_lease"]), z.string()).meta({description: "", title: "OfferingType"});

export type OfferingType = z.infer<typeof OfferingTypeSchema>;

/**
 * Listing identity, header identifiers, and latest-observed agreement terms; process events by effective_date ascending; array order breaks same-date ties. Carry status, list_price, and rent_period forward independently from the latest event that supplies each field. Original asking terms come from the earliest event supplying them. close_price comes from the latest closed event supplying it.
 */
export const ListingSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "events": z.array(ListingEventSchema).nullable().optional(), "expiration_date": z.iso.date().nullable().optional(), "exposure_type": z.string().nullable().optional().meta({description: "public | office_exclusive | private | coming_soon | other (open)"}), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "identifiers": z.array(ListingIdentifierSchema).nullable().optional(), "listing_agreement_type": z.string().nullable().optional().meta({description: "exclusive_right_to_sell | exclusive_agency | open | net | other (open)"}), "listing_contract_date": z.iso.date().nullable().optional(), "marketing_channel": z.string().nullable().optional().meta({description: "mls | private_network | public_portal | owner_direct | auction_platform | other (open)"}), "offering_type": OfferingTypeSchema, "participants": z.array(ListingParticipantSchema).nullable().optional(), "property": z.string(), "property_state": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "remarks": z.string().nullable().optional().meta({description: "Source- or vendor-authored narrative interpreted through provenance."}), "service_level": z.string().nullable().optional().meta({description: "full_service | limited_service | entry_only | other (open)"}), "special_listing_conditions": z.array(CodeableConceptSchema).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Listing identity, header identifiers, and latest-observed agreement terms; process events by effective_date ascending; array order breaks same-date ties. Carry status, list_price, and rent_period forward independently from the latest event that supplies each field. Original asking terms come from the earliest event supplying them. close_price comes from the latest closed event supplying it.", title: "Listing"});

export type Listing = z.infer<typeof ListingSchema>;

/**
 */
export const LoanEventTypeSchema = z.intersection(z.enum(["origination", "assignment", "modification", "satisfaction", "release", "default", "reinstatement", "other"]), z.string()).meta({description: "", title: "LoanEventType"});

export type LoanEventType = z.infer<typeof LoanEventTypeSchema>;

/**
 * Dated loan lifecycle — assignments, modifications, satisfactions. Recording fields apply when the event is a recorded instrument; they stay null for unrecorded servicing events.
 */
export const LoanEventSchema = z.strictObject({"amount": z.union([MoneySchema, z.null()]).optional(), "artifacts": z.array(z.string()).nullable().optional(), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "effective_date": z.iso.date().nullable().optional(), "event_type": LoanEventTypeSchema, "extras": z.union([AnySchema, z.null()]).optional(), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional(), "to_party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Canonical assignee for assignment-like events"})}).meta({description: "Dated loan lifecycle — assignments, modifications, satisfactions. Recording fields apply when the event is a recorded instrument; they stay null for unrecorded servicing events.", title: "LoanEvent"});

export type LoanEvent = z.infer<typeof LoanEventSchema>;

/**
 * role = borrower | lender | beneficiary | trustee (assignees live in loan events)
 */
export const LoanPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = borrower | lender | beneficiary | trustee (assignees live in loan events)", title: "LoanParty"});

export type LoanParty = z.infer<typeof LoanPartySchema>;

/**
 * Recorded debt. Originating lender identity and classification live on the canonical Party referenced by a LoanParty. Assignments, modifications, satisfactions, and other lifecycle assertions are dated events; consumers derive current status and satisfaction dates.
 */
export const LoanSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "due_date": z.iso.date().nullable().optional(), "events": z.array(LoanEventSchema).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "interest_rate": z.number().min(0).nullable().optional().meta({description: "Nominal interest rate in percentage points; 6.5 means 6.5 percent."}), "is_assumable": z.boolean().nullable().optional(), "is_construction": z.boolean().nullable().optional(), "is_heloc": z.boolean().nullable().optional(), "is_purchase_money": z.boolean().nullable().optional(), "is_seller_carryback": z.boolean().nullable().optional(), "is_variable_rate": z.boolean().nullable().optional(), "lien_position": z.number().int().min(1).nullable().optional(), "loan_amount": z.union([MoneySchema, z.null()]).optional(), "loan_type": z.string().nullable().optional().meta({description: "conventional | fha | va | ... (open)"}), "parcel": z.string().nullable().optional(), "parties": z.array(LoanPartySchema).nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "purpose": z.string().nullable().optional().meta({description: "purchase | refinance | construction | heloc | ..."}), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional(), "term_months": z.number().int().min(1).nullable().optional(), "transfer": z.string().nullable().optional().meta({description: "Purchase-money linkage"}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Recorded debt. Originating lender identity and classification live on the canonical Party referenced by a LoanParty. Assignments, modifications, satisfactions, and other lifecycle assertions are dated events; consumers derive current status and satisfaction dates.", title: "Loan"});

export type Loan = z.infer<typeof LoanSchema>;

/**
 * What an operating statement represents.
 */
export const StatementBasisSchema = z.intersection(z.enum(["actual", "budget", "pro_forma", "stabilized", "projected", "other"]), z.string()).meta({description: "What an operating statement represents.", title: "StatementBasis"});

export type StatementBasis = z.infer<typeof StatementBasisSchema>;

/**
 */
export const StatementLineItemSchema = z.strictObject({"amount": MoneySchema, "category": z.string().meta({description: "income | expense | capital | other (open vocab)"}), "extras": z.union([AnySchema, z.null()]).optional(), "label": z.string()}).meta({description: "", title: "StatementLineItem"});

export type StatementLineItem = z.infer<typeof StatementLineItemSchema>;

/**
 * Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).
 */
export const OperatingStatementSchema = z.strictObject({"capex": z.union([MoneySchema, z.null()]).optional(), "egi": z.union([MoneySchema, z.null()]).optional().meta({description: "Effective gross income"}), "extras": z.union([AnySchema, z.null()]).optional(), "ground_lease_expense": z.union([MoneySchema, z.null()]).optional(), "ground_lease_included_in_opex": z.boolean().nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "line_items": z.array(StatementLineItemSchema).nullable().optional(), "noi": z.union([MoneySchema, z.null()]).optional().meta({description: "Net operating income; may be negative"}), "opex_total": z.union([MoneySchema, z.null()]).optional(), "period_end": z.iso.date().nullable().optional(), "period_start": z.iso.date().nullable().optional().meta({description: "For fiscal-year, trailing-12, or partial periods. period_start and period_end must be provided together; omit both for calendar-year statements."}), "pgi": z.union([MoneySchema, z.null()]).optional().meta({description: "Potential gross income"}), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "reimbursements": z.union([MoneySchema, z.null()]).optional().meta({description: "Expense recoveries from tenants (CRE)"}), "reserves": z.union([MoneySchema, z.null()]).optional().meta({description: "Reserves for replacement"}), "reserves_included_in_opex": z.boolean().nullable().optional(), "statement_basis": StatementBasisSchema.optional(), "statement_year": z.number().int().min(1000).max(2200).meta({description: "The calendar year the statement is for (the year containing period_end for fiscal/trailing periods)."}), "vacancy_loss": z.union([MoneySchema, z.null()]).optional(), "vacancy_pct": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 5 means 5 percent."}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Operating performance of a property for one period: income, vacancy, expenses, NOI. The stated totals are authoritative; line_items provide detail and are not required to sum to the totals. Canonical relations: egi = pgi - vacancy_loss (+ reimbursements where recovered); noi = egi - opex_total. capex and reserves sit below the NOI line unless the corresponding *_included_in_opex flag is true; same for ground_lease_expense. All Money fields on one statement, including line items, MUST share a single currency (validator-enforced, not expressible in LinkML). Growth-rate and other model assumptions are out of scope (valuation extension).", title: "OperatingStatement"});

export type OperatingStatement = z.infer<typeof OperatingStatementSchema>;

/**
 */
export const OwnershipInterestSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "interest_pct": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 75 means 75 percent."}), "is_owner_occupied": z.boolean().nullable().optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string().nullable().optional().meta({description: "owner | trustee | gp | lp"})}).meta({description: "", title: "OwnershipInterest"});

export type OwnershipInterest = z.infer<typeof OwnershipInterestSchema>;

/**
 * One ownership regime; end_date is end-exclusive, NULL = current.
 */
export const OwnershipPeriodSchema = z.strictObject({"acquired_via_transfer": z.string().nullable().optional(), "disposed_via_transfer": z.string().nullable().optional(), "end_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "interests": z.array(OwnershipInterestSchema).nullable().optional(), "mailing_address": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Owner's mailing address during THIS period"}), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "start_date": z.iso.date().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional(), "vesting_type": z.string().nullable().optional().meta({description: "joint_tenants | tenants_in_common | community_property | ... (open)"})}).meta({description: "One ownership regime; end_date is end-exclusive, NULL = current.", title: "OwnershipPeriod"});

export type OwnershipPeriod = z.infer<typeof OwnershipPeriodSchema>;

/**
 */
export const ParcelSchema = z.strictObject({"boundary": z.union([GeometrySchema, z.null()]).optional().meta({description: "GeoJSON MultiPolygon (optional)"}), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "jurisdiction": z.string(), "land_area": z.union([AreaSchema, z.null()]).optional(), "legal_description": z.string().nullable().optional(), "normalized_parcel_number": z.string().nullable().optional().meta({description: "Matching only, never identity"}), "parcel_number": z.string().meta({description: "RAW as issued (RESO UPI rule)"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "retired_date": z.iso.date().nullable().optional().meta({description: "Set by lineage events"}), "unit_designator": z.string().nullable().optional().meta({description: "Condo sub-parcel discriminator"}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "Parcel"});

export type Parcel = z.infer<typeof ParcelSchema>;

/**
 * Namespaced external parcel IDs — assessor account numbers, alternate APNs, Universal Parcel Identifier (UPI) URNs (scheme upi, value the full urn verbatim including any :sub: subcomponent). Mirrors PropertyIdentifier for identifiers scoped to a parcel rather than the property.
 */
export const ParcelIdentifierSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "namespace": z.string().nullable().optional(), "parcel": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "scheme": z.string(), "value": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Namespaced external parcel IDs — assessor account numbers, alternate APNs, Universal Parcel Identifier (UPI) URNs (scheme upi, value the full urn verbatim including any :sub: subcomponent). Mirrors PropertyIdentifier for identifiers scoped to a parcel rather than the property.", title: "ParcelIdentifier"});

export type ParcelIdentifier = z.infer<typeof ParcelIdentifierSchema>;

/**
 */
export const ParcelLineageTypeSchema = z.intersection(z.enum(["split", "merge", "renumber"]), z.string()).meta({description: "", title: "ParcelLineageType"});

export type ParcelLineageType = z.infer<typeof ParcelLineageTypeSchema>;

/**
 */
export const ParcelLineageSchema = z.strictObject({"effective_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": ParcelLineageTypeSchema, "predecessor_parcel": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "successor_parcel": z.string(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "ParcelLineage"});

export type ParcelLineage = z.infer<typeof ParcelLineageSchema>;

/**
 * An open-vocabulary classification qualified by the system that defines its code. Codes from different systems are not assumed equivalent.
 */
export const ClassificationSchema = z.strictObject({"code": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "display": z.string().nullable().optional(), "system": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])"))}).meta({description: "An open-vocabulary classification qualified by the system that defines its code. Codes from different systems are not assumed equivalent.", title: "Classification"});

export type Classification = z.infer<typeof ClassificationSchema>;

/**
 */
export const PartyAddressSchema = z.strictObject({"address": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "extras": z.union([AnySchema, z.null()]).optional(), "is_primary": z.boolean().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "role": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "situs | entrance | alias | address_range | former | mailing | other (open vocabulary)"}), "valid_from": z.iso.date().nullable().optional(), "valid_to": z.iso.date().nullable().optional()}).meta({description: "", title: "PartyAddress"});

export type PartyAddress = z.infer<typeof PartyAddressSchema>;

/**
 */
export const PartyContactSchema = z.strictObject({"do_not_contact": z.boolean().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "is_primary": z.boolean().nullable().optional(), "kind": z.string().meta({description: "phone | email | website | other"}), "label": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "value": z.string()}).meta({description: "", title: "PartyContact"});

export type PartyContact = z.infer<typeof PartyContactSchema>;

/**
 */
export const PartyTypeSchema = z.intersection(z.enum(["person", "organization"]), z.string()).meta({description: "", title: "PartyType"});

export type PartyType = z.infer<typeof PartyTypeSchema>;

/**
 * One canonical model for every actor — owners, buyers, tenants and lessees, borrowers, lenders, brokers, trustees, claimants, contractors, associations, and valuation performers.
 */
export const PartySchema = z.strictObject({"addresses": z.array(PartyAddressSchema).nullable().optional(), "classifications": z.array(ClassificationSchema).nullable().optional().meta({description: "System-qualified actor classifications from an open vocabulary; use producer-namespaced system and code values"}), "contacts": z.array(PartyContactSchema).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": PartyTypeSchema, "legal_form": z.union([ClassificationSchema, z.null()]).optional().meta({description: "Optional system-qualified legal form for an organization under an identified jurisdictional or producer vocabulary. This is not an industry classification or a contextual role such as lender, broker, tenant, or association."}), "name": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical display name for this Party in the profile; nonblank with no leading or trailing whitespace. Source-specific wording is attributed through provenance or SourceArtifact."}), "name_first": z.string().nullable().optional().meta({description: "Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems."}), "name_last": z.string().nullable().optional().meta({description: "Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems."}), "name_middle": z.string().nullable().optional().meta({description: "Optional producer-derived parsed component of a person's canonical display name; not an independent identity and not universally applicable across naming systems."}), "normalized_name": z.string().nullable().optional().meta({description: "Producer-derived matching key for the canonical display name. Its normalization algorithm is producer-defined; it is not authoritative display text or a separate identity."}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "One canonical model for every actor — owners, buyers, tenants and lessees, borrowers, lenders, brokers, trustees, claimants, contractors, associations, and valuation performers.", title: "Party"});

export type Party = z.infer<typeof PartySchema>;

/**
 * Building permits — header + lifecycle dates. Inspections/projects are extension.
 */
export const PermitSchema = z.strictObject({"applied_date": z.iso.date().nullable().optional(), "artifacts": z.array(z.string()).nullable().optional(), "contractor_party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Canonical contractor Party reference; credential records are outside core v0.2"}), "description": z.string().nullable().optional(), "expiration_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "fees": z.union([MoneySchema, z.null()]).optional(), "finaled_date": z.iso.date().nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "issued_date": z.iso.date().nullable().optional(), "job_value": z.union([MoneySchema, z.null()]).optional(), "kind": z.string().nullable().optional().meta({description: "roofing | solar | hvac | adu | pool | new_construction | ... (open)"}), "permit_number": z.string().nullable().optional(), "permitting_jurisdiction": z.string().nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "status": z.string().nullable().optional().meta({description: "issued | finaled | expired | ... (open)"}), "structure": z.string().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Building permits — header + lifecycle dates. Inspections/projects are extension.", title: "Permit"});

export type Permit = z.infer<typeof PermitSchema>;

/**
 */
export const EstateTypeSchema = z.intersection(z.enum(["fee_simple", "leased_fee", "leasehold", "life_estate", "cooperative_shares", "other"]), z.string()).meta({description: "", title: "EstateType"});

export type EstateType = z.infer<typeof EstateTypeSchema>;

/**
 * Stable property identity; mutable descriptive fields come from PropertyFacts.
 */
export const PropertySchema = z.strictObject({"building_count": z.number().int().nullable().optional().meta({description: "Producer summary when structures are not enumerated"}), "estate_type": EstateTypeSchema.optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "location": z.union([GeoPointSchema, z.null()]).optional(), "name": z.string().nullable().optional(), "property_use_class": z.string().nullable().optional().meta({description: "PUCS class when property_use_system identifies PUCS"}), "property_use_subtype": z.string().nullable().optional(), "property_use_system": z.string().nullable().optional().meta({description: "Required when use fields are set; no default"}), "property_use_type": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Stable property identity; mutable descriptive fields come from PropertyFacts.", title: "Property"});

export type Property = z.infer<typeof PropertySchema>;

/**
 */
export const PropertyAddressSchema = z.strictObject({"address": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_primary": z.boolean().nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "role": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "situs | entrance | alias | address_range | former | mailing | other (open vocabulary)"}), "valid_from": z.iso.date().nullable().optional(), "valid_to": z.iso.date().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "PropertyAddress"});

export type PropertyAddress = z.infer<typeof PropertyAddressSchema>;

/**
 * HOA or property-association relationship; identity and classification live on the canonical Party.
 */
export const PropertyAssociationSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "fee": z.union([MoneySchema, z.null()]).optional(), "fee_period": RentPeriodSchema.optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "The canonical association organization"}), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "HOA or property-association relationship; identity and classification live on the canonical Party.", title: "PropertyAssociation"});

export type PropertyAssociation = z.infer<typeof PropertyAssociationSchema>;

/**
 * Namespaced external IDs — MLS keys, vendor IDs, tax account numbers.
 */
export const PropertyIdentifierSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "namespace": z.string().nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "scheme": z.string(), "value": z.string(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Namespaced external IDs — MLS keys, vendor IDs, tax account numbers.", title: "PropertyIdentifier"});

export type PropertyIdentifier = z.infer<typeof PropertyIdentifierSchema>;

/**
 * Property ↔ parcel, many-to-many over time (splits/merges/condos). end_date is end-exclusive.
 */
export const PropertyParcelSchema = z.strictObject({"end_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_primary": z.boolean().nullable().optional(), "parcel": z.string(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "start_date": z.iso.date().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Property ↔ parcel, many-to-many over time (splits/merges/condos). end_date is end-exclusive.", title: "PropertyParcel"});

export type PropertyParcel = z.infer<typeof PropertyParcelSchema>;

/**
 */
export const PropertyStateSchema = z.strictObject({"building_count": z.number().int().nullable().optional().meta({description: "Producer summary when structures are not enumerated"}), "estate_type": EstateTypeSchema.optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "location": z.union([GeoPointSchema, z.null()]).optional(), "name": z.string().nullable().optional(), "property_use_class": z.string().nullable().optional().meta({description: "PUCS class when property_use_system identifies PUCS"}), "property_use_subtype": z.string().nullable().optional(), "property_use_system": z.string().nullable().optional().meta({description: "Required when use fields are set; no default"}), "property_use_type": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "subject": z.string(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "PropertyState"});

export type PropertyState = z.infer<typeof PropertyStateSchema>;

/**
 */
export const LengthUnitSchema = z.intersection(z.enum(["ft", "m"]), z.string()).meta({description: "", title: "LengthUnit"});

export type LengthUnit = z.infer<typeof LengthUnitSchema>;

/**
 * Linear measurement with explicit unit.
 */
export const LengthSchema = z.strictObject({"unit": LengthUnitSchema, "value": z.number()}).meta({description: "Linear measurement with explicit unit.", title: "Length"});

export type Length = z.infer<typeof LengthSchema>;

/**
 */
export const SiteStateSchema = z.strictObject({"block": z.string().nullable().optional(), "buildable_units": z.number().int().nullable().optional(), "depth": z.union([LengthSchema, z.null()]).optional(), "easements": z.array(z.string()).nullable().optional(), "entitlement_status": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "flood_zone": z.string().nullable().optional(), "frontage": z.union([LengthSchema, z.null()]).optional(), "hazard_zones": z.array(z.string()).nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_corner": z.boolean().nullable().optional(), "land_use": z.union([CodeableConceptSchema, z.null()]).optional(), "land_use_category": z.string().nullable().optional(), "lot_number": z.string().nullable().optional(), "lot_size": z.union([AreaSchema, z.null()]).optional(), "phase_number": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "restrictions": z.array(z.string()).nullable().optional(), "section_township_range": z.string().nullable().optional(), "site_influences": z.array(z.string()).nullable().optional(), "subdivision": z.string().nullable().optional(), "subject": z.string(), "topography": z.string().nullable().optional(), "tract_number": z.string().nullable().optional(), "usable_land_area": z.union([AreaSchema, z.null()]).optional(), "usable_land_area_basis": z.string().nullable().optional(), "utilities": z.array(z.string()).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional(), "view_types": z.array(z.string()).nullable().optional(), "zoning_code": z.string().nullable().optional()}).meta({description: "", title: "SiteState"});

export type SiteState = z.infer<typeof SiteStateSchema>;

/**
 */
export const SpaceStateSchema = z.strictObject({"bathrooms": z.number().nullable().optional(), "bedrooms": z.number().int().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "floor_number": z.number().int().nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_active": z.boolean().nullable().optional(), "is_adu": z.boolean().nullable().optional(), "occupancy": z.string().nullable().optional().meta({description: "owner_occupied | tenant | vacant"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rentable_area": z.union([AreaSchema, z.null()]).optional(), "space_use": z.string().nullable().optional(), "subject": z.string(), "usable_area": z.union([AreaSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "SpaceState"});

export type SpaceState = z.infer<typeof SpaceStateSchema>;

/**
 */
export const RatingScopeSchema = z.intersection(z.enum(["overall", "exterior", "interior", "component", "other"]), z.string()).meta({description: "", title: "RatingScope"});

export type RatingScope = z.infer<typeof RatingScopeSchema>;

/**
 * A rating qualified by the system that defines its code. PHDS does not assume codes from different systems are ordinally or semantically equal.
 */
export const RatingSchema = z.strictObject({"code": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "description": z.string().nullable().optional(), "display": z.string().nullable().optional(), "scope": RatingScopeSchema.optional(), "system": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])"))}).meta({description: "A rating qualified by the system that defines its code. PHDS does not assume codes from different systems are ordinally or semantically equal.", title: "Rating"});

export type Rating = z.infer<typeof RatingSchema>;

/**
 * Internationally neutral commercial facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Historical/per-period financial figures belong on dated records such as OperatingStatement and UnitRentObservation. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
 */
export const CommercialDetailsSchema = z.strictObject({"clear_height": z.union([LengthSchema, z.null()]).optional(), "dock_doors": z.number().int().nullable().optional(), "drive_in_doors": z.number().int().nullable().optional(), "elevators": z.number().int().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "has_sprinkler": z.boolean().nullable().optional(), "market_classification": z.union([RatingSchema, z.null()]).optional().meta({description: "Competitive market positioning under the named rating system; distinct from physical condition and construction quality."}), "occupancy_pct": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 95 means 95 percent."}), "parking_ratio": z.union([UnitRateSchema, z.null()]).optional().meta({description: "e.g. spaces per 1000 sqft — denominator explicit"}), "parking_spaces": z.number().int().nullable().optional(), "parking_types": z.array(z.string()).nullable().optional().meta({description: "surface | structured | underground | covered | ..."}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "submarket": z.string().nullable().optional().meta({description: "CRE market geography"}), "tenancy": z.string().nullable().optional().meta({description: "single_tenant | multi_tenant"}), "tenant_count": z.number().int().nullable().optional().meta({description: "Distinct legal tenants (not suites or leases); timing follows the containing Structure or StructureState context"})}).meta({description: "Internationally neutral commercial facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Historical/per-period financial figures belong on dated records such as OperatingStatement and UnitRentObservation. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.", title: "CommercialDetails"});

export type CommercialDetails = z.infer<typeof CommercialDetailsSchema>;

/**
 * Repeatable events — never a flat year on the structure.
 */
export const RenovationSchema = z.strictObject({"completed_date": z.iso.date().nullable().optional(), "completed_year": z.number().int().min(1).max(2200).nullable().optional(), "cost": z.union([MoneySchema, z.null()]).optional(), "description": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "kind": z.string().nullable().optional().meta({description: "kitchen | bath | roof | addition | gut | ... (open)"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional()}).meta({description: "Repeatable events — never a flat year on the structure.", title: "Renovation"});

export type Renovation = z.infer<typeof RenovationSchema>;

/**
 * Internationally neutral residential facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.
 */
export const ResidentialDetailsSchema = z.strictObject({"adu_legally_rentable": z.boolean().nullable().optional(), "architectural_design": z.string().nullable().optional().meta({description: "ranch | cape_cod | colonial | craftsman | ..."}), "attachment": z.string().nullable().optional().meta({description: "detached | attached | semi_detached | rowhouse_townhouse | ..."}), "basement_type": z.string().nullable().optional().meta({description: "none | full | partial | walkout | cellar"}), "bathrooms_full": z.number().int().nullable().optional(), "bathrooms_half": z.number().int().nullable().optional(), "bathrooms_three_quarter": z.number().int().nullable().optional(), "bedrooms_total": z.number().int().min(0).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "fireplaces": z.number().int().nullable().optional(), "garage_attachment": z.string().nullable().optional().meta({description: "attached | detached | built_in"}), "garage_type": z.string().nullable().optional().meta({description: "garage | carport | parking_garage | driveway | none | ..."}), "has_adu": z.boolean().nullable().optional(), "has_attic": z.boolean().nullable().optional(), "has_pool": z.boolean().nullable().optional(), "occupancy": z.string().nullable().optional().meta({description: "owner_occupied | tenant | vacant"}), "parking_spaces": z.number().int().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "renewable_energy_components": z.array(z.string()).nullable().optional().meta({description: "solar | geothermal | wind"}), "rooms_total": z.number().int().nullable().optional()}).meta({description: "Internationally neutral residential facts. When nested in Structure, these fields describe observation-derived current state. When nested in StructureState, they are asserted historical facts governed by PropertyStateSnapshot.as_of_date. provenance.retrieved_at is retrieval metadata, not the effective date. Events reference a separately bundled PropertyStateSnapshot rather than containing the snapshot. Optional standards profiles may constrain values for a specific use case.", title: "ResidentialDetails"});

export type ResidentialDetails = z.infer<typeof ResidentialDetailsSchema>;

/**
 */
export const StructureStateSchema = z.strictObject({"areas": z.array(AreaMeasureSchema).nullable().optional().meta({description: "Long-tail area kinds; canonical kinds forbidden here (validator-enforced)"}), "basement_area": z.union([AreaSchema, z.null()]).optional(), "basement_finished_area": z.union([AreaSchema, z.null()]).optional(), "commercial": z.union([CommercialDetailsSchema, z.null()]).optional(), "condition_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair."}), "construction_method": z.string().nullable().optional().meta({description: "site_built | manufactured | modular | container | 3d_printed | ..."}), "construction_status": z.string().nullable().optional().meta({description: "complete | proposed | under_construction"}), "construction_type": z.string().nullable().optional(), "cooling_types": z.array(z.string()).nullable().optional(), "effective_year_built": z.number().int().nullable().optional(), "exterior_wall_type": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "features": z.array(z.string()).nullable().optional().meta({description: "Open vocabulary for physical features and amenities"}), "foundation_material": z.string().nullable().optional(), "foundation_type": z.string().nullable().optional(), "garage_area": z.union([AreaSchema, z.null()]).optional(), "gross_area": z.union([AreaSchema, z.null()]).optional(), "ground_floor_area": z.union([AreaSchema, z.null()]).optional(), "heating_fuel_type": z.string().nullable().optional(), "heating_types": z.array(z.string()).nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": z.string().nullable().optional().meta({description: "building | barn | silo | shed | outbuilding | ... (open)"}), "living_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance."}), "name": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "quality_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair."}), "renovations": z.array(RenovationSchema).nullable().optional(), "rentable_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method."}), "residential": z.union([ResidentialDetailsSchema, z.null()]).optional(), "roof_material_type": z.string().nullable().optional(), "roof_style_type": z.string().nullable().optional(), "sewer_type": z.string().nullable().optional(), "stories": z.number().nullable().optional(), "structure_number": z.string().nullable().optional(), "subject": z.string(), "unit_count": z.number().int().min(1).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional(), "water_type": z.string().nullable().optional(), "year_built": z.number().int().min(1).max(2200).nullable().optional(), "year_built_estimated": z.boolean().nullable().optional()}).meta({description: "", title: "StructureState"});

export type StructureState = z.infer<typeof StructureStateSchema>;

/**
 * Sparse asserted physical state effective on as_of_date.
 */
export const PropertyStateSnapshotSchema = z.strictObject({"as_of_date": z.iso.date(), "basis": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "at_sale | at_listing | at_lease | inspection | reported | inferred (open vocabulary)"}), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "property": z.string(), "property_state": z.union([PropertyStateSchema, z.null()]).optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "site_states": z.array(SiteStateSchema).nullable().optional(), "space_states": z.array(SpaceStateSchema).nullable().optional(), "structure_states": z.array(StructureStateSchema).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Sparse asserted physical state effective on as_of_date.", title: "PropertyStateSnapshot"});

export type PropertyStateSnapshot = z.infer<typeof PropertyStateSnapshotSchema>;

/**
 * Supporting rent-roll detail. space resolves to canonical Space. tenant is the canonical legal lessee Party when present, and lease resolves to canonical LeaseEvent when known. When both tenant and lease are present, tenant must match a party with role: lessee declared by that lease, if the lease declares any lessee. References remain optional for aggregate, vacant, unleased, or unresolved lines; the line does not duplicate canonical space identity, tenant names, or lease dates. When a source tenant cannot be resolved to a canonical Party, omit tenant and preserve the source evidence through the RentRoll provenance and profile-level SourceArtifact records; do not mint a placeholder Party or copy the source name into extras.
 */
export const RentRollLineSchema = z.strictObject({"contract_rent": z.union([MoneySchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "lease": z.string().nullable().optional(), "market_rent": z.union([MoneySchema, z.null()]).optional(), "occupancy_status": z.union([CodeableConceptSchema, z.null()]).optional(), "reported_area": z.union([AreaSchema, z.null()]).optional(), "space": z.string().nullable().optional(), "tenant": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional()}).meta({description: "Supporting rent-roll detail. space resolves to canonical Space. tenant is the canonical legal lessee Party when present, and lease resolves to canonical LeaseEvent when known. When both tenant and lease are present, tenant must match a party with role: lessee declared by that lease, if the lease declares any lessee. References remain optional for aggregate, vacant, unleased, or unresolved lines; the line does not duplicate canonical space identity, tenant names, or lease dates. When a source tenant cannot be resolved to a canonical Party, omit tenant and preserve the source evidence through the RentRoll provenance and profile-level SourceArtifact records; do not mint a placeholder Party or copy the source name into extras.", title: "RentRollLine"});

export type RentRollLine = z.infer<typeof RentRollLineSchema>;

/**
 * Dated rent and occupancy observation applicable to any property use. Header totals are authoritative reported values; lines are supporting detail and are not required to sum to the totals. All Money values on one rent roll MUST use one currency (validator-enforced). For its as_of_date, this record governs reported occupancy and rent facts; current-state fields such as Space.occupancy and CommercialDetails.occupancy_pct do not override it. A line preserves dated rent-roll assertions and does not by itself create a canonical Space, Party, or LeaseEvent.
 */
export const RentRollSchema = z.strictObject({"as_of_date": z.iso.date(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "lines": z.array(RentRollLineSchema).nullable().optional(), "occupancy_pct": z.number().min(0).max(100).nullable().optional().meta({description: "Occupancy in 0–100 percentage points; 95 means 95 percent."}), "occupied_unit_count": z.number().int().min(0).nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rent_period": RentPeriodSchema.optional(), "total_contract_rent": z.union([MoneySchema, z.null()]).optional(), "total_market_rent": z.union([MoneySchema, z.null()]).optional(), "unit_count": z.number().int().min(0).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Dated rent and occupancy observation applicable to any property use. Header totals are authoritative reported values; lines are supporting detail and are not required to sum to the totals. All Money values on one rent roll MUST use one currency (validator-enforced). For its as_of_date, this record governs reported occupancy and rent facts; current-state fields such as Space.occupancy and CommercialDetails.occupancy_pct do not override it. A line preserves dated rent-roll assertions and does not by itself create a canonical Space, Party, or LeaseEvent.", title: "RentRoll"});

export type RentRoll = z.infer<typeof RentRollSchema>;

/**
 * Reliability of a recorded price
 */
export const PriceDisclosureSchema = z.intersection(z.enum(["full", "partial", "estimated", "nominal", "non_disclosure", "unknown"]), z.string()).meta({description: "Reliability of a recorded price", title: "PriceDisclosure"});

export type PriceDisclosure = z.infer<typeof PriceDisclosureSchema>;

/**
 * role = buyer | seller | buyer_broker | seller_broker
 */
export const SaleEventPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = buyer | seller | buyer_broker | seller_broker", title: "SaleEventParty"});

export type SaleEventParty = z.infer<typeof SaleEventPartySchema>;

/**
 * Sale-to-listing relation. Vocabulary stays sale-oriented; relist chains are listing-to-listing facts and "this MLS reported the sale" is a SaleEvidence row with listing set — neither belongs here.
 */
export const SaleListingRelationshipSchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "listing": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "relationship_type": z.string().meta({description: "resulted_in_sale | prior_listing | other (open)"})}).meta({description: "Sale-to-listing relation. Vocabulary stays sale-oriented; relist chains are listing-to-listing facts and \"this MLS reported the sale\" is a SaleEvidence row with listing set — neither belongs here.", title: "SaleListingRelationship"});

export type SaleListingRelationship = z.infer<typeof SaleListingRelationshipSchema>;

/**
 */
export const SaleTypeEnumSchema = z.intersection(z.enum(["arms_length", "reo", "short_sale", "auction", "related_party", "portfolio", "partial_interest", "land_contract", "new_construction", "other"]), z.string()).meta({description: "", title: "SaleTypeEnum"});

export type SaleTypeEnum = z.infer<typeof SaleTypeEnumSchema>;

/**
 * A market sale referencing its recorded transfer. A quitclaim never becomes a comp.
 */
export const SaleEventSchema = z.strictObject({"buyer_financing": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "Financing the purchaser used to close"}), "cap_rate": z.number().min(0).max(100).nullable().optional().meta({description: "Capitalization rate in percentage points; 5.75 means 5.75 percent."}), "close_date": z.iso.date().meta({description: "Closing date of the market sale (reconciled layer)"}), "concessions": z.array(CodeableConceptSchema).nullable().optional(), "concessions_amount": z.union([MoneySchema, z.null()]).optional(), "concessions_comments": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "listings": z.array(SaleListingRelationshipSchema).nullable().optional().meta({description: "Absent/empty for off-market sales"}), "noi_at_sale": z.union([MoneySchema, z.null()]).optional(), "occupancy_at_sale_pct": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 90 means 90 percent."}), "opex_at_sale": z.union([MoneySchema, z.null()]).optional(), "parties": z.array(SaleEventPartySchema).nullable().optional(), "price_code": z.union([CodeableConceptSchema, z.null()]).optional(), "price_disclosure": PriceDisclosureSchema.optional(), "price_per_area": z.union([UnitRateSchema, z.null()]).optional(), "price_per_unit": z.union([MoneySchema, z.null()]).optional(), "property": z.string(), "property_state": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "remarks": z.string().nullable().optional().meta({description: "Source- or vendor-authored narrative interpreted through provenance."}), "sale_price": z.union([MoneySchema, z.null()]).optional().meta({description: "Reconciled market-sale amount — NOT the raw MLS ClosePrice claim (that is SaleEvidence.close_price)"}), "sale_type": SaleTypeEnumSchema.optional(), "supporting_operating_statement": z.string().nullable().optional().meta({description: "Traceability: the statement noi_at_sale derives from, when known"}), "transfer": z.string().nullable().optional(), "unit_count_at_sale": z.number().int().min(1).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "A market sale referencing its recorded transfer. A quitclaim never becomes a comp.", title: "SaleEvent"});

export type SaleEvent = z.infer<typeof SaleEventSchema>;

/**
 * One source's claims about a market sale. SaleEvent holds the reconciliation; evidence rows hold the disagreeing inputs (MLS close record, deed, assessor, broker or appraiser verification). One row = one source = one provenance. When an MLS-sourced row links a listing, its close_date/close_price should equal that listing's closed-event values (convention, deliberately unenforced).
 */
export const SaleEvidenceSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "close_date": z.iso.date().nullable().optional(), "close_price": z.union([MoneySchema, z.null()]).optional(), "concessions_amount": z.union([MoneySchema, z.null()]).optional(), "document_number": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "listing": z.string().nullable().optional(), "provenance": ProvenanceSchema, "remarks": z.string().nullable().optional(), "sale": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "transfer": z.string().nullable().optional(), "verification_method": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "mls_record | deed | assessor | broker_confirmed | appraiser_verified | ... (open)"}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "One source's claims about a market sale. SaleEvent holds the reconciliation; evidence rows hold the disagreeing inputs (MLS close record, deed, assessor, broker or appraiser verification). One row = one source = one provenance. When an MLS-sourced row links a listing, its close_date/close_price should equal that listing's closed-event values (convention, deliberately unenforced).", title: "SaleEvidence"});

export type SaleEvidence = z.infer<typeof SaleEvidenceSchema>;

/**
 */
export const SiteSchema = z.strictObject({"block": z.string().nullable().optional(), "buildable_units": z.number().int().nullable().optional(), "depth": z.union([LengthSchema, z.null()]).optional(), "easements": z.array(z.string()).nullable().optional(), "entitlement_status": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "flood_zone": z.string().nullable().optional(), "frontage": z.union([LengthSchema, z.null()]).optional(), "hazard_zones": z.array(z.string()).nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_corner": z.boolean().nullable().optional(), "land_use": z.union([CodeableConceptSchema, z.null()]).optional(), "land_use_category": z.string().nullable().optional(), "lot_number": z.string().nullable().optional(), "lot_size": z.union([AreaSchema, z.null()]).optional(), "phase_number": z.string().nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "restrictions": z.array(z.string()).nullable().optional(), "section_township_range": z.string().nullable().optional(), "site_influences": z.array(z.string()).nullable().optional(), "subdivision": z.string().nullable().optional(), "topography": z.string().nullable().optional(), "tract_number": z.string().nullable().optional(), "usable_land_area": z.union([AreaSchema, z.null()]).optional(), "usable_land_area_basis": z.string().nullable().optional(), "utilities": z.array(z.string()).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional(), "view_types": z.array(z.string()).nullable().optional(), "zoning_code": z.string().nullable().optional()}).meta({description: "", title: "Site"});

export type Site = z.infer<typeof SiteSchema>;

/**
 * Source material that supports or preserves an assertion. Semantic validation requires at least one nonblank uri or storage_reference.
 */
export const SourceArtifactSchema = z.strictObject({"captured_at": rfc3339DateTimeSchema.nullable().optional(), "content_hash": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "hash_scheme": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Producer-namespaced content hashing scheme"}), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": z.union([CodeableConceptSchema, z.null()]).optional(), "media_type": z.string().nullable().optional().meta({description: "MIME media type"}), "order": z.number().int().min(0).nullable().optional().meta({description: "Presentation order within the owning record's artifact list"}), "original_filename": z.string().nullable().optional(), "page_count": z.number().int().min(1).nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "short_description": z.string().nullable().optional(), "source_modified_at": rfc3339DateTimeSchema.nullable().optional().meta({description: "When the source system last modified this media/document record"}), "storage_reference": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Producer-defined object or document storage reference"}), "title": z.string().nullable().optional(), "uri": rfc3986UriSchema.regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Source material that supports or preserves an assertion. Semantic validation requires at least one nonblank uri or storage_reference.", title: "SourceArtifact"});

export type SourceArtifact = z.infer<typeof SourceArtifactSchema>;

/**
 * Leasable suites/units (CRE, multifamily).
 */
export const SpaceSchema = z.strictObject({"bathrooms": z.number().nullable().optional(), "bedrooms": z.number().int().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "floor_number": z.number().int().nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "is_active": z.boolean().nullable().optional(), "is_adu": z.boolean().nullable().optional(), "occupancy": z.string().nullable().optional().meta({description: "owner_occupied | tenant | vacant"}), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rentable_area": z.union([AreaSchema, z.null()]).optional(), "space_identifier": z.string(), "space_use": z.string().nullable().optional(), "structure": z.string().nullable().optional(), "usable_area": z.union([AreaSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Leasable suites/units (CRE, multifamily).", title: "Space"});

export type Space = z.infer<typeof SpaceSchema>;

/**
 */
export const StructureSchema = z.strictObject({"areas": z.array(AreaMeasureSchema).nullable().optional().meta({description: "Long-tail area kinds; canonical kinds forbidden here (validator-enforced)"}), "basement_area": z.union([AreaSchema, z.null()]).optional(), "basement_finished_area": z.union([AreaSchema, z.null()]).optional(), "commercial": z.union([CommercialDetailsSchema, z.null()]).optional(), "condition_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair."}), "construction_method": z.string().nullable().optional().meta({description: "site_built | manufactured | modular | container | 3d_printed | ..."}), "construction_status": z.string().nullable().optional().meta({description: "complete | proposed | under_construction"}), "construction_type": z.string().nullable().optional(), "cooling_types": z.array(z.string()).nullable().optional(), "effective_year_built": z.number().int().nullable().optional(), "exterior_wall_type": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "features": z.array(z.string()).nullable().optional().meta({description: "Open vocabulary for physical features and amenities"}), "foundation_material": z.string().nullable().optional(), "foundation_type": z.string().nullable().optional(), "garage_area": z.union([AreaSchema, z.null()]).optional(), "gross_area": z.union([AreaSchema, z.null()]).optional(), "ground_floor_area": z.union([AreaSchema, z.null()]).optional(), "heating_fuel_type": z.string().nullable().optional(), "heating_types": z.array(z.string()).nullable().optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "kind": z.string().nullable().optional().meta({description: "building | barn | silo | shed | outbuilding | ... (open)"}), "living_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance."}), "name": z.string().nullable().optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "quality_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair."}), "renovations": z.array(RenovationSchema).nullable().optional(), "rentable_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method."}), "residential": z.union([ResidentialDetailsSchema, z.null()]).optional(), "roof_material_type": z.string().nullable().optional(), "roof_style_type": z.string().nullable().optional(), "sewer_type": z.string().nullable().optional(), "stories": z.number().nullable().optional(), "structure_number": z.string().nullable().optional(), "unit_count": z.number().int().min(1).nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional(), "water_type": z.string().nullable().optional(), "year_built": z.number().int().min(1).max(2200).nullable().optional(), "year_built_estimated": z.boolean().nullable().optional()}).meta({description: "", title: "Structure"});

export type Structure = z.infer<typeof StructureSchema>;

/**
 */
export const TaxInstallmentSchema = z.strictObject({"amount": z.union([MoneySchema, z.null()]).optional(), "amount_paid": z.union([MoneySchema, z.null()]).optional(), "due_date": z.iso.date().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "installment_number": z.number().int().nullable().optional(), "is_delinquent": z.boolean().nullable().optional(), "paid_date": z.iso.date().nullable().optional()}).meta({description: "", title: "TaxInstallment"});

export type TaxInstallment = z.infer<typeof TaxInstallmentSchema>;

/**
 */
export const TaxLineItemSchema = z.strictObject({"amount": z.union([MoneySchema, z.null()]).optional(), "extras": z.union([AnySchema, z.null()]).optional(), "jurisdiction": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Optional canonical taxing jurisdiction responsible for this line item"}), "rate": z.number().nullable().optional().meta({description: "Source-defined tax rate or levy value; not governed by the _pct percentage-points convention."})}).meta({description: "", title: "TaxLineItem"});

export type TaxLineItem = z.infer<typeof TaxLineItemSchema>;

/**
 */
export const TaxBillSchema = z.strictObject({"amount_billed": z.union([MoneySchema, z.null()]).optional(), "amount_paid": z.union([MoneySchema, z.null()]).optional(), "bill_number": z.string().nullable().optional(), "delinquent_amount": z.union([MoneySchema, z.null()]).optional(), "delinquent_year": z.number().int().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "installments": z.array(TaxInstallmentSchema).nullable().optional(), "is_delinquent": z.boolean().nullable().optional(), "jurisdiction": z.string().meta({description: "Issuing authority"}), "line_items": z.array(TaxLineItemSchema).nullable().optional(), "parcel": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rate_code_area": z.string().nullable().optional(), "tax_year": z.number().int(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "TaxBill"});

export type TaxBill = z.infer<typeof TaxBillSchema>;

/**
 * role = grantor | grantee
 */
export const TransferPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "role = grantor | grantee", title: "TransferParty"});

export type TransferParty = z.infer<typeof TransferPartySchema>;

/**
 * Every recorded deed/instrument — the ownership ledger, NOT comps.
 */
export const TransferSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "consideration": z.union([MoneySchema, z.null()]).optional().meta({description: "Often $0 / nominal"}), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "effective_date": z.iso.date().nullable().optional().meta({description: "Legal/economic effectiveness — may differ from instrument_date and recorded_date"}), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "interest_conveyed": z.union([CodeableConceptSchema, z.null()]).optional().meta({description: "The legal or beneficial interest conveyed by this transfer."}), "is_distressed": z.boolean().nullable().optional(), "is_inter_family": z.boolean().nullable().optional(), "parcel": z.string().nullable().optional(), "partial_interest_pct": z.number().min(0).max(100).nullable().optional().meta({description: "0–100 percentage points; 25 means 25 percent."}), "parties": z.array(TransferPartySchema).nullable().optional(), "price_code": z.union([CodeableConceptSchema, z.null()]).optional(), "price_disclosure": PriceDisclosureSchema.optional(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional(), "transfer_tax": z.union([MoneySchema, z.null()]).optional().meta({description: "Doc stamps; price-inference basis in many places"}), "transfer_type": z.string().meta({description: "warranty_deed | quitclaim | foreclosure | tax_deed | ... (open)"}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Every recorded deed/instrument — the ownership ledger, NOT comps.", title: "Transfer"});

export type Transfer = z.infer<typeof TransferSchema>;

/**
 */
export const RateBasisSchema = z.intersection(z.enum(["per_unit", "per_bed", "per_area", "per_room", "per_key", "per_slip", "per_stall", "per_pad", "other"]), z.string()).meta({description: "", title: "RateBasis"});

export type RateBasis = z.infer<typeof RateBasisSchema>;

/**
 */
export const RateTypeSchema = z.intersection(z.enum(["asking", "market", "effective", "contract"]), z.string()).meta({description: "", title: "RateType"});

export type RateType = z.infer<typeof RateTypeSchema>;

/**
 * Advertised/going rates — floorplans, storage units, slips. NOT executed leases.
 */
export const UnitRentObservationSchema = z.strictObject({"bathrooms": z.number().nullable().optional(), "bedrooms": z.number().int().nullable().optional(), "concessions_note": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "observed_date": z.iso.date(), "property": z.string(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rate": MoneySchema, "rate_basis": RateBasisSchema.optional(), "rate_period": RentPeriodSchema.optional(), "rate_type": RateTypeSchema.optional(), "unit_area": z.union([AreaSchema, z.null()]).optional(), "unit_count": z.number().int().nullable().optional().meta({description: "Units of this type"}), "unit_type": z.string().meta({description: "'1BR/1BA', '10x10'"}), "units_available": z.number().int().nullable().optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Advertised/going rates — floorplans, storage units, slips. NOT executed leases.", title: "UnitRentObservation"});

export type UnitRentObservation = z.infer<typeof UnitRentObservationSchema>;

/**
 */
export const ValuationTypeSchema = z.intersection(z.enum(["avm", "appraisal", "bpo", "broker_opinion", "internal"]), z.string()).meta({description: "", title: "ValuationType"});

export type ValuationType = z.infer<typeof ValuationTypeSchema>;

/**
 * Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.
 */
export const ValuationSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "as_of_date": z.iso.date(), "confidence_score": z.number().int().min(0).max(100).nullable().optional().meta({description: "Source-defined confidence score; not governed by the _pct percentage-points convention."}), "exposure_days": z.number().int().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "forecast_standard_deviation": z.number().nullable().optional().meta({description: "Source-defined AVM forecast standard deviation; not governed by the _pct percentage-points convention."}), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "indicated_value_cost": z.union([MoneySchema, z.null()]).optional(), "indicated_value_income": z.union([MoneySchema, z.null()]).optional(), "indicated_value_sales_comparison": z.union([MoneySchema, z.null()]).optional(), "interest": z.string().nullable().optional().meta({description: "Interest valued for this opinion; independent of Property.estate_type and Transfer.interest_conveyed."}), "kind": ValuationTypeSchema, "land_value": z.union([MoneySchema, z.null()]).optional().meta({description: "Cost-approach site value"}), "performed_by_party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).nullable().optional().meta({description: "Canonical person or organization that performed the valuation"}), "property": z.string(), "property_state": z.string().nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "report_date": z.iso.date().nullable().optional(), "valuation_method": z.string().nullable().optional().meta({description: "desktop | exterior | hybrid | traditional"}), "value": MoneySchema, "value_high": z.union([MoneySchema, z.null()]).optional(), "value_low": z.union([MoneySchema, z.null()]).optional(), "value_per_area": z.union([UnitRateSchema, z.null()]).optional(), "value_premise": z.string().nullable().optional().meta({description: "as_is | as_completed | as_stabilized"}), "value_type": z.string().nullable().optional().meta({description: "market_value | liquidation | insurable | land | ... (open)"}), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "Opinions of value — AVM/appraisal/BPO. Never tax-roll values (those are assessments). Append-only.", title: "Valuation"});

export type Valuation = z.infer<typeof ValuationSchema>;

/**
 * The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
 */
export const PropertyProfileSchema = z.strictObject({"addresses": z.array(AddressSchema).nullable().optional().meta({description: "Address bundle referenced by property/parties/ownership"}), "artifacts": z.array(SourceArtifactSchema).nullable().optional(), "assessments": z.array(AssessmentSchema).nullable().optional(), "associations": z.array(PropertyAssociationSchema).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "foreclosure_cases": z.array(ForeclosureCaseSchema).nullable().optional(), "identifiers": z.array(PropertyIdentifierSchema).nullable().optional(), "jurisdictions": z.array(JurisdictionSchema).nullable().optional(), "leases": z.array(LeaseEventSchema).nullable().optional(), "liens": z.array(LienSchema).nullable().optional(), "listings": z.array(ListingSchema).nullable().optional(), "loans": z.array(LoanSchema).nullable().optional(), "operating_statements": z.array(OperatingStatementSchema).nullable().optional(), "ownership": z.array(OwnershipPeriodSchema).nullable().optional(), "parcel_identifiers": z.array(ParcelIdentifierSchema).nullable().optional(), "parcel_lineage": z.array(ParcelLineageSchema).nullable().optional(), "parcels": z.array(ParcelSchema).nullable().optional(), "parties": z.array(PartySchema).nullable().optional(), "permits": z.array(PermitSchema).nullable().optional(), "property": PropertySchema, "property_addresses": z.array(PropertyAddressSchema).nullable().optional(), "property_parcels": z.array(PropertyParcelSchema).nullable().optional(), "property_state_snapshots": z.array(PropertyStateSnapshotSchema).nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rent_rolls": z.array(RentRollSchema).nullable().optional(), "sale_evidence": z.array(SaleEvidenceSchema).nullable().optional(), "sales": z.array(SaleEventSchema).nullable().optional(), "site": z.union([SiteSchema, z.null()]).optional(), "spaces": z.array(SpaceSchema).nullable().optional(), "structures": z.array(StructureSchema).nullable().optional(), "tax_bills": z.array(TaxBillSchema).nullable().optional(), "transfers": z.array(TransferSchema).nullable().optional(), "unit_rents": z.array(UnitRentObservationSchema).nullable().optional(), "valuations": z.array(ValuationSchema).nullable().optional()}).meta({description: "The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.", title: "PropertyProfile"});

export type PropertyProfile = z.infer<typeof PropertyProfileSchema>;

/**
 * Thin capture envelope for a county assessor / public-records fetch. The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the fetch outcome. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence.
 */
export const AssessorObservationSchema = z.strictObject({"artifact_refs": z.array(z.string()).nullable().optional().meta({description: "References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent."}), "assessor_url": rfc3986UriSchema.nullable().optional(), "error": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "profile": z.union([PropertyProfileSchema, z.null()]).optional(), "provenance": ProvenanceSchema, "query_address": z.string().nullable().optional(), "query_parcel_number": z.string().nullable().optional(), "status": AssessorStatusSchema}).meta({description: "Thin capture envelope for a county assessor / public-records fetch. The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the fetch outcome. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence.", title: "AssessorObservation"});

export type AssessorObservation = z.infer<typeof AssessorObservationSchema>;

/**
 */
export const EntitySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "id": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")).meta({description: "Canonical identifier; nonblank with no leading or trailing whitespace"}), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "verifications": z.array(VerificationAttributionSchema).nullable().optional()}).meta({description: "", title: "Entity"});

export type Entity = z.infer<typeof EntitySchema>;

/**
 * What kind of property content an extraction produced. This is a content axis only — where the content came from (public record, vendor, scrape) is Provenance's job. Precedence: when the primary extracted content is a transaction, listing, or lease, use that value even if the page is also a record of property facts. `other` means successfully classified but outside this taxonomy; put the producer's raw label in source_category.
 */
export const ExtractionCategorySchema = z.intersection(z.enum(["sales_transaction", "sale_listing", "lease_listing", "in_place_lease", "property_facts", "other"]), z.string()).meta({description: "What kind of property content an extraction produced. This is a content axis only — where the content came from (public record, vendor, scrape) is Provenance's job. Precedence: when the primary extracted content is a transaction, listing, or lease, use that value even if the page is also a record of property facts. `other` means successfully classified but outside this taxonomy; put the producer's raw label in source_category.", title: "ExtractionCategory"});

export type ExtractionCategory = z.infer<typeof ExtractionCategorySchema>;

/**
 * Outcome of an extraction attempt over already-fetched content. Fetch-level failures (timeout, api_error) belong to the envelope that did the fetching, e.g. AssessorStatus.
 */
export const ExtractionStatusSchema = z.intersection(z.enum(["success", "parse_error", "irrelevant_page", "model_error"]), z.string()).meta({description: "Outcome of an extraction attempt over already-fetched content. Fetch-level failures (timeout, api_error) belong to the envelope that did the fetching, e.g. AssessorStatus.", title: "ExtractionStatus"});

export type ExtractionStatus = z.infer<typeof ExtractionStatusSchema>;

/**
 * Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the extraction outcome. By convention profile and category accompany success and error accompanies failure statuses — validators deliberately do not enforce this pairing (LinkML rules would materialize in only some generated validators, diverging the contracts), so consumers must branch on status, not on field presence.
 */
export const ExtractionObservationSchema = z.strictObject({"artifact_refs": z.array(z.string()).nullable().optional().meta({description: "References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent."}), "category": ExtractionCategorySchema.optional().meta({description: "Content classification; expected when status is success"}), "error": z.string().nullable().optional(), "extracted_at": rfc3339DateTimeSchema.nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "model": z.string().nullable().optional().meta({description: "Extraction model identifier"}), "profile": z.union([PropertyProfileSchema, z.null()]).optional(), "provenance": ProvenanceSchema, "source_category": z.string().nullable().optional().meta({description: "Raw or more specific producer classification, especially when category is `other`"}), "source_url": rfc3986UriSchema.nullable().optional(), "status": ExtractionStatusSchema}).meta({description: "Capture envelope for LLM or scrape extraction (suitable as an LLM structured-output target). The payload is a sparse PropertyProfile (only the sections the source provides; a valid profile still requires its `property` section); status reports the extraction outcome. By convention profile and category accompany success and error accompanies failure statuses — validators deliberately do not enforce this pairing (LinkML rules would materialize in only some generated validators, diverging the contracts), so consumers must branch on status, not on field presence.", title: "ExtractionObservation"});

export type ExtractionObservation = z.infer<typeof ExtractionObservationSchema>;

/**
 * Outcome of an MLS / listing-feed record capture.
 */
export const MlsObservationStatusSchema = z.intersection(z.enum(["success", "not_found", "api_error", "parse_error", "ambiguous"]), z.string()).meta({description: "Outcome of an MLS / listing-feed record capture.", title: "MlsObservationStatus"});

export type MlsObservationStatus = z.infer<typeof MlsObservationStatusSchema>;

/**
 * Capture envelope for an MLS / RESO Web API / feed record. The payload is a sparse PropertyProfile (a valid profile still requires its `property` section); listing-centric property characteristics belong on the profile's state snapshots, not on timeless entities. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence. Source fields the profile does not model may be preserved under extras with producer-namespaced keys; the namespace and field naming are the producer's choice. original_entry_at and source_modified_at are the feed record's clocks (RESO OriginalEntryTimestamp / ModificationTimestamp); they duplicate provenance source clocks so they survive when the profile is detached from the envelope, and should agree with them.
 */
export const MlsObservationSchema = z.strictObject({"artifact_refs": z.array(z.string()).nullable().optional().meta({description: "References to SourceArtifact IDs in the nested profile.artifacts bundle; invalid when profile or profile.artifacts is absent."}), "error": z.string().nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "original_entry_at": rfc3339DateTimeSchema.nullable().optional(), "profile": z.union([PropertyProfileSchema, z.null()]).optional(), "provenance": ProvenanceSchema, "query": z.string().nullable().optional().meta({description: "Listing id/key or address queried"}), "source_modified_at": rfc3339DateTimeSchema.nullable().optional(), "source_record_key": z.string().nullable().optional().meta({description: "The feed's ListingKey for this record"}), "status": MlsObservationStatusSchema}).meta({description: "Capture envelope for an MLS / RESO Web API / feed record. The payload is a sparse PropertyProfile (a valid profile still requires its `property` section); listing-centric property characteristics belong on the profile's state snapshots, not on timeless entities. By convention profile accompanies success and error accompanies failure statuses — validators deliberately do not enforce this pairing, so consumers must branch on status, not on field presence. Source fields the profile does not model may be preserved under extras with producer-namespaced keys; the namespace and field naming are the producer's choice. original_entry_at and source_modified_at are the feed record's clocks (RESO OriginalEntryTimestamp / ModificationTimestamp); they duplicate provenance source clocks so they survive when the profile is detached from the envelope, and should agree with them.", title: "MlsObservation"});

export type MlsObservation = z.infer<typeof MlsObservationSchema>;

/**
 */
export const PropertyFactsSchema = z.strictObject({"building_count": z.number().int().nullable().optional().meta({description: "Producer summary when structures are not enumerated"}), "estate_type": EstateTypeSchema.optional(), "location": z.union([GeoPointSchema, z.null()]).optional(), "name": z.string().nullable().optional(), "property_use_class": z.string().nullable().optional().meta({description: "PUCS class when property_use_system identifies PUCS"}), "property_use_subtype": z.string().nullable().optional(), "property_use_system": z.string().nullable().optional().meta({description: "Required when use fields are set; no default"}), "property_use_type": z.string().nullable().optional()}).meta({description: "", title: "PropertyFacts"});

export type PropertyFacts = z.infer<typeof PropertyFactsSchema>;

/**
 * This row corresponds to a document recorded/registered with an authority (county recorder, land registry, notarial protocol). Narrowly about document identity and recording — legal effect lives on the host class.
 */
export const RecordedInstrumentSchema = z.strictObject({"artifacts": z.array(z.string()).nullable().optional(), "document_number": z.string().nullable().optional().meta({description: "Primary identifier assigned by the recording/registry authority"}), "document_type": z.union([CodeableConceptSchema, z.null()]).optional(), "instrument_date": z.iso.date().nullable().optional().meta({description: "Date executed/signed as dated on the instrument"}), "recorded_date": z.iso.date().nullable().optional().meta({description: "Date accepted, recorded, or registered by the authority"}), "recording_authority": z.string().nullable().optional().meta({description: "Authority maintaining the record (optional — parcel context is inference, not identity)"}), "recording_book": z.string().nullable().optional(), "recording_page": z.string().nullable().optional(), "registry_reference": z.string().nullable().optional().meta({description: "Alternate authority-issued reference: title, dealing, folio, or notarial-act number"}), "related_instruments": z.array(InstrumentReferenceSchema).nullable().optional()}).meta({description: "This row corresponds to a document recorded/registered with an authority (county recorder, land registry, notarial protocol). Narrowly about document identity and recording — legal effect lives on the host class.", title: "RecordedInstrument"});

export type RecordedInstrument = z.infer<typeof RecordedInstrumentSchema>;

/**
 */
export const SiteFactsSchema = z.strictObject({"block": z.string().nullable().optional(), "buildable_units": z.number().int().nullable().optional(), "depth": z.union([LengthSchema, z.null()]).optional(), "easements": z.array(z.string()).nullable().optional(), "entitlement_status": z.string().nullable().optional(), "flood_zone": z.string().nullable().optional(), "frontage": z.union([LengthSchema, z.null()]).optional(), "hazard_zones": z.array(z.string()).nullable().optional(), "is_corner": z.boolean().nullable().optional(), "land_use": z.union([CodeableConceptSchema, z.null()]).optional(), "land_use_category": z.string().nullable().optional(), "lot_number": z.string().nullable().optional(), "lot_size": z.union([AreaSchema, z.null()]).optional(), "phase_number": z.string().nullable().optional(), "restrictions": z.array(z.string()).nullable().optional(), "section_township_range": z.string().nullable().optional(), "site_influences": z.array(z.string()).nullable().optional(), "subdivision": z.string().nullable().optional(), "topography": z.string().nullable().optional(), "tract_number": z.string().nullable().optional(), "usable_land_area": z.union([AreaSchema, z.null()]).optional(), "usable_land_area_basis": z.string().nullable().optional(), "utilities": z.array(z.string()).nullable().optional(), "view_types": z.array(z.string()).nullable().optional(), "zoning_code": z.string().nullable().optional()}).meta({description: "", title: "SiteFacts"});

export type SiteFacts = z.infer<typeof SiteFactsSchema>;

/**
 */
export const SpaceFactsSchema = z.strictObject({"bathrooms": z.number().nullable().optional(), "bedrooms": z.number().int().nullable().optional(), "floor_number": z.number().int().nullable().optional(), "is_active": z.boolean().nullable().optional(), "is_adu": z.boolean().nullable().optional(), "occupancy": z.string().nullable().optional().meta({description: "owner_occupied | tenant | vacant"}), "rentable_area": z.union([AreaSchema, z.null()]).optional(), "space_use": z.string().nullable().optional(), "usable_area": z.union([AreaSchema, z.null()]).optional()}).meta({description: "", title: "SpaceFacts"});

export type SpaceFacts = z.infer<typeof SpaceFactsSchema>;

/**
 */
export const StructureFactsSchema = z.strictObject({"areas": z.array(AreaMeasureSchema).nullable().optional().meta({description: "Long-tail area kinds; canonical kinds forbidden here (validator-enforced)"}), "basement_area": z.union([AreaSchema, z.null()]).optional(), "basement_finished_area": z.union([AreaSchema, z.null()]).optional(), "commercial": z.union([CommercialDetailsSchema, z.null()]).optional(), "condition_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Physical-condition ratings. Semantic validation permits at most one rating for each system and scope pair."}), "construction_method": z.string().nullable().optional().meta({description: "site_built | manufactured | modular | container | 3d_printed | ..."}), "construction_status": z.string().nullable().optional().meta({description: "complete | proposed | under_construction"}), "construction_type": z.string().nullable().optional(), "cooling_types": z.array(z.string()).nullable().optional(), "effective_year_built": z.number().int().nullable().optional(), "exterior_wall_type": z.string().nullable().optional(), "features": z.array(z.string()).nullable().optional().meta({description: "Open vocabulary for physical features and amenities"}), "foundation_material": z.string().nullable().optional(), "foundation_type": z.string().nullable().optional(), "garage_area": z.union([AreaSchema, z.null()]).optional(), "gross_area": z.union([AreaSchema, z.null()]).optional(), "ground_floor_area": z.union([AreaSchema, z.null()]).optional(), "heating_fuel_type": z.string().nullable().optional(), "heating_types": z.array(z.string()).nullable().optional(), "kind": z.string().nullable().optional().meta({description: "building | barn | silo | shed | outbuilding | ... (open)"}), "living_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Finished area intended for human habitation. The measurement method and treatment of above-grade and below-grade space come from the applicable profile or provenance."}), "name": z.string().nullable().optional(), "quality_ratings": z.array(RatingSchema).nullable().optional().meta({description: "Construction-quality ratings. Semantic validation permits at most one rating for each system and scope pair."}), "renovations": z.array(RenovationSchema).nullable().optional(), "rentable_area": z.union([AreaSchema, z.null()]).optional().meta({description: "Area of occupiable premises allocated to a tenant or available for lease under the stated measurement method."}), "residential": z.union([ResidentialDetailsSchema, z.null()]).optional(), "roof_material_type": z.string().nullable().optional(), "roof_style_type": z.string().nullable().optional(), "sewer_type": z.string().nullable().optional(), "stories": z.number().nullable().optional(), "structure_number": z.string().nullable().optional(), "unit_count": z.number().int().min(1).nullable().optional(), "water_type": z.string().nullable().optional(), "year_built": z.number().int().min(1).max(2200).nullable().optional(), "year_built_estimated": z.boolean().nullable().optional()}).meta({description: "", title: "StructureFacts"});

export type StructureFacts = z.infer<typeof StructureFactsSchema>;

/**
 * A contextual role-bearing relationship from an event or record to a canonical Party. Exact source wording belongs in provenance or source artifacts, not in a second actor-name field. Multi-party is the norm (couples on deeds, co-borrowers).
 */
export const TransactionPartySchema = z.strictObject({"extras": z.union([AnySchema, z.null()]).optional(), "party": z.string().regex(new RegExp("^[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF](?:[\\s\\S]*[^\\u0009-\\u000D\\u0020\\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000\\uFEFF])?(?![\\s\\S])")), "role": z.string(), "sequence": z.number().int().min(1).nullable().optional()}).meta({description: "A contextual role-bearing relationship from an event or record to a canonical Party. Exact source wording belongs in provenance or source artifacts, not in a second actor-name field. Multi-party is the norm (couples on deeds, co-borrowers).", title: "TransactionParty"});

export type TransactionParty = z.infer<typeof TransactionPartySchema>;

/**
 * The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.
 */
export const PhdsSchema = z.looseObject({"addresses": z.array(AddressSchema).nullable().optional().meta({description: "Address bundle referenced by property/parties/ownership"}), "artifacts": z.array(SourceArtifactSchema).nullable().optional(), "assessments": z.array(AssessmentSchema).nullable().optional(), "associations": z.array(PropertyAssociationSchema).nullable().optional(), "extras": z.union([AnySchema, z.null()]).optional(), "foreclosure_cases": z.array(ForeclosureCaseSchema).nullable().optional(), "identifiers": z.array(PropertyIdentifierSchema).nullable().optional(), "jurisdictions": z.array(JurisdictionSchema).nullable().optional(), "leases": z.array(LeaseEventSchema).nullable().optional(), "liens": z.array(LienSchema).nullable().optional(), "listings": z.array(ListingSchema).nullable().optional(), "loans": z.array(LoanSchema).nullable().optional(), "operating_statements": z.array(OperatingStatementSchema).nullable().optional(), "ownership": z.array(OwnershipPeriodSchema).nullable().optional(), "parcel_identifiers": z.array(ParcelIdentifierSchema).nullable().optional(), "parcel_lineage": z.array(ParcelLineageSchema).nullable().optional(), "parcels": z.array(ParcelSchema).nullable().optional(), "parties": z.array(PartySchema).nullable().optional(), "permits": z.array(PermitSchema).nullable().optional(), "property": PropertySchema, "property_addresses": z.array(PropertyAddressSchema).nullable().optional(), "property_parcels": z.array(PropertyParcelSchema).nullable().optional(), "property_state_snapshots": z.array(PropertyStateSnapshotSchema).nullable().optional(), "provenance": z.union([ProvenanceSchema, z.null()]).optional(), "rent_rolls": z.array(RentRollSchema).nullable().optional(), "sale_evidence": z.array(SaleEvidenceSchema).nullable().optional(), "sales": z.array(SaleEventSchema).nullable().optional(), "site": z.union([SiteSchema, z.null()]).optional(), "spaces": z.array(SpaceSchema).nullable().optional(), "structures": z.array(StructureSchema).nullable().optional(), "tax_bills": z.array(TaxBillSchema).nullable().optional(), "transfers": z.array(TransferSchema).nullable().optional(), "unit_rents": z.array(UnitRentObservationSchema).nullable().optional(), "valuations": z.array(ValuationSchema).nullable().optional()}).meta({$id: "https://example.org/phds", $schema: "https://json-schema.org/draft/2019-09/schema", description: "The flagship interchange document: everything known about one property, as one bundle. Every section is exactly the corresponding entity shape. parties[] is the deduplicated bundle carrying every Party referenced by any record, including tenants, lenders, contractors, associations, and valuation performers (required for lossless round-trip). Conformance requires passing the entities → profile → entities round-trip test.", metamodel_version: "1.11.0", title: "phds", version: "0.2.0"});

export type Phds = z.infer<typeof PhdsSchema>;
