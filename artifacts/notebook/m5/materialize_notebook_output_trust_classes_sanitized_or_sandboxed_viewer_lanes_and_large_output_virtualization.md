# Notebook output trust classes, sanitized or sandboxed viewer lanes, and large-output virtualization

## Artifact summary

This artifact is the canonical checked-in packet for M05-016. It contains:

- Closed vocabulary listings for `output_viewer_lane_class`, `output_size_bucket`, and `output_virtualization_state_class`.
- Worked `NotebookOutputViewerLane` examples covering sanitized inline, sandboxed virtualized, trusted-active open-detail, blocked active content, and stale fallback states.
- Worked `LargeOutputVirtualizationRecord` examples covering small (no virtualization), large (virtualized), very-large (truncated), and large (lazy-pending) states.

## Provenance

- Packet id: `nb.output_viewer.packet.m5.01`
- Schema version: `1`
- As of: `2026-06-09T00:00:00Z`
- Source module: `crates/aureline-notebook/src/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/`

## Downstream consumers

- Docs and help surfaces ingest this packet to describe output viewer behavior.
- CI and release tooling validate that the packet parses and passes validation.
- Support exports reference this packet instead of re-describing viewer lane rules.
