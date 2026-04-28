# Why-unavailable microcopy fixtures

Worked examples for
[`docs/ux/disabled_reason_grammar.md`](../../../docs/ux/disabled_reason_grammar.md)
and
[`artifacts/ux/disabled_reason_classes.yaml`](../../../artifacts/ux/disabled_reason_classes.yaml).

Each fixture carries one `why_unavailable_case_record`. The shape is
intentionally lightweight until a schema lands:

- `reason_class_id` names the canonical class.
- `command` pins the canonical command identity and action label.
- `explanation` carries the required fields from the grammar contract:
  current cause, safe next step, alternate route, freshness, docs/help
  anchor, template refs, and placeholders.
- `surface_projections` shows how palette, menu, status/banner,
  diagnostics, docs/help, support export, and accessibility narration
  preserve the same class and next-step semantics.

The examples use source-language copy only to exercise the grammar.
Runtime surfaces must resolve final strings through message ids and
typed placeholders.

## Cases

| File | Reason class | Purpose |
| --- | --- | --- |
| `policy_blocked_share_preview.yaml` | `policy_blocked` | External sharing blocked by policy, with local preview alternate. |
| `dependency_missing_rename_symbol.yaml` | `dependency_missing` | Rename requires a missing language pack, with text-search alternate. |
| `degraded_index_semantic_rename.yaml` | `degraded_index` | Semantic rename is unavailable while the index rebuilds. |
| `wrong_target_apply_refactor.yaml` | `wrong_target` | Refactor command needs a selected code range. |
| `read_only_snapshot_edit.yaml` | `read_only` | Recovered snapshot can be inspected but not edited. |
| `host_unavailable_remote_task.yaml` | `host_unavailable` | Remote task runner cannot reach its host, with local-only alternate. |
| `unsupported_mode_voice_command.yaml` | `unsupported_current_mode` | A mutating command is unavailable in companion voice mode. |
| `budget_exhausted_ai_apply.yaml` | `quota_or_budget_exhausted` | Managed AI apply is blocked by exhausted budget. |
| `preview_only_restore_checkpoint.yaml` | `preview_only` | Restore cannot apply until preview opens. |
| `stale_cached_context_repreview.yaml` | `stale_context` | Cached context drifted and requires refresh/re-preview. |
