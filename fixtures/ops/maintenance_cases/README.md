# Scheduled-maintenance, drain, migration, and failover worked cases

These fixtures anchor the contract in
[`/docs/ops/maintenance_migration_failover_contract.md`](../../../docs/ops/maintenance_migration_failover_contract.md)
and validate against
[`/schemas/ops/maintenance_notice.schema.json`](../../../schemas/ops/maintenance_notice.schema.json)
and
[`/schemas/ops/tenant_migration_event.schema.json`](../../../schemas/ops/tenant_migration_event.schema.json).

Each fixture keeps planned-operations communication distinct from
incident banners or generic-offline copy, and restates every boundary
axis (tenant, region, residency, key ownership, endpoint identity)
when migration or failover changes the effective environment.

## Coverage matrix

| Case | Kind | State | Required follow-up | Boundary posture |
| --- | --- | --- | --- | --- |
| [`read_only_window_publish_later.yaml`](./read_only_window_publish_later.yaml) | scheduled_read_only_window | read_only_window | publish-later | unchanged |
| [`drain_before_failover.yaml`](./drain_before_failover.yaml) | scheduled_drain_window + regional_failover | drain_window + drain_before_failover | reconnect-later, review-new-boundary | region and endpoint identity change at cutover |
| [`tenant_migration_new_region.yaml`](./tenant_migration_new_region.yaml) | tenant_migration | in_progress_migration | export-before-cutover, review-new-boundary, reconnect-later | tenant, region, residency, endpoint identity change; key ownership preserved |
| [`cached_stale_status_after_event.yaml`](./cached_stale_status_after_event.yaml) | post_maintenance_reconciliation + tenant_migration | reconciling_after_window + completed_boundary_changed | review-new-boundary, open-history | historical retained boundary change with stale label |
| [`export_before_maintenance.yaml`](./export_before_maintenance.yaml) | scheduled_export_freeze | pre_window_export | export-before-maintenance | unchanged |

## Fixture rules

- Opaque refs stand in for tenants, endpoints, queues, evidence, and
  reconnect tokens.
- Exact UTC times, the IANA display timezone, and the UTC offset that
  applied at window start or cutover are present on every record.
- No fixture uses raw URLs, raw hostnames, raw tenant names, raw
  account ids, raw endpoint credentials, raw policy bodies, or raw
  secret material.
- `display_copy.all_work_broken_implied`,
  `display_copy.incident_language_used`, and the `generic_offline_banner_used`
  / `generic_all_clear_used` invariants stay false on every record.
- Every fixture quotes the contract document and the locality /
  continuity seed in `narrative_refs`.
