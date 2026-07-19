# PHDS Zod 4 Generation Design

## Summary

PHDS will generate checked-in Zod 4 schemas from its normalized JSON Schema
artifacts. The Zod output will expose runtime validators and inferred TypeScript
types for every named JSON Schema definition, including the document roots.

LinkML remains the source of truth. Generated JSON Schema remains the executable
structural contract. This implementation reads only JSON Schema. A future change
could use LinkML to enrich documentation metadata that JSON Schema cannot carry,
but it must not reinterpret validation semantics from LinkML independently.

## Goals

- Generate a complete, statically importable Zod 4 representation of PHDS.
- Export a named Zod schema and inferred type for every `$defs` entry.
- Preserve the structural behavior of the generated JSON Schema.
- Preserve useful documentation metadata, especially titles and descriptions.
- Support local references without dereferencing the schema into one large
  expression or silently weakening recursive structures.
- Generate deterministic, checked-in artifacts covered by the existing drift
  check.
- Generate equivalent artifacts for the optional UAD and BOMA profiles already
  represented by generated JSON Schema artifacts.
- Verify generated Zod schemas against the same positive and structural-negative
  fixtures used by the existing validation pipeline.

## Non-goals

- Replacing LinkML or JSON Schema as an authoritative PHDS representation.
- Reimplementing `tools/profile_validation.py` in TypeScript.
- Making semantic counterexamples fail structural Zod validation.
- Publishing an npm package in this change.
- Supporting Zod 3.
- Depending on generated Rust or TypeScript types as generator input.
- Providing a general-purpose JSON Schema to Zod package outside PHDS.

## Source-of-truth boundary

The generation path is:

```text
LinkML source
    -> normalized generated JSON Schema
        -> generated Zod 4 TypeScript
```

JSON Schema controls all validation behavior in the Zod output. This keeps Zod
aligned with the same intermediate contract used for structural fixtures and
avoids separately interpreting LinkML rules in multiple generators.

The implementation will not read LinkML for metadata enrichment. PHDS's current
JSON Schema already carries the titles and descriptions needed by Zod consumers.
Any later LinkML enrichment requires its own design change identifying the exact
metadata field and proving that validation behavior is unchanged.

## Generator ownership

PHDS will own a focused generator at `tools/gen_zod.py`. It may borrow
parsing and formatting ideas from these projects without depending on them at
runtime:

- `dmitryrechkin/json-schema-to-zod`
- `StefanTerdell/json-schema-to-zod`
- `hey-jj/json-schema-to-zod`

The existing packages are not suitable as direct dependencies for the PHDS
artifact. The dynamic implementation does not emit static source and lacks the
required definition/reference behavior. The static implementations either are
archived or do not preserve `$defs` and `$ref` as named exports.

Keeping the generator in Python matches the existing code-generation toolchain
and avoids requiring a third-party JavaScript generator during `just gen`.

## Generated artifacts

The generator will produce:

```text
schema/generated/phds.zod.ts
schema/generated/standards/uad_3_6.zod.ts
schema/generated/standards/boma_building_class.metro.zod.ts
schema/generated/standards/boma_building_class.international.zod.ts
```

Each file is standalone apart from its Zod 4 import. Each JSON Schema `$defs`
entry becomes two public exports:

```ts
export const MoneySchema = /* generated Zod schema */;
export type Money = z.infer<typeof MoneySchema>;
```

The JSON Schema document root also receives a predictable named schema and
inferred type distinct from every `$defs` export:

```text
phds.zod.ts                                  PhdsSchema
uad_3_6.zod.ts                               Uad36Schema
boma_building_class.metro.zod.ts             BomaMetroSchema
boma_building_class.international.zod.ts     BomaInternationalSchema
```

For example, `$defs.PropertyProfile` is exported as `PropertyProfileSchema`,
while the complete `phds.schema.json` document is exported as `PhdsSchema`.
They are not conflated: the current JSON Schema gives the document root and its
similarly shaped named definition different `additionalProperties` behavior.

## Reference handling and ordering

The generator will collect every local reference and construct a dependency
graph for `$defs`.

- A local `$ref` becomes a reference to the corresponding named schema.
- A missing local reference is a generation error containing the source JSON
  pointer and missing target.
- External references are outside this design and cause a generation error.
- Acyclic definitions are emitted in stable dependency order.
- Recursive or mutually recursive object properties use Zod 4 getter-based
  references so strict TypeScript can infer the recursive types. `z.lazy` is
  allowed only for recursive positions where the inferred public type remains
  specific.
- If a recursive shape cannot be emitted without weakening its inferred type,
  generation fails with the participating definition names.
- Recursion must never fall back to `z.any()`.
- Definition and property traversal order must be deterministic.

## JSON Schema to Zod mapping

### Presence and nullability

Property presence and value nullability are independent:

- A property absent from its containing object's `required` array is optional.
- A schema accepting JSON `null` is nullable.
- An optional nullable property is both `.nullable()` and `.optional()`.

### Objects

