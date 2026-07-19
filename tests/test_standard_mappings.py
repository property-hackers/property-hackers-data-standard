"""Tests for the machine-readable cross-standard mapping index.

The census pins every mapping annotation in the schema so drift between
schema annotations and docs/crosswalks/reso-dd20-alignment.md surfaces
here. The scenario tests exercise the index the way an importer would:
route a simulated flat RESO feed record into the layered listing -> sale
model and validate the result at both the wire-shape and semantic layers.
"""

from __future__ import annotations

from pathlib import Path
import unittest

from tests.test_generated_contract import _load_pydantic_module
from tools.profile_validation import validate_profile
from tools.reso_status_matrix import (
    STANDARD_STATUS_TO_LISTING_STATUS,
    normalize_standard_status,
)
from tools.standard_mappings import (
    MappingEntry,
    _schema_view,
    entries_for_prefix,
    entries_for_target,
    mapping_entries,
)

CROSSWALK_PATH = (
    Path(__file__).resolve().parents[1]
    / "docs"
    / "crosswalks"
    / "reso-dd20-alignment.md"
)


def _usd(amount: str) -> dict:
    return {"amount": amount, "currency": "USD"}


class MappingIndexCensusTests(unittest.TestCase):
    def test_reso_is_currently_the_only_mapped_standard(self):
        prefixes = {e.target.split(":", 1)[0] for e in mapping_entries()}
        self.assertEqual({"reso"}, prefixes)
        self.assertEqual(mapping_entries(), entries_for_prefix("reso"))

    def test_full_mapping_census(self):
        expected = (
            MappingEntry("Listing", "expiration_date", "exact", "reso:ExpirationDate"),
            MappingEntry("Listing", "listing_agreement_type", "close", "reso:ListingAgreement"),
            MappingEntry("Listing", "listing_contract_date", "exact", "reso:ListingContractDate"),
            MappingEntry("Listing", "service_level", "close", "reso:ListingService"),
            MappingEntry("Listing", "special_listing_conditions", "close", "reso:SpecialListingConditions"),
            MappingEntry("ListingEvent", "close_price", "close", "reso:ClosePrice"),
            MappingEntry("ListingEvent", "list_price", "close", "reso:ListPrice"),
            MappingEntry("ListingEvent", "list_price_low", "close", "reso:ListPriceLow"),
            MappingEntry("ListingEvent", "source_status", "exact", "reso:MlsStatus"),
            MappingEntry("ListingEvent", "status", "close", "reso:StandardStatus"),
            MappingEntry("MlsObservation", "original_entry_at", "close", "reso:OriginalEntryTimestamp"),
            MappingEntry("MlsObservation", "source_modified_at", "close", "reso:ModificationTimestamp"),
            MappingEntry("Parcel", "parcel_number", "exact", "reso:ParcelNumber"),
            MappingEntry("Provenance", "originating_system", "close", "reso:OriginatingSystemName"),
            MappingEntry("Provenance", "source_system", "close", "reso:SourceSystemName"),
            MappingEntry("ResidentialDetails", "bedrooms_total", "exact", "reso:BedroomsTotal"),
            MappingEntry("SaleEvent", "buyer_financing", "close", "reso:BuyerFinancing"),
            MappingEntry("SaleEvent", "close_date", "close", "reso:CloseDate"),
            MappingEntry("SaleEvent", "concessions", "close", "reso:Concessions"),
            MappingEntry("SaleEvent", "concessions_amount", "close", "reso:ConcessionsAmount"),
            MappingEntry("SaleEvent", "concessions_comments", "close", "reso:ConcessionsComments"),
            MappingEntry("SaleEvent", "sale_price", "related", "reso:ClosePrice"),
            MappingEntry("SaleEvidence", "close_date", "exact", "reso:CloseDate"),
            MappingEntry("SaleEvidence", "close_price", "close", "reso:ClosePrice"),
            MappingEntry("SaleEvidence", "concessions_amount", "close", "reso:ConcessionsAmount"),
            MappingEntry("SourceArtifact", "media_type", "close", "reso:MimeType"),
            MappingEntry("SourceArtifact", "order", "close", "reso:Order"),
            MappingEntry("SourceArtifact", "short_description", "close", "reso:ShortDescription"),
            MappingEntry("SourceArtifact", "source_modified_at", "close", "reso:ModificationTimestamp"),
            MappingEntry("SourceArtifact", "uri", "close", "reso:MediaURL"),
            MappingEntry("Structure", "living_area", "exact", "reso:LivingArea"),
            MappingEntry("Structure", "year_built", "exact", "reso:YearBuilt"),
            MappingEntry("StructureFacts", "living_area", "exact", "reso:LivingArea"),
            MappingEntry("StructureFacts", "year_built", "exact", "reso:YearBuilt"),
            MappingEntry("StructureState", "living_area", "exact", "reso:LivingArea"),
            MappingEntry("StructureState", "year_built", "exact", "reso:YearBuilt"),
        )
        self.assertEqual(expected, mapping_entries())

    def test_every_annotated_reso_term_is_documented_in_the_crosswalk(self):
        crosswalk = CROSSWALK_PATH.read_text(encoding="utf-8")
        for entry in entries_for_prefix("reso"):
            term = entry.target.split(":", 1)[1]
            with self.subTest(target=entry.target):
                self.assertIn(term, crosswalk)

    def test_close_price_routes_to_all_three_price_layers(self):
        by_slot = {
            (e.host_class, e.slot): e.strength
            for e in entries_for_target("reso:ClosePrice")
        }
        self.assertEqual(
            {
                ("ListingEvent", "close_price"): "close",
                ("SaleEvidence", "close_price"): "close",
                ("SaleEvent", "sale_price"): "related",
            },
            by_slot,
        )


