"""Lint the LinkML source for regex constructs that diverge across consumers.

Schema `pattern` values are compiled by Python's re (linkml-validate,
gen-pydantic output), Python jsonschema, and ECMA engines (AJV et al.).
Only the dialect-portable subset is allowed. Patterns are discovered by
walking the parsed YAML (any quoting style), and checks are token-aware:
escaped literals like `\\$` or `\\\\d` are not false positives, and the
portable `[\\s\\S]` sentinel is explicitly allowed inside character classes.
"""

from __future__ import annotations

import json
from pathlib import Path
import unittest

import yaml

ROOT = Path(__file__).resolve().parents[1]
NONBLANK_TRIMMED_PATTERN = (
    r"^[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-\u200A"
    r"\u2028\u2029\u202F\u205F\u3000\uFEFF]"
    r"(?:[\s\S]*[^\u0009-\u000D\u0020\u0085\u00A0\u1680\u2000-"
    r"\u200A\u2028\u2029\u202F\u205F\u3000\uFEFF])?(?![\s\S])"
)

# escape-pair -> why it is forbidden (outside character classes for b/B)
FORBIDDEN_ESCAPES = {
    "d": r"Python \d matches Unicode digits, ECMA \d is ASCII-only — use [0-9]",
    "D": r"complement of \d diverges the same way — use [^0-9]",
    "w": r"Python \w is Unicode-aware, ECMA \w is [A-Za-z0-9_] — spell the class out",
    "W": r"complement of \w diverges the same way",
    "b": r"word boundary depends on \w and diverges with it",
    "B": r"non-boundary depends on \w and diverges with it",
    "A": r"Python-only anchor; not valid ECMA",
    "Z": r"Python-only anchor; not valid ECMA",
    "z": r"not supported by Python re or ECMA",
    "p": r"Unicode property escapes are ECMA-only (with /u); Python re lacks \p",
    "P": r"Unicode property escapes are ECMA-only (with /u); Python re lacks \P",
}

# after "(?", only these introducers are portable across both dialects
PORTABLE_GROUP_INTRODUCERS = (":", "=", "!", "<=", "<!")


def pattern_issues(pattern: str) -> list[str]:
    """Return dialect-portability problems in a regex pattern string."""
    issues = []
    in_class = False
    i = 0
    while i < len(pattern):
        c = pattern[i]
        if c == "\\":
            nxt = pattern[i + 1] if i + 1 < len(pattern) else ""
            if nxt in FORBIDDEN_ESCAPES:
                # \s / \S never appear here (not in FORBIDDEN_ESCAPES); \b
                # inside a class is a literal backspace in both dialects.
                if not (nxt in "bB" and in_class):
                    issues.append(f"\\{nxt}: {FORBIDDEN_ESCAPES[nxt]}")
            i += 2
            continue
        if c == "[" and not in_class:
            in_class = True
        elif c == "]" and in_class:
            in_class = False
        elif not in_class and pattern.startswith("(?", i):
            rest = pattern[i + 2 :]
            if not rest.startswith(PORTABLE_GROUP_INTRODUCERS):
                if rest.startswith("P"):
                    issues.append("(?P...): Python-only named-group syntax")
                elif rest.startswith("<"):
                    issues.append("(?<name>): ECMA named groups are not Python re syntax")
                else:
                    issues.append("(?flags): inline flags are not portable to ECMA")
            i += 2
            continue
        i += 1
    if pattern.endswith("$") and not pattern.endswith("\\$"):
        issues.append(
            "trailing $: Python $ also matches before a trailing newline, "
            "ECMA $ does not — end with (?![\\s\\S]) instead"
        )
    return issues


def _walk_patterns(node, source):
    if isinstance(node, dict):
        for key, value in node.items():
            if key == "pattern" and isinstance(value, str):
                yield source, value
            else:
                yield from _walk_patterns(value, source)
    elif isinstance(node, list):
        for item in node:
            yield from _walk_patterns(item, source)


def schema_patterns():
    for path in sorted((ROOT / "schema").glob("*.yaml")):
        doc = yaml.safe_load(path.read_text(encoding="utf-8"))
        yield from _walk_patterns(doc, path.name)


