# M5 Diagnostic Quality Parity Proof

`support_export.json` is the checked support export of the M5 diagnostic-quality
snapshot and imported-versus-live delta packet
(`DiagnosticQualityParityPacket`). It is the canonical artifact downstream
Problems, review, CLI/headless, AI evidence, support, and release-visible debt
surfaces ingest through
`aureline_runtime::m5_diagnostic_quality_snapshots_and_imported_versus_live_deltas::current_m5_diagnostic_quality_parity_export`
instead of cloning provider-local quality state.

The packet carries:

- one **diagnostic-quality snapshot** per claimed governance lane — a live
  language-service snapshot, a live runtime/test snapshot, an imported scanner
  snapshot held read-only, and a stale nightly-CI import — each naming the active
  quality-profile ref and fingerprint, the rule-pack/tool versions in force, the
  recent collection ids the findings were drawn from, the suppression/baseline
  refs and release-visible debt count, the imported scanner session refs, and the
  last save-participant outcomes;
- one **imported-versus-live delta packet** per comparison — an imported SARIF
  scan against a live local rerun, a stale CI scan against a current local rerun,
  and a runtime lane against a static lane — each keeping its two sides' distinct
  imported-versus-live origin and freshness and stating a compatibility verdict
  with explicit notes.

It proves the honesty guarantees the surfaces depend on:

- imported, CI, runtime, and local-rerun findings **cannot impersonate one
  another** — every delta's two sides keep distinct origins, and a comparison
  that crosses the imported/live boundary blocks an exact-delta claim unless the
  origins differ;
- a **profile / rule-pack / tool / anchor mismatch blocks** an exact delta rather
  than silently flattening the two sides — the stale CI scan is held to
  `blocked_rule_pack_mismatch` with named compatibility notes, and the imported
  scan is `compatible_with_local_confirmation` rather than rendered as live truth;
- **release-visible debt** is assembled from the snapshots, retaining owner,
  expiry, baseline, and suppression truth instead of a hand-written summary.

The stale CI-import snapshot is the auto-downgrade demonstration: its governance
state is stale against the current rule-pack epoch, so it auto-downgrades from
`beta` to `held` with a `stale_governance_state` trigger and a precise degraded
label, while every other snapshot's effective qualification equals its claim.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_quality_parity > \
  artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_diagnostic_quality_parity summary > \
  artifacts/m5/diagnostics/diagnostic-quality-parity-proof/support_export.md
```

The artifact validates against
[`schemas/quality/diagnostic-quality-parity.schema.json`](../../../../schemas/quality/diagnostic-quality-parity.schema.json)
(composed from
[`schemas/quality/diagnostic-quality-snapshot.schema.json`](../../../../schemas/quality/diagnostic-quality-snapshot.schema.json)
and
[`schemas/quality/diagnostic-delta-packet.schema.json`](../../../../schemas/quality/diagnostic-delta-packet.schema.json))
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/imported-vs-live-deltas/quality_parity_set.json`](../../../../fixtures/quality/m5/imported-vs-live-deltas/quality_parity_set.json).
