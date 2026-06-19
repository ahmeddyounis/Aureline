# M5 Quality-Session Ledger

- Packet: `m5-quality-session-ledger:stable:0001`
- Label: `M5 Quality-Session Ledger`
- Workspace: `workspace:m5:quality-actions`
- Minted: `2026-06-19T00:00:00Z`
- Sessions: 8
- Trigger paths covered: 6
- Action classes covered: 10

| Session | Trigger | Outcome | Proposals | Mutating | Preview-first | Apply-blocked |
| --- | --- | --- | --- | --- | --- | --- |
| `session:m5:on-type:notebook:0001` | on_type | applied | 1 | 1 | false | false |
| `session:m5:on-save:framework:0001` | on_save | preview_required | 2 | 2 | true | false |
| `session:m5:manual:request-data:0001` | manual_command | preview_required | 2 | 2 | true | false |
| `session:m5:headless:package:0001` | cli_headless | applied | 1 | 1 | false | false |
| `session:m5:review:governance:0001` | review | blocked_by_policy | 2 | 2 | true | true |
| `session:m5:import-comparison:scanner:0001` | import_comparison | applied | 2 | 0 | false | false |
| `session:m5:generated-protected:0001` | manual_command | preview_required | 1 | 1 | true | false |
| `session:m5:unknown-unstable:0001` | manual_command | failed | 1 | 1 | true | true |

- `session:m5:on-type:notebook:0001` — On-type formatting auto-applied a trivia-safe range edit in a notebook cell. (on_type)
  - format_range / trivia_safe / auto_apply_allowed → preview: not_required, rollback: current_buffer_undo
- `session:m5:on-save:framework:0001` — On-save formatting and organize-imports require a preview before apply. (on_save)
  - format_document / local_syntax_safe / preview_before_apply → preview: batch_scope_preview, rollback: grouped_workspace_checkpoint
  - organize_imports / semantic_local / preview_before_apply → preview: structured_diff, rollback: single_file_checkpoint
- `session:m5:manual:request-data:0001` — Manual quick-fix and fix-all-rule require preview before apply. (manual_command)
  - quick_fix_single / semantic_local / preview_before_apply → preview: structured_diff, rollback: single_file_checkpoint
  - fix_all_rule / cross_file_semantic / preview_before_apply → preview: batch_scope_preview, rollback: grouped_workspace_checkpoint
- `session:m5:headless:package:0001` — Headless lint autofix auto-applied a localized, syntax-safe batch. (cli_headless)
  - lint_autofix_batch / local_syntax_safe / auto_apply_allowed → preview: not_required, rollback: single_file_checkpoint
- `session:m5:review:governance:0001` — Review-apply suppression and baseline updates are blocked pending policy or trust. (review)
  - suppression_proposal / semantic_local / blocked_pending_policy_or_trust → preview: policy_or_repo_mutation_preview_required, rollback: policy_audit_only
  - baseline_update / semantic_local / blocked_pending_policy_or_trust → preview: policy_or_repo_mutation_preview_required, rollback: policy_audit_only
- `session:m5:import-comparison:scanner:0001` — Imported-scan comparison stayed read-only and never read as a local apply. (import_comparison)
  - scanner_read_only / local_syntax_safe / read_only_action → preview: not_required, rollback: no_mutation
  - validation_recheck / local_syntax_safe / read_only_action → preview: not_required, rollback: no_mutation
- `session:m5:generated-protected:0001` — A generated, lockfile, and manifest family reused the preview-first lifecycle. (manual_command)
  - format_document / generated_or_protected / preview_before_apply → preview: batch_scope_preview, rollback: grouped_workspace_checkpoint
- `session:m5:unknown-unstable:0001` — An unknown or unstable fix was blocked pending user review, not silently applied. (manual_command)
  - quick_fix_single / unknown_or_unstable / blocked_pending_user_review → preview: issue_link_or_typed_review_required, rollback: manual_recovery_required
