# Refactor and replace transaction worked fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/editor/refactor_and_replace_transaction_contract.md`](../../../docs/editor/refactor_and_replace_transaction_contract.md)
and the boundary schemas at
[`/schemas/editor/refactor_transaction.schema.json`](../../../schemas/editor/refactor_transaction.schema.json),
[`/schemas/editor/refactor_preview.schema.json`](../../../schemas/editor/refactor_preview.schema.json),
and
[`/schemas/editor/refactor_outcome.schema.json`](../../../schemas/editor/refactor_outcome.schema.json).

Each fixture is a single record of one of these shapes:

- `refactor_transaction_record`
- `refactor_preview_record`
- `refactor_outcome_record`

The corpus keeps only opaque transaction / preview / outcome /
checkpoint / approval-ticket / review-packet / provider / epoch /
policy / execution-context / patch / symbol / path-set / member-set
handles plus typed vocabulary and export-safe summaries. No fixture
carries raw match text, raw replacement text, raw diff bodies, raw
patch hunks, raw paths, raw symbol bodies, raw provider logs, raw
command lines, raw response bodies, or raw secret material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `workspace_replace_excluded_scopes.yaml` | `refactor_preview_record` | Workspace replace whose preview names user-filter, generated-pair, protected, and dirty-buffer exclusions; preview completeness is `preview_partial_due_to_excluded_paths` and a workspace-scope checkpoint is bound. |
| `partial_rename_stale_graph.yaml` | `refactor_transaction_record` | Rename-symbol transaction blocked-pending-user-review because the language server only proved the active-workset slice; preview requirement is `rename_impact_preview_required`, apply posture is `blocked_pending_user_review`. |
| `generated_file_blocked_refactor.yaml` | `refactor_outcome_record` | Refactor / move / extract that ended `blocked_no_changes_made` because the generator-paired and protected client-handoff paths would have been mutated; reversal class is `manual_review_required_no_automatic_path`. |
| `imported_multi_file_patch_with_checkpoint.yaml` | `refactor_transaction_record` | Imported multi-file SARIF auto-fix patch with `imported_patch_ref` bound on both descriptors, a captured pre-apply checkpoint, an approval-ticket linkage, and replay-hint disclosure. |

## Cross-walk to the contract

- `workspace_replace_excluded_scopes.yaml` covers section 3 (preview
  packet) — match counts, exclusion rows, file-state warnings,
  affected counts, index freshness, and checkpoint posture.
- `partial_rename_stale_graph.yaml` covers section 4 (transaction
  record) — semantic-layer state floor for rename, blocking reason
  classes, preview-requirement gates, and the validation plan.
- `generated_file_blocked_refactor.yaml` covers section 5 (outcome
  record) — `blocked_no_changes_made` final state, per-member
  skipped-or-blocked rows for generated and protected paths, and the
  manual-review reversal class.
- `imported_multi_file_patch_with_checkpoint.yaml` covers section
  1.5 (`imported_or_generated_patch_apply`) and section 3.6
  (checkpoint and rollback posture) — `imported_patch_ref`
  attribution on both descriptors, imported-patch atomicity
  inheritance, and replay via `rebuild_imported_patch_from_source`.
