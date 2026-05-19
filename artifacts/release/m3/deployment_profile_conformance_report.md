# Deployment-profile conformance report

This report is the M3 beta-exit conformance excerpt for the deployment-profile continuity corpus. It binds the matrix of marketed deployment rows (individual-local, self-hosted / sovereign, enterprise-online hybrid, enterprise-online mirror-only, air-gapped, managed-cloud) and the control-plane vs. data-plane outage drills to the shell's `DeploymentProfilePage::audit()` invariant. Every case below resolves through one inspectable `DeploymentProfilePage` whose audit MUST return an empty defect set.

Bound contract: [`docs/ops/m3/deployment_profile_and_continuity_beta.md`](../../../docs/ops/m3/deployment_profile_and_continuity_beta.md).

Bound qualification doc: [`docs/ops/m3/deployment_profile_claim_qualification.md`](../../../docs/ops/m3/deployment_profile_claim_qualification.md).

## Coverage summary

- Marketed-row cases: **17**
- Outage drills: **10**
- Residual-dependency rows: **11**
- All cases pass `audit()`: **true**
- All drills pass `audit()`: **true**
- Profiles present: `individual_local`, `self_hosted`, `enterprise_online`, `air_gapped`, `managed_cloud`
- Surface lenses present: `desktop`, `cli_headless`, `companion_handoff`, `support_export`
- Outage drill classes present: `control_plane_unavailable`, `data_plane_blocked_pending_reconnect`, `mirror_only_fallback`, `offline_cache_only`, `sign_out_to_local_only`, `org_switch_boundary_recheck`, `seat_loss_continue_local`, `region_mismatch_boundary_recheck`, `stale_policy_cache`, `stale_catalog_cache`

## Marketed-row matrix

| Case id | Profile | Product label | Surface lens | Control plane | Data plane | Safest next action |
|---|---|---|---|---|---|---|
| `individual_local_desktop` | `individual_local` | `desktop_local_first` | `desktop` | `not_applicable` | `available_local_safe` | `continue_local` |
| `individual_local_cli_headless` | `individual_local` | `desktop_local_first` | `cli_headless` | `not_applicable` | `available_local_safe` | `continue_local` |
| `individual_local_support_export` | `individual_local` | `desktop_local_first` | `support_export` | `not_applicable` | `available_local_safe` | `continue_local` |
| `self_hosted_sovereign_desktop` | `self_hosted` | `self_hosted_sovereign` | `desktop` | `healthy` | `available_local_safe` | `continue_local` |
| `self_hosted_sovereign_cli_headless` | `self_hosted` | `self_hosted_sovereign` | `cli_headless` | `healthy` | `available_local_safe` | `continue_local` |
| `self_hosted_sovereign_support_export` | `self_hosted` | `self_hosted_sovereign` | `support_export` | `healthy` | `available_local_safe` | `continue_local` |
| `enterprise_online_hybrid_desktop` | `enterprise_online` | `hybrid_remote_attach` | `desktop` | `healthy` | `available_local_safe` | `continue_local` |
| `enterprise_online_hybrid_companion_handoff` | `enterprise_online` | `hybrid_remote_attach` | `companion_handoff` | `healthy` | `available_local_safe` | `continue_local` |
| `enterprise_online_hybrid_support_export` | `enterprise_online` | `hybrid_remote_attach` | `support_export` | `healthy` | `available_local_safe` | `continue_local` |
| `enterprise_online_mirrored_desktop` | `enterprise_online` | `hybrid_remote_attach` | `desktop` | `mirror_only` | `available_local_safe` | `continue_local` |
| `enterprise_online_mirrored_support_export` | `enterprise_online` | `hybrid_remote_attach` | `support_export` | `mirror_only` | `available_local_safe` | `continue_local` |
| `air_gapped_mirror_only_desktop` | `air_gapped` | `air_gapped_mirror_only` | `desktop` | `not_applicable` | `available_local_safe` | `continue_local` |
| `air_gapped_mirror_only_cli_headless` | `air_gapped` | `air_gapped_mirror_only` | `cli_headless` | `not_applicable` | `available_local_safe` | `continue_local` |
| `air_gapped_mirror_only_support_export` | `air_gapped` | `air_gapped_mirror_only` | `support_export` | `not_applicable` | `available_local_safe` | `continue_local` |
| `managed_cloud_desktop` | `managed_cloud` | `browser_companion_handoff_default_home` | `desktop` | `healthy` | `available_local_safe` | `continue_local` |
| `managed_cloud_companion_handoff` | `managed_cloud` | `browser_companion_handoff_default_home` | `companion_handoff` | `healthy` | `available_local_safe` | `continue_local` |
| `managed_cloud_support_export` | `managed_cloud` | `browser_companion_handoff_default_home` | `support_export` | `healthy` | `available_local_safe` | `continue_local` |

## Outage drills

