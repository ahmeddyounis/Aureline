# Pending-Review Trays and Approval-Invalidation Banners: Reviewer-Scope and Reopen-Safe Truth

- Packet: `pending-review-approval:stable:0001`
- Surface: `Pending-review trays and approval-invalidation banners`
- Pending trays: 4
- Approval banners: 7 (5 invalidating)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Pending trays

- **PR #7001** [`tray:pr-7001`]: scope `awaiting_my_review`, next `submit_your_review`, drafts 1, follow-up 0
- **PR #7002** [`tray:pr-7002`]: scope `awaiting_other_reviewers`, next `await_reviewers`, drafts 0, follow-up 0
- **PR #7003** [`tray:pr-7003`]: scope `changes_requested`, next `address_changes_requested`, drafts 0, follow-up 0
- **PR #7004** [`tray:pr-7004`]: scope `awaiting_my_review`, next `publish_follow_up_packet`, drafts 2, follow-up 1

## Approval banners

- **PR #8001** [`banner:pr-8001`]: kind `approval_invalidation`, cause `stale_base`, reopen-safe true
- **PR #8002** [`banner:pr-8002`]: kind `approval_invalidation`, cause `rebased_stack`, reopen-safe false
- **PR #8003** [`banner:pr-8003`]: kind `approval_invalidation`, cause `rewritten_series`, reopen-safe true
- **PR #8004** [`banner:pr-8004`]: kind `approval_invalidation`, cause `changed_queue_state`, reopen-safe false
- **PR #8005** [`banner:pr-8005`]: kind `approval_invalidation`, cause `policy_drift`, reopen-safe true
- **PR #8090** [`banner:pr-8090`]: kind `generic_warning`, cause `stale_base`, reopen-safe false
- **PR #8091** [`banner:pr-8091`]: kind `queue_block`, cause `changed_queue_state`, reopen-safe false
