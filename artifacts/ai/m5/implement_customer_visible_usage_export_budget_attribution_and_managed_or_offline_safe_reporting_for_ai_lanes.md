# Customer-Visible Usage Export, Budget Attribution, And Managed Or Offline-Safe Reporting

- Packet: `usage-reporting:stable:0001`
- Schema: `schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json`
- Support export: `artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/support_export.json`
- Fixture: `fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/`

## Coverage

The packet materializes the customer-facing AI usage-reporting surface into one
row per reporting period of a governed AI lane. Every row carries the
customer-visible usage export the customer can read, the budget attribution that
splits the period's spend across the dimensions that consumed it, and the managed
or offline-safe reporting continuity that says whether the report survives a
managed outage.

- The composer managed lane reports at Stable: a metered-medium period total
  charged to the subscriber and split across the workspace (dominant) and model
  (minor) dimensions, a complete and available customer-visible export, a single
  pinned region with bounded retention plus user export, and a
  `managed_with_offline_fallback` continuity that is generatable offline.
- The review BYOK lane reports at Beta: its managed service was down, so the
  report was served from the offline fallback (`managed_unavailable_used_offline`)
  while staying complete and on-request — the offline-safe continuity working as
  designed — with a metered-low total attributed to the BYOK user.
- The explain local lane reports at Preview: a `local_only` continuity generated
  entirely on-device, bytes pinned `on_device_only` with no provider retention, a
  bundled no-cost total, and a complete available export.
- The background managed lane is `managed_only` with no offline fallback: the same
  outage left it `managed_unavailable_no_fallback`, its export `unavailable` and
  `partial_degraded`, and its attribution an estimate, so it dropped out of every
  claimed lane to `held`, carries no evidence refs, and narrows to `unavailable`
  on stale proof or provider unavailability.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to present a usage report greener than its export,
attribution, and continuity posture can back. A partial or estimate-only export
and an estimate-only attribution may not back a Stable claim; an export may not
advertise availability when the report is unavailable; no attributed slice may
cost more than the period total; a charged total or line discloses who is
charged; an offline-safe or offline-fallback lane must be generatable offline; a
local lane may not be managed-only; region and retention are disclosed so locality
never hides; and a degraded, unavailable, or policy-blocked report narrows the
claim instead of hiding behind a Stable label. Cost, provider, region, retention,
and reporting authority are never hidden behind generic AI language. Raw provider
endpoints, credential bodies, raw provider payloads, exact token counts, exact
usage counts, and exact spend amounts never cross the boundary.
