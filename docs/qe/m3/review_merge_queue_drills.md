# Review merge-queue and browser-handoff drills

QE reviewer doc for the M3 merge-queue and browser-handoff audit
corpus. Pairs with:

- the audit fixtures at
  [`fixtures/review/m3/merge_queue_audit/`](../../../fixtures/review/m3/merge_queue_audit/),
- the audit schema at
  [`schemas/review/merge_queue_audit_case.schema.json`](../../../schemas/review/merge_queue_audit_case.schema.json),
- the audit validator at
  [`ci/check_merge_queue_audit_corpus.py`](../../../ci/check_merge_queue_audit_corpus.py),
- the audit report at
  [`artifacts/review/m3/merge_queue_audit_report.md`](../../../artifacts/review/m3/merge_queue_audit_report.md),
- the provider staleness matrix at
  [`artifacts/review/m3/provider_staleness_matrix.json`](../../../artifacts/review/m3/provider_staleness_matrix.json),
- the landing-candidate beta doc at
  [`docs/review/m3/merge_queue_beta.md`](../../review/m3/merge_queue_beta.md),
- the beta review-workspace doc at
  [`docs/review/m3/review_workspace_beta.md`](../../review/m3/review_workspace_beta.md).

The drills here keep `queue OK` from becoming a vague green umbrella.
Each drill replays the relevant provider-staleness, expired-auth,
stale-base, check-invalidation, parent-stack, queue-policy,
local-CI-parity, or browser-handoff scenario and verifies that:

1. Drift between provider state, local drafts, and queue eligibility
   is surfaced as a labeled drift state.
2. Landing and queue actions advertise command-graph preview and emit
   audit events. Provider mutations cannot pass the audit unattributed.
3. Local-draft comments and review notes survive provider refresh
   failure, queue revalidation, and browser handoff without being
   misrepresented as provider-synced.
4. Stale base/head invalidation is never hidden behind a collapsed
   status.
5. Browser handoff and reopen either return the exact reviewed
   provider object or a truthful placeholder; raw URLs and raw
   provider payloads never leak into support exports.
6. Support/export packets reconstruct review identity, provider
   freshness, queue eligibility reason, and local-draft state.

## Drill index

| Drill | Scenario | Anchor fixture | Labeled drift state | Expected eligibility |
| --- | --- | --- | --- | --- |
| `drill:provider-outage-preserves-local-drafts` | `provider_outage` | `provider_outage_local_drafts_preserved.json` | `provider_freshness_drift_labeled` | `queue_not_eligible` |
| `drill:expired-auth-blocks-publish-mint-only` | `expired_auth` | `expired_auth_blocks_publish.json` | `provider_freshness_drift_labeled` | `queue_not_eligible` |
| `drill:stale-base-head-invalidates-queue-entry` | `stale_base_head` | `stale_base_head_invalidates_queue.json` | `queue_eligibility_drift_labeled` | `queue_not_eligible` |
| `drill:check-invalidation-blocks-landing` | `check_invalidation` | `check_invalidation_blocks_landing.json` | `queue_eligibility_drift_labeled` | `queue_not_eligible` |
| `drill:parent-stack-blockage-holds-queue` | `parent_stack_blockage` | `parent_stack_blockage_holds_queue.json` | `parent_stack_drift_labeled` | `queue_not_eligible` |
| `drill:queue-policy-change-requires-re-review` | `queue_policy_change` | `queue_policy_change_requires_re_review.json` | `queue_policy_drift_labeled` | `queue_not_eligible` |
| `drill:local-ci-parity-disagreement-blocks-landing` | `local_ci_parity_disagreement` | `local_ci_parity_disagreement_blocks_landing.json` | `local_ci_parity_drift_labeled` | `queue_not_eligible` |
| `drill:browser-handoff-open-provider-exact-object` | `browser_handoff_open_provider` | `browser_handoff_open_provider_returns_exact_object.json` | `no_drift_labeled` | `queue_eligible` |
| `drill:browser-handoff-reopen-truthful-placeholder` | `browser_handoff_reopen_return` | `browser_handoff_reopen_returns_truthful_placeholder.json` | `browser_handoff_drift_labeled` | `queue_not_eligible` |

## `drill:provider-outage-preserves-local-drafts`

