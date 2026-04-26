# Workspace Entry Route Case Fixtures

Seed corpus for the route matrix in
[`/docs/ux/workspace_entry_route_matrix.md`](../../../docs/ux/workspace_entry_route_matrix.md)
and the schema at
[`/schemas/workspace/entry_route.schema.json`](../../../schemas/workspace/entry_route.schema.json).

Each file is one JSON `workspace_entry_route_case_record`. The cases
exercise distinct entry-route semantics, required preview disclosure,
surface parity, deep-link authority constraints, and failure recovery.

Every fixture:

- keeps route id, entry verb, source surface, and measurement route id
  distinct;
- previews target identity, trust/policy posture, destination writes,
  side effects, imported artifacts, restore class, and prerequisites
  before commit;
- records Start Center, main menu, command palette, deep-link resolver,
  and workspace-switcher parity;
- preserves a cancellation, rollback, previous-workspace, safe-open, or
  restricted-continuation escape path;
- carries no raw absolute paths, raw URLs with credentials, raw
  provider payloads, raw import payloads, raw policy bundles, or raw
  session bodies.

## Cases

| Fixture | Route | Main risk exercised |
| --- | --- | --- |
| [`open_folder_plain.json`](./open_folder_plain.json) | `workspace_entry.open_folder` | Plain local open keeps trust pending and previous workspace preserved until admission. |
| [`open_workspace_multi_root_boundary.json`](./open_workspace_multi_root_boundary.json) | `workspace_entry.open_workspace` | Workspace manifest previews root boundary and per-root trust before switch. |
| [`clone_repository_review_first.json`](./clone_repository_review_first.json) | `workspace_entry.clone_repository` | Clone materializes into staging and does not imply trust or setup execution. |
| [`import_portable_state_compare.json`](./import_portable_state_compare.json) | `workspace_entry.import` | Import compares portable state and retains rollback checkpoint before apply. |
| [`resume_snapshot_reauth_required.json`](./resume_snapshot_reauth_required.json) | `workspace_entry.resume_snapshot` | Managed snapshot resume requires authority re-evaluation and reauth. |
| [`restore_last_session_missing_remote.json`](./restore_last_session_missing_remote.json) | `workspace_entry.restore_last_session` | Restore names missing remote and never reruns execution sessions. |
| [`deep_link_managed_workspace_requires_review.json`](./deep_link_managed_workspace_requires_review.json) | `workspace_entry.deep_link` | Deep link into managed workspace cannot widen authority without review. |
| [`open_in_safe_mode_crash_loop.json`](./open_in_safe_mode_crash_loop.json) | `workspace_entry.open_in_safe_mode` | Recovery ladder enters safe mode with restricted fallback and extension disablement. |
| [`continue_in_restricted_mode_after_policy_narrow.json`](./continue_in_restricted_mode_after_policy_narrow.json) | `workspace_entry.continue_in_restricted_mode` | Policy-narrowed workspace keeps edit/search floor and previous workspace escape. |
