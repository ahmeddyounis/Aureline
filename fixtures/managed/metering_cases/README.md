# Managed Metering Cases

Worked fixtures for
[`/docs/managed/metering_and_usage_export_contract.md`](../../../docs/managed/metering_and_usage_export_contract.md)
and the boundary schemas at
[`/schemas/managed/quota_state.schema.json`](../../../schemas/managed/quota_state.schema.json)
and
[`/schemas/managed/usage_export_row.schema.json`](../../../schemas/managed/usage_export_row.schema.json).

The cases use opaque refs for tenants, organizations, users,
workspaces, providers, models, policies, meters, export jobs, and
support/offboarding packets. Raw prompts, raw code payloads, raw user
emails, raw tenant names, raw billing ids, raw invoice ids, provider
URLs, and provider account identifiers do not appear.

| Fixture | Record kind | Main state | Coverage |
|---|---|---|---|
| `authoritative_ai_quota_fresh.yaml` | `quota_state_record` | `authoritative` | Fresh managed AI quota state with exact amount and reset window. |
| `cached_workspace_meter_beyond_floor.yaml` | `quota_state_record` | `cached` | Stale cached meter beyond freshness floor, narrowing managed workspace actions. |
| `local_estimate_ai_usage_pending_reconciliation.yaml` | `quota_state_record` | `estimated` | Local AI usage estimate before service reconciliation. |
| `service_unavailable_usage_gap.yaml` | `quota_state_record` | `unavailable` | Missing service meter blocks only bounded managed actions. |
| `policy_suppressed_quota_state.yaml` | `quota_state_record` | `policy_suppressed` | Policy hides amounts while preserving typed suppression. |
| `monthly_authoritative_usage_export_row.yaml` | `usage_export_row_record` | `authoritative` | Monthly export row preserving scope, time basis, unit, and attribution. |
| `unavailable_gap_usage_export_row.yaml` | `usage_export_row_record` | `unavailable` | Exported gap marker for missing service data. |
| `policy_suppressed_usage_export_row.yaml` | `usage_export_row_record` | `policy_suppressed` | Exported policy-suppression marker with suppressed fields. |