| Drill id | Class | Profile | Control plane | Data plane | Safest next action | Local-safe remains |
|---|---|---|---|---|---|---|
| `control_plane_unavailable` | `control_plane_unavailable` | `managed_cloud` | `unavailable` | `available_local_safe` | `continue_local` | true |
| `data_plane_blocked_pending_reconnect` | `data_plane_blocked_pending_reconnect` | `enterprise_online` | `healthy` | `available_local_safe` | `continue_local` | true |
| `mirror_only_fallback` | `mirror_only_fallback` | `enterprise_online` | `mirror_only` | `available_local_safe` | `continue_local` | true |
| `offline_cache_only` | `offline_cache_only` | `enterprise_online` | `stale_cache` | `available_local_safe` | `continue_local` | true |
| `sign_out_to_local_only` | `sign_out_to_local_only` | `individual_local` | `not_applicable` | `available_local_safe` | `continue_local` | true |
| `org_switch_boundary_recheck` | `org_switch_boundary_recheck` | `enterprise_online` | `boundary_recheck_required` | `available_local_safe` | `continue_local` | true |
| `seat_loss_continue_local` | `seat_loss_continue_local` | `enterprise_online` | `boundary_recheck_required` | `available_local_safe` | `continue_local` | true |
| `region_mismatch_boundary_recheck` | `region_mismatch_boundary_recheck` | `enterprise_online` | `boundary_recheck_required` | `available_local_safe` | `continue_local` | true |
| `stale_policy_cache` | `stale_policy_cache` | `self_hosted` | `stale_cache` | `available_local_safe` | `continue_local` | true |
| `stale_catalog_cache` | `stale_catalog_cache` | `self_hosted` | `stale_cache` | `available_local_safe` | `continue_local` | true |

## Continuity assertions

- `control_plane_unavailable`: Control-plane impairment never collapses into generic 'service degraded' copy while local-safe data-plane work remains; the safest next action stays continue_local.
- `data_plane_blocked_pending_reconnect`: Remote disconnect does not turn the desktop into a thin client; local-core capabilities continue independent of remote-agent reconnect.
- `mirror_only_fallback`: Mirror-only fallback names the offline-parity guardrail and emits at least one mirror/offline artifact row; live fetch claim is suppressed.
- `offline_cache_only`: Cached last-known-good is served under an explicit stale label; control-plane recovery is offered as an alternate action.
- `sign_out_to_local_only`: Sign-out preserves local-core continuity; managed surfaces are not silently retained as claimed.
- `org_switch_boundary_recheck`: Org-switch keeps local-core continuity available and surfaces RecheckBoundary as the explicit recovery action; managed surfaces are not silently inherited.
- `seat_loss_continue_local`: Seat loss does not strip local-core capability or evict cached workspace state; ContinueLocal remains the safest next action.
- `region_mismatch_boundary_recheck`: Region mismatch pauses cross-region writes and offers an explicit RecheckBoundary path; local-core continuity is preserved.
- `stale_policy_cache`: Stale policy cache is served under an explicit stale label; Retry policy sync is offered as an alternate action without blocking local-core work.
- `stale_catalog_cache`: Stale catalog cache is served under an explicit stale label; local-core continuity continues without depending on the catalog.

## Residual-dependency matrix

Per-profile posture, projected from `artifacts/governance/residual_dependencies.yaml`. The full JSON projection lives at `artifacts/release/m3/residual_dependency_matrix.json`.

| Dependency | Individual local | Self-hosted | Enterprise online | Air-gapped | Managed cloud |
|---|---|---|---|---|---|
| `sign_in` | `not_applicable_structural` | `required` | `required` | `optional` | `required` |
| `package_registry` | `optional` | `required` | `required` | `mirrored` | `required` |
| `remote_mirror` | `optional` | `optional` | `optional` | `required` | `optional` |
| `remote_agent` | `not_applicable_structural` | `optional` | `optional` | `forbidden` | `optional` |
| `symbol_service` | `not_applicable_structural` | `optional` | `optional` | `mirrored` | `optional` |
| `ai_provider` | `optional` | `optional` | `optional` | `forbidden` | `required` |
| `policy_bundle` | `optional` | `required` | `required` | `mirrored` | `required` |
| `docs_pack` | `cached` | `cached` | `cached` | `mirrored` | `cached` |
| `browser_handoff` | `optional` | `optional` | `optional` | `forbidden` | `required` |
| `companion_notification_channel` | `not_applicable_structural` | `optional` | `optional` | `forbidden` | `required` |
| `hosted_control_plane_reachability` | `not_applicable_structural` | `optional` | `required` | `forbidden` | `required` |

## Verification

```bash
cargo test -p aureline-shell --test deployment_profile_corpus_fixtures
```

The test loads every fixture under `fixtures/deployment/m3/profile_truth/` and `fixtures/deployment/m3/control_plane_vs_data_plane/`, deserializes each one through the shared `DeploymentProfilePage` shape, and asserts that `DeploymentProfilePage::audit()` returns an empty defect set. It also asserts the fixture corpus matches the seeded packet and that the rendered conformance report and residual-dependency matrix match this packet byte-for-byte.
