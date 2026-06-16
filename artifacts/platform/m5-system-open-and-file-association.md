# M5 system-open and file-association intake

Generated from the seeded report in
[`crate::m5_system_entry`](../../crates/aureline-shell/src/m5_system_entry/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- report-md > \
  artifacts/platform/m5-system-open-and-file-association.md
```

- Report id: `shell:m5_system_entry:report:v1`
- Source schema ref: `schemas/platform/m5-system-entry.schema.json`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered intakes: `10`
- Marketed intakes: `10`
- Project-entry parity intakes: `10`
- Blocking findings: `0`
- Narrowable marketed intakes: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `native_desktop_matrix_ref` | `artifacts/platform/m5-native-desktop-matrix.md` |
| `install_topology_ref` | `artifacts/install/m5/m5-install-and-portability-governance.md` |
| `project_entry_contract_ref` | `docs/ux/project_entry_contract.md` |
| `entry_interstitial_ref` | `shell:entry_interstitials:v1` |
| `handoff_review_ref` | `docs/public/m3/handoff_and_repro_boundary.md` |
| `auth_recovery_ref` | `artifacts/auth/m5_auth_and_recovery.md` |

## Per-intake-kind coverage

| Intake kind | Registered intakes |
| ----------- | -----------------: |
| File | 2 |
| Folder | 2 |
| Workspace | 2 |
| Review / deep link | 2 |
| Patch / state bundle | 1 |
| Provider return | 1 |

## Per-scope coverage

| Scope class | Intakes | Gated behind interstitial |
| ----------- | ------: | ------------------------: |
| Plain local read | 4 | 0 |
| Widens to workspace scope | 3 | 3 |
| Crosses boundary | 3 | 3 |
| Widens to provider mutation | 0 | 0 |
| Requires trust decision | 0 | 0 |

## Resulting-mode index

| Intake | Kind | Verb | Resulting mode | Scope | Availability |
| ------ | ---- | ---- | -------------- | ----- | ------------ |
| `intake:case.mixed_root` | Workspace | `open` | `workspace_with_roots` | `widens_to_workspace_scope` | `mixed_root` |
| `intake:case.moved_target` | Folder | `open` | `folder` | `plain_local_read` | `moved_target` |
| `intake:case.policy_blocked` | Review / deep link | `open` | `inspect_only` | `crosses_boundary` | `blocked_by_policy` |
| `intake:case.wrong_association` | File | `open` | `single_file` | `plain_local_read` | `wrong_association` |
| `intake:file.system_open` | File | `open` | `single_file` | `plain_local_read` | `exact_available` |
| `intake:folder.system_open` | Folder | `open` | `folder` | `plain_local_read` | `exact_available` |
| `intake:patch_bundle.file_association` | Patch / state bundle | `import` | `extract_then_review` | `widens_to_workspace_scope` | `exact_available` |
| `intake:provider_return.auth_callback` | Provider return | `resume` | `resume_live_session` | `crosses_boundary` | `exact_available` |
| `intake:review_link.protocol_handler` | Review / deep link | `open` | `inspect_only` | `crosses_boundary` | `exact_available` |
| `intake:workspace.file_association` | Workspace | `open` | `workspace_with_roots` | `widens_to_workspace_scope` | `exact_available` |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-intake rows

### `intake:case.mixed_root` (workspace via system_open)

- Descriptor revision: `intake:case.mixed_root:rev:2026.06.01-01`
- Literal target: `literal:case.mixed_root:captured` (`file_uri`)
- Canonical target: `canonical:case.mixed_root:workspace_manifest`
- Detected target kind: `workspace_manifest`
- Intended verb / resulting mode: `open` / `workspace_with_roots`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:case.mixed_root`
- Channel/build owner: `channel-owner:intake:case.mixed_root` (`channel_scoped_owner`)
- Trust checkpoint: `trust:intake:case.mixed_root:profile_tenant_policy`
- Scope: `widens_to_workspace_scope` (interstitial required: `true`)
- Availability: `mixed_root`
- Recovery actions: `select_intended_root`, `choose_different_target`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A workspace whose roots span mismatched or unavailable roots does not silently merge them; the intake asks the user to select the intended root behind an interstitial.
- Degraded-state vocabulary:
  - This workspace spans roots that no longer match
  - Select the root you meant to open
  - Open just the manifest instead

Findings: none.

### `intake:case.moved_target` (folder via recent_item)

- Descriptor revision: `intake:case.moved_target:rev:2026.06.01-01`
- Literal target: `literal:case.moved_target:captured` (`posix_path`)
- Canonical target: `canonical:case.moved_target:folder_root`
- Detected target kind: `local_folder`
- Intended verb / resulting mode: `open` / `folder`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:case.moved_target`
- Channel/build owner: `channel-owner:intake:case.moved_target` (`channel_scoped_owner`)
- Trust checkpoint: `trust:intake:case.moved_target:profile_tenant_policy`
- Scope: `plain_local_read` (interstitial required: `false`)
- Availability: `moved_target`
- Recovery actions: `choose_different_target`, `reopen_in_active_profile`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A recent-item reopen whose folder moved shows a truthful placeholder with a target picker rather than opening an empty or stale shell.
- Degraded-state vocabulary:
  - This item moved or was removed
  - Reopen in the original workspace
  - Choose a different folder

Findings: none.

### `intake:case.policy_blocked` (review_link via protocol_handler)

- Descriptor revision: `intake:case.policy_blocked:rev:2026.06.01-01`
- Literal target: `literal:case.policy_blocked:captured` (`deep_link_uri`)
- Canonical target: `canonical:case.policy_blocked:review_item`
- Detected target kind: `review_or_work_item_deep_link`
- Intended verb / resulting mode: `open` / `inspect_only`
- Parity: `routed_to_review_surface` (reuses in-product path: `true`)
- Canonical command: `cmd:review.open_handoff`
- Active profile owner: `profile-owner:intake:case.policy_blocked`
- Channel/build owner: `channel-owner:intake:case.policy_blocked` (`managed_fleet_owned`)
- Trust checkpoint: `trust:intake:case.policy_blocked:profile_tenant_policy`
- Scope: `crosses_boundary` (interstitial required: `true`)
- Availability: `blocked_by_policy`
- Recovery actions: `show_policy_block_detail`, `return_to_review`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A review deep link blocked by managed policy degrades truthfully to a policy-block detail with a return path, never a silent dead-end or an unscoped retry.
- Degraded-state vocabulary:
  - This link was blocked by policy
  - See why this was blocked
  - Return to the review surface

Findings: none.

### `intake:case.wrong_association` (file via file_association)

- Descriptor revision: `intake:case.wrong_association:rev:2026.06.01-01`
- Literal target: `literal:case.wrong_association:captured` (`windows_drive_path`)
- Canonical target: `canonical:case.wrong_association:single_file`
- Detected target kind: `local_file`
- Intended verb / resulting mode: `open` / `single_file`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:case.wrong_association`
- Channel/build owner: `channel-owner:intake:case.wrong_association` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:intake:case.wrong_association:profile_tenant_policy`
- Scope: `plain_local_read` (interstitial required: `false`)
- Availability: `wrong_association`
- Recovery actions: `open_with_correct_handler`, `choose_different_target`
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A file delivered through an association owned by another channel is not silently opened; the intake offers the correct handler and a target picker instead.
- Degraded-state vocabulary:
  - This file type is registered to another channel
  - Open with the channel that owns this type
  - Choose a different file

Findings: none.

### `intake:file.system_open` (file via system_open)

- Descriptor revision: `intake:file.system_open:rev:2026.06.01-01`
- Literal target: `literal:file.system_open:captured` (`posix_path`)
- Canonical target: `canonical:file.system_open:single_file`
- Detected target kind: `local_file`
- Intended verb / resulting mode: `open` / `single_file`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:file.system_open`
- Channel/build owner: `channel-owner:intake:file.system_open` (`channel_scoped_owner`)
- Trust checkpoint: `trust:intake:file.system_open:profile_tenant_policy`
- Scope: `plain_local_read` (interstitial required: `false`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A system open of a single file resolves to a plain local read in the active profile and never silently widens into a workspace open.
- Degraded-state vocabulary:
  - Open this file
  - This file is no longer available
  - Choose a different file

Findings: none.

### `intake:folder.system_open` (folder via system_open)

- Descriptor revision: `intake:folder.system_open:rev:2026.06.01-01`
- Literal target: `literal:folder.system_open:captured` (`posix_path`)
- Canonical target: `canonical:folder.system_open:folder_root`
- Detected target kind: `local_folder`
- Intended verb / resulting mode: `open` / `folder`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:folder.system_open`
- Channel/build owner: `channel-owner:intake:folder.system_open` (`channel_scoped_owner`)
- Trust checkpoint: `trust:intake:folder.system_open:profile_tenant_policy`
- Scope: `plain_local_read` (interstitial required: `false`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A system open of a folder lands on the folder root in the active profile and offers, but never auto-commits, the wider workspace-candidate mode.
- Degraded-state vocabulary:
  - Open this folder
  - This folder moved or was removed
  - Choose a different folder

Findings: none.

### `intake:patch_bundle.file_association` (patch_bundle via file_association)

- Descriptor revision: `intake:patch_bundle.file_association:rev:2026.06.01-01`
- Literal target: `literal:patch_bundle.file_association:captured` (`file_uri`)
- Canonical target: `canonical:patch_bundle.file_association:portable_state_package`
- Detected target kind: `portable_state_package`
- Intended verb / resulting mode: `import` / `extract_then_review`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.import.bundle`
- Active profile owner: `profile-owner:intake:patch_bundle.file_association`
- Channel/build owner: `channel-owner:intake:patch_bundle.file_association` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:intake:patch_bundle.file_association:profile_tenant_policy`
- Scope: `widens_to_workspace_scope` (interstitial required: `true`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A patch or state bundle resolves to an extract-then-review import that previews the change before any write, gated behind an explicit interstitial.
- Degraded-state vocabulary:
  - Extract and review this bundle before applying
  - This bundle is registered to another channel
  - Open the bundle file without importing

Findings: none.

### `intake:provider_return.auth_callback` (provider_return via auth_callback)

- Descriptor revision: `intake:provider_return.auth_callback:rev:2026.06.01-01`
- Literal target: `literal:provider_return.auth_callback:captured` (`provider_callback`)
- Canonical target: `canonical:provider_return.auth_callback:pending_sign_in`
- Detected target kind: `managed_cloud_workspace`
- Intended verb / resulting mode: `resume` / `resume_live_session`
- Parity: `routed_to_auth_recovery` (reuses in-product path: `true`)
- Canonical command: `cmd:auth.resume_pending_sign_in`
- Active profile owner: `profile-owner:intake:provider_return.auth_callback`
- Channel/build owner: `channel-owner:intake:provider_return.auth_callback` (`channel_scoped_owner`)
- Trust checkpoint: `trust:intake:provider_return.auth_callback:profile_tenant_policy`
- Scope: `crosses_boundary` (interstitial required: `true`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A browser auth callback returns to the exact pending sign-in in the originating profile behind an interstitial and never silently mutates provider state.
- Degraded-state vocabulary:
  - Return to Aureline to finish signing in
  - This sign-in link has expired
  - Sign-in was blocked by policy

Findings: none.

### `intake:review_link.protocol_handler` (review_link via protocol_handler)

- Descriptor revision: `intake:review_link.protocol_handler:rev:2026.06.01-01`
- Literal target: `literal:review_link.protocol_handler:captured` (`deep_link_uri`)
- Canonical target: `canonical:review_link.protocol_handler:review_item`
- Detected target kind: `review_or_work_item_deep_link`
- Intended verb / resulting mode: `open` / `inspect_only`
- Parity: `routed_to_review_surface` (reuses in-product path: `true`)
- Canonical command: `cmd:review.open_handoff`
- Active profile owner: `profile-owner:intake:review_link.protocol_handler`
- Channel/build owner: `channel-owner:intake:review_link.protocol_handler` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:intake:review_link.protocol_handler:profile_tenant_policy`
- Scope: `crosses_boundary` (interstitial required: `true`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: A review or work-item deep link opens the review surface inspect-only behind an interstitial and is never coerced into a mutating provider action.
- Degraded-state vocabulary:
  - Review this item without making changes
  - This review link points to an item you cannot access
  - This review link has expired

Findings: none.

### `intake:workspace.file_association` (workspace via file_association)

- Descriptor revision: `intake:workspace.file_association:rev:2026.06.01-01`
- Literal target: `literal:workspace.file_association:captured` (`file_uri`)
- Canonical target: `canonical:workspace.file_association:workspace_manifest`
- Detected target kind: `workspace_manifest`
- Intended verb / resulting mode: `open` / `workspace_with_roots`
- Parity: `entry_flow_resolved` (reuses in-product path: `true`)
- Canonical command: `cmd:workspace.open.target`
- Active profile owner: `profile-owner:intake:workspace.file_association`
- Channel/build owner: `channel-owner:intake:workspace.file_association` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:intake:workspace.file_association:profile_tenant_policy`
- Scope: `widens_to_workspace_scope` (interstitial required: `true`)
- Availability: `exact_available`
- Recovery actions: _(none required)_
- Claimed platforms: `macos`, `windows`, `linux`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:system_entry:narrow_on_stale_evidence`
- Marketed: `yes`
- Continuity note: Opening a workspace manifest widens to multi-root workspace scope, so it always shows an explicit interstitial before it commits rather than auto-opening every root.
- Degraded-state vocabulary:
  - Open this workspace with all its roots
  - This workspace manifest is registered to another channel
  - Open just the manifest file instead

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_system_entry -- validate
cargo test -p aureline-shell --test m5_system_entry_fixtures
python3 tools/ci/m5/system_entry_check.py
```
