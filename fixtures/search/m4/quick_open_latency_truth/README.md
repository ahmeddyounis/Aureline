# quick_open_latency_truth fixture corpus

Fixture corpus for the M4 stable quick-open, file/symbol/command-search
latency and partial-index truth packet (`schemas/search/quick_open_latency_truth.schema.json`).

Each fixture is a `QuickOpenLatencyTruthPacketInput` with an `expect` block
that pins the materialized packet's promotion state, finding count, archetype
and surface token sets, partial-index truth tokens, and exported support
posture. Tests in `crates/aureline-search/tests/quick_open_latency_truth_cases.rs`
load each case and assert that `QuickOpenLatencyTruthPacket::materialize`
agrees.

Cases:

- `baseline_stable.json` — A Rust workspace quick-open row hits its p50/p95
  budgets, keeps `hot_set_ready` visible with an explicit readiness
  transition, labels the partial-index downgrade, and preserves the same
  packet across all five required consumer projections.
- `budget_breach_blocks_stable.json` — A Java/Kotlin symbol-search row's
  observed p95 latency exceeds the published budget with no waiver; the
  packet blocks the stable claim.
- `partial_index_unlabeled_narrowed.json` — A TypeScript/Javascript
  file-search row reports `partial_index` truth but drops the disclosure ref;
  the packet narrows below stable with `partial_index_not_labeled`.
- `session_state_collapsed_narrowed.json` — A Rust symbol-search row keeps
  `provider_limited` visible but the docs/help projection collapses the
  state; the packet narrows below stable with `session_state_collapsed`.
