# M5 execution-lifecycle surface certification fixtures (M05-827)

Byte-identical copies of the checked-in release proof for the M5 execution-lifecycle
component surface certification packet:

- `support_export.json` — the canonical metadata-only support export (12 surfaces,
  7 green / 5 yellow / 0 red).
- `matrix.csv` — the machine-readable per-surface truth-axis / claim / status matrix.

Both files mirror
`artifacts/release/m5-execution-lifecycle-surface-certification-proof/` and are
produced by the same seeded builder
(`seeded_m5_execution_surface_cert_packet()`), so they stay aligned with the source
of truth. Regenerate with the `dump_m5_execution_lifecycle_surface_certification`
example (see
`docs/run-test-debug/m5_execution_lifecycle_surface_certification.md`).

Schema: `schemas/ui/m5-execution-lifecycle-surface-certification.schema.json`.
