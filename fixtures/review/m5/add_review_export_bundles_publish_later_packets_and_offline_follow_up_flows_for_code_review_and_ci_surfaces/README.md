# Review/Export Bundle Fixtures

These fixtures are valid, export-safe packets that exercise the provenance,
redaction, truth-freshness, publish-later, and offline follow-up narrowing
behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## stale_truth_publish_blocked.json

A CI-run bundle whose underlying truth is from a prior base, so the freshness is
`stale_prior_truth`, the publish state is `publish_blocked` and
`blocked_stale_truth_review_required`, and the export carries explicit attention
reasons. Demonstrates that a stale truth narrows the publish rather than shipping
a possibly-wrong state.

## offline_replay_held.json

A review-thread bundle whose follow-up was queued while offline, so the
connectivity is `offline_queued`, the follow-up disposition is `hold_for_review`,
`replay_ready` stays false, the export is `blocked_offline_no_replay_authority`,
and the export carries explicit attention reasons. Demonstrates that an offline
follow-up is held for review and never pre-authorized to auto-fire on reconnect.
