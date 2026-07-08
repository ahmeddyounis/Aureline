# Review-Request Rows: Provider/Local-vs-Provider Truth

- Packet: `review-request-row:stable:0001`
- Surface: `Review-request rows: local-versus-provider truth`
- Rows: 4 (1 provider-backed, 1 local estimate, 1 showing a stale relation)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **PR #4821** [`provider_backed_request`]: provider org/repo pull request #4821 vs feature/login — base/head `current`, stack `standalone`, scope `full_request`, provider freshness `provider_fresh`
- **local:feature/report** [`local_review_estimate`]: local workspace bundle vs feature/report — base/head `current`, stack `standalone`, scope `full_request`, provider freshness `local_only_continuation`
- **export:MR-317** [`offline_exported_packet`]: exported review packet (cached) vs hotfix/crash — base/head `stale_base`, stack `standalone`, scope `full_request`, provider freshness `provider_stale`
- **MR #902** [`browser_handoff_placeholder`]: provider org/repo merge request (unreachable) vs feature/import — base/head `unknown`, stack `stack_member_parent_blocked`, scope `stack_segment`, provider freshness `provider_unreachable`

This summary is a checked-in projection of the canonical support export at
`artifacts/review/m5/implement_review_request_rows_with_provider_base_head_freshness_scope_and_local_versus_provider_truth/support_export.json`.
It is regenerated from `ReviewRequestRowPacket::render_markdown_summary`.
