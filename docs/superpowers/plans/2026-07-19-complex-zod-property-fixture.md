# Complex Zod Property Fixture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:test-driven-development. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Publish and deeply test a realistic residential property sale lifecycle through the generated Zod 4 `PropertyProfileSchema`.

**Architecture:** Store the exact wire document under `examples/zod/`, exercise it from a focused TypeScript behavioral suite, and link it from a near-front README example. Existing generated schemas remain unchanged.

**Tech Stack:** Zod 4.4.3, TypeScript 7.0.2, Node.js 24+, node:test, Python unittest, LinkML validation.

## Global Constraints

- Four parties only: seller, buyer, listing agent, buyer agent.
- Fixture path is `examples/zod/PropertyProfile-complex-residential-sale.json`.
- Test public Zod parsing and issue paths, never private schema internals.
- Cross-record resolution remains Python semantic validation.
- Do not commit. Staging was authorized by the user on 2026-07-19 after the full quality gate passes.

---

### Task 1: Add the complex behavioral Zod suite and exact fixture

**Files:**
- Create: `tests/zod_complex_property.test.ts`
- Create: `examples/zod/PropertyProfile-complex-residential-sale.json`
- Modify: `package.json`

**Interfaces:**
- Consumes `PropertyProfileSchema.safeParse(input: unknown)`.
- Produces a public JSON fixture and a runtime suite discovered by `npm run test:zod:runtime`.

- [x] Write the test file first. Load the exact fixture path; assert positive parsing and representative assessment, listing, sale, snapshot, ownership, and structure values. Add table-driven one-defect mutations with exact issue paths, plus the unresolved-reference structural-success boundary.
- [x] Run `node --import tsx --test tests/zod_complex_property.test.ts`; expect RED because the fixture is missing.
- [x] Add the exact approved four-party JSON fixture.
- [x] Run the focused test; expect all cases GREEN.
- [x] Change `test:zod:runtime` to run `tests/zod_*.test.ts`, then run `npm run test:zod` and confirm both suites execute.

### Task 2: Publish the example near the front of the README

**Files:**
- Modify: `tests/test_docs_examples_hygiene.py`
- Modify: `README.md`

**Interfaces:**
- README links `examples/zod/PropertyProfile-complex-residential-sale.json` and demonstrates `PropertyProfileSchema.safeParse`.

- [x] Extend `test_readme_documents_zod_generator_and_consumer_contract` first to require the fixture link, a near-front `## TypeScript and Zod example` section, and the parsing call.
- [x] Run the focused unittest and confirm RED because the section/link is absent.
- [x] Add the concise README section after `What you can use it for`, before `Design highlights`.
- [x] Run the focused unittest and confirm GREEN.
- [x] Validate the fixture directly with LinkML and `tools/profile_validation.py`.
- [x] Run `npm run test:zod`, `just check-generated`, `just check`, and `git diff --check`.

### Task 3: Refresh the fixture after the RESO-aligned main rebase

- [x] Capture structural failures from the retired listing, sale, transfer, property-parcel, and ownership fields.
- [x] Migrate the fixture to the current RESO-aligned PHDS wire shape while retaining the approved four-party lifecycle.
- [x] Add positive assertions for the listing identifier, buyer financing concept, and sale-listing relationship.
- [x] Re-run LinkML validation, Python semantic validation, strict TypeScript compilation, all 19 Zod runtime tests, generated drift checks, and the complete repository quality gate.
