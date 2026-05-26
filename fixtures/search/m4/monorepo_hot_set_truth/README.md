# monorepo_hot_set_truth fixture corpus

Fixture corpus for the M4 stable monorepo hot-set indexing, warming-state,
and graceful-degradation truth packet
(`schemas/search/monorepo_hot_set_truth.schema.json`).

Each fixture is a `MonorepoHotSetTruthPacketInput` with an `expect` block
that pins the materialized packet's promotion state, finding count,
archetype and lane token sets, readiness states, degradation tokens, and
exported support posture. Tests in
`crates/aureline-search/tests/monorepo_hot_set_truth_cases.rs` load each
case and assert that `MonorepoHotSetTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — A large-single-root filename_index row keeps
  `hot_set_ready` visible with an explicit warming transition, labels its
  `hot_set_only` degradation class, holds the foreground responsiveness
  invariants intact, and preserves the same packet across all five
  required consumer projections.
- `degradation_unlabeled_blocks_stable.json` — A large-single-root
  filename_index row reports `hot_set_only` degradation but drops the
  disclosure ref; the packet blocks the stable claim with
  `degradation_not_labeled`.
- `missing_warming_transition_blocks_stable.json` — A polyglot-multi-root
  symbol_index row reports `partial_index` readiness but records no
  warming transition; the packet blocks the stable claim with
  `missing_warming_transition`.
- `edit_input_blocked_blocks_stable.json` — A very-large-monorepo
  text_index row reports edit input is blocked behind index warm-up; the
  packet blocks the stable claim with `edit_input_blocked_by_warmup`.
- `projection_drops_degradation_blocks_stable.json` — A docs/help
  projection flips `preserves_degradation_labels` to false; the packet
  blocks the stable claim with `degradation_vocabulary_dropped` and
  `consumer_projection_drift`.
