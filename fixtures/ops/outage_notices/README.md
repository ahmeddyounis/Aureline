# Outage and maintenance notice worked cases

These fixtures anchor the contract in
[`/docs/ux/control_data_plane_status_contract.md`](../../../docs/ux/control_data_plane_status_contract.md)
and validate against
[`/schemas/ops/outage_notice.schema.json`](../../../schemas/ops/outage_notice.schema.json).

Each fixture keeps control-plane effects, data-plane effects, local
continuity, blocked writes, deferral paths, boundary review, and stale
notice retention explicit.

## Coverage matrix

| Case | Kind | State | Required guidance | Boundary posture |
| --- | --- | --- | --- | --- |
| [`scheduled_maintenance_export_before.json`](./scheduled_maintenance_export_before.json) | planned maintenance | scheduled | export before maintenance | unchanged |
| [`read_only_publish_later.json`](./read_only_publish_later.json) | unplanned degradation | read-only | publish later | unchanged |
| [`drain_reconnect_later.json`](./drain_reconnect_later.json) | planned maintenance | drain | reconnect later | unchanged |
| [`tenant_migration_boundary_change.json`](./tenant_migration_boundary_change.json) | tenant migration | migration | review new boundary | tenant, region, residency changed |
| [`regional_failover_review_boundary.json`](./regional_failover_review_boundary.json) | unplanned degradation | failover | review new boundary, reconnect later | region, key, endpoint changed |
| [`reconciling_publish_queue_review.json`](./reconciling_publish_queue_review.json) | unplanned degradation | reconciling | publish later | unchanged |
| [`resolved_boundary_change_retained.json`](./resolved_boundary_change_retained.json) | tenant migration | resolved | review new boundary | historical retained boundary change |

## Fixture rules

- Opaque refs stand in for tenants, endpoints, queues, evidence, and
  support artifacts.
- Exact UTC times, display timezones, and UTC offsets are present on
  every notice.
- No fixture uses raw URLs, raw hostnames, raw tenant names, raw account
  ids, raw endpoint credentials, raw policy bodies, or raw secret
  material.
