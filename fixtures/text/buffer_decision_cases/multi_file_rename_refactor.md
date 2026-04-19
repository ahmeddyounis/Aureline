# Fixture: multi-file rename refactor with workspace-level undo group

## Scenario

The user invokes "Rename Symbol" on a public function `compute`
that is referenced in three files (its definition file plus two
call sites). The preview shows nine occurrences across the three
files. The user accepts. Later, the user makes an unrelated edit
in a fourth file, then presses `Cmd-Z` repeatedly.

## Hooks exercised

- `undo_group_open` — fires once per buffer the refactor touches.
- `checkpoint_create` — fires once before the refactor runs.
- `transaction_apply` — fires once per occurrence rewritten across
  the three buffers, each carrying its per-buffer `undo_group_id`
  and a shared `workspace_group_id`.
- `undo_group_close` — fires once per buffer when the workspace
  group commits.
- `undo_apply` — undo on any of the three buffers reverts the
  workspace group across all three buffers atomically.

## Undo classes emitted

- `refactor_multi_file`

## Stack elements stressed

- Workspace-level multi-file group referencing each buffer's
  per-buffer journal.
- Only-revertible posture: redo pins to the snapshot lineage and a
  divergent edit inside any member buffer drops the redo stack
  for the workspace group.
- Checkpoint taken before the refactor for restoration.

## Expected observable outcomes

- The journal records nine transactions with `class_id =
  refactor_multi_file` distributed across three per-buffer journals,
  all referencing one `workspace_group_id` and one
  `checkpoint_handle`.
- A single `Cmd-Z` from any of the three buffers reverts all nine
  occurrences across the three buffers; partial-buffer revert is a
  contract violation.
- After undo + an unrelated edit in the fourth file + `Cmd-Shift-Z`,
  redo re-applies the workspace group (the divergent edit was
  outside any member buffer).
- After undo + a divergent edit inside any member buffer +
  `Cmd-Shift-Z`, redo refuses to re-apply the workspace group; the
  redo stack for that group is dropped.
- Crash recovery from the checkpoint plus journal forward replay
  reaches the same post-refactor state without user intervention.

## ADR sections motivating this fixture

- Undo / redo and transaction grouping — single undo journal per
  buffer; workspace-level history references per-buffer journals.
- Undo-class taxonomy — `refactor_multi_file` row, including the
  `cross_buffer_redo_stack_loss` reopen trigger.
- Snapshot and checkpoint semantics — checkpoint at the start of
  any multi-file mutation.
