# Workflow-bundle lifecycle compatibility report

- Corpus: `workspace.workflow_bundle_lifecycle.beta`
- Boundary: `aureline_workspace::bundles::WorkflowBundleReviewRecord`
- Replay: `cargo test -p aureline-qe --test workflow_bundle_lifecycle_conformance`
- Generated from the same source as the corpus fixtures and the certification
  freshness matrix; do not hand-edit drill rows.

This report is the published evidence that every claimed beta workflow-bundle row
stays declarative, reviewable, reversible, and certification-honest across the
install, update, rebase/adopt, keep-local, remove/rollback, drift-banner,
mirror-only, and offline lanes. The harness fails CI if any row drifts from the
pinned truth, if a badge over-claims stale evidence, if removal endangers a
user-owned asset, if a guardrail widens trust, or if this report or the
certification matrix stops covering a drill.

## Positive lifecycle rows

| Drill | Source | Effective badge | Support claim | Evidence | Mirror posture | Lifecycle flows |
| --- | --- | --- | --- | --- | --- | --- |
| `certified.full_lifecycle_live_or_mirror` | certified | certified | stable_launch_wedge_claim | fresh_current | live_or_mirror | install, update, rebase_adopt, keep_local, remove_rollback, drift_banner |
| `managed_approved.mirror_only_update` | managed_approved | managed_approved | managed_org_claim | fresh_current | mirror_only | install, update, keep_local, remove_rollback, drift_banner, mirror_only |
| `community.offline_install_experimental` | community | community | community_no_certification_claim | evidence_unknown | signed_offline_bundle | install, keep_local, remove_rollback, drift_banner, offline_install |
| `imported.round_trip_preserves_markers` | imported | imported | imported_pending_review_claim | imported_evidence | signed_offline_bundle | install, keep_local, remove_rollback, drift_banner, offline_install, imported_round_trip |
| `local_draft.keep_local_no_claim` | local_draft | local_draft | local_draft_no_claim | evidence_unknown | live_origin_only | install, keep_local, remove_rollback, drift_banner |
| `certified.stale_evidence_retest_pending` | certified | retest_pending | limited_retest_pending_claim | stale_past_window | live_or_mirror | update, drift_banner, remove_rollback, stale_evidence_downgrade |
| `managed_approved.stale_dependency_limited` | managed_approved | limited | managed_org_claim | stale_past_window | mirror_only | update, drift_banner, remove_rollback, mirror_only, stale_dependency_downgrade, stale_mirror_downgrade |
| `community.dependency_marker_propagation` | community | community | community_no_certification_claim | aging_within_window | live_or_mirror | install, update, keep_local, remove_rollback, drift_banner, dependency_markers |

## Automatic badge downgrades

| Drill | Trigger | Effective badge | Support |
| --- | --- | --- | --- |
| `community.offline_install_experimental` | stale/unknown evidence, experimental support promise | community | experimental |
| `certified.stale_evidence_retest_pending` | stale/unknown evidence | retest_pending | officially_supported |
| `managed_approved.stale_dependency_limited` | stale/unknown evidence, lifecycle-sensitive dependency, stale mirror | limited | legacy_deprecated |

## Dependency-marker propagation

| Drill | Capability markers | Lifecycle-sensitive dependencies |
| --- | --- | --- |
| `imported.round_trip_preserves_markers` | `capability_marker:imported_keymap.community_supported`, `capability_marker:imported_webview.host_specific` | `lifecycle_dependency:imported_extension.block_apply_preserve_data` |
| `managed_approved.stale_dependency_limited` | `capability_marker:remote_indexing.beta_only` | `lifecycle_dependency:certification_evidence.retest_window` |
| `community.dependency_marker_propagation` | `capability_marker:provider_completions.policy_gated`, `capability_marker:companion_pairing.host_specific`, `capability_marker:labs_inline_chat.labs` | `lifecycle_dependency:extension_set.update_sensitive`, `lifecycle_dependency:provider_link.remove_sensitive` |

## Negative drills

| Drill | Rejected because the message contains |
| --- | --- |
| `negative.certified_badge_on_stale_evidence` | `stale or retest-required evidence cannot render certified` |
| `negative.removal_marks_user_asset_safe` | `user_owned removable assets must be not_safe_to_remove_user_owned` |
| `negative.guardrail_widens_workspace_trust` | `guardrails forbid silent` |
| `negative.imported_overclaims_certified` | `imported source must remain imported pending review` |
| `negative.adopt_skips_change_preview` | `route through bundle_change_preview` |
| `negative.install_missing_certification_axis` | `missing required axes` |
| `negative.support_export_enables_raw_secret` | `support_export raw export booleans must remain false` |
