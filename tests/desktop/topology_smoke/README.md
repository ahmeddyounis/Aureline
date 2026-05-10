# Desktop topology smoke

Unattended proof lane that replays the suspend/resume, monitor
attach/detach, and mixed-DPI rows in
[`fixtures/desktop/m1_suspend_resume_matrix.yaml`](../../../fixtures/desktop/m1_suspend_resume_matrix.yaml)
against pure-geometry projections of the runtime windowing safety guard
in
[`crates/aureline-shell/src/windowing/display_safety.rs`](../../../crates/aureline-shell/src/windowing/display_safety.rs).

The lane is deliberately runnable on CI/nightly without a graphical
display: it consumes the same low-level geometry shape that the
display-safety unit tests under
[`fixtures/windowing/topology_cases/`](../../../fixtures/windowing/topology_cases/)
already use.

## What the lane proves

For every row in the matrix the runner asserts:

- **Clamped on-screen** — after the named topology change, the window's
  safe-bounds anchor lands inside a real display rectangle. No silent
  off-screen stranding.
- **Layout intent survives or downgrades honestly** — the row declares
  one of `exact_restore`, `compatible_restore`, `layout_only`,
  `evidence_only`, or `no_restore` from the frozen window/display
  matrix vocabulary.
- **Topology change classes are recorded** — `scale_changed` for
  mixed-DPI rows; `display_removed` for rows that drop a display;
  `wake_display_reconnect` or `safe_bounds_changed` for suspend/resume
  rows.
- **No silent re-runs on suspend/resume** — task, debug, and preview
  surfaces never claim `live_session_continued` after wake; the
  `no_hidden_rerun_live_surfaces` drill is required.
- **Failure drills are reproducible** — each row names a forced-failure
  input plus the precise report the smoke lane records when the
  display-safety guard fails to clamp or drops a topology class.

## Run

```bash
python3 tests/desktop/topology_smoke/run_topology_smoke.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/topology_smoke_validation_capture.json`
and exits non-zero on any check failure.

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture (defaults to
  `artifacts/build/build_identity.json`).

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `artifacts/desktop/m1_topology_smoke_report.md` |
| Smoke matrix | `fixtures/desktop/m1_suspend_resume_matrix.yaml` |
| Latest capture | `artifacts/milestones/m1/captures/topology_smoke_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/topology_smoke.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.topology_smoke` so reviewers can find the latest
capture, owner, and validation-lane reference without searching ad hoc
folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `crates/aureline-shell/src/windowing/display_safety.rs`
- `fixtures/windowing/topology_cases/`
- `artifacts/qa/window_display_matrix.yaml`
- `artifacts/platform/claimed_desktop_profiles.yaml`

Stale captures are surfaced by the artifact-index validator at
`ci/check_m1_artifact_index.py`.
