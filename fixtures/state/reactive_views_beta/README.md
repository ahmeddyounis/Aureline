# Reactive-state and materialized-view invalidation beta corpus

Protected fixture corpus for the reactive-state and materialized-view
invalidation beta. Each fixture is one
`materialized_view_case_record` bound to:

- one materialized-view class from the closed list:
  `ephemeral_projection`, `durable_local_materialization`,
  `exportable_snapshot`, `managed_replicated_view`,
- one authority label from the closed list:
  `workspace_vfs`, `buffer_editor`, `derived_knowledge`, `execution`,
  `policy_entitlement`, `provider_overlay`,
- one monotonic `authority_epoch`,
- one `subscriber_epoch` entry per required consumer surface (`shell`,
  `search`, `graph`, `ai`, `review`, `support`),
- one `epoch_parity_state` from the closed list (`aligned`,
  `drift_detected`, `awaiting_resync`, `terminal_unavailable`) that
  the evaluator re-derives from the subscriber rows,
- one `support_export` projection declaring the export posture and
  the metadata-safe baseline, and
- one closed `downgrade_label` from the reactive-views vocabulary.

A failing row downgrades using the closed `downgrade_label` list; no
ad-hoc vocabulary is admitted. Open gaps are drawn from the closed
`open_gap_class` enumeration so reviewer matrix entries stay
auditable.

Boundary schema:
[`schemas/state/materialized_view.schema.json`](../../../schemas/state/materialized_view.schema.json).

Crate consumer:
[`crates/aureline-reactive-state/src/reactive_views/mod.rs`](../../../crates/aureline-reactive-state/src/reactive_views/mod.rs).

Reviewer doc:
[`docs/state/m3/reactive_state_beta.md`](../../../docs/state/m3/reactive_state_beta.md).

Baseline report:
[`artifacts/support/m3/reactive_state_beta_report.md`](../../../artifacts/support/m3/reactive_state_beta_report.md).
