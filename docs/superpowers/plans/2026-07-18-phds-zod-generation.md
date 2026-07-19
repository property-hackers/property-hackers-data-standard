# PHDS Zod 4 Generation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Generate deterministic, checked-in Zod 4 runtime schemas and inferred TypeScript types from every PHDS JSON Schema artifact.

**Architecture:** A PHDS-owned Python generator reads normalized JSON Schema, validates every schema keyword, orders `$defs` by their local-reference graph, and emits named Zod schemas and inferred types. Python unit tests, strict TypeScript compilation, and shared runtime fixtures keep Zod aligned with the authoritative JSON Schema contract.

**Tech Stack:** Python 3.10+ standard library, `unittest`, LinkML 1.11.1 JSON Schema, Zod 4.4.3, TypeScript 7.0.2, tsx 4.23.1, Node.js 24, npm, just.

## Global Constraints

> Execution override (2026-07-18): the user explicitly prohibited commits for this implementation. Ignore every task-level commit-approval step and do not commit any files. On 2026-07-19, the user explicitly authorized staging the completed Zod changes after the full quality gate passes.

- LinkML remains the source of truth; normalized JSON Schema controls Zod validation behavior.
- The generator reads JSON Schema only. It does not interpret LinkML or consume Rust, Pydantic, or TypeScript output.
- Target Zod 4 only.
- Export every `$defs` entry as `NameSchema` and `type Name = z.infer<typeof NameSchema>`.
- Export document roots separately as `PhdsSchema`, `Uad36Schema`, `BomaMetroSchema`, and `BomaInternationalSchema`.
- Preserve titles and descriptions with `.meta(...)`; emit descriptions as JSDoc on public schemas.
- Preserve defaults only as metadata; never emit Zod `.default()`.
- Fail with input path and JSON pointer for unknown validation keywords, external/dangling references, or recursive shapes that would weaken inference.
- Emit no `z.any()`. `$defs.Any` is an explicit null/boolean/object/number/string union.
- Zod owns structural validation only; Python retains cross-record semantic validation.
- Keep UAD and BOMA output isolated from core.
- Never commit without Chase's explicit permission for that specific commit. Staging alone is authorized for this completed Zod change set after verification.
- Preserve the RESO listing/sale alignment work from `main`; Zod must follow its
  regenerated JSON Schema contract.

## File Structure

- Create `tools/zod_generator/__init__.py`: public generator API.
- Create `tools/zod_generator/generator.py`: traversal, dependency analysis, keyword validation, and source emission.
- Create `tools/gen_zod.py`: JSON Schema-to-stdout CLI.
- Create `tests/test_gen_zod.py`: focused Python generator tests.
- Create `tests/test_zod_generated_source.py`: checked-in export and source guard rails.
- Create four `schema/generated/**/*.zod.ts` artifacts for core, UAD, BOMA Metro, and BOMA International.
- Create `package.json`, `package-lock.json`, and `tsconfig.zod.json`: locked strict TypeScript toolchain.
- Create `tests/zod_contract.test.ts`: Zod runtime fixture parity.
- Modify `.gitignore`, `justfile`, `.github/workflows/ci.yml`, `README.md`, and `tests/test_docs_examples_hygiene.py`.

---

### Task 1: Establish the generator API, scalar rendering, metadata, and CLI

**Files:**
- Create: `tools/zod_generator/__init__.py`
- Create: `tools/zod_generator/generator.py`
- Create: `tools/gen_zod.py`
- Create: `tests/test_gen_zod.py`

**Interfaces:**
- Produces `generate_zod(schema: Mapping[str, object], *, root_name: str, source_name: str = "<memory>") -> str`.
- Produces `GenerationError(ValueError)` with `source_name` and RFC 6901-style JSON pointer.
- Produces CLI `python tools/gen_zod.py INPUT --root-name NAME`, writing exactly one terminal newline to stdout.

- [x] **Step 1: Write failing API and scalar tests**

