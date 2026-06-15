# Commercial-control-plane matrix — human-readable rendering

Human-readable rendering of the canonical commercial-control-plane matrix. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-commercial-control-plane.json`.

## Per-lane matrix

| Lane | Service family | Meter family | Unit / window | As-of | Scope owner | Chargeback scopes | Fail posture | Forecast | Export | Claim |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| managed_lane.ai_gateway | ai_gateway_family | ai_gateway_meter_family | tokens / calendar_month_utc | required | organization | personal, workspace, organization | fail_open_local_safe_with_label | forecast_authoritative | parity_with_csv_and_json | managed_full |
| managed_lane.settings_sync | sync_family | profile_or_settings_sync_meter_family | bytes_stored / rolling_30d | required | workspace | personal, workspace, organization | fail_open_local_safe | forecast_best_effort_local | parity_with_json_only | managed_full |
| managed_lane.companion_relay | collaboration_relay_family | collaboration_relay_meter_family | participant_minutes / rolling_24h | required | workspace | workspace, organization | fail_closed_managed_only | forecast_best_effort_local | parity_with_csv_and_json | managed_full |
| managed_lane.registry_mirror | registry_or_mirror_metadata_family | registry_or_mirror_meter_family | download_count / calendar_month_utc | required | organization | organization, tenant | fail_open_local_safe | forecast_best_effort_local | parity_with_csv_and_json | managed_full |
| managed_lane.support_ingest | telemetry_or_support_ingest_family | support_ingest_meter_family | support_bundle_count / rolling_30d | required | tenant | organization, tenant | fail_open_local_safe_with_label | forecast_best_effort_local | parity_with_csv_and_json | managed_full |
| managed_lane.managed_workspace | remote_workspace_control_plane_family | remote_workspace_control_plane_meter_family | workspace_hours / calendar_month_utc | required | organization | workspace, organization | fail_closed_managed_only | forecast_authoritative | parity_with_json_only | managed_full |

Every lane keeps a non-empty local-safe baseline (for example, the AI gateway lane keeps
direct and BYOK AI routes plus local editing, search, and Git), so a metering or rating
failure narrows the managed action but never the local core.

## Managed-state vocabulary

| State | Entitlement | Posture origin | Claim cap | Distinct from |
| --- | --- | --- | --- | --- |
| signed_in | entitlement_active | account | managed_full | — |
| local_only | entitlement_not_applicable | local_only_no_managed_account | local_safe_only | — |
| reauth_required | entitlement_pending_recheck | account | managed_narrowed | seat_removed, org_switched, grace_period, managed_blocked |
| managed_blocked | entitlement_suspended_admin | policy | local_safe_only | seat_removed, org_switched, grace_period, reauth_required |
| grace_period | entitlement_in_grace | account | managed_narrowed | seat_removed, org_switched, reauth_required, managed_blocked |
| seat_removed | entitlement_suspended_admin | seat | local_safe_only | org_switched, grace_period, reauth_required, managed_blocked |
| plan_downgrade | entitlement_active | plan | managed_narrowed | seat_removed, org_switched |
| org_switched | entitlement_pending_recheck | org | managed_narrowed | seat_removed, grace_period, reauth_required, managed_blocked |
| forecast_threshold | entitlement_active | metering_quota | managed_narrowed | meter_stale |
| meter_stale | entitlement_pending_recheck | metering_quota | managed_narrowed | forecast_threshold |

## Narrowing under each active managed state

| Active state | Full lanes | Narrowed lanes | Local-safe-only lanes |
| --- | --- | --- | --- |
| (none) / signed_in | 6 | 0 | 0 |
| local_only | 0 | 6 | 6 |
| reauth_required | 0 | 6 | 0 |
| managed_blocked | 0 | 6 | 6 |
| grace_period | 0 | 6 | 0 |
| seat_removed | 0 | 6 | 6 |
| plan_downgrade | 0 | 6 | 0 |
| org_switched | 0 | 6 | 0 |
| forecast_threshold | 0 | 6 | 0 |
| meter_stale | 0 | 6 | 0 |

(`local_safe_only` lanes are also counted as narrowed.)

## Consumer bindings

| Consumer surface | Binds lanes |
| --- | --- |
| account_surface | all six lanes |
| diagnostics | all six lanes |
| help_about | ai_gateway, settings_sync, registry_mirror |
| support_admin_packet | support_ingest, companion_relay, managed_workspace |
| claim_public_truth_automation | all six lanes |

## Summary

- 6 managed lanes, one per service family and meter family.
- 10 frozen managed-state tokens; the four loss conditions stay mutually distinct.
- 5 consumer surfaces, each projecting the effective claim, never a stronger one.
- Every lane keeps a local-safe baseline; a stale meter never blocks the local core.
