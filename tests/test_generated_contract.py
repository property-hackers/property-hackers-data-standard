"""Cross-generator conformance tests for committed PHDS artifacts."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import unittest

from jsonschema import Draft202012Validator
from pydantic import ValidationError


ROOT = Path(__file__).resolve().parents[1]
GENERATED = ROOT / "schema" / "generated"

def _load_pydantic_module():
    path = GENERATED / "phds_pydantic.py"
    spec = importlib.util.spec_from_file_location("phds_generated", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load generated module: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class GeneratedContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.schema = json.loads(
            (GENERATED / "phds.schema.json").read_text(encoding="utf-8")
        )
        cls.profile_validator = Draft202012Validator(cls.schema)
        cls.money_validator = Draft202012Validator(
            {"$ref": "#/$defs/Money", "$defs": cls.schema["$defs"]}
        )
        cls.assessor_validator = Draft202012Validator(
            {"$ref": "#/$defs/AssessorObservation", "$defs": cls.schema["$defs"]}
        )
        cls.models = _load_pydantic_module()

    def test_documented_assessor_statuses_are_accepted(self):
        for status in (
            "success",
            "not_found",
            "timeout",
            "api_error",
            "parse_error",
            "invalid_address",
            "ambiguous",
        ):
            payload = {"status": status, "provenance": {}}
            with self.subTest(status=status, validator="jsonschema"):
                self.assertEqual([], list(self.assessor_validator.iter_errors(payload)))
            with self.subTest(status=status, validator="pydantic"):
                self.models.AssessorObservation.model_validate(payload)

    def _extraction_validator(self):
        return Draft202012Validator(
            {"$ref": "#/$defs/ExtractionObservation", "$defs": self.schema["$defs"]}
        )

    def _definition_validator(self, name):
        return Draft202012Validator(
            {"$ref": f"#/$defs/{name}", "$defs": self.schema["$defs"]}
        )

    def _class_reference(self, property_schema):
        if "$ref" in property_schema:
            return property_schema["$ref"]

        branches = property_schema.get("anyOf", [])
        null_branches = [branch for branch in branches if branch == {"type": "null"}]
        non_null_branches = [branch for branch in branches if branch != {"type": "null"}]
        self.assertEqual([{"type": "null"}], null_branches)
        self.assertEqual(1, len(non_null_branches))
        self.assertIn("$ref", non_null_branches[0])
        return non_null_branches[0]["$ref"]

    def test_rating_requires_system_and_code_in_both_validators(self):
        validator = self._definition_validator("Rating")
        valid = {"system": "urn:example:condition", "code": "average"}
        self.assertEqual([], list(validator.iter_errors(valid)))
        self.models.Rating.model_validate(valid)
        for invalid in (
            {},
            {"code": "average"},
            {"system": "urn:example:condition"},
            {"system": "", "code": "average"},
            {"system": "\ufeffurn:example:condition", "code": "average"},
            {"system": "urn:example:condition ", "code": "average"},
            {"system": "urn:example:condition", "code": ""},
            {"system": "urn:example:condition", "code": " average"},
            {"system": "urn:example:condition", "code": "average\u3000"},
        ):
            self.assertTrue(list(validator.iter_errors(invalid)))
            with self.assertRaises(ValidationError):
                self.models.Rating.model_validate(invalid)

    def test_classification_requires_trimmed_system_and_code(self):
        validator = self._definition_validator("Classification")
        valid = {"system": "urn:example:party-class", "code": "bank"}
        self.assertEqual([], list(validator.iter_errors(valid)))
        self.models.Classification.model_validate(valid)
        for invalid in (
            {},
            {"code": "bank"},
            {"system": "urn:example:party-class"},
            {"system": "", "code": "bank"},
            {"system": "\ufeffurn:example:party-class", "code": "bank"},
            {"system": "urn:example:party-class ", "code": "bank"},
            {"system": "urn:example:party-class", "code": ""},
            {"system": "urn:example:party-class", "code": " bank"},
            {"system": "urn:example:party-class", "code": "bank\u3000"},
        ):
            with self.subTest(invalid=invalid):
                self.assertTrue(list(validator.iter_errors(invalid)))
                with self.assertRaises(ValidationError):
                    self.models.Classification.model_validate(invalid)

    def test_structure_ratings_and_market_classification_are_typed(self):
        structure = self.schema["$defs"]["Structure"]["properties"]
        commercial = self.schema["$defs"]["CommercialDetails"]["properties"]
        for field in ("condition_ratings", "quality_ratings"):
            collection = structure[field]
            self.assertEqual(["array", "null"], collection["type"])
            self.assertEqual("#/$defs/Rating", collection["items"]["$ref"])
        self.assertEqual(
            "#/$defs/Rating",
            self._class_reference(commercial["market_classification"]),
        )

    def test_property_addresses_are_typed_temporal_associations(self):
        property_fields = self.schema["$defs"]["Property"]["properties"]
        profile_fields = self.schema["$defs"]["PropertyProfile"]["properties"]
        association_fields = self.schema["$defs"]["PropertyAddress"]["properties"]
        self.assertNotIn("situs_address", property_fields)
        self.assertIn("property_addresses", profile_fields)
        self.assertEqual(
            {"property", "address"},
            set(self.schema["$defs"]["PropertyAddress"]["required"])
            & {"property", "address"},
        )
        self.assertIn("role", association_fields)
        self.assertIn("valid_from", association_fields)
        self.assertIn("valid_to", association_fields)

    def test_address_metadata_is_owned_by_concrete_associations(self):
        mixin_fields = self.schema["$defs"]["AddressAssociation"]["properties"]
        property_fields = self.schema["$defs"]["PropertyAddress"]["properties"]
        party_fields = self.schema["$defs"]["PartyAddress"]["properties"]
        for field in ("extras", "provenance"):
            with self.subTest(field=field):
                self.assertNotIn(field, mixin_fields)
                self.assertIn(field, property_fields)
                self.assertIn(field, party_fields)

    def test_address_hash_is_optional_and_scheme_is_available(self):
        validator = self._definition_validator("Address")
        payload = {"id": "address-1", "country": "CA"}
        self.assertEqual([], list(validator.iter_errors(payload)))
        self.models.Address.model_validate(payload)
        properties = self.schema["$defs"]["Address"]["properties"]
        self.assertIn("address_hash_scheme", properties)
        self.assertNotIn("address_hash", self.schema["$defs"]["Address"]["required"])

    def test_address_hash_pair_rejects_blank_or_untrimmed_values(self):
        validator = self._definition_validator("Address")
        for field in ("address_hash", "address_hash_scheme"):
            for invalid in ("", " ", " leading", "trailing ", "\ufeffvalue"):
                with self.subTest(field=field, invalid=repr(invalid)):
                    payload = {
                        "id": "address-1",
                        "country": "CA",
                        "address_hash": "hash-value",
                        "address_hash_scheme": "producer.scheme",
                        field: invalid,
                    }
                    self.assertTrue(list(validator.iter_errors(payload)))
                    with self.assertRaises(ValidationError):
                        self.models.Address.model_validate(payload)

    def test_physical_state_classes_are_sparse_identified_assertions(self):
        for current, state in (
            ("Property", "PropertyState"),
            ("Site", "SiteState"),
            ("Structure", "StructureState"),
            ("Space", "SpaceState"),
        ):
            with self.subTest(current=current, state=state):
                state_def = self.schema["$defs"][state]
                self.assertIn("id", state_def["required"])
                self.assertIn("subject", state_def["required"])
                self.assertNotEqual(current, state)

                # A state is a sparse assertion: identity plus its stable
                # subject is valid without copying current physical facts.
                payload = {"id": f"{state.lower()}-1", "subject": "subject-1"}
                self.assertEqual(
                    [],
                    list(self._definition_validator(state).iter_errors(payload)),
                )
                getattr(self.models, state).model_validate(payload)

    def test_physical_fact_mixins_are_shared_across_generated_languages(self):
        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        rust = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        rust_structs = {
            name: body
            for name, body in re.findall(
                r"pub struct (\w+) \{(.*?)\n\}", rust, re.DOTALL
            )
        }
        for current, facts, state, identity_fields in (
            ("Property", "PropertyFacts", "PropertyState", set()),
            ("Site", "SiteFacts", "SiteState", {"property"}),
            ("Structure", "StructureFacts", "StructureState", {"property"}),
            (
                "Space",
                "SpaceFacts",
                "SpaceState",
                {"property", "structure", "space_identifier"},
            ),
        ):
            with self.subTest(current=current, facts=facts, state=state):
                fact_fields = set(self.schema["$defs"][facts]["properties"])
                current_fields = set(self.schema["$defs"][current]["properties"])
                state_fields = set(self.schema["$defs"][state]["properties"])
                self.assertTrue(fact_fields)
                self.assertLessEqual(fact_fields, current_fields)
                self.assertLessEqual(fact_fields, state_fields)
                self.assertEqual(
                    fact_fields,
                    set(getattr(self.models, facts).model_fields),
                )
                self.assertLessEqual(
                    fact_fields,
                    set(getattr(self.models, current).model_fields),
                )
                self.assertLessEqual(
                    fact_fields,
                    set(getattr(self.models, state).model_fields),
                )
                self.assertIn(
                    f"export interface {current} extends Entity, {facts}", ts
                )
                self.assertIn(
                    f"export interface {state} extends Entity, {facts}", ts
                )
                for field in fact_fields:
                    self.assertIn(f"pub {field}:", rust_structs[current])
                    self.assertIn(f"pub {field}:", rust_structs[state])
                self.assertLessEqual(identity_fields, current_fields - fact_fields)
                self.assertNotIn("subject", current_fields)
                self.assertIn("subject", state_fields - fact_fields)

    def test_detail_descriptions_distinguish_current_and_historical_contexts(self):
        for detail in ("ResidentialDetails", "CommercialDetails"):
            with self.subTest(detail=detail):
                description = self.schema["$defs"][detail]["description"]
                self.assertIn("observation-derived current state", description)
                self.assertIn("PropertyStateSnapshot.as_of_date", description)
                self.assertIn(
                    "provenance.retrieved_at is retrieval metadata", description
                )
                self.assertIn(
                    "separately bundled PropertyStateSnapshot", description
                )
                self.assertNotIn("snapshots on SaleEvent", description)

    def test_events_reference_snapshot_and_profile_bundles_it(self):
        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        for event in ("SaleEvent", "LeaseEvent", "Listing", "Valuation"):
            with self.subTest(event=event):
                # Non-inlined LinkML references serialize as identifier
                # strings in JSON/Pydantic; TypeScript preserves the target
                # identity type in its generated contract.
                property_schema = self.schema["$defs"][event]["properties"]
                self.assertEqual(
                    {"string", "null"}, set(property_schema["property_state"]["type"])
                )
                model = getattr(self.models, event)
                self.assertIn("property_state", model.model_fields)
                interface = re.search(
                    rf"export interface {event}[^{{]*\{{(.*?)\n\}}", ts, re.DOTALL
                )
                self.assertIsNotNone(interface)
                self.assertIn(
                    "property_state?: PropertyStateSnapshotId,", interface.group(1)
                )
        self.assertIn(
            "property_state_snapshots",
            self.schema["$defs"]["PropertyProfile"]["properties"],
        )

    def test_rent_roll_contract_is_dated_and_bundled(self):
        self.assertIn("RentRoll", self.schema["$defs"])
        rent_roll = self.schema["$defs"]["RentRoll"]
        self.assertIn("as_of_date", rent_roll["required"])
        self.assertIn("property", rent_roll["required"])
        self.assertIn(
            "rent_rolls",
            self.schema["$defs"]["PropertyProfile"]["properties"],
        )

    def test_rent_roll_line_uses_typed_optional_references(self):
        self.assertIn("RentRollLine", self.schema["$defs"])
        line_definition = self.schema["$defs"]["RentRollLine"]
        line = line_definition["properties"]
        self.assertIn(
            "tenant is the canonical legal lessee Party",
            line_definition["description"],
        )
        self.assertIn("role: lessee", line_definition["description"])
        for reference in ("space", "tenant", "lease"):
            with self.subTest(reference=reference):
                self.assertIn(reference, line)
                self.assertNotIn(
                    reference,
                    line_definition.get("required", []),
                )
        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        interface = re.search(
            r"export interface RentRollLine \{(.*?)\n\}", ts, re.DOTALL
        )
        self.assertIsNotNone(interface)
        self.assertIn("tenant?: PartyId,", interface.group(1))

    def test_party_and_profile_descriptions_cover_all_referenced_roles(self):
        party_description = self.schema["$defs"]["Party"]["description"]
        profile_description = self.schema["$defs"]["PropertyProfile"]["description"]
        self.assertIn("tenants and lessees", party_description)
        self.assertIn("lenders", party_description)
        self.assertIn("contractors", party_description)
        self.assertIn("associations", party_description)
        self.assertIn("valuation performers", party_description)
        self.assertIn("every Party referenced by any record", profile_description)

    def test_relationship_models_require_canonical_party_references(self):
        transaction_party = self.schema["$defs"]["TransactionParty"]
        association = self.schema["$defs"]["PropertyAssociation"]
        self.assertIn("party", transaction_party["required"])
        self.assertIn("party", association["required"])
        self.assertTrue(self.models.TransactionParty.model_fields["party"].is_required())
        self.assertTrue(
            self.models.PropertyAssociation.model_fields["party"].is_required()
        )

    def test_tax_line_item_uses_an_optional_typed_jurisdiction_reference(self):
        definition = self.schema["$defs"]["TaxLineItem"]
        properties = definition["properties"]
        self.assertIn("jurisdiction", properties)
        self.assertNotIn("jurisdiction", definition.get("required", []))
        payload = {"jurisdiction": "jurisdiction-1"}
        validator = self._definition_validator("TaxLineItem")
        self.assertEqual([], list(validator.iter_errors(payload)))
        self.models.TaxLineItem.model_validate(payload)
        for invalid in ("", " ", " jurisdiction-1", "jurisdiction-1 "):
            with self.subTest(invalid=repr(invalid)):
                invalid_payload = {"jurisdiction": invalid}
                self.assertTrue(list(validator.iter_errors(invalid_payload)))
                with self.assertRaises(ValidationError):
                    self.models.TaxLineItem.model_validate(invalid_payload)

        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        rust = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        ts_item = re.search(
            r"export interface TaxLineItem \{(.*?)\n\}", ts, re.DOTALL
        )
        rust_item = re.search(r"pub struct TaxLineItem \{(.*?)\n\}", rust, re.DOTALL)
        self.assertIsNotNone(ts_item)
        self.assertIsNotNone(rust_item)
        self.assertIn("jurisdiction?: JurisdictionId,", ts_item.group(1))
        self.assertIn("pub jurisdiction: Option<String>", rust_item.group(1))

    def test_office_example_uses_canonical_tax_jurisdiction(self):
        path = ROOT / "examples" / "PropertyProfile-office-mission-ks.json"
        profile = json.loads(path.read_text(encoding="utf-8"))
        jurisdictions = {
            jurisdiction["id"]: jurisdiction
            for jurisdiction in profile["jurisdictions"]
        }
        line = profile["tax_bills"][0]["line_items"][0]
        self.assertEqual("juris-512-unified", line["jurisdiction"])
        self.assertEqual("512 UNIFIED", jurisdictions[line["jurisdiction"]]["name"])

    def test_entity_supports_typed_verification_attribution(self):
        self.assertIn("VerificationAttribution", self.schema["$defs"])
        entity = self.schema["$defs"]["Entity"]["properties"]
        verification = self.schema["$defs"]["VerificationAttribution"]
        self.assertIn("verifications", entity)
        self.assertIn("verifier", verification["required"])
        self.assertIn("verified_at", verification["required"])

    def test_artifacts_are_bundled_and_referenced(self):
        self.assertIn("SourceArtifact", self.schema["$defs"])
        profile = self.schema["$defs"]["PropertyProfile"]["properties"]
        self.assertIn("artifacts", profile)
        for entity in ("Transfer", "Listing", "Valuation", "Permit"):
            with self.subTest(entity=entity):
                self.assertIn(
                    "artifacts", self.schema["$defs"][entity]["properties"]
                )
        for entity in ("Listing", "SaleEvent", "LeaseEvent"):
            with self.subTest(entity=entity):
                remarks = self.schema["$defs"][entity]["properties"].get("remarks")
                self.assertIsNotNone(remarks)
                self.assertIn("source- or vendor-authored", remarks["description"].lower())
        for envelope in ("AssessorObservation", "ExtractionObservation"):
            with self.subTest(envelope=envelope):
                self.assertIn(
                    "artifact_refs", self.schema["$defs"][envelope]["properties"]
                )

        artifact = self.schema["$defs"]["SourceArtifact"]["properties"]
        for field in ("uri", "storage_reference", "content_hash", "hash_scheme"):
            with self.subTest(field=field):
                self.assertIn("pattern", artifact[field])

    def test_rust_wire_contract_enforces_nonblank_trimmed_boundaries(self):
        wire_test = (
            GENERATED / "phds-rust" / "tests" / "wire_format.rs"
        ).read_text(encoding="utf-8")
        self.assertIn(
            "boundary_constrained_strings_reject_invalid_wire_values",
            wire_test,
        )
        self.assertIn("boundary_constrained_strings_accept_international_values", wire_test)
        self.assertIn("address_hash: ' hash'", wire_test)
        self.assertIn("address_hash_scheme: 'producer.scheme '", wire_test)

    def test_artifact_locator_and_hash_boundaries_match_across_contracts(self):
        validator = self._definition_validator("SourceArtifact")
        model = self.models.SourceArtifact
        cases = (
            ("uri", " https://example.jp/report.pdf"),
            ("storage_reference", " "),
            ("content_hash", "hash "),
            ("hash_scheme", "\ufeffsha256"),
        )
        for field, invalid in cases:
            with self.subTest(field=field):
                payload = {
                    "id": "artifact-1",
                    "storage_reference": "資料/評価書",
                    "content_hash": "hash",
                    "hash_scheme": "urn:hash:sha256",
                    field: invalid,
                }
                self.assertTrue(list(validator.iter_errors(payload)))
                with self.assertRaises(ValidationError):
                    model.model_validate(payload)

        valid = {
            "id": "artifact-国際-1",
            "storage_reference": "資料/評価書",
            "content_hash": "ハッシュ値",
            "hash_scheme": "方式.sha256",
        }
        self.assertEqual([], list(validator.iter_errors(valid)))
        model.model_validate(valid)

    def test_generated_artifact_documentation_preserves_scope_and_requirements(self):
        reference_description = (
            "References to SourceArtifact IDs in the nested profile.artifacts "
            "bundle; invalid when profile or profile.artifacts is absent."
        )
        for envelope in ("AssessorObservation", "ExtractionObservation"):
            with self.subTest(envelope=envelope, contract="json-schema"):
                self.assertEqual(
                    reference_description,
                    self.schema["$defs"][envelope]["properties"]["artifact_refs"][
                        "description"
                    ],
                )
            with self.subTest(envelope=envelope, contract="pydantic"):
                self.assertEqual(
                    reference_description,
                    getattr(self.models, envelope).model_fields[
                        "artifact_refs"
                    ].description,
                )

        artifact_requirement = (
            "Semantic validation requires at least one nonblank uri or "
            "storage_reference."
        )
        self.assertIn(
            artifact_requirement,
            self.schema["$defs"]["SourceArtifact"]["description"],
        )
        self.assertIn(artifact_requirement, self.models.SourceArtifact.__doc__)

        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        self.assertEqual(2, ts.count(f"/** {reference_description} */"))
        self.assertIn(artifact_requirement, ts)

    def test_transaction_party_relationship_requires_canonical_party(self):
        validator = self._definition_validator("TransferParty")
        valid = {"role": "grantor", "party": "party-1"}
        invalid = {"role": "grantor"}
        self.assertEqual([], list(validator.iter_errors(valid)))
        self.models.TransferParty.model_validate(valid)
        self.assertTrue(list(validator.iter_errors(invalid)))
        with self.assertRaises(ValidationError):
            self.models.TransferParty.model_validate(invalid)

    def test_canonical_entity_ids_and_party_names_are_nonblank_and_trimmed(self):
        property_validator = self._definition_validator("Property")
        party_validator = self._definition_validator("Party")
        for invalid in (
            "",
            " ",
            "\t",
            " leading",
            "trailing ",
            "\ufeffleading",
            "trailing\u3000",
        ):
            with self.subTest(field="id", invalid=repr(invalid)):
                payload = {"id": invalid}
                self.assertTrue(list(property_validator.iter_errors(payload)))
                with self.assertRaises(ValidationError):
                    self.models.Property.model_validate(payload)
            with self.subTest(field="name", invalid=repr(invalid)):
                payload = {"id": "party-1", "kind": "person", "name": invalid}
                self.assertTrue(list(party_validator.iter_errors(payload)))
                with self.assertRaises(ValidationError):
                    self.models.Party.model_validate(payload)

        for name in ("山田 太郎", "Мария Иванова", "François D’Arcy"):
            with self.subTest(name=name):
                payload = {"id": "party-international", "kind": "person", "name": name}
                self.assertEqual([], list(party_validator.iter_errors(payload)))
                self.models.Party.model_validate(payload)

    def test_every_typed_party_reference_rejects_blank_or_untrimmed_values(self):
        cases = {
            "TransactionParty": ({"role": "participant"}, "party"),
            "OwnershipInterest": ({}, "party"),
            "PropertyAssociation": (
                {"id": "association-1", "property": "property-1"},
                "party",
            ),
            "LoanEvent": ({"event_kind": "assignment"}, "to_party"),
            "Permit": (
                {"id": "permit-1", "property": "property-1"},
                "contractor_party",
            ),
            "RentRollLine": ({}, "tenant"),
            "Valuation": (
                {
                    "id": "valuation-1",
                    "property": "property-1",
                    "kind": "appraisal",
                    "value": {"amount": "100", "currency": "USD"},
                    "as_of_date": "2026-01-01",
                },
                "performed_by_party",
            ),
            "VerificationAttribution": (
                {"verified_at": "2026-07-18T12:00:00Z"},
                "verifier",
            ),
        }
        self.assertEqual(8, len(cases))
        for class_name, (base, field) in cases.items():
            validator = self._definition_validator(class_name)
            model = getattr(self.models, class_name)
            valid = {**base, field: "party-1"}
            self.assertEqual([], list(validator.iter_errors(valid)))
            model.model_validate(valid)
            for invalid in ("", " ", " party-1", "party-1 ", "\ufeffparty-1"):
                with self.subTest(
                    class_name=class_name, field=field, invalid=repr(invalid)
                ):
                    payload = {**base, field: invalid}
                    self.assertTrue(list(validator.iter_errors(payload)))
                    with self.assertRaises(ValidationError):
                        model.model_validate(payload)

    def test_party_classifications_are_typed_across_generated_contracts(self):
        party = self.schema["$defs"]["Party"]
        classifications = party["properties"]["classifications"]
        self.assertEqual("array", classifications["type"][0])
        self.assertEqual("#/$defs/Classification", classifications["items"]["$ref"])
        payload = {
            "id": "party-bank",
            "kind": "organization",
            "name": "Example Bank",
            "classifications": [
                {"system": "urn:phds:example:party-classification", "code": "bank"}
            ],
        }
        self.assertEqual([], list(self._definition_validator("Party").iter_errors(payload)))
        self.models.Party.model_validate(payload)

        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        ts_party = re.search(r"export interface Party[^{]*\{(.*?)\n\}", ts, re.DOTALL)
        self.assertIsNotNone(ts_party)
        self.assertIn("classifications?: Classification[],", ts_party.group(1))
        ts_classification = re.search(
            r"export interface Classification[^{]*\{(.*?)\n\}", ts, re.DOTALL
        )
        self.assertIsNotNone(ts_classification)
        self.assertIn("system: string,", ts_classification.group(1))
        self.assertIn("code: string,", ts_classification.group(1))

        rust = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(encoding="utf-8")
        rust_party = re.search(r"pub struct Party \{(.*?)\n\}", rust, re.DOTALL)
        self.assertIsNotNone(rust_party)
        self.assertIn(
            "pub classifications: Option<Vec<Classification>>",
            rust_party.group(1),
        )
        rust_classification = re.search(
            r"pub struct Classification \{(.*?)\n\}", rust, re.DOTALL
        )
        self.assertIsNotNone(rust_classification)
        self.assertIn("pub system: String", rust_classification.group(1))
        self.assertIn("pub code: String", rust_classification.group(1))

    def test_party_legal_form_is_typed_across_generated_contracts(self):
        party = self.schema["$defs"]["Party"]
        properties = party["properties"]
        self.assertEqual(
            "#/$defs/Classification", self._class_reference(properties["legal_form"])
        )
        payload = {
            "id": "party-organization",
            "kind": "organization",
            "name": "Example Organization",
            "legal_form": {
                "system": "urn:producer.example:legal-form",
                "code": "example-form",
            },
        }
        self.assertEqual([], list(self._definition_validator("Party").iter_errors(payload)))
        self.models.Party.model_validate(payload)

        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        rust = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        ts_party = re.search(r"export interface Party[^{]*\{(.*?)\n\}", ts, re.DOTALL)
        rust_party = re.search(r"pub struct Party \{(.*?)\n\}", rust, re.DOTALL)
        self.assertIsNotNone(ts_party)
        self.assertIsNotNone(rust_party)
        self.assertIn("legal_form?: Classification,", ts_party.group(1))
        self.assertIn(
            "pub legal_form: Option<Classification>", rust_party.group(1)
        )

    def test_all_address_references_reject_blank_or_untrimmed_values(self):
        cases = {
            "PartyAddress": ({}, "address"),
            "PropertyAddress": (
                {"id": "property-address-1", "property": "property-1"},
                "address",
            ),
            "OwnershipPeriod": (
                {"id": "ownership-1", "property": "property-1"},
                "mailing_address",
            ),
        }
        for class_name, (base, field) in cases.items():
            validator = self._definition_validator(class_name)
            model = getattr(self.models, class_name)
            valid = {**base, field: "address-1"}
            self.assertEqual([], list(validator.iter_errors(valid)))
            model.model_validate(valid)
            for invalid in ("", " ", " address-1", "address-1 ", "\ufeffaddress-1"):
                with self.subTest(
                    class_name=class_name, field=field, invalid=repr(invalid)
                ):
                    payload = {**base, field: invalid}
                    self.assertTrue(list(validator.iter_errors(payload)))
                    with self.assertRaises(ValidationError):
                        model.model_validate(payload)

    def test_generated_rust_denies_unknown_fields_and_preserves_extras(self):
        rust = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        closed_struct_count = len(
            re.findall(r"^pub struct (?!Anything\b)", rust, re.MULTILINE)
        )
        strict_count = rust.count(
            '#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]'
        )
        self.assertGreater(closed_struct_count, 0)
        self.assertEqual(closed_struct_count, strict_count)
        self.assertNotIn("pub struct Any {", rust)
        self.assertIn("pub struct Anything(", rust)
        self.assertIn("pub type Any = Anything;", rust)
        self.assertIn("pub serde_value::Value", rust)

        wire_test = (
            GENERATED / "phds-rust" / "tests" / "wire_format.rs"
        ).read_text(encoding="utf-8")
        self.assertIn("generic_unknown_fields_are_rejected", wire_test)
        self.assertIn("property_profile_extras_round_trip_losslessly", wire_test)
        self.assertIn("nested_object", wire_test)
        self.assertIn("nested_list", wire_test)

    def test_documented_extraction_statuses_are_accepted(self):
        # Failure statuses carry no category: an unclassifiable or failed
        # extraction must be representable honestly.
        validator = self._extraction_validator()
        for status in ("success", "parse_error", "irrelevant_page", "model_error"):
            payload = {"status": status, "provenance": {}}
            with self.subTest(status=status, validator="jsonschema"):
                self.assertEqual([], list(validator.iter_errors(payload)))
            with self.subTest(status=status, validator="pydantic"):
                self.models.ExtractionObservation.model_validate(payload)

    def test_unknown_extraction_status_is_rejected(self):
        validator = self._extraction_validator()
        payload = {"status": "completely_made_up", "provenance": {}}
        self.assertTrue(list(validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.ExtractionObservation.model_validate(payload)

    def test_documented_extraction_categories_are_accepted(self):
        validator = self._extraction_validator()
        for category in (
            "sales_transaction",
            "sale_listing",
            "lease_listing",
            "in_place_lease",
            "property_facts",
            "other",
        ):
            payload = {"status": "success", "category": category, "provenance": {}}
            with self.subTest(category=category, validator="jsonschema"):
                self.assertEqual([], list(validator.iter_errors(payload)))
            with self.subTest(category=category, validator="pydantic"):
                self.models.ExtractionObservation.model_validate(payload)

    def test_unknown_extraction_category_is_rejected(self):
        validator = self._extraction_validator()
        payload = {"status": "success", "category": "completely_made_up", "provenance": {}}
        self.assertTrue(list(validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.ExtractionObservation.model_validate(payload)

    def test_unknown_assessor_status_is_rejected(self):
        payload = {"status": "completely_made_up", "provenance": {}}
        self.assertTrue(list(self.assessor_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.AssessorObservation.model_validate(payload)

    def test_canonical_money_is_accepted_by_both_validators(self):
        payload = {"amount": "994250.00", "currency": "USD"}
        self.assertEqual([], list(self.money_validator.iter_errors(payload)))
        self.models.Money.model_validate(payload)

    def test_noncanonical_money_is_rejected_by_both_validators(self):
        payload = {"amount": "994,250.00", "currency": "USD"}
        self.assertTrue(list(self.money_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.Money.model_validate(payload)

    def test_unicode_digit_money_is_rejected_by_both_validators(self):
        # Python's \d matches Unicode decimal digits; ECMA's \d is ASCII-only.
        # Patterns must use [0-9] so every dialect rejects the same values.
        payload = {"amount": "١٢٣.٤٥", "currency": "USD"}
        self.assertTrue(list(self.money_validator.iter_errors(payload)))
        with self.assertRaises(ValidationError):
            self.models.Money.model_validate(payload)

    def test_trailing_newline_money_is_rejected_by_both_validators(self):
        # Python's re treats `$` as also matching before a trailing newline,
        # while ECMA regex (AJV et al.) does not — the patterns must reject
        # these in every dialect or clients diverge on the same document.
        for payload in (
            {"amount": "994250.00\n", "currency": "USD"},
            {"amount": "994250.00", "currency": "USD\n"},
        ):
            with self.subTest(payload=payload, validator="jsonschema"):
                self.assertTrue(list(self.money_validator.iter_errors(payload)))
            with self.subTest(payload=payload, validator="pydantic"):
                with self.assertRaises(ValidationError):
                    self.models.Money.model_validate(payload)

    def test_property_profile_fixtures_agree_across_validators(self):
        for path in sorted((ROOT / "examples").glob("*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=path.name, validator="jsonschema"):
                self.assertEqual([], list(self.profile_validator.iter_errors(payload)))
            with self.subTest(path=path.name, validator="pydantic"):
                self.models.PropertyProfile.model_validate(payload)

        for path in sorted((ROOT / "counter_examples" / "schema").glob("*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=path.name, validator="jsonschema"):
                self.assertTrue(list(self.profile_validator.iter_errors(payload)))
            with self.subTest(path=path.name, validator="pydantic"):
                with self.assertRaises(ValidationError):
                    self.models.PropertyProfile.model_validate(payload)

        for path in sorted((ROOT / "counter_examples" / "semantic").glob("*.json")):
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=path.name, validator="jsonschema"):
                self.assertEqual([], list(self.profile_validator.iter_errors(payload)))
            with self.subTest(path=path.name, validator="pydantic"):
                self.models.PropertyProfile.model_validate(payload)

    def test_money_counter_examples_have_exactly_one_intended_defect(self):
        cases = {
            "PropertyProfile-bad-money-amount.json": "994,250.00",
            "PropertyProfile-money-trailing-newline.json": "994250.00\n",
            "PropertyProfile-money-unicode-digits.json": "١٢٣.٤٥",
        }
        json_path = ("assessments", 0, "assessed_total_value")
        amount_path = (*json_path, "amount")

        def descendants(error):
            yield error
            for child in error.context:
                yield from descendants(child)

        for filename, invalid_amount in cases.items():
            path = ROOT / "counter_examples" / "schema" / filename
            payload = json.loads(path.read_text(encoding="utf-8"))
            with self.subTest(path=filename, validator="jsonschema"):
                errors = list(self.profile_validator.iter_errors(payload))
                self.assertEqual(1, len(errors), errors)
                self.assertEqual(json_path, tuple(errors[0].absolute_path))
                pattern_errors = [
                    error
                    for error in descendants(errors[0])
                    if error.validator == "pattern"
                ]
                self.assertEqual(1, len(pattern_errors), pattern_errors)
                self.assertEqual(amount_path, tuple(pattern_errors[0].absolute_path))
                self.assertEqual(invalid_amount, pattern_errors[0].instance)

            with self.subTest(path=filename, validator="pydantic"):
                with self.assertRaises(ValidationError) as raised:
                    self.models.PropertyProfile.model_validate(payload)
                errors = raised.exception.errors(include_url=False)
                self.assertEqual(1, len(errors), errors)
                self.assertEqual(amount_path, errors[0]["loc"])
                self.assertEqual("value_error", errors[0]["type"])
                self.assertIn("Invalid amount format", errors[0]["msg"])
                self.assertEqual(invalid_amount, errors[0]["input"])

    def test_every_enum_variant_has_rust_rename_and_round_trip_coverage(self):
        # Census: every permissible value of every vocabulary enum must have a
        # serde rename in the generated crate and an assertion in the Rust
        # wire-format test. Fails when a new enum/value lands without coverage.
        lib_rs = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(encoding="utf-8")
        wire_test = (GENERATED / "phds-rust" / "tests" / "wire_format.rs").read_text(
            encoding="utf-8"
        )
        enums = {
            name: d["enum"]
            for name, d in self.schema["$defs"].items()
            if d.get("type") == "string" and "enum" in d
        }
        self.assertGreaterEqual(len(enums), 23)
        for name, values in enums.items():
            for wire in values:
                variant = "".join(w.capitalize() for w in wire.split("_"))
                with self.subTest(enum=name, value=wire):
                    self.assertIn(f'serde(rename = "{wire}")', lib_rs)
                    self.assertIn(
                        f'assert_round_trip({name}::{variant}, "{wire}");', wire_test
                    )

    def test_typescript_census_every_enum_ranged_slot_is_enum_typed(self):
        # Schema-driven census (mirrors the Rust one): every property in the
        # JSON Schema whose type resolves to a vocabulary enum must appear in
        # phds.ts typed with that enum, with matching optionality/cardinality.
        ts = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        interfaces = {}
        for match in re.finditer(
            r"export interface (\w+)(?: extends ([^{]+))? \{([^}]*)\}", ts
        ):
            parents = tuple(
                parent.strip() for parent in (match.group(2) or "").split(",")
                if parent.strip()
            )
            interfaces[match.group(1)] = (parents, match.group(3))

        def interface_surface(name, seen=frozenset()):
            if name in seen or name not in interfaces:
                return ""
            parents, body = interfaces[name]
            inherited = "".join(
                interface_surface(parent, seen | {name}) for parent in parents
            )
            return body + inherited
        enum_names = {
            n
            for n, d in self.schema["$defs"].items()
            if d.get("type") == "string" and "enum" in d
        }
        checked = 0
        for cls_name, d in self.schema["$defs"].items():
            if d.get("type") != "object":
                continue
            required = set(d.get("required", []))
            for prop, spec in d.get("properties", {}).items():
                is_array = spec.get("type") == "array"
                ref = (spec.get("items", {}) if is_array else spec).get("$ref")
                enum = ref.rsplit("/", 1)[-1] if ref else None
                if enum not in enum_names:
                    continue
                optional = "" if prop in required else "?"
                suffix = "[]" if is_array else ""
                declaration = f"{prop}{optional}: {enum}{suffix},"
                checked += 1
                with self.subTest(cls=cls_name, prop=prop):
                    # scoped to the class's own interface body, so one
                    # interface (including its declared mixins) regressing
                    # to `string` cannot hide behind an identical declaration
                    # on an unrelated interface
                    self.assertIn(cls_name, interfaces)
                    self.assertIn(declaration, interface_surface(cls_name))
        self.assertGreaterEqual(checked, 25)

    def test_typescript_types_enum_ranged_slots_with_their_enums(self):
        # Every enum-ranged slot must reference its enum type, not `string` —
        # for all enums, not just AssessorStatus.
        source = (GENERATED / "phds.ts").read_text(encoding="utf-8")
        for declaration in (
            "status: AssessorStatus,",
            "status: ExtractionStatus,",
            "category?: ExtractionCategory,",
            "method?: CaptureMethod,",
            "verification?: VerificationStatus,",
            "unit: AreaUnit,",
            "status?: ListingStatus,",
        ):
            self.assertIn(declaration, source)

    def test_rust_uses_canonical_snake_case_wire_values(self):
        source = (GENERATED / "phds-rust" / "src" / "lib.rs").read_text(
            encoding="utf-8"
        )
        # gen-rust --serde emits a per-variant rename for every enum; spot-check
        # values whose PascalCase variant name diverges from the wire value.
        for wire in ("not_found", "invalid_address", "llm_extraction", "pending_review"):
            self.assertIn(f'serde(rename = "{wire}")', source)
        self.assertTrue(
            (GENERATED / "phds-rust" / "tests" / "wire_format.rs").exists(),
            "Rust wire-format conformance test must exist outside src/",
        )


if __name__ == "__main__":
    unittest.main()
