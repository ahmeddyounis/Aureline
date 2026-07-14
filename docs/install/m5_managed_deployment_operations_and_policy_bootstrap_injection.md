# M5 managed-deployment operations and policy-bootstrap-injection registries

This lane is the managed-deployment execution lane over the frozen
[M5 install-topology matrix](./m5_install_topology_contract.md) and its
[install-topology and state-root registries](./m5_install_topology_and_state_root_registries.md). It
makes *managed deployment* a contract instead of a set of installer flags: it resolves every claimed
managed profile's silent install, silent uninstall, repair-or-verify, channel-pinning, and update-deferral
operation to one inspectable object, publishes the complete copyable receipt (install ID, timestamp,
failure summary, repair/verify receipt), keeps admin-owned versus user-owned responsibilities explicit so a
managed installer never looks user-controlled, and publishes the policy-bundle / bootstrap injection —
policy-bundle source, bootstrap target, applied settings, admin owner, and deferral window — with documented
channel-pin / update-deferral continuity. Installer, update, diagnostics, admin, docs, and support surfaces
resolve one canonical managed-deployment truth instead of a per-surface, hand-copied receipt assumption.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_managed_deployment_operations_and_policy_bootstrap_injection` (the
  authoritative validator).
- **Schema:**
  `schemas/install/m5-managed-deployment-operations-and-policy-bootstrap-injection.schema.json`.
- **Upstream contracts:** rows point back at the frozen
  [`schemas/install/m5-install-topology-matrix.schema.json`](../../schemas/install/m5-install-topology-matrix.schema.json),
  the [`schemas/install/m5-install-topology.schema.json`](../../schemas/install/m5-install-topology.schema.json)
  managed install-topology grammar, and the
  [`schemas/install/m5-install-topology-and-state-root-registries.schema.json`](../../schemas/install/m5-install-topology-and-state-root-registries.schema.json)
  implement lane as their canonical delivery-topology source.
- **Checked proof:**
  `artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/install/m5-managed-deployment-operations-and-policy-bootstrap-injection/`
  (`per_machine_managed_beta_narrowed.json`, `offline_airgap_bundle_preview_narrowed.json`).

## Two registries

1. **Managed operation** (`resolve_managed_operation_entry`) — publishes one inspectable managed operation per
   profile: a supported silent-install / silent-uninstall / repair-or-verify / channel-pin / update-defer
   operation, an operation-target root, a receipt root, a failure-diagnostics root, the complete copyable
   receipt inventory (install ID, timestamp, failure summary, repair/verify receipt), and a disclosed
   admin-versus-user ownership. A clean entry names a canonical registry token, covers the canonical /
   accessible / audit presentation forms, stays accountable with no misrepresentation, and inventories every
   receipt field. Otherwise it degrades honestly — a managed installer presented as user-controlled degrades
   to `managed_installer_presented_as_user_controlled`, and an ambiguous ownership degrades so a failure can
   never strand the user without knowing who owns the operation. `managed_operation_is_accountable` is the
   guardrail that rejects an unsupported operation, an incomplete receipt, or any misrepresentation.
2. **Policy-bootstrap injection** (`resolve_policy_bootstrap_injection_entry`) — keeps the policy-bundle /
   bootstrap injection published and channel-pin / update-deferral continuity documented. A clean entry names
   a classified injection surface, discloses the policy-bundle source, bootstrap target, applied settings,
   admin owner, and deferral window, and documents channel-pin / update-deferral continuity; an injection
   surface that hides a disclosure field or drops continuity notes degrades honestly.

## Managed-operation receipt reference

The operation entry carries its operation, operation-target root, receipt root, failure-diagnostics root, and
a disclosed ownership, so the registry — never a hand-copied per-profile assumption — is the single source of
truth. `render_managed_operation_receipt_table()` renders exactly this, and only clean, accountable operations
appear.

| profile_id | operation | operation_target_root | receipt_root | failure_diagnostics_root | ownership |
| --- | --- | --- | --- | --- | --- |
| `profile.per_machine_managed` | silent_install | `%ProgramFiles%\Aureline` | `%ProgramData%\Aureline\receipts` | `%ProgramData%\Aureline\logs` | admin_owned |
| `profile.per_user_managed` | silent_uninstall | `%LOCALAPPDATA%\Aureline` | `%LOCALAPPDATA%\Aureline\receipts` | `%LOCALAPPDATA%\Aureline\logs` | user_owned |

A managed installer presented as user-controlled degrades to
`managed_installer_presented_as_user_controlled`, an incomplete receipt inventory degrades, and an ambiguous
ownership degrades, so a misrepresentation, an incomplete receipt, or an ambiguous ownership can never turn
release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Claimed managed profiles expose install / uninstall / repair-verify / pinning / deferral through one
  inspectable contract.** Clean operation entries cover the silent-install, silent-uninstall, repair-or-verify,
  channel-pin, and update-defer operations across the installer / update / diagnostics / admin / support
  surfaces with a complete receipt inventory, a receipt-incomplete example degrades, and no clean operation
  published an incomplete receipt (`managed_operation_contract_not_proven` otherwise).
- **Managed-install failures preserve actionable diagnostics and do not strand the user in ambiguous ownership
  state.** An ownership-ambiguous example degrades, at least one clean disclosed operation entry is present,
  and no clean operation entry is ambiguous (`ownership_disclosure_not_proven` otherwise).
- **Enterprise rollout drills fail when bootstrap-policy injection, channel pinning, or repair/verify semantics
  drift from the published matrix.** A misrepresented-installer example degrades, no clean operation was
  misrepresented, an injection-disclosure-incomplete example degrades, and a pin-and-deferral-continuity
  example degrades (`drift_detection_not_proven` otherwise).

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- support-export
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- csv
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- report
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- receipt-table
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- fixture-per-machine-managed-beta-narrowed
cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- fixture-offline-airgap-bundle-preview-narrowed
```
