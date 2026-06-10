# Merge-Queue Readiness, Stale-Base Invalidation, and Approval-Recomputation Flows

This document is the contract for the M5 packet that keeps a merge queue honest
as the base advances under queued changes. The packet is the canonical M5 control
source for this lane: the merge-queue panel, review-workspace header,
CLI/headless output, diagnostics, Help/About, and support exports ingest the
checked-in packet rather than cloning status text.

- Record kind: `merge_queue_readiness_stale_base_invalidation_and_approval_recomputation`
- Schema: [`schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`](../../../schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json)
- Canonical support export: [`artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/support_export.json`](../../../artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/support_export.json)
- Summary artifact: [`artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md`](../../../artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md)
- Fixtures: [`fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/`](../../../fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/)
- Producer: `aureline_review::current_merge_queue_readiness_export`

## Pillars

### Merge-queue readiness

Each `readiness_entries[]` row binds a queue entry to its `target_identity_label`
(what is queued to land), a `durable_anchor_id` that ties the entry to its review
anchor, a `queue_position`, a `base_freshness` class, a `readiness_verdict`, a
`mutation_authority` class, and human-readable `required_checks_summary` and
`required_approvals_summary`. A non-`ready` verdict must carry at least one entry
in `blocking_reasons`, so readiness is never overstated relative to the reasons a
change cannot land.

| Field | Source contract |
| --- | --- |
| queue entry, position | [`schemas/review/merge_queue_entry.schema.json`](../../../schemas/review/merge_queue_entry.schema.json) |
| landing eligibility | [`schemas/review/landing_candidate.schema.json`](../../../schemas/review/landing_candidate.schema.json) |
| anchor stability / approval invalidation | [`schemas/review/review_stabilization.schema.json`](../../../schemas/review/review_stabilization.schema.json) |
| check / pipeline run | [`schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json) |

### Stale-base invalidation

Each `stale_base_invalidations[]` row records an `entry_id`, a
`base_advance_label`, an `invalidation_action`, whether `recompute_required`, and
an `invalidation_label`. The action is one of `requeue_after_rerun`,
`auto_rebase_proposed`, `eject_to_author`, `hold_for_resolution`, or
`no_action_needed`. Any action other than `no_action_needed` must carry a
non-empty `invalidation_label`. Every readiness entry that is
`blocked_on_stale_base` or whose `base_freshness` is `stale_base` or `diverged`
must appear in at least one stale-base invalidation row, so no entry is silently
kept green after the base advances.

### Approval recomputation

Each `approval_recomputations[]` row records an `entry_id`, the `trigger`
(`diff_changed`, `base_advanced`, `requeue_requested`, `reviewer_removed`,
`policy_changed`), the `outcome`, the `approvals_before` and `approvals_after`
counts, and a `recomputation_label`. The outcome is one of `retained`,
`reset_full`, `invalidated_partial`, `re_requested`, or `not_applicable`. An
outcome that lowers or re-requests approvals (`reset_full`, `invalidated_partial`,
`re_requested`) must carry a non-empty `recomputation_label`, and an invalidating
outcome (`reset_full`, `invalidated_partial`) must never report more approvals
after than before. Every readiness `entry_id` must appear in at least one
approval-recomputation row, so no entry points at approvals that were never
recomputed.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate:

- `readiness_verdict_explicit` and `readiness_never_overstated` — verdicts are
  explicit and non-ready entries always carry their blocking reasons.
- `stale_base_invalidation_labeled_not_hidden` and
  `approval_recompute_labeled_not_hidden` — invalidation and recomputation are
  labeled rather than silently absorbed.
- `approval_recomputes_on_base_or_diff_change` and `base_freshness_explicit` —
  approvals recompute when the base or diff changes, and base freshness is shown.
- `target_identity_explicit`, `no_hidden_write_scope`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade
triggers are `proof_stale`, `policy_blocked`, `stale_base_unlabeled`,
`approval_recompute_unlabeled`, `readiness_overstated`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/)
show a stale-base entry ejected to its author with a full approval reset, and an
offline queue whose approval recomputation could not run; both remain valid
because narrowing is explicit, not hidden.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live provider responses never cross this boundary. The packet
carries only metadata, readiness verdicts, invalidation outcomes, approval
recomputation outcomes, and contract references. Every merge-queue, requeue,
rerun, or eject action stays read-only or attributable and reviewable.
