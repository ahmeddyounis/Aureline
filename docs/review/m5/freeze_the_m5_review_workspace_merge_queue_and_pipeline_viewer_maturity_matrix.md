# M5 Review-Workspace, Merge-Queue, and Pipeline-Viewer Maturity Matrix

This document is the contract for the frozen M5 matrix that qualifies four
review, CI, and preview lanes. The matrix is the canonical M5 control source for
this lane: dashboards, docs, Help/About surfaces, and support exports ingest the
checked-in packet rather than cloning status text.

- Record kind: `freeze_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix`
- Schema: [`schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json`](../../../schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json)
- Canonical support export: [`artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/support_export.json`](../../../artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/support_export.json)
- Summary artifact: [`artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md`](../../../artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md)
- Fixtures: [`fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/`](../../../fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/)
- Producer: `aureline_review::current_stable_m5_review_ci_preview_matrix_export`

## Lanes

| Lane | Qualification | Source contract |
| --- | --- | --- |
| `review_workspace` | Stable | [`schemas/review/review_workspace.schema.json`](../../../schemas/review/review_workspace.schema.json) |
| `merge_queue` | Stable | [`schemas/review/merge_queue_entry.schema.json`](../../../schemas/review/merge_queue_entry.schema.json) |
| `pipeline_viewer` | Stable | [`schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json) |
| `remote_preview` | Beta | [`schemas/runtime/preview_route.schema.json`](../../../schemas/runtime/preview_route.schema.json) |

Each lane row binds a qualification class to its evidence requirement, required
evidence packet refs, downgrade triggers, rollback posture, source contracts, and
the consumer surfaces that must project the lane's qualification truth.

## Track invariant

Review-workspace anchors stay durable, merge-queue truth stays fresh, pipeline
logs/artifacts stay safe-previewed, and remote preview routes stay time-bounded
and attributable. The `trust_review` block encodes these as hard invariants — all
must hold for the matrix to validate:

- `review_workspace_anchors_durable` and `stale_base_labels_explicit` — anchors
  survive edits, rebases, and reopens, and stale-base or outdated-diff states are
  labeled rather than silently hidden.
- `merge_queue_truth_fresh` and `rerun_cancel_authority_attributable` — CI-status
  truth stays fresh relative to the head it gates, and every rerun or cancel
  action stays individually attributable and reviewable.
- `pipeline_logs_artifacts_safe_previewed` — pipeline logs and artifacts render
  through the safe-preview boundary; no raw build log or artifact body crosses
  the support boundary.
- `remote_preview_time_bounded` and `remote_preview_attributable` — remote
  preview routes auto-expire at their bound and stay attributable to their opener
  and origin.
- `browser_handoff_return_path_safe`, `no_hidden_write_scope`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected lane. The supported
downgrade triggers are `proof_stale`, `policy_blocked`, `merge_queue_status_stale`,
`anchor_drift`, `safe_preview_unavailable`, `preview_route_expired`,
`trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/)
show a merge-queue narrowing on stale CI status and a held remote-preview lane;
both remain valid packets because narrowing is explicit, not hidden.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live preview origin responses never cross this boundary. The
packet carries only metadata, qualification truth, and contract references. Every
rerun, cancel, merge-queue, preview, or publish-later action stays attributable
and reviewable.
