# M5 Review, Merge-Queue, and Pipeline Maturity Matrix

- Packet: `m5-review-ci-preview-matrix:stable:0001`
- Schema: `schemas/review/freeze-the-m5-review-workspace-merge-queue-and-pipeline-viewer-maturity-matrix.schema.json`
- Support export: `artifacts/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/support_export.json`
- Contract doc: `docs/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md`
- Fixtures: `fixtures/review/m5/freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix/`

## Coverage

- The review workspace is qualified Stable: anchors stay durable across edits, rebases, and reopens, stale-base and outdated-diff states are labeled rather than hidden, and approval state resets on base change.
- The merge queue is qualified Stable: CI-status truth stays fresh relative to the head it gates, and every rerun and cancel action stays individually attributable and reviewable with no hidden write scope.
- The pipeline viewer is qualified Stable: logs and artifacts render through the safe-preview boundary, suspicious content stays neutralized, and no raw build log or artifact body crosses the support boundary.
- Remote preview is qualified Beta: routes are time-bounded and auto-expire at their bound while staying attributable to their opener and origin. Persistent hosting and unbounded sharing stay out of scope.
- Every lane carries required evidence packet refs, downgrade triggers, rollback posture, and consumer-surface parity.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Trust guardrails

The matrix proves that no provider overlay, browser handoff, or preview server creates hidden write scope or stale truth. Review-workspace anchors stay durable and stale bases are labeled, merge-queue truth stays fresh while rerun/cancel authority stays attributable, pipeline logs and artifacts stay safe-previewed, and remote preview routes stay time-bounded and attributable. Stale or underqualified rows automatically narrow before publication rather than hiding the lane.
