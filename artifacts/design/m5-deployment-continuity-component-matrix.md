# M5 Deployment/Continuity Component Matrix

- Packet: `m5-deployment-continuity-component-matrix:stable:0001`
- Label: `M5 Deployment/Continuity Component Matrix`
- Components: 10 across 9 / 9 families (5 degraded)

## Components

- **component:install-profile-card:0001** (install_profile_card) — Install-profile card on the About page for a managed install
  - An install-profile card keeps install mode, channel, updater owner, and durable state roots explicit
  - family=install_profile_card truth=live mode=managed export_safe=true assistive=true
- **component:install-profile-card:0002** (install_profile_card) — Install-profile card for a portable install with an unavailable state root
  - An install-profile card discloses that a portable state root is currently unavailable rather than imply a fully resolved install
  - family=install_profile_card truth=cached_offline mode=portable export_safe=true assistive=true
  - Degraded: trigger=state_root_unavailable — The portable drive holding this install's durable state root is not mounted; the card names the expected root and offers a re-attach route
- **component:side-by-side-import-sheet:0001** (side_by_side_import_sheet) — Side-by-side import sheet for a preview install next to stable
  - A side-by-side import sheet keeps handler ownership inspectable and never captures the default handler from the other install
  - family=side_by_side_import_sheet truth=live mode=desktop export_safe=true assistive=true
- **component:rollout-ring-row:0001** (rollout_ring_row) — Rollout-ring row for a held canary ring
  - A rollout-ring row discloses that this fleet sits in a held canary ring rather than imply general availability
  - family=rollout_ring_row truth=live mode=managed export_safe=true assistive=true
  - Degraded: trigger=rollout_paused — Promotion for this canary ring is held pending a gate; the row names the ring and keeps a rollback path available
- **component:deployment-summary-card:0001** (deployment_summary_card) — Deployment summary card for a self-hosted tenant
  - A deployment summary card keeps operating mode, tenant/region, and both control-plane and data-plane status visible
  - family=deployment_summary_card truth=live mode=self_hosted export_safe=true assistive=true
- **component:residual-dependency-row:0001** (residual_dependency_row) — Residual-dependency row for a self-hosted license-activation dependency
  - A residual-dependency row keeps a remaining vendor dependency explicit rather than let a self-hosted claim read as fully independent
  - family=residual_dependency_row truth=live mode=self_hosted export_safe=true assistive=true
  - Degraded: trigger=residual_vendor_dependency — This self-hosted install still contacts the vendor for periodic license activation; the row names the dependency and its cadence
- **component:control-plane-data-plane-status-strip:0001** (control_plane_data_plane_status_strip) — Control-plane/data-plane status strip during a managed control-plane outage
  - A status strip keeps control-plane and data-plane distinct so a control-plane outage never reads as a broken local runtime
  - family=control_plane_data_plane_status_strip truth=live mode=managed export_safe=true assistive=true
  - Degraded: trigger=control_plane_impaired — The managed control plane is unreachable; local editing and runtime continue, and policy sync will resume when the control plane returns
- **component:mirror-offline-artifact-row:0001** (mirror_offline_artifact_row) — Mirror/offline artifact row for a stale mirrored update artifact
  - A mirror/offline artifact row discloses freshness and signature truth so stale mirrored content never reads as a live source
  - family=mirror_offline_artifact_row truth=cached_offline mode=air_gapped export_safe=true assistive=true
  - Degraded: trigger=mirror_stale — This artifact came from offline media last synced 9 days ago; it is shown as cached-offline, not as a current live source
- **component:mode-change-review-sheet:0001** (mode_change_review_sheet) — Mode-change review sheet for a desktop-to-managed channel switch
  - A mode-change review sheet shows the cache reuse and rollback consequences before a durable boundary change, never after
  - family=mode_change_review_sheet truth=live mode=desktop export_safe=true assistive=true
- **component:channel-association-review-row:0001** (channel_association_review_row) — Channel-association review row for a protocol-handler change
  - A channel-association review row discloses the current owner and reviews the change before apply, never silently capturing the handler
  - family=channel_association_review_row truth=live mode=desktop export_safe=true assistive=true
