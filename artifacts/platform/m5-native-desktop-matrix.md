# M5 native-desktop integration and reopen matrix

Generated from the seeded matrix in
[`crate::m5_native_desktop`](../../crates/aureline-shell/src/m5_native_desktop/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report-md > \
  artifacts/platform/m5-native-desktop-matrix.md
```

- Report id: `shell:m5_native_desktop:matrix:v1`
- Source schema ref: `schemas/platform/m5-native-desktop-matrix.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered surfaces: `10`
- Marketed surfaces: `10`
- Controls checked: `70`
- Blocking findings: `0`
- Narrowable marketed surfaces: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `install_topology_ref` | `artifacts/install/m5/m5-install-and-portability-governance.md` |
| `embedded_boundary_ref` | `artifacts/ux/m5/embedded-boundary-audits/m5_embedded_boundaries_audit.md` |
| `activity_center_ref` | `artifacts/ux/m5/durable-attention-packets/m5_activity_objects_audit.md` |
| `auth_recovery_ref` | `artifacts/auth/m5_auth_and_recovery.md` |
| `channel_ownership_ref` | `artifacts/release/channel_ownership_audit.yaml` |
| `protocol_handler_ownership_ref` | `artifacts/platform/protocol_handler_ownership_matrix.yaml` |
| `file_association_ownership_ref` | `artifacts/platform/file_association_ownership_matrix.yaml` |

## Per-control coverage

| Control | Satisfied | Not applicable | Narrowed | Failed |
| ------- | --------: | -------------: | -------: | -----: |
| Trust / policy evaluation | 10 | 0 | 0 | 0 |
| Channel / build ownership | 10 | 0 | 0 | 0 |
| Wrong-target recovery | 10 | 0 | 0 | 0 |
| Unavailable-path recovery | 10 | 0 | 0 | 0 |
| Policy-block recovery | 10 | 0 | 0 | 0 |
| Signal durability | 2 | 8 | 0 | 0 |
| Notification privacy | 2 | 8 | 0 | 0 |

## Per-surface-kind coverage

| Surface kind | Registered surfaces |
| ------------ | ------------------: |
| System open / save / reveal | 1 |
| File association | 1 |
| Protocol handler | 1 |
| Auth callback | 1 |
| Recent item | 1 |
| Dock / taskbar / jump-list | 1 |
| OS notification | 1 |
| Badge / progress | 1 |
| Removable / network path | 1 |
| Credential-store lock state | 1 |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Reopen anchor index

| Surface kind | Entry | Reopen anchor |
| ------------ | ----- | ------------- |
| Auth callback | `entry:auth_callback.browser_return` | `reopen:anchor:auth_callback:browser_return` |
| Badge / progress | `entry:badge_progress.dock_badge` | `reopen:anchor:badge_progress:dock_badge` |
| Dock / taskbar / jump-list | `entry:dock_taskbar_jumplist.reopen` | `reopen:anchor:dock_taskbar_jumplist:reopen` |
| File association | `entry:file_association.notebook_doc` | `reopen:anchor:file_association:notebook_doc` |
| OS notification | `entry:os_notification.run_complete` | `reopen:anchor:os_notification:run_complete` |
| Protocol handler | `entry:protocol_handler.aureline_scheme` | `reopen:anchor:protocol_handler:aureline_scheme` |
| Recent item | `entry:recent_item.workspace_list` | `reopen:anchor:recent_item:workspace_list` |
| Removable / network path | `entry:removable_path.network_share` | `reopen:anchor:removable_path:network_share` |
| Credential-store lock state | `entry:store_lock_state.credential_store` | `reopen:anchor:store_lock_state:credential_store` |
| System open / save / reveal | `entry:system_open.workspace_target` | `reopen:anchor:system_open:workspace_target` |

## Per-surface rows

### `entry:auth_callback.browser_return` (auth_callback)

- Descriptor revision: `entry-rev:auth_callback.browser_return:2026.06.01-01`
- Channel/build owner: `channel-owner:auth_callback.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:auth_callback.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:auth_callback:browser_return`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A browser auth callback returns to the exact pending sign-in in the originating profile, and an expired or policy-blocked callback recovers truthfully instead of dead-ending.
- Degraded-state vocabulary:
  - Return to Aureline to finish signing in
  - This sign-in link has expired
  - Sign-in was blocked by policy

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:auth_callback.browser_return:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:auth_callback.browser_return:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:auth_callback.browser_return:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:badge_progress.dock_badge` (badge_progress)

- Descriptor revision: `entry-rev:badge_progress.dock_badge:2026.06.01-01`
- Channel/build owner: `channel-owner:badge_progress.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:badge_progress.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:badge_progress:dock_badge`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A dock badge and progress indicator derive from durable counts for the active profile only, hide private detail on shared surfaces, and pause under policy rather than polling.
- Degraded-state vocabulary:
  - Counts reflect this profile only
  - Hidden on the lock screen
  - Paused by policy

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:badge_progress.dock_badge:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:badge_progress.dock_badge:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:badge_progress.dock_badge:policy_block_recovery` | `-` | - |
| Signal durability | `satisfied` | `-` | `-` | `reopen:anchor:badge_progress:dock_badge` | - |
| Notification privacy | `satisfied` | `-` | `-` | `-` | - |

Findings: none.

### `entry:dock_taskbar_jumplist.reopen` (dock_taskbar_jumplist)

- Descriptor revision: `entry-rev:dock_taskbar_jumplist.reopen:2026.06.01-01`
- Channel/build owner: `channel-owner:dock_taskbar_jumplist.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:dock_taskbar_jumplist.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:dock_taskbar_jumplist:reopen`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A dock, taskbar, or jump-list reopen routes through policy and lands on the durable target, and a pinned entry whose target is gone or blocked recovers truthfully.
- Degraded-state vocabulary:
  - This pinned item is no longer available
  - Reopen in this profile
  - Removed by policy

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:dock_taskbar_jumplist.reopen:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:dock_taskbar_jumplist.reopen:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:dock_taskbar_jumplist.reopen:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:file_association.notebook_doc` (file_association)

- Descriptor revision: `entry-rev:file_association.notebook_doc:2026.06.01-01`
- Channel/build owner: `channel-owner:file_association.shared_default` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:file_association.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:file_association:notebook_doc`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: An opened document carries the file path through profile and policy evaluation, and a side-by-side channel cannot claim the shared default without explicit arbitration.
- Degraded-state vocabulary:
  - Open with Aureline
  - This file type is registered to another channel
  - Reopen the original file

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:file_association.notebook_doc:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:file_association.notebook_doc:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:file_association.notebook_doc:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:os_notification.run_complete` (os_notification)

- Descriptor revision: `entry-rev:os_notification.run_complete:2026.06.01-01`
- Channel/build owner: `channel-owner:os_notification.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:os_notification.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:os_notification:run_complete`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A completion notification derives from a durable activity object, reopens the exact run in the signed-in profile, stays privacy-safe on the lock screen, and respects quiet hours and policy.
- Degraded-state vocabulary:
  - Reopen the item this alert is about
  - This item is no longer available
  - Muted by quiet hours

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:os_notification.run_complete:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:os_notification.run_complete:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:os_notification.run_complete:policy_block_recovery` | `-` | - |
| Signal durability | `satisfied` | `-` | `-` | `reopen:anchor:os_notification:run_complete` | - |
| Notification privacy | `satisfied` | `-` | `-` | `-` | - |

Findings: none.

### `entry:protocol_handler.aureline_scheme` (protocol_handler)

- Descriptor revision: `entry-rev:protocol_handler.aureline_scheme:2026.06.01-01`
- Channel/build owner: `channel-owner:protocol_handler.shared_default` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:protocol_handler.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:protocol_handler:aureline_scheme`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A deep link resolves the exact target in the signed-in profile or fails closed with a recovery action; the scheme owner is inspectable so no install can silently take it over.
- Degraded-state vocabulary:
  - Reopen this link in your signed-in profile
  - This link points to a target you cannot access
  - This link has expired

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:protocol_handler.aureline_scheme:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:protocol_handler.aureline_scheme:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:protocol_handler.aureline_scheme:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:recent_item.workspace_list` (recent_item)

- Descriptor revision: `entry-rev:recent_item.workspace_list:2026.06.01-01`
- Channel/build owner: `channel-owner:recent_item.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:recent_item.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:recent_item:workspace_list`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A recent-item reopen re-evaluates profile and policy and lands on the exact target, and a moved or removed item shows a truthful placeholder with a recovery action.
- Degraded-state vocabulary:
  - This item moved or was removed
  - Reopen in the original workspace
  - Sign in to reopen this item

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:recent_item.workspace_list:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:recent_item.workspace_list:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:recent_item.workspace_list:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:removable_path.network_share` (removable_path)

- Descriptor revision: `entry-rev:removable_path.network_share:2026.06.01-01`
- Channel/build owner: `channel-owner:removable_path.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:removable_path.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:removable_path:network_share`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A removable volume or network share that disappears keeps the user on the last saved copy with a truthful placeholder and a reconnect action, never a silent data loss.
- Degraded-state vocabulary:
  - Reconnect the drive to continue
  - This volume is no longer mounted
  - Working from the last saved copy

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:removable_path.network_share:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:removable_path.network_share:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:removable_path.network_share:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:store_lock_state.credential_store` (store_lock_state)

- Descriptor revision: `entry-rev:store_lock_state.credential_store:2026.06.01-01`
- Channel/build owner: `channel-owner:store_lock_state.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:store_lock_state.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:store_lock_state:credential_store`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A locked credential store preserves the pending action with a truthful unlock prompt and a recovery action, and never silently signs the user out or proceeds without trust evaluation.
- Degraded-state vocabulary:
  - Unlock the credential store to continue
  - The credential store is locked
  - Signed out until you unlock

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:store_lock_state.credential_store:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:store_lock_state.credential_store:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:store_lock_state.credential_store:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

### `entry:system_open.workspace_target` (system_open)

- Descriptor revision: `entry-rev:system_open.workspace_target:2026.06.01-01`
- Channel/build owner: `channel-owner:system_open.active_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:system_open.profile_tenant_policy`
- Reopen anchor: `reopen:anchor:system_open:workspace_target`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A system open routes through the active profile and tenant policy before it reveals a target, and a missing target reopens to a truthful placeholder rather than a blank window.
- Degraded-state vocabulary:
  - Open in this profile
  - Choose a different target
  - This target is no longer available

| Control | Status | Failure | Recovery path | Durable object | Narrowing reason |
| ------- | ------ | ------- | ------------- | -------------- | ---------------- |
| Trust / policy evaluation | `satisfied` | `-` | `-` | `-` | - |
| Channel / build ownership | `satisfied` | `-` | `-` | `-` | - |
| Wrong-target recovery | `satisfied` | `-` | `recovery:entry:system_open.workspace_target:wrong_target_recovery` | `-` | - |
| Unavailable-path recovery | `satisfied` | `-` | `recovery:entry:system_open.workspace_target:unavailable_path_recovery` | `-` | - |
| Policy-block recovery | `satisfied` | `-` | `recovery:entry:system_open.workspace_target:policy_block_recovery` | `-` | - |
| Signal durability | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_signal_durability_is_not_applicable |
| Notification privacy | `not_applicable` | `-` | `-` | `-` | surface_emits_no_os_signal_so_notification_privacy_is_not_applicable |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- validate
cargo test -p aureline-shell --test m5_native_desktop_fixtures
python3 tools/ci/m5/native_desktop_check.py
```
