# Entitlement summaries — human-readable rendering

Human-readable rendering of the canonical entitlement-summary set. This row is a depth-lane
proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-entitlement-summary.json`.

## Per-state summary

| Managed state | Entitlement | Plan tier | Role | Seat owner | Account scope | Posture origin | Degradation | Claim | Snapshot freshness |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| signed_in | entitlement_active | team_plan | org_admin | organization | organization | account | no_degradation | managed_full | freshness_live |
| local_only | entitlement_not_applicable | local_only_no_plan | no_managed_role | personal | personal | local_only_no_managed_account | local_only_no_account | local_safe_only | (none) |
| reauth_required | entitlement_pending_recheck | team_plan | member | organization | organization | account | managed_narrowed | managed_narrowed | freshness_aging |
| managed_blocked | entitlement_expired | team_plan | member | organization | organization | policy | managed_blocked_explicit | local_safe_only | freshness_recent |
| grace_period | entitlement_in_grace | team_plan | org_admin | organization | organization | account | managed_narrowed | managed_narrowed | freshness_recent |
| seat_removed | entitlement_suspended_admin | team_plan | member | organization | organization | seat | managed_blocked_explicit | local_safe_only | freshness_recent |
| plan_downgrade | entitlement_active | individual_pro | org_owner | personal | organization | plan | managed_narrowed | managed_narrowed | freshness_recent |
| org_switched | entitlement_pending_recheck | team_plan | org_admin | organization | organization | org | managed_narrowed | managed_narrowed | freshness_aging |
| forecast_threshold | entitlement_active | enterprise_plan | billing_admin | organization | tenant | metering_quota | managed_narrowed | managed_narrowed | freshness_live |
| meter_stale | entitlement_pending_recheck | enterprise_plan | billing_admin | organization | tenant | metering_quota | managed_narrowed | managed_narrowed | freshness_stale |

Every summary carries a non-empty local-only continuation note (for example, the
`meter_stale` summary keeps local editing, search, and Git available and continues managed
reads against the last confirmed snapshot, labeled stale), so a stale or unavailable managed
path narrows the managed claim but never the local core.

## Seat loss and expiry are explicit, not generic sign-in

| Condition | Managed state | Posture origin | Degradation |
| --- | --- | --- | --- |
| Seat reclaimed | seat_removed | seat | managed_blocked_explicit |
| Entitlement expired | managed_blocked | policy | managed_blocked_explicit |
| Reauthentication (sign-in) pending | reauth_required | account | managed_narrowed |
| Org switched | org_switched | org | managed_narrowed |

A seat loss is cited to the `seat` origin and an expiry to the `policy` origin — distinct
from the reauthentication (sign-in) family at the `account` origin — so a surface can never
draw one generic account error over four different conditions.

## Quota snapshots carry no raw number

Every `quota_snapshot` descriptor pins the meter unit, aggregation window, scope owner,
as-of time, and freshness class, with `carries_raw_number` always `false`. The snapshot age
is legible (live, recent, aging, stale) without exposing a raw spend or quota number, and a
local-only summary carries no managed snapshot at all.

## Surface bindings

| Surface | Binds summaries |
| --- | --- |
| account_surface | all ten summaries |
| diagnostics | all ten summaries |
| support_admin_packet | all ten summaries |
| help_about | signed_in, local_only, managed_blocked, seat_removed |
| feature_entry_point | reauth_required, managed_blocked, seat_removed, grace_period, plan_downgrade, org_switched, forecast_threshold, meter_stale |

## Summary

- 10 entitlement summaries, one per managed-state token.
- 5 surfaces, each projecting the effective claim, never a stronger one, and rendering the
  local-only continuation.
- 2 summaries degrade to an explicit managed-blocked state; 6 narrow; 1 backs the full
  claim; the local-only summary carries no plan, role, or quota.
- Every summary keeps a local-only continuation note; a stale meter never blocks the local
  core.