- Scenario class: `provider_outage`
- Fixture: `fixtures/review/m3/merge_queue_audit/provider_outage_local_drafts_preserved.json`

### Steps

1. Open the landing strip for the queued candidate.
2. Trigger a provider overlay refresh while the provider returns no
   response (`provider_outage_user_must_retry`).
3. Compose a comment draft and a review-note draft locally.
4. Inspect the merge-queue inspector while the overlay remains
   unavailable.

### Expected assertions

- `provider_freshness_class = provider_outage_user_must_retry`.
- `eligibility_state` drops to `queue_not_eligible`; queue authority is
  no longer claimed.
- `blocked_reasons` contains `provider_overlay_unavailable`.
- `invalidation_reasons` contains `provider_overlay_stale`.
- Both drafts persist locally with
  `misrepresented_as_provider_synced = false`.
- Support export carries `reconstructs_local_draft_state = true`.

## `drill:expired-auth-blocks-publish-mint-only`

- Scenario class: `expired_auth`
- Fixture: `fixtures/review/m3/merge_queue_audit/expired_auth_blocks_publish.json`

### Steps

1. Attempt to publish a local landing candidate to the provider.
2. Verify that publish posture is
   `publish_minted_not_launched`.
3. Verify that the refresh-overlay command is blocked by
   `provider_auth_expired`.

### Expected assertions

- Publish remains an inert intent: posture is
  `publish_minted_not_launched` and `actionable = false`.
- `blocked_reasons` contains `provider_auth_expired` and
  `provider_overlay_unavailable`.
- Local landing-action draft survives with
  `preserved_state_class = local_draft_preserved_pending_publish`.

## `drill:stale-base-head-invalidates-queue-entry`

- Scenario class: `stale_base_head`
- Fixture: `fixtures/review/m3/merge_queue_audit/stale_base_head_invalidates_queue.json`

### Steps

1. Allow the target branch to move ahead of the reviewed base
   revision.
2. Confirm the queue entry transitions to
   `queued_invalidated_by_stale_base`.
3. Attempt `publish_to_provider` and `dequeue`.

### Expected assertions

- `stale_base_state = base_stale_blocks_landing` is visible in the
  audit fixture.
- `invalidation_reasons` contains `stale_base`.
- `expected.stale_base_hidden = false`.
- `publish_to_provider.blocked_reasons` contains
  `base_revision_stale`.
- `dequeue` is actionable; `publish_to_provider` is not.

## `drill:check-invalidation-blocks-landing`

- Scenario class: `check_invalidation`
- Fixture: `fixtures/review/m3/merge_queue_audit/check_invalidation_blocks_landing.json`

### Steps

1. Run a code change that invalidates the existing approval.
2. Observe the required check transition to
   `checks_stale_blocks_landing`.
3. Try to enqueue.

### Expected assertions

- `approval_state = approval_invalidated_by_changes`.
- `checks_freshness_state = checks_stale_blocks_landing`.
- `enqueue` is blocked with `required_check_failed`,
  `required_check_stale`, and `approval_invalidated`.
- Local request-changes draft is preserved through queue
  revalidation.

## `drill:parent-stack-blockage-holds-queue`

- Scenario class: `parent_stack_blockage`
- Fixture: `fixtures/review/m3/merge_queue_audit/parent_stack_blockage_holds_queue.json`

### Steps

1. Stack the candidate behind an unmerged parent change.
2. Observe queue state remains `queued` but eligibility drops to
   `queue_not_eligible`.
3. Inspect `parent_stack_state_class = parent_stack_blocks_landing`.

### Expected assertions

- `parent_stack_drift_labeled` is reported as the labeled drift
  state.
- `blocked_reasons` and `invalidation_reasons` both include
  `parent_stack_blocked`.
- `dequeue` is actionable; `publish_to_provider` is blocked.

## `drill:queue-policy-change-requires-re-review`

- Scenario class: `queue_policy_change`
- Fixture: `fixtures/review/m3/merge_queue_audit/queue_policy_change_requires_re_review.json`

### Steps

1. Change the repo merge-queue policy mid-review.
2. Confirm the candidate is dequeued by the provider.
3. Confirm a local approval draft survives.

### Expected assertions