- `additionalProperties: false` maps to `z.strictObject(...)`.
- `additionalProperties: true` maps to `z.looseObject(...)`.
- Schema-valued `additionalProperties` maps to a typed catch-all.
- `properties`, `required`, `minProperties`, and `maxProperties` retain their
  constraints.

### Strings

- `pattern`, `minLength`, and `maxLength` retain their constraints.
- The currently emitted `date`, `date-time`, and `uri` formats map to Zod 4
  validators with parity tests covering accepted and rejected values.
- Any other format causes generation to fail until an explicit, tested mapping
  is added.
- A non-empty enum containing only strings maps to `z.enum(...)`; other enum
  values map to literals or unions of literals.

### Numbers

- `integer` requires integer values.
- `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum`, and
  `multipleOf` retain their constraints.
- Numeric enums use literals or unions of literals.

### Arrays

- `items`, tuple forms, `minItems`, `maxItems`, and `uniqueItems` retain their
  constraints.
- Unsupported array evaluation keywords cause generation to fail.

### Composition

- `anyOf` maps to a union with equivalent acceptance behavior.
- `allOf` maps to a stable chain of Zod intersections.
- `oneOf` must enforce exclusive matching rather than behaving as an ordinary
  union.
- `const` maps to a literal.
- Supported conditional schemas use explicit refinements.

### Metadata

Generated schemas use Zod 4 `.meta(...)` for supported JSON Schema annotations,
including:

- `title`
- `description`
- `examples`
- `deprecated`

Descriptions also appear as JSDoc on public exports. A JSON Schema `default` is
stored only in `.meta({ default: ... })`; the generator never uses Zod
`.default()`, because doing so would transform input and change property-presence
semantics.

Document-level `$schema`, `$id`, and LinkML `metamodel_version` values are retained
as metadata on the generated document-root schema.

## Unsupported-keyword policy

The generator maintains an explicit set of supported validation keywords and an
explicit allowlist of annotation-only keywords.

An unknown validation keyword is a hard generation failure. The error includes:

- the unsupported keyword;
- the JSON pointer of the schema node;
- the input schema path.

The generator must never silently discard validation behavior and must never emit
`z.any()`. PHDS's definition named `Any` is still constrained to null, boolean,
object, number, or string and therefore emits an explicit union rather than
`z.any()`.

## Build integration

Each Zod artifact is generated immediately after its corresponding normalized
JSON Schema in the `justfile`:

- core Zod follows `phds.schema.json`;
- UAD Zod follows `uad_3_6.schema.json`;
- BOMA Metro and International Zod follow their respective JSON Schemas.

Because `just check-generated` compares the entire `schema/generated` directory
against clean regeneration, the new artifacts automatically participate in the
existing drift check.

The repository will add a minimal, locked Node development toolchain containing
Zod 4 and TypeScript. Generated consumers require only Zod 4 at runtime.

## Verification

### Generator unit tests

Focused Python tests cover:

- definitions and local references;
- forward, recursive, and mutually recursive references;
- required, optional, nullable, and optional-nullable properties;
- strict, loose, and typed-catch-all objects;
- strings, formats, patterns, enums, and bounds;
- numbers and arrays;
- composition and literals;
- metadata and JSDoc escaping;
- deterministic ordering;
- missing references, external references, and unsupported keywords.

### Static TypeScript checks

Generated modules compile under strict TypeScript against the pinned Zod 4
version. Contract tests assert that every `$defs` entry has a corresponding
schema and inferred-type export.

The generated source is checked to contain no `z.any()` occurrences.

### Runtime parity

A Zod runtime test suite checks:

- all core valid fixtures pass both `PhdsSchema` and the closed
  `PropertyProfileSchema` definition;
- all structural core counterexamples fail;
- valid optional-profile fixtures pass their generated root schemas;
- structural optional-profile counterexamples fail their corresponding schemas.

Semantic counterexamples are outside Zod's structural responsibility. They must
remain structurally valid and continue to fail only through the existing Python
semantic validators.

### Commands

The repository will add `just test-zod` and include it in `just check`.
`just test-generated` remains responsible for Python generator and source-level
contract tests; `just test-zod` owns TypeScript compilation and Zod runtime
parity.

## Documentation

The README will:

- list Zod 4 among generated contracts;
- include the Zod artifact locations in the repository layout;
- document the Node requirement for full checks;
- include `just test-zod` in the quickstart;
- show a short `PropertyProfileSchema.safeParse(...)` import example.

## Acceptance criteria

- All four Zod artifacts regenerate deterministically from their JSON Schemas.
- Every `$defs` entry is exported as a named schema and inferred type.
- Titles and descriptions are available through Zod metadata and public JSDoc.
- No structural constraint is silently dropped.
- Local references retain named identity and do not degrade to `z.any()`.
- Generated modules compile with the pinned Zod 4 release under strict
  TypeScript.
- Existing valid fixtures pass the appropriate generated Zod root.
- Existing structural counterexamples fail the appropriate generated Zod root.
- Semantic counterexamples remain assigned to the semantic validation layer.
- `just check-generated`, `just test-generated`, `just test-zod`, and
  `just check` pass.