class PatternCheckerTests(unittest.TestCase):
    """The checker itself must catch known divergences and allow the portable subset."""

    def test_flags_divergent_constructs(self):
        for bad in (
            r"^\d+$",
            r"^[0-9]+\Z",
            r"\A[0-9]+",
            r"^\w+(?![\s\S])",
            r"^ab\b",
            r"\B",
            r"\p{L}+",
            r"(?i)abc",
            r"(?P<year>[0-9]{4})",
            r"(?<year>[0-9]{4})",
            r"^[0-9]+$",
        ):
            with self.subTest(pattern=bad):
                self.assertTrue(pattern_issues(bad))

    def test_allows_portable_subset(self):
        for good in (
            r"^-?[0-9]+(\.[0-9]+)?(?![\s\S])",
            r"^[A-Z]{3}(?![\s\S])",
            r"^(?:a|b)+(?![\s\S])",
            r"^(?=x)x[0-9]*(?![\s\S])",
            r"^(?<=a)b(?![\s\S])",
            r"price\$",          # escaped literal $ is fine
            r"^a\\d(?![\s\S])",  # literal backslash then d, not \d
            r"^[\b]?x(?![\s\S])",  # backspace literal inside a class
        ):
            with self.subTest(pattern=good):
                self.assertEqual([], pattern_issues(good))