class ResoFeedImportScenarioTests(unittest.TestCase):
    """A closed listing arrives as one flat RESO Property-resource record;
    the mapping index is the routing table into the layered model."""

    FEED = {
        "ListingKey": "K2400031",
        "ListingId": "7654321",
        "ListingContractDate": "2026-01-04",
        "OnMarketDate": "2026-01-05",
        "ListPrice": "415000",
        "StandardStatus": "Closed",
        "MlsStatus": "S",
        "CloseDate": "2026-04-01",
        "ClosePrice": "407500",
        "BuyerFinancing": "Conventional",
        "ConcessionsAmount": "5000",
        "OriginatingSystemName": "Example MLS",
        "LO1_BoardCode": "EX",
    }

    # feed field -> (host class, slot, strength) as the import routes it.
    # Fields absent here route through documented transformations instead:
    # ListingKey/ListingId become ListingIdentifier rows, OnMarketDate
    # becomes the listed event's effective_date, StandardStatus=Closed is
    # disambiguated to sold via offering_type.
    ROUTES = (
        ("ListingContractDate", "Listing", "listing_contract_date", "exact"),
        ("ListPrice", "ListingEvent", "list_price", "close"),
        ("StandardStatus", "ListingEvent", "status", "close"),
        ("MlsStatus", "ListingEvent", "source_status", "exact"),
        ("ClosePrice", "ListingEvent", "close_price", "close"),
        ("ClosePrice", "SaleEvidence", "close_price", "close"),
        ("ClosePrice", "SaleEvent", "sale_price", "related"),
        ("CloseDate", "SaleEvent", "close_date", "close"),
        ("CloseDate", "SaleEvidence", "close_date", "exact"),
        ("BuyerFinancing", "SaleEvent", "buyer_financing", "close"),
        ("ConcessionsAmount", "SaleEvent", "concessions_amount", "close"),
        ("ConcessionsAmount", "SaleEvidence", "concessions_amount", "close"),
        ("OriginatingSystemName", "Provenance", "originating_system", "close"),
    )

    def _imported_profile(self) -> dict:
        feed = self.FEED
        provenance = {
            "provider": "Example Aggregator",
            "originating_system": feed["OriginatingSystemName"],
            "method": "api",
        }
        return {
            "property": {"id": "prop-1"},
            "listings": [
                {
                    "id": "lst-1",
                    "property": "prop-1",
                    "offering_type": "for_sale",
                    "identifiers": [
                        {
                            "scheme": "listing_key",
                            "namespace": "example-mls",
                            "value": feed["ListingKey"],
                            "is_primary": True,
                        },
                        {
                            "scheme": "listing_id",
                            "namespace": "example-mls",
                            "value": feed["ListingId"],
                        },
                    ],
                    "listing_contract_date": feed["ListingContractDate"],
                    "marketing_channel": "mls",
                    "events": [
                        {
                            "effective_date": feed["OnMarketDate"],
                            "event_type": "listed",
                            "list_price": _usd(feed["ListPrice"]),
                        },
                        {
                            "effective_date": feed["CloseDate"],
                            "event_type": "closed",
                            "status": normalize_standard_status(
                                feed["StandardStatus"], "for_sale"
                            ),
                            "source_status": feed["MlsStatus"],
                            "close_price": _usd(feed["ClosePrice"]),
                        },
                    ],
                }
            ],
            "sales": [
                {
                    "id": "sale-1",
                    "property": "prop-1",
                    "close_date": feed["CloseDate"],
                    "sale_price": _usd(feed["ClosePrice"]),
                    "buyer_financing": {"code": feed["BuyerFinancing"]},
                    "concessions_amount": _usd(feed["ConcessionsAmount"]),
                    "listings": [
                        {"listing": "lst-1", "relationship_type": "resulted_in_sale"}
                    ],
                }
            ],
            "sale_evidence": [
                {
                    "id": "ev-1",
                    "sale": "sale-1",
                    "listing": "lst-1",
                    "close_date": feed["CloseDate"],
                    "close_price": _usd(feed["ClosePrice"]),
                    "concessions_amount": _usd(feed["ConcessionsAmount"]),
                    "verification_method": {"code": "mls_record"},
                    "provenance": provenance,
                }
            ],
            "provenance": provenance,
            "extras": {"example-mls.lo1_board_code": feed["LO1_BoardCode"]},
        }

    def test_every_route_is_declared_in_the_schema_mapping_annotations(self):
        entries = set(mapping_entries())
        for field, host, slot, strength in self.ROUTES:
            with self.subTest(field=field, host=host, slot=slot):
                self.assertIn(MappingEntry(host, slot, strength, f"reso:{field}"), entries)

    def test_local_field_has_no_mapping_and_rides_in_extras(self):
        self.assertEqual((), entries_for_target("reso:LO1_BoardCode"))
        profile = self._imported_profile()
        self.assertEqual(
            {"example-mls.lo1_board_code": "EX"}, profile["extras"]
        )

    def test_imported_profile_is_wire_valid_and_semantically_clean(self):
        profile = self._imported_profile()
        module = _load_pydantic_module()
        module.PropertyProfile(**profile)
        self.assertEqual([], validate_profile(profile))


