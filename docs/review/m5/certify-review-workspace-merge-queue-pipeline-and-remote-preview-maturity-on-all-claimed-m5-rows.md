# M5 Review-Workspace, Merge-Queue, Pipeline, and Remote-Preview Certification

This document is the contract for the M5 certification packet that qualifies
every claimed M5 review, CI, and preview *row* on the mainline branch. Where the
[frozen maturity matrix](freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md)
locks four lanes at lane granularity, this packet certifies the individual rows
that feed those lanes and aggregates their verdicts into a single promotion
verdict. Dashboards, docs, Help/About surfaces, and support exports ingest the
checked-in packet rather than cloning status text.

- Record kind: `certify_review_workspace_merge_queue_pipeline_and_remote_preview_maturity_on_all_claimed_m5_rows`
- Schema: [`schemas/review/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.schema.json`](../../../schemas/review/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.schema.json)
- Canonical support export: [`artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/support_export.json`](../../../artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/support_export.json)
- Summary artifact: [`artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md`](../../../artifacts/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows.md)
- Fixtures: [`fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/`](../../../fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/)
- Producer / first consumer: `aureline_review::certify_from_current_exports`
- Reader: `aureline_review::current_m5_review_certification_export`

## Certified rows

Each row binds a certification verdict to its upstream evidence packet — the
record kind, support-export artifact, schema, and contract doc that back the
claim — plus the downgrade triggers, rollback posture, and proof freshness for
that row.

| Row | Lane | Claimed | Verdict | Evidence packet |
| --- | --- | --- | --- | --- |
| `durable_review_header` | `review_workspace` | Stable | Certified | [durable review headers](implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md) |
| `merge_queue_readiness` | `merge_queue` | Stable | Certified | [merge-queue readiness](add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md) |
| `pipeline_viewer` | `pipeline_viewer` | Stable | Certified | [pipeline viewer](implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md) |
| `remote_preview_route` | `remote_preview` | Beta | Narrowed | [remote preview routes](add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure.md) |
| `rerun_cancel_review` | `merge_queue` | Stable | Certified | [rerun/cancel actions](ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md) |
| `evidence_card` | `review_workspace` | Stable | Certified | [AI review evidence cards](ship_ai_review_evidence_finding_cards_and_review_pack_integration_with_change_objects.md) |
| `review_export_bundle` | `review_workspace` | Stable | Certified | [review/export bundles](add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md) |
| `maturity_matrix` | `cross_cutting` | Stable | Certified | [maturity matrix](freeze_the_m5_review_workspace_merge_queue_and_pipeline_viewer_maturity_matrix.md) |

## First-consumer certification

`certify_from_current_exports` is the first real consumer of the claimed rows: it
reads each row's checked-in support export through that row's own producer and
certifies the row only when its evidence currently validates. A row whose
upstream export fails to parse or validate is recorded as `blocked`, so a stale
or underqualified row narrows the certification automatically instead of leaving
it greener than the evidence. The checked-in support export is exactly the packet
that function produces, so `cargo test -p aureline-review` fails if any upstream
export drifts.

## Compatibility report

`compatibility_report` aggregates the per-row verdicts: counts of certified,
narrowed, blocked, and uncertified rows; `all_rows_publishable` (true only when
no row is blocked or uncertified); and a human-readable promotion note. The
report must agree with the row verdicts or the packet fails validation, so the
report can never claim more than the rows prove.

## Downgrade automation and freshness

`apply_downgrade_automation` takes per-row observations (evidence validity, proof
freshness, upstream narrowing) and narrows the affected rows: invalid evidence
moves a row to `blocked`; stale proof or a narrowed upstream narrows a `certified`
row to `narrowed_certified`; the per-row `proof_fresh` flag is updated. The report
is recomputed so CI or release tooling can fail promotion or narrow the claim
automatically. The packet-level `proof_freshness` carries the SLO (168 hours) and
last-refresh timestamp; `auto_narrow_on_stale` records that stale proof narrows
the certification. Supported downgrade triggers are `proof_stale`,
`evidence_packet_invalid`, `policy_blocked`, `merge_queue_status_stale`,
`anchor_drift`, `safe_preview_unavailable`, `preview_route_expired`,
`trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/certify-review-workspace-merge-queue-pipeline-and-remote-preview-maturity-on-all-claimed-m5-rows/)
show a merge-queue row blocked on invalid evidence and a durable-header row
narrowed on stale proof; both remain valid packets because narrowing is explicit,
not hidden.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live preview origin responses never cross this boundary. The
packet carries only metadata, certification verdicts, and contract references.
Every rerun, cancel, merge-queue, preview, or publish-later action behind these
rows stays attributable and reviewable.
