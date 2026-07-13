# M5 install-topology and state-root registries

This lane is the first implement lane over the frozen
[M5 install-topology matrix](./m5_install_topology_contract.md). It turns the *per-user managed /
per-machine managed / side-by-side stable-plus-preview* install-topology grammar and the *portable-mode /
offline-air-gap* state-root-boundary grammar into registry resolvers that produce export-safe, honest
projections, so About, update, diagnostics, admin, installer, docs, CLI, and support surfaces resolve one
canonical delivery-topology and state-root truth instead of a per-surface, hand-copied path assumption.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_install_topology_and_state_root_registries` (the authoritative validator).
- **Combined schema:**
  `schemas/install/m5-install-topology-and-state-root-registries.schema.json`.
- **Domain schemas:** install-topology rows point at
  [`schemas/install/m5-install-topology.schema.json`](../../schemas/install/m5-install-topology.schema.json)
  and state-root-boundary rows point at
  [`schemas/install/m5-state-root-boundaries.schema.json`](../../schemas/install/m5-state-root-boundaries.schema.json)
  as their canonical domain contracts.
- **Checked proof:** `artifacts/release/m5-install-topology-and-state-root-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/install/m5-install-topology-and-state-root-registries/`
  (`side_by_side_channel_beta_narrowed.json`, `offline_airgap_bundle_preview_narrowed.json`).

## Two registries

1. **Install topology** (`resolve_install_topology_entry`) — publishes one stable install-topology object per
   delivery profile: install mode, channel, updater owner, binary / artifact root, primary writable state
   roots, policy roots, and rollback target. A clean entry names a canonical registry token, a classified
   delivery scope, and an install-topology role, covers the canonical / accessible / audit resolution forms,
   publishes a complete object, keeps managed-versus-user scopes and side-by-side channels on explicitly
   isolated state namespaces, and explains any coexistence handoff. Otherwise it degrades honestly — a preview
   channel that reuses a stable state namespace without an explicit handoff degrades to
   `state_namespace_reused_without_handoff`.
2. **State-root boundaries** (`resolve_state_root_boundary_entry`) — keeps portable-mode and offline / air-gap
   state-root boundaries truthful and complete. A clean entry names a classified state-root surface and
   provides the writable-state-root / policy-root / rollback-target disclosure triple; a boundary that spills
   hidden machine-global durable state, narrows rollback below the full artifact graph, or asserts an
   unexplained scope degrades to `state_boundary_untruthful_or_incomplete`.

## Per-family install-topology / state-root reference

The delivery scope carries its canonical install mode, and the resolver publishes the full topology object, so
the registry — never a hand-copied per-profile assumption — is the single source of truth.
`install_topology_object_is_complete` rejects an object missing any field, and `state_namespace_is_isolated`
rejects a reused state namespace.

| delivery scope | install mode | updater owner | binary root | writable state roots | policy roots | rollback target |
| --- | --- | --- | --- | --- | --- | --- |
| per-user managed | per_user_managed_install | per_user_updater | `%LOCALAPPDATA%\Aureline\app` | `%LOCALAPPDATA%\Aureline\state` | `%LOCALAPPDATA%\Aureline\policy` | `artifact-graph:per-user:stable` |
| per-machine managed | per_machine_managed_install | admin_owned_updater | `C:\Program Files\Aureline` | `C:\ProgramData\Aureline\state` | `C:\ProgramData\Aureline\policy` | `artifact-graph:per-machine:stable` |
| side-by-side channels | side_by_side_channels | per_user_updater | `~/Applications/Aureline Preview.app` | `~/Library/Application Support/Aureline Preview` | `~/Library/Application Support/Aureline Preview/policy` | `artifact-graph:side-by-side:preview` |
| portable mode | (state-root boundary) | — | `.\AurelinePortable` | `.\AurelinePortable\state` | `.\AurelinePortable\policy` | `artifact-graph:portable:full` |
| offline / air-gap | (state-root boundary) | offline_updater | `/opt/aureline-offline` | `/opt/aureline-offline/state` | `/opt/aureline-offline/policy` | `artifact-graph:offline:bundled-full` |

A reused state namespace degrades to `state_namespace_reused_without_handoff`, an incomplete object degrades to
`install_topology_object_incomplete`, and a hidden machine-global spill degrades to
`state_boundary_untruthful_or_incomplete`, so a reused namespace, an incomplete object, or a hidden spill can
never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Every claimed delivery profile resolves to one stable install-topology object with channel / owner / root /
  rollback fields.** Clean install entries cover the canonical per-user / per-machine / side-by-side delivery
  scopes and the first About / update / diagnostics / admin / support surfaces, an object-incomplete example
  degrades, and no clean install entry published an incomplete object.
- **Diagnostics and support packets report install topology and state-root boundaries without manual
  reconstruction.** Shared-versus-isolated state is explicit: a reused-namespace example and an unbound example
  degrade, a clean isolated install entry is present, and no clean entry lost namespace isolation.
- **The suite fails when a platform cannot explain shared-versus-isolated state for the active profile.** Clean
  state-root entries cover the portable / offline / diagnostics surfaces with full resolution-form coverage
  while providing the disclosure triple, and a boundary that hides a machine-global spill degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- support-export
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- csv
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- report
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- state-root-boundary-table
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- fixture-side-by-side-channel-beta-narrowed
cargo run -p aureline-ui --example dump_m5_install_topology_and_state_root_registries -- fixture-offline-airgap-bundle-preview-narrowed
```
