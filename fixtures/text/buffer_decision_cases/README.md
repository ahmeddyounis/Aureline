# Buffer decision-case fixtures

These fixtures are short, reviewable scenarios that anchor the
protected-hot-path hook names and the undo-class taxonomy defined in
[ADR 0003](../../../docs/adr/0003-buffer-undo-large-file.md) to
concrete inputs and observable outcomes. They are not a test suite;
they are the vocabulary the buffer prototype, the save pipeline, the
large-file controller, the recovery work, and the eventual
mutation-journal lane use when they instrument a hook or emit a
class.

**Scope rules**

- Every fixture names the hooks it exercises, the undo class(es) it
  emits, the stack element it stresses, and the observable outcome
  the prototype should capture.
- Fixtures never assert latency numbers; the benchmark lab owns
  budgets. Fixtures only describe *what* to measure, not *how fast*.
- Do not ship binary content larger than a few KiB inside this tree.
  If a fixture requires a multi-GiB file, name the construction
  recipe (e.g. "concatenate `/usr/share/dict/words` 1024 times")
  rather than checking the file in.
- A new fixture MUST hit at least one protected-hot-path hook or
  emit at least one undo class, and MUST cite the ADR section that
  motivates it.

**Index**

| Fixture                                                            | Primary hooks                                                                       | Undo classes emitted                                  | Stack element stressed                                 |
|--------------------------------------------------------------------|-------------------------------------------------------------------------------------|-------------------------------------------------------|--------------------------------------------------------|
| [`typing_word_coalesce.md`](./typing_word_coalesce.md)             | `text_edit_apply`, `transaction_apply`, `undo_apply`                                | `text_edit`                                           | Coalescing rules, journal writes                       |
| [`multi_cursor_single_keystroke.md`](./multi_cursor_single_keystroke.md) | `text_edit_apply`, `transaction_apply`, `undo_apply`, `redo_apply`              | `multi_cursor_text_edit`                              | Single-undo-group rule across N cursors                |
| [`single_file_rename_refactor.md`](./single_file_rename_refactor.md) | `undo_group_open`, `undo_group_close`, `transaction_apply`, `undo_apply`           | `refactor_single_file`                                | Named undo group, preview/apply path                   |
| [`multi_file_rename_refactor.md`](./multi_file_rename_refactor.md) | `undo_group_open`, `undo_group_close`, `checkpoint_create`, `transaction_apply`, `undo_apply` | `refactor_multi_file`                       | Workspace-level group, only-revertible posture         |
| [`format_on_save_with_participants.md`](./format_on_save_with_participants.md) | `save_pipeline_run`, `save_participant_rebase`, `transaction_apply`     | `save_participant_group`, `formatter_run`             | Save pipeline, staged participants, atomic rename      |
| [`ai_apply_multi_file_patch.md`](./ai_apply_multi_file_patch.md)   | `undo_group_open`, `undo_group_close`, `checkpoint_create`, `transaction_apply`     | `machine_generated_change`                            | Attribution, preview/apply, cross-buffer group         |
| [`external_modification_clean_and_dirty.md`](./external_modification_clean_and_dirty.md) | `external_change_detected`, `external_change_merge`, `transaction_apply`  | `external_reload`                                     | VFS handoff, diff/merge/choose                         |
| [`decode_failure_recovery.md`](./decode_failure_recovery.md)       | `decode_recovery_open`, `decode_recovery_resolve`, `transaction_apply`              | `decode_recovery_change`                              | Encoding detection, raw-bytes preservation             |
| [`bom_and_newline_preservation.md`](./bom_and_newline_preservation.md) | `save_pipeline_run`, `transaction_apply`                                       | `text_edit`, `save_participant_group`                 | BOM / dominant-newline / final-newline stickiness      |
| [`large_file_open_and_constrained_edit.md`](./large_file_open_and_constrained_edit.md) | `buffer_open`, `large_file_mode_enter`, `text_edit_apply`, `large_file_mode_exit` | `text_edit`                                | Large-file switch conditions, reduced-capability list  |
| [`crash_recovery_journal_replay.md`](./crash_recovery_journal_replay.md) | `checkpoint_create`, `journal_recovery_replay`, `transaction_apply`            | (replay of any class)                                 | Recovery journal forward replay                        |
