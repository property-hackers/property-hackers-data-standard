from __future__ import annotations

from contextlib import redirect_stderr
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

from linkml_runtime.utils.schemaview import SchemaView

from tests.test_generated_contract import _load_pydantic_module
from tools.profile_validation import (
    ValidationIssue,
    main,
    reachable_reference_rules,
    reference_target_collections,
    validate_profile,
)
from tools.standards_validation import boma_placement_issues
import tools.profile_validation as profile_validation


ROOT = Path(__file__).resolve().parents[1]
SEMANTIC_COUNTER_EXAMPLES = ROOT / "counter_examples" / "semantic"

INFERRED_INLINING_SCHEMA = """\
id: https://example.org/inferred-inlining
name: inferred_inlining
prefixes:
  linkml: https://w3id.org/linkml/
  test: https://example.org/inferred-inlining/
default_prefix: test
imports: [linkml:types]
classes:
  Party:
    attributes:
      id: {identifier: true, required: true}
  Value:
    attributes:
      nested_party: {range: Party}
  Host:
    attributes:
      direct_party: {range: Party}
      value: {range: Value}
  PropertyProfile:
    tree_root: true
    attributes:
      host: {range: Host, inlined: true}
"""


class ProfileValidationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.models = _load_pydantic_module()

    def assert_issues(self, profile, expected):
        self.assertEqual(expected, validate_profile(profile))

    def test_hash_requires_scheme(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [
                {"id": "a1", "country": "CA", "address_hash": "hash-1"}
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "addresses[0]",
                    "address_hash requires address_hash_scheme",
                )
            ],
        )

    def test_boma_market_classification_is_rejected_inside_rating_collections(self):
        profile = {
            "structures": [
                {
                    "condition_ratings": [
                        {
                            "system": "urn:phds:standard:boma:office:metro",
                            "code": "A",
                        }
                    ]
                }
            ]
        }

        self.assertEqual(
            [
                "structures[0].condition_ratings[0]: BOMA market classification "
                "is not a physical condition or construction quality rating"
            ],
            boma_placement_issues(profile),
        )

    def test_every_entity_id_is_unique_across_top_level_and_nested_records(self):
        profile = {
            "property": {"id": "property-1"},
            "structures": [{"id": "shared-id", "property": "property-1"}],
            "artifacts": [
                {"id": "property-1", "storage_reference": "reports/one.pdf"}
            ],
            "property_state_snapshots": [
                {
                    "id": "snapshot-1",
                    "property": "property-1",
                    "as_of_date": "2026-01-01",
                    "structure_states": [
                        {"id": "shared-id", "subject": "shared-id"}
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "artifacts[0].id",
                    "duplicate Entity id; first declared at property.id",
                ),
                ValidationIssue(
                    "property_state_snapshots[0].structure_states[0].id",
                    "duplicate Entity id; first declared at structures[0].id",
                ),
            ],
        )

    def test_non_inlined_references_resolve_by_declared_target_bundle(self):
        profile = {
            "property": {"id": "property-1"},
            "jurisdictions": [
                {"id": "jurisdiction-1", "country": "CA", "name": "Registry"}
            ],
            "parcels": [
                {
                    "id": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "parcel_number": "P-1",
                }
            ],
            "structures": [{"id": "structure-1", "property": "property-1"}],
            "spaces": [
                {
                    "id": "space-1",
                    "property": "property-1",
                    "structure": "missing-structure",
                }
            ],
            "transfers": [
                {
                    "id": "transfer-1",
                    "property": "property-1",
                    "transfer_type": "deed",
                    "parcel": "missing-parcel",
                    "recording_authority": "missing-jurisdiction",
                    "related_instruments": [
                        {"recording_authority": "missing-related-authority"}
                    ],
                }
            ],
            "sales": [
                {
                    "id": "sale-1",
                    "property": "property-1",
                    "close_date": "2026-01-01",
                    "transfer": "missing-transfer",
                    "supporting_operating_statement": "missing-statement",
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "sales[0].supporting_operating_statement",
                    "operating statement reference does not resolve",
                ),
                ValidationIssue(
                    "sales[0].transfer", "transfer reference does not resolve"
                ),
                ValidationIssue(
                    "spaces[0].structure", "structure reference does not resolve"
                ),
                ValidationIssue(
                    "transfers[0].parcel", "parcel reference does not resolve"
                ),
                ValidationIssue(
                    "transfers[0].recording_authority",
                    "jurisdiction reference does not resolve",
                ),
                ValidationIssue(
                    "transfers[0].related_instruments[0].recording_authority",
                    "jurisdiction reference does not resolve",
                ),
            ],
        )

    def test_every_reachable_non_inlined_linkml_reference_has_a_target_bundle(self):
        rules = reachable_reference_rules()
        self.assertEqual(90, len(rules))
        mapped_ranges = set(reference_target_collections())
        self.assertFalse(
            {rule.target_class for rule in rules} - mapped_ranges,
            "every non-inlined class reference reachable from PropertyProfile must map "
            "to a canonical profile target bundle",
        )
        self.assertTrue(
            {
                ("SaleEvent", "transfer", "Transfer"),
                ("SaleEvent", "supporting_operating_statement", "OperatingStatement"),
                ("Loan", "transfer", "Transfer"),
                ("ForeclosureCase", "loan", "Loan"),
                ("InstrumentReference", "recording_authority", "Jurisdiction"),
                ("RentRollLine", "tenant", "Party"),
                ("VerificationAttribution", "verifier", "Party"),
                ("SaleListingRelationship", "listing", "Listing"),
                ("ParcelIdentifier", "parcel", "Parcel"),
            }.issubset(
                {(rule.host_class, rule.slot, rule.target_class) for rule in rules}
            )
        )

    def test_reference_discovery_uses_inferred_non_inlined_semantics(self):
        with tempfile.TemporaryDirectory() as tmp:
            schema_path = Path(tmp) / "inferred-inlining.yaml"
            schema_path.write_text(INFERRED_INLINING_SCHEMA, encoding="utf-8")
            view = SchemaView(str(schema_path))
            profile_validation.reachable_reference_rules.cache_clear()
            try:
                with patch.object(profile_validation, "_schema_view", return_value=view):
                    rules = profile_validation.reachable_reference_rules()
            finally:
                profile_validation.reachable_reference_rules.cache_clear()

        self.assertEqual(
            {
                ("Host", "direct_party", "Party"),
                ("Value", "nested_party", "Party"),
            },
            {(rule.host_class, rule.slot, rule.target_class) for rule in rules},
        )

    def test_typed_node_traversal_uses_inferred_inline_semantics(self):
        with tempfile.TemporaryDirectory() as tmp:
            schema_path = Path(tmp) / "inferred-inlining.yaml"
            schema_path.write_text(INFERRED_INLINING_SCHEMA, encoding="utf-8")
            view = SchemaView(str(schema_path))
            profile = {
                "host": {
                    "direct_party": "party-1",
                    "value": {"nested_party": "party-2"},
                }
            }
            with patch.object(profile_validation, "_schema_view", return_value=view):
                typed_nodes = list(profile_validation._typed_nodes(profile))

        self.assertEqual(
            [
                ("", "PropertyProfile"),
                ("host", "Host"),
                ("host.value", "Value"),
            ],
            [(path, class_name) for path, class_name, _ in typed_nodes],
        )

    def test_reference_validation_fails_closed_when_a_target_bundle_is_unmapped(self):
        mappings_without_party = {
            target: collection
            for target, collection in profile_validation.REFERENCE_TARGET_COLLECTIONS.items()
            if target != "Party"
        }
        profile = {
            "property": {"id": "property-1"},
            "rent_rolls": [
                {
                    "id": "rent-roll-1",
                    "property": "property-1",
                    "as_of_date": "2026-01-01",
                    "lines": [{"tenant": "party-1"}],
                }
            ],
        }

        with patch.object(
            profile_validation,
            "REFERENCE_TARGET_COLLECTIONS",
            mappings_without_party,
        ):
            with self.assertRaisesRegex(
                RuntimeError,
                r"no PropertyProfile target bundle for Party \(RentRollLine\.tenant\)",
            ):
                validate_profile(profile)

    def test_every_stable_physical_record_is_scoped_to_the_profile_property(self):
        profile = {
            "property": {"id": "property-1"},
            "site": {"id": "site-1", "property": "other-property"},
            "structures": [{"id": "structure-1", "property": "other-property"}],
            "spaces": [{"id": "space-1", "property": "other-property"}],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "site.property", "property reference does not resolve"
                ),
                ValidationIssue(
                    "spaces[0].property", "property reference does not resolve"
                ),
                ValidationIssue(
                    "structures[0].property", "property reference does not resolve"
                ),
            ],
        )

    def test_party_address_dates_must_be_ordered(self):
        profile = {
            "property": {"id": "property-1"},
            "addresses": [{"id": "address-1", "country": "JP"}],
            "parties": [
                {
                    "id": "party-1",
                    "kind": "organization",
                    "name": "テナント",
                    "addresses": [
                        {
                            "address": "address-1",
                            "valid_from": "2026-02-01",
                            "valid_to": "2026-01-31",
                        }
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "parties[0].addresses[0]", "valid_to precedes valid_from"
                )
            ],
        )

    def test_party_primary_address_intervals_for_the_same_role_must_not_overlap(self):
        profile = {
            "property": {"id": "property-1"},
            "addresses": [
                {"id": "address-1", "country": "JP"},
                {"id": "address-2", "country": "JP"},
            ],
            "parties": [
                {
                    "id": "party-1",
                    "kind": "organization",
                    "name": "テナント",
                    "addresses": [
                        {
                            "address": "address-1",
                            "role": {"code": "mailing"},
                            "is_primary": True,
                            "valid_from": "2024-01-01",
                            "valid_to": "2024-12-31",
                        },
                        {
                            "address": "address-2",
                            "role": {"code": "mailing"},
                            "is_primary": True,
                            "valid_from": "2024-12-31",
                        },
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "parties[0].addresses[1]",
                    "primary address validity overlaps for role mailing",
                )
            ],
        )

    def test_normative_date_intervals_must_be_ordered(self):
        base = {
            "property": {"id": "property-1"},
            "jurisdictions": [
                {
                    "id": "jurisdiction-1",
                    "country": "CA",
                    "name": "Registry",
                    "kind": "registry",
                }
            ],
            "parcels": [
                {
                    "id": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "parcel_number": "P-1",
                }
            ],
        }
        cases = (
            (
                "property parcel",
                {
                    "property_parcels": [
                        {
                            "id": "property-parcel-1",
                            "property": "property-1",
                            "parcel": "parcel-1",
                            "start_date": "2026-01-01",
                            "end_date": "2025-01-01",
                        }
                    ]
                },
                "property_parcels[0]",
                "end_date precedes start_date",
            ),
            (
                "ownership",
                {
                    "ownership": [
                        {
                            "id": "ownership-1",
                            "property": "property-1",
                            "start_date": "2026-01-01",
                            "end_date": "2025-01-01",
                        }
                    ]
                },
                "ownership[0]",
                "end_date precedes start_date",
            ),
            (
                "lease",
                {
                    "leases": [
                        {
                            "id": "lease-1",
                            "property": "property-1",
                            "commencement_date": "2026-01-01",
                            "expiration_date": "2025-01-01",
                        }
                    ]
                },
                "leases[0]",
                "expiration_date precedes commencement_date",
            ),
            (
                "lease escalation",
                {
                    "leases": [
                        {
                            "id": "lease-1",
                            "property": "property-1",
                            "escalations": [
                                {
                                    "escalation_type": "fixed_percent",
                                    "effective_from": "2026-01-01",
                                    "effective_until": "2025-01-01",
                                }
                            ],
                        }
                    ]
                },
                "leases[0].escalations[0]",
                "effective_until precedes effective_from",
            ),
            (
                "foreclosure",
                {
                    "foreclosure_cases": [
                        {
                            "id": "foreclosure-1",
                            "property": "property-1",
                            "opened_date": "2026-01-01",
                            "resolved_date": "2025-01-01",
                        }
                    ]
                },
                "foreclosure_cases[0]",
                "resolved_date precedes opened_date",
            ),
            (
                "permit application to issue",
                {
                    "permits": [
                        {
                            "id": "permit-1",
                            "property": "property-1",
                            "applied_date": "2026-01-01",
                            "issued_date": "2025-01-01",
                        }
                    ]
                },
                "permits[0]",
                "issued_date precedes applied_date",
            ),
            (
                "permit issue to final",
                {
                    "permits": [
                        {
                            "id": "permit-1",
                            "property": "property-1",
                            "issued_date": "2026-01-01",
                            "finaled_date": "2025-01-01",
                        }
                    ]
                },
                "permits[0]",
                "finaled_date precedes issued_date",
            ),
            (
                "permit issue to expiration",
                {
                    "permits": [
                        {
                            "id": "permit-1",
                            "property": "property-1",
                            "issued_date": "2026-01-01",
                            "expiration_date": "2025-01-01",
                        }
                    ]
                },
                "permits[0]",
                "expiration_date precedes issued_date",
            ),
            (
                "permit application to final without issue date",
                {
                    "permits": [
                        {
                            "id": "permit-1",
                            "property": "property-1",
                            "applied_date": "2026-01-01",
                            "finaled_date": "2025-01-01",
                        }
                    ]
                },
                "permits[0]",
                "finaled_date precedes applied_date",
            ),
            (
                "permit application to expiration without issue date",
                {
                    "permits": [
                        {
                            "id": "permit-1",
                            "property": "property-1",
                            "applied_date": "2026-01-01",
                            "expiration_date": "2025-01-01",
                        }
                    ]
                },
                "permits[0]",
                "expiration_date precedes applied_date",
            ),
            (
                "operating statement",
                {
                    "operating_statements": [
                        {
                            "id": "statement-1",
                            "property": "property-1",
                            "statement_year": 2026,
                            "period_start": "2026-12-31",
                            "period_end": "2026-01-01",
                        }
                    ]
                },
                "operating_statements[0]",
                "period_end precedes period_start",
            ),
        )

        for label, (addition, path, message) in (
            (label, (addition, path, message))
            for label, addition, path, message in cases
        ):
            with self.subTest(label=label):
                self.assert_issues(
                    {**base, **addition},
                    [ValidationIssue(path, message)],
                )

    def test_structure_rating_system_and_scope_pairs_are_unique_per_collection(self):
        rating = {
            "system": "urn:example:condition",
            "code": "average",
            "scope": "overall",
        }
        profile = {
            "property": {"id": "property-1"},
            "structures": [
                {
                    "id": "structure-1",
                    "property": "property-1",
                    "condition_ratings": [rating, {**rating, "code": "good"}],
                }
            ],
            "property_state_snapshots": [
                {
                    "id": "snapshot-1",
                    "property": "property-1",
                    "as_of_date": "2026-01-01",
                    "structure_states": [
                        {
                            "id": "structure-state-1",
                            "subject": "structure-1",
                            "quality_ratings": [
                                {
                                    "system": "urn:example:quality",
                                    "code": "high",
                                    "scope": "interior",
                                },
                                {
                                    "system": "urn:example:quality",
                                    "code": "average",
                                    "scope": "interior",
                                },
                            ],
                        }
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "property_state_snapshots[0].structure_states[0].quality_ratings[1]",
                    "duplicate rating system and scope in quality_ratings",
                ),
                ValidationIssue(
                    "structures[0].condition_ratings[1]",
                    "duplicate rating system and scope in condition_ratings",
                ),
            ],
        )

    def test_operating_statement_period_endpoints_are_both_present_or_both_absent(self):
        for field in ("period_start", "period_end"):
            with self.subTest(field=field):
                profile = {
                    "property": {"id": "property-1"},
                    "operating_statements": [
                        {
                            "id": "statement-1",
                            "property": "property-1",
                            "statement_year": 2026,
                            field: "2026-01-01",
                        }
                    ],
                }
                self.assert_issues(
                    profile,
                    [
                        ValidationIssue(
                            "operating_statements[0]",
                            "period_start and period_end must be provided together",
                        )
                    ],
                )

    def test_rent_roll_tenant_must_be_a_declared_lessee_when_lease_declares_one(self):
        base = {
            "property": {"id": "property-1"},
            "parties": [
                {"id": "tenant-1", "kind": "organization", "name": "Tenant 1"},
                {"id": "tenant-2", "kind": "organization", "name": "Tenant 2"},
                {"id": "landlord-1", "kind": "organization", "name": "Landlord"},
            ],
        }
        cases = (
            ([{"role": "lessee", "party": "tenant-1"}], "tenant-1", []),
            (
                [{"role": "lessee", "party": "tenant-1"}],
                "tenant-2",
                [
                    ValidationIssue(
                        "rent_rolls[0].lines[0].tenant",
                        "tenant does not match a lessee declared by the referenced lease",
                    )
                ],
            ),
            (
                [
                    {"role": "lessee", "party": "tenant-1"},
                    {"role": "lessee", "party": "tenant-2"},
                ],
                "tenant-2",
                [],
            ),
            ([{"role": "landlord", "party": "landlord-1"}], "tenant-2", []),
        )
        for parties, tenant, expected in cases:
            with self.subTest(parties=parties, tenant=tenant):
                profile = {
                    **base,
                    "leases": [
                        {
                            "id": "lease-1",
                            "property": "property-1",
                            "parties": parties,
                        }
                    ],
                    "rent_rolls": [
                        {
                            "id": "roll-1",
                            "property": "property-1",
                            "as_of_date": "2026-01-01",
                            "lines": [{"lease": "lease-1", "tenant": tenant}],
                        }
                    ],
                }
                self.assert_issues(profile, expected)

    def test_generated_profile_with_default_null_bundles_is_valid(self):
        profile = self.models.PropertyProfile.model_validate(
            {"property": {"id": "p1"}}
        ).model_dump(mode="json")
        self.assertIsNone(profile["addresses"])
        self.assertIsNone(profile["property_state_snapshots"])
        self.assertIsNone(profile["sales"])

        self.assert_issues(profile, [])

    def test_explicit_null_nested_state_collections_are_valid(self):
        profile = self.models.PropertyProfile.model_validate(
            {
                "property": {"id": "p1"},
                "property_state_snapshots": [
                    {
                        "id": "snapshot1",
                        "property": "p1",
                        "as_of_date": "2025-01-01",
                        "site_states": None,
                        "structure_states": None,
                        "space_states": None,
                    }
                ],
            }
        ).model_dump(mode="json")
        snapshot = profile["property_state_snapshots"][0]
        self.assertIsNone(snapshot["site_states"])
        self.assertIsNone(snapshot["structure_states"])
        self.assertIsNone(snapshot["space_states"])

        self.assert_issues(profile, [])

    def test_cli_reports_invalid_json_and_root_shape_without_traceback(self):
        cases = {
            "invalid-json.json": ("{", "invalid JSON"),
            "invalid-root.json": ("[]", "profile root must be a JSON object"),
        }
        with tempfile.TemporaryDirectory() as directory:
            for filename, (contents, expected) in cases.items():
                with self.subTest(filename=filename):
                    path = Path(directory) / filename
                    path.write_text(contents, encoding="utf-8")
                    stderr = io.StringIO()
                    with patch("sys.argv", ["profile_validation.py", str(path)]):
                        with redirect_stderr(stderr):
                            status = main()

                    self.assertEqual(1, status)
                    self.assertIn(expected, stderr.getvalue())
                    self.assertNotIn("Traceback", stderr.getvalue())

    def test_cli_accepts_generated_profile_with_default_null_bundles(self):
        profile = self.models.PropertyProfile.model_validate(
            {"property": {"id": "p1"}}
        ).model_dump(mode="json")
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "profile.json"
            path.write_text(json.dumps(profile), encoding="utf-8")
            stderr = io.StringIO()
            with patch("sys.argv", ["profile_validation.py", str(path)]):
                with redirect_stderr(stderr):
                    status = main()

        self.assertEqual(0, status)
        self.assertEqual("", stderr.getvalue())

    def test_property_address_dates_and_references_are_checked(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [{"id": "a1", "country": "JP"}],
            "property_addresses": [
                {
                    "id": "pa1",
                    "property": "p1",
                    "address": "missing",
                    "valid_from": "2025-01-02",
                    "valid_to": "2025-01-01",
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "property_addresses[0]", "valid_to precedes valid_from"
                ),
                ValidationIssue(
                    "property_addresses[0].address",
                    "address reference does not resolve",
                ),
            ],
        )

    def test_property_address_property_must_match_profile_property(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [{"id": "a1", "country": "RU"}],
            "property_addresses": [
                {"id": "pa1", "property": "p2", "address": "a1"}
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "property_addresses[0].property",
                    "property reference does not resolve",
                )
            ],
        )

    def test_party_and_ownership_address_references_must_resolve(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [{"id": "a1", "country": "JP"}],
            "parties": [
                {
                    "id": "party1",
                    "kind": "person",
                    "name": "山田 太郎",
                    "addresses": [{"address": "missing-party-address"}],
                }
            ],
            "ownership": [
                {
                    "id": "ownership1",
                    "property": "p1",
                    "mailing_address": "missing-mailing-address",
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "ownership[0].mailing_address",
                    "address reference does not resolve",
                ),
                ValidationIssue(
                    "parties[0].addresses[0].address",
                    "address reference does not resolve",
                ),
            ],
        )

    def test_duplicate_address_ids_do_not_cascade_into_unresolved_errors(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [
                {"id": "a1", "country": "CA"},
                {"id": "a1", "country": "CA"},
            ],
            "property_addresses": [
                {"id": "pa1", "property": "p1", "address": "a1"}
            ],
            "parties": [
                {
                    "id": "party1",
                    "kind": "organization",
                    "name": "Example Organization",
                    "addresses": [{"address": "a1"}],
                }
            ],
            "ownership": [
                {"id": "ownership1", "property": "p1", "mailing_address": "a1"}
            ],
        }

        self.assert_issues(
            profile,
            [ValidationIssue("addresses[1].id", "duplicate address id")],
        )

    def test_legal_form_is_invalid_for_a_person_party(self):
        profile = {
            "property": {"id": "p1"},
            "parties": [
                {
                    "id": "party1",
                    "kind": "person",
                    "name": "山田 太郎",
                    "legal_form": {
                        "system": "urn:producer.example:legal-form",
                        "code": "example-form",
                    },
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "parties[0].legal_form",
                    "legal_form is only valid for an organization Party",
                )
            ],
        )

    def test_tax_line_item_jurisdiction_must_resolve(self):
        profile = {
            "property": {"id": "p1"},
            "jurisdictions": [
                {
                    "id": "jurisdiction-1",
                    "country": "CA",
                    "name": "Example District",
                    "kind": "taxing_district",
                }
            ],
            "parcels": [
                {
                    "id": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "parcel_number": "P-1",
                }
            ],
            "tax_bills": [
                {
                    "id": "tax-bill-1",
                    "parcel": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "tax_year": 2026,
                    "line_items": [{"jurisdiction": "jurisdiction-missing"}],
                }
            ],
        }
        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "tax_bills[0].line_items[0].jurisdiction",
                    "jurisdiction reference does not resolve",
                )
            ],
        )

    def test_duplicate_jurisdiction_ids_do_not_cascade_into_tax_line_errors(self):
        profile = {
            "property": {"id": "p1"},
            "jurisdictions": [
                {
                    "id": "jurisdiction-1",
                    "country": "JP",
                    "name": "District One",
                    "kind": "taxing_district",
                },
                {
                    "id": "jurisdiction-1",
                    "country": "JP",
                    "name": "District Duplicate",
                    "kind": "taxing_district",
                },
            ],
            "parcels": [
                {
                    "id": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "parcel_number": "P-1",
                }
            ],
            "tax_bills": [
                {
                    "id": "tax-bill-1",
                    "parcel": "parcel-1",
                    "jurisdiction": "jurisdiction-1",
                    "tax_year": 2026,
                    "line_items": [{"jurisdiction": "jurisdiction-1"}],
                }
            ],
        }
        self.assert_issues(
            profile,
            [ValidationIssue("jurisdictions[1].id", "duplicate jurisdiction id")],
        )

    def test_primary_address_intervals_for_the_same_role_must_not_overlap(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [
                {"id": "a1", "country": "CA"},
                {"id": "a2", "country": "CA"},
            ],
            "property_addresses": [
                {
                    "id": "pa1",
                    "property": "p1",
                    "address": "a1",
                    "role": {"system": "urn:roles", "code": "situs"},
                    "is_primary": True,
                    "valid_from": "2024-01-01",
                    "valid_to": "2024-12-31",
                },
                {
                    "id": "pa2",
                    "property": "p1",
                    "address": "a2",
                    "role": {"system": "urn:roles", "code": "situs"},
                    "is_primary": True,
                    "valid_from": "2024-12-31",
                },
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "property_addresses[1]",
                    "primary address validity overlaps for role situs",
                )
            ],
        )

    def test_nonoverlapping_or_different_role_primary_addresses_are_valid(self):
        profile = {
            "property": {"id": "p1"},
            "addresses": [
                {"id": "a1", "country": "CA"},
                {"id": "a2", "country": "CA"},
                {"id": "a3", "country": "CA"},
            ],
            "property_addresses": [
                {
                    "id": "pa1",
                    "property": "p1",
                    "address": "a1",
                    "role": {"code": "situs"},
                    "is_primary": True,
                    "valid_to": "2024-12-31",
                },
                {
                    "id": "pa2",
                    "property": "p1",
                    "address": "a2",
                    "role": {"code": "situs"},
                    "is_primary": True,
                    "valid_from": "2025-01-01",
                },
                {
                    "id": "pa3",
                    "property": "p1",
                    "address": "a3",
                    "role": {"code": "entrance"},
                    "is_primary": True,
                },
            ],
        }

        self.assert_issues(profile, [])

    def test_snapshot_property_and_state_subject_references_must_resolve(self):
        base_profile = {
            "property": {"id": "p1"},
            "site": {"id": "site1", "property": "p1"},
            "structures": [{"id": "structure1", "property": "p1"}],
            "spaces": [
                {
                    "id": "space1",
                    "property": "p1",
                    "structure": "structure1",
                    "space_identifier": "101",
                }
            ],
        }
        cases = {
            "property_state": (
                {"id": "ps1", "subject": "missing"},
                "property_state.subject",
                "property reference does not resolve",
            ),
            "site_states": (
                [{"id": "ss1", "subject": "missing"}],
                "site_states[0].subject",
                "site reference does not resolve",
            ),
            "structure_states": (
                [{"id": "sts1", "subject": "missing"}],
                "structure_states[0].subject",
                "structure reference does not resolve",
            ),
            "space_states": (
                [{"id": "sps1", "subject": "missing"}],
                "space_states[0].subject",
                "space reference does not resolve",
            ),
        }

        for group, (state_payload, relative_path, message) in cases.items():
            with self.subTest(group=group):
                snapshot = {
                    "id": "snapshot1",
                    "property": "p1",
                    "as_of_date": "2025-01-01",
                    group: state_payload,
                }
                self.assert_issues(
                    {**base_profile, "property_state_snapshots": [snapshot]},
                    [
                        ValidationIssue(
                            f"property_state_snapshots[0].{relative_path}",
                            message,
                        )
                    ],
                )

        snapshot = {
            "id": "snapshot1",
            "property": "other-property",
            "as_of_date": "2025-01-01",
        }
        self.assert_issues(
            {**base_profile, "property_state_snapshots": [snapshot]},
            [
                ValidationIssue(
                    "property_state_snapshots[0].property",
                    "property reference does not resolve",
                )
            ],
        )

    def test_all_event_snapshot_references_must_resolve(self):
        required_fields = {
            "sales": {"close_date": "2025-01-01"},
            "leases": {},
            "listings": {"kind": "for_sale"},
            "valuations": {
                "kind": "appraisal",
                "value": {"amount": "1", "currency": "CAD"},
                "as_of_date": "2025-01-01",
            },
        }
        for collection, fields in required_fields.items():
            with self.subTest(collection=collection):
                profile = {
                    "property": {"id": "p1"},
                    collection: [
                        {
                            "id": "event1",
                            "property": "p1",
                            "property_state": "missing",
                            **fields,
                        }
                    ],
                }
                self.assert_issues(
                    profile,
                    [
                        ValidationIssue(
                            f"{collection}[0].property_state",
                            "property_state reference does not resolve",
                        )
                    ],
                )

    def test_duplicate_reference_target_ids_are_rejected_without_cascades(self):
        cases = {
            "address": (
                {
                    "property": {"id": "p1"},
                    "addresses": [
                        {"id": "a1", "country": "CA"},
                        {"id": "a1", "country": "JP"},
                    ],
                    "property_addresses": [
                        {"id": "pa1", "property": "p1", "address": "a1"}
                    ],
                },
                ValidationIssue("addresses[1].id", "duplicate address id"),
            ),
            "property_state_snapshot": (
                {
                    "property": {"id": "p1"},
                    "property_state_snapshots": [
                        {
                            "id": "snapshot1",
                            "property": "p1",
                            "as_of_date": "2024-01-01",
                        },
                        {
                            "id": "snapshot1",
                            "property": "p1",
                            "as_of_date": "2025-01-01",
                        },
                    ],
                    "sales": [
                        {
                            "id": "sale1",
                            "property": "p1",
                            "close_date": "2025-01-01",
                            "property_state": "snapshot1",
                        }
                    ],
                },
                ValidationIssue(
                    "property_state_snapshots[1].id",
                    "duplicate property_state_snapshot id",
                ),
            ),
            "structure": (
                {
                    "property": {"id": "p1"},
                    "structures": [
                        {"id": "structure1", "property": "p1"},
                        {"id": "structure1", "property": "p1"},
                    ],
                    "property_state_snapshots": [
                        {
                            "id": "snapshot1",
                            "property": "p1",
                            "as_of_date": "2025-01-01",
                            "structure_states": [
                                {"id": "state1", "subject": "structure1"}
                            ],
                        }
                    ],
                },
                ValidationIssue("structures[1].id", "duplicate structure id"),
            ),
            "space": (
                {
                    "property": {"id": "p1"},
                    "spaces": [
                        {
                            "id": "space1",
                            "property": "p1",
                            "space_identifier": "101",
                        },
                        {
                            "id": "space1",
                            "property": "p1",
                            "space_identifier": "102",
                        },
                    ],
                    "property_state_snapshots": [
                        {
                            "id": "snapshot1",
                            "property": "p1",
                            "as_of_date": "2025-01-01",
                            "space_states": [
                                {"id": "state1", "subject": "space1"}
                            ],
                        }
                    ],
                },
                ValidationIssue("spaces[1].id", "duplicate space id"),
            ),
            "party": (
                {
                    "property": {"id": "p1"},
                    "parties": [
                        {"id": "party1", "kind": "person", "name": "One"},
                        {"id": "party1", "kind": "person", "name": "Two"},
                    ],
                    "rent_rolls": [
                        {
                            "id": "rr1",
                            "property": "p1",
                            "as_of_date": "2026-01-01",
                            "lines": [{"tenant": "party1"}],
                        }
                    ],
                },
                ValidationIssue("parties[1].id", "duplicate party id"),
            ),
            "lease": (
                {
                    "property": {"id": "p1"},
                    "leases": [
                        {"id": "lease1", "property": "p1"},
                        {"id": "lease1", "property": "p1"},
                    ],
                    "rent_rolls": [
                        {
                            "id": "rr1",
                            "property": "p1",
                            "as_of_date": "2026-01-01",
                            "lines": [{"lease": "lease1"}],
                        }
                    ],
                },
                ValidationIssue("leases[1].id", "duplicate lease id"),
            ),
            "rent_roll": (
                {
                    "property": {"id": "p1"},
                    "rent_rolls": [
                        {"id": "rr1", "property": "p1", "as_of_date": "2026-01-01"},
                        {"id": "rr1", "property": "p1", "as_of_date": "2026-02-01"},
                    ],
                },
                ValidationIssue("rent_rolls[1].id", "duplicate rent_roll id"),
            ),
        }

        for target, (profile, expected) in cases.items():
            with self.subTest(target=target):
                self.assert_issues(profile, [expected])

    def test_state_subject_resolution_is_typed(self):
        profile = {
            "property": {"id": "property-id"},
            "site": {"id": "site-id", "property": "property-id"},
            "spaces": [
                {
                    "id": "space-id",
                    "property": "property-id",
                    "space_identifier": "101",
                }
            ],
            "property_state_snapshots": [
                {
                    "id": "snapshot1",
                    "property": "property-id",
                    "as_of_date": "2025-01-01",
                    "property_state": {"id": "property-state1", "subject": "property-id"},
                    "site_states": [{"id": "site-state1", "subject": "site-id"}],
                    "structure_states": [
                        {"id": "structure-state1", "subject": "space-id"}
                    ],
                    "space_states": [{"id": "space-state1", "subject": "space-id"}],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "property_state_snapshots[0].structure_states[0].subject",
                    "structure reference does not resolve",
                )
            ],
        )

    def test_rent_roll_rejects_mixed_currency(self):
        profile = {
            "property": {"id": "p1"},
            "rent_rolls": [
                {
                    "id": "rr1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "total_contract_rent": {
                        "amount": "1000",
                        "currency": "CAD",
                    },
                    "lines": [
                        {
                            "contract_rent": {
                                "amount": "700",
                                "currency": "USD",
                            }
                        }
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "rent_rolls[0]",
                    "all rent roll money must use one currency",
                )
            ],
        )

    def test_rent_roll_header_totals_need_not_sum_to_lines(self):
        profile = {
            "property": {"id": "p1"},
            "rent_rolls": [
                {
                    "id": "rr1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "total_contract_rent": {
                        "amount": "2500",
                        "currency": "CAD",
                    },
                    "lines": [
                        {
                            "contract_rent": {
                                "amount": "700",
                                "currency": "CAD",
                            }
                        },
                        {
                            "contract_rent": {
                                "amount": "800",
                                "currency": "CAD",
                            }
                        },
                    ],
                }
            ],
        }

        self.assert_issues(profile, [])

    def test_rent_roll_example_keeps_authoritative_non_summing_totals(self):
        path = ROOT / "examples" / "PropertyProfile-rent-roll.json"
        self.assertTrue(path.exists(), f"missing rent-roll example: {path}")
        profile = json.loads(
            path.read_text(encoding="utf-8")
        )
        roll = profile["rent_rolls"][0]
        self.assertEqual(3, len(roll["lines"]))
        stated_total = roll["total_contract_rent"]["amount"]
        line_total = sum(
            float(line["contract_rent"]["amount"])
            for line in roll["lines"]
            if line.get("contract_rent")
        )
        self.assertNotEqual(float(stated_total), line_total)
        vacant_lines = [
            line
            for line in roll["lines"]
            if (line.get("occupancy_status") or {}).get("code") == "vacant"
        ]
        occupied_lines = [
            line
            for line in roll["lines"]
            if (line.get("occupancy_status") or {}).get("code") == "occupied"
        ]
        self.assertEqual(roll["occupied_unit_count"], len(occupied_lines))
        self.assertEqual(1, len(vacant_lines))
        self.assertNotIn("tenant", vacant_lines[0])
        self.assertNotIn("contract_rent", vacant_lines[0])
        self.assertIn("market_rent", vacant_lines[0])
        self.assert_issues(profile, [])

    def test_rent_roll_property_and_counts_are_consistent(self):
        profile = {
            "property": {"id": "p1"},
            "rent_rolls": [
                {
                    "id": "rr1",
                    "property": "other-property",
                    "as_of_date": "2026-01-01",
                    "unit_count": 1,
                    "occupied_unit_count": 2,
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "rent_rolls[0]",
                    "occupied_unit_count exceeds unit_count",
                ),
                ValidationIssue(
                    "rent_rolls[0].property",
                    "property reference does not resolve",
                ),
            ],
        )

    def test_optional_rent_roll_references_resolve_by_target_type(self):
        profile = {
            "property": {"id": "p1"},
            "spaces": [
                {
                    "id": "space1",
                    "property": "p1",
                    "space_identifier": "101",
                }
            ],
            "parties": [
                {"id": "party1", "kind": "organization", "name": "Tenant"}
            ],
            "leases": [{"id": "lease1", "property": "p1"}],
            "rent_rolls": [
                {
                    "id": "rr1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "lines": [
                        {
                            "space": "missing-space",
                            # space1 exists, but only in the Space bundle. A
                            # cross-bundle ID must not satisfy a Party ref.
                            "tenant": "space1",
                            "lease": "missing-lease",
                        },
                        {
                            "space": "space1",
                            "tenant": "party1",
                            "lease": "lease1",
                        },
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "rent_rolls[0].lines[0].lease",
                    "lease reference does not resolve",
                ),
                ValidationIssue(
                    "rent_rolls[0].lines[0].space",
                    "space reference does not resolve",
                ),
                ValidationIssue(
                    "rent_rolls[0].lines[0].tenant",
                    "party reference does not resolve",
                ),
            ],
        )

    def test_rent_roll_space_and_lease_relationship_is_consistent(self):
        base = {
            "property": {"id": "p1"},
            "spaces": [
                {"id": "space1", "property": "p1", "space_identifier": "1"},
                {"id": "space2", "property": "p1", "space_identifier": "2"},
            ],
            "leases": [
                {"id": "lease1", "property": "p1", "space": "space2"},
            ],
        }
        line = {"space": "space1", "lease": "lease1"}
        expected = ValidationIssue(
            "rent_rolls[0].lines[0].lease",
            "lease space does not match rent roll line space",
        )
        profile = {
            **base,
            "rent_rolls": [
                {
                    "id": "rr1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "lines": [line],
                }
            ],
        }
        self.assert_issues(profile, [expected])

    def test_every_actor_reference_resolves_to_the_canonical_party_bundle(self):
        profile = {
            "property": {"id": "p1"},
            "parties": [
                {"id": "party1", "kind": "organization", "name": "Actor"}
            ],
            "associations": [
                {"id": "association1", "property": "p1", "party": "party1"}
            ],
            "transfers": [
                {
                    "id": "transfer1",
                    "property": "p1",
                    "parties": [{"role": "grantor", "party": "party1"}],
                }
            ],
            "sales": [
                {
                    "id": "sale1",
                    "property": "p1",
                    "parties": [{"role": "buyer", "party": "party1"}],
                }
            ],
            "listings": [
                {
                    "id": "listing1",
                    "property": "p1",
                    "participants": [
                        {"role": "listing_agent", "party": "party1"}
                    ],
                }
            ],
            "leases": [
                {
                    "id": "lease1",
                    "property": "p1",
                    "parties": [{"role": "lessee", "party": "party1"}],
                }
            ],
            "loans": [
                {
                    "id": "loan1",
                    "property": "p1",
                    "parties": [{"role": "lender", "party": "party1"}],
                    "events": [
                        {"event_type": "assignment", "to_party": "party1"}
                    ],
                }
            ],
            "liens": [
                {
                    "id": "lien1",
                    "property": "p1",
                    "parties": [{"role": "claimant", "party": "party1"}],
                }
            ],
            "foreclosure_cases": [
                {
                    "id": "foreclosure1",
                    "property": "p1",
                    "parties": [{"role": "trustee", "party": "party1"}],
                }
            ],
            "permits": [
                {
                    "id": "permit1",
                    "property": "p1",
                    "contractor_party": "party1",
                }
            ],
            "ownership": [
                {
                    "id": "ownership1",
                    "property": "p1",
                    "interests": [{"party": "party1"}],
                }
            ],
            "rent_rolls": [
                {
                    "id": "rent-roll1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "lines": [{"tenant": "party1"}],
                }
            ],
            "valuations": [
                {
                    "id": "valuation1",
                    "property": "p1",
                    "performed_by_party": "party1",
                }
            ],
        }
        self.assert_issues(profile, [])

        profile["parties"] = []
        expected_paths = {
            "associations[0].party",
            "transfers[0].parties[0].party",
            "sales[0].parties[0].party",
            "listings[0].participants[0].party",
            "leases[0].parties[0].party",
            "loans[0].parties[0].party",
            "loans[0].events[0].to_party",
            "liens[0].parties[0].party",
            "foreclosure_cases[0].parties[0].party",
            "permits[0].contractor_party",
            "ownership[0].interests[0].party",
            "rent_rolls[0].lines[0].tenant",
            "valuations[0].performed_by_party",
        }
        self.assertEqual(
            [ValidationIssue(path, "party reference does not resolve") for path in sorted(expected_paths)],
            validate_profile(profile),
        )

    def test_duplicate_party_target_does_not_cascade_into_reference_errors(self):
        profile = {
            "property": {"id": "p1"},
            "parties": [
                {"id": "party1", "kind": "person", "name": "One"},
                {"id": "party1", "kind": "person", "name": "Two"},
            ],
            "permits": [
                {
                    "id": "permit1",
                    "property": "p1",
                    "contractor_party": "party1",
                }
            ],
            "rent_rolls": [
                {
                    "id": "rent-roll1",
                    "property": "p1",
                    "as_of_date": "2026-01-01",
                    "lines": [{"tenant": "party1"}],
                }
            ],
        }
        self.assert_issues(
            profile,
            [ValidationIssue("parties[1].id", "duplicate party id")],
        )

    def test_verification_attribution_resolves_parties_on_all_entity_occurrences(self):
        profile = {
            "property": {
                "id": "p1",
                "verifications": [
                    {"verifier": "missing-property-verifier", "verified_at": "2026-07-18T12:00:00Z"}
                ],
            },
            "parties": [
                {"id": "party1", "kind": "organization", "name": "Verifier"}
            ],
            "transfers": [
                {
                    "id": "transfer1",
                    "property": "p1",
                    "verifications": [
                        {"verifier": "party1", "verified_at": "2026-07-18T12:00:00Z"}
                    ],
                }
            ],
            "artifacts": [
                {
                    "id": "artifact1",
                    "uri": "https://example.org/artifact1",
                    "verifications": [
                        {"verifier": "missing-artifact-verifier", "verified_at": "2026-07-18T12:00:00Z"}
                    ],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "artifacts[0].verifications[0].verifier",
                    "party reference does not resolve",
                ),
                ValidationIssue(
                    "property.verifications[0].verifier",
                    "party reference does not resolve",
                ),
            ],
        )

    def test_artifact_hash_pairing_and_references_are_checked(self):
        profile = {
            "property": {"id": "p1"},
            "artifacts": [
                {
                    "id": "artifact-hash-only",
                    "storage_reference": "hash-only",
                    "content_hash": "abc123",
                },
                {
                    "id": "artifact-scheme-only",
                    "storage_reference": "scheme-only",
                    "hash_scheme": "vendor.example.sha256",
                },
                {
                    "id": "artifact-valid",
                    "storage_reference": "valid",
                    "content_hash": "def456",
                    "hash_scheme": "vendor.example.sha256",
                },
            ],
            "transfers": [
                {
                    "id": "transfer1",
                    "property": "p1",
                    "artifacts": ["artifact-valid", "missing-transfer-artifact"],
                }
            ],
            "valuations": [
                {
                    "id": "valuation1",
                    "property": "p1",
                    "artifacts": ["missing-valuation-artifact"],
                }
            ],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "artifacts[0]", "content_hash requires hash_scheme"
                ),
                ValidationIssue(
                    "artifacts[1]", "content_hash requires hash_scheme"
                ),
                ValidationIssue(
                    "transfers[0].artifacts[1]",
                    "artifact reference does not resolve",
                ),
                ValidationIssue(
                    "valuations[0].artifacts[0]",
                    "artifact reference does not resolve",
                ),
            ],
        )

    def test_artifact_requires_a_nonblank_locator(self):
        profile = {
            "property": {"id": "p1"},
            "artifacts": [{"id": "artifact-without-locator"}],
        }

        self.assert_issues(
            profile,
            [
                ValidationIssue(
                    "artifacts[0]",
                    "artifact requires uri or storage_reference",
                )
            ],
        )

    def test_capture_artifact_refs_resolve_only_against_nested_profile_bundle(self):
        self.assertTrue(hasattr(profile_validation, "validate_capture_envelope"))
        validate_capture = profile_validation.validate_capture_envelope
        cases = {
            "null refs": ({"artifact_refs": None}, []),
            "valid assessor": (
                {
                    "status": "success",
                    "artifact_refs": ["artifact1"],
                    "profile": {
                        "property": {"id": "p1"},
                        "artifacts": [
                            {"id": "artifact1", "uri": "https://example.ca/a.pdf"}
                        ],
                    },
                },
                [],
            ),
            "valid extraction": (
                {
                    "status": "success",
                    "artifact_refs": ["artifact1"],
                    "profile": {
                        "property": {"id": "p1"},
                        "artifacts": [
                            {"id": "artifact1", "storage_reference": "jp/report/1"}
                        ],
                    },
                },
                [],
            ),
            "missing profile": (
                {"artifact_refs": ["artifact1"]},
                [
                    ValidationIssue(
                        "artifact_refs[0]",
                        "artifact reference does not resolve against profile.artifacts",
                    )
                ],
            ),
            "missing bundle": (
                {
                    "artifact_refs": ["artifact1"],
                    "profile": {"property": {"id": "p1"}},
                },
                [
                    ValidationIssue(
                        "artifact_refs[0]",
                        "artifact reference does not resolve against profile.artifacts",
                    )
                ],
            ),
            "missing id": (
                {
                    "artifact_refs": ["missing"],
                    "profile": {
                        "property": {"id": "p1"},
                        "artifacts": [
                            {"id": "artifact1", "uri": "https://example.ru/a.pdf"}
                        ],
                    },
                },
                [
                    ValidationIssue(
                        "artifact_refs[0]",
                        "artifact reference does not resolve against profile.artifacts",
                    )
                ],
            ),
        }
        for label, (envelope, expected) in cases.items():
            with self.subTest(label=label):
                self.assertEqual(expected, validate_capture(envelope))

    def test_capture_cli_selects_capture_semantics_explicitly(self):
        envelope = {"artifact_refs": ["missing"]}
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "capture.json"
            path.write_text(json.dumps(envelope), encoding="utf-8")
            stderr = io.StringIO()
            with patch(
                "sys.argv",
                [
                    "profile_validation.py",
                    "--document-type",
                    "capture-envelope",
                    str(path),
                ],
            ):
                with redirect_stderr(stderr):
                    status = main()

        self.assertEqual(1, status)
        self.assertEqual(
            "artifact_refs[0]: artifact reference does not resolve against profile.artifacts\n",
            stderr.getvalue(),
        )

    def test_every_artifact_reference_host_reports_exact_missing_paths(self):
        cases = (
            ("listings", {"id": "listing1", "property": "p1"}),
            ("permits", {"id": "permit1", "property": "p1"}),
            ("transfers", {"id": "transfer1", "property": "p1"}),
            ("loans", {"id": "loan1", "property": "p1"}),
            ("valuations", {"id": "valuation1", "property": "p1"}),
        )
        for collection, record in cases:
            with self.subTest(collection=collection, refs="missing"):
                profile = {
                    "property": {"id": "p1"},
                    "artifacts": None,
                    collection: [{**record, "artifacts": ["missing"]}],
                }
                self.assert_issues(
                    profile,
                    [
                        ValidationIssue(
                            f"{collection}[0].artifacts[0]",
                            "artifact reference does not resolve",
                        )
                    ],
                )
            with self.subTest(collection=collection, refs="explicit null"):
                profile = {
                    "property": {"id": "p1"},
                    "artifacts": None,
                    collection: [{**record, "artifacts": None}],
                }
                self.assert_issues(profile, [])

    def test_duplicate_artifact_target_does_not_create_ambiguous_resolution(self):
        profile = {
            "property": {"id": "p1"},
            "artifacts": [
                {"id": "artifact1", "storage_reference": "duplicate/1"},
                {"id": "artifact1", "storage_reference": "duplicate/2"},
            ],
            "valuations": [
                {
                    "id": "valuation1",
                    "property": "p1",
                    "artifacts": ["artifact1"],
                }
            ],
        }

        self.assert_issues(
            profile,
            [ValidationIssue("artifacts[1].id", "duplicate artifact id")],
        )

    def test_blank_party_references_are_reported_at_every_actor_path(self):
        profile = {
            "property": {"id": "p1"},
            "associations": [{"party": " "}],
            "transfers": [{"parties": [{"party": " "}]}],
            "sales": [{"parties": [{"party": " "}]}],
            "listings": [{"participants": [{"party": " "}]}],
            "leases": [{"parties": [{"party": " "}]}],
            "loans": [
                {"parties": [{"party": " "}], "events": [{"to_party": " "}]}
            ],
            "liens": [{"parties": [{"party": " "}]}],
            "foreclosure_cases": [{"parties": [{"party": " "}]}],
            "permits": [{"contractor_party": " "}],
            "ownership": [{"interests": [{"party": " "}]}],
            "rent_rolls": [{"property": "p1", "lines": [{"tenant": " "}]}],
            "valuations": [{"performed_by_party": " "}],
        }
        expected_paths = {
            "associations[0].party",
            "transfers[0].parties[0].party",
            "sales[0].parties[0].party",
            "listings[0].participants[0].party",
            "leases[0].parties[0].party",
            "loans[0].parties[0].party",
            "loans[0].events[0].to_party",
            "liens[0].parties[0].party",
            "foreclosure_cases[0].parties[0].party",
            "permits[0].contractor_party",
            "ownership[0].interests[0].party",
            "rent_rolls[0].lines[0].tenant",
            "valuations[0].performed_by_party",
        }
        self.assert_issues(
            profile,
            [
                ValidationIssue(path, "party reference does not resolve")
                for path in sorted(expected_paths)
            ],
        )

    def test_semantic_counterexamples_have_one_stable_issue(self):
        expected = {
            "PropertyProfile-address-hash-without-scheme.json": ValidationIssue(
                "addresses[0]", "address_hash requires address_hash_scheme"
            ),
            "PropertyProfile-overlapping-primary-addresses.json": ValidationIssue(
                "property_addresses[1]",
                "primary address validity overlaps for role situs",
            ),
            "PropertyProfile-overlapping-primary-party-addresses.json": ValidationIssue(
                "parties[0].addresses[1]",
                "primary address validity overlaps for role mailing",
            ),
            "PropertyProfile-reversed-lease-period.json": ValidationIssue(
                "leases[0]",
                "expiration_date precedes commencement_date",
            ),
            "PropertyProfile-unresolved-state-reference.json": ValidationIssue(
                "property_state_snapshots[0].structure_states[0].subject",
                "structure reference does not resolve",
            ),
            "PropertyProfile-rent-roll-mixed-currency.json": ValidationIssue(
                "rent_rolls[0]",
                "all rent roll money must use one currency",
            ),
            "PropertyProfile-unresolved-artifact.json": ValidationIssue(
                "valuations[0].artifacts[0]",
                "artifact reference does not resolve",
            ),
            "PropertyProfile-artifact-without-locator.json": ValidationIssue(
                "artifacts[0]", "artifact requires uri or storage_reference"
            ),
            "PropertyProfile-unresolved-party-address.json": ValidationIssue(
                "parties[0].addresses[0].address",
                "address reference does not resolve",
            ),
            "PropertyProfile-unresolved-ownership-address.json": ValidationIssue(
                "ownership[0].mailing_address",
                "address reference does not resolve",
            ),
            "PropertyProfile-unresolved-tax-line-jurisdiction.json": ValidationIssue(
                "tax_bills[0].line_items[0].jurisdiction",
                "jurisdiction reference does not resolve",
            ),
            "PropertyProfile-jsonld-entity-id-collision.json": ValidationIssue(
                "artifacts[0].id",
                "duplicate Entity id; first declared at property.id",
            ),
            "PropertyProfile-duplicate-rating-system-scope.json": ValidationIssue(
                "structures[0].condition_ratings[1]",
                "duplicate rating system and scope in condition_ratings",
            ),
            "PropertyProfile-duplicate-listing-identifier.json": ValidationIssue(
                "listings[0].identifiers[1]",
                "duplicate listing identifier (scheme, namespace, value)",
            ),
            "PropertyProfile-two-primary-listing-identifiers.json": ValidationIssue(
                "listings[0].identifiers[1]",
                "multiple primary listing identifiers in one namespace",
            ),
            "PropertyProfile-listing-event-effective-at-mismatch.json": ValidationIssue(
                "listings[0].events[0]",
                "effective_at lexical date does not match effective_date",
            ),
            "PropertyProfile-close-price-on-active-event.json": ValidationIssue(
                "listings[0].events[0]",
                "close_price is only valid on a closed event",
            ),
            "PropertyProfile-lease-list-price-without-rent-period.json": ValidationIssue(
                "listings[0].events[0]",
                "for_lease list_price requires rent_period",
            ),
            "PropertyProfile-unresolved-sale-listing-relationship.json": ValidationIssue(
                "sales[0].listings[0].listing",
                "listing reference does not resolve",
            ),
            "PropertyProfile-unresolved-sale-evidence-sale.json": ValidationIssue(
                "sale_evidence[0].sale",
                "sale reference does not resolve",
            ),
            "PropertyProfile-unresolved-sale-evidence-artifact.json": ValidationIssue(
                "sale_evidence[0].artifacts[0]",
                "artifact reference does not resolve",
            ),
        }
        self.assertEqual(
            set(expected),
            {path.name for path in SEMANTIC_COUNTER_EXAMPLES.glob("*.json")},
        )
        for filename, issue in expected.items():
            with self.subTest(filename=filename):
                profile = json.loads(
                    (SEMANTIC_COUNTER_EXAMPLES / filename).read_text(encoding="utf-8")
                )
                self.assert_issues(profile, [issue])

    def test_negative_fixture_census_and_validation_guards_are_pinned(self):
        schema_names = {
            "PropertyProfile-artifact-blank-locator.json",
            "PropertyProfile-blank-entity-id.json",
            "PropertyProfile-blank-party-name.json",
            "PropertyProfile-blank-party-reference.json",
            "PropertyProfile-blank-address-reference.json",
            "PropertyProfile-bad-money-amount.json",
            "PropertyProfile-money-trailing-newline.json",
            "PropertyProfile-money-unicode-digits.json",
            "PropertyProfile-property-address-missing-address.json",
            "PropertyProfile-rating-missing-system.json",
            "PropertyProfile-rent-roll-missing-date.json",
            "PropertyProfile-state-missing-date.json",
            "PropertyProfile-blank-listing-identifier-value.json",
            "PropertyProfile-sale-evidence-missing-provenance.json",
        }
        semantic_names = {
            "PropertyProfile-address-hash-without-scheme.json",
            "PropertyProfile-overlapping-primary-addresses.json",
            "PropertyProfile-overlapping-primary-party-addresses.json",
            "PropertyProfile-reversed-lease-period.json",
            "PropertyProfile-rent-roll-mixed-currency.json",
            "PropertyProfile-unresolved-state-reference.json",
            "PropertyProfile-unresolved-artifact.json",
            "PropertyProfile-artifact-without-locator.json",
            "PropertyProfile-unresolved-party-address.json",
            "PropertyProfile-unresolved-ownership-address.json",
            "PropertyProfile-unresolved-tax-line-jurisdiction.json",
            "PropertyProfile-jsonld-entity-id-collision.json",
            "PropertyProfile-duplicate-rating-system-scope.json",
            "PropertyProfile-duplicate-listing-identifier.json",
            "PropertyProfile-two-primary-listing-identifiers.json",
            "PropertyProfile-listing-event-effective-at-mismatch.json",
            "PropertyProfile-close-price-on-active-event.json",
            "PropertyProfile-lease-list-price-without-rent-period.json",
            "PropertyProfile-unresolved-sale-listing-relationship.json",
            "PropertyProfile-unresolved-sale-evidence-sale.json",
            "PropertyProfile-unresolved-sale-evidence-artifact.json",
        }
        self.assertEqual(
            schema_names,
            {
                path.name
                for path in (ROOT / "counter_examples" / "schema").glob("*.json")
            },
        )
        self.assertEqual(
            semantic_names,
            {
                path.name
                for path in (ROOT / "counter_examples" / "semantic").glob("*.json")
            },
        )

        justfile = (ROOT / "justfile").read_text(encoding="utf-8")
        self.assertIn(
            "schema_counterexamples=(counter_examples/schema/*.json)", justfile
        )
        self.assertIn(
            '${schema_counterexamples[0]}', justfile
        )
        self.assertIn(
            "semantic_counterexamples=(counter_examples/semantic/*.json)", justfile
        )
        self.assertIn(
            '${semantic_counterexamples[0]}', justfile
        )