- `queue_policy_state_class = queue_policy_changed_requires_re_review`.
- `policy_block_state = policy_blocked`.
- `enqueue.blocked_reasons` contains `queue_policy_changed`,
  `policy_blocked`, and `approval_missing`.
- Local approval draft remains preserved through queue revalidation
  with `misrepresented_as_provider_synced = false`.

## `drill:local-ci-parity-disagreement-blocks-landing`

- Scenario class: `local_ci_parity_disagreement`
- Fixture: `fixtures/review/m3/merge_queue_audit/local_ci_parity_disagreement_blocks_landing.json`

### Steps

1. Trigger a local review-pack check that disagrees with the CI
   provider overlay on the same required check.
2. Inspect the merge-queue audit while the disagreement persists.
3. Re-run the pipeline to attempt reconvergence.

### Expected assertions

- `local_ci_parity_state_class = local_and_ci_disagree_user_review_required`.
- `blocked_reasons` contains `local_ci_parity_disagreement`.
- `invalidation_reasons` contains `local_ci_parity_disagreement` and
  `checks_stale`.
- `rerun_pipeline` and `dequeue` remain actionable;
  `publish_to_provider` is blocked.

## `drill:browser-handoff-open-provider-exact-object`

- Scenario class: `browser_handoff_open_provider`
- Fixture: `fixtures/review/m3/merge_queue_audit/browser_handoff_open_provider_returns_exact_object.json`

### Steps

1. Launch the typed browser handoff for the landing candidate.
2. Authenticate / consent in the provider browser surface.
3. Return to the product surface and re-inspect the queue.

### Expected assertions

- `handoff_destination_class = code_host_web`.
- `outcome_class = handoff_returned_exact_object`.
- Return anchor refers back to
  `review.landing_candidate.fixture.provider_queued` exactly.
- `labeled_drift_state = no_drift_labeled` is the only green case in
  the corpus.
- Local draft survives the handoff with
  `preserved_state_class = local_draft_preserved_through_browser_handoff`.

## `drill:browser-handoff-reopen-truthful-placeholder`

- Scenario class: `browser_handoff_reopen_return`
- Fixture: `fixtures/review/m3/merge_queue_audit/browser_handoff_reopen_returns_truthful_placeholder.json`

### Steps

1. Launch the typed browser handoff while the queued candidate is
   stale-base-invalidated.
2. After returning, observe that the reviewed object is no longer the
   one the provider currently exposes.
3. Confirm the handoff returns a truthful placeholder rather than
   silently routing to a different live object.

### Expected assertions

- `outcome_class = handoff_returned_truthful_placeholder`.
- Return anchor is the placeholder ref; the candidate id remains the
  reviewed candidate.
- `browser_handoff_drift_labeled` is reported.
- `publish_to_provider` is blocked by `base_revision_stale`;
  `refresh_provider_overlay` and `dequeue` remain actionable.
- `raw_url_export_allowed = false` and
  `raw_provider_payload_export_allowed = false` in the support export.

## How the audit refuses green-under-drift

The validator at
[`ci/check_merge_queue_audit_corpus.py`](../../../ci/check_merge_queue_audit_corpus.py)
rejects any audit case that:

- carries drift in provider freshness or queue eligibility while
  declaring `labeled_drift_state = no_drift_labeled`,
- omits `preview_supported = true` or `emits_audit_event = true` from
  any landing or queue command,
- contains a local draft with
  `misrepresented_as_provider_synced != false`,
- sets `expected.stale_base_hidden != false`,
- declares a `stale_base_state = base_stale_blocks_landing` without
  recording `stale_base` in `invalidation_reasons`,
- omits any of
  `reconstructs_review_identity`,
  `reconstructs_provider_freshness`,
  `reconstructs_queue_eligibility_reason`, or
  `reconstructs_local_draft_state` from the support export.

The corpus, the schema, the validator, the audit report at
[`artifacts/review/m3/merge_queue_audit_report.md`](../../../artifacts/review/m3/merge_queue_audit_report.md),
and the provider staleness matrix at
[`artifacts/review/m3/provider_staleness_matrix.json`](../../../artifacts/review/m3/provider_staleness_matrix.json)
together form the M3 release-evidence gate: queue eligibility and
landing claims cannot promote to stable while the corpus reports any
finding.