class StandardStatusMatrixTests(unittest.TestCase):
    """The canonical StandardStatus -> ListingStatus data in
    tools/reso_status_matrix.py, table-driven and synced to the crosswalk."""

    def test_matrix_covers_all_documented_standard_statuses(self):
        self.assertEqual(
            {
                "Coming Soon",
                "Active",
                "Active Under Contract",
                "Pending",
                "Hold",
                "Closed",
                "Canceled",
                "Expired",
                "Withdrawn",
                "Incomplete",
                "Delete",
            },
            set(STANDARD_STATUS_TO_LISTING_STATUS),
        )

    def test_every_target_is_a_listing_status_enum_value(self):
        allowed = set(
            _schema_view().get_enum("ListingStatus").permissible_values.keys()
        )
        for status, mapping in STANDARD_STATUS_TO_LISTING_STATUS.items():
            with self.subTest(status=status):
                self.assertIn(mapping.for_sale, allowed)
                self.assertIn(mapping.for_lease, allowed)

    def test_closed_branches_by_offering_type(self):
        self.assertEqual("sold", normalize_standard_status("Closed", "for_sale"))
        self.assertEqual("leased", normalize_standard_status("Closed", "for_lease"))

    def test_record_management_states_normalize_to_other_and_preserve_raw(self):
        for status in ("Incomplete", "Delete"):
            mapping = STANDARD_STATUS_TO_LISTING_STATUS[status]
            with self.subTest(status=status):
                self.assertEqual(("other", "other", True), (
                    mapping.for_sale,
                    mapping.for_lease,
                    mapping.preserve_raw,
                ))

    def test_unknown_status_raises_instead_of_guessing(self):
        with self.assertRaises(KeyError):
            normalize_standard_status("Rented", "for_lease")

    def test_matrix_agrees_with_crosswalk_status_table(self):
        section = CROSSWALK_PATH.read_text(encoding="utf-8").split(
            "## Status matrix", 1
        )[1].split("\n## ", 1)[0]
        rows: dict[str, str] = {}
        for line in section.splitlines():
            if not line.startswith("|"):
                continue
            cells = [cell.strip() for cell in line.strip("|").split("|")]
            if len(cells) >= 2 and cells[0] not in ("StandardStatus", "") and not set(cells[0]) <= {"-"}:
                rows[cells[0]] = cells[1]
        self.assertEqual(set(STANDARD_STATUS_TO_LISTING_STATUS), set(rows))
        for status, mapping in STANDARD_STATUS_TO_LISTING_STATUS.items():
            with self.subTest(status=status):
                self.assertIn(mapping.for_sale, rows[status])
                self.assertIn(mapping.for_lease, rows[status])