def _listing_profile(identifiers):
    return {
        "property": {"id": "prop-1"},
        "listings": [
            {
                "id": "lst-1",
                "property": "prop-1",
                "offering_type": "for_sale",
                "identifiers": identifiers,
            }
        ],
    }


class ListingIdentifierRules(unittest.TestCase):
    def test_duplicate_identifier_rejected(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1"},
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1"},
                ]
            )
        )
        self.assertTrue(
            any("duplicate listing identifier" in i.message for i in issues), issues
        )

    def test_two_primaries_in_namespace_rejected(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1", "is_primary": True},
                    {"scheme": "listing_id", "namespace": "mls-x", "value": "12345", "is_primary": True},
                ]
            )
        )
        self.assertTrue(
            any("multiple primary" in i.message for i in issues), issues
        )

    def test_distinct_namespaces_each_primary_ok(self):
        issues = validate_profile(
            _listing_profile(
                [
                    {"scheme": "listing_key", "namespace": "mls-x", "value": "K1", "is_primary": True},
                    {"scheme": "listing_key", "namespace": "mls-y", "value": "K9", "is_primary": True},
                ]
            )
        )
        self.assertFalse(
            any("identifier" in i.message for i in issues), issues
        )


class IdentifierBundleRules(unittest.TestCase):
    def _profile(self, collection, rows):
        profile = {"property": {"id": "prop-1"}, collection: rows}
        if collection == "parcel_identifiers":
            profile["jurisdictions"] = [{"id": "jur-1", "country": "US"}]
            profile["parcels"] = [
                {"id": "parcel-1", "jurisdiction": "jur-1", "parcel_number": "P-1"}
            ]
        return profile

    def test_duplicate_property_identifier_rejected(self):
        row = {"id": "ident-1", "property": "prop-1", "scheme": "tax_account", "value": "R1"}
        issues = validate_profile(
            self._profile("identifiers", [row, {**row, "id": "ident-2"}])
        )
        self.assertEqual(
            [
                ValidationIssue(
                    "identifiers[1]",
                    "duplicate property identifier (scheme, namespace, value)",
                )
            ],
            issues,
        )

    def test_duplicate_parcel_identifier_rejected(self):
        row = {"id": "pident-1", "parcel": "parcel-1", "scheme": "upi", "value": "urn:reso:upi:2.0:US:20091:P-1"}
        issues = validate_profile(
            self._profile("parcel_identifiers", [row, {**row, "id": "pident-2"}])
        )
        self.assertEqual(
            [
                ValidationIssue(
                    "parcel_identifiers[1]",
                    "duplicate parcel identifier (scheme, namespace, value)",
                )
            ],
            issues,
        )

    def test_distinct_identifiers_ok(self):
        issues = validate_profile(
            self._profile(
                "parcel_identifiers",
                [
                    {"id": "pident-1", "parcel": "parcel-1", "scheme": "upi", "value": "urn:reso:upi:2.0:US:20091:P-1"},
                    {"id": "pident-2", "parcel": "parcel-1", "scheme": "assessor_account", "value": "R96142"},
                ],
            )
        )
        self.assertEqual([], issues)


