# Merge-Queue Readiness, Stale-Base Invalidation, and Approval Recomputation

- Packet: `merge-queue-readiness:stable:0001`
- Schema: `schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`
- Support export: `artifacts/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/support_export.json`
- Contract doc: `docs/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows.md`
- Fixtures: `fixtures/review/m5/add_merge_queue_readiness_stale_base_invalidation_and_approval_recomputation_flows/`
- Producer: `aureline_review::current_merge_queue_readiness_export`

## Coverage

- **Merge-queue readiness** rows carry the target identity (what is queued to
  land), the durable review anchor id, the queue position, the base-freshness
  class, the readiness verdict, and the mutation authority. A non-ready verdict
  (`blocked_on_checks`, `blocked_on_approvals`, `blocked_on_stale_base`,
  `blocked_on_policy`, `holding`) must carry at least one blocking reason, so the
  queue never overstates that a change is ready.
- **Stale-base invalidation** rows record, per entry, how a base advance
  invalidated the entry's readiness. The invalidation action (`requeue_after_rerun`,
  `auto_rebase_proposed`, `eject_to_author`, `hold_for_resolution`,
  `no_action_needed`) is labeled with a non-empty invalidation label whenever the
  action does anything, and `recompute_required` records whether checks must rerun.
- **Approval recomputation** rows record, per entry, how approvals were recomputed
  when the diff or base changed. The outcome (`retained`, `reset_full`,
  `invalidated_partial`, `re_requested`, `not_applicable`) is labeled when it
  lowers or re-requests approvals, and an invalidating outcome never increases the
  approval count.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: readiness verdicts are explicit and never overstated;
stale-base invalidation and approval recomputation are labeled rather than hidden;
approvals recompute on base or diff change; base freshness and target identity are
explicit; no merge-queue surface creates hidden write scope; downgrade narrows the
claim instead of hiding the lane; and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`stale_base_unlabeled`, `approval_recompute_unlabeled`, `readiness_overstated`,
`trust_narrowing`, `scope_expansion_unqualified`, and
`upstream_dependency_narrowed`.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live provider responses never cross this boundary. The packet
carries only metadata, readiness verdicts, invalidation outcomes, approval
recomputation outcomes, and contract references. Every merge-queue, requeue,
rerun, or eject action stays read-only or attributable and reviewable.
