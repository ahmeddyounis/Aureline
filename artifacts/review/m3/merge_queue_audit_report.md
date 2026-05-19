# Merge-queue and browser-handoff audit report

This report is the deterministic, support- and partner-facing summary
of the M3 merge-queue/browser-handoff audit corpus. It is generated
from the checked-in audit fixtures under
[`fixtures/review/m3/merge_queue_audit/`](../../../fixtures/review/m3/merge_queue_audit/)
and validated against
[`schemas/review/merge_queue_audit_case.schema.json`](../../../schemas/review/merge_queue_audit_case.schema.json)
by
[`ci/check_merge_queue_audit_corpus.py`](../../../ci/check_merge_queue_audit_corpus.py).

It pairs with:

- the provider staleness matrix at
  [`artifacts/review/m3/provider_staleness_matrix.json`](provider_staleness_matrix.json),
- the reviewer drills doc at
  [`docs/qe/m3/review_merge_queue_drills.md`](../../../docs/qe/m3/review_merge_queue_drills.md),
- the landing-candidate beta doc at
  [`docs/review/m3/merge_queue_beta.md`](../../../docs/review/m3/merge_queue_beta.md),
- the local-CI parity report at
  [`artifacts/review/m3/local_ci_parity_report.md`](local_ci_parity_report.md).

The corpus exists to keep the merge-queue, browser-handoff, and
landing-candidate lanes from collapsing into a single green status:
provider staleness, expired auth, stale base/head, check
invalidation, parent-stack blockage, queue-policy change, local-CI
parity disagreement, and browser handoff/reopen each appear as a
separately labeled drift state instead of being swept under
`landing OK`.

## 1 Audit cases

| Fixture | scenario_class | labeled_drift_state | eligibility_state |
| --- | --- | --- | --- |
| `provider_outage_local_drafts_preserved.json` | `provider_outage` | `provider_freshness_drift_labeled` | `queue_not_eligible` |
| `expired_auth_blocks_publish.json` | `expired_auth` | `provider_freshness_drift_labeled` | `queue_not_eligible` |
| `stale_base_head_invalidates_queue.json` | `stale_base_head` | `queue_eligibility_drift_labeled` | `queue_not_eligible` |
| `check_invalidation_blocks_landing.json` | `check_invalidation` | `queue_eligibility_drift_labeled` | `queue_not_eligible` |
| `parent_stack_blockage_holds_queue.json` | `parent_stack_blockage` | `parent_stack_drift_labeled` | `queue_not_eligible` |
| `queue_policy_change_requires_re_review.json` | `queue_policy_change` | `queue_policy_drift_labeled` | `queue_not_eligible` |
| `local_ci_parity_disagreement_blocks_landing.json` | `local_ci_parity_disagreement` | `local_ci_parity_drift_labeled` | `queue_not_eligible` |
| `browser_handoff_open_provider_returns_exact_object.json` | `browser_handoff_open_provider` | `no_drift_labeled` | `queue_eligible` |
| `browser_handoff_reopen_returns_truthful_placeholder.json` | `browser_handoff_reopen_return` | `browser_handoff_drift_labeled` | `queue_not_eligible` |

The single `no_drift_labeled` row is the deliberate green baseline:
provider reachable, base current, checks current, approval current,
policy clear, handoff returns the exact reviewed object, local draft
preserved. Every other row downgrades the candidate explicitly.

## 2 Landing-action audit coverage

| Audit class | Fixtures asserting it |
| --- | --- |
| `enqueue_audited` | `provider_outage_local_drafts_preserved.json`, `check_invalidation_blocks_landing.json`, `queue_policy_change_requires_re_review.json` |
| `dequeue_audited` | `stale_base_head_invalidates_queue.json`, `parent_stack_blockage_holds_queue.json`, `local_ci_parity_disagreement_blocks_landing.json`, `browser_handoff_reopen_returns_truthful_placeholder.json` |
| `request_changes_audited` | `check_invalidation_blocks_landing.json`, `queue_policy_change_requires_re_review.json` |
| `rerun_pipeline_audited` | `stale_base_head_invalidates_queue.json`, `check_invalidation_blocks_landing.json`, `local_ci_parity_disagreement_blocks_landing.json` |
| `refresh_provider_overlay_audited` | `provider_outage_local_drafts_preserved.json`, `expired_auth_blocks_publish.json`, `queue_policy_change_requires_re_review.json`, `browser_handoff_open_provider_returns_exact_object.json`, `browser_handoff_reopen_returns_truthful_placeholder.json` |
| `publish_to_provider_minted_not_launched` | `expired_auth_blocks_publish.json`, `stale_base_head_invalidates_queue.json`, `parent_stack_blockage_holds_queue.json`, `local_ci_parity_disagreement_blocks_landing.json`, `browser_handoff_open_provider_returns_exact_object.json`, `browser_handoff_reopen_returns_truthful_placeholder.json` |

