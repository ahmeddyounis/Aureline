# Attributable Rerun / Cancel Actions, Execution-Context Reuse, and Side-Effect Review

- Packet: `rerun-cancel-review:stable:0001`
- Schema: `schemas/review/ship-attributable-rerun-or-cancel-actions-with-execution-context-reuse-and-side-effect-review.schema.json`
- Support export: `artifacts/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/support_export.json`
- Contract doc: `docs/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review.md`
- Fixtures: `fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/`
- Producer: `aureline_review::current_rerun_cancel_review_export`

## Coverage

- **Attributable rerun / cancel actions** carry the run they act on, the durable
  review anchor, the rerun/cancel control class, the target scope, the
  provider-mode mutation mode, the blocked class, the actor attribution, the
  audit row that will land, and the reviewable effect summary. A known control
  must resolve to its fixed target scope (`rerun_workflow` and `cancel_workflow`
  to `entire_workflow_run`, `rerun_failed_jobs` to `failed_jobs_only`, and so
  on), and `unknown_control_provider_owned` is never flattened into a known
  control. Every action carries a non-empty actor attribution and audit row, so a
  rerun or cancel can never fire unattributably.
- **Execution-context reuse** records, per action, the execution context id, the
  reuse decision (`reuse_identical_context`, `reuse_with_pinned_inputs`,
  `reuse_with_refreshed_secrets`, `fork_new_context`, `provider_default_context`,
  `unknown_context_provider_owned`), and the context freshness. A reused context
  that is `degraded_cached`, `stale`, or `unverified` must carry a non-empty
  staleness label so it is flagged and reviewed rather than silently replayed.
- **Side-effect review** records, per action, the typed side-effect class
  (`no_external_side_effect`, `triggers_deployment`, `triggers_release`,
  `sends_notifications`, `writes_external_artifact`, `mutates_provider_run_state`,
  `consumes_quota_or_cost`, `unknown_side_effect_provider_owned`), the
  acknowledgment requirement, and a disclosure label. Every action carries at
  least one side-effect row, and any non-inert side effect requires an
  acknowledgment stronger than `no_ack_required` before the control can fire.
- **Provider-mode mutation modes** are limited to the three that reach upstream
  provider state — `publish_now` (cites an approval ticket), `open_in_provider`
  (cites a browser-handoff packet), and `deferred_publish` (cites a publish-later
  queue item). The local-only `local_draft` mode is intentionally absent because
  rerun and cancel controls cannot remain local.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: control class and target scope are explicit and never
overstated; every mutating action is attributable and records an audit row;
execution-context reuse is explicit and a stale reuse is flagged rather than
hidden; side effects are reviewed before invocation and a non-inert side effect
requires acknowledgment; no action creates hidden write scope; the mutation mode
cites the grant it depends on; downgrade narrows the claim instead of hiding the
lane; and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`action_attribution_missing`, `context_reuse_stale`, `side_effect_unreviewed`,
`run_control_authority_revoked`, `trust_narrowing`, `scope_expansion_unqualified`,
and `upstream_dependency_narrowed`.

## Boundary

Raw run, log, and artifact bodies, raw provider payloads, raw URLs, raw absolute
paths, raw author email addresses, credentials, and live provider responses never
cross this boundary. The packet carries only metadata, control classes, target
scopes, mutation modes, reuse decisions, context freshness, side-effect classes,
acknowledgment requirements, reviewable effect summaries, and contract
references. Every rerun, cancel, context reuse, and side effect stays
attributable and reviewable before any upstream effect fires.
