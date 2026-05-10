# Proof packet: desktop topology smoke lane

Purpose: anchor proof captures for the unattended suspend/resume,
multi-monitor, and mixed-DPI smoke matrix that proves desktop
continuity on the claimed dogfood rows.

Reviewer entry point:
[`/artifacts/desktop/m1_topology_smoke_report.md`](../../desktop/m1_topology_smoke_report.md).

Canonical sources (non-exhaustive):

- `fixtures/desktop/m1_suspend_resume_matrix.yaml` — smoke rows,
  geometry blocks, expected truth tokens, and failure drills.
- `tests/desktop/topology_smoke/run_topology_smoke.py` — unattended
  runner that replays the matrix and emits the durable JSON capture.
- `artifacts/qa/window_display_matrix.yaml` — frozen window/display
  scenario rows and required drills the smoke matrix inherits from.
- `artifacts/platform/claimed_desktop_profiles.yaml` — claimed dogfood
  profile roster the smoke rows must resolve against.

Live runtime consumer (read-only):

- `crates/aureline-shell/src/windowing/display_safety.rs` — the
  display-safety guard the matrix projects against; the runner mirrors
  its pure-geometry math so the lane is unattended-runnable.
- `fixtures/windowing/topology_cases/` — low-level geometry fixtures
  the display-safety unit tests already consume.

Validation captures:

- `artifacts/milestones/m1/captures/topology_smoke_validation_capture.json`

Refresh: re-run the validation lane after a change to the runtime
windowing safety guard, the low-level geometry fixtures, the
window/display continuity matrix, or the claimed desktop profile
roster.

Closure rule: the lane stays open until the latest capture lands under
the governed proof root and the matrix's required scenario coverage
(suspend/resume, display topology change, mixed-DPI) all report PASS.
