# Certify test discovery / session / watch / coverage / flaky / snapshot evidence quality and selector portability

This document is the contract for the M5 test-evidence certification. It makes the
canonical test-intelligence objects landed earlier in the batch **release-bearing**
on every claimed M5 framework, notebook, and CI-import test row, so Milestone 5 can
ship this depth area with proof and downgrade behavior instead of feature copy that
outruns evidence.

Where the test-intelligence qualification matrix freezes *which* canonical object
class each claimed surface resolves to, this certification answers a sharper
question per **row**: is the discovery, session, watch, coverage, flaky, snapshot,
and selector-portability evidence this row claims actually *current*, reopenable,
and inside its freshness window? A row that loses current proof auto-narrows below
its claim rather than coasting on an adjacent green row.

## Source of truth

The canonical record is the checked support export at
`artifacts/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/support_export.json`,
validated against
`schemas/testing/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.schema.json`.
The packet is produced by
`crates/aureline-runtime/src/certify_test_discovery_session_watch_coverage_flaky_snapshot_evidence_quality`
and dumped by the `dump_certify_test_evidence_quality` example. No product,
docs/help, review, support, or release-control surface may present a greener claim
than this certification.

## Rows and dimensions

Each row covers one claimed M5 test row, keyed by a `row_kind`:

- `framework_pack_row` — a framework-pack test row,
- `notebook_row` — a notebook-backed test row,
- `ai_test_generation_row` — an AI-assisted test-generation row,
- `review_panel_row` — a review / pull-request test-panel row,
- `ci_import_row` — an imported-CI test row.

Each row certifies an evidence dimension set. The **required core** every claimed
row must certify is `discovery_truth`, `session_truth`, `watch_truth`, and
`selector_portability`. The quality dimensions — `coverage_evidence`,
`flaky_evidence`, and `snapshot_evidence` — are certified by the rows that claim
them. Each dimension certification names a `proof_currency` and, unless the proof is
missing, a reopenable `proof_ref` keyed by a non-display `proof_fingerprint_token`
distinct from the ref, so certification review reopens the same discovery / session
/ watch / coverage / flaky / snapshot evidence object that backs the grade.

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
current proof but keeps its full grade.

## Guardrails

- Parameterized templates stay distinct from their concrete invocations.
- Imported CI rows never read as a live local rerun; an imported row's flag agrees
  with its `imported_read_only` subject identity, and imported proof backs only the
  imported claim.
- Quarantine and stale coverage never hide behind a generic green grade; a stale
  quality dimension narrows the row.
- Snapshot, golden, and test-generation proposals use the same preview / diff /
  apply rules used elsewhere.

## Consumers

Product test-intelligence surfaces, docs/help, review, support/export, and
release-control all ingest this one certification instead of cloning test-state
language by hand. Narrowed rows are labeled below their claim in every surface.

## Boundary discipline

The packet is metadata-only. Raw test source, raw provider payloads, raw log bytes,
provider cursors, credentials, and raw artifact bodies never cross this boundary; it
carries only typed class tokens, booleans, opaque ids, fingerprint digests, and
redaction-aware reviewable labels.
