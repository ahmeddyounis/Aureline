# Runtime Continuity Surface Qualification Packet

This file is the artifact-level companion document for the checked-in
runtime-continuity surface qualification packet.

- **Canonical JSON**:
  `artifacts/runtime/runtime-continuity-surface-qualification/support_export.json`
- **Schema**:
  `schemas/runtime/runtime-continuity-surface-qualification.schema.json`
- **Typed consumer**:
  `crates/aureline-runtime/src/runtime_continuity_surface_qualification/mod.rs`

The packet is the canonical runtime-continuity evidence index for the currently
claimed M5 profiles that consume queue fairness, restore fidelity, transcript
export, protocol/clipboard, and shared-control truth from the checked
`queue_session_terminal_governance` packet.

The packet auto-narrows any profile that loses current queue/restore/terminal
proof instead of allowing generic desktop continuity to overclaim operability on
mirrored, managed, or browser-handoff surfaces.