Create `tests/test_gen_zod.py` with imports and these initial cases:

```python
from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

from tools.zod_generator import GenerationError, generate_zod

ROOT = Path(__file__).resolve().parents[1]


class ZodGeneratorTests(unittest.TestCase):
    def test_scalar_root_preserves_pattern_metadata_and_type(self):
        source = generate_zod(
            {
                "title": "Amount",
                "description": "Decimal wire amount",
                "type": "string",
                "pattern": r"^-?[0-9]+(?:\.[0-9]+)?$",
            },
            root_name="AmountDocument",
        )
        self.assertIn('import * as z from "zod";', source)
        self.assertIn("export const AmountDocumentSchema", source)
        self.assertIn("new RegExp(", source)
        self.assertIn('title: "Amount"', source)
        self.assertIn('description: "Decimal wire amount"', source)
        self.assertIn(
            "export type AmountDocument = z.infer<typeof AmountDocumentSchema>;",
            source,
        )
        self.assertTrue(source.endswith("\n"))
        self.assertFalse(source.endswith("\n\n"))

    def test_default_is_metadata_not_a_transform(self):
        source = generate_zod(
            {"type": "string", "default": "draft"}, root_name="StatusDocument"
        )
        self.assertIn('default: "draft"', source)
        self.assertNotIn(".default(", source)

    def test_jsdoc_escapes_comment_terminators_and_multiline_text(self):
        source = generate_zod(
            {"type": "string", "description": "first line\ncloses */ comment"},
            root_name="Documented",
        )
        self.assertIn(" * first line", source)
        self.assertIn(" * closes *\\/ comment", source)

    def test_unknown_keyword_reports_source_and_pointer(self):
        with self.assertRaisesRegex(
            GenerationError,
            r"fixture\.schema\.json:#/mystery: unsupported schema keyword 'mystery'",
        ):
            generate_zod(
                {"type": "string", "mystery": True},
                root_name="Fixture",
                source_name="fixture.schema.json",
            )

    def test_cli_requires_root_name(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "schema.json"
            path.write_text(json.dumps({"type": "string"}), encoding="utf-8")
            result = subprocess.run(
                (sys.executable, "tools/gen_zod.py", str(path)),
                cwd=ROOT,
                capture_output=True,
                text=True,
            )
        self.assertNotEqual(0, result.returncode)
        self.assertIn("--root-name", result.stderr)
```

- [x] **Step 2: Run the test and confirm the missing-module error**

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

Expected: import `ERROR` because `tools.zod_generator` does not exist.

- [x] **Step 3: Implement the public API and deterministic helpers**

Create `tools/zod_generator/__init__.py`:

```python
from .generator import GenerationError, generate_zod

__all__ = ["GenerationError", "generate_zod"]
```

Create `tools/zod_generator/generator.py` around these stable types:

```python
from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
import json
import re
from typing import Any


class GenerationError(ValueError):
    pass


@dataclass(frozen=True)
class Context:
    source_name: str
    definitions: Mapping[str, Mapping[str, Any]]

    def fail(self, pointer: tuple[str, ...], message: str) -> GenerationError:
        encoded = "/".join(
            part.replace("~", "~0").replace("/", "~1") for part in pointer
        )
        return GenerationError(f"{self.source_name}:#/{encoded}: {message}")


def _typescript_string(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def _identifier(value: str) -> str:
    if not re.fullmatch(r"[A-Za-z_$][A-Za-z0-9_$]*", value):
        raise GenerationError(f"invalid TypeScript export name: {value!r}")
    return value
```

Implement scalar `null`, boolean, string, integer, and number branches; append string pattern and numeric bound methods; serialize metadata with sorted keys. `generate_zod` must frame the import, JSDoc, root schema, inferred type, and one trailing newline.

- [x] **Step 4: Add the thin CLI**

Create `tools/gen_zod.py`:

