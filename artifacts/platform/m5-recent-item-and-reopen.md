# M5 recent-item, dock/taskbar, and jump-list reopen fidelity

Generated from the seeded report in
[`crate::m5_recent_items_and_reopen`](../../crates/aureline-shell/src/m5_recent_items_and_reopen/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- report-md > \
  artifacts/platform/m5-recent-item-and-reopen.md
```

- Report id: `shell:m5_recent_items_and_reopen:report:v1`
- Source schema ref: `schemas/platform/m5-reopen-target.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered reopen targets: `9`
- Marketed reopen targets: `9`
- Exact-object reopen targets: `4`
- Blocking findings: `0`
- Narrowable marketed targets: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `native_desktop_matrix_ref` | `artifacts/platform/m5-native-desktop-matrix.md` |
| `system_entry_intake_ref` | `artifacts/platform/m5-system-open-and-file-association.md` |
| `install_topology_ref` | `artifacts/install/m5/m5-install-and-portability-governance.md` |
| `restore_provenance_ref` | `shell:restore:provenance_and_placeholders:v1` |
| `start_center_ref` | `shell:m5_start_center_and_switcher:v1` |
| `entry_interstitial_ref` | `shell:entry_interstitials:v1` |

## Per-surface coverage

| Reopen surface | Registered targets |
| -------------- | -----------------: |
| Recent item | 3 |
| Dock | 2 |
| Taskbar | 2 |
| Jump list | 2 |

## Per-availability coverage

| Availability | Targets | With recovery |
| ------------ | ------: | ------------: |
| Exact object | 4 | 0 |
| Moved target | 1 | 1 |
| Missing root | 1 | 1 |
| Changed channel | 1 | 1 |
| Stale provider-linked | 1 | 1 |
| Wrong target detected | 1 | 1 |

## Per-platform coverage

| Platform | Claimed targets |
| -------- | --------------: |
| `macos` | 5 |
| `windows` | 7 |
| `linux` | 3 |

## Reopen-identity index

| Reopen target | Surface | Kind | Availability | Restore | Action |
| ------------- | ------- | ---- | ------------ | ------- | ------ |
| `reopen:case.changed_channel` | Dock | `local_repo_root` | `changed_channel` | `layout_only` | `reopen_object` |
| `reopen:case.missing_root` | Jump list | `workspace_manifest` | `missing_root` | `layout_only` | `reopen_object` |
| `reopen:case.moved_target` | Recent item | `local_file` | `moved_target` | `layout_only` | `reopen_object` |
| `reopen:case.stale_provider_linked` | Taskbar | `managed_cloud_workspace` | `stale_provider_linked` | `evidence_only` | `privileged_or_mutating` |
| `reopen:case.wrong_target` | Recent item | `local_file` | `wrong_target_detected` | `none` | `reopen_object` |
| `reopen:dock.exact` | Dock | `local_repo_root` | `exact_object` | `exact` | `reopen_object` |
| `reopen:jump_list.exact` | Jump list | `workspace_manifest` | `exact_object` | `compatible` | `reopen_object` |
| `reopen:recent_item.exact` | Recent item | `local_file` | `exact_object` | `exact` | `reopen_object` |
| `reopen:taskbar.exact` | Taskbar | `local_folder` | `exact_object` | `exact` | `reveal_object` |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-target rows

### `reopen:case.changed_channel` (local_repo_root on dock)

- Descriptor revision: `reopen:case.changed_channel:rev:2026.06.01-01`
- Literal target: `literal:case.changed_channel:captured`
- Canonical object: `canonical:case.changed_channel:repo_root`
- Originating channel/build owner: `channel-owner:reopen:case.changed_channel` (`shared_default_arbitrated`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:case.changed_channel`
- Trust checkpoint: `trust:reopen:case.changed_channel:profile_tenant_policy`
- Target freshness: `stale` (captured `2026-06-16T00:00:00Z`)
- Availability: `changed_channel`
- Restore availability: `layout_only` (trust `pending_evaluation`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: `locate_missing_target`, `remove_from_recents`
- Placeholder label: `placeholder:case.changed_channel:changed_channel`
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `macos`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A dock reopen whose registration is now owned by a side-by-side or portable channel shows a changed-channel placeholder naming the owning channel rather than silently reopening under the wrong build.
- Degraded-state vocabulary:
  - Another channel now owns this recent item
  - Reopen in the channel that owns it
  - Remove from list

Findings: none.

### `reopen:case.missing_root` (workspace_manifest on jump_list)

- Descriptor revision: `reopen:case.missing_root:rev:2026.06.01-01`
- Literal target: `literal:case.missing_root:captured`
- Canonical object: `canonical:case.missing_root:workspace_manifest`
- Originating channel/build owner: `channel-owner:reopen:case.missing_root` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:case.missing_root`
- Trust checkpoint: `trust:reopen:case.missing_root:profile_tenant_policy`
- Target freshness: `stale` (captured `2026-06-16T00:00:00Z`)
- Availability: `missing_root`
- Restore availability: `layout_only` (trust `trusted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: `locate_missing_target`, `open_without_restore`, `remove_from_recents`
- Placeholder label: `placeholder:case.missing_root:missing_root`
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `windows`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A pinned jump-list reopen whose root volume or share is unmounted shows a missing-root placeholder with locate and open-without-restore actions rather than opening an empty shell.
- Degraded-state vocabulary:
  - This workspace root is missing or unmounted
  - Locate the workspace
  - Open anyway without restore

Findings: none.

### `reopen:case.moved_target` (local_file on recent_item)

- Descriptor revision: `reopen:case.moved_target:rev:2026.06.01-01`
- Literal target: `literal:case.moved_target:captured`
- Canonical object: `canonical:case.moved_target:single_file`
- Originating channel/build owner: `channel-owner:reopen:case.moved_target` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:case.moved_target`
- Trust checkpoint: `trust:reopen:case.moved_target:profile_tenant_policy`
- Target freshness: `stale` (captured `2026-06-16T00:00:00Z`)
- Availability: `moved_target`
- Restore availability: `layout_only` (trust `trusted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: `locate_missing_target`, `remove_from_recents`
- Placeholder label: `placeholder:case.moved_target:moved_target`
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A recent-item reopen whose file moved shows a truthful moved-target placeholder with a locate action and preserves the cached identity until a new location is selected.
- Degraded-state vocabulary:
  - This item moved or was removed
  - Locate the file
  - Remove from list

Findings: none.

### `reopen:case.stale_provider_linked` (managed_cloud_workspace on taskbar)

- Descriptor revision: `reopen:case.stale_provider_linked:rev:2026.06.01-01`
- Literal target: `literal:case.stale_provider_linked:captured`
- Canonical object: `canonical:case.stale_provider_linked:cloud_workspace`
- Originating channel/build owner: `channel-owner:reopen:case.stale_provider_linked` (`managed_fleet_owned`)
- Side-by-side / portable plausible: `false`
- Active profile owner: `profile-owner:reopen:case.stale_provider_linked`
- Trust checkpoint: `trust:reopen:case.stale_provider_linked:profile_tenant_policy`
- Target freshness: `stale` (captured `2026-06-16T00:00:00Z`)
- Availability: `stale_provider_linked`
- Restore availability: `evidence_only` (trust `pending_evaluation`, portability `provider_linked`)
- Action: `privileged_or_mutating` (summary-only: `false`)
- Reviewed return surface: `artifacts/auth/m5_auth_and_recovery.md`
- Canonical command: `cmd:auth.resume_pending_sign_in`
- Recovery actions: `reauth`, `reconnect`, `open_read_only_cached_view`
- Placeholder label: `placeholder:case.stale_provider_linked:stale_provider_linked`
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `windows`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A taskbar reopen of a provider-linked cloud workspace whose authority went stale is privileged, so it routes through the reviewed auth-recovery surface to reauthorize instead of mutating provider state directly from the shortcut.
- Degraded-state vocabulary:
  - This cloud workspace needs reauthorization
  - Reauthorize to reopen
  - Open a read-only cached view

Findings: none.

### `reopen:case.wrong_target` (local_file on recent_item)

- Descriptor revision: `reopen:case.wrong_target:rev:2026.06.01-01`
- Literal target: `literal:case.wrong_target:captured`
- Canonical object: `canonical:case.wrong_target:single_file`
- Conflicting object: `canonical:case.wrong_target:conflicting_file`
- Originating channel/build owner: `channel-owner:reopen:case.wrong_target` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:case.wrong_target`
- Trust checkpoint: `trust:reopen:case.wrong_target:profile_tenant_policy`
- Target freshness: `stale` (captured `2026-06-16T00:00:00Z`)
- Availability: `wrong_target_detected`
- Restore availability: `none` (trust `restricted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: `locate_missing_target`, `remove_from_recents`
- Placeholder label: `placeholder:case.wrong_target:wrong_target_detected`
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A recent-item reopen whose captured literal now resolves to a different object than the one registered is detected and shown as a wrong-target placeholder naming the conflicting object, never silently reopened.
- Degraded-state vocabulary:
  - This item now points at a different object
  - Locate the original object
  - Remove from list

Findings: none.

### `reopen:dock.exact` (local_repo_root on dock)

- Descriptor revision: `reopen:dock.exact:rev:2026.06.01-01`
- Literal target: `literal:dock.exact:captured`
- Canonical object: `canonical:dock.exact:repo_root`
- Originating channel/build owner: `channel-owner:reopen:dock.exact` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:dock.exact`
- Trust checkpoint: `trust:reopen:dock.exact:profile_tenant_policy`
- Target freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Availability: `exact_object`
- Restore availability: `exact` (trust `trusted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: _(none required)_
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `macos`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A dock recent-documents reopen of a repository lands on the exact root in the active profile and names the channel that owns the dock registration.
- Degraded-state vocabulary:
  - Reopen this repository
  - This repository moved or was removed
  - Locate the repository

Findings: none.

### `reopen:jump_list.exact` (workspace_manifest on jump_list)

- Descriptor revision: `reopen:jump_list.exact:rev:2026.06.01-01`
- Literal target: `literal:jump_list.exact:captured`
- Canonical object: `canonical:jump_list.exact:workspace_manifest`
- Originating channel/build owner: `channel-owner:reopen:jump_list.exact` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:jump_list.exact`
- Trust checkpoint: `trust:reopen:jump_list.exact:profile_tenant_policy`
- Target freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Availability: `exact_object`
- Restore availability: `compatible` (trust `trusted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: _(none required)_
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `windows`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A pinned jump-list reopen of a workspace lands on the exact manifest with a compatible restore, never silently widening into an unrelated workspace.
- Degraded-state vocabulary:
  - Reopen this workspace
  - This workspace moved or was removed
  - Locate the workspace

Findings: none.

### `reopen:recent_item.exact` (local_file on recent_item)

- Descriptor revision: `reopen:recent_item.exact:rev:2026.06.01-01`
- Literal target: `literal:recent_item.exact:captured`
- Canonical object: `canonical:recent_item.exact:single_file`
- Originating channel/build owner: `channel-owner:reopen:recent_item.exact` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:recent_item.exact`
- Trust checkpoint: `trust:reopen:recent_item.exact:profile_tenant_policy`
- Target freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Availability: `exact_object`
- Restore availability: `exact` (trust `trusted`, portability `local_only`)
- Action: `reopen_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.open.target`
- Recovery actions: _(none required)_
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A recent-item reopen of a still-present file lands on the exact object in the active profile with the originating channel owner shown next to it.
- Degraded-state vocabulary:
  - Reopen this file
  - This file is no longer available
  - Locate the file

Findings: none.

### `reopen:taskbar.exact` (local_folder on taskbar)

- Descriptor revision: `reopen:taskbar.exact:rev:2026.06.01-01`
- Literal target: `literal:taskbar.exact:captured`
- Canonical object: `canonical:taskbar.exact:folder_root`
- Originating channel/build owner: `channel-owner:reopen:taskbar.exact` (`channel_scoped_owner`)
- Side-by-side / portable plausible: `true`
- Active profile owner: `profile-owner:reopen:taskbar.exact`
- Trust checkpoint: `trust:reopen:taskbar.exact:profile_tenant_policy`
- Target freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Availability: `exact_object`
- Restore availability: `exact` (trust `trusted`, portability `local_only`)
- Action: `reveal_object` (summary-only: `true`)
- Canonical command: `cmd:workspace.reveal.target`
- Recovery actions: _(none required)_
- Restore provenance: `shell:restore:provenance_and_placeholders:v1`
- Claimed platforms: `windows`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:reopen_target:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A taskbar recent reveal of a folder opens the OS file manager at the exact location and never silently opens or mutates the workspace.
- Degraded-state vocabulary:
  - Reveal this folder
  - This folder moved or was removed
  - Locate the folder

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- validate
cargo test -p aureline-shell --test m5_reopen_target_fixtures
python3 tools/ci/m5/reopen_target_check.py
```
