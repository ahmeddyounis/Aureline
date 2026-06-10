# Notebook debugger-support states, breakpoint affordances, and unsupported-state cues

## Artifact summary

This artifact is the canonical checked-in packet for M05-021. It contains:

- Closed vocabulary listings for `debugger_support_state_class`, `breakpoint_affordance_class`, `breakpoint_affordance_posture_class`, and `unsupported_state_cue_class`.
- Worked `NotebookDebuggerSupportState` examples covering idle, paused, stepping, running, disconnected, unsupported, partially supported, and degraded states.
- Worked `BreakpointAffordance` examples covering set, remove, enable, disable, clear-all, and set-conditional actions with available and unavailable postures.
- Worked `UnsupportedStateCue` examples covering no-kernel, adapter-unavailable, kernel-does-not-implement-debug-protocol, remote-parity-unverified, policy-blocked, bridge-cancelled-by-restart, and cell-stepping-unsupported cues.

## Provenance

- Packet id: `nb.debugger.packet.m5.01`
- Schema version: `1`
- As of: `2026-06-09T00:00:00Z`
- Source module: `crates/aureline-notebook/src/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues/`

## Downstream consumers

- Docs and help surfaces ingest this packet to describe debugger-support behavior and breakpoint affordances.
- CI and release tooling validate that the packet parses and passes validation.
- Support exports reference this packet instead of re-describing debugger-support rules.