Every command records `preview_supported = true` and
`emits_audit_event = true`. Provider mutations stay attributable; no
landing action can pass the audit by skipping command-graph preview.

## 3 Local-draft preservation

Every fixture carries at least one local draft and asserts:

- `misrepresented_as_provider_synced = false` on each draft, and
- `expected.local_draft_preserved = true` plus
  `expected.local_draft_misrepresented_as_provider_synced = false`.

Drafts survive provider refresh failure (`provider_outage`,
`expired_auth`), queue revalidation (`stale_base_head`,
`check_invalidation`, `queue_policy_change`,
`parent_stack_blockage`, `local_ci_parity_disagreement`), and browser
handoff/reopen (`browser_handoff_open_provider`,
`browser_handoff_reopen_return`).

## 4 Browser handoff and reopen

The two browser-handoff fixtures prove the spec's exact-object /
truthful-placeholder rule:

- `browser_handoff_open_provider_returns_exact_object.json` returns
  the same `review.landing_candidate.fixture.provider_queued` object
  identity on return (`handoff_returned_exact_object`).
- `browser_handoff_reopen_returns_truthful_placeholder.json` returns
  `review.landing_candidate.fixture.stale_base.placeholder` and
  records `handoff_returned_truthful_placeholder` because the queued
  candidate moved under a stale base.

Both fixtures keep `reversible_handoff = true` and
`raw_url_export_allowed = false`.

## 5 Support/export reconstruction

Every audit case's `support_export` packet asserts:

- `reconstructs_review_identity = true`
- `reconstructs_provider_freshness = true`
- `reconstructs_queue_eligibility_reason = true`
- `reconstructs_local_draft_state = true`
- `raw_url_export_allowed = false`
- `raw_provider_payload_export_allowed = false`
- `raw_comment_body_export_allowed = false`

so any drill report or partner packet can replay the queue eligibility
reason, the provider freshness class, the surviving local-draft state,
and the typed handoff identity without leaking raw provider URLs,
payloads, or comment bodies.

## 6 Acceptance summary

- **Drift is labeled, never green.** No fixture combines a drifted
  axis with `expected.passes_green_under_drift = true`. Every drift
  scenario maps to a non-`no_drift_labeled` `labeled_drift_state`.
- **Landing/queue actions cannot skip preview, lose local-draft state,
  or hide stale-base invalidation.** Schema and validator enforce
  preview/audit posture, draft preservation, and stale-base reason
  surfacing.
- **Browser handoff and reopen route truthfully.** Exact-object or
  truthful-placeholder routing is required and validated.
- **Support exports rebuild every axis.** Identity, freshness,
  eligibility reason, and draft state are reconstructable from the
  packet alone.

## 7 Wiring

- Schema — [`schemas/review/merge_queue_audit_case.schema.json`](../../../schemas/review/merge_queue_audit_case.schema.json)
- Fixtures — [`fixtures/review/m3/merge_queue_audit/`](../../../fixtures/review/m3/merge_queue_audit/)
- Validator — [`ci/check_merge_queue_audit_corpus.py`](../../../ci/check_merge_queue_audit_corpus.py)
- Provider staleness matrix — [`artifacts/review/m3/provider_staleness_matrix.json`](provider_staleness_matrix.json)
- Reviewer drills doc — [`docs/qe/m3/review_merge_queue_drills.md`](../../../docs/qe/m3/review_merge_queue_drills.md)
- Landing-candidate beta doc — [`docs/review/m3/merge_queue_beta.md`](../../../docs/review/m3/merge_queue_beta.md)
