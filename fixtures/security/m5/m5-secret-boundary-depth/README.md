# M5 Secret Boundary Depth Fixtures

This fixture set mirrors the canonical packet in `artifacts/security/m5/m5-secret-boundary-depth.json`.

- `canonical_packet.json` is the checked packet replay used by `aureline-auth` tests.
- `support_export.json` is the metadata-only support export derived from the same packet.

The fixture boundary is redaction-safe:

- raw secret values are excluded
- raw handle ids are excluded
- row ids, qualification rows, consumer projections, default modes, projection
  modes, consumer identities, projection controls, deployment-profile parity
  rows, repairable states, Project Doctor finding codes, support-bundle
  lineage refs, export posture, and repair-owner tokens are preserved

The checked fixture pair preserves the canonical evidence index:

- `qualification_rows` carry one row/profile qualification per deployment
  profile.
- `consumer_projections` preserve the same `evidence_index_ref`,
  `qualification_row_refs`, and qualification counts the canonical packet
  publishes.
