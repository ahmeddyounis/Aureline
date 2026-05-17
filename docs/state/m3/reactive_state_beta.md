# Reactive-state and materialized-view invalidation beta

This reviewer doc is the contract for the reactive-state and
materialized-view invalidation beta. The beta gathers shell, search,
graph, AI, review, and support surfaces around one cross-surface
epoch-parity model so a single materialized view cannot project a
different epoch on one surface and quietly hide drift on another.

Each `materialized_view_case_record` binds one materialized view to:

- one `view_class` from the closed list:
  `ephemeral_projection`, `durable_local_materialization`,
  `exportable_snapshot`, `managed_replicated_view`;
- one `authority_label` from the closed list:
  `workspace_vfs`, `buffer_editor`, `derived_knowledge`, `execution`,
  `policy_entitlement`, `provider_overlay`;
- one monotonic `authority_epoch` produced by the authority;
- one `subscriber_epochs` entry per required consumer surface (`shell`,
  `search`, `graph`, `ai`, `review`, `support`) recording the epoch
  each surface has observed, its `observed_freshness`, and the
  `last_invalidation_cause` that produced its latest frame;
- one `parity_state` from the closed list (`aligned`,
  `drift_detected`, `awaiting_resync`, `terminal_unavailable`) that the
  evaluator re-derives from the subscriber rows;
- one `support_export` projection declaring the export posture and the
  metadata-safe baseline (no raw private material, no ambient
  authority, no destructive resets) so support bundles can preserve
  epoch state without re-running the underlying producer; and
- one `downgrade_label` from the closed reactive-views vocabulary
  (`none`, `red_blocks_beta_row`, `yellow_surface_partial`,
  `yellow_authority_skew`, `degraded_to_authority_only`,
  `stale_corpus_blocks_release_candidate`).

Implementation:
[`crates/aureline-reactive-state/src/reactive_views/mod.rs`](../../../crates/aureline-reactive-state/src/reactive_views/mod.rs).
Boundary schema:
[`schemas/state/materialized_view.schema.json`](../../../schemas/state/materialized_view.schema.json).
Protected fixture corpus:
[`fixtures/state/reactive_views_beta/`](../../../fixtures/state/reactive_views_beta/).
Baseline report:
[`artifacts/support/m3/reactive_state_beta_report.md`](../../../artifacts/support/m3/reactive_state_beta_report.md).
Integration drill:
[`crates/aureline-reactive-state/tests/reactive_views_beta.rs`](../../../crates/aureline-reactive-state/tests/reactive_views_beta.rs).

## Why this lane exists

The alpha subscription-envelope work (ADR
[`0005`](../../adr/0005-subscription-envelope-and-invalidation-semantics.md))
froze the per-frame envelope vocabulary: `freshness`, `completeness`,
`stale_reason`, `terminal_reason`, `authority`, `derivation`, and the
four materialized-view classes. Beta surfaces (shell, search, graph,
AI, review, support) have to share more than the envelope — they have
to share the same *view of which epoch is current* for every
materialized view they project. Without that shared epoch, a stale
search panel and a fresh review pane can disagree about the workspace
without either side knowing. The beta lane closes that gap by:

1. declaring one authority epoch per view, advanced only by the
   authority producer, and
2. recording, per consumer surface, the epoch that surface has
   observed and the invalidation cause it observed it through.

The evaluator then refuses claims like "parity_state = aligned" when
any subscriber lags the authority epoch or carries a non-authoritative
frame; reviewers see the drift in the matrix instead of having to
re-derive it from prose.

## Required coverage

The corpus seeds at least one case per `view_class`, and at least one
case must declare `parity_state` in `{drift_detected,
awaiting_resync}` so the cross-surface drift contract is exercised by
fixtures and not anecdotes.

| Required class | Seeded by |
| --- | --- |
| `ephemeral_projection` | `ephemeral_shell_status_aligned_case.yaml` |
| `durable_local_materialization` | `durable_workspace_index_aligned_case.yaml`, `derived_graph_neighborhood_resync_case.yaml` |
| `exportable_snapshot` | `exportable_support_snapshot_aligned_case.yaml` |
| `managed_replicated_view` | `managed_review_state_drift_case.yaml` |

Required surfaces (`shell`, `search`, `graph`, `ai`, `review`,
`support`) appear in every case; the evaluator refuses any case
missing a required `surface_kind` entry.

## What the evaluator refuses

- `parity_state = aligned` when any subscriber's `observed_epoch !=
  authority_epoch` or `observed_freshness != authoritative`.
- `parity_state = drift_detected` without at least one subscriber
  whose `observed_epoch < authority_epoch`.
- `parity_state = awaiting_resync` without at least one subscriber
  declaring `last_invalidation_cause = resync_required` or
  `observed_freshness` in `{stale, warming}`.
- `parity_state = terminal_unavailable` without at least one
  subscriber declaring `observed_freshness = unavailable`.
- `aligned` rows that carry any non-none `downgrade_label` or
  non-none `open_gap_class`.
- Downgraded rows (`drift_detected`, `awaiting_resync`,
  `terminal_unavailable`) that drop the closed downgrade label or
  fail to record at least one closed `open_gap`.
- Support-export projections that drop `view_class`,
  `authority_label`, `authority_epoch`, or `subscriber_epochs` from
  the exported metadata.
- Support-export projections that admit raw private material, ambient
  authority, or fail to preserve user-authored files.
- `exportable_snapshot` or `managed_replicated_view` rows that pin
  `posture = local_only`; these classes must declare
  `metadata_safe_export` or `held_record` so support bundles can
  preserve their epoch state.

## What this lane does NOT own

- Live transport for the subscription envelope — that contract lives
  in `aureline-reactive-state`'s alpha modules and ADR
  [`0005`](../../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Per-view payload bytes. The beta projection preserves epoch and
  authority truth in metadata-safe form; the payload bytes (search
  index files, graph caches, replicated review state) remain owned by
  their producers under the closed posture vocabulary.
- New invalidation causes or surface kinds. Extending either lands as
  a coordinated schema, Rust module, fixture, and reviewer-doc patch.

## Out of scope

- Live runtime measurement of cross-surface latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by the
  support-export pipeline and the chrome.
- Adding new downgrade labels, open-gap classes, parity states, or
  view classes without updating the schema, the Rust module, this
  reviewer doc, the baseline report, and the protected corpus
  together.
