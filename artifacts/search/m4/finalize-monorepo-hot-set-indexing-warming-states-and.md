# Monorepo hot-set indexing, warming states, and graceful degradation — proof packet

## Summary

The stable monorepo hot-set truth packet at
`artifacts/search/m4/monorepo_hot_set_truth_packet.json` covers every
combination of the six certified monorepo archetypes and four governed
indexing lanes. Each row pins the readiness state of the lane, the
graceful-degradation class in force, the warming transition that fired,
the hot-set coverage estimate, and the foreground responsiveness
invariants (edit input unblocked, first useful quick-open row within
budget, full-index deferred with disclosure).

| Archetype                       | filename_index | path_index | symbol_index | text_index |
|---------------------------------|---------------:|-----------:|-------------:|-----------:|
| Small single root               | fully_indexed  | fully_indexed | fully_indexed | fully_indexed |
| Medium single root              | fully_indexed  | fully_indexed | hot_set_ready | partial_index |
| Large single root               | hot_set_ready  | hot_set_ready | partial_index | partial_index |
| Polyglot multi-root             | hot_set_ready  | partial_index | partial_index | partial_index |
| Generated-artifact-dominant     | hot_set_ready  | partial_index | partial_index | stale_index |
| Very large monorepo             | hot_set_ready  | partial_index | partial_index | reindexing |

Every non-`fully_indexed` row carries a labeled disclosure ref into
`docs/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md`
and a warming-state transition so the useful-before-ready behavior stays
observable.

## How the packet stays useful before fully warm

Every row publishes a first-useful-row latency in milliseconds and a
published budget. Quick-open lanes on every archetype emit a first useful
row before the cold lane finishes indexing. The packet refuses to certify
when a row reports edit input is blocked behind warm-up or when the
observed first-useful-row latency exceeds its budget without explicit
disclosure.

## How the packet protects degradation honesty

Every row whose `degradation` field is not `no_degradation` MUST carry a
`degradation_disclosure_ref` into the reviewer doc, and the row's
`readiness_state` MUST agree with the declared degradation class. The
narrowed corpus case
`fixtures/search/m4/monorepo_hot_set_truth/degradation_unlabeled_blocks_stable.json`
exercises the inverse: a large-single-root filename_index row reports
`hot_set_only` but drops the disclosure ref, and the packet blocks the
stable claim with `degradation_not_labeled`.

## How the packet protects warming-state visibility across projections

The packet binds a stable lane × archetype identity per row. Each of the
five required consumer projections (`search_shell`, `docs_help`,
`cli_headless`, `support_export`, `release_proof_index`) MUST preserve
the same packet id, lane identity, readiness vocabulary, degradation
labels, and responsiveness invariants and MUST support JSON export. The
narrowed corpus case
`fixtures/search/m4/monorepo_hot_set_truth/projection_drops_degradation_blocks_stable.json`
exercises the inverse: a docs/help projection drops degradation labels
and the packet blocks the stable claim with
`degradation_vocabulary_dropped` plus the upstream
`consumer_projection_drift`.

## How the packet protects responsiveness

The packet refuses to certify when a row reports `edit_input_unblocked =
false`, or when the observed first-useful-row latency exceeds the
published budget, or when a row claims `full_index_deferred_with_disclosure`
that disagrees with the readiness state. The narrowed corpus case
`fixtures/search/m4/monorepo_hot_set_truth/edit_input_blocked_blocks_stable.json`
exercises this path: a very-large-monorepo text_index row reports edit
input is blocked behind warm-up and the packet blocks the stable claim
with `edit_input_blocked_by_warmup`.

## How the packet protects warming transitions

Rows whose readiness state is not `fully_indexed` MUST record at least one
warming transition. The narrowed corpus case
`fixtures/search/m4/monorepo_hot_set_truth/missing_warming_transition_blocks_stable.json`
exercises the inverse: a polyglot-multi-root symbol_index row drops its
warming transition and the packet blocks the stable claim with
`missing_warming_transition`.

## Source contract refs

- Schema: `schemas/search/monorepo_hot_set_truth.schema.json`
- Reviewer doc: `docs/search/m4/finalize-monorepo-hot-set-indexing-warming-states-and.md`
- Rust implementation: `crates/aureline-search/src/monorepo_hot_set_truth/mod.rs`
- Fixture corpus: `fixtures/search/m4/monorepo_hot_set_truth/`
- Latency-truth packet: `artifacts/search/m4/quick_open_latency_truth_packet.json`
