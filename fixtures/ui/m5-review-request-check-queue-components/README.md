# M5 Review-Request, Checks, and Merge-Queue Component Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every component
present, trust-review and consumer-projection invariants satisfied, proof freshness
valid, and the provider/local distinction, stale-provider vocabulary,
browser-handoff boundary, and local-continue fallback populated on every row — the
difference is which components are narrowed and why.

## merge_queue_provider_stale_narrowed.json

The merge-queue entry is narrowed to Beta because provider queue status has gone
stale relative to the head it gates; the entry shows last-known position and owner
with an explicit staleness label and holds auto-merge until status refreshes.
Demonstrates the `provider_freshness_stale` downgrade trigger narrowing a
merge-queue claim rather than flattening it into a local estimate. The
stack-dependency chip and approval-invalidation banner stay at their baseline Beta
and Preview maturities.

## approval_invalidation_banner_held.json

The approval-invalidation banner is held pending upstream approval-recomputation
graduation. Held components do not require evidence packets and use the
`not_applicable` rollback posture; no banner claim is offered while held, and the
review workspace continues to show last-known approval state labeled provider-backed.
The review-request row, checks-summary card, pending-review tray, merge-readiness
panel, and merge-queue entry remain Stable.
