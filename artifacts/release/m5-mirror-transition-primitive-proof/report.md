# M5 Mirror-Transition Primitive: Mirror/Offline Artifact Rows, Mode-Change Review Sheet, and Channel-Association Review Row

- Packet: `m5-mirror-transition-primitive:stable:0001`
- Label: `M5 Mirror-Transition Primitive: Mirror/Offline Artifact Rows, Mode-Change Review Sheet, and Channel-Association Review Row`
- Mirror surfaces: 6 / 6
- Artifact classes: docs, extensions, models, updates, policy_bundles
- Continuity states: current_verified, needs_refresh, offline_cache_only, mirror_unavailable, verification_failed, pinned_offline
- Cache dispositions: reuse_valid, invalidate_stale, rebuild_required, preserve_pinned

## Mirror surfaces

- **Update-center mirror surface**
  - Owner: Update-center guild
  - Scope: Update-center mirror artifacts with verify / open-manifest actions during a release-channel switch
  - Worked cases: 1
    - `transition:update-center:0001` → managed→managed via `channel_switch`, artifacts `2`, posture `current_verified`, cache `reuse_valid`, rollback `available`
- **Mirror-manager surface**
  - Owner: Mirror-manager guild
  - Scope: Mirror-manager marking a stale self-hosted mirror needs-refresh before a mirror re-attach
  - Worked cases: 1
    - `transition:mirror-manager:0002` → self_hosted→self_hosted via `mirror_reattach`, artifacts `1`, posture `needs_refresh`, cache `rebuild_required`, rollback `requires_checkpoint`
- **Admin deployment console**
  - Owner: Deployment-admin guild
  - Scope: Admin console reviewing a managed-to-air-gapped disconnect with cached-offline artifacts and a checkpoint rollback
  - Worked cases: 1
    - `transition:admin-disconnect:0003` → managed→air_gapped via `online_offline_transition`, artifacts `2`, posture `offline_cache_only`, cache `preserve_pinned`, rollback `requires_checkpoint`
- **Diagnostics mirror pane**
  - Owner: Diagnostics guild
  - Scope: Diagnostics mirror pane surfacing a failed signature verification and invalidating the stale cache
  - Worked cases: 1
    - `transition:diagnostics-verify:0004` → desktop→desktop via `channel_switch`, artifacts `1`, posture `verification_failed`, cache `invalidate_stale`, rollback `available`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing an imported-truth state-root migration and its rollback path
  - Worked cases: 1
    - `transition:support-replay:0005` → managed→managed via `state_root_migration`, artifacts `1`, posture `needs_refresh`, cache `rebuild_required`, rollback `available`
- **Docs mirror reference**
  - Owner: Docs / help guild
  - Scope: Docs mirror reference framing a desktop install with a current, verified first-party mirror
  - Worked cases: 1
    - `transition:docs-reference:0006` → desktop→desktop via `channel_switch`, artifacts `1`, posture `current_verified`, cache `reuse_valid`, rollback `not_applicable`
