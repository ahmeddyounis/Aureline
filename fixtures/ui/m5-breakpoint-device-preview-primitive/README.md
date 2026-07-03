# M5 Breakpoint / Device-Preview Row Primitive Fixtures

Protected fixture corpus for the M5 breakpoint / device-preview row primitive — the
device preview row, the live-versus-mock runtime-truth cue, and the compare /
open-source continuity actions resolved once per preview target (M05-807, batch
B94).

- `breakpoint_device_preview_primitive_stable.json` — the canonical, fully valid
  primitive packet. It binds all six claimed visual-design surface families
  (`desktop_designer`, `source_first_preview`, `browser_runtime_inspector`,
  `framework_pack_preview`, `embedded_shell_designer`, `support_export_replay`)
  and carries worked preview cases that exercise a mock / captured / stale view
  disclosing its runtime truth (AC1), a source-anchored device / breakpoint switch
  (AC2), and a degrade explained with a shared downgrade trigger (AC3), including
  live-desktop-and-mobile, mock, tethered-stale, captured-snapshot, runtime-only,
  and unknown-freshness cases. This is a byte-identical copy of the checked support
  export at
  `artifacts/release/m5-breakpoint-device-preview-proof/support_export.json`, which
  is the `include_str!` source of truth verified by
  `checked_support_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-breakpoint-device-preview-primitive.schema.json` and against
`M5BreakpointPreviewPacket::validate()`. Each worked preview case is
self-consistent: its stored resolution equals a fresh
`resolve_breakpoint_preview(&input)`. Fixtures carry only typed class tokens, opaque
target / span refs, opaque viewport / breakpoint / variant tokens, booleans, and
redacted labels — never raw source bodies, screenshots, runtime payloads, or
credentials.
