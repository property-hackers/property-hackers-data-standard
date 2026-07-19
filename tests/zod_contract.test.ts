import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import test from "node:test";

import {
  PhdsSchema,
  PropertyProfileSchema,
  ProvenanceSchema,
} from "../schema/generated/phds.zod.ts";
import type { Provenance } from "../schema/generated/phds.zod.ts";
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

type Equal<Left, Right> =
  (<Value>() => Value extends Left ? 1 : 2) extends
  (<Value>() => Value extends Right ? 1 : 2) ? true : false;
type Assert<Condition extends true> = Condition;
type _SourceUrlRemainsAString = Assert<
  Equal<Exclude<Provenance["source_url"], null | undefined>, string>
>;
type _RetrievedAtRemainsAString = Assert<
  Equal<Exclude<Provenance["retrieved_at"], null | undefined>, string>
>;

test("core examples pass document and closed profile schemas", () => {
  const paths = files("examples", "PropertyProfile-");
  assert.equal(paths.length, 9);
  for (const path of paths) {
    const payload = json(path);
    assert.equal(PhdsSchema.safeParse(payload).success, true, path);
    assert.equal(PropertyProfileSchema.safeParse(payload).success, true, path);
  }
});

test("core structural counterexamples fail", () => {
  const paths = files("counter_examples/schema", "PropertyProfile-");
  assert.equal(paths.length, 14);
  for (const path of paths) {
    assert.equal(PropertyProfileSchema.safeParse(json(path)).success, false, path);
  }
});

test("core semantic counterexamples remain structurally valid", () => {
  const paths = files("counter_examples/semantic", "PropertyProfile-");
  assert.equal(paths.length, 21);
  for (const path of paths) {
    assert.equal(PropertyProfileSchema.safeParse(json(path)).success, true, path);
  }
});

test("URI format follows RFC 3986 rather than URL normalization", () => {
  for (const source_url of [
    "https://example.com/a%20b",
    "urn:example:animal:ferret:nose",
    "mailto:person@example.com",
    "https://[2001:db8::1]/resource",
    "https://[2001:0db8:0:0:0:0:0:1]/resource",
    "https://[V1.a:b]/resource",
  ]) {
    assert.equal(ProvenanceSchema.safeParse({ source_url }).success, true, source_url);
  }

  for (const source_url of [
    "https://example.com/a b",
    "https://example.com/%ZZ",
    "//example.com/relative",
  ]) {
    assert.equal(ProvenanceSchema.safeParse({ source_url }).success, false, source_url);
  }
});

test("date-time format accepts RFC 3339 case and leap-second forms", () => {
  for (const retrieved_at of [
    "2020-01-01t00:00:00z",
    "2016-12-31T23:59:60Z",
    "2016-12-31T15:59:60-08:00",
  ]) {
    assert.equal(ProvenanceSchema.safeParse({ retrieved_at }).success, true, retrieved_at);
  }

  for (const retrieved_at of [
    "2020-01-01 00:00:00Z",
    "2020-02-30T00:00:00Z",
    "2016-12-31T23:58:60Z",
    "2020-01-01T00:00:00+24:00",
  ]) {
    assert.equal(ProvenanceSchema.safeParse({ retrieved_at }).success, false, retrieved_at);
  }
});

test("valid optional-standard examples pass their root schemas", () => {
  const cases = [
    ["examples/standards/uad-property-profile.json", Uad36Schema],
    ["examples/standards/boma-metro-property-profile.json", BomaMetroSchema],
    [
      "examples/standards/boma-international-property-profile.json",
      BomaInternationalSchema,
    ],
  ] as const;

  for (const [path, schema] of cases) {
    assert.equal(schema.safeParse(json(path)).success, true, path);
  }
});

test("structurally invalid UAD examples fail the UAD root schema", () => {
  const paths = [
    "counter_examples/standards/uad-missing-scope.json",
    "counter_examples/standards/uad-invalid-condition-code.json",
    "counter_examples/standards/uad-quality-rating-in-condition.json",
  ];

  for (const path of paths) {
    assert.equal(Uad36Schema.safeParse(json(path)).success, false, path);
  }
});

test("structurally invalid BOMA examples fail their root schemas", () => {
  const cases = [
    [
      "counter_examples/standards/boma-metro-invalid-code.json",
      BomaMetroSchema,
    ],
    [
      "counter_examples/standards/boma-international-invalid-code.json",
      BomaInternationalSchema,
    ],
    [
      "counter_examples/standards/boma-system-code-mismatch.json",
      BomaMetroSchema,
    ],
  ] as const;

  for (const [path, schema] of cases) {
    assert.equal(schema.safeParse(json(path)).success, false, path);
  }
});

test("semantic optional-standard counterexamples remain structurally valid", () => {
  const cases = [
    [
      "counter_examples/standards/uad-core-semantic-invalid.json",
      Uad36Schema,
    ],
    [
      "counter_examples/standards/boma-market-rating-in-condition.json",
      BomaMetroSchema,
    ],
  ] as const;

  for (const [path, schema] of cases) {
    assert.equal(schema.safeParse(json(path)).success, true, path);
  }
});
