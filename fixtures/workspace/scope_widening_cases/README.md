# Scope-diff review and cross-repo jump-event fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/workspace/scope_widening_and_cross_repo_jump_contract.md`](../../../docs/workspace/scope_widening_and_cross_repo_jump_contract.md)
and are validated by the schemas at:

- [`/schemas/workspace/scope_diff_review.schema.json`](../../../schemas/workspace/scope_diff_review.schema.json)
- [`/schemas/workspace/cross_repo_jump_event.schema.json`](../../../schemas/workspace/cross_repo_jump_event.schema.json)

Each fixture names the record kind it exercises, the trigger /
jump-origin / continuity classes it covers, and the worked sections
of the scope-widening and cross-repo jump contract it motivates.

**Scope rules**

- Fixtures validate against the schemas above; they do not encode
  wire bytes or runtime execution-context envelopes.
- A new fixture MUST exercise at least one trigger class, review
  state, expected-cost class, source-note class, support-impact
  field, jump-origin class, jump kind, workset-continuity class, or
  back-target kind, and MUST cite the section of the contract that
  motivates it.
- Monotonic timestamps and stable ids are opaque; they read well
  rather than reflecting any real clock.
- Filesystem-root refs are opaque pointers to ADR-0006
  filesystem-identity records. Raw absolute paths, raw remote URLs,
  raw policy bodies, raw query bodies, raw document bodies, and raw
  symbol definitions never appear.
- Workset ids are quoted by reference; no fixture mints a parallel
  stable scope id.

**Index**

| Fixture | Record kind | Key classes exercised | Doc section |
|---|---|---|---|
| [`widen_current_repo_to_selected_workset.yaml`](./widen_current_repo_to_selected_workset.yaml) | `scope_diff_review_record` | `search_widen_with_review` ; `pending_user_review` ; `targeted_index` / `moderate` ; `workspace_local_cache` / `profile_local_cache` / `remote_authoritative_optional` ; `changes_readiness = true` ; `confirm_widen` / `cancel_widen_keep_current_scope` / `remember_scope_choice` | §1, §1.1, §1.4, §1.5, §1.6, §1.7 |
| [`widen_selected_workset_to_full_workspace.yaml`](./widen_selected_workset_to_full_workspace.yaml) | `scope_diff_review_record` | `refactor_widen` ; `in_review` ; `full_reindex` / `very_expensive_remote_only` ; `remote_authoritative_required` / `imported_provider_index` ; `includes_managed_provider_refs_after_widen` / `reduces_support_export_completeness` / `reduces_offline_capability` ; multi-root widen (4 roots added) | §1, §1.5, §1.6, §1.7, §6 |
| [`cross_repo_peek_outside_scope.yaml`](./cross_repo_peek_outside_scope.yaml) | `cross_repo_jump_event_record` | `search_result_row` ; `peek_inline` ; `peeked_outside_scope` ; `outside_current_scope` destination marker ; `back_to_search_result` / `back_closes_peek` ; `preserves_source_workset_ref` / `preserves_source_query_session_ref` / `preserves_source_anchor_ref` all true | §2, §2.1, §2.3, §2.4, §2.5, §3 |
| [`blocked_widening_by_trust_or_policy.yaml`](./blocked_widening_by_trust_or_policy.yaml) | `scope_diff_review_record` | `navigation_deep_link_widen` ; `blocked_by_policy` ; `blocked_by_active_trust_stage` ; `blocked_by_admin_policy` ; `external_provider_index` ; `request_trust_review` / `request_policy_review_admin_only` ; `confirm_widen` forbidden | §1, §1.3, §1.8, §1.9, §6 |

**Coverage contract**

This fixture set MUST cover, at minimum: a widening from current
repo to a selected workset (review record), a widening from a
selected workset to the full workspace (review record), a cross-
repo peek that does not switch the active workset (jump-event
record), and a widening blocked by trust or policy (review record).
Adding fixtures that exercise additional trigger classes, jump
origins, continuity classes, or back-target kinds is welcome;
removing a class this directory already covers is a breaking
change.
