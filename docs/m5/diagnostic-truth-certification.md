# Certify diagnostic record / source / collection / anchor-remap / quality-session truth

This document is the contract for the M5 diagnostic-truth certification. It makes the
canonical diagnostic-truth objects landed earlier in the batch — the normalized
diagnostic record, the source descriptor, the collection snapshot, the append-only
anchor-remap history, and the quality-action / session ledger — **release-bearing**
on every claimed M5 code-quality and runtime-diagnostic row, so Milestone 5 can ship
this depth area with proof and downgrade behavior instead of feature copy that
outruns evidence.

Where the diagnostic-truth lane matrix freezes *which* canonical object class each
claimed surface resolves to, this certification answers a sharper question per
**row**: is the record-identity, source-descriptor, collection-snapshot,
anchor-remap, and quality-session evidence this row claims actually *current*,
reopenable, and inside its freshness window? A row that loses current proof
auto-narrows below its claim rather than coasting on an adjacent green row.

## Source of truth

The canonical record is the checked support export at
`artifacts/m5/diagnostics/certification-report/support_export.json`, validated against
`schemas/quality/m5-diagnostic-cert-report.schema.json`. The packet is produced by
`crates/aureline-runtime/src/certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth`
and dumped by the `dump_m5_diagnostic_truth_certification` example. Downstream
surfaces ingest it through
`aureline_runtime::certify_m5_diagnostic_record_source_collection_remap_and_quality_session_truth::current_m5_diagnostic_truth_certification_export`.
No editor decoration, Problems, review, CLI/headless, support, AI-evidence, or
release-visible-debt surface may present a greener claim than this certification.

## Rows and dimensions

Each row covers one claimed M5 diagnostic-producing surface, keyed by a `row_kind`:

- `notebook_row` — a notebook-backed diagnostic row,
- `framework_row` — a framework-pack / language-analyzer diagnostic row,
- `request_data_row` — a request / data-tooling diagnostic row,
- `preview_runtime_row` — a preview / runtime diagnostic row,
- `package_row` — a package-lane diagnostic row,
- `imported_scanner_row` — an imported scanner / SARIF / CI-scan row,
- `review_support_cli_row` — a review / support / CLI diagnostic row.

Each row certifies an evidence dimension set. The **required core** every claimed row
must certify is `record_identity`, `source_descriptor`, `collection_snapshot`, and
`anchor_remap`. The `quality_session` dimension is certified by the rows that own a
mutating fix route. Each dimension certification names a `proof_currency` and, unless
the proof is missing, a reopenable `proof_ref` keyed by a non-display
`proof_fingerprint_token` distinct from the ref, so certification review reopens the
same record / source / collection / remap / session evidence object that backs the
grade. The subject keeps its `source_kind` and its imported-versus-live
`origin_class` from the canonical diagnostic vocabularies so unlike sources are never
flattened into a synthetic finding.

## Auto-narrowing

A row is certified at its claimed grade only when every required-core dimension is
certified and every certified dimension carries **current** proof for the row's
imported posture:

- a local row needs `verified_current` or `cached_within_window` proof;
- an imported row needs `imported_current` proof.

A `stale_expired`, `missing_proof`, or `requires_review` proof — or imported proof
standing in for a local claim — auto-narrows the row to an effective grade strictly
below its claim, with a recorded `narrow_trigger` and a precise `narrowed_label`
rather than a generic non-answer. The validator rejects any claimed row that loses
current proof but keeps its full grade. The narrowed set is published as a
waiver-and-downgrade log at
`artifacts/m5/diagnostics/waiver-and-downgrade-log/support_export.md`; there are no
manual waivers — auto-narrowing is the only mechanism by which a row sits below its
claim.

## Guardrails

- Display clustering never erases a constituent's source kind; unlike sources are
  not flattened into a synthetic finding.
- The imported-versus-live class stays explicit across every surface; an imported
  row's flag agrees with an imported / replayed `origin_class`, and imported proof
  backs only the imported claim — an imported scan never reads as a live local rerun.
- Partial / streaming collection completeness stays visible rather than masquerading
  as a complete whole-workspace enumeration; a stale collection narrows the row.
- Anchor remap stays append-only evidence rather than a silent repair or relabel.
- Every mutating fix route serializes through the typed quality-action / session
  preview / apply / revert lifecycle.

## Consumers

Editor decorations, the Problems surface, review, CLI/headless output, support/export,
AI evidence, and release-visible debt all ingest this one certification instead of
cloning diagnostic-state language by hand. Narrowed rows are labeled below their
claim in every surface.

## Boundary discipline

The packet is metadata-only. Raw diagnostic source, raw provider payloads, raw
scanner report bytes, provider cursors, credentials, and raw artifact bodies never
cross this boundary; it carries only typed class tokens, booleans, opaque ids,
fingerprint digests, and redaction-aware reviewable labels.
