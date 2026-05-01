# Provider-linked object header, browser-handoff sheet, and reuse fixtures

Worked cases for the contract frozen in
[`/docs/providers/provider_link_header_and_handoff_contract.md`](../../../docs/providers/provider_link_header_and_handoff_contract.md).

Each fixture is a self-contained YAML document bundling the records a
single scenario would emit. Every record is schema-valid against one
of the boundary schemas:

- [`/schemas/providers/provider_link_header.schema.json`](../../../schemas/providers/provider_link_header.schema.json)
  (provider-link header records, the handoff-reason reuse projection,
  and the header-degradation event).
- [`/schemas/providers/browser_handoff_sheet.schema.json`](../../../schemas/providers/browser_handoff_sheet.schema.json)
  (the browser-handoff sheet, the local-or-cached alternative, and the
  sheet-level audit event).

The owning `connected_provider_record` (the registry-row anchor every
fixture references through `connected_provider_record_id`) is frozen
in [`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
and is exercised in
[`/fixtures/providers/connected_account_cases/`](../connected_account_cases/)
and
[`/fixtures/providers/provider_mode_cases/`](../provider_mode_cases/);
the fixtures here intentionally leave that record out of the bundle so
each case is focused on the header / sheet / reuse / degradation shape
under test.

The `__fixture__` header on every file names the scenario, the record
kinds emitted, and the closed vocabulary members the case exercises.
The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                                   | Record kinds emitted                                                            | Provider class                  | Edit mode / sheet shape                                  |
|-----------------------------------------------------------------|---------------------------------------------------------------------------------|---------------------------------|----------------------------------------------------------|
| `code_host_pull_request_publish_now_header.yaml`                | `provider_link_header_record`                                                   | `review_or_code_host`           | `publish` → `publish_now`                                |
| `issue_tracker_local_draft_only_header.yaml`                    | `provider_link_header_record`                                                   | `issue_or_planning_tracker`     | `draft` → `local_draft` (no provider state)              |
| `docs_portal_inspect_only_imported_snapshot_header.yaml`        | `provider_link_header_record`                                                   | `docs_or_portal_provider`       | `inspect_only` (bounded_stale snapshot)                  |
| `unknown_actor_class_blocked_for_repair_header.yaml`            | `provider_link_header_record`, `header_degradation_event_record`                | `ci_or_check_provider`          | `blocked_for_repair` (`actor_revoked`)                   |
| `mirror_derived_disclosure_header.yaml`                         | `provider_link_header_record`, `header_degradation_event_record`                | `package_registry_provider`     | `inspect_only` (`mirror_derived_disclosure`)             |
| `header_degradation_freshness_drift_unbounded_stale.yaml`       | `provider_link_header_record`, `header_degradation_event_record`                | `docs_or_portal_provider`       | `inspect_only` after drift, publish blocked              |
| `ci_rerun_browser_handoff_sheet_user_login.yaml`                | `browser_handoff_sheet_record`, `local_or_cached_alternative_record`, `browser_handoff_sheet_audit_event_record` | `ci_or_check_provider` (sheet on `ci_provider_web`) | `requires_provider_login_session`, alternative `defer_to_publish_later_queue` |
| `release_publisher_irreversible_handoff_sheet.yaml`             | `browser_handoff_sheet_record`, `local_or_cached_alternative_record`, `browser_handoff_sheet_audit_event_record` | `release_publisher_provider` (sheet on `release_publisher_web`) | `irreversible_external_publish`, alternative `request_admin_assisted_handoff` |
| `external_generic_runbook_handoff_sheet.yaml`                   | `browser_handoff_sheet_record`, `local_or_cached_alternative_record`            | (sheet on `external_generic_web`, reason `external_docs_or_runbook`) | `leaves_workspace_scope`, alternative `download_local_export` |
| `notification_and_support_export_reuse.yaml`                    | `handoff_reason_reuse_record` (×3)                                              | `ci_or_check_provider`          | reuse on `desktop_notification`, `activity_center_row`, `support_export_summary` |

Vocabulary coverage across the bundle:

- `provider_class`: `review_or_code_host`, `issue_or_planning_tracker`,
  `ci_or_check_provider`, `docs_or_portal_provider`,
  `package_registry_provider`, `release_publisher_provider` (the
  remaining classes — `identity_or_enterprise_provider`,
  `callback_or_event_provider`, `ai_provider`, `managed_admin_provider`
  — are bound by the same schema rules).
- `local_or_provider_source`: `local_authoritative` (via the
  publish-now header's local-side companion), `provider_authoritative`,
  `mirror_derived`, `imported_snapshot_only`,
  `local_draft_only_no_provider_state`.
- `freshness_tier`: `live_authoritative`, `recently_synced_fresh`,
  `bounded_stale`, `unbounded_stale`,
  `unknown_freshness_repair_required`.
- `edit_mode`: `draft`, `publish`, `browser_only` (via the sheet
  fixtures' originating headers), `inspect_only`, `blocked_for_repair`.
- `actor_class`: `human_account`, `installation_or_app_grant`,
  `policy_injected_service_identity`, `unknown_actor_class`.
- `destination_class`: `ci_provider_web`, `release_publisher_web`,
  `external_generic_web`.
- `reason_code`: `publish_requires_browser_auth`,
  `external_docs_or_runbook`.
- `privacy_or_data_loss_consequence_class`:
  `requires_provider_login_session`, `irreversible_external_publish`,
  `leaves_workspace_scope`.
- `replay_posture`: `single_use`, `read_only_resumable`.
- `alternative_class`: `defer_to_publish_later_queue`,
  `request_admin_assisted_handoff`, `download_local_export`.
- `sheet_state_class`: `ready_for_user_confirmation`,
  `user_confirmed_pending_launch`.
- `sheet_audit_event_id`: `browser_handoff_sheet_minted`,
  `browser_handoff_sheet_user_confirmed`.
- `degradation_class`: `actor_revoked`, `mirror_derived_disclosure`,
  `freshness_drift_to_unbounded_stale`.
- `retained_action_class` and `removed_action_class`: covered in the
  three degradation events.
- `reuse_surface_class`: `desktop_notification`, `activity_center_row`,
  `support_export_summary`.

Adding a case is additive-minor. Repurposing a `case_id` is breaking
and requires a new decision row.