def _event_profile(offering_type, event):
    return {
        "property": {"id": "prop-1"},
        "listings": [
            {
                "id": "lst-1",
                "property": "prop-1",
                "offering_type": offering_type,
                "events": [event],
            }
        ],
    }


class ListingEventRules(unittest.TestCase):
    def test_effective_at_must_match_effective_date(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "effective_at": "2026-03-02T09:00:00-05:00",
                    "event_type": "listed",
                },
            )
        )
        self.assertTrue(
            any("effective_at" in i.message for i in issues), issues
        )

    def test_close_price_only_on_closed_event(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "event_type": "price_change",
                    "close_price": {"amount": "400000", "currency": "USD"},
                },
            )
        )
        self.assertTrue(
            any("close_price" in i.message for i in issues), issues
        )

    def test_lease_list_price_requires_rent_period(self):
        issues = validate_profile(
            _event_profile(
                "for_lease",
                {
                    "effective_date": "2026-03-01",
                    "event_type": "listed",
                    "list_price": {"amount": "2500", "currency": "USD"},
                },
            )
        )
        self.assertTrue(
            any("rent_period" in i.message for i in issues), issues
        )

    def test_valid_closed_event_passes(self):
        issues = validate_profile(
            _event_profile(
                "for_sale",
                {
                    "effective_date": "2026-03-01",
                    "effective_at": "2026-03-01T16:30:00-05:00",
                    "event_type": "closed",
                    "status": "sold",
                    "close_price": {"amount": "400000", "currency": "USD"},
                },
            )
        )
        self.assertEqual(issues, [])


