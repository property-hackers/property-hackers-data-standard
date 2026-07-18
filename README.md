# PHDS: Property Hackers Data Standard

An open, internationally neutral data standard for real property of any use.
It covers physical property, parties, public records, transactions, financing,
operations, leasing, and valuation across residential, commercial, industrial,
agricultural, and other property types.

> Status: early draft (0.2), published as an RFC. Everything is open for discussion; issues and pull requests are welcome.

The [LinkML](https://linkml.io) source separates value types and provenance
([`core.yaml`](schema/core.yaml)), canonical entities
([`entities.yaml`](schema/entities.yaml)), the `PropertyProfile` interchange
document ([`profiles.yaml`](schema/profiles.yaml)), and capture envelopes
([`capture.yaml`](schema/capture.yaml)).

## Why

Property systems repeatedly model the same facts in incompatible shapes. PHDS
provides a common source for database design, generated types, validation, and
JSON interchange without assuming a country, registry, vendor, property use,
unit system, currency, or rating scale. It generates and tests JSON Schema,
JSON-LD, Pydantic, TypeScript, and Rust contracts.

## What you can use it for

- Generate types and database designs from a shared property model.
- Structure and validate extracted or AI-produced data.
- Enforce structural and semantic rules at system boundaries.
- Exchange property profiles without custom mappings for every integration.

## Design highlights

- A normalized canonical model plus a one-property `PropertyProfile`
  interchange document.
- Stable physical identities with sparse dated state; omitted historical facts
  remain unknown rather than inheriting current values.
- Multiple temporal property addresses with roles, primary status, validity
  periods, and provenance.
- System-qualified condition, quality, and market ratings without a mandatory
  national or industry scale.
- Dated rent rolls with authoritative reported totals and optional canonical
  references to spaces, tenants, and leases.
- Evidence artifacts, attributed remarks, and verification by canonical parties.
- Explicit `Money`, `Area`, `Length`, `UnitRate`, and `CodeableConcept` value
  types. Units are explicit, and money amounts use decimal strings on the wire.
- No country defaults; optional standards remain outside the core import closure.
- `extras` preserves producer-specific information under documented extension
  conventions without changing canonical PHDS meaning.

## Time and identity semantics

Stable entities hold observation-derived current facts. Sparse state records
under `PropertyStateSnapshot` hold historical assertions, with `as_of_date` as
their effective date. Capture timestamps say when information was retrieved or
extracted—not when it was true. See
[Physical-state semantics](docs/semantics/physical-state.md).

Each actor is one canonical `Party`; relationships carry typed references and
roles rather than duplicate names or classifications. `Party.name` is the
canonical display name, while normalized and parsed fields are producer-derived
matching aids. Organization legal form uses a system-qualified
`Classification`; business classifications use `Party.classifications`. For
unresolved or redacted tenants, see
[Rent-roll semantics](docs/semantics/rent-rolls.md).

`*_pct` fields and `cap_rate` use 0–100 percentage points (`5.75` means 5.75%).
`Provenance.confidence` uses a 0–1 fraction (`0.8` means 80%). Source-defined
scores and rates document their own scales.

## Profiles and extensions

`PropertyProfile` is the neutral core interchange root. Optional constrained
profiles under `schema/standards/` opt into external code sets without entering
the core import closure.

Producer-specific data belongs in `extras` using namespaced keys. See
[Extension conventions](docs/extensions.md).

## Repository layout

```text
schema/
  core.yaml                 value types, enums, and provenance
  entities.yaml             normalized canonical entities
  profiles.yaml             PropertyProfile interchange document
  capture.yaml              operational capture envelopes; core generator entry point
  standards/                optional external-standards profiles; not imported by core
  generated/                generated JSON Schema, JSON-LD, Pydantic, TypeScript, and Rust
examples/                   valid illustrative core fixtures
examples/standards/         valid optional-profile fixtures
counter_examples/schema/    fixtures rejected by structural validation
counter_examples/semantic/  structurally valid fixtures rejected by semantic validation
counter_examples/standards/ fixtures rejected by optional standards profiles
docs/semantics/             normative semantic guidance beyond structural schemas
docs/crosswalks/            field and concept alignment with external standards
```

## Quickstart

Requires Python 3.10+, [just](https://github.com/casey/just), and a Rust
toolchain with `cargo`.

```sh
just venv             # create .venv and install the pinned LinkML toolchain
just gen              # regenerate all core and optional-profile artifacts
just check-generated  # fail if regeneration changes schema/generated
just validate         # valid fixtures pass; negative fixtures fail as intended
just test-generated   # source, semantic, round-trip, and generated-contract tests
just test-rust        # core and standards-profile Rust wire-format tests
just check            # generated-drift + semantic + round-trip + standards-profile + Rust checks
```

`just check-generated` regenerates from an empty isolated tree and compares the
result with `schema/generated`. It does not write to the checkout or use the Git
index, so unrelated work cannot affect drift detection.

## Fixtures

Core fixtures are illustrative and may retain public corporate, institutional,
jurisdiction, geographic, and source-system values. Natural-person names are
synthetic or altered.

## Roadmap

- SQL DDL generation for PostgreSQL
- Zod generation and Elixir and Ruby clients
- Registered namespace URIs (currently `example.org` placeholders)
- A separate multi-property comparable-analysis extension
- Typed, multi-authority Party credentials with issuer, jurisdiction, status,
  and validity periods

## Referenced standards and independence

PHDS is internationally neutral. These materials inform mappings, optional
profiles, and design research; none is a globally preferred classification:

- [RESO Data Dictionary](https://www.reso.org/data-dictionary/)
- [Fannie Mae condition and quality guidance](https://selling-guide.fanniemae.com/sel/b4-1.3-06/property-condition-and-quality-construction-improvements)
- [Freddie Mac UAD and forms redesign](https://sf.freddiemac.com/faqs/uad-and-forms-redesign)
- [MISMO](https://www.mismo.org/)
- [Appraisal Institute Property Use Classification System](https://www.appraisalinstitute.org/insights-and-resources/resources/standards-of-professional-practice/professional-practice-samples-templates-and-documents/property-use-classification-system-pucs)
- [BOMA building class definitions](https://boma.org/boma-standards/building-class-definitions)
- [OSCRE Industry Data Model](https://www.oscre.org/Industry-Data-Model/Introducing-the-Data-Model)
- [ASTM E2018 property condition assessment guidance](https://www.astm.org/news/press-releases/e2018-revision)
- [RICS technical due diligence guidance](https://www.rics.org/profession-standards/rics-standards-and-guidance/sector-standards/real-estate-standards/technical-due-diligence-of-commercial-property)

PHDS is independent and is not affiliated with or endorsed by any referenced
organization. Names and trademarks belong to their owners. PHDS mappings do not
redistribute standards, establish certification, or make an external standard
normative for core. Consult each publisher for licensing, geographic scope, and
authoritative versions.

## License

MIT. See [LICENSE](LICENSE).

Feedback: please open a GitHub issue.
