# Provider sync-health view, replay-blast-radius summary, and degraded-import vocabulary fixtures

Worked cases for the contract frozen in
[`/docs/providers/provider_sync_health_contract.md`](../../../docs/providers/provider_sync_health_contract.md).

Each fixture is a self-contained YAML document bundling the
records a single scenario would emit. Every record is schema-valid
against the boundary schema:

- [`/schemas/providers/provider_sync_health_view.schema.json`](../../../schemas/providers/provider_sync_health_view.schema.json)
  (`provider_sync_health_view_record`).

The owning `connected_provider_record` (the registry-row anchor
every fixture references through `connected_provider_record_id`)
is frozen in
[`/schemas/integration/browser_handoff_packet.schema.json`](../../../schemas/integration/browser_handoff_packet.schema.json)
and is exercised in
[`/fixtures/providers/connected_account_cases/`](../connected_account_cases/);
the fixtures here intentionally leave that record out of the bundle
so each case is focused on the sync-health view shape under test.
The paired `provider_event_record`, `import_session_record`, and
`webhook_replay_record` are the cross-tool surfaces every non-owning
consumer reads; each fixture cites them through opaque
`replayed_event_refs`, `delayed_event_refs`, `denied_event_refs`,
`duplicate_event_refs`, and `imported_overlay_refs`.

The `__fixture__` header on every file names the scenario, the
record kinds emitted, and the closed-vocabulary members the case
exercises. The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                                        | Provider class                       | Current mode      | Failure class            | Cursor state                       | Degraded-import classes        |
|----------------------------------------------------------------------|--------------------------------------|-------------------|--------------------------|------------------------------------|--------------------------------|
| `code_host_live_mode_no_degradation.yaml`                            | `review_or_code_host`                | `live`            | (none)                   | `cursor_current`                   | (none)                         |
| `issue_planning_delayed_mode_replay_held.yaml`                       | `issue_or_planning_tracker`          | `delayed`         | `replay_held`            | `cursor_lagging`                   | `backfilled`                   |
| `ci_checks_partial_mode_missing_pages.yaml`                          | `ci_or_check_provider`               | `partial`         | `missing_page_detected`  | `cursor_missing_page_detected`     | `partial`                      |
| `docs_portal_replayed_mode_after_cursor_reset.yaml`                  | `docs_or_portal_provider`            | `replayed`        | `cursor_reset_required`  | `cursor_reset_completed`           | `backfilled`                   |
| `release_publisher_denied_mode_signature_failure.yaml`               | `release_publisher_provider`         | `denied`          | `signature_denied`       | `cursor_paused`                    | (none — delivery rejected)      |
| `artifact_registry_offline_mode_provider_unreachable.yaml`           | `package_registry_provider`          | `offline`         | `provider_unreachable`   | `cursor_paused`                    | (none — no new imports)         |
| `ai_provider_mirror_derived_mode_offline_capture.yaml`               | `ai_provider`                        | `mirror_derived`  | `provider_unreachable`   | `cursor_lagging`                   | `mirror_derived`               |
| `identity_provider_denied_mode_host_mismatch.yaml`                   | `identity_or_enterprise_provider`    | `denied`          | `host_mismatch`          | `cursor_paused`                    | `host_mismatch`                |
| `managed_admin_partial_mode_stale_permission.yaml`                   | `managed_admin_provider`             | `partial`         | `stale_permission`       | `cursor_lagging`                   | `partial`, `stale_permission`  |

Vocabulary coverage across the bundle:

- `provider_class`: `review_or_code_host`,
  `issue_or_planning_tracker`, `ci_or_check_provider`,
  `docs_or_portal_provider`, `release_publisher_provider`,
  `package_registry_provider`, `ai_provider`,
  `identity_or_enterprise_provider`, `managed_admin_provider`
  (nine of ten classes exercised; `callback_or_event_provider`
  is bound by the same schema rules).
- `current_mode_class`: `live`, `delayed`, `partial`,
  `replayed`, `denied`, `offline`, `mirror_derived` (all seven
  modes exercised).
- `failure_class`: `replay_held`, `missing_page_detected`,
  `cursor_reset_required`, `signature_denied`,
  `provider_unreachable`, `host_mismatch`, `stale_permission`
  (the remaining classes — `network_unreachable`,
  `credential_expired_or_revoked`, `tenant_switch`,
  `policy_blocked`, `rate_limited`, `partial_failure`,
  `unknown_failure_repair_required` — are bound by the same
  schema rules).
- `cursor_state_class`: `cursor_current`, `cursor_lagging`,
  `cursor_paused`, `cursor_missing_page_detected`,
  `cursor_reset_completed` (the remaining classes —
  `cursor_page_gap_detected`, `cursor_reset_pending`,
  `cursor_reset_in_progress`, `cursor_unknown_repair_required`
  — are bound by the same schema rules).
- `degraded_import_class`: `partial`, `backfilled`,
  `mirror_derived`, `stale_permission`, `host_mismatch` (all
  five frozen classes exercised).
- `escalation_surface_class`: `toast_or_banner`, `detail_view`,
  `cli_inspection`, `support_export`, `audit_packet` (all five
  required members exercised on every record per the schema
  floor).
- `support_export_channel_class`: `support_bundle`,
  `audit_packet`, `migration_note`, `admin_assisted_handoff_packet`,
  `object_handoff_packet` (all five classes exercised).
- `object_class`: `pull_request`, `issue_or_work_item`,
  `check_run`, `docs_page`, `release_artifact`,
  `package_version`, `registry_entry`, `principal_subject`,
  `admin_surface`, `audit_entry`, `other` (the remaining classes
  — `consent_flow`, `install_target`, `tenant_or_org` — are
  bound by the same schema rules).
- `redaction_class`: `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only` (all four classes exercised).

Adding a case is additive-minor. Repurposing a `case_id` is
breaking and requires a new decision row.
