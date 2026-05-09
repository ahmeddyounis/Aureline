# Proof packet: hot-path metrics and trace hooks

Purpose: anchor proof captures for startup, file open/switch, typing, and
scrolling hot-path metrics emitted by the live desktop shell.

Canonical sources (non-exhaustive):

- `crates/aureline-telemetry/src/hot_path_metrics.rs`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `artifacts/benchmarks/journey_segment_ids.yaml`
- `fixtures/perf/hot_path_trace_reference.json`
- `docs/perf/m1_hot_path_metrics.md`

Evidence storage:

- Traces: `artifacts/milestones/m1/traces/`
- Captures: `artifacts/milestones/m1/captures/`

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/hot_path_metrics_validation_capture.json`
- Command: `cargo test -p aureline-telemetry`

