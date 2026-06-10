# Notebook debugger bridge, frame-to-cell linkage, and kernel restart consequence records

## Artifact summary

This artifact is the canonical checked-in packet for M05-027. It contains:

- Closed vocabulary listings for `debugger_frame_cell_link_class`, `debugger_frame_cell_link_posture_class`, `kernel_restart_kind`, `kernel_restart_consequence_class`, and `kernel_restart_debugger_action_class`.
- Worked `DebuggerFrameCellLink` examples covering exact cell match, nearest cell heuristic, no cell mapping, in-cell library code, in-cell external dependency, and frame stale after restart.
- Worked `KernelRestartDebuggerConsequence` examples covering bridge preserved, bridge reset, bridge cancelled, bridge unavailable, breakpoints retained, breakpoints lost, variable state lost, and execution queue cleared.

## Provenance

- Packet id: `nb.debugger.bridge.packet.m5.01`
- Schema version: `1`
- As of: `2026-06-09T00:00:00Z`
- Source module: `crates/aureline-notebook/src/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records/`

## Downstream consumers

- Docs and help surfaces ingest this packet to describe debugger frame-to-cell linkage and restart consequences.
- CI and release tooling validate that the packet parses and passes validation.
- Support exports reference this packet instead of re-describing debugger-bridge rules.
