# PHDS: Property Hackers Data Standard

An open data standard for real property. It covers parcels, structures,
assessments, taxes, deeds and transfers, sales, listings, leases, loans, liens,
foreclosures, permits, parties, and valuations in residential and commercial
real estate.

> Status: early draft (0.1), published as an RFC. Everything is open for discussion; issues and pull requests are welcome.

The schema itself: [`schema/entities.yaml`](schema/entities.yaml) (the entities), with [`schema/core.yaml`](schema/core.yaml) (value types and enums), [`schema/profiles.yaml`](schema/profiles.yaml) (the PropertyProfile interchange document), and [`schema/capture.yaml`](schema/capture.yaml) (capture envelopes; the generator entry point, since its import closure is the full standard).

## Why

Real property data has no usable open standard. MISMO, the mortgage industry's
standard, is unfortunately closed: its full reference model sits behind
membership. Meanwhile, AI-assisted software development means more hobbyist
programmers and real estate professionals are building their own tools for
property research, analysis, and valuation. The recurring problem is data
consistency: every vendor, county system, and app shapes the same facts
differently.

PHDS provides schema definitions for database DDL, structured types, and JSON
contracts with a consistent shape. It gives parsers a consistent target
shape for real estate data and supports an ecosystem where data moves between
tools because it has a common shape. PHDS is authored in
[LinkML](https://linkml.io), which generates JSON Schema, Pydantic, and
TypeScript types. PHDS also
includes per-field crosswalks to RESO, UAD 3.6/MISMO, Appraisal Institute PUCS,
BOMA, and OSCRE.

## What you can use it for

- Generate types, structs, or table definitions for your own project from the
  LinkML source or the generated JSON Schema, instead of designing a property
  model from scratch.
- Coerce LLM output into structured data: use the generated Pydantic models
  with tools like [Instructor](https://github.com/instructor-ai/instructor),
  or the JSON Schema directly with any structured-output API, to parse
  listings, deeds, and assessor pages into consistent records.
- Validate property payloads at system boundaries (the fixtures in `examples/`
  show the expected shape).
- Exchange data with other tools that speak PHDS without writing a custom
  mapping for each pair.

## Design highlights

- Normalized entities and a `PropertyProfile` document for one property.
- Value types for `Money`, `Area`, `Length`, `UnitRate`, and `CodeableConcept`.
  Units are explicit, and money values use decimal strings on the wire.
- Events for listing history, loan assignments and satisfactions, foreclosure
  cases and filings, and renovations.
- Typed multi-party relationships for deeds, sales, loans, and liens.
- `RecordedInstrument` for recorded or registered documents across recorder,
  land registry, and notarial systems.
- No country defaults. Units and currencies are explicit, and UAD 3.6 is a
  profile rather than part of the core model.
- Source and verification information on each record. `extras` preserves data
  a producer cannot otherwise model.

## Repository layout

```
schema/
  core.yaml           value types, enums, provenance
  entities.yaml       normalized canonical entities
  profiles.yaml       the PropertyProfile interchange document
  capture.yaml        capture envelopes (fetch/extraction outcomes; generator entry point)
  generated/          JSON Schema, Pydantic, TypeScript, and Rust (regenerate with `just gen`)
examples/             real-world-derived, pseudonymized fixtures that must validate
counter_examples/     fixtures that must fail validation
docs/crosswalks/      standards alignment, including UAD 3.6
```

## Quickstart

Requires Python 3.10+, [just](https://github.com/casey/just), and a Rust
toolchain (`cargo`) for the Rust wire-format tests.

```sh
just venv       # create .venv and install the LinkML toolchain
just gen        # regenerate JSON Schema / pydantic / TypeScript / Rust from the LinkML
just validate   # examples must pass, counter_examples must fail
just check      # gen + validate + generated-contract tests + Rust wire-format tests
```

## Fixtures

The `examples/` fixtures derive from real public-record data (two commercial
properties, one single-family residence). Private individuals' names, street
identifiers, coordinates, recording numbers, and hashes are pseudonymized.
Corporate and institutional parties are retained as public record.

## Roadmap

- SQL DDL generator (LinkML to Postgres)
- Zod generation; Elixir (Ecto embedded schemas) and Ruby clients
- Round-trip conformance harness (entities to profile to entities) in CI
- Registered namespace URIs (currently `example.org` placeholders)

## License

MIT. See [LICENSE](LICENSE). Industry standards referenced (RESO, UAD/MISMO,
PUCS, BOMA, OSCRE) remain the property of their respective organizations; PHDS
maps to them but does not redistribute their artifacts.

Feedback: please open a GitHub issue.
