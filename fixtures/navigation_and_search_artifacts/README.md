# Navigation and search-artifacts worked fixtures

Worked fixtures for the navigation-continuity, bookmark-drift, and
saved-query / deep-link contract frozen in
[`docs/navigation/navigation_and_saved_query_contract.md`](../../docs/navigation/navigation_and_saved_query_contract.md).

Each `.json` file in this directory is one record validated against
one of:

- [`schemas/navigation/navigation_artifacts.schema.json`](../../schemas/navigation/navigation_artifacts.schema.json)
- [`schemas/search/saved_query_bundle.schema.json`](../../schemas/search/saved_query_bundle.schema.json)

The contract's three acceptance bullets (one back / forward and
recent-location model across surfaces; deep links reopen with
current permissions; sensitive search material is local-only or
hashed by default) are exercised across the cases below.

## Navigation-artifact fixtures

| File                                                              | Record kind                                | Exercised contract clauses                                                                                                  |
|-------------------------------------------------------------------|--------------------------------------------|------------------------------------------------------------------------------------------------------------------------------|
| `breadcrumb_trail_editor_to_leaf_symbol.json`                     | `navigation_breadcrumb_trail_record`       | Breadcrumb starts at workspace root; ordered, contiguous segments down to a leaf symbol.                                     |
| `outline_snapshot_editor_file.json`                               | `navigation_outline_snapshot_record`       | File outline rooted at `file_outline_root`; container + leaf symbol nodes referenced by parent id.                            |
| `bookmark_bound_editor_symbol.json`                               | `navigation_bookmark_record`               | `bound` drift state with `no_user_action_required_bound_or_remapped` user action.                                            |
| `bookmark_remapped_with_chain.json`                               | `navigation_bookmark_record`               | `remapped` state; non-empty `remap_chain_target_id_refs` with prior target ids.                                              |
| `bookmark_drifted_user_must_pick_successor.json`                  | `navigation_bookmark_record`               | `drifted` state; required user action `user_must_pick_successor_drifted`.                                                    |
| `bookmark_missing_target_with_denial.json`                        | `navigation_bookmark_record`               | `missing_target` state; required user action `user_must_re_anchor_or_delete_missing_target`.                                 |
| `bookmark_scope_unavailable_remote_shard.json`                    | `navigation_bookmark_record`               | `scope_unavailable` state for a review diff hunk on an unreachable remote shard.                                              |
| `bookmark_archived_tombstone.json`                                | `navigation_bookmark_record`               | `archived` state with `archived_at` set.                                                                                      |
| `navigation_history_entry_search_result_open.json`                | `navigation_history_entry_record`          | `search_result_open` origin with `originating_search_session_id_ref` and `originating_command_id_ref`.                       |
| `navigation_history_entry_back_step.json`                         | `navigation_history_entry_record`          | `back_or_forward_step` origin; `previous_history_entry_id_ref` set.                                                          |
| `navigation_history_entry_session_restore_replay.json`            | `navigation_history_entry_record`          | `session_restore_replay` origin replaying durable history; new build identity.                                               |
| `peek_context_definition_promoted_to_jump.json`                   | `navigation_peek_context_record`           | Definition peek opened on a history entry and later promoted to a jump.                                                       |
| `navigation_artifact_audit_event_drift_denial.json`               | `navigation_artifact_audit_event_record`   | Denial emitted when a downstream surface attempted to render a missing-target bookmark as bound.                              |

## Saved-query bundle and search-collection-snapshot fixtures

| File                                                              | Record kind                                  | Exercised contract clauses                                                                                                                                  |
|-------------------------------------------------------------------|----------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `saved_query_bundle_user_authored_local.json`                     | `saved_query_bundle_record`                  | `user_authored_local` source resolves to `local_only_no_share` and `local_by_default_no_remote_retention` (gated by schema).                                 |
| `saved_query_bundle_org_shared_admin_curated.json`                | `saved_query_bundle_record`                  | `org_shared_admin_curated` source backed by an admin-consented provider index.                                                                                |
| `saved_query_bundle_support_export_redacted.json`                 | `saved_query_bundle_record`                  | `support_export_captured` source resolves to `support_export_redacted_only` plus `literal_hashed_with_local_salt` (gated by schema).                         |
| `saved_query_bundle_managed_provider_read_only.json`              | `saved_query_bundle_record`                  | `managed_workspace_provisioned` source resolves to `managed_admin_published_read_only_no_export` plus `managed_provider_retention_locked` (gated by schema). |
| `search_collection_snapshot_reopen_exact.json`                    | `search_collection_snapshot_record`          | Reopen-honesty `current_scope_matches_captured_scope_exact`; minimum export fields cited.                                                                     |
| `search_collection_snapshot_reopen_scope_widened.json`            | `search_collection_snapshot_record`          | Reopen-honesty `current_scope_widened_versus_captured_scope_disclosed`; captured rows are replay material.                                                    |
| `search_collection_snapshot_reopen_build_drifted.json`            | `search_collection_snapshot_record`          | Reopen-honesty `current_build_drifted_versus_captured_build_disclosed`; offers re-run on user request.                                                        |
| `search_collection_snapshot_reopen_policy_drifted.json`           | `search_collection_snapshot_record`          | Reopen-honesty `current_policy_epoch_drifted_versus_captured_disclosed`; provider attribution chip retained.                                                  |
| `saved_query_bundle_audit_event_reopen_refused.json`              | `saved_query_bundle_audit_event_record`      | Denial `reopen_must_disclose_current_versus_captured_drift` when reopen attempted to paint stale captured rows as current truth.                              |
| `saved_query_bundle_audit_event_literal_export_refused.json`      | `saved_query_bundle_audit_event_record`      | Denial `sensitive_literal_export_requires_explicit_opt_in` when an export tried to disclose a literal without explicit user opt-in.                          |

## Redaction posture

Every fixture carries only opaque ids (workspace handles, file
handles, symbol handles, planner-pass / result-set / fused-row /
session ids, scope-binding ids, deep-link binding ids, saved-query
ids, bundle ids, snapshot ids, policy-epoch handles, build-identity
handles), monotonic placeholder timestamps, and redaction-aware
reviewable labels. Raw absolute paths, raw symbol bodies, raw
document bodies, raw notebook cell text, raw terminal bytes, raw
URLs, and raw query bodies do not appear in any fixture.
