# Desktop topology smoke report

Reviewer-facing entry point for the unattended suspend/resume,
multi-monitor, and mixed-DPI smoke lane that proves desktop continuity
on the claimed dogfood rows.

The lane is a closed loop:

1. **Matrix** —
   [`fixtures/desktop/m1_suspend_resume_matrix.yaml`](../../fixtures/desktop/m1_suspend_resume_matrix.yaml)
   names the smoke rows, the inherited window/display scenario ids,
   the claimed profile ids, the deterministic geometry blocks, and the
   expected truth tokens.
2. **Runner** —
   [`tests/desktop/topology_smoke/run_topology_smoke.py`](../../tests/desktop/topology_smoke/run_topology_smoke.py)
   replays each row and emits a durable JSON capture.
3. **Capture** —
   [`artifacts/milestones/m1/captures/topology_smoke_validation_capture.json`](../milestones/m1/captures/topology_smoke_validation_capture.json)
   records pass/fail per row plus the failure-drill replay.
4. **Proof packet** —
   [`artifacts/milestones/m1/proof_packets/topology_smoke.md`](../milestones/m1/proof_packets/topology_smoke.md)
   anchors the lane in the M1 proof index.

## Run the lane

```bash
python3 tests/desktop/topology_smoke/run_topology_smoke.py --repo-root .
```

Exit code is non-zero if any row fails the protected walk (clamp,
focus, authority, session truth) or the failure drill stops being
reproducible.

## Smoke rows seeded

| Smoke row | Inherits | Claimed profiles | Topology transition |
| --- | --- | --- | --- |
| `aureline.desktop.smoke.suspend_resume_remote_rebind_laptop_wake` | `suspend_resume_remote_rebind` | macOS 15+, Windows 11 23H2+, Ubuntu 24.04 GNOME Wayland | Laptop sleep with docked external display, wake with the external display detached. |
| `aureline.desktop.smoke.display_attach_detach_recenter` | `display_detach_dock_safe_bounds` | macOS 15+, Windows 11 23H2+, Ubuntu 24.04 GNOME X11, Fedora GNOME Wayland | External display undocked while a window occupies it. |
| `aureline.desktop.smoke.mixed_dpi_cross_monitor_reflow` | `mixed_dpi_cross_monitor_reflow` | macOS 15+, Windows 11 23H2+, Ubuntu 24.04 GNOME Wayland, Fedora GNOME Wayland | Window crosses from a 2x retina display onto a 1x external display. |

The protected walk for each row asserts the window is clamped on-screen,
focus stays recoverable, and the layout intent either survives or
downgrades honestly using the frozen
[`docs/ux/window_display_contract.md`](../../docs/ux/window_display_contract.md)
vocabulary.

## Failure drill posture

Each row also names a forced-failure input and the precise report the
matrix records when the upstream display-safety guard fails to clamp
or drops a topology class. Failures point to the actionable owner
(`@ahmeddyounis`) and the next action (re-run the upstream guard
against the named geometry fixture and confirm the missing class is
emitted).

## Where this fits in the claimed dogfood loop

- **Window/display scenarios** are reused from
  [`artifacts/qa/window_display_matrix.yaml`](../qa/window_display_matrix.yaml);
  the smoke rows do not invent a parallel scenario vocabulary.
- **Claimed dogfood profiles** are reused from
  [`artifacts/platform/claimed_desktop_profiles.yaml`](../platform/claimed_desktop_profiles.yaml);
  every smoke row's `claimed_profile_ids` must resolve.
- **Runtime safety guard** is the canonical truth at
  [`crates/aureline-shell/src/windowing/display_safety.rs`](../../crates/aureline-shell/src/windowing/display_safety.rs);
  the smoke runner mirrors that pure-geometry projection so the lane
  is unattended-runnable on CI without a graphical display.

## Refresh policy

Refresh the capture and update `as_of` when any of the following
change:

- the runtime windowing safety guard
  (`crates/aureline-shell/src/windowing/display_safety.rs`);
- the low-level geometry fixtures the guard already consumes
  (`fixtures/windowing/topology_cases/`);
- the window/display continuity matrix
  (`artifacts/qa/window_display_matrix.yaml`); or
- the claimed desktop profile roster
  (`artifacts/platform/claimed_desktop_profiles.yaml`).

Stale captures are surfaced by
[`ci/check_m1_artifact_index.py`](../../ci/check_m1_artifact_index.py).
