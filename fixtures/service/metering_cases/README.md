# Commercial control-plane meter row worked cases

These fixtures anchor the contract in
[`/docs/service/metering_and_chargeback_contract.md`](../../../docs/service/metering_and_chargeback_contract.md)
and validate against
[`/schemas/service/meter_row.schema.json`](../../../schemas/service/meter_row.schema.json).

Each fixture demonstrates that entitlement, usage, forecast,
chargeback, grace, offboarding, and export truth surfaces honestly —
without collapsed personal-workspace-org totals, without forecast
theatre under unauthoritative authority, without a stale meter
blocking local-core work, and without billing-pressure prompts
preceding export, support, delete, or local-continuation prompts.

## Coverage matrix

| Case | Operating mode | Meter family | Notable posture |
| --- | --- | --- | --- |
| [`personal_vs_org_chargeback_view.yaml`](./personal_vs_org_chargeback_view.yaml) | `enterprise_saas` | `ai_gateway_meter_family` | Chargeback scope switcher exposes `personal_scope`, `workspace_scope`, and `organization_scope` side-by-side; personal, workspace, and organization usage do not collapse into one total. |
| [`stale_quota_data.yaml`](./stale_quota_data.yaml) | `enterprise_saas` | `ai_gateway_meter_family` | `cached_beyond_freshness_floor` meter; `forecast_unavailable` confidence; managed inference fails closed; export, support, and continue-local-work prompts retained. |
| [`grace_period_offboarding.yaml`](./grace_period_offboarding.yaml) | `enterprise_saas` | `support_ingest_meter_family` | `entitlement_in_grace`; grace-or-offboarding card cites the offboarding-export and deletion-job records; export, support, delete, and continue-local-work prompts precede every billing-pressure prompt. |
| [`ai_gateway_quota_exhaustion.yaml`](./ai_gateway_quota_exhaustion.yaml) | `enterprise_saas` | `ai_gateway_meter_family` | Daily inference budget exhausted; managed inference fails closed; BYOK and local AI continuity preserved; prompt list keeps export and continue-local-work ahead of `increase_budget`, `upgrade_plan`, and `add_payment_method`. |
| [`usage_summary_csv_json_export.yaml`](./usage_summary_csv_json_export.yaml) | `enterprise_saas` | `support_ingest_meter_family` | `parity_with_csv_and_json` export parity with non-null csv and json manifest refs; canonical JSON export is the boundary, CSV ships with a manifest mapping columns back to the row fields. |

## Fixture rules

- Opaque refs stand in for tenants, regions, scopes, sessions, quota
  state records, usage export rows, manifests, navigation
  destinations, deletion jobs, and offboarding exports.
- Exact UTC timestamps appear on every record.
- No fixture uses raw URLs, raw hostnames, raw cloud-region
  identifiers, raw tenant names, raw account ids, raw billing
  account ids, raw invoice numbers, raw price-list values, raw key
  bytes, raw certificate bodies, or raw secret material.
- `display_copy.whole_product_failure_implied`,
  `display_copy.as_of_time_omitted`,
  `display_copy.measurement_unit_omitted`,
  `display_copy.owner_scope_omitted`,
  `display_copy.personal_workspace_org_collapsed`,
  `display_copy.forecast_under_unauthoritative_state`,
  `display_copy.upgrade_prompted_before_export_or_support`,
  `display_copy.local_continuation_path_obscured`, and
  `display_copy.local_core_blocking` remain false on every record.
- Every fixture cites the contract document, the operating-mode and
  capacity contract, the managed metering and usage-export contract,
  and the managed-service seed in `narrative_refs`.
