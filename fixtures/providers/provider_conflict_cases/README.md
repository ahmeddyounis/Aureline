# Provider conflict review, compare/reconcile flow, and last-writer-wins prohibition fixtures

Worked cases for the contract frozen in
[`/docs/providers/provider_conflict_review_contract.md`](../../../docs/providers/provider_conflict_review_contract.md).

Each fixture is a self-contained YAML document bundling the
records a single scenario would emit. Every record is schema-valid
against the boundary schema:

- [`/schemas/providers/provider_conflict_review.schema.json`](../../../schemas/providers/provider_conflict_review.schema.json)
  (`provider_conflict_review_record`).

The owning `connected_provider_record` (the registry-row anchor
every fixture references through `connected_provider_record_id`)
is frozen in
[`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
and is exercised in
[`/fixtures/providers/connected_account_cases/`](../connected_account_cases/);
the fixtures here intentionally leave that record out of the bundle
so each case is focused on the conflict-review shape under test.
The paired `deferred_publish_queue_item_record` and
`publish_later_queue_item_record` are the cross-tool surfaces
every non-owning consumer reads; each fixture cites them through
`originating_queue_item_ref` so support exports and admin
reconciliation consoles can pivot.

The `__fixture__` header on every file names the scenario, the
record kinds emitted, and the closed-vocabulary members the case
exercises. The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                                                          | Surface class                  | Review state                              | Disposition                              | Stale source / drift                    |
|----------------------------------------------------------------------------------------|--------------------------------|-------------------------------------------|------------------------------------------|-----------------------------------------|
| `code_host_pr_text_conflict_rebase_local_on_remote.yaml`                               | `code_host_surface`            | `resolved_with_rebase`                    | `rebase_local_on_remote`                 | `remote_edit_since_import`              |
| `issue_review_metadata_conflict_edit_local_then_review_again.yaml`                     | `issue_or_planning_surface`    | `resolved_with_edit_local`                | `edit_local_then_review_again`           | `remote_edit_since_import`              |
| `docs_page_target_renamed_fork_into_new_target.yaml`                                   | `docs_or_portal_surface`       | `resolved_with_fork_into_new_target`      | `fork_into_new_target`                   | `target_identity_drift`                 |
| `ci_check_run_permission_loss_blocked_escalate.yaml`                                   | `ci_or_checks_surface`         | `blocked_by_permission_loss`              | `escalate_for_admin_review`              | `changed_permissions`                   |
| `release_artifact_attachment_conflict_accept_remote.yaml`                              | `release_publisher_surface`    | `resolved_with_accept_remote`             | `accept_remote_as_authoritative`         | `stale_import_snapshot`                 |
| `managed_admin_unknown_actor_blocked_escalate.yaml`                                    | `managed_admin_surface`        | `blocked_by_actor_unknown`                | `escalate_for_admin_review`              | `actor_or_scope_change`                 |
| `ai_provider_transition_conflict_pending_user_review.yaml`                             | `ai_provider_surface`          | `pending_user_review`                     | `pending`                                | `remote_edit_since_import`              |
| `identity_provider_blocked_by_policy_review_pending.yaml`                              | `identity_provider_surface`    | `blocked_by_policy`                       | `pending`                                | `changed_permissions`                   |
| `artifact_registry_text_conflict_discard_local_keep_remote.yaml`                       | `artifact_registry_surface`    | `resolved_with_discard_local`             | `discard_local_keep_remote`              | `remote_edit_since_import`              |

Vocabulary coverage across the bundle:

- `surface_class`: `code_host_surface`,
  `issue_or_planning_surface`, `docs_or_portal_surface`,
  `ci_or_checks_surface`, `release_publisher_surface`,
  `managed_admin_surface`, `ai_provider_surface`,
  `identity_provider_surface`, `artifact_registry_surface` (all
  nine surfaces exercised).
- `reviewer_actor_class`: `human_account`, `admin_reviewer`,
  `unknown_actor_class` (the remaining classes —
  `installation_or_app_grant`, `delegated_user_token`,
  `project_scoped_grant`, `policy_injected_service_identity` —
  are bound by the same schema rules).
- `review_state_class`: `resolved_with_rebase`,
  `resolved_with_edit_local`,
  `resolved_with_fork_into_new_target`,
  `blocked_by_permission_loss`, `resolved_with_accept_remote`,
  `blocked_by_actor_unknown`, `pending_user_review`,
  `blocked_by_policy`, `resolved_with_discard_local` (the
  remaining classes — `in_review`, `blocked_by_freshness`,
  `blocked_by_target_identity_drift`,
  `resolved_with_open_in_provider`, `cancelled` — are bound by
  the same schema rules).
- `blocked_reason_class`: `none`, `waiting_permission_repair`,
  `waiting_actor_resolution`, `waiting_admin_policy_review` (the
  remaining classes — `waiting_freshness_refresh`,
  `waiting_target_identity_resolution` — are bound by the same
  schema rules).
- `freshness_class`: `fresh`, `bounded_stale`,
  `unbounded_stale` (`unknown_freshness_repair_required` is bound
  by the same schema rules).
- `actor_drift_class`: `actor_unchanged`,
  `actor_unknown_repair_required` (the remaining classes —
  `actor_subject_changed_since_draft`,
  `actor_class_changed_since_draft`,
  `actor_grant_revoked_since_draft`,
  `actor_token_expired_since_draft` — are bound by the same
  schema rules).
- `scope_drift_class`: `scope_unchanged`,
  `additional_scopes_required_since_draft`,
  `surplus_scopes_since_draft` (the remaining classes —
  `tenant_or_org_scope_realigned_since_draft`,
  `project_scope_realigned_since_draft`,
  `delegated_grant_lapsed_since_draft` — are bound by the same
  schema rules).
- `draft_state_class`: `draft_unchanged_since_compare`,
  `draft_modified_since_compare` (the remaining classes —
  `draft_rolled_back_since_compare`,
  `draft_deleted_since_compare` — are bound by the same schema
  rules).
- `conflict_class`: `text_conflict`, `structured_field_conflict`,
  `review_metadata_conflict`, `transition_conflict`,
  `comment_conflict`, `status_update_conflict`,
  `attachment_conflict`, `permission_conflict`,
  `target_identity_conflict` (all nine classes exercised).
- `diff_signal_class`: `text_diff`, `structured_field_change`,
  `review_metadata_change`, `transition_change`,
  `comment_added_or_removed`, `status_update_added`,
  `attachment_added_or_removed`, `permission_changed`,
  `target_renamed` (the remaining classes —
  `comment_body_changed`, `target_relocated`,
  `target_replaced`, `target_deleted` — are bound by the same
  schema rules).
- `drift_source_class`: `remote_edit_since_import`,
  `stale_import_snapshot`, `changed_permissions`,
  `target_identity_drift`, `actor_or_scope_change` (the
  `unknown_drift_repair_required` class is bound by the same
  schema rules).
- `reconcile_action_class`: `compare_local_with_remote`,
  `compare_intended_publish_with_remote`,
  `rebase_local_on_remote`, `edit_local_draft`,
  `discard_local_draft`, `export_local_draft`,
  `open_in_provider`, `accept_remote_as_authoritative`,
  `fork_into_new_target`, `escalate_for_admin_review` (all ten
  classes exercised; the `admissible: false` cases are exercised
  on `rebase_local_on_remote` and `accept_remote_as_authoritative`).
- `resolution_disposition_class`: `pending`,
  `rebase_local_on_remote`, `edit_local_then_review_again`,
  `discard_local_keep_remote`,
  `accept_remote_as_authoritative`, `fork_into_new_target`,
  `escalate_for_admin_review` (the remaining classes —
  `discard_local_cancel`, `open_in_provider_to_resolve`,
  `cancel_review` — are bound by the same schema rules).
- `support_export_channel_class`: `support_bundle`,
  `audit_packet`, `migration_note`,
  `admin_assisted_handoff_packet`, `object_handoff_packet` (all
  five classes exercised).
- `redaction_class`: `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only` (all four classes exercised).

Adding a case is additive-minor. Repurposing a `case_id` is
breaking and requires a new decision row.