```python
#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys

from zod_generator import GenerationError, generate_zod


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path)
    parser.add_argument("--root-name", required=True)
    args = parser.parse_args(argv)
    try:
        schema = json.loads(args.input.read_text(encoding="utf-8"))
        sys.stdout.write(
            generate_zod(
                schema,
                root_name=args.root_name,
                source_name=args.input.as_posix(),
            )
        )
    except (OSError, json.JSONDecodeError, GenerationError) as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [x] **Step 5: Run focused tests**

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

Expected: all Task 1 tests pass with normalized output.

- [x] **Step 6: Stop for commit approval**

Report verification and changed files. Only after explicit approval: `git commit -m "feat: Add Zod generator foundation"` with only Task 1 files staged.

---

### Task 2: Map structured schemas, presence, formats, and value constraints

**Files:**
- Modify: `tools/zod_generator/generator.py`
- Modify: `tests/test_gen_zod.py`

**Interfaces:**
- Produces `_render_schema(schema, context, pointer, *, property_required=True, recursive_names=frozenset()) -> str`.
- Maps objects, arrays, explicit type unions, enums, literals, bounds, and current formats without weakening input.

- [x] **Step 1: Add failing table-driven mapping tests**

Add a presence test whose object contains required string, optional string, required nullable string, and optional nullable string. Assert:

```python
self.assertIn("z.strictObject({", source)
self.assertIn('"required_text": z.string()', source)
self.assertIn('"optional_text": z.string().optional()', source)
self.assertIn('"nullable_text": z.string().nullable()', source)
self.assertIn('"optional_nullable": z.string().nullable().optional()', source)
```

Add cases asserting:

```python
self.assertIn('"on": z.iso.date()', source)
self.assertIn('"at": z.iso.datetime({ offset: true })', source)
self.assertIn('"uri": z.url()', source)
self.assertIn("z.number().min(0).max(1)", source)
self.assertIn("z.number().int().min(1)", source)
self.assertIn('z.enum(["open", "closed"])', source)
self.assertIn("z.array(z.string()).min(1).max(3)", source)
self.assertNotIn("z.any()", source)
```

Cover `additionalProperties` false/true/schema, `minLength`, `maxLength`, exclusive bounds, `multipleOf`, `prefixItems`, `uniqueItems`, `minProperties`, and `maxProperties` with one focused case each.

- [x] **Step 2: Run tests and confirm structured-root failures**

Run: `.venv/bin/python -m unittest tests.test_gen_zod.ZodGeneratorTests -v`

Expected: new cases fail because objects, arrays, and unions are not implemented.

- [x] **Step 3: Implement the ordered dispatcher**

Use this precedence:

```python
if "$ref" in schema:
    expression = _render_reference(
        schema["$ref"], context, pointer + ("$ref",), recursive_names
    )
elif "const" in schema:
    expression = _render_literal(schema["const"])
elif "enum" in schema:
    expression = _render_enum(schema["enum"], context, pointer + ("enum",))
elif any(key in schema for key in ("anyOf", "allOf", "oneOf", "not", "if")):
    expression = _render_composition(schema, context, pointer, recursive_names)
elif isinstance(schema.get("type"), list):
    expression = _render_type_union(schema, context, pointer, recursive_names)
elif schema.get("type") == "object" or "properties" in schema:
    expression = _render_object(schema, context, pointer, recursive_names)
elif schema.get("type") == "array":
    expression = _render_array(schema, context, pointer, recursive_names)
else:
    expression = _render_scalar(schema, context, pointer)
