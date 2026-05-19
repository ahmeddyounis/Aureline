# Deployment-profile continuity corpus: control-plane vs. data-plane drills

This directory holds the outage-drill pages that prove the deployment
contract's control-plane vs. data-plane separation. Each
`drills/<drill_id>.json` file is a
[`DeploymentProfilePage`](../../../../crates/aureline-shell/src/deployment_profile/mod.rs)
record that names one of the closed
`outage_drill_class` values:

- `control_plane_unavailable` — vendor control plane unreachable; local
  data plane stays local-safe.
- `data_plane_blocked_pending_reconnect` — remote attach transport drops;
  local-core capabilities continue.
- `mirror_only_fallback` — live control-plane fetch suppressed;
  mirror-only routing engaged with offline-parity guardrail.
- `offline_cache_only` — offline cache active inside the grace window;
  cached last-known-good is served under an explicit stale label.
- `sign_out_to_local_only` — sign-out collapses the install to the
  individual-local baseline.
- `org_switch_boundary_recheck` — org switch requires tenant and region
  boundary recheck.
- `seat_loss_continue_local` — seat is revoked; local-core continuity is
  preserved.
- `region_mismatch_boundary_recheck` — remote target region differs from
  the customer-pinned region.
- `stale_policy_cache` — policy service offline; cache serves
  last-known-good.
- `stale_catalog_cache` — catalog service offline; cache serves
  last-known-good.

Every drill page MUST pass `DeploymentProfilePage::audit()` with an
empty defect set; the corpus replay test enforces that and the
`safest_next_action` invariant (where local-safe data-plane work remains
and control-plane is healthy, the safest next action MUST be
`continue_local` or `await_resolution`).

The drills are bound to the corpus packet at
[`../profile_truth/packet.json`](../profile_truth/packet.json) and the
release-evidence excerpt at
[`artifacts/release/m3/deployment_profile_conformance_report.md`](../../../../artifacts/release/m3/deployment_profile_conformance_report.md).
