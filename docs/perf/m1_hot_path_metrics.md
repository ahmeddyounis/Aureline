# Hot-path metrics capture (startup, file open, typing, scrolling)

This document is the reviewer-facing entry point for the hot-path metrics and
trace-hook capture that protects the startup → open → type → scroll loop.

Normative sources (non-exhaustive):

- `schemas/traces/trace_event.schema.json`
- `artifacts/benchmarks/journey_segment_ids.yaml`
- `artifacts/perf/protected_path_ledger.yaml`
- `fixtures/perf/hot_path_trace_reference.json`

## Canonical code touchpoints

- `crates/aureline-telemetry/src/hot_path_metrics.rs`
  - Hot-path counters and trace-hook recorder.
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
  - Live wiring for milestone marks, file open/switch, and input-to-paint spans.

## Stable segment ids (trace-facing join points)

The collector emits `journey_segment_id` values that resolve against
`artifacts/benchmarks/journey_segment_ids.yaml`:

- `seg.startup.ui_dispatch.process_start`
- `seg.startup.ui_dispatch.boot`
- `seg.startup.ui_dispatch.first_useful_chrome_ready`
- `seg.first_paint.renderer_work.submit`
- `seg.quick_open.ui_dispatch.file_open_to_paint`
- `seg.quick_open.ui_dispatch.file_switch_to_paint`
- `seg.input_to_paint.ui_dispatch.keystroke_to_paint`
- `seg.input_to_paint.ui_dispatch.scroll_to_paint`

## Capturing a developer-local metrics trace

Run the shell with an output path:

- `cargo run -p aureline-shell --bin aureline_shell -- --emit-hot-path-metrics artifacts/milestones/m1/traces/hot_path_metrics.json`

Then exercise the protected loop:

1) open a file via the command palette,
2) type a few characters,
3) scroll the viewport,
4) close the window to flush the capture.

## Failure drill (counter/span rejection)

Trigger an open-file failure (for example, by committing a missing path) and
confirm the emitted capture includes at least one span closed with
`outcome_class = errored_caught` rather than silently dropping the event.