```

Apply nullability before optionality. Use `z.strictObject`, `z.looseObject`, or `.catchall(...)` from `additionalProperties`. Preserve property insertion order.

- [x] **Step 4: Implement only current format mappings and exact uniqueness**

Define:

```python
FORMAT_RENDERERS = {
    "date": "z.iso.date()",
    "date-time": "z.iso.datetime({ offset: true })",
    "uri": "z.url()",
}
```

Unknown formats fail at their `format` pointer. Preserve a pattern alongside a format. For `uniqueItems`, emit a deterministic JSON-value canonicalizer that sorts object keys before comparing; do not use JavaScript object identity.

- [x] **Step 5: Run focused tests**

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

Expected: all Task 1-2 tests pass and generated samples contain no `z.any()`.

- [x] **Step 6: Stop for commit approval**

Only after explicit approval: `git commit -m "feat: Map JSON Schema constraints to Zod 4"` with the generator and its test staged.

---

### Task 3: Emit named definitions, local references, ordering, and recursion

**Files:**
- Modify: `tools/zod_generator/generator.py`
- Modify: `tests/test_gen_zod.py`

**Interfaces:**
- Produces `_definition_dependencies(definitions) -> dict[str, frozenset[str]]`.
- Produces deterministic strongly connected components and public exports for every definition.
- Uses Zod 4 object getters for recursive object-property edges.

- [x] **Step 1: Add failing reference tests**

Use in-memory `$defs` for `Invoice -> Money` and assert `MoneySchema` is emitted first, `InvoiceSchema` references it, and both inferred types exist. Add `Node.child -> Node` and assert:

```python
self.assertIn("get child()", source)
self.assertIn("return NodeSchema.optional();", source)
self.assertNotIn("z.any()", source)
```

Add mutual object recursion (`A.b -> B`, `B.a -> A`), dangling local reference, external reference, escaped JSON-pointer name, and an irreducible non-object cycle that must list all cycle members in `GenerationError`.

- [x] **Step 2: Run tests and confirm `$ref`/`$defs` failures**

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

Expected: new reference tests fail before graph emission exists.

- [x] **Step 3: Implement reference parsing and graph traversal**

Accept only `#/$defs/<escaped-name>`, decode `~1` and `~0`, and verify targets. Walk schema-bearing positions only: `properties` values, `items`, `prefixItems`, composition branches, schema-valued `additionalProperties`, and conditionals. Sort definition names for traversal and component ordering while preserving property order inside schemas.

- [x] **Step 4: Emit definitions before the distinct document root**

For each ordered component, render and append:

```ts
/** Definition description. */
export const MoneySchema = /* expression */;
export type Money = z.infer<typeof MoneySchema>;
```

Then emit `root_name` independently. For recursive object fields emit getter properties. Use `z.lazy` only when strict TypeScript retains a specific inferred type; otherwise fail instead of annotating `z.ZodType<unknown>`.

- [x] **Step 5: Run tests twice for deterministic output**

Run the full `tests.test_gen_zod` module twice. Expected: both runs pass with identical exact-string assertions.

- [x] **Step 6: Stop for commit approval**

Only after explicit approval: `git commit -m "feat: Generate named Zod references"` with Task 3 files staged.

---

### Task 4: Complete composition and fail-closed keyword coverage

**Files:**
- Modify: `tools/zod_generator/generator.py`
- Modify: `tests/test_gen_zod.py`

**Interfaces:**
- Produces exact `anyOf`, `allOf`, exclusive `oneOf`, `not`, and `if`/`then`/`else` behavior.
- Produces `_validate_keywords(schema, context, pointer) -> None` for every schema node.

- [x] **Step 1: Add failing composition tests**

Assert `anyOf` emits a union, `allOf` emits stable nested `z.intersection`, `oneOf` counts successful branch `safeParse` calls and requires exactly one, `not` negates `safeParse`, and conditionals validate the selected branch against the original value.

- [x] **Step 2: Add nested unsupported-keyword tests**

Place `{"type": "string", "unhandled": 1}` under `properties.name` and require `fixture.schema.json:#/properties/name/unhandled`. Verify annotation acceptance for `$schema`, `$id`, `$defs`, `title`, `description`, `examples`, `deprecated`, `default`, `readOnly`, and `metamodel_version`.

- [x] **Step 3: Run tests and confirm composition/keyword failures**

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

- [x] **Step 4: Implement composition without silent approximation**

