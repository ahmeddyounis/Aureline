# Merge queue and landing candidate beta

The landing-candidate packet sits on top of the beta review-workspace
packet and gives review surfaces, CLI/headless, support/export, and
merge-queue inspectors one shared object model for *what landing means*
without collapsing local Git evidence, provider overlays, and queue state
into a single ambiguous status.

The companion schemas live at:

- [`/schemas/review/landing_candidate.schema.json`](../../../schemas/review/landing_candidate.schema.json)
- [`/schemas/review/merge_queue_entry.schema.json`](../../../schemas/review/merge_queue_entry.schema.json)

The canonical fixtures live under:

- [`/fixtures/review/m3/merge_queue_and_landing/`](../../../fixtures/review/m3/merge_queue_and_landing/)

The Rust types are exported from `aureline_review::landing`, defined in
[`crates/aureline-review/src/landing/mod.rs`](../../../crates/aureline-review/src/landing/mod.rs).
The integration test
[`crates/aureline-review/tests/landing_candidate.rs`](../../../crates/aureline-review/tests/landing_candidate.rs)
replays each fixture from the existing alpha seed fixture and validates
the closed acceptance states.

## Record shape

One `review_landing_candidate_packet` contains:

| Block | Purpose |
| --- | --- |
| `review_workspace` | The canonical VCS `review_workspace_record` so local, provider-overlay, imported, and browser-handoff sources share identity and freshness vocabulary. |
| `landing_candidate` | The reviewed landing candidate: target branch, base/head, change object, worktree identity, review-pack and environment-capsule digests, merge strategy, authority, separable truths, required checks, invalidation reasons, blocked reasons, and `landing_requires_explicit_candidate = true`. |
| `merge_queue_entry` | Optional queue snapshot with authority class, lifecycle state, position/length, required checks, capture/expire timestamps, and queue-specific invalidation reasons. `authoritative_position` and `local_estimate_only` make queue-state authority explicit. |
| `commands` | Command-graph operations defined for this candidate (enqueue, dequeue, approve, request_changes, rerun_pipeline, publish_to_provider, refresh_provider_overlay, mark_review_pack_landed). Each command must emit an audit event when executed and may advertise preview/dry-run. `publish_to_provider` must declare a provider publish posture. |
| `support_export` | Metadata-only export packet with reopen context, reopen command, command refs, optional merge-queue entry ref, source schema refs, an eligibility snapshot that mirrors candidate state, and `raw_url_export_allowed = false`, `raw_provider_payload_export_allowed = false`. |
| `inspection` | Deterministic booleans and counts (`mergeable`, `queue_eligible`, `queued`, `stale_base_blocks_landing`, `checks_stale_blocks_landing`, `policy_blocks_landing`, `approval_invalidated`, `provider_authoritative`, `queue_state_is_local_estimate_only`, `candidate_invalidated`, `landing_requires_explicit_candidate`, `command_count`, `required_check_count`, `preview_capable`, `support_export_reopenable`). |

## Separable truths

Review workspace strips and sheets project each axis below from its own
closed vocabulary. Landing is never reduced to one collapsed status.

| Axis | Closed vocabulary |
| --- | --- |
| Authority | `provider_authoritative_queue_state`, `repo_policy_managed_queue_state`, `aureline_local_estimate_only` |
| Mergeable | `mergeable`, `not_mergeable_blocking`, `mergeable_pending_eligibility` |
| Eligibility | `queue_eligible`, `queue_not_eligible` (derived from blocked reasons) |
| Queue state | `not_queued`, `queued`, `dequeued_by_user`, `dequeued_by_provider`, `queued_invalidated_by_stale_base` |
| Stale base | `base_current`, `base_stale_within_grace`, `base_stale_blocks_landing` |
| Checks freshness | `checks_current`, `checks_stale_within_grace`, `checks_stale_blocks_landing` |
| Approval | `approval_not_required_by_policy`, `approval_required_outstanding`, `approved_current`, `approval_invalidated_by_changes` |
| Policy block | `policy_clear`, `policy_blocked` |
| Invalidation | `stale_base`, `checks_stale`, `approval_invalidated`, `policy_blocked`, `review_pack_version_changed`, `worktree_scope_changed`, `environment_capsule_changed`, `provider_overlay_stale`, `queue_dequeued_by_provider` |
| Blocked reason | `target_branch_missing`, `required_check_failed`, `required_check_stale`, `base_revision_stale`, `approval_missing`, `approval_invalidated`, `policy_blocked`, `review_pack_version_drift`, `worktree_scope_changed`, `environment_capsule_changed` |

