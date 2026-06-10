# M5 Review, Merge-Queue, and Pipeline Maturity Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every lane
present, trust-review and consumer-projection invariants satisfied, and proof
freshness valid — the difference is which lanes are narrowed and why.

## merge_queue_status_stale_narrowed.json

The merge-queue lane is narrowed to Beta because CI-status truth has gone stale
relative to the head it gates; the panel shows last-known status with an explicit
staleness label and holds auto-merge until status refreshes. Demonstrates the
`merge_queue_status_stale` downgrade trigger narrowing a merge-queue claim rather
than hiding the lane. Remote preview is also Beta in this packet.

## remote_preview_held.json

The remote-preview lane is held pending upstream preview-route attribution
graduation. Held lanes do not require evidence packets and use the
`not_applicable` rollback posture; no remote preview route is offered while held.
The review workspace, merge queue, and pipeline viewer remain Stable.
