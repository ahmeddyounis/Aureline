# M5 mirror-transition primitive fixtures

Protected fixtures for the reusable **mirror-transition primitive** — the mirror/offline
artifact rows, mode-change / disconnect review sheet, and channel-association review row
that resolve from one transition context and share one transition identity (task
M05-831).

The primitive *narrows* the remaining three operational families of the frozen
[deployment/continuity component matrix](../../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
(`mirror_offline_artifact_row`, `mode_change_review_sheet`,
`channel_association_review_row`) into one working resolver:

- **AC1** — offline and mirror transitions never read like generic warnings: every
  artifact row names its source class, artifact class, verification state, and one shared
  continuity state (`Mirror unavailable`, `Offline cache only`, `Verification failed`,
  `Needs refresh`, …), and mirrored / cached content is never shown as current.
- **AC2** — artifact verification / manifests stay accessible from the same component
  family across deployment profiles: every row keeps a verify-signature and an
  open-manifest action reachable.
- **AC3** — mode changes preserve export-before-change and rollback truth: the sheet keeps
  a preserved-local-state ref, an export-before-change action, and a rollback path, and a
  channel association never silently captures a default handler.

## Files

- `support_export.json` — byte-identical copy of the canonical release proof at
  `artifacts/release/m5-mirror-transition-primitive-proof/support_export.json`.
- `matrix.csv` — one row per mirror surface family.

## Source of truth

Both files are emitted from the in-crate seeded builder
`seeded_m5_mirror_transition_packet()` in
`crates/aureline-install/src/implement_the_m5_mirror_offline_mode_change_and_channel_association_primitive/`.
Do not hand-edit; regenerate from the builder so the packet, the checked-in release
proof, and these fixtures stay byte-aligned. The boundary carries only opaque refs and
typed class tokens — never raw config bytes, credentials, or mirror URLs.
