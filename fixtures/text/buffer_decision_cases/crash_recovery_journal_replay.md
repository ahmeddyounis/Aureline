# Fixture: crash recovery replays the journal forward from the last checkpoint

## Scenario

The user opens a clean file, types four words across two
multicursor edits, runs a single-file rename refactor (named undo
group, seven occurrences), saves, and then types another sentence.
The host loses power before the user issues another save.

On next start, the buffer recovers state from the most recent
checkpoint plus the journal's forward replay.

## Hooks exercised

- `checkpoint_create` — fires at every automatic checkpoint
  (open, save, refactor start, mode transition).
- `transaction_apply` — fires for every committed transaction
  before the crash and for every replayed transaction during
  recovery.
- `journal_recovery_replay` — fires once at recovery, reporting
  the checkpoint id and the count of replayed transactions per
  class id.

## Undo classes emitted

- Replay reproduces the original class ids verbatim:
  `text_edit`, `multi_cursor_text_edit`, `refactor_single_file`,
  `save_participant_group`. No new class is emitted by recovery.

## Stack elements stressed

- Recovery journal: append-only on-disk record paired with each
  checkpoint.
- Checkpoint cadence: open, save, refactor start (because it
  emits a workspace-level group), mode transitions, and
  destructive migrations.
- Replay determinism: the journal forward replay produces a
  buffer byte-for-byte identical to the pre-crash state up to
  the last committed transaction.

## Expected observable outcomes

- Recovery replays from the post-save checkpoint forward, applying
  the post-save text edits in order. The recovered buffer matches
  the pre-crash state up to the last committed transaction; the
  user does not lose any keystroke that committed.
- `journal_recovery_replay` fires exactly once with the checkpoint
  handle and a per-class replay count.
- The undo stack after recovery includes every undo group from
  before the crash: `Cmd-Z` from the recovered state walks back
  through the post-save text edits, then the save group, then the
  refactor group, then the multicursor edits, then the original
  text edits — same as it would have without the crash.
- Recovery never silently discards user bytes; a failed replay
  surfaces a recoverable banner rather than erasing the journal.

## ADR sections motivating this fixture

- Snapshot and checkpoint semantics — checkpoints on save, mode
  transition, and multi-file mutation start; recovery journal
  forward replay.
- Performance and memory assumptions — recovery duration from
  the most recent checkpoint plus the journal forward to the
  last user keystroke.
