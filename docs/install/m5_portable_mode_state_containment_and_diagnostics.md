# M5 portable-mode state-containment and diagnostics registries

This lane is the portable-mode runtime-enforcement lane over the frozen
[M5 install-topology matrix](./m5_install_topology_contract.md) and its
[install-topology and state-root registries](./m5_install_topology_and_state_root_registries.md). It
makes *portable* a contract instead of a marketing shortcut: it resolves every claimed portable profile
to a colocated or explicitly named sibling-state layout, inventories the complete durable-root set of
settings / secrets / services / shell hooks, proves hidden machine-global mutation is absent or explicitly
blocked, keeps portable state distinguishable from ordinary installed state, and publishes discoverable
portable-mode diagnostics — executable root, state roots, log / crash locations, update posture, and any
unsupported shell-integration paths — with documented retained-versus-replaced update continuity. About,
update, diagnostics, admin, docs, and support surfaces resolve one canonical portable-mode truth instead of
a per-surface, hand-copied path assumption.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_portable_mode_state_containment_and_diagnostics` (the authoritative validator).
- **Schema:**
  `schemas/install/m5-portable-mode-state-containment-and-diagnostics.schema.json`.
- **Upstream contracts:** rows point back at the frozen
  [`schemas/install/m5-install-topology-matrix.schema.json`](../../schemas/install/m5-install-topology-matrix.schema.json),
  the [`schemas/install/m5-state-root-boundaries.schema.json`](../../schemas/install/m5-state-root-boundaries.schema.json)
  portable-mode / offline state-root grammar, and the
  [`schemas/install/m5-install-topology-and-state-root-registries.schema.json`](../../schemas/install/m5-install-topology-and-state-root-registries.schema.json)
  implement lane as their canonical delivery-topology source.
- **Checked proof:** `artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/install/m5-portable-mode-state-containment-and-diagnostics/`
  (`side_by_side_channel_beta_narrowed.json`, `offline_airgap_bundle_preview_narrowed.json`).

## Two registries

1. **Portable state layout** (`resolve_portable_state_layout_entry`) — publishes one contained portable-state
   layout per profile: a colocated-under-executable or explicitly named sibling-directory containment, an
   executable root, a colocated state root, a log-and-crash root, the complete durable-class inventory
   (durable settings, stored secrets, background services, shell hooks), and a distinguishable state origin.
   A clean entry names a canonical registry token, covers the canonical / accessible / audit presentation
   forms, keeps the layout contained with no hidden machine-global write, and inventories every durable root.
   Otherwise it degrades honestly — a durable class that wrote to a hidden machine-global path degrades to
   `hidden_machine_global_durable_spill`, and an ambiguous origin degrades so portable state can never be
   confused with ordinary installed state. `portable_layout_is_contained` is the guardrail that rejects a
   non-colocated / non-sibling containment or any hidden machine-global spill.
2. **Portable diagnostics** (`resolve_portable_diagnostics_entry`) — keeps portable-mode diagnostics
   discoverable and update continuity documented. A clean entry names a classified diagnostics surface,
   discloses the executable root, state roots, log-and-crash locations, update posture, and any unsupported
   shell-integration paths, and documents retained-versus-replaced update continuity; a diagnostics surface
   that hides a disclosure field or drops update-continuity notes degrades honestly.

## Portable root inventory reference

The layout entry carries its containment, executable root, colocated state root, log-and-crash root, and a
distinguishable state origin, so the registry — never a hand-copied per-profile assumption — is the single
source of truth. `render_portable_root_inventory_table()` renders exactly this, and only clean, contained
layouts appear.

| profile_id | containment | executable_root | colocated_state_root | log_and_crash_root | state_origin |
| --- | --- | --- | --- | --- | --- |
| `profile.portable_colocated` | colocated_under_executable | `.\AurelinePortable\app` | `.\AurelinePortable\state` | `.\AurelinePortable\logs` | portable_colocated |
| `profile.portable_named_sibling` | named_sibling_directory | `.\AurelinePortable\app` | `.\Aureline-Portable-State` | `.\Aureline-Portable-State\logs` | portable_named_sibling |

A hidden machine-global spill degrades to `hidden_machine_global_durable_spill`, an incomplete durable-root
inventory degrades, and an ambiguous origin degrades, so a spill, an incomplete inventory, or an
indistinguishable origin can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Portable mode can identify all durable roots and prove hidden machine-global mutation is absent or
  explicitly blocked.** Clean layout entries cover the colocated and named-sibling containments across the
  About / update / diagnostics / admin / docs / support surfaces with a complete durable-root inventory, an
  inventory-incomplete example degrades, and no clean layout published an incomplete inventory
  (`portable_root_inventory_not_proven` otherwise).
- **Support / export paths can distinguish portable state from ordinary installed state without guessing.**
  An origin-ambiguous example degrades, at least one clean distinguishable layout entry is present, and no
  clean layout entry is ambiguous (`portable_state_distinguishability_not_proven` otherwise).
- **Portable-profile tests fail when durable settings, secrets, or services spill outside documented portable
  roots.** A hidden-machine-global-spill example degrades, no clean layout entry spilled, a
  diagnostics-disclosure-incomplete example degrades, and an update-continuity example degrades
  (`portable_spill_detection_not_proven` otherwise).

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- support-export
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- csv
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- report
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- root-inventory-table
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- fixture-side-by-side-channel-beta-narrowed
cargo run -p aureline-ui --example dump_m5_portable_mode_state_containment_and_diagnostics -- fixture-offline-airgap-bundle-preview-narrowed
```
