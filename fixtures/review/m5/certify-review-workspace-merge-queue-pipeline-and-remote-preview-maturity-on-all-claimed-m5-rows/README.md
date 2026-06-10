# M5 Review, Merge-Queue, Pipeline, and Remote-Preview Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
downgrade automation the canonical support export keeps green. Each keeps every
claimed row present, trust-review and consumer-projection invariants satisfied,
proof freshness valid, and the compatibility report in agreement with the row
verdicts — the difference is which rows narrowed or blocked and why.

## merge_queue_evidence_blocked.json

The merge-queue readiness row is blocked because its evidence packet failed
validation. `apply_downgrade_automation` moved the row to `blocked`, the
compatibility report shows one blocked row, and `all_rows_publishable` is
`false` — proving stale or underqualified evidence narrows the claim instead of
shipping greener than the proof.

## durable_header_proof_stale_narrowed.json

The durable review-header row is narrowed from `certified` to
`narrowed_certified` because its proof went stale relative to the freshness SLO;
the row's `proof_fresh` flag flips to `false` while every other row stays
certified. Demonstrates proof-staleness narrowing a claim rather than hiding the
row.
