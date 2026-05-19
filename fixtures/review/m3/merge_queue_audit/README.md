# Merge-queue and browser-handoff audit corpus

These fixtures exercise the release-evidence audit corpus that sits on
top of the beta review-workspace and landing-candidate packets. Each
case projects a single drift scenario across the same closed
vocabulary so review surfaces, support exports, and merge-queue
inspectors cannot collapse them into one green status.

The companion schema lives at
[`/schemas/review/merge_queue_audit_case.schema.json`](../../../../schemas/review/merge_queue_audit_case.schema.json)
and is validated by
[`/ci/check_merge_queue_audit_corpus.py`](../../../../ci/check_merge_queue_audit_corpus.py).

The corpus is required by:

- [`/docs/qe/m3/review_merge_queue_drills.md`](../../../../docs/qe/m3/review_merge_queue_drills.md)
- [`/artifacts/review/m3/merge_queue_audit_report.md`](../../../../artifacts/review/m3/merge_queue_audit_report.md)
- [`/artifacts/review/m3/provider_staleness_matrix.json`](../../../../artifacts/review/m3/provider_staleness_matrix.json)

| Fixture | Scenario | Labeled drift state |
| --- | --- | --- |
| `provider_outage_local_drafts_preserved.json` | `provider_outage` | `provider_freshness_drift_labeled` |
| `expired_auth_blocks_publish.json` | `expired_auth` | `provider_freshness_drift_labeled` |
| `stale_base_head_invalidates_queue.json` | `stale_base_head` | `queue_eligibility_drift_labeled` |
| `check_invalidation_blocks_landing.json` | `check_invalidation` | `queue_eligibility_drift_labeled` |
| `parent_stack_blockage_holds_queue.json` | `parent_stack_blockage` | `parent_stack_drift_labeled` |
| `queue_policy_change_requires_re_review.json` | `queue_policy_change` | `queue_policy_drift_labeled` |
| `local_ci_parity_disagreement_blocks_landing.json` | `local_ci_parity_disagreement` | `local_ci_parity_drift_labeled` |
| `browser_handoff_open_provider_returns_exact_object.json` | `browser_handoff_open_provider` | `no_drift_labeled` |
| `browser_handoff_reopen_returns_truthful_placeholder.json` | `browser_handoff_reopen_return` | `browser_handoff_drift_labeled` |

Each fixture proves the spec's invariants:

1. Any drift between provider state, local drafts, and queue
   eligibility is surfaced as a labeled state in the corpus instead of
   passing as green.
2. Landing and queue actions cannot pass the audit when they skip
   command-graph preview, lose local-draft state, or hide stale-base
   invalidation.
3. Browser handoff and reopen proofs demonstrate exact-object routing
   or a truthful placeholder for every claimed provider row.
4. Support/export packets reconstruct review identity, provider
   freshness, queue eligibility reason, and local-draft state from the
   resulting packet.
