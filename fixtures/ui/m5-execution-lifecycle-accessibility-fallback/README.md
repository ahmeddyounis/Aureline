# M5 execution-lifecycle accessibility fallback fixtures (M05-826)

Byte-identical copies of the checked-in release proof for the M5 execution-lifecycle
component accessibility fallback & auto-narrowing packet:

- `support_export.json` — the canonical metadata-only support export (7 rows,
  2 green / 5 yellow / 0 red).
- `matrix.csv` — the machine-readable per-family reach / claim / status matrix.

Both files mirror
`artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/` and are
produced by the same seeded builder
(`seeded_m5_execution_a11y_fallback_packet()`), so they stay aligned with the source
of truth. Regenerate with the `dump_m5_execution_lifecycle_accessibility_fallback`
example (see
`docs/run-test-debug/m5_execution_lifecycle_accessibility_fallback.md`).

Schema: `schemas/ui/m5-execution-lifecycle-accessibility-fallback.schema.json`.
