# Reactive-state and materialized-view invalidation beta baseline report

This artifact is the reviewer-facing baseline rendering of the
reactive-view report produced by the
[`reactive_views`](../../../crates/aureline-reactive-state/src/reactive_views/mod.rs)
module from the protected corpus under
[`/fixtures/state/reactive_views_beta/`](../../../fixtures/state/reactive_views_beta/).
It records the authority epoch, cross-surface epoch parity state,
support-export posture, downgrade label, and open-gap classes for
every claimed materialized view in the beta corpus. The report stays
metadata-safe: it never carries raw private material or ambient
authority, and every row is drawn from the closed reactive-views
vocabularies.

Schema: `schemas/state/materialized_view.schema.json`
(record kind `materialized_view_report_record`, version 1).
Reviewer doc: [`docs/state/m3/reactive_state_beta.md`](../../../docs/state/m3/reactive_state_beta.md).
Corpus manifest:
[`fixtures/state/reactive_views_beta/manifest.yaml`](../../../fixtures/state/reactive_views_beta/manifest.yaml).

## Matrix rows

| View ID | View class | Authority label | Authority epoch | Parity state | Subscriber epoch range | Support export posture | Downgrade label | Open-gap classes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `view:graph_neighborhood` | `durable_local_materialization` | `derived_knowledge` | 88 | `awaiting_resync` | 86–88 | `local_only` | `degraded_to_authority_only` | `drift_recovery_manual` |
| `view:managed_review_state` | `managed_replicated_view` | `execution` | 311 | `drift_detected` | 310–311 | `metadata_safe_export` | `yellow_surface_partial` | `replication_pending` |
| `view:shell_chrome_status` | `ephemeral_projection` | `workspace_vfs` | 42 | `aligned` | 42–42 | `local_only` | `none` | `none` |
| `view:support_evidence_snapshot` | `exportable_snapshot` | `policy_entitlement` | 9 | `aligned` | 9–9 | `metadata_safe_export` | `none` | `none` |
| `view:workspace_search_index` | `durable_local_materialization` | `derived_knowledge` | 187 | `aligned` | 187–187 | `local_only` | `none` | `none` |

## Per-view-class summary

| View class | Cases | Aligned | Drift detected | Awaiting resync | Terminal unavailable | Downgrade required |
| --- | --- | --- | --- | --- | --- | --- |
| `ephemeral_projection` | 1 | 1 | 0 | 0 | 0 | 0 |
| `durable_local_materialization` | 2 | 1 | 0 | 1 | 0 | 1 |
| `exportable_snapshot` | 1 | 1 | 0 | 0 | 0 | 0 |
| `managed_replicated_view` | 1 | 0 | 1 | 0 | 0 | 1 |

The two non-aligned rows (`view:managed_review_state` and
`view:graph_neighborhood`) carry closed downgrade labels
(`yellow_surface_partial`, `degraded_to_authority_only`) and at least
one closed open-gap entry (`replication_pending`,
`drift_recovery_manual`). Every aligned row declares
`downgrade_label = none` and no open gaps. The evaluator refuses any
deviation from these contracts.

## Open gaps

- `view:managed_review_state` (`replication_pending`): the managed
  review replica lags by one epoch on the AI and support surfaces
  until the next replication tick lands.
- `view:graph_neighborhood` (`drift_recovery_manual`): graph and AI
  subscribers must be resynced by re-running the derived neighborhood
  producer; automatic resync from an external-change cause is pending.

## Safety baseline

- `raw_private_material_excluded = true` on every case and the report.
- `ambient_authority_excluded = true` on every case and the report.
- `destructive_resets_present = false` on every case.
- `preserves_user_authored_files = true` on every case and on every
  support-export projection.

## Out-of-scope

- Live runtime measurement of cross-surface latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by the
  support-export pipeline and the chrome.
- Adding new downgrade labels, open-gap classes, parity states, or
  view classes without updating the schema, the Rust module, the
  reviewer doc, this report, and the protected corpus together.
