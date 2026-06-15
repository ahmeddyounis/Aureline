# Chargeback scope views — human-readable rendering

Human-readable rendering of the canonical chargeback-scope view set. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/service/m5-chargeback-scope-views.json`.

## Per-family chargeback view

| Service family | Meter unit | Window | Scopes (narrow → broad) | Chain root | As-of | Freshness | Export |
| --- | --- | --- | --- | --- | --- | --- | --- |
| ai_gateway_family | tokens | calendar_month_utc | personal, workspace, team, organization | organization | 2026-06-15T00:00:00Z | freshness_live | CSV + JSON |
| sync_family | bytes_stored | rolling_30d | personal, workspace, team, organization | organization | 2026-06-15T00:00:00Z | freshness_recent | CSV + JSON |
| collaboration_relay_family | participant_minutes | rolling_24h | workspace, team, organization | organization | 2026-06-15T00:00:00Z | freshness_recent | CSV + JSON |
| registry_or_mirror_metadata_family | download_count | calendar_month_utc | team, organization, tenant | tenant | 2026-06-15T00:00:00Z | freshness_aging | CSV + JSON |
| telemetry_or_support_ingest_family | support_bundle_count | rolling_30d | team, organization, tenant | tenant | 2026-06-15T00:00:00Z | freshness_stale | CSV + JSON |
| remote_workspace_control_plane_family | workspace_hours | calendar_month_utc | workspace, team, organization | organization | 2026-06-15T00:00:00Z | freshness_live | CSV + JSON |

Each view keeps one cost truth per scope, with an opaque owner identity, and never collapses
personal, workspace, team, and organization into one ambiguous owner total. Every view keeps
a non-empty local-safe baseline, so a stale or unavailable metering path narrows the managed
chargeback view but never local editing, search, Git, or local automation.

## Inherited versus direct attribution

Each scope carries a **direct** line (cost charged to the scope itself) and an **inherited**
line (a share rolled up from the broader parent scope it inherits from). The broadest scope
in a view is the inheritance-chain root, so its inherited line is explicitly not applicable
rather than a hidden zero.

| Scope | Direct | Inherited from |
| --- | --- | --- |
| personal | shown, bound to unit/as-of/scope | workspace |
| workspace | shown, bound to unit/as-of/scope | team |
| team | shown, bound to unit/as-of/scope | organization |
| organization (root, when broadest) | shown, bound to unit/as-of/scope | not applicable — `suppressed_no_managed_number` |
| tenant (root, when broadest) | shown, bound to unit/as-of/scope | not applicable — `suppressed_no_managed_number` |

No cost line carries a raw spend or quota number (`carries_raw_number` is always `false`); a
shown value is bound to its unit, as-of time, and scope owner, and the chain-root inherited
line is explicitly suppressed.

## CSV/JSON export parity

The set exports at CSV/JSON parity. The CSV carries one row per view, scope, and attribution
basis (direct then inherited), with the same fields the JSON packet carries:

```text
view_id,service_family,meter_family,scope_owner,owner_identity,attribution_basis,inherited_from,meter_unit,aggregation_window,as_of,freshness,value_presentation,carries_raw_number,effective_marketed_claim
chargeback_scope.ai_gateway,ai_gateway_family,ai_gateway_meter_family,personal,scope-owner.ai_gateway.personal.opaque,direct,,tokens,calendar_month_utc,2026-06-15T00:00:00Z,freshness_live,month_to_date_bound_to_unit_as_of_scope,false,managed_full
chargeback_scope.ai_gateway,ai_gateway_family,ai_gateway_meter_family,personal,scope-owner.ai_gateway.personal.opaque,inherited,workspace,tokens,calendar_month_utc,2026-06-15T00:00:00Z,freshness_live,month_to_date_bound_to_unit_as_of_scope,false,managed_full
chargeback_scope.ai_gateway,ai_gateway_family,ai_gateway_meter_family,team,scope-owner.ai_gateway.team.opaque,inherited,organization,tokens,calendar_month_utc,2026-06-15T00:00:00Z,freshness_live,month_to_date_bound_to_unit_as_of_scope,false,managed_full
chargeback_scope.ai_gateway,ai_gateway_family,ai_gateway_meter_family,organization,scope-owner.ai_gateway.organization.opaque,inherited,,tokens,calendar_month_utc,2026-06-15T00:00:00Z,freshness_live,suppressed_no_managed_number,false,managed_full
```

The full export carries 40 rows — 20 scope truths across the six views, each with a direct
and an inherited line. Dump it with
`cargo run -p aureline-service --example dump_m5_chargeback_scope_views -- csv`.

## Switching scope does not collapse the truth

| Switcher guarantee | Value |
| --- | --- |
| active_scope (canonical) | organization |
| available_scopes | personal, workspace, team, organization, tenant |
| preserves_active_scope | true |
| preserves_inherited_direct_separation | true |
| preserves_owner_identity | true |
| never_collapses_scopes | true |

A scope switch moves the active scope without dropping a scope, the inherited-versus-direct
separation, or an owner identity. A switch to a scope the switcher does not offer is a
validation failure.

## A narrowing is not one generic account error

A view's effective marketed claim is recomputed from the active managed state's cap, so a
removed seat, an org switch, a grace window, and a sign-in failure each narrow the marketed
chargeback claim with their own typed state and recovery cue — never one generic account
error.

| Active managed state | Effective claim across the 6 views |
| --- | --- |
| signed_in | 6 full, 0 narrowed |
| managed_blocked / seat_removed / local_only | 6 local-safe-only |
| grace_period / plan_downgrade / org_switched / forecast_threshold / meter_stale | 6 narrowed, 0 local-safe-only |

## Summary

- 6 chargeback views, one per service family; personal, workspace, team, organization, and
  tenant never collapse into one ambiguous owner bucket.
- 20 per-scope cost truths, each separating a direct line from an inherited share that names
  its parent scope; the chain root explicitly suppresses its inherited value.
- Every cost line binds its value to the unit, as-of time, and scope owner and carries no raw
  number; the set exports at CSV/JSON parity (40 rows).
- A scope switcher preserves the active scope, the inherited-versus-direct separation, and the
  owner identity, and never collapses the scopes.
- Every view keeps a local-safe baseline; a stale meter never blocks the local core; loss
  conditions stay distinct.
- 5 surfaces, each projecting the effective claim, never a stronger one.
