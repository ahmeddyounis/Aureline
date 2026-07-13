# M5 monitor-topology geometry-remap and restore-bounds registries

This lane is the monitor-topology continuation of the responsive-geometry family named by the frozen
[M5 shell-metric / density matrix](m5_shell_metric_density_contract.md). It ties concrete shell geometry to
real desktop topology changes so a monitor attach or detach, a DPI change, an undock, a fullscreen transition,
or a snapped-layout recovery preserves a usable on-screen layout rather than replaying stale absolute
coordinates.

The authoritative gate is the Rust validator in `crates/aureline-ui`
(`m5_monitor_geometry_remap_and_restore_bounds`). This document describes the contract; the checked-in support
export under `artifacts/release/m5-monitor-geometry-remap-and-restore-bounds-proof/` and the narrowed fixtures
under `fixtures/ui/m5-monitor-geometry-remap-and-restore-bounds/` are minted from the same seed builders.

## Two registry resolvers

- **`resolve_restore_bounds_entry`** — monitor-aware restore bounds. A restore entry reads clean only when it
  names a canonical registry token, a classified restore-surface kind, and a classified topology change; clamps
  the restored surface into visible bounds; never reopens fully off-screen or traps focus; preserves usable
  editor / panel / inspector geometry; persists layout intent and monitor-affinity hints instead of stale
  absolute coordinates; and — whenever fidelity is reduced — offers a one-click or command-backed recenter /
  reset affordance.
- **`resolve_geometry_remap_provenance_entry`** — geometry-remap provenance. A provenance entry reads clean only
  when it names a classified topology change and fidelity outcome; preserves the workspace, focus chain, and
  critical state; records the remap reason; and records every mandatory provenance field so support can
  diagnose why fidelity changed.

## Restore-surface kinds

| Kind | Meaning |
| --- | --- |
| `restorable_window` | A restorable top-level window. |
| `approval_sheet` | An approval sheet; called out directly by the acceptance criteria. |
| `dialog` | A dialog. |
| `docked_panel` | A docked panel restored into a shell zone. |
| `split_layout` | A restored split layout. |

## Topology changes

| Change | Meaning |
| --- | --- |
| `monitor_attach` | A monitor was attached. |
| `monitor_detach` | A monitor was detached. |
| `undock` | The device was undocked. |
| `dpi_change` | The DPI / scale factor changed (a mixed-DPI transition). |
| `fullscreen_transition` | A fullscreen transition entered or left. |
| `snapped_layout_recovery` | A snapped-layout recovery. |

## Fidelity outcomes (descending fidelity)

| Outcome | Meaning |
| --- | --- |
| `exact_bounds_restored` | The exact persisted bounds were restored (full fidelity). |
| `proportional_intent_remap` | Layout intent and proportions were remapped onto the new topology. |
| `monitor_affinity_fallback` | The affinity hint could not be honored; fell back to a nearby monitor. |
| `recenter_reset` | The surface was recentered / reset into visible bounds (failure-safe recovery). |

A reduced-fidelity outcome (`proportional_intent_remap`, `monitor_affinity_fallback`, `recenter_reset`) must be
surfaced as recoverable product truth — a recenter / reset affordance on the restore entry, and a recorded
fidelity outcome in provenance — rather than a silent regression.

## Hard invariants

Every registry row asserts, and the validator enforces, that:

- a restore never reopens fully off-screen or traps focus after a monitor or DPI change;
- a remap never replays stale absolute coordinates without a visible-bounds clamp;
- a remap never silently drops the workspace, focus chain, or critical state; and
- reduced fidelity is never left without a recenter / reset affordance or recorded provenance.

## Acceptance criteria (proven by resolved examples)

1. **No off-screen or focus-trap restore.** Clean restore entries cover the canonical topology changes and
   restore-surface kinds; an off-screen and a focus-trap example degrade; no clean entry reopens off-screen or
   traps focus.
2. **Mixed-DPI usable geometry.** Clean restore entries cover the DPI-change trigger and the first shell /
   editor / review / notebook / data surfaces; a loses-usable-geometry example degrades; at least one clean
   reduced-fidelity restore surfaces a recoverable recenter affordance; no clean entry loses usable geometry.
3. **Diagnosable remap provenance.** Clean provenance entries cover the canonical fidelity outcomes; a
   provenance-incomplete and a reason-unrecorded example degrade; no clean provenance entry silently drops
   workspace or state.

## Source contracts

Each row and the packet trace back to the frozen matrix and the canonical density-mode domain schema
(`schemas/shell/m5-density-mode.schema.json`), the domain contract the responsive-geometry family resolves
from, so no surface forks a private monitor-topology meaning.
