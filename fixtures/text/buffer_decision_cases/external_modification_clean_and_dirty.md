# Fixture: external modification of a clean buffer auto-reloads; dirty buffer routes to merge

## Scenario

Case A — clean buffer. The user has a file open with no unsaved
changes. An external process (formatter run from the terminal,
git checkout, generator) rewrites the file on disk.

Case B — dirty buffer. The user has unsaved local edits in the
same file. An external process rewrites the file on disk while
the buffer is dirty.

## Hooks exercised

- `external_change_detected` — fires once per VFS-reported on-disk
  change.
- `external_change_merge` — fires when the buffer adopts external
  bytes, distinguishing `auto_reload_clean` vs
  `merged_choice_dirty`.
- `transaction_apply` — fires when the resulting `external_reload`
  transaction commits.

## Undo classes emitted

- `external_reload` (in both cases — auto-reload for clean,
  merged-choice for dirty).

## Stack elements stressed

- VFS handoff: external-change detection routes through the VFS,
  not through the renderer or the editor command graph.
- Clean-vs-dirty branching: clean buffers MAY auto-reload by
  policy; dirty buffers route to an explicit diff / merge / choose
  flow and never silently overwrite either side.
- Per-buffer journal honesty: the new bytes appear as an
  `external_reload` transaction with `adopted_bytes_origin`
  recorded.

## Expected observable outcomes

- Case A (clean): `external_change_detected` fires, the workspace
  policy decides auto-reload, and the buffer commits an
  `external_reload` transaction with `adopted_bytes_origin =
  auto_reload_clean`. The dirty / clean state remains clean.
- Case B (dirty): `external_change_detected` fires; the buffer
  surfaces a diff / merge / choose UI and does not auto-adopt.
  When the user resolves the merge, `external_change_merge` fires
  and the journal records an `external_reload` transaction with
  `adopted_bytes_origin = merged_choice_dirty` and a non-null
  `merge_resolution_handle`.
- A dirty buffer that adopts external bytes without going through
  the merge flow triggers the `silent_overwrite_observed` reopen
  trigger and is rejected by the journal.
- Undo after the reload returns the buffer to its pre-reload
  snapshot (only-revertible posture).

## ADR sections motivating this fixture

- Source-fidelity rules — external modification handling.
- Undo-class taxonomy — `external_reload` row, including the
  `silent_overwrite_observed` reopen trigger.
