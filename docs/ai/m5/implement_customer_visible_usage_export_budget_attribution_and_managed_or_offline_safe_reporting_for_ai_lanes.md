# Customer-visible usage export, budget attribution, and managed or offline-safe reporting for AI lanes

This contract materializes the customer-facing AI usage-reporting surface into
one export-safe truth packet whose unit of truth is a usage report row. Shell,
docs, support export, and release tooling consume the packet directly instead of
re-describing usage, attribution, or continuity state by hand.

- Packet type: `aureline_ai::implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes::UsageReportingPacket`
- Schema: [`schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json`](../../../schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json)
- Support export: [`artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/support_export.json`](../../../artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/support_export.json)
- Fixtures: [`fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/`](../../../fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/)

## The usage report row

Each `UsageReportRow` binds, for one reporting period of one governed AI lane:

| Field | Meaning |
| --- | --- |
| `lane_id`, `lane_label`, `reporting_period_label` | Identity, label, and review-safe period for the report. |
| `resolved_mode`, `quota_family` | Local, BYOK, managed, or enterprise-gateway mode and the quota family that rations the lane. |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `continuity` | Managed-only, managed-with-offline-fallback, offline-safe, or local-only. |
| `offline_generatable` | Whether the report can be produced offline without a managed service. |
| `generation_state` | Generated, degraded-partial, managed-unavailable-used-offline, managed-unavailable-no-fallback, or pending. |
| `region`, `retention` | Disclosed region and retention posture for the lane. |
| `usage_export` | Customer-visible export: availability, customer-visibility, completeness, redaction, and format. |
| `budget_attribution` | Period total cost band, measurement, charge owner, and the attributed slices that split it. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `rollback_posture`, `rollback_verified` | Reversal posture for a reporting-policy change and whether it was drilled. |
| `evidence_packet_refs` | Evidence backing a claimed report. |

## Invariants enforced by validation

- **Customer-visible export, honest completeness.** A customer-visible export
  must name its format, and a `partial_degraded` or `estimate_only` export may not
  back a Stable claim. An export may not advertise itself as available
  (`available_now` or `on_request`) when the report itself is unavailable.
- **Attribution reconciles and discloses who pays.** Budget attribution carries
  at least one line, no attributed slice may cost more than the period total band,
  and a charged total or line must disclose its charge owner rather than leaving it
  `charge_unknown_unverified`. An `estimated_unverified_band` total or
  `estimate_band` measurement may not back a Stable claim.
- **Managed or offline-safe reporting is honest about reach.** An offline-safe,
  offline-fallback, or local-only continuity must actually be `offline_generatable`;
  a local lane can never be managed-only; and an offline fallback can only have
  served the report (`managed_unavailable_used_offline`) on a lane that is
  offline-capable. A degraded or unavailable generation state narrows the claim
  instead of claiming Stable, while a clean offline fallback that served the full
  report keeps its claim because the continuity feature worked.
- **Locality never hides.** A local lane keeps its bytes on-device
  (`on_device_only`) with no provider retention (`no_retention_local_only`); an
  off-device lane may not claim its bytes stay on-device; and a `policy_blocked`
  region or retention posture narrows the claim.
- **Claimed reports carry evidence and a verified reversal.** Stable, Beta, and
  Preview reports must list at least one evidence packet ref, and any reversing
  rollback posture must be `verified`.
- **Narrow, never hide.** Every report carries the `proof_stale` and
  `provider_unavailable` downgrade triggers, and every rule must narrow strictly
  below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema, the frozen M5 AI workflow matrix schema whose qualification,
downgrade, and rollback vocabularies the packet reuses, and the routing-policy
schema whose mode, quota, and cost-band vocabularies it reuses. The
`proof_freshness` block records the freshness SLO and asserts that stale proof
automatically narrows claimed reports. Reading the checked export through
`current_usage_reporting_export` re-validates every invariant, so a stale or
malformed artifact fails the consuming surface rather than shipping an optimistic
usage report.

## Boundary

Raw provider endpoints, credential bodies, raw provider payloads, exact token
counts, exact usage counts, and exact spend amounts stay outside the support
boundary. The packet carries modes, families, coarse cost bands, coarse share
classes, region and retention postures, and review-safe labels only; the export
itself is redacted to coarse bands and per-dimension aggregates.
