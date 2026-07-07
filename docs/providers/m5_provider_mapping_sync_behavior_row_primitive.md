# M5 provider mapping / sync-behavior row primitive

This contract ships two reusable M5 provider primitives — the **project/board mapping row**
and the **sync-behavior row** — so the destination and publication mode for provider-backed
actions stay explicit. It narrows two families from the frozen
[provider-account / offline-capture component matrix](../../schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json)
into two resolvers, and is implemented under
`crates/aureline-provider/src/ship_project_or_board_mapping_rows_and_sync_behavior_rows_with_inherited_local_policy_scope_read_only_comment_transition_sync_modes_and_change_reset_parity_across_claimed_m5_provider_lanes/`.

- Boundary schema: [`schemas/ui/m5-provider-mapping-sync-behavior-row.schema.json`](../../schemas/ui/m5-provider-mapping-sync-behavior-row.schema.json)
- Support export: `artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-provider-mapping-sync-behavior-row-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-provider-mapping-sync-behavior-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-provider-mapping-sync-behavior-row-primitive/`

## Why

M5 cannot honestly claim provider-backed team-workflow continuity if a user still has to
guess **which** provider project or board a lookup or write will target, or infer that a
single ambiguous `synced` label covers materially different write and mirroring behaviors.
These two rows close that gap on top of the already-claimed M5 provider workflows.

## Project/board mapping row

`resolve_project_board_mapping_row` takes one mapping's provider project/space label, its
repo/workspace relation, its target kind, its mapping origin, and an optional lock note, and
derives:

- **Mapping scope** — the six frozen origins grouped into `inherited_scope` (inherited
  default, auto-match, imported config), `local_scope` (the user's explicit choice),
  `policy_scope` (an admin policy pin), or `unmapped_scope`.
- **Row posture** — one-to-one from the frozen mapping origin, so the six origins never
  collapse into one generic "mapped" chip.
- **Explicit destination** — true when the row points at a real target, false when it flags
  itself unmapped. The resolver never resolves an unmapped row to a silent default
  (`assumes_default_destination_silently` is always `false`).
- **Change / reset actions** — reveal and export are always offered; change is offered unless
  the mapping is policy-locked; reset back to the inherited default is offered when the
  current mapping is a user override, an auto-match, or an imported config. A policy-pinned
  mapping is locked and must carry its lock note (`MissingLockNoteForPolicyLock` otherwise).

## Sync-behavior row

`resolve_sync_behavior_row` takes one row's provider sync mode, its effective write scope, and
its queued-draft state, and derives:

- **Sync-behavior class** — from the frozen sync mode and write scope: `offline_capture_only`,
  `sync_paused`, `read_only_metadata`, and — for live / manual / scheduled modes —
  `full_bidirectional_sync`, `comment_link_sync`, or `status_transition_sync` by write scope.
  This is the acceptance-criterion separation that replaces the one ambiguous `synced` label
  (`collapses_into_generic_synced` is always `false`).
- **Local-draft queue** — the queued-draft state is always visible
  (`hides_local_draft_queue_state` is always `false`); `has_pending_local_work` is true for a
  pending, queued, blocked, or failed draft.
- **Actions** — reveal, change-mode, and export are always offered; view-queue when local work
  is pending; retry-publish when a prior publish failed.

## Consumers and parity

One matrix binds five claimed provider surface consumers — the mapping-picker panel, the
sync-behavior panel, the provider status bar, the headless/CLI mappings surface, and the
support mapping export — to the same mapping/sync vocabulary, anatomy, export fields, and
non-visual accessibility routes, so the destination and publication-mode grammar stays
identical across desktop, headless/export, and support consumers. Two checked-in narrowed
fixtures hold the sync-behavior panel at Preview and the headless/CLI mappings surface at Beta
while keeping every consumer visible.

Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the export
boundary; every project label, relation, and mapping/sync identity is carried only as an
opaque, export-safe representation.
