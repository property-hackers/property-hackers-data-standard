# Rent-roll identity and evidence

A `RentRollLine` is a dated source assertion. Its optional `tenant` field is a
reference to the canonical legal-lessee `Party` only when that identity is
known well enough to resolve. An occupied line does not require an identified
tenant: `occupancy_status`, rent, area, and the other dated facts remain useful
when identity is redacted, illegible, partial, or otherwise unresolved.

When identity is unresolved, omit `tenant`. A producer MUST NOT mint a
placeholder Party such as "Unknown tenant", and MUST NOT copy the source tenant
text into `extras`; either choice creates a competing actor identity outside
the canonical Party record.

Preserve the evidence instead. Use the containing `RentRoll.provenance` to
attribute the assertion and retain the original document or image as a
profile-level `SourceArtifact`. If source wording later resolves to a real
Party, a producer may emit a new canonical Party and reference it from a later
assertion without rewriting the original evidence.
