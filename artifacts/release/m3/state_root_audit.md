# Beta state-root audit

This audit is generated from the beta ring-rollout packet and the exact-build install diagnostics packet. It keeps state-root ownership inspectable for silent deployment, managed rollout, self-serve rollout, and rollback review without embedding host-specific paths.

- **Audit id:** `release.state_root_audit.beta.rollback_safe_promotion`
- **Source packet:** `artifacts/release/m3/ring_rollout/packet.json`
- **Generated at:** `2026-05-17T16:30:00Z`
- **Exact build:** `build-id:aureline:beta:2.1.0-beta.1:aarch64-apple-darwin:release:b7ee32adb5eb`
- **State-root map:** `artifacts/release/state_root_map.yaml`

| Diagnostic row | Channel | Install mode | Updater owner | State roots | Review | Result |
|---|---|---|---|---|---|---|
| `install.diagnostics.windows.stable` | `stable` | `per_user_installed` | `user` | `state.per_user_configuration_root.stable`<br>`state.per_user_recovery_root.stable`<br>`state.per_user_derived_cache_root.stable` | `explicit_import_review_required` | `pass` |
| `install.diagnostics.windows.preview` | `preview` | `side_by_side_preview` | `user` | `state.per_user_configuration_root.preview`<br>`state.per_user_recovery_root.preview`<br>`state.per_user_derived_cache_root.preview` | `explicit_import_review_required` | `pass` |
| `install.diagnostics.windows.portable_stable` | `portable_stable` | `portable` | `user` | `state.portable_colocated_root.portable_stable` | `portable_no_os_ownership` | `pass` |
| `install.diagnostics.windows.managed_fleet` | `stable` | `managed_deployed` | `managed_fleet` | `state.per_user_configuration_root.stable`<br>`state.per_user_recovery_root.stable`<br>`state.per_user_derived_cache_root.stable`<br>`state.per_machine_shared_data_root.stable`<br>`state.per_machine_admin_policy_root.stable` | `admin_policy_review_required` | `pass` |

## Audit Row Refs

<a id="audit-row-install.diagnostics.windows.stable"></a>
- `install.diagnostics.windows.stable`
<a id="audit-row-install.diagnostics.windows.preview"></a>
- `install.diagnostics.windows.preview`
<a id="audit-row-install.diagnostics.windows.portable_stable"></a>
- `install.diagnostics.windows.portable_stable`
<a id="audit-row-install.diagnostics.windows.managed_fleet"></a>
- `install.diagnostics.windows.managed_fleet`

## Findings

- `state_root_audit.no_cross_channel_overlap`: Stable, preview, and portable rows do not share mutable durable roots.
- `state_root_audit.managed_policy_root_scoped`: Managed policy roots remain scoped to the managed fleet row and are exported as metadata only.
- `state_root_audit.exact_build_joined`: Every audited row resolves to the same exact-build identity carried by the artifact graph and install diagnostics.

## Consumer Rule

About, diagnostics, CLI, silent-deployment summaries, and support export quote these state-root refs. A rollout or rollback action is non-conforming if it names a state root that is absent from this audit or if it creates more than one active package state for a channel.
