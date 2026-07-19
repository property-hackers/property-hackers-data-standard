# Complex Zod Property Fixture Design

## Goal

Add a public, realistic single-family sale lifecycle fixture and a dedicated behavioral Zod suite that demonstrates deep validation across the generated `PropertyProfileSchema`.

## Scenario

The fixture represents one Fulton County, Georgia property with one parcel and site, one residential structure, two annual assessments, listing-time and sale-time state snapshots, one seller, one buyer, one listing agent, one buyer agent, a listing lifecycle, recorded transfer, arms-length sale, and consecutive ownership periods.

`parcels[]` owns cadastral parcel identity and assessment context. `property_parcels[]` is the temporal property-to-parcel association. `site` owns physical and functional land facts for the property. All record references use bundled entity IDs consistently.

## Files and public surface

- Add `examples/zod/PropertyProfile-complex-residential-sale.json` as the exact public wire fixture.
- Add `tests/zod_complex_property.test.ts` for positive parsing, deep mutation failures, and the structural-versus-semantic boundary.
- Update the Zod runtime npm script to execute both TypeScript test files.
- Add a near-front README section linking the fixture and showing `PropertyProfileSchema.safeParse`.
- Extend README contract tests so the example cannot disappear silently.

## Zod behavior

The positive test parses the full fixture through `PropertyProfileSchema` and asserts representative nested parsed values. Table-driven negative tests clone the fixture, introduce exactly one defect, assert failure, and require the precise Zod issue path. Cases cover assessment year and money, listing status and participant sequence, snapshot dates, structure year, ownership percentage, and strict unknown nested fields.

A separate test replaces a bundled party reference with an unknown nonblank ID and expects structural Zod success. This documents that cross-record reference resolution belongs to Python semantic validation rather than Zod.

## Constraints

- Use four parties only: seller, buyer, listing agent, and buyer agent.
- Keep the new fixture outside the existing `examples/PropertyProfile-*.json` nine-fixture census.
- Test public parsing behavior and `ZodError.issues`; do not inspect private Zod schema internals.
- Do not weaken or modify generated schemas for this test.
- Do not commit. Staging was authorized by the user on 2026-07-19 after the full quality gate passes.

## Verification

- The new suite must fail before the fixture/runtime wiring exists and pass afterward.
- `npm run typecheck:zod`, all Zod runtime tests, the README contract test, JSON Schema validation, Python semantic validation, `just check-generated`, and `just check` must pass.
