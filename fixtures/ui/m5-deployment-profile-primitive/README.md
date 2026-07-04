# M5 deployment-profile primitive fixtures

Protected fixtures for the reusable **deployment-profile primitive** — the
install-profile card, side-by-side import sheet, and rollout-ring row that resolve
from one deployment context and share one deployment identity (task M05-829).

The primitive *narrows* three families of the frozen
[deployment/continuity component matrix](../../../schemas/ui/m5-deployment-continuity-component-matrix.schema.json)
(`install_profile_card`, `side_by_side_import_sheet`, `rollout_ring_row`) into one
working resolver:

- **AC1** — install ownership and rollback target are never hidden: the card names
  which build / channel / install mode owns the running app and what rollback
  exists.
- **AC2** — a side-by-side handoff never depends on hidden state sharing: the import
  sheet names the shared-vs-isolated state model and preserves a rollback checkpoint
  before durable state moves across channels.
- **AC3** — managed rollout preserves ring identity and promotion evidence rather
  than flattening every install into one generic version list.

## Files

- `support_export.json` — byte-identical copy of the canonical release proof at
  `artifacts/release/m5-deployment-profile-primitive-proof/support_export.json`.
- `matrix.csv` — one row per deployment surface family.

## Source of truth

Both files are emitted from the in-crate seeded builder
`seeded_m5_deployment_profile_packet()` in
`crates/aureline-install/src/implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive/`.
Do not hand-edit; regenerate from the builder so the packet, the checked-in release
proof, and these fixtures stay byte-aligned. The boundary carries only opaque refs
and typed class tokens — never raw config bytes, credentials, or mirror URLs.
