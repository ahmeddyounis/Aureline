# M5 Deployment-Profile Primitive: Install-Profile Card, Side-by-Side Import Sheet, and Rollout-Ring Row

- Packet: `m5-deployment-profile-primitive:stable:0001`
- Label: `M5 Deployment-Profile Primitive: Install-Profile Card, Side-by-Side Import Sheet, and Rollout-Ring Row`
- Deployment surfaces: 6 / 6
- Install scopes: per_user, per_machine, portable, offline
- Rollback targets: checkpoint_available, prior_build_retained, no_rollback, unknown
- State-sharing models: isolated, shared_read_only, shared_writable, one_time_copy

## Deployment surfaces

- **About / install-profile card**
  - Owner: Install-profile guild
  - Scope: About-page install-profile card naming mode, scope, channel, updater owner, roots, and rollback
  - Worked cases: 2
    - `deployment:desktop-per-user:0001` → mode `desktop`/`per_user`, rollback `checkpoint_available`, ring `general_availability`/`promoted`
    - `deployment:portable-offline:0002` → mode `portable`/`portable`, rollback `prior_build_retained`, ring `general_availability`/`promoted`
- **Update center**
  - Owner: Update-center guild
  - Scope: Update-center rollout-ring row naming channel, ring, promotion state, and rollback path
  - Worked cases: 1
    - `deployment:managed-broad:0006` → mode `managed`/`per_machine`, rollback `prior_build_retained`, ring `broad`/`promoted`
- **Admin fleet console**
  - Owner: Fleet-rollout guild
  - Scope: Admin fleet console preserving ring owner, platform scope, and promotion evidence
  - Worked cases: 1
    - `deployment:managed-canary:0005` → mode `managed`/`per_machine`, rollback `checkpoint_available`, ring `canary`/`held`
- **Side-by-side review**
  - Owner: Side-by-side handoff guild
  - Scope: Side-by-side import sheet naming shared-vs-isolated state and preserving a checkpoint before moves
  - Worked cases: 2
    - `deployment:side-by-side-copy:0003` → mode `desktop`/`per_user`, rollback `checkpoint_available`, ring `early_adopter`/`promoted`
    - `deployment:side-by-side-shared:0004` → mode `desktop`/`per_user`, rollback `prior_build_retained`, ring `early_adopter`/`promoted`
- **Diagnostics deployment pane**
  - Owner: Diagnostics guild
  - Scope: Diagnostics deployment pane keeping the rollback target explicit even when unknown
  - Worked cases: 1
    - `deployment:self-hosted-diagnostics:0007` → mode `self_hosted`/`per_machine`, rollback `unknown`, ring `general_availability`/`promoted`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing deployment truth from an imported snapshot
  - Worked cases: 1
    - `deployment:support-replay:0008` → mode `desktop`/`per_user`, rollback `prior_build_retained`, ring `general_availability`/`promoted`
