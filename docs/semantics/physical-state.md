# Physical state and capture observations

`PropertyStateSnapshot.as_of_date` is the effective date of an asserted physical state. Capture-envelope timestamps record when a producer retrieved or extracted information. They answer different questions and may differ by years.

State records are sparse assertions. Omitted historical fields are unknown or unasserted and MUST NOT be populated from current entities unless a producer emits a separate, provenance-bearing inference.

`Property`, `Site`, `Structure`, and `Space` identify stable subjects. Their corresponding `*State` records identify assertions about those subjects. A stable subject and a state assertion MUST NOT share an identifier.

## Referencing a snapshot from an event

`SaleEvent`, `Listing`, `LeaseEvent`, and `Valuation` may carry an optional
`property_state` reference. It is a non-inlined, profile-local identifier and
MUST resolve to an entity in the same `PropertyProfile.property_state_snapshots`
bundle when present:

```json
{
  "property_state_snapshots": [
    {
      "id": "state-at-sale",
      "property": "property-1",
      "as_of_date": "2025-06-30"
    }
  ],
  "sales": [
    {
      "id": "sale-1",
      "property": "property-1",
      "close_date": "2025-06-30",
      "property_state": "state-at-sale"
    }
  ]
}
```

The reference does not inline or copy the physical facts. The referenced
snapshot's `as_of_date` remains the physical-state effective date; capture and
provenance timestamps continue to describe observation or retrieval time.
