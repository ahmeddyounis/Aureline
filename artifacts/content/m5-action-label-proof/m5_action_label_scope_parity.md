# Action-Label and Count/Scope-Language Parity Catalog

- Catalog: `m5-action-label-scope-catalog:stable:0001`
- Label: `Stable Action-Label and Count/Scope-Language Parity Catalog`
- Reference locale: `en`
- Verbs: 7 | Scopes: 7 | Objects: 8 | Labels: 10 | Disclosures: 5
- Banned ambiguous tokens: continue, accept, submit, ok, confirm, proceed, done, go, yes, next, finish
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Action labels

- `action.review.approve_selected_changes` [approval / selected / reviewed] on `review_sheet`
  - Label: Approve {count} selected changes
- `action.batch.approve_all_matching_changes` [batch_mutation / all_matching / unreviewed_batch] on `batch_action_bar`
  - Label: Approve all matching changes
- `action.batch.rerun_visible_tasks` [batch_mutation / visible / review_required] on `batch_action_bar`
  - Label: Rerun {count} visible tasks
- `action.batch.delete_selected_files` [destructive / selected / review_required] on `confirmation_dialog`
  - Label: Delete {count} selected files
- `action.export.export_loaded_results` [export / loaded / no_review_needed] on `export_report_heading`
  - Label: Export {count} loaded results
- `action.install.install_extension` [install / single_object / review_required] on `confirmation_dialog`
  - Label: Install extension
- `action.publish.publish_selected_documents` [publish / selected / review_required] on `confirmation_dialog`
  - Label: Publish {count} selected documents
- `action.review.approve_changes_in_sheet` [approval / selected / reviewed] on `review_sheet`
  - Label: Approve {count} changes
- `action.batch.apply_all_matching_fixes` [batch_mutation / all_matching / partially_reviewed] on `batch_action_bar`
  - Label: Apply all matching fixes
- `action.cli.export_all_matching_results` [export / all_matching / no_review_needed] on `cli_help_summary`
  - Label: Export all matching results

## Count/scope disclosures

- `disclosure.batch_bar.selected_with_policy_excluded` on `batch_action_bar`
  - Phrase: {acted_count} selected changes (exact); {hidden_count} hidden by policy, {outside_count} outside current workset not included.
- `disclosure.activity_row.reran_loaded_tasks` on `toast_activity_row`
  - Phrase: Rerun: {acted_count} loaded tasks (exact).
- `disclosure.export_heading.all_matching_with_status` on `export_report_heading`
  - Phrase: {total_count} all matching findings (approx.); {hidden_count} hidden by policy withheld.
- `disclosure.cli.loaded_vs_all_matching_results` on `cli_help_summary`
  - Phrase: {loaded_count} loaded of {matching_count} all matching results (partial).
- `disclosure.review_sheet.selected_outside_workset` on `review_sheet`
  - Phrase: {acted_count} selected changes (exact); {outside_count} outside current workset not included.

## Cross-surface scope reuse

- `all_matching`: batch_action_bar, cli_help_summary, export_report_heading
- `hidden_by_policy`: batch_action_bar, export_report_heading
- `loaded`: cli_help_summary, export_report_heading, toast_activity_row
- `outside_current_workset`: batch_action_bar, review_sheet
- `selected`: batch_action_bar, confirmation_dialog, review_sheet
- `single_object`: confirmation_dialog
- `visible`: batch_action_bar