- `anyOf`: `z.union` for two or more branches; sole branch directly.
- `allOf`: left-associated `z.intersection` chain.
- `oneOf`: union plus `superRefine`, requiring exactly one successful branch.
- `not`: `z.unknown().refine` with negated branch parsing; never `z.any()`.
- conditionals: base schema plus `superRefine`, selecting `then` or `else` from the `if` result and forwarding issues without transforming data.

- [x] **Step 5: Validate explicit supported keywords at every node**

Define and use these exact sets; validate at the start of every `_render_schema` and do not validate property or definition names as keywords:

```python
ANNOTATION_KEYWORDS = frozenset({
    "$id", "$schema", "$defs", "title", "description", "examples",
    "deprecated", "default", "readOnly", "metamodel_version",
})
VALIDATION_KEYWORDS = frozenset({
    "$ref", "type", "properties", "required", "additionalProperties",
    "pattern", "format", "minLength", "maxLength", "minimum", "maximum",
    "exclusiveMinimum", "exclusiveMaximum", "multipleOf", "enum", "const",
    "items", "prefixItems", "minItems", "maxItems", "uniqueItems",
    "minProperties", "maxProperties", "anyOf", "allOf", "oneOf", "not",
    "if", "then", "else",
})
```

Extend `_meta` to carry `readOnly`, and to carry `$id`, `$schema`, and `metamodel_version` on the document root. Add exact tests for those root metadata keys.

- [x] **Step 6: Prove all four live JSON Schemas generate**

Add one test loading core, UAD, BOMA Metro, and BOMA International JSON Schemas with their final root names. Assert generation succeeds, emits the root, ends with one newline, and contains no `z.any()`.

Run: `.venv/bin/python -m unittest tests.test_gen_zod -v`

Expected: all generator tests pass.

- [x] **Step 7: Stop for commit approval**

Only after explicit approval: `git commit -m "feat: Close Zod schema conversion contracts"` with Task 4 files staged.

---

### Task 5: Generate and census core and standards artifacts

**Files:**
- Modify: `justfile:11-33`
- Create: `tests/test_zod_generated_source.py`
- Create: `schema/generated/phds.zod.ts`
- Create: `schema/generated/standards/uad_3_6.zod.ts`
- Create: `schema/generated/standards/boma_building_class.metro.zod.ts`
- Create: `schema/generated/standards/boma_building_class.international.zod.ts`

**Interfaces:**
- Consumes the Task 1 CLI and Task 4 renderer.
- Produces four checked-in generated modules and a JSON Schema-to-export census.

- [x] **Step 1: Write the failing artifact census**

Create `tests/test_zod_generated_source.py` with a table of JSON Schema path, Zod path, and root name. For each case, parse `$defs` and extract these source declarations:

```python
schemas = set(re.findall(r"^export const (\w+)Schema =", source, re.MULTILINE))
types = set(re.findall(r"^export type (\w+) = z\.infer<", source, re.MULTILINE))
expected = set(schema["$defs"]) | {root_name}
self.assertEqual(expected, schemas)
self.assertEqual(expected, types)
self.assertNotIn("z.any()", source)
self.assertTrue(source.endswith("\n"))
self.assertFalse(source.endswith("\n\n"))
```

Use roots `Phds`, `Uad36`, `BomaMetro`, and `BomaInternational`.

- [x] **Step 2: Run the census and confirm missing artifacts**

Run: `.venv/bin/python -m unittest tests.test_zod_generated_source -v`

Expected: `FileNotFoundError` for the first `.zod.ts` file.

- [x] **Step 3: Add generation after each normalized JSON Schema command**

Add these commands to the matching `justfile` recipes:

