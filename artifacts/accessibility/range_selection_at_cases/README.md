# Range-selection assistive-tech cases

Reviewer-facing accessibility cases paired with
[`/docs/verification/focus_and_batch_scope_packet.md`](../../../docs/verification/focus_and_batch_scope_packet.md),
the dense-review acceptance-pack family, and the dense-collection task
corpus rows already seeded in the repository.

These files complement the broader fixture set under
[`/fixtures/accessibility/ime_and_text_cases/`](../../../fixtures/accessibility/ime_and_text_cases/).
They focus specifically on dense-collection keyboard and screen-reader
flows that must stay concrete enough to gate later collection regressions.

Coverage in this seed:

- `current_row_selection.yaml`
- `range_extension_across_virtualized_rows.yaml`
- `hidden_selected_count_inspection.yaml`
- `batch_review_keyboard_only_open.yaml`

Rules:

1. Every case cites the dense-review acceptance-pack, checklist, task,
   and tree-coverage rows.
2. Every case names the keyboard route, expected focus owner, and the
   announcement or text result that proves the path stayed accessible.
3. `passed`, `degraded`, and `failed` mean the same thing here as in the
   accessibility review packet template.
