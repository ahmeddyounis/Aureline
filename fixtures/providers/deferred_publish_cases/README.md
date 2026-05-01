# Deferred-publish queue, stale-target review, and continuity-event fixtures

Worked cases for the contract frozen in
[`/docs/providers/deferred_publish_queue_contract.md`](../../../docs/providers/deferred_publish_queue_contract.md).

Each fixture is a self-contained YAML document bundling the records
a single scenario would emit. Every record is schema-valid against
one of the boundary schemas:

- [`/schemas/providers/deferred_publish_queue_item.schema.json`](../../../schemas/providers/deferred_publish_queue_item.schema.json)
  (`deferred_publish_queue_item_record`,
  `deferred_publish_continuity_event_record`).
- [`/schemas/providers/deferred_publish_review.schema.json`](../../../schemas/providers/deferred_publish_review.schema.json)
  (`deferred_publish_review_record`).

The owning `connected_provider_record` (the registry-row anchor every
fixture references through `connected_provider_record_id`) is frozen
in [`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
and is exercised in
[`/fixtures/providers/connected_account_cases/`](../connected_account_cases/)
and
[`/fixtures/providers/provider_mode_cases/`](../provider_mode_cases/);
the fixtures here intentionally leave that record out of the bundle
so each case is focused on the deferred-publish queue, the stale-
target review, and the continuity-event shape under test. The
paired `publish_later_queue_item_record` is the cross-tool surface
every non-owning consumer reads; each fixture cites it through
`publish_later_queue_item_ref` so support exports and admin
reconciliation consoles can pivot.

The `__fixture__` header on every file names the scenario, the
record kinds emitted, and the closed-vocabulary members the case
exercises. The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                                                 | Record kinds emitted                                                                                                | Surface class                | Queue state(s)                                              | Stale-target risk                  |
|-------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|------------------------------|-------------------------------------------------------------|------------------------------------|
| `code_host_pr_publish_offline_capture_then_reconnect.yaml`                    | queue item, continuity event (`connectivity_restored_revalidated`), review (`admit_for_drain`)                      | `code_host_surface`          | `captured_offline` → `ready_for_drain`                       | `target_unchanged`                 |
| `release_publish_blocked_by_reauth_step_up.yaml`                              | queue item, review (`hold_for_reauth`)                                                                              | `release_publisher_surface`  | `pending_reauth`                                             | `target_unchanged`                 |
| `issue_create_blocked_by_rescope_additional_scopes.yaml`                      | queue item, review (`hold_for_rescope`)                                                                             | `issue_or_planning_surface`  | `pending_rescope`                                            | `target_unchanged`                 |
| `pull_request_merge_blocked_by_predecessor_dependency.yaml`                   | queue item, review (`hold_for_dependency`)                                                                          | `code_host_surface`          | `pending_dependency`                                         | `target_unchanged`                 |
| `docs_publish_stale_target_compare_with_remote.yaml`                          | queue item, continuity event (`target_freshness_drift_invalidated`), review (`hold_for_target_refresh`)             | `docs_or_portal_surface`     | `pending_target_refresh`                                     | `target_drifted_unbounded`         |
| `release_artifact_blocked_by_tenant_switch_grant_revoked.yaml`                | queue item, two continuity events (`tenant_switched_invalidated`, `grant_revoked_invalidated`), review (`cancel_at_user_request`) | `artifact_registry_surface`  | `ready_for_drain` → `pending_admin_review` → `cancelled_by_user` | `target_actor_scope_changed`       |

Vocabulary coverage across the bundle:

- `action_kind`: `pull_request_review_comment_publish`,
  `release_publish`, `issue_create`, `pull_request_merge`,
  `docs_page_publish`, `release_artifact_attach` (the remaining
  action kinds — `pull_request_create`, `pull_request_update`,
  `pull_request_review_decision_publish`, `branch_push`,
  `issue_update`, `issue_state_transition`, `issue_comment_publish`,
  `check_run_request_rerun`, `docs_page_update`,
  `artifact_version_publish`, `consent_grant_acknowledge`,
  `admin_console_action_request` — are bound by the same schema
  rules).
- `mutation_mode`: `deferred_publish` (the publish-later contract
  exercises the other modes).
- `surface_class`: `code_host_surface`, `release_publisher_surface`,
  `issue_or_planning_surface`, `docs_or_portal_surface`,
  `artifact_registry_surface`.
- `queue_state`: `captured_offline`, `pending_reauth`,
  `pending_rescope`, `pending_dependency`, `pending_target_refresh`,
  `pending_admin_review`, `ready_for_drain`, `cancelled_by_user`.
- `stale_target_risk_class`: `target_unchanged`,
  `target_drifted_unbounded`, `target_actor_scope_changed`.
- `dependency_class`: `connectivity_restored`,
  `approval_ticket_admitted`, `consequence_preview_confirmed`,
  `freshness_floor_satisfied`, `reauth_completed`,
  `rescope_completed`, `effective_scope_satisfied`,
  `account_mapping_resolved`, `predecessor_queue_item`,
  `target_object_must_exist`, `conflict_resolved` (the remaining
  classes — `browser_handoff_complete`, `provider_health_recovered`,
  `remote_availability_recovered`, `policy_epoch_stable`,
  `user_reconfirm_required` — are bound by the same schema rules).
- `dependency_state`: `unmet`, `met`.
- `retry_backoff_class`: `linear_bounded`, `after_user_reconfirm`,
  `after_dependency_clears`, `no_retry_manual_only`.
- `reauth_class`: `not_required`,
  `step_up_authenticator_required`,
  `admin_re_authorisation_required`.
- `rescope_class`: `not_required`, `additional_scopes_required`,
  `tenant_or_org_scope_realignment_required`.
- `serialization_format_class`: `inline_record_safe`,
  `offloaded_to_local_evidence_store`,
  `requires_repair_before_restart`.
- `continuity_event_class`: `connectivity_restored_revalidated`,
  `target_freshness_drift_invalidated`,
  `tenant_switched_invalidated`, `grant_revoked_invalidated`.
- `review_state_class`: `ready_for_drain`, `blocked_by_reauth`,
  `blocked_by_rescope`, `blocked_by_dependency`,
  `blocked_by_freshness`, `blocked_by_actor_unknown`.
- `blocked_reason_class`: `none`, `waiting_reauth`,
  `waiting_rescope`, `waiting_dependency`,
  `waiting_freshness_refresh`, `waiting_actor_resolution`.
- `compare_or_rebase_option_class`: `none_offered`,
  `compare_with_remote`, `rebase_local_on_remote`,
  `refresh_target_then_review`, `fork_into_new_target`,
  `abandon_in_favor_of_remote`.
- `export_or_copy_fallback_class`: `none_offered`,
  `export_evidence_packet`, `copy_summary_for_offline`,
  `export_via_object_handoff_packet`,
  `export_for_admin_assisted_handoff`.
- `review_action_class`: `confirm_retry_now`,
  `confirm_retry_after_reauth`, `confirm_retry_after_rescope`,
  `confirm_retry_after_target_refresh`,
  `defer_with_new_prerequisite`, `cancel_queue_item`,
  `request_admin_assisted_handoff`, `compare_with_remote`,
  `copy_summary_for_offline`, `export_evidence_packet`.
- `review_disposition_class`: `admit_for_drain`,
  `hold_for_reauth`, `hold_for_rescope`, `hold_for_dependency`,
  `hold_for_target_refresh`, `cancel_at_user_request`.

Adding a case is additive-minor. Repurposing a `case_id` is breaking
and requires a new decision row.
