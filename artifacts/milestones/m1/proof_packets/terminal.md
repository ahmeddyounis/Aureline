# Proof packet: terminal lane

Purpose: anchor integrated terminal proof captures in one indexed location.

Canonical sources (non-exhaustive):

- `artifacts/terminal/shell_integration_signals.yaml`
- `fixtures/terminal/session_cases/local_session_real_pty_close_provenance.json`

Evidence storage:

- Smoke outputs: `artifacts/milestones/m1/smoke_outputs/`
- Traces: `artifacts/milestones/m1/traces/`

Current implementation status:

- `aureline-terminal` now opens a local desktop PTY through `portable-pty`
  and spawns the user's default shell for fresh local sessions.
- The host keeps the canonical session header while routing input bytes to
  the PTY writer and draining output bytes from a bounded reader-thread ring.
- Clean child exit, explicit close, resize, forced transport loss, same-target
  reconnect, and launch-failure quarantine all reuse the existing lifecycle
  vocabulary.
