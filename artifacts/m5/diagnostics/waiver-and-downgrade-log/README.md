# M5 Diagnostic-Truth Waiver and Downgrade Log

`support_export.md` is the release-visible record of every claimed M5 diagnostic row
currently held below its claim, with the trigger and the precise label that narrowed
it. It is derived deterministically from the certification packet
(`DiagnosticTruthCertificationPacket::render_waiver_and_downgrade_log`) at
`artifacts/m5/diagnostics/certification-report/support_export.json`.

There are **no manual waivers**: a diagnostic row sits below its claim only by
automatic narrowing when current, reopenable proof cannot back it. Each entry names
the row, its claim and effective grade, the narrow trigger, the precise narrowed
label, and the dimensions whose proof is no longer current. When every claimed row
holds current proof the log records zero downgraded rows.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_truth_certification waiver > \
  artifacts/m5/diagnostics/waiver-and-downgrade-log/support_export.md
```

The log tracks the certification report at
[`artifacts/m5/diagnostics/certification-report/`](../certification-report/README.md);
regenerate both together whenever the certification corpus changes.
