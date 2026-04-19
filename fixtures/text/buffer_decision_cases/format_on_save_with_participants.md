# Fixture: format-on-save with multiple participants and a mid-flight external change

## Scenario

The user has `format_on_save`, `organise_imports_on_save`, and an
`ai_quick_fix_on_save` participant enabled. The user edits a file,
then presses `Cmd-S`. While the save pipeline is running, an
external process (linter, generator) rewrites the same file on
disk. The save participant detects the on-disk change before the
atomic rename completes.

## Hooks exercised

- `save_pipeline_run` — fires once for the save attempt.
- `save_participant_rebase` — fires once when the participant
  detects the mid-flight on-disk change and either rebases against
  the new on-disk bytes or aborts the participant.
- `transaction_apply` — fires for every participant transaction
  that successfully commits, all sharing one save group id.
- `external_change_detected` — fires when the VFS reports the
  on-disk bytes diverged from the staged copy.

## Undo classes emitted

- `save_participant_group` (the outer save group)
- `formatter_run` (subsumed under the save group id)

## Stack elements stressed

- Atomic save pipeline ordering: encode → temp-write → fsync →
  atomic rename → optional parent-directory fsync.
- Staged-buffer rule: participants operate on a staged copy and
  rebase / abort if the on-disk file changes mid-flight.
- One save plus its participants is one undo group.

## Expected observable outcomes

- On a successful atomic save with no mid-flight change: one
  `save_pipeline_run` with `save_mode = atomic_rename`, a single
  save group containing the participant transactions, and a
  single `Cmd-Z` after save reverses the participant edits and
  the save back to the pre-participant state where possible.
- On a mid-flight external change: `save_participant_rebase`
  fires, the participant either rebases (re-runs against the new
  on-disk bytes inside the same save group) or aborts (rolls back
  the save group and surfaces a recoverable diff / merge / choose
  surface). Either outcome leaves the on-disk file consistent and
  the journal honest about what ran.
- On a host filesystem where atomic rename is unsafe (network
  mount, fuse mount, OneDrive placeholder), `save_pipeline_run`
  reports `save_mode = journal_mediated_fallback`; the
  `save_participant_group` row records the fallback so support
  bundles can explain why the path differed.

## ADR sections motivating this fixture

- Source-fidelity rules — atomic save pipeline and external
  modification handling.
- Undo-class taxonomy — `save_participant_group` and
  `formatter_run` rows, including the
  `rename_unsafe_filesystem_common` reopen trigger.