```just
    .venv/bin/python tools/gen_zod.py schema/generated/phds.schema.json --root-name Phds > schema/generated/phds.zod.ts
    .venv/bin/python tools/gen_zod.py schema/generated/standards/uad_3_6.schema.json --root-name Uad36 > schema/generated/standards/uad_3_6.zod.ts
    .venv/bin/python tools/gen_zod.py schema/generated/standards/boma_building_class.metro.schema.json --root-name BomaMetro > schema/generated/standards/boma_building_class.metro.zod.ts
    .venv/bin/python tools/gen_zod.py schema/generated/standards/boma_building_class.international.schema.json --root-name BomaInternational > schema/generated/standards/boma_building_class.international.zod.ts
```

- [x] **Step 4: Regenerate and inspect scope**

Run: `just gen`, `git status --short`, and `git diff --check`.

Expected: four new generated files and intentional source/test/recipe changes only; no changes to existing generated contracts.

- [x] **Step 5: Run census and drift tests**

Run:

```bash
.venv/bin/python -m unittest tests.test_gen_zod tests.test_zod_generated_source -v
just check-generated
```

Expected: pass. After the RESO alignment on `main`, the live counts are 114 core
definitions, 125 UAD definitions, and 131 definitions in each BOMA JSON Schema;
treat changed counts as review prompts, not hard-coded assertions.

- [x] **Step 6: Stop for commit approval**

Only after explicit approval: `git commit -m "feat: Generate Zod contracts for PHDS profiles"` with only Task 5 files staged.

---

### Task 6: Add strict compilation and runtime fixture parity

**Files:**
- Create: `package.json`
- Create: `package-lock.json`
- Create: `tsconfig.zod.json`
- Create: `tests/zod_contract.test.ts`
- Modify: `.gitignore:1-8`
- Modify: `justfile:78-93`

**Interfaces:**
- Produces `npm run test:zod` and `just test-zod`.
- Validates core and optional-profile fixtures using generated root schemas.

- [x] **Step 1: Add the locked Node project**

Create `package.json`:

```json
{
  "name": "phds-generated-contract-tests",
  "private": true,
  "type": "module",
  "engines": { "node": ">=24" },
  "scripts": {
    "typecheck:zod": "tsc -p tsconfig.zod.json --noEmit",
    "test:zod:runtime": "node --import tsx --test tests/zod_contract.test.ts",
    "test:zod": "npm run typecheck:zod && npm run test:zod:runtime"
  },
  "dependencies": { "zod": "4.4.3" },
  "devDependencies": {
    "@types/node": "26.1.1",
    "tsx": "4.23.1",
    "typescript": "7.0.2"
  }
}
```

Append `node_modules/` to `.gitignore`. Run `npm install --package-lock-only` and `npm ci`. Expected: a committed lockfile and ignored install directory.

- [x] **Step 2: Add strict no-emit compilation**

Create `tsconfig.zod.json`:

```json
{
  "compilerOptions": {
    "allowImportingTsExtensions": true,
    "forceConsistentCasingInFileNames": true,
    "module": "ESNext",
    "moduleResolution": "Bundler",
    "noEmit": true,
    "skipLibCheck": false,
    "strict": true,
    "target": "ES2022",
    "types": ["node"]
  },
  "include": ["schema/generated/**/*.zod.ts", "tests/zod_*.test.ts"]
}
```

Run: `npm run typecheck:zod`.

Expected: generated modules and every Zod runtime suite compile. Fix generator defects rather than weakening strictness or using `z.ZodType<unknown>`.

- [x] **Step 3: Add runtime tests using shared fixtures**

Create `tests/zod_contract.test.ts` importing `PhdsSchema`, `PropertyProfileSchema`, `Uad36Schema`, `BomaMetroSchema`, and `BomaInternationalSchema`. Start with these concrete helpers and core cases:

