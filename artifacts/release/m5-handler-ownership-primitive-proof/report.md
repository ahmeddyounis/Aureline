# M5 Handler-Ownership Primitive: Ownership / Precedence Disclosure Card, Channel-Association Review Rows, and Recovery Alignment

- Packet: `m5-handler-ownership-primitive:stable:0001`
- Label: `M5 Handler-Ownership Primitive: Ownership / Precedence Disclosure Card, Channel-Association Review Rows, and Recovery Alignment`
- Handler surfaces: 6 / 6
- Channel classes: file_association, protocol_handler, recent_item_reopen, notification_action, deep_link, system_open
- Precedence states: sole_owner, primary_among_installs, shared_contested, superseded, not_registered
- Association actions: keep, reassign, cancel, preview_change

## Handler surfaces

- **About desktop-integration section**
  - Owner: Desktop-integration guild
  - Scope: About desktop-integration section naming the primary owner and precedence for the file and protocol handlers
  - Worked cases: 1
    - `ownership:about-integration:0001` → owner `primary_stable_install` (primary_among_installs), 2 channels, 0 recovery paths
- **Diagnostics handler-ownership pane**
  - Owner: Diagnostics guild
  - Scope: Diagnostics handler-ownership pane naming both installs contesting a file association and leaving the choice to the user
  - Worked cases: 1
    - `ownership:diagnostics-handlers:0002` → owner `side_by_side_beta_install` (shared_contested), 2 channels, 1 recovery paths
- **Install / side-by-side review**
  - Owner: Install / side-by-side guild
  - Scope: Install review previewing a portable install reassigning the protocol handler with a reversible change
  - Worked cases: 1
    - `ownership:install-review:0003` → owner `portable_install` (primary_among_installs), 2 channels, 0 recovery paths
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Support replay reconstructing sole fleet handler ownership with every recovery path carrying the rollback identity
  - Worked cases: 1
    - `ownership:support-replay:0004` → owner `managed_fleet_install` (sole_owner), 4 channels, 4 recovery paths
- **Notification / activity center**
  - Owner: Notification / activity guild
  - Scope: Notification center framing a sole-owner desktop install whose notification and deep-link recovery routes activate this build
  - Worked cases: 1
    - `ownership:notification-center:0005` → owner `primary_stable_install` (sole_owner), 2 channels, 2 recovery paths
- **Docs handler reference**
  - Owner: Docs / help guild
  - Scope: Docs handler reference framing a single-install desktop that solely owns the file association
  - Worked cases: 1
    - `ownership:docs-reference:0006` → owner `primary_stable_install` (sole_owner), 1 channels, 0 recovery paths