class SaleListingRelationshipRules(unittest.TestCase):
    @staticmethod
    def _profile(listing_property="prop-1"):
        return {
            "property": {"id": "prop-1"},
            "listings": [
                {"id": "lst-1", "property": listing_property, "offering_type": "for_sale"}
            ],
            "sales": [
                {
                    "id": "sale-1",
                    "property": "prop-1",
                    "close_date": "2026-04-01",
                    "listings": [
                        {"listing": "lst-1", "relationship_type": "resulted_in_sale"}
                    ],
                }
            ],
        }

    def test_resolved_same_property_ok(self):
        self.assertEqual(validate_profile(self._profile()), [])

    def test_unresolved_listing_reference(self):
        profile = self._profile()
        profile["sales"][0]["listings"][0]["listing"] = "lst-missing"
        issues = validate_profile(profile)
        self.assertTrue(
            any("does not resolve" in i.message for i in issues), issues
        )

    def test_cross_property_listing_rejected(self):
        profile = self._profile(listing_property="prop-other")
        # prop-other must exist somewhere for the listing itself to resolve;
        # the point is the sale and listing disagree about the property.
        issues = validate_profile(profile)
        self.assertTrue(
            any("different property" in i.message for i in issues), issues
        )


class SaleEvidenceRules(unittest.TestCase):
    @staticmethod
    def _profile():
        return {
            "property": {"id": "prop-1"},
            "sales": [
                {"id": "sale-1", "property": "prop-1", "close_date": "2026-04-01"}
            ],
            "sale_evidence": [
                {
                    "id": "ev-1",
                    "sale": "sale-1",
                    "close_date": "2026-04-01",
                    "close_price": {"amount": "400000", "currency": "USD"},
                    "verification_method": {"code": "mls_record"},
                    "provenance": {"provider": "MLS X"},
                }
            ],
        }

    def test_valid_evidence_passes(self):
        self.assertEqual(validate_profile(self._profile()), [])

    def test_unresolved_sale_rejected(self):
        profile = self._profile()
        profile["sale_evidence"][0]["sale"] = "sale-missing"
        issues = validate_profile(profile)
        self.assertTrue(
            any("does not resolve" in i.message for i in issues), issues
        )

    def test_cross_property_listing_rejected(self):
        profile = self._profile()
        profile["listings"] = [
            {"id": "lst-1", "property": "prop-other", "offering_type": "for_sale"}
        ]
        profile["sale_evidence"][0]["listing"] = "lst-1"
        # prop-other need not exist as its own Property entity; the listing's
        # own property reference may also fail to resolve, but the point of
        # this test is that the sale and listing disagree about the property.
        issues = validate_profile(profile)
        self.assertTrue(
            any(
                "listing refers to a different property than the evidenced sale"
                in i.message
                for i in issues
            ),
            issues,
        )

    def test_cross_property_transfer_rejected(self):
        profile = self._profile()
        profile["transfers"] = [
            {
                "id": "trn-1",
                "property": "prop-other",
                "transfer_type": "warranty_deed",
            }
        ]
        profile["sale_evidence"][0]["transfer"] = "trn-1"
        issues = validate_profile(profile)
        self.assertTrue(
            any(
                "transfer refers to a different property than the evidenced sale"
                in i.message
                for i in issues
            ),
            issues,
        )


if __name__ == "__main__":
    unittest.main()
