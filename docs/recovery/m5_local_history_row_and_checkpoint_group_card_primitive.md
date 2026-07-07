# M5 local-history-row and checkpoint-group-card primitive

Status: implemented (B105, task M05-893)

This is the first `implement_` lane that narrows the frozen
[M5 local-history / write-scope component matrix](./m5_local_history_write_scope_component_matrix.md)
into two reusable primitives: the **local-history row** and the
**checkpoint-group card**. It closes the gap between the deeper mutation-journal,
checkpoint, and generated-artifact systems and the reusable history components a
user actually reads before restoring or exporting.

Truth source (checked in):

- Schema: `schemas/ui/m5-local-history-row-and-checkpoint-group-card.schema.json`
- Support export: `artifacts/release/m5-local-history-row-and-checkpoint-group-card-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-local-history-row-and-checkpoint-group-card-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-local-history-row-and-checkpoint-group-card-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-local-history-row-and-checkpoint-group-card-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_history_local_history_row_checkpoint_group_card_primitive`; the in-code
seed builders, the checked support export, and the fixtures never drift.

## What the primitives implement

The matrix names the two families and freezes their controlled vocabulary
(snapshot origins, actor classes, capture fidelities, checkpoint lineage classes,
mutation classes, retention postures, export-redaction postures, surface families,
deployment lines, consumer surfaces, accessibility routes, qualification classes,
and downgrade triggers). This lane implements the two contracts as resolvers so a
user can tell, from the row or the card alone, mutation lineage before any restore
or export — never having to infer whether a snapshot came from typing, AI,
automation, import, repair, or crash recovery, and never confusing local history
with Git history.

### `resolve_local_history_row`

Takes one snapshot's origin, actor class, capture fidelity, mutation class,
retention posture, timestamp, object identity, branch/worktree identity,
command/trigger label, and source-removed signal. Derives the **row posture** in a
fixed blocking-first order:

1. `expired_unrestorable` — retention has expired and purged (cannot restore).
2. `metadata_only_reference` — a metadata-only capture with no restorable body.
3. `unattributed_snapshot` — an unknown / unattributed actor.
4. `purge_pending_snapshot` — history pending purge under retention policy.
5. `automated_capture` — an AI, automation, or import actor.
6. `restorable_snapshot` — an attributed, retained, full-body snapshot.

The row always offers **reveal-lineage** (actor + timestamp stay inspectable even
when the captured object is gone), offers **open** only when the object was not
removed, and offers **compare** / **restore** only when the snapshot can actually
restore — so a metadata-only or expired snapshot never reads as a full restorable
body, and the exact object identity is always preserved.

### `resolve_checkpoint_group_card`

Takes one grouped checkpoint's lineage class, mutation class, originating command,
file count, pre/post-risk note, export posture, managed-file signal, and
restore-path readiness. Derives the **card posture** in a fixed blocking-first
order:

1. `restore_blocked_group` — no restore path is available.
2. `high_risk_group` — a pre/post-risk note requires review before restore.
3. `generated_artifact_group` — the group touches generated or managed files.
4. `imported_group` — an imported checkpoint.
5. `multi_file_group` — a multi-file grouped transaction (file-count truth kept).
6. `atomic_checkpoint` — a restorable single-action checkpoint.

The card preserves the grouped moment as one attributable checkpoint, never
collapses its file-count truth, always offers **reveal-lineage**, offers
**preview-scope** for multi-file or managed-touching groups, and offers
**compare** / **restore** only when the restore path is available.

## Claimed recovery consumers

One matrix row per claimed M5 mutation/recovery surface, proving the same row and
card grammar renders consistently everywhere:

- **Editor recovery** — the editor local-history timeline.
- **Refactor history** — refactor-apply history and grouped refactor checkpoints.
- **AI apply review** — AI-apply captures and generated-artifact groups.
- **Importer actions** — external-import rows and imported checkpoints.
- **Support evidence** — the support / evidence export.

## Guardrails

This lane does not widen into new version-control semantics, new rollback engines,
or new AI mutation classes; it does not re-architect mutation-journal storage, Git
history, or repair-transaction engines. It hardens the shared local-history row and
checkpoint-group card layered on top of already-claimed M5 systems, with four hard
invariants enforced per row: never mask the snapshot actor or timestamp, never hide
a capture fidelity or generated-or-managed-file caveat, never invent a private
history grammar, and never bypass the restore-scope review.
