# Attributable Rerun / Cancel Action, Execution-Context Reuse, and Side-Effect Review Fixtures

These fixtures are valid, export-safe packets that exercise the attribution,
context-reuse, and side-effect narrowing behavior the canonical support export
keeps green. Each one keeps the trust-review and consumer-projection invariants
satisfied and proof freshness valid — the difference is which states are narrowed
and why.

## context_reuse_stale_blocked.json

A rerun that would reuse a degraded recorded execution context, so the action is
`blocked_context_reuse_stale_review_required`, carries an explicit staleness
label and attention reasons, and queues its external artifact write for a later
drain (`requires_deferred_queue`). Demonstrates that a degraded reused context is
flagged and reviewed rather than silently replayed, and that a non-inert side
effect is acknowledged before the control can fire.

## unknown_control_provider_owned.json

A provider-owned control the contract does not recognise yet, so the action is
`unknown_control_provider_owned` with an `unverified`, `unknown_context_provider_owned`
execution context, carries explicit attention reasons, opens in the provider via
a browser handoff, and acknowledges its unknown side effect with
`requires_browser_handoff`. Demonstrates that an unknown control is never
flattened into rerun or cancel, that an unverified context reuse is not assumed,
and that every action stays attributable.
