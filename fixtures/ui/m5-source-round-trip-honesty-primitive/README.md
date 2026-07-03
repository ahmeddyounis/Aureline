# M5 Source-Round-Trip Honesty Primitive Fixtures

Protected fixture corpus for the M5 source-round-trip honesty primitive — the
source-sync chip, the round-trip conflict banner, the unsupported-construct card,
and the generated-or-protected-file boundary notice resolved once per designer
target (M05-806, batch B94).

- `round_trip_honesty_primitive_stable.json` — the canonical, fully valid primitive
  packet. It binds all six claimed visual-design surface families
  (`desktop_designer`, `source_first_preview`, `browser_runtime_inspector`,
  `framework_pack_preview`, `embedded_shell_designer`, `support_export_replay`)
  and carries worked round-trip cases that exercise a hard block refusing a silent
  write (AC1), an exact source-first fallback named when round-trip support drops
  (AC2), and a narrowing / read-only outcome explained with a downgrade trigger
  (AC3), including conflict, unsupported-construct, generated-file, runtime-only,
  and mixed-managed-region cases. This is a byte-identical copy of the checked
  support export at
  `artifacts/release/m5-source-round-trip-honesty-proof/support_export.json`, which
  is the `include_str!` source of truth verified by
  `checked_support_export_matches_builder`.

Every fixture validates against
`schemas/ui/m5-source-round-trip-honesty-primitive.schema.json` and against
`M5RoundTripHonestyPacket::validate()`. Each worked status case is self-consistent:
its stored resolution equals a fresh `resolve_round_trip_status(&input)`. Fixtures
carry only typed class tokens, opaque target / span refs, booleans, and redacted
labels — never raw source bodies, diff hunks, file contents, or credentials.
