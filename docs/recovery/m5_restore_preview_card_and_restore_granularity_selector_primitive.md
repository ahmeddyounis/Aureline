# M5 restore-preview-card and restore-granularity-selector primitive

Status: implemented (B105, task M05-894)

This is the second `implement_` lane that narrows the frozen
[M5 local-history / write-scope component matrix](./m5_local_history_write_scope_component_matrix.md)
into two reusable primitives: the **restore-preview card** and the
**restore-granularity selector**. It builds on the
[local-history-row and checkpoint-group-card primitive](./m5_local_history_row_and_checkpoint_group_card_primitive.md)
and makes rollback, compare, and recovery diff-first and lineage-preserving — a
restore is a new attributable checkpoint, never an invisible rewrite of local
history.

Truth source (checked in):

- Schema: `schemas/ui/m5-restore-preview-card-and-restore-granularity-selector.schema.json`
- Support export: `artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-restore-preview-card-and-restore-granularity-selector-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-restore-preview-card-and-restore-granularity-selector-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-restore-preview-card-and-restore-granularity-selector-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_history_restore_preview_card_restore_granularity_selector_primitive`; the
in-code seed builders, the checked support export, and the fixtures never drift.

## What the primitives implement

The matrix names the two families and freezes their controlled vocabulary (restore
granularities, restore drift states, managed-file caveats, retention postures,
export-redaction postures, restore-selection modes, capture fidelities, mutation
classes, surface families, deployment lines, consumer surfaces, accessibility
routes, qualification classes, and downgrade triggers). This lane implements the two
contracts as resolvers so a user can tell, from the card or the selector alone, what
past state a restore compares against, which exact file or object it touches, how the
target drifted, whether it reaches a generated or managed file, which restore
granularity is on offer, and that the restore records a new attributable checkpoint.

### `resolve_restore_preview_card`

Takes one restore's past-state and current-state labels, object identity, mutation
class, capture fidelity, drift state, managed-file caveat, offered restore
granularity, retention posture, export posture, selection-valid signal, and
restore-path readiness. Derives the **preview posture** in a fixed blocking-first
order:

1. `restore_blocked_preview` — the restore path is unavailable (cannot restore).
2. `conflict_preview` — a pending conflict must resolve before any restore.
3. `external_drift_preview` — the baseline diverged externally, moved, or was deleted.
4. `managed_file_preview` — the restore reaches a generated or managed file.
5. `local_drift_preview` — the restore would land over unsaved local edits.
6. `clean_restore_preview` — a clean apply onto an unchanged target.

The card always offers **inspect-diff** (the past-versus-current comparison is
always reachable), offers **resolve-conflict** only for a pending conflict, offers
**restore-whole-file** only when the restore can commit, and offers
**restore-selected-range** only when a selected range is valid and the granularity
is partial. Every restore `creates_new_checkpoint` and `preserves_history_trail`, so
the existing history trail is never erased and the exact object identity is always
preserved.

### `resolve_restore_granularity_selector`

Takes one restore's drift state, its multi-file and selectable-range signals, its
generated-or-managed signal, and its restore-path readiness. Derives the **selector
posture** in a fixed blocking-first order:

1. `selector_blocked` — the restore path is unavailable (dry-run only).
2. `dry_run_only_selector` — a pending conflict blocks the apply (dry-run only).
3. `exclude_generated_selector` — a generated/managed target defaults to excluding generated files.
4. `range_scoped_selector` — a valid selectable range (choose hunks / symbols).
5. `file_scoped_selector` — a multi-file restore (choose files).
6. `whole_scope_selector` — a single-file apply-all.

A dry-run **inspect-scope** action is always available, scope narrowing stays a
first-class choice (**narrow-to-files**, **narrow-to-range**, **exclude-generated**),
and an apply always records a new attributable checkpoint — a restore is never
collapsed into an all-or-nothing, history-erasing apply.

## Claimed mutation / recovery consumers

One matrix row per claimed M5 mutation/recovery surface, proving the same preview and
selector grammar renders consistently everywhere:

- **Editor restore** — the editor local-history restore surface.
- **AI apply restore** — AI-apply revert and generated-artifact restores.
- **Import restore** — imported restores that may land over local edits.
- **Repair restore** — repair-transaction restores behind pending conflicts.
- **Recovery center** — restore-blocked recovery with dry-run-only selectors.

## Guardrails

This lane does not widen into new version-control semantics, new rollback engines,
or new AI mutation classes; it does not re-architect mutation-journal storage, Git
history, or repair-transaction engines. It hardens the shared restore-preview card
and restore-granularity selector layered on top of already-claimed M5 systems, with
four hard invariants enforced per row: never mask the past or current state, never
hide the drift baseline or a generated-or-managed-file caveat, never collapse a
partial restore into a whole-snapshot restore, and never erase the existing history
trail.
