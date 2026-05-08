# Proof packet: native window bootstrap and input dispatch

Reviewer entrypoint: `docs/architecture/m1_window_bootstrap.md`.

This lane exists to keep the native desktop shell bootstrap canonical and
traceable: window creation, event-loop wiring, input dispatch root, and the
startup milestones used by later protected-path traces.

## Evidence

- Startup trace (developer-local capture):
  - `artifacts/milestones/m1/traces/window_bootstrap_startup_trace.json`
- Degraded drill capture (clipboard disabled at startup):
  - `artifacts/milestones/m1/traces/window_bootstrap_startup_trace.degraded_clipboard_disabled.json`
- Validation capture:
  - `artifacts/milestones/m1/captures/window_bootstrap_validation_capture.json`

