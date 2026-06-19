# M5 Normalized Diagnostic-Record Proof

`support_export.json` is the checked support export of the M5 normalized
diagnostic-record set (`NormalizedDiagnosticRecordSetPacket`). It is the canonical
artifact downstream support, AI-evidence, review, and release-visible debt
surfaces ingest through
`aureline_runtime::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins::current_m5_normalized_diagnostic_record_set_export`
instead of cloning provider-local finding state.

The set normalizes one finding per M5 finding surface — notebook cell, framework
pack, request/API tooling, data tooling, preview runtime, package lane, language
provider, editor-structural guard, and imported scanner — onto the canonical v1
diagnostic record, and proves the record-level guarantees the surfaces depend on:

- a reopen handle for the editor, Problems, review, CLI/headless, AI evidence, and
  support export, each resolving to the same canonical diagnostic id without
  provider-specific translation loss;
- a stable-identity family whose observations all resolve to the same id and
  anchor family across ordinary refresh, adapter refresh, surface hop, and
  presentational change;
- typed suppression and baseline joins kept attached to the record's own
  `suppression_refs` / `baseline_refs`.

The data-tooling entry is the auto-downgrade demonstration: it has not yet
published an AI-evidence reopen handle, so it auto-downgrades from `beta` to
`held` with a `missing_reopen_surface` trigger and a precise degraded label, while
every other entry's effective qualification equals its claim.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_normalized_diagnostic_records > \
  artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_normalized_diagnostic_records summary > \
  artifacts/m5/diagnostics/diagnostic-record-proof/support_export.md
```

The artifact validates against
[`schemas/quality/diagnostic-record.schema.json`](../../../../schemas/quality/diagnostic-record.schema.json)
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/diagnostic-records/normalized_record_set.json`](../../../../fixtures/quality/m5/diagnostic-records/normalized_record_set.json).