## Acceptance rules

The validator enforces these rules:

1. A landing candidate cites a target branch, base/head revs, change
   object, worktree identity, review-pack digest, and environment capsule
   digest. Each required check id must already appear in the workspace
   beta packet's `check_freshness` rows.
2. `landing_requires_explicit_candidate` is always `true`: ambient branch
   state or hidden provider mutation cannot land. Only the reviewed
   candidate may.
3. Eligibility state is derived from blocked reasons. Provider- or
   repo-authoritative candidates that claim `queue_eligible` must surface
   a `merge_queue_entry`. Local-estimate candidates may omit the entry.
4. Merge-queue entries under provider or repo-policy authority must
   report `queue_position` when `queued`. Local-estimate entries must not
   claim an authoritative position; `authoritative_position` and
   `local_estimate_only` reflect the queue's actual authority.
5. Every command must emit an audit event when executed and use a closed
   command class. `publish_to_provider` must declare a provider publish
   posture from the closed vocabulary. Each command tracks blocked
   reasons and an `actionable` flag derived from them.
6. Support/export packets must include the candidate id, command refs,
   `support_export` and `cli_headless_entry` consumer surfaces, both
   landing/merge-queue schema refs, and an `eligibility_snapshot` that
   mirrors the candidate state. Raw URLs and raw provider payloads are
   never exported through this packet, and support exports can reopen the
   landing context via the reopen command id.
7. Stale base, stale checks, approval invalidation, policy blocks, and
   environment/review-pack/worktree drift each contribute their own
   invalidation reason; queue readiness goes stale when any of them
   change materially.

## Command-graph operations

| Class | Purpose |
| --- | --- |
| `enqueue` | Request that the provider or policy queue admit the candidate. |
| `dequeue` | Withdraw the candidate from the queue without landing. |
| `approve` | Record an approving review intent against the candidate. |
| `request_changes` | Record a change-request review intent. |
| `rerun_pipeline` | Re-run a required check evidence pipeline. |
| `publish_to_provider` | Mint or progress a provider publish (declares posture). |
| `refresh_provider_overlay` | Pull a fresh provider overlay snapshot. |
| `mark_review_pack_landed` | Record that the matching review pack landed. |

All commands are inert objects. Executing them remains the responsibility
of the command graph layer and the provider-publish lane; this packet
only describes intent, audit posture, blocked reasons, and the exact
target identity.

## Fixtures

| Fixture | Coverage |
| --- | --- |
| `provider_authoritative_mergeable_queued.json` | Provider-authoritative candidate is mergeable, queue eligible, and queued. |
| `stale_base_invalidates_queue_entry.json` | Provider queue holds the candidate but stale base blocks landing; eligibility drops to `queue_not_eligible`. |
| `local_estimate_no_provider_overlay.json` | Local estimate only; queue state projects as `not_queued`; refresh provider overlay before claiming eligibility. |
| `policy_blocked_with_invalidated_approval.json` | Repo policy queue removed the candidate after approval invalidation and a stale required check; truths surface independently. |

## Consumer path

`LandingCandidatePacket::from_workspace_packet` consumes the existing
`ReviewWorkspaceBetaPacket` and a `LandingCandidateInput`, then
materializes the landing candidate, optional merge-queue entry, command
records, support/export packet, and inspection row.
`project_landing_candidate_packet` parses a materialized JSON packet into
`LandingCandidateProjection`, the compact CLI/headless and inspector
projection.

Browser handoff and reopen paths use the workspace-level browser handoff
record and the landing support-export reopen command — the candidate
identity and queue context never get dropped on a generic provider page.