```ts
import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

import { PhdsSchema, PropertyProfileSchema } from "../schema/generated/phds.zod.ts";
import { Uad36Schema } from "../schema/generated/standards/uad_3_6.zod.ts";
import { BomaMetroSchema } from "../schema/generated/standards/boma_building_class.metro.zod.ts";
import { BomaInternationalSchema } from "../schema/generated/standards/boma_building_class.international.zod.ts";

const json = (path: string): unknown =>
  JSON.parse(readFileSync(path, "utf8")) as unknown;

const files = (directory: string, prefix: string): string[] =>
  readdirSync(directory)
    .filter((name) => name.startsWith(prefix) && name.endsWith(".json"))
    .sort()
    .map((name) => join(directory, name));

test("core examples pass document and closed profile schemas", () => {
  for (const path of files("examples", "PropertyProfile-")) {
    const payload = json(path);
    assert.equal(PhdsSchema.safeParse(payload).success, true, path);
    assert.equal(PropertyProfileSchema.safeParse(payload).success, true, path);
  }
});

test("core structural counterexamples fail", () => {
  for (const path of files("counter_examples/schema", "PropertyProfile-")) {
    assert.equal(PropertyProfileSchema.safeParse(json(path)).success, false, path);
  }
});

test("core semantic counterexamples remain structurally valid", () => {
  for (const path of files("counter_examples/semantic", "PropertyProfile-")) {
    assert.equal(PropertyProfileSchema.safeParse(json(path)).success, true, path);
  }
});
```

Use the same `json` helper for explicit standards tables.

Required cases:

- Every `examples/PropertyProfile-*.json` passes both `PhdsSchema` and closed `PropertyProfileSchema`.
- Every `counter_examples/schema/PropertyProfile-*.json` fails `PropertyProfileSchema`.
- Every `counter_examples/semantic/PropertyProfile-*.json` passes structural Zod validation.
- Valid standards: `uad-property-profile.json`, `boma-metro-property-profile.json`, `boma-international-property-profile.json`.
- Structurally invalid UAD: `uad-missing-scope.json`, `uad-invalid-condition-code.json`, `uad-quality-rating-in-condition.json`.
- Structurally invalid BOMA: `boma-metro-invalid-code.json`, `boma-international-invalid-code.json`, `boma-system-code-mismatch.json`.
- Semantically invalid but structurally valid: `uad-core-semantic-invalid.json`, `boma-market-rating-in-condition.json`.

Use `safeParse(payload).success` and include the fixture path in every assertion message.

- [x] **Step 4: Run TypeScript and runtime checks**

Run:

```bash
npm run typecheck:zod
npm run test:zod:runtime
npm run test:zod
```

Expected: pass. For any parity mismatch, first add a focused failing Python generator test, then fix the generator.

- [x] **Step 5: Wire just**

Add:

```just
# Generated Zod contracts must compile and match structural fixture behavior.
test-zod:
    npm run test:zod
```

Change aggregate check to `check: check-generated validate test-generated test-zod test-rust`.

Run: `just test-zod` and `just test-generated`. Expected: pass.

- [x] **Step 6: Stop for commit approval**

Only after explicit approval: `git commit -m "test: Verify generated Zod contracts at runtime"` with only Task 6 files staged.

---

### Task 7: Integrate CI, documentation, and final verification

**Files:**
- Modify: `.github/workflows/ci.yml:14-28`
- Modify: `README.md:18-22,79-126`
- Modify: `tests/test_docs_examples_hygiene.py`

**Interfaces:**
- Consumes `just test-zod` and the locked Node project.
- Produces CI coverage and documented Zod usage.

- [x] **Step 1: Add a failing README hygiene test**

Extend `tests/test_docs_examples_hygiene.py` to require `Zod 4`, `schema/generated/phds.zod.ts`, `PropertyProfileSchema.safeParse`, `just test-zod`, and `Node.js 24+`; assert the roadmap no longer lists Zod generation.

Run the single test. Expected: fail against the current README.

- [x] **Step 2: Update README contract, layout, quickstart, usage, and roadmap**

- Add Zod 4 to generated contracts.
- List core and standards `.zod.ts` artifacts.
- Add Node.js 24+, `npm ci`, and `just test-zod` to Quickstart.
- Remove Zod generation from Roadmap.
- Show:

