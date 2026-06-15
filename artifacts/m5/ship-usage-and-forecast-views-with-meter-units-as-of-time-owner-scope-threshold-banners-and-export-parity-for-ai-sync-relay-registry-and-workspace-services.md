# Usage and forecast views — human-readable rendering

Human-readable rendering of the canonical usage-and-forecast view set. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-usage-forecast-views.json`.

## Per-family usage and forecast view

| Service family | Meter unit | Window | Owner scope | As-of | Freshness | Threshold status | Banner severity | Export |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ai_gateway_family | tokens | calendar_month_utc | organization | 2026-06-15T00:00:00Z | freshness_live | approaching_threshold | advisory | CSV + JSON |
| sync_family | bytes_stored | rolling_30d | workspace | 2026-06-15T00:00:00Z | freshness_recent | within_budget | informational | CSV + JSON |
| collaboration_relay_family | participant_minutes | rolling_24h | workspace | 2026-06-15T00:00:00Z | freshness_recent | budget_exhausted | critical | CSV + JSON |
| registry_or_mirror_metadata_family | download_count | calendar_month_utc | organization | 2026-06-15T00:00:00Z | freshness_aging | forecast_unavailable | advisory | CSV + JSON |
| telemetry_or_support_ingest_family | support_bundle_count | rolling_30d | tenant | 2026-06-15T00:00:00Z | freshness_stale | meter_stale_unconfirmed | advisory | CSV + JSON |
| remote_workspace_control_plane_family | workspace_hours | calendar_month_utc | organization | 2026-06-15T00:00:00Z | freshness_live | threshold_crossed | warning | CSV + JSON |

Each view shows the meter unit, the month-to-date value (bound to its unit, as-of time, and
scope owner — never a raw number), the threshold/forecast status, and CSV/JSON export
parity. Every view keeps a non-empty local-safe baseline, so a stale or unavailable
metering path narrows the managed view but never local editing, search, Git, or local
automation.

## Forecast banners explain what changes next

| Threshold status | What changes next |
| --- | --- |
| within_budget | Nothing changes until the forecast crosses the threshold; the value is shown with its unit, as-of time, and scope owner. |
| approaching_threshold | At the threshold, new managed-broker work pauses while local editing, search, and Git continue. |
| threshold_crossed | New managed actions narrow to the plan floor next while the local core continues unchanged. |
| budget_exhausted | New managed-only actions pause until the window resets, while local work continues. |
| forecast_unavailable | The month-to-date value is shown with its as-of time and no projection is implied. |
| meter_stale_unconfirmed | The number is labeled with its last as-of time and cannot be confirmed now; the local core is never blocked. |

Each banner carries a non-empty what-changes-next sentence recomputed from its threshold
status, so the surface explains the consequence rather than relying on a warning color.

## A threshold is not an account error

The forecast/threshold status is metering posture, distinct from the managed-state
vocabulary. A view's effective marketed claim is recomputed from the active managed state's
cap, so a removed seat, an org switch, a grace window, and a sign-in failure each narrow the
marketed usage claim with their own typed state and recovery cue — never one generic account
error.

| Active managed state | Effective claim across the 6 views |
| --- | --- |
| signed_in | 6 full, 0 narrowed |
| managed_blocked / seat_removed / local_only | 6 local-safe-only |
| grace_period / plan_downgrade / org_switched / forecast_threshold / meter_stale | 6 narrowed, 0 local-safe-only |

## Surface bindings

| Surface | Binds views |
| --- | --- |
| account_usage_surface | all six views |
| service_health_diagnostics | all six views |
| help_about | ai_gateway, settings_sync, registry_mirror |
| support_admin_export | support_ingest, companion_relay, managed_workspace |
| release_center | all six views |

## Summary

- 6 usage-and-forecast views, one per service family; unlike families never merge into one
  opaque total.
- 6 distinct meter units, 6 distinct threshold statuses exercised.
- Every measurement binds its value to the unit, as-of time, and scope owner and carries no
  raw number; every view exports at CSV/JSON parity.
- Every banner explains what changes next; every view keeps a local-safe baseline; a stale
  meter never blocks the local core.
- 5 surfaces, each projecting the effective claim, never a stronger one.
