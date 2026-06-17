# M5 native-desktop qualification matrix

Generated from the seeded qualification family in
[`crate::m5_native_desktop_qualification`](../../../crates/aureline-shell/src/m5_native_desktop_qualification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- report-md > \
  artifacts/platform/m5-native-desktop-qualification/m5_native_desktop_qualification.md
```

- Report id: `shell:m5_native_desktop_qualification:v1`
- Source schema ref: `schemas/platform/m5-native-desktop-qualification.schema.json`
- Certifies matrix: `artifacts/platform/m5-native-desktop-matrix.md`
- Claimed platforms: `macos`, `windows`, `linux`
- Registered profiles: `6`
- Marketed profiles: `6`
- Dimensions checked: `42`
- Claim scope: published `5`, narrowed `1`, withheld `0`
- Blocking findings: `0`
- Narrowable marketed profiles: `0`
- Status: **clean**
- Generated at: `2026-06-16T00:00:00Z`

## Cross-links

| Upstream packet | Ref |
| --------------- | --- |
| `native_desktop_matrix_ref` | `artifacts/platform/m5-native-desktop-matrix.md` |
| `channel_ownership_ref` | `artifacts/release/channel_ownership_audit.yaml` |
| `protocol_handler_ownership_ref` | `artifacts/platform/protocol_handler_ownership_matrix.yaml` |
| `file_association_ownership_ref` | `artifacts/platform/file_association_ownership_matrix.yaml` |
| `reopen_corpus_ref` | `fixtures/platform/exact_target_reopen_cases` |
| `notification_privacy_ref` | `artifacts/platform/lock_screen_privacy_rows.yaml` |
| `external_root_recovery_ref` | `artifacts/platform/m5-store-lock-and-external-root-recovery.md` |
| `install_topology_ref` | `artifacts/install/m5/m5-install-and-portability-governance.md` |

## Per-dimension coverage

| Dimension | Drill | Qualified | Not applicable | Narrowed | Unqualified | Failed |
| --------- | ----- | --------: | -------------: | -------: | ----------: | -----: |
| Channel / build ownership | `channel_ownership_audit` | 6 | 0 | 0 | 0 | 0 |
| Protocol-handler ownership | `handler_conflict` | 5 | 1 | 0 | 0 | 0 |
| File-association ownership | `handler_conflict` | 5 | 1 | 0 | 0 | 0 |
| Reopen fidelity | `wrong_target_reopen` | 6 | 0 | 0 | 0 | 0 |
| Notification privacy | `lock_screen_privacy` | 6 | 0 | 0 | 0 | 0 |
| External-root recovery | `missing_root_recovery` | 6 | 0 | 0 | 0 | 0 |
| Store-lock recovery | `store_lock` | 6 | 0 | 0 | 0 | 0 |

## Claim scope

| Profile | Platform | Channel | Claim | Reason |
| ------- | -------- | ------- | ----- | ------ |
| `profile:linux.portable` | `linux` | `portable` | **Narrowed** | narrowed_dimensions:protocol_handler_ownership,file_association_ownership |
| `profile:linux.stable` | `linux` | `stable` | **Published** | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:macos.beta` | `macos` | `beta` | **Published** | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:macos.stable` | `macos` | `stable` | **Published** | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:windows.managed_fleet` | `windows` | `managed_fleet` | **Published** | all_marketed_dimensions_qualified_with_fresh_evidence |
| `profile:windows.stable` | `windows` | `stable` | **Published** | all_marketed_dimensions_qualified_with_fresh_evidence |

## Findings summary

| Class | Count |
| ----- | ----: |
| _(none)_ | 0 |

## Per-profile rows

### `profile:linux.portable` (linux / portable)

- Descriptor revision: `profile-rev:linux.portable:2026.06.01-01`
- Channel/build owner: `channel-owner:linux.portable.appimage` (`portable_non_registering`)
- Trust checkpoint: `trust:linux.portable.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Narrowed**
- Marketed: `yes`
- Continuity note: The Linux portable build registers no OS-level handler, so protocol and file-association ownership are explicitly not claimed; it still lands recent-item reopens exactly, keeps notification content privacy-safe, and recovers from a missing mount or a locked secret service.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:linux.portable:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `not_applicable` | `-` | `-` | portable_build_registers_no_protocol_handler_so_protocol_handler_ownership_is_not_applicable |
| File-association ownership | `handler_conflict` | `not_applicable` | `-` | `-` | portable_build_registers_no_file_association_so_file_association_ownership_is_not_applicable |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:linux.portable:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:linux.portable:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:linux.portable:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:linux.portable:store_lock` | - |

Findings: none.

### `profile:linux.stable` (linux / stable)

- Descriptor revision: `profile-rev:linux.stable:2026.06.01-01`
- Channel/build owner: `channel-owner:linux.stable.desktop_entry` (`shared_default_arbitrated`)
- Trust checkpoint: `trust:linux.stable.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Published**
- Marketed: `yes`
- Continuity note: The Linux stable channel arbitrates the shared default desktop entry explicitly, lands recent-item reopens on the exact target, keeps notification content privacy-safe, and recovers truthfully from a missing mount or a locked secret service.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:linux.stable:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:linux.stable:handler_conflict` | - |
| File-association ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:linux.stable:handler_conflict` | - |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:linux.stable:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:linux.stable:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:linux.stable:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:linux.stable:store_lock` | - |

Findings: none.

### `profile:macos.beta` (macos / beta)

- Descriptor revision: `profile-rev:macos.beta:2026.06.01-01`
- Channel/build owner: `channel-owner:macos.beta.signed_app_bundle` (`channel_scoped_owner`)
- Trust checkpoint: `trust:macos.beta.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Published**
- Marketed: `yes`
- Continuity note: The macOS beta channel owns a side-by-side registration that cannot collide with stable, reopens the exact target through policy, keeps lock-screen copy summary-first, and recovers from a missing volume or a locked keychain with its own current proof.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:macos.beta:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:macos.beta:handler_conflict` | - |
| File-association ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:macos.beta:handler_conflict` | - |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:macos.beta:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:macos.beta:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:macos.beta:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:macos.beta:store_lock` | - |

Findings: none.

### `profile:macos.stable` (macos / stable)

- Descriptor revision: `profile-rev:macos.stable:2026.06.01-01`
- Channel/build owner: `channel-owner:macos.stable.signed_app_bundle` (`channel_scoped_owner`)
- Trust checkpoint: `trust:macos.stable.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Published**
- Marketed: `yes`
- Continuity note: The macOS stable channel owns its own protocol and file-association registrations, lands every reopen on the exact target, keeps lock-screen notifications summary-first, and recovers truthfully from a missing volume or a locked keychain.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:macos.stable:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:macos.stable:handler_conflict` | - |
| File-association ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:macos.stable:handler_conflict` | - |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:macos.stable:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:macos.stable:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:macos.stable:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:macos.stable:store_lock` | - |

Findings: none.

### `profile:windows.managed_fleet` (windows / managed_fleet)

- Descriptor revision: `profile-rev:windows.managed_fleet:2026.06.01-01`
- Channel/build owner: `channel-owner:windows.managed_fleet.central_deployment` (`managed_fleet_owned`)
- Trust checkpoint: `trust:windows.managed_fleet.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Published**
- Marketed: `yes`
- Continuity note: The managed Windows fleet owns protocol and file-association registrations centrally with an inspectable owner, reopens fleet targets exactly under policy, suppresses private notification detail by admin policy, and recovers from a disconnected network home or a locked managed vault.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:windows.managed_fleet:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:windows.managed_fleet:handler_conflict` | - |
| File-association ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:windows.managed_fleet:handler_conflict` | - |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:windows.managed_fleet:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:windows.managed_fleet:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:windows.managed_fleet:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:windows.managed_fleet:store_lock` | - |

Findings: none.

### `profile:windows.stable` (windows / stable)

- Descriptor revision: `profile-rev:windows.stable:2026.06.01-01`
- Channel/build owner: `channel-owner:windows.stable.per_user_install` (`channel_scoped_owner`)
- Trust checkpoint: `trust:windows.stable.profile_tenant_policy`
- Evidence freshness: `fresh` (captured `2026-06-16T00:00:00Z`)
- Downgrade rule: `downgrade:native_desktop_qualification:narrow_on_stale_or_red`
- Claim state: **Published**
- Marketed: `yes`
- Continuity note: The Windows stable channel registers protocol and file associations per install with an inspectable owner, reopens jump-list and taskbar targets exactly, hides private notification detail on the lock screen, and recovers from a disconnected share or a locked credential vault.

| Dimension | Drill | Status | Failure | Drill ref | Narrowing reason |
| --------- | ----- | ------ | ------- | --------- | ---------------- |
| Channel / build ownership | `channel_ownership_audit` | `qualified` | `-` | `drill:profile:windows.stable:channel_ownership_audit` | - |
| Protocol-handler ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:windows.stable:handler_conflict` | - |
| File-association ownership | `handler_conflict` | `qualified` | `-` | `drill:profile:windows.stable:handler_conflict` | - |
| Reopen fidelity | `wrong_target_reopen` | `qualified` | `-` | `drill:profile:windows.stable:wrong_target_reopen` | - |
| Notification privacy | `lock_screen_privacy` | `qualified` | `-` | `drill:profile:windows.stable:lock_screen_privacy` | - |
| External-root recovery | `missing_root_recovery` | `qualified` | `-` | `drill:profile:windows.stable:missing_root_recovery` | - |
| Store-lock recovery | `store_lock` | `qualified` | `-` | `drill:profile:windows.stable:store_lock` | - |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop_qualification -- validate
cargo test -p aureline-shell --test m5_native_desktop_qualification_fixtures
python3 tools/ci/m5/native_desktop_qualification_check.py
```