```ts
import { PropertyProfileSchema } from "./schema/generated/phds.zod";

const result = PropertyProfileSchema.safeParse(payload);
if (!result.success) console.error(result.error.issues);
```

- [x] **Step 2a: Document third-party vendoring and AI-assisted integration**

Document the current consumer workflow: copy the generated core Zod module into
a vendor directory, pin the PHDS release or commit, import the public schema,
and keep product-specific refinements outside the generated file. Give AI coding
agents the same source/version and generated-file boundary.

- [x] **Step 3: Install Node and npm dependencies in CI**

After Python setup add:

```yaml
      - uses: actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020 # v4
        with:
          node-version: "24"
          cache: npm
```

After `just venv` add a step named `Install locked Node toolchain` running `npm ci`. Preserve the existing `git status --porcelain` artifact gate.

- [x] **Step 4: Run focused documentation and Zod suites**

Run:

```bash
.venv/bin/python -m unittest tests.test_docs_examples_hygiene tests.test_gen_zod tests.test_zod_generated_source -v
npm run test:zod
```

Expected: pass.

- [x] **Step 5: Run full verification**

Run:

```bash
just check-generated
just validate
just test-generated
just test-zod
just test-rust
just check
git diff --check
git status --short
```

Expected: every command exits 0, `git diff --check` is silent, and status contains
only intended Zod work.

If the asdf Cargo shim has no configured Rust, use the Homebrew binaries:

```bash
RUSTC=/opt/homebrew/opt/rust/bin/rustc /opt/homebrew/opt/rust/bin/cargo test --manifest-path schema/generated/phds-rust/Cargo.toml --features serde
RUSTC=/opt/homebrew/opt/rust/bin/rustc /opt/homebrew/opt/rust/bin/cargo test --manifest-path schema/generated/standards/uad_3_6-rust/Cargo.toml --features serde
RUSTC=/opt/homebrew/opt/rust/bin/rustc /opt/homebrew/opt/rust/bin/cargo test --manifest-path schema/generated/standards/boma_building_class-rust/Cargo.toml --features serde
```

- [x] **Step 6: Review scope and stop for final commit approval**

Review `git diff --stat`, the complete handwritten diff, and generated artifacts.
Confirm the Zod diff is based on the current `main` contract. Report exact
verification results and proposed scope. Only after explicit approval, create the
final repo-style commit with subject `feat: Generate and verify Zod 4 property
contracts` and contiguous body bullets covering generation, metadata/reference
fidelity, optional profiles, TypeScript/runtime tests, drift checks, CI, and
documentation.

---

### Task 8: Refresh against the RESO-aligned main contract

**Files:**
- Modify: `examples/zod/PropertyProfile-complex-residential-sale.json`
- Regenerate: `schema/generated/**/*.zod.ts`
- Modify: Zod fixture census tests and plan documentation

- [x] Rebase `feat/zod-generation` onto `main` at `1b7b595` while preserving the staged Zod work through a temporary staged stash.
- [x] Read the updated README in full and review the RESO listing, sale, parcel-identifier, provenance, and mapping changes that affect generated contracts.
- [x] Confirm the generator can consume all four updated JSON Schema artifacts without a generator implementation change.
- [x] Capture RED evidence from the stale fixture and census assertions after the rebase.
- [x] Update the complex fixture to the current listing identifiers, lifecycle events, sale financing/concessions, sale-listing relationship, transfer, and ownership date fields.
- [x] Regenerate core, UAD, BOMA Metro, and BOMA International Zod modules and update the live fixture censuses to 9 valid, 14 structural, and 21 semantic fixtures.
- [x] Validate the complex fixture with LinkML and Python semantic validation.
- [x] Run `just check-generated && just check`; verify 297 Python tests, 19 Zod tests, and 30 Rust wire tests pass.
- [x] Review and stage the refresh with `git add -p`; leave all work uncommitted.