class SchemaPatternDialectTests(unittest.TestCase):
    def test_schema_declares_patterns(self):
        # DecimalString type, shared amount + currency slots, country x2
        self.assertGreaterEqual(len(list(schema_patterns())), 5)

    def test_schema_patterns_are_dialect_portable(self):
        for source, pattern in schema_patterns():
            with self.subTest(source=source, pattern=pattern):
                self.assertEqual([], pattern_issues(pattern))

    def test_percentage_point_slots_are_normatively_described(self):
        schema = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        descriptions = []
        for class_def in schema["classes"].values():
            for name, slot in class_def.get("attributes", {}).items():
                if name.endswith("_pct") or name in {"cap_rate", "abatement_percent"}:
                    descriptions.append((name, slot.get("description", "")))
        self.assertTrue(descriptions)
        for name, description in descriptions:
            with self.subTest(slot=name):
                self.assertIn("percentage points", description)

    def test_rate_and_confidence_slots_define_their_numeric_semantics(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        core = yaml.safe_load((ROOT / "schema/core.yaml").read_text())

        loan = entities["classes"]["Loan"]["attributes"]
        escalation = entities["classes"]["LeaseEscalation"]["attributes"]
        tax_line_item = entities["classes"]["TaxLineItem"]["attributes"]
        valuation = entities["classes"]["Valuation"]["attributes"]
        provenance = core["classes"]["Provenance"]["attributes"]

        self.assertIn("percentage points", loan["interest_rate"]["description"])
        self.assertIn("percentage points", escalation["cpi_floor"]["description"])
        self.assertIn("percentage points", escalation["cpi_cap"]["description"])

        escalation_value = escalation["escalation_value"].get("description", "")
        self.assertIn("fixed_percent", escalation_value)
        self.assertIn("0-100 percentage points", escalation_value)
        self.assertIn("3 means 3 percent", escalation_value)
        self.assertIn("fixed_amount", escalation_value)
        self.assertIn("increment", escalation_value)
        self.assertIn("currency of the parent LeaseEvent.rent", escalation_value)
        self.assertIn("period specified by LeaseEvent.rent_period", escalation_value)

        for slot in (
            tax_line_item["rate"],
            valuation["forecast_standard_deviation"],
            valuation["confidence_score"],
        ):
            with self.subTest(slot=slot):
                self.assertIn("Source-defined", slot["description"])
                self.assertIn(
                    "not governed by the _pct percentage-points convention",
                    slot["description"],
                )

        confidence = provenance["confidence"]
        self.assertEqual(0, confidence["minimum_value"])
        self.assertEqual(1, confidence["maximum_value"])
        self.assertIn("Fraction from 0 through 1", confidence["description"])
        self.assertIn("0.8 means 80 percent confidence", confidence["description"])

    def test_unit_rent_rate_types_cover_observation_semantics(self):
        core = yaml.safe_load((ROOT / "schema/core.yaml").read_text())
        values = set(core["enums"]["RateType"]["permissible_values"])
        self.assertEqual({"asking", "market", "effective", "contract"}, values)

    def test_physical_detail_descriptions_distinguish_effective_and_capture_time(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        for detail in ("ResidentialDetails", "CommercialDetails"):
            with self.subTest(detail=detail):
                description = entities["classes"][detail]["description"]
                self.assertIn("observation-derived current state", description)
                self.assertIn("StructureState", description)
                self.assertIn("PropertyStateSnapshot.as_of_date", description)
                self.assertIn(
                    "provenance.retrieved_at is retrieval metadata", description
                )
                self.assertIn(
                    "separately bundled PropertyStateSnapshot", description
                )
                self.assertNotIn("snapshots on SaleEvent", description)

    def test_actor_roles_reference_canonical_parties(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        classes = entities["classes"]

        transaction_party = classes["TransactionParty"]["attributes"]
        self.assertTrue(transaction_party["party"]["required"])

        association = classes["PropertyAssociation"]["attributes"]
        self.assertTrue(association["party"]["required"])

        party_description = classes["Party"]["description"]
        for role in (
            "tenants and lessees",
            "lenders",
            "contractors",
            "associations",
            "valuation performers",
        ):
            with self.subTest(role=role):
                self.assertIn(role, party_description)

    def test_canonical_identity_fields_are_nonblank_and_trimmed(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        classes = entities["classes"]
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN,
            classes["Entity"]["attributes"]["id"]["pattern"],
        )
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN,
            classes["Party"]["attributes"]["name"]["pattern"],
        )

        party_references = {
            f"{class_name}.{slot_name}": slot
            for class_name, class_def in classes.items()
            for slot_name, slot in class_def.get("attributes", {}).items()
            if isinstance(slot, dict) and slot.get("range") == "Party"
        }
        self.assertEqual(
            {
                "TransactionParty.party",
                "OwnershipInterest.party",
                "PropertyAssociation.party",
                "LoanEvent.to_party",
                "Permit.contractor_party",
                "RentRollLine.tenant",
                "Valuation.performed_by_party",
                "VerificationAttribution.verifier",
            },
            set(party_references),
        )
        for path, slot in party_references.items():
            with self.subTest(path=path):
                self.assertEqual(NONBLANK_TRIMMED_PATTERN, slot.get("pattern"))

    def test_party_classifications_are_open_system_qualified_concepts(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        core = yaml.safe_load((ROOT / "schema/core.yaml").read_text())
        classifications = entities["classes"]["Party"]["attributes"][
            "classifications"
        ]
        self.assertEqual("Classification", classifications["range"])
        self.assertTrue(classifications["multivalued"])
        self.assertTrue(classifications["inlined"])
        self.assertTrue(classifications["inlined_as_list"])
        description = classifications["description"].lower()
        self.assertIn("system-qualified", description)
        self.assertIn("open vocabulary", description)

        for class_name in ("Classification", "Rating"):
            with self.subTest(class_name=class_name):
                class_def = core["classes"][class_name]
                self.assertEqual("CodeableConcept", class_def["is_a"])
                for field in ("system", "code"):
                    slot = class_def["attributes"][field]
                    self.assertTrue(slot["required"])
                    self.assertEqual(NONBLANK_TRIMMED_PATTERN, slot["pattern"])

    def test_party_legal_form_is_open_and_separate_from_roles_and_industry(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        party = entities["classes"]["Party"]
        attributes = party["attributes"]

        self.assertEqual("Classification", attributes["legal_form"]["range"])
        self.assertFalse(attributes["legal_form"].get("required", False))
        legal_form_description = attributes["legal_form"]["description"].lower()
        self.assertIn("legal form", legal_form_description)
        self.assertIn("jurisdiction", legal_form_description)
        self.assertIn("not", legal_form_description)
        self.assertIn("role", legal_form_description)
        self.assertIn("industry", legal_form_description)

    def test_party_name_fields_distinguish_display_matching_and_parsed_semantics(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        attributes = entities["classes"]["Party"]["attributes"]

        self.assertIn("canonical display name", attributes["name"]["description"].lower())
        normalized = attributes["normalized_name"]["description"].lower()
        self.assertIn("producer-derived", normalized)
        self.assertIn("matching", normalized)
        self.assertIn("not authoritative", normalized)
        for field in ("name_first", "name_middle", "name_last"):
            with self.subTest(field=field):
                description = attributes[field]["description"].lower()
                self.assertIn("producer-derived", description)
                self.assertIn("parsed", description)
                self.assertIn("not", description)
                self.assertIn("identity", description)

    def test_every_typed_address_reference_is_nonblank_and_trimmed(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        classes = entities["classes"]
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN,
            classes["AddressAssociation"]["attributes"]["address"]["pattern"],
        )
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN,
            classes["OwnershipPeriod"]["attributes"]["mailing_address"]["pattern"],
        )
        address = classes["Address"]["attributes"]
        self.assertEqual(NONBLANK_TRIMMED_PATTERN, address["address_hash"]["pattern"])
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN, address["address_hash_scheme"]["pattern"]
        )

    def test_listing_events_define_ordered_lifecycle_and_price_assertions(self):
        classes = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())["classes"]
        listing = classes["Listing"]
        listing_event = classes["ListingEvent"]
        events = listing["attributes"]["events"]
        self.assertEqual("ListingEvent", events["range"])
        self.assertTrue(events["multivalued"])
        self.assertTrue(events["inlined"])
        self.assertTrue(events["inlined_as_list"])
        self.assertTrue(events.get("list_elements_ordered"))

        event_attributes = listing_event["attributes"]
        self.assertTrue(event_attributes["effective_date"]["required"])
        self.assertTrue(event_attributes["event_type"]["required"])
        self.assertEqual("datetime", event_attributes["effective_at"]["range"])
        self.assertEqual("datetime", event_attributes["observed_at"]["range"])
        self.assertEqual("ListingStatus", event_attributes["status"]["range"])
        self.assertEqual("Money", event_attributes["list_price"]["range"])
        self.assertEqual("Money", event_attributes["list_price_low"]["range"])
        self.assertEqual("RentPeriod", event_attributes["rent_period"]["range"])
        self.assertEqual("Money", event_attributes["close_price"]["range"])

        lifecycle_assertions = {
            "effective_date",
            "effective_at",
            "observed_at",
            "event_type",
            "status",
            "source_status",
            "list_price",
            "list_price_low",
            "rent_period",
            "close_price",
        }
        self.assertTrue(lifecycle_assertions.isdisjoint(listing["attributes"]))

        description = listing["description"].casefold()
        for rule in (
            "effective_date ascending",
            "array order",
            "status, list_price, and rent_period",
            "forward independently",
            "earliest event",
            "latest closed event",
        ):
            self.assertIn(rule, description)

    def test_loan_events_define_the_loan_lifecycle(self):
        classes = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())["classes"]
        loan = classes["Loan"]
        loan_event = classes["LoanEvent"]
        events = loan["attributes"]["events"]

        self.assertEqual("LoanEvent", events["range"])
        self.assertTrue(events["multivalued"])
        self.assertTrue(events["inlined"])
        self.assertTrue(events["inlined_as_list"])
        self.assertEqual("LoanEventType", loan_event["attributes"]["event_type"]["range"])
        self.assertTrue(loan_event["attributes"]["event_type"]["required"])
        self.assertIn("lifecycle", loan["description"].casefold())

    def test_foreclosure_roles_cover_mortgage_and_lien_proceedings(self):
        classes = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())["classes"]
        description = classes["ForeclosureCaseParty"]["description"].casefold()

        for role in ("lender", "trustee", "borrower", "claimant", "debtor"):
            with self.subTest(role=role):
                self.assertIn(role, description)

        flagship = json.loads(
            (ROOT / "examples/PropertyProfile-amazon-warehouse-orl.json").read_text(
                encoding="utf-8"
            )
        )
        lien = next(item for item in flagship["liens"] if item["kind"] == "mechanics")
        claimant = next(
            party["party"] for party in lien["parties"] if party["role"] == "claimant"
        )
        foreclosure = next(
            case for case in flagship["foreclosure_cases"] if case.get("loan") is None
        )
        self.assertIn(
            {"role": "claimant", "party": claimant, "sequence": 1},
            foreclosure["parties"],
        )

    def test_rent_roll_tenant_is_the_canonical_legal_lessee(self):
        classes = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())["classes"]
        description = classes["RentRollLine"]["description"].lower()
        self.assertIn("canonical legal lessee", description)
        self.assertIn("role: lessee", description)

    def test_tax_line_item_uses_a_canonical_jurisdiction_reference(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        attributes = entities["classes"]["TaxLineItem"]["attributes"]
        self.assertEqual("Jurisdiction", attributes["jurisdiction"]["range"])
        self.assertFalse(attributes["jurisdiction"].get("inlined", True))
        self.assertFalse(attributes["jurisdiction"].get("required", False))
        self.assertEqual(
            NONBLANK_TRIMMED_PATTERN, attributes["jurisdiction"]["pattern"]
        )

    def test_contractor_credentials_are_external_to_the_party_reference(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        permit = entities["classes"]["Permit"]["attributes"]["contractor_party"]
        self.assertIn("credential", permit["description"].lower())
        self.assertIn("outside", permit["description"].lower())

    def test_artifact_scope_and_locator_requirements_are_documented(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        capture = yaml.safe_load((ROOT / "schema/capture.yaml").read_text())
        reference_description = (
            "References to SourceArtifact IDs in the nested profile.artifacts "
            "bundle; invalid when profile or profile.artifacts is absent."
        )
        for envelope in ("AssessorObservation", "ExtractionObservation", "MlsObservation"):
            with self.subTest(envelope=envelope):
                self.assertEqual(
                    reference_description,
                    capture["classes"][envelope]["attributes"]["artifact_refs"][
                        "description"
                    ],
                )

        artifact_description = entities["classes"]["SourceArtifact"]["description"]
        self.assertIn(
            "Semantic validation requires at least one nonblank uri or storage_reference.",
            artifact_description,
        )


class PublicArchitectureContractTests(unittest.TestCase):
    def test_structure_rating_concepts_are_multivalued_and_uad_constrains_items(self):
        entities = yaml.safe_load((ROOT / "schema/entities.yaml").read_text())
        facts = entities["classes"]["StructureFacts"]["attributes"]
        for plural, singular in (
            ("condition_ratings", "condition_rating"),
            ("quality_ratings", "quality_rating"),
        ):
            with self.subTest(slot=plural):
                self.assertNotIn(singular, facts)
                self.assertEqual("Rating", facts[plural]["range"])
                self.assertTrue(facts[plural]["multivalued"])
                self.assertTrue(facts[plural]["inlined"])
                self.assertTrue(facts[plural]["inlined_as_list"])

        uad = yaml.safe_load((ROOT / "schema/standards/uad_3_6.yaml").read_text())
        for class_name in ("Uad36Structure", "Uad36StructureState"):
            usage = uad["classes"][class_name]["slot_usage"]
            with self.subTest(class_name=class_name):
                self.assertEqual(
                    "Uad36ConditionRating", usage["condition_ratings"]["range"]
                )
                self.assertEqual(
                    "Uad36QualityRating", usage["quality_ratings"]["range"]
                )

        example = json.loads(
            (ROOT / "examples/standards/uad-property-profile.json").read_text()
        )
        self.assertEqual(
            {"overall", "exterior", "interior"},
            {
                rating["scope"]
                for rating in example["structures"][0]["condition_ratings"]
            },
        )

    def test_all_public_schema_versions_are_0_2_0(self):
        paths = [
            ROOT / "schema/core.yaml",
            ROOT / "schema/entities.yaml",
            ROOT / "schema/profiles.yaml",
            ROOT / "schema/capture.yaml",
        ]
        for path in paths:
            with self.subTest(path=path.name):
                self.assertEqual(
                    "0.2.0", str(yaml.safe_load(path.read_text())["version"])
                )

    def test_core_import_closure_excludes_optional_standards(self):
        for name in ("core.yaml", "entities.yaml", "profiles.yaml", "capture.yaml"):
            source = (ROOT / "schema" / name).read_text(encoding="utf-8")
            with self.subTest(schema=name):
                self.assertNotIn("standards/", source)
                self.assertNotIn("uad_3_6", source)
                self.assertNotIn("boma_building_class", source)

    def test_generated_drift_recipe_is_tree_scoped_and_git_independent(self):
        justfile = (ROOT / "justfile").read_text(encoding="utf-8")
        self.assertIn("check-generated:", justfile)
        recipe = justfile.split("check-generated:", 1)[1].split("\n\n", 1)[0]
        self.assertIn("tools/check_generated.py", recipe)
        self.assertNotIn("git ", recipe)
        self.assertNotIn("just gen", recipe)
        self.assertNotIn("schema/generated", recipe)
        self.assertIn(
            "check: check-generated validate test-generated test-rust", justfile
        )


class DocumentationContractTests(unittest.TestCase):
    def test_readme_describes_v0_2_interchange_and_validation_surfaces(self):
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        normalized = " ".join(readme.split()).casefold()
        for text in (
            "early draft (0.2)",
            "sparse dated state",
            "temporal property addresses",
            "system-qualified condition, quality, and market ratings",
            "dated rent rolls",
            "evidence artifacts",
            "schema/standards/",
            "docs/semantics/",
            "counter_examples/schema/",
            "counter_examples/semantic/",
            "semantic validation",
            "round-trip",
            "standards-profile",
            "generated-drift",
            "observation-derived current facts",
            "`as_of_date` as their effective date",
            "one canonical `Party`",
            "0–100 percentage points",
            "0–1 fraction",
            "optional constrained profiles",
            "empty isolated tree",
            "does not write to the checkout",
            "[Physical-state semantics](docs/semantics/physical-state.md)",
            "[Extension conventions](docs/extensions.md)",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)
        roadmap = readme.split("## Roadmap", 1)[1].split("## ", 1)[0]
        self.assertNotIn("Round-trip conformance harness", roadmap)
        self.assertNotIn("provenance.retrieved_at as", readme)

    def test_readme_contains_standards_links_and_independence_disclaimer(self):
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        section = readme.split("## Referenced standards and independence", 1)[1]
        normalized = " ".join(section.split()).casefold()
        for url in (
            "https://www.reso.org/data-dictionary/",
            "https://www.reso.org/universal-parcel-identifier/",
            "https://selling-guide.fanniemae.com/sel/b4-1.3-06/property-condition-and-quality-construction-improvements",
            "https://sf.freddiemac.com/faqs/uad-and-forms-redesign",
            "https://www.mismo.org/",
            "https://www.appraisalinstitute.org/insights-and-resources/resources/standards-of-professional-practice/professional-practice-samples-templates-and-documents/property-use-classification-system-pucs",
            "https://boma.org/boma-standards/building-class-definitions",
            "https://www.oscre.org/Industry-Data-Model/Introducing-the-Data-Model",
            "https://www.astm.org/news/press-releases/e2018-revision",
            "https://www.rics.org/profession-standards/rics-standards-and-guidance/sector-standards/real-estate-standards/technical-due-diligence-of-commercial-property",
        ):
            with self.subTest(url=url):
                self.assertIn(url, section)
        for text in (
            "PHDS is independent",
            "not affiliated with or endorsed",
            "Names and trademarks",
            "do not redistribute standards",
            "establish certification",
            "normative for core",
            "licensing",
            "geographic scope",
            "authoritative versions",
            "Consult each publisher",
            "internationally neutral",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)

    def test_extension_conventions_protect_canonical_meaning(self):
        source = (ROOT / "docs/extensions.md").read_text(encoding="utf-8")
        normalized = " ".join(source.split()).casefold()
        for text in (
            "namespace.field_name",
            "com.example.energy_score",
            "MUST treat unknown extension keys as optional data",
            "preserve them during a lossless round trip",
            "MUST NOT override or change the meaning of a canonical PHDS field",
            "does not define a machine-readable extension registry in v0.2",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)

    def test_uad_crosswalk_uses_v0_2_rating_and_time_semantics(self):
        source = (ROOT / "docs/crosswalks/uad36-alignment.md").read_text(
            encoding="utf-8"
        )
        normalized = " ".join(source.split()).casefold()
        for text in (
            "system-qualified `Rating` objects",
            "optional UAD profile",
            "PHDS remains internationally neutral",
            "observation-derived current facts",
            "`PropertyStateSnapshot.as_of_date`",
            "capture and provenance timestamps",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)

    def test_physical_state_guide_documents_event_snapshot_references(self):
        source = (ROOT / "docs/semantics/physical-state.md").read_text(
            encoding="utf-8"
        )
        normalized = " ".join(source.split()).casefold()
        for text in (
            "property_state_snapshots",
            "property_state",
            "saleevent",
            "listing",
            "leaseevent",
            "valuation",
            "must resolve",
            "non-inlined",
            "profile-local",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)

    def test_rent_roll_guide_omits_unresolved_tenants_without_shadow_identity(self):
        source = (ROOT / "docs/semantics/rent-rolls.md").read_text(encoding="utf-8")
        normalized = " ".join(source.split()).casefold()
        for text in (
            "omit `tenant`",
            "must not mint a placeholder party",
            "must not copy the source tenant text into `extras`",
            "sourceartifact",
            "provenance",
            "occupied",
            "unresolved",
        ):
            with self.subTest(text=text):
                self.assertIn(text.casefold(), normalized)

        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        self.assertIn("[Rent-roll semantics](docs/semantics/rent-rolls.md)", readme)

if __name__ == "__main__":
    unittest.main()
