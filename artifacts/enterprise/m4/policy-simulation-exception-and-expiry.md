# Policy Simulation, Exception Preview, Approval History, and Expiry Packet

- Packet: `policy:simulation-expiry:default`
- Schema version: `1`
- Contract ref: `policy:simulation_exception_expiry_stable:v1`
- Qualification: `stable`
- Stable simulation views: `2`
- Exception preview sheets: `2`
- Approval history rows: `2`
- Policy diff summaries: `2`
- Expiry banners: `4`
- Defects: `0`

## Stable Objects

| Object | Record kind | Purpose |
| --- | --- | --- |
| Policy simulation view | `policy_simulation_view_record` | Shows changed areas, before/after values, affected surfaces, degraded behavior, stale/offline notes, and linked expiry/approval context before apply. |
| Exception preview sheet | `policy_exception_preview_sheet_record` | Shows exact bypass scope, owner, reason, mitigation, evidence, expiry, and lapse fallback. |
| Approval history row | `policy_approval_history_row_record` | Shows remembered decisions as bounded governed records with revoke/open-details actions and material drift triggers. |
| Policy diff impact summary | `policy_diff_impact_summary_record` | Shows changed area, previous versus simulated value, impacted surfaces, and degraded consequences. |
| Expiry banner | `policy_expiry_banner_record` | Shows what expires, when, what happens afterward, and the renewal/review action. |
| Review packet | `policy_simulation_exception_expiry_review_packet_record` | Links diff, simulation outcome, owner, expiry, chronology, and reapproval triggers across desktop, CLI/headless, and admin/support handoff. |

## Guardrails Verified

1. The source policy simulation beta page audits clean.
2. Every simulation view links at least one diff summary and expiry banner.
3. Every exception or waiver sheet includes scope, owner, reason, mitigation,
   evidence, expiry, fallback behavior, and export-safe lineage.
4. Every approval history row is bounded by expiry and includes target, policy,
   version, and authority drift triggers.
5. Every exception, waiver, and remembered decision has an expiry banner.
6. The review packet covers desktop, CLI/headless, and admin/support handoff and
   includes chronology refs.

## Canonical Paths

- Runtime owner: `aureline_policy::policy_simulation_and_expiry`
- Schema: `schemas/policy/policy-simulation-exception-and-expiry.schema.json`
- Fixtures: `fixtures/enterprise/m4/policy-simulation-exception-and-expiry/`
- Docs: `docs/enterprise/m4/policy-simulation-exception-and-expiry.md`
