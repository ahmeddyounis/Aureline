# Privacy-scoped collection persistence across the first real M5 surfaces

This document is the contract for binding filter ASTs, saved views, and column
presets to **durable, reopenable, privacy-scoped persistence records** across the
first real M5 dense collection surfaces: the pipeline run list, the review queue,
the incident list, the graph list, the marketplace results grid, and the
provider/admin table.

Where the dense-collection freeze
(`crates/aureline-collections/src/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/`)
classifies *what* each surface's filter, scope, count, and batch semantics are,
this lane makes that classification durable: it takes the frozen taxonomy and the
shared `aureline-search` collection objects and persists, per surface, the exact
filter/view/column state a user had active — so the state reopens through shared
objects rather than surface-local serialization, survives portability and
support/export, and fails visibly when it can no longer replay.

## Source of truth

- Packet type: `M5CollectionPersistencePacket`
  (`crates/aureline-collections/src/implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence/`).
- Boundary schema:
  `schemas/collections/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.schema.json`.
- Checked support export:
  `artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/support_export.json`.
- Markdown summary:
  `artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.md`.
- Protected fixtures:
  `fixtures/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/`.
- Conformance dump: `cargo run -p aureline-collections --example dump_m5_collection_persistence [support|summary]`.

## Each binding is a shared object, not a state blob

Every `PersistedCollectionState` binds one `DenseCollectionSurface` to a real
`aureline_search::SavedCollectionView`. The saved view carries the persisted
filter AST, visible and pinned columns, owner scope, privacy class, and fallback
behavior — the same shared object search, review, and admin grids already speak.
The binding adds the persistence dimensions this lane owns:

- a `persisted_schema_version` and a `PersistenceCompatibility`
  (`current`, `migratable_forward`, or `incompatible_needs_reset`);
- an `IncompatibilityResolution` (`migrate_forward`, `reset_to_default`, or
  `refuse_until_rebound`) and a precise, visible label, required whenever the
  state is not current;
- a `PersistedColumnPreset` whose required identity columns may never be dropped
  from the visible set;
- the full scope-counter vocabulary so visible / loaded / matching / selected
  never blur on reopen;
- explicit flags that the persisted state excludes transient selection, provider
  cursors, and secret-bearing material.

## Reopen is a first consumer

`PersistedCollectionState::reopen` reconstructs the reopen outcome:

| compatibility | resolution | outcome |
| --- | --- | --- |
| `current` | (none) | `restored_exact` |
| `migratable_forward` | `migrate_forward` | `restored_after_migration` |
| `migratable_forward` | `reset_to_default` | `reset_to_default` |
| `incompatible_needs_reset` | `reset_to_default` | `reset_to_default` |
| any incompatible | `refuse_until_rebound` | `refused_needs_rebind` |

An `incompatible_needs_reset` state may not claim a forward migration — it cannot
be migrated, so the validator rejects that combination. An incompatible state
never silently restores as if it were exact; it migrates, resets, or refuses, and
always carries a precise label.

`PersistedCollectionState::support_reconstruction` projects the same state into a
redaction-aware `CollectionStateReconstruction` — surface, saved-view id, scope
label, owner/privacy tokens, filter-AST id and clause count, visible and pinned
columns, compatibility, resolution, and reopen outcome — so diagnostics and
support packets can reconstruct which filter/view/column state the user had active
on a claimed M5 collection without any raw filter literal, cursor, or selection
identity.

## Privacy scope has teeth

- A shared, policy-pinned, or provider-owned view may not be persisted as
  `local_only_private`; a view that is shared or synced must be portable.
- Any portable view must carry a portable filter AST — no local-only literals —
  so persistence cannot smuggle local material into a shared or synced view.
- Transient selection and provider cursors are never persisted, on either the
  binding flags or the embedded saved-view object.
- The export-safe JSON carries only ids, tokens, labels, counts, and booleans;
  the validator refuses any export that carries raw boundary material.

## Acceptance criteria mapping

- **Claimed M5 dense surfaces reopen the same filter/view/column state through
  shared objects** — each binding embeds a shared `SavedCollectionView` plus a
  persisted column preset, and `reopen` returns `restored_exact` for current
  state.
- **Saved views preserve owner/privacy scope, hide transient selections and
  provider cursors, and fail visibly with migration or reset when incompatible**
  — enforced by `privacy_scope_consistent`, `excludes_transient_state`, and the
  compatibility/resolution invariants.
- **Diagnostics and support packets can reconstruct active state** —
  `support_reconstructions` projects every binding into a reconstruction record.

## Guardrails

- Do not let a row highlight stand in for durable selection: selection is never
  persisted; the durable state is the filter/view/column binding.
- Do not hide provider or policy narrowing inside a generic filter chip:
  narrowing source classes remain visible on the shared filter AST.
- Do not treat visible rows as all matching rows without an explicit step: the
  full scope-counter vocabulary is preserved on reopen.
- Do not let an incompatible state load silently: it fails visibly with a precise
  migration or reset label.
