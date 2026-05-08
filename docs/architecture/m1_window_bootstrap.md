# Native window bootstrap and input dispatch (desktop shell)

This document is the reviewer-facing entry point for the native desktop shell
bootstrap: window creation, event-loop wiring, input dispatch, and the startup
milestones that seed protected-path traces.

Normative sources:

- `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`
- `docs/architecture/critical_sequence_diagrams.md` (Warm startup to first edit)
- `schemas/traces/trace_event.schema.json`
- `artifacts/benchmarks/journey_segment_ids.yaml`
- `artifacts/perf/protected_path_ledger.yaml`

## Canonical code touchpoints

- `crates/aureline-shell/src/bootstrap/mod.rs`
  - Canonical entry point for native desktop shell bootstrap.
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
  - Owns the live `winit` event loop, input dispatch root, and startup milestone
    capture hook points.
- `crates/aureline-shell/src/windowing/winit_softbuffer.rs`
  - Owns native window creation plus the `softbuffer` surface binding.

## Startup milestones (trace-facing vocabulary)

The native shell bootstrap emits three milestone marks intended to be consumed
by later trace sinks and dashboards via the normalised trace-event vocabulary.

Current milestone → trace join points:

- First interactive shell
  - Meaning (current): command entry is ready (registry + keybinding resolver
    seeded; palette is admissible).
  - Segment id: `seg.startup.ui_dispatch.first_useful_chrome_ready`
  - Budget ref: `path.shell.first_useful_chrome`

- Editor surface ready
  - Meaning (current): canonical desktop frame exists and the main-workspace
    surface can host editor groups (placeholder occupant wiring is live).
  - Segment id: `seg.startup.ui_dispatch.boot`
  - Budget ref: `path.shell.launch`

- First shell frame submitted
  - Meaning (current): first rendered frame has been composed and submitted to
    the software surface.
  - Segment id: `seg.first_paint.renderer_work.submit`
  - Budget ref: `path.shell.launch`

These marks are intentionally narrow: they name *bootstrap readiness* and do not
claim a full editor file-open or first-edit path yet (see
`docs/architecture/critical_sequence_diagrams.md` stages
`seq_stage.warm_start.quick_open_buffer` and `seq_stage.warm_start.first_edit_paint`).

## Failure drill (input device unavailable)

The bootstrap supports a degraded run where clipboard support is unavailable at
startup. The shell still boots to a truthful first state and remains keyboard
operable.

Use `--disable-clipboard` when launching `aureline_shell`.

## Capturing a developer-local startup trace

The shell can emit a small JSON export of normalised trace-event records for the
startup milestones:

- `cargo run -p aureline-shell --bin aureline_shell -- --emit-startup-trace artifacts/milestones/m1/traces/window_bootstrap_startup_trace.json --exit-after-first-frame`

Degraded capture variant (clipboard disabled):

- `cargo run -p aureline-shell --bin aureline_shell -- --emit-startup-trace artifacts/milestones/m1/traces/window_bootstrap_startup_trace.degraded_clipboard_disabled.json --exit-after-first-frame --disable-clipboard`

