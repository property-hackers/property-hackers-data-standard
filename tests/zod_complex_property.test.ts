import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

import { PropertyProfileSchema } from "../schema/generated/phds.zod.ts";

const FIXTURE = "examples/zod/PropertyProfile-complex-residential-sale.json";

const loadFixture = (): unknown =>
  JSON.parse(readFileSync(FIXTURE, "utf8")) as unknown;

const cloneFixture = (): unknown => structuredClone(loadFixture());

const setAtPath = (
  root: unknown,
  path: ReadonlyArray<string | number>,
  value: unknown,
): void => {
  assert.ok(path.length > 0);
  let current = root;

  for (const segment of path.slice(0, -1)) {
    assert.ok(current !== null && typeof current === "object");
    current = (current as Record<string | number, unknown>)[segment];
  }

  assert.ok(current !== null && typeof current === "object");
  const finalSegment = path.at(-1);
  if (finalSegment === undefined) throw new Error("mutation path must not be empty");
  (current as Record<string | number, unknown>)[finalSegment] = value;
};

const expectIssueAt = (
  payload: unknown,
  expectedPath: ReadonlyArray<string | number>,
): void => {
  const result = PropertyProfileSchema.safeParse(payload);
  assert.equal(result.success, false);

  if (!result.success) {
    assert.ok(
      result.error.issues.some(
        (issue) =>
          JSON.stringify(issue.path) === JSON.stringify(expectedPath),
      ),
      JSON.stringify(result.error.issues, null, 2),
    );
  }
};

test("validates a realistic residential sale lifecycle", () => {
  const result = PropertyProfileSchema.safeParse(loadFixture());

  if (!result.success) assert.fail(JSON.stringify(result.error.issues, null, 2));

  assert.equal(result.data.property.id, "property-1842-oak-ridge");
  assert.equal(result.data.parcels?.[0].land_area?.value, 0.38);
  assert.equal(result.data.site?.usable_land_area?.value, 0.31);
  assert.equal(result.data.assessments?.length, 2);
  assert.equal(result.data.property_state_snapshots?.length, 2);
  assert.equal(result.data.listings?.[0].events?.length, 4);
  assert.equal(result.data.listings?.[0].identifiers?.[0].value, "7501842");
  assert.equal(result.data.sales?.[0].sale_price?.amount, "598000.00");
  assert.equal(result.data.sales?.[0].buyer_financing?.code, "conventional");
  assert.equal(
    result.data.sales?.[0].listings?.[0].listing,
    "listing-oak-ridge-2025",
  );
  assert.equal(result.data.ownership?.length, 2);
  assert.equal(result.data.structures?.[0].residential?.bedrooms_total, 4);
});

const mutationCases = [
  {
    name: "assessment tax year below its minimum",
    mutationPath: ["assessments", 0, "tax_year"],
    value: 999,
    issuePath: ["assessments", 0, "tax_year"],
  },
  {
    name: "numeric money amount instead of a decimal string",
    mutationPath: ["assessments", 0, "assessed_total_value", "amount"],
    value: 380000,
    issuePath: ["assessments", 0, "assessed_total_value"],
  },
  {
    name: "unknown listing status",
    mutationPath: ["listings", 0, "events", 2, "status"],
    value: "under_contract-ish",
    issuePath: ["listings", 0, "events", 2, "status"],
  },
  {
    name: "listing participant sequence below one",
    mutationPath: ["listings", 0, "participants", 0, "sequence"],
    value: 0,
    issuePath: ["listings", 0, "participants", 0, "sequence"],
  },
  {
    name: "invalid point-in-time snapshot date",
    mutationPath: ["property_state_snapshots", 1, "as_of_date"],
    value: "2025-02-30",
    issuePath: ["property_state_snapshots", 1, "as_of_date"],
  },
  {
    name: "structure year below its minimum",
    mutationPath: ["structures", 0, "year_built"],
    value: 0,
    issuePath: ["structures", 0, "year_built"],
  },
  {
    name: "ownership percentage above one hundred",
    mutationPath: ["ownership", 1, "interests", 0, "interest_pct"],
    value: 125,
    issuePath: ["ownership", 1, "interests", 0, "interest_pct"],
  },
  {
    name: "unknown field on a strict nested sale",
    mutationPath: ["sales", 0, "invented_sale_field"],
    value: true,
    issuePath: ["sales", 0],
  },
] as const;

for (const { name, mutationPath, value, issuePath } of mutationCases) {
  test(`rejects ${name}`, () => {
    const payload = cloneFixture();
    setAtPath(payload, mutationPath, value);
    expectIssueAt(payload, issuePath);
  });
}

test("leaves cross-record reference resolution to semantic validation", () => {
  const payload = cloneFixture();
  setAtPath(
    payload,
    ["sales", 0, "parties", 0, "party"],
    "party-not-bundled",
  );

  assert.equal(PropertyProfileSchema.safeParse(payload).success, true);
});
