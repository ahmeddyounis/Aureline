# Merge-queue and landing-candidate fixtures

These fixtures exercise the landing-candidate packet built on top of the
beta review-workspace packet. Each fixture seeds the workspace from the
canonical alpha review seed fixture, then attaches a landing candidate,
optional merge-queue entry, command-graph operations, and a metadata-safe
support/export packet.

The cases prove that review surfaces can show `mergeable`, `queue
eligible`, `queued`, `stale base`, `checks stale`, `policy blocked`, and
`approval invalidated` as separable, inspectable truths rather than one
collapsed status.

| Fixture | Coverage |
| --- | --- |
| `provider_authoritative_mergeable_queued.json` | Provider-authoritative landing candidate is mergeable, queue eligible, and queued at position 2 of 5; commands include dequeue, rerun pipeline, and publish-to-provider with preview/audit. |
| `stale_base_invalidates_queue_entry.json` | Provider queue still holds the candidate, but stale base blocks landing; eligibility drops to `queue_not_eligible`, the queue state is `queued_invalidated_by_stale_base`, and the publish command is blocked by `base_revision_stale`. |
| `local_estimate_no_provider_overlay.json` | Aureline local estimate only, no merge-queue entry; `queue_state` projects as `not_queued`, and refreshing the provider overlay is suggested before queue eligibility is claimed. |
| `policy_blocked_with_invalidated_approval.json` | Repo policy holds the candidate after approval is invalidated and a required check is stale; the candidate surfaces approval-invalidated and policy-blocked as separate truths. |
