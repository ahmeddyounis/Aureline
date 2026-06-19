# M5 Diagnostic-Truth Certification Corpus

## diagnostic_truth_certification_corpus.json

The certification corpus for the M5 diagnostic-truth certification packet. Every
claimed M5 diagnostic-producing row — notebook, framework, request/data,
preview/runtime, package, imported-scanner, and review/support/CLI — certifies its
`record_identity`, `source_descriptor`, `collection_snapshot`, and `anchor_remap`
proof (the required core) plus the `quality_session` dimension on the rows that own a
mutating fix route (notebook format/quick-fix, framework lint-autofix, package
lockfile mutation, and the imported scan-comparison session).

The eighth row is the auto-narrowing drill: a framework row claims `certified`, but
its collection-snapshot evidence has aged outside its freshness window
(`collection_snapshot` carries a `stale_expired` proof currency). Because a claimed
row may not outrun current proof, the row auto-narrows to an effective grade of
`uncertified`, records a `stale_dimension_proof` narrow trigger, and carries a precise
narrowed label rather than a generic provider error. Every other row keeps current,
reopenable proof for each dimension it certifies, so its effective grade equals its
claim.

The imported-scanner row is held read-only: its `imported_row` flag agrees with an
`imported_snapshot` origin class, and its proof currency is `imported_current`, which
backs the imported row's claim but never a local one — an imported scan never reads as
a live local rerun. Each row keeps its canonical `source_kind`, so unlike sources
(language service, build/task, runtime/test, policy, editor-structural, and scanner
import) are never flattened into a synthetic finding. Each dimension certification
names a reopenable proof ref keyed by a non-display fingerprint distinct from the ref.

The fixture validates against
`schemas/quality/m5-diagnostic-cert-report.schema.json` and is byte-identical to the
checked support export at
`artifacts/m5/diagnostics/certification-report/support_export.json`.
