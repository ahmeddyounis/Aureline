# M5 display-topology-recovery bounds-recovery and role-continuity registries

This lane is the display-topology-recovery implement lane over the frozen
[M5 window-restore matrix](./m5_window_restore_contract.md). It turns the *on-screen bounds-recovery* grammar and
the *role-continuity-fence* grammar into registry resolvers that produce export-safe, honest projections, so the
shell, recovery, diagnostics, admin, workspace, session, docs, CLI, and support surfaces resolve one canonical
display-topology-recovery truth instead of a per-surface, hand-copied remap. Every claimed M5 restore resolves
each window, dialog, or sheet to one explicit bounds-recovery posture (affinity monitor restored, clamped onto
visible bounds, rescaled for a DPI change, relocated to a primary fallback, or a fullscreen surface restored to
windowed bounds) that keeps the surface on visible bounds after display detach, dock / undock, DPI change, or
fullscreen / desktop moves while preserving the monitor-affinity hint and layout intent, and it fences off any
reset of an auxiliary window into a generic window, so follow / presentation state, collaboration role badges,
and auxiliary-window purpose stay visible after remap, material topology adjustments are recorded in restore
provenance and diagnostics, and a remap that only recovered bounds or context can never overclaim that layout
fidelity was unchanged.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_display_topology_recovery_and_role_continuity_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/shell/m5-display-topology-recovery-and-role-continuity-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/shell/m5-restore-fidelity.schema.json`](../../schemas/shell/m5-restore-fidelity.schema.json) and
  [`schemas/shell/m5-window-topology.schema.json`](../../schemas/shell/m5-window-topology.schema.json) as its
  canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/ui/m5-display-topology-recovery-and-role-continuity-registries/`
  (`dpi_rescale_beta_narrowed.json`, `reduced_fidelity_preview_narrowed.json`).

## Two registries

1. **On-screen bounds recovery** (`resolve_bounds_recovery_entry`) — resolves each window, dialog, or sheet to
   one stable bounds-recovery object: the bounds-recovery state and canonical bounds-recovery mode, the window
   surface, the monitor-affinity hint, the resolved visible bounds, the layout intent, the provenance class, and
   the distinct keyboard-reach plan. A clean entry names a canonical registry token, a classified bounds-recovery
   state, and a window-restore role, covers the canonical / accessible / audit resolution forms, publishes a
   complete object, resolves the bounds before the surface is presented, and records a material topology
   adjustment in provenance. Otherwise it degrades honestly — a window presented before its bounds were resolved
   onto visible bounds degrades to `present_preceded_bounds`.
2. **Role-continuity fence** (`resolve_role_continuity_fence_entry`) — blocks resetting a follow / presentation
   state, a collaboration role badge, or an auxiliary-window purpose into a generic window. A clean entry names a
   classified role-continuity class and provides the preserved-role-label / boundary-label / provenance-hint
   disclosure triple; a fence that resets a present role into generic, drops the role after remap, or hides that
   layout fidelity was reduced degrades to `role_continuity_resets_or_overclaims`.

## Per-remap bounds-recovery reference

The bounds-recovery state carries its canonical bounds-recovery mode, and the resolver publishes the full bounds
object, so the registry — never a hand-copied per-surface remap assumption — is the single source of truth.
`bounds_recovery_object_is_complete` rejects an object missing any field, `bounds_precede_present` rejects an
off-screen present, and `role_continuity_fence_holds` rejects a fence that reset a role into generic or hid a
reduced fidelity.

| bounds-recovery state | recovery mode | window surface | affinity hint | resolved bounds | provenance | keyboard reach |
| --- | --- | --- | --- | --- | --- | --- |
| affinity monitor restored | affinity_monitor_restored | `window-surface.editor.main` | `affinity.monitor.primary` | `bounds.visible.primary-1440p` | `provenance.live-layout` | `keyboard-reach.focus-cycle` |
| clamped onto visible bounds | clamped_onto_visible_bounds | `window-surface.preview.detached` | `affinity.monitor.secondary-detached` | `bounds.visible.primary-clamped` | `provenance.reduced-fidelity` | `keyboard-reach.focus-cycle` |
| rescaled for DPI change | rescaled_for_dpi_change | `window-surface.dialog.confirm` | `affinity.monitor.docked-hidpi` | `bounds.visible.docked-rescaled` | `provenance.reduced-fidelity` | `keyboard-reach.dialog-trap` |
| relocated to primary fallback | relocated_to_primary_fallback | `window-surface.aux.inspector` | `affinity.monitor.unplugged` | `bounds.visible.primary-fallback` | `provenance.reduced-fidelity` | `keyboard-reach.focus-cycle` |
| restored fullscreen to windowed | restored_fullscreen_to_windowed | `window-surface.presentation.main` | `affinity.monitor.primary` | `bounds.visible.primary-windowed` | `provenance.live-layout` | `keyboard-reach.focus-cycle` |

An off-screen present degrades to `present_preceded_bounds`, an incomplete object degrades to
`bounds_recovery_object_incomplete`, and a role reset degrades to `role_continuity_resets_or_overclaims`, so an
off-screen present, an incomplete object, or a role reset can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **No restored or remapped window can open wholly off-screen or strand a blocking dialog beyond keyboard
  reach.** The bounds are resolved before the surface is presented: a present-first example degrades, an unbound
  example degrades, a clean bounds-before-present entry is present, and no clean entry presented first.
- **Auxiliary windows preserve their intended role and boundary labels after monitor or DPI changes.** Clean
  bounds entries cover the canonical affinity-restored / clamped / rescaled / relocated / fullscreen-restored
  states and the first shell / recovery / diagnostics / admin / support surfaces, an object-incomplete example
  degrades, and no clean bounds entry published an incomplete object.
- **Display-topology drills fail when remap loses collaboration / presentation context or hides that fidelity was
  reduced.** Clean role-continuity-fence entries cover the follow-presentation / collaboration-badge /
  auxiliary-purpose classes with full resolution-form coverage while providing the disclosure triple, and a fence
  that resets a role into generic or hides that fidelity was reduced degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- support-export
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- csv
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- report
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- bounds-recovery-table
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- fixture-dpi-rescale-beta-narrowed
cargo run -p aureline-ui --example dump_m5_display_topology_recovery_and_role_continuity_registries -- fixture-reduced-fidelity-preview-narrowed
```
