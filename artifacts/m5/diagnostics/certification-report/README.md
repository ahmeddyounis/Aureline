# M5 Diagnostic-Truth Certification Report

`support_export.json` is the checked support export of the M5 diagnostic-truth
certification packet (`DiagnosticTruthCertificationPacket`). It is the canonical
artifact the editor, Problems, review, CLI/headless, support, AI evidence, and
release-visible debt surfaces ingest through
`aureline_runtime::certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth::current_m5_diagnostic_truth_certification_export`
instead of narrating diagnostic maturity by hand.

The packet certifies every claimed M5 diagnostic-producing row — notebook,
framework, request/data, preview/runtime, package, imported-scanner, and
review/support/CLI — against the normalized record/source/collection/remap/session
model. Each row binds:

- a durable subject keyed by a canonical `source_kind` and an imported-versus-live
  `origin_class`, plus a non-display fingerprint distinct from its id;
- per-dimension certifications over `record_identity`, `source_descriptor`,
  `collection_snapshot`, `anchor_remap`, and (for rows with a mutating fix route)
  `quality_session`, each naming a proof currency and a reopenable proof ref;
- the row's guardrail state — source kind preserved, imported/live class preserved,
  collection completeness visible, anchor remap append-only, and mutating routes
  routed through a typed quality session;
- a claimed grade and an effective grade that ranks strictly below the claim when any
  dimension loses current proof.

The stale-collection framework row is the auto-downgrade demonstration: its
`collection_snapshot` proof aged outside its freshness window, so it auto-narrows
from `certified` to `uncertified` with a `stale_dimension_proof` trigger and a precise
narrowed label, while every other row's effective grade equals its claim. The
imported-scanner row is held read-only — its `imported_row` flag agrees with an
`imported_snapshot` origin and its proof currency is `imported_current`, which backs
the imported claim but never a local one.

`support_export.md` is the deterministic Markdown summary of the same packet. The
release-visible waiver-and-downgrade log derived from this packet lives at
`artifacts/m5/diagnostics/waiver-and-downgrade-log/support_export.md`.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_truth_certification > \
  artifacts/m5/diagnostics/certification-report/support_export.json
cargo run -p aureline-runtime --example dump_m5_diagnostic_truth_certification summary > \
  artifacts/m5/diagnostics/certification-report/support_export.md
cargo run -p aureline-runtime --example dump_m5_diagnostic_truth_certification waiver > \
  artifacts/m5/diagnostics/waiver-and-downgrade-log/support_export.md
cp artifacts/m5/diagnostics/certification-report/support_export.json \
  fixtures/quality/m5/certification-corpus/diagnostic_truth_certification_corpus.json
```

The artifact validates against
[`schemas/quality/m5-diagnostic-cert-report.schema.json`](../../../../schemas/quality/m5-diagnostic-cert-report.schema.json)
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/certification-corpus/diagnostic_truth_certification_corpus.json`](../../../../fixtures/quality/m5/certification-corpus/diagnostic_truth_certification_corpus.json).
The contract doc is
[`docs/m5/diagnostic-truth-certification.md`](../../../../docs/m5/diagnostic-truth-certification.md).
