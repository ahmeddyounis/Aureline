# Review-Component Accessibility, Headless, and Export Parity

- Packet: `review-component-accessibility:stable:0001`
- Surface: `Review-component accessibility, headless, and export parity`
- Accessibility rows: 7 (5 claim-narrowed)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Accessibility rows

- **review_request_row** [`row:rr-fresh`]: condition `provider_fresh`, claim `provider_backed`
- **checks_summary_card** [`row:cs-stale`]: condition `provider_freshness_stale`, claim `locally_reviewable`
- **merge_readiness_panel** [`row:mr-estimate`]: condition `queue_authority_local_estimate`, claim `estimate_only`
- **merge_queue_entry** [`row:mq-handoff`]: condition `browser_handoff_required`, claim `handoff_required`
- **stack_dependency_chip** [`row:sd-fresh`]: condition `provider_fresh`, claim `provider_backed`
- **approval_invalidation_banner** [`row:ai-approval-missing`]: condition `approval_lineage_missing`, claim `approval_unverified`
- **pending_review_tray** [`row:pt-stale`]: condition `provider_freshness_stale`, claim `locally_reviewable`
