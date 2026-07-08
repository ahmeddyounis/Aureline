# Merge-Readiness Panel Fixtures

These fixtures are valid, export-safe merge-readiness panel packets that exercise the
governance distinction, queue-result-authority derivation, and queue-block truth the
canonical support export keeps green. Each one keeps the trust-review and
consumer-projection invariants satisfied, covers every governance kind
(`provider_managed`, `repo_policy_managed`, `aureline_local_estimate`) so all three
are distinguishable from the panel alone, keeps every blocked reason / stale-base note
/ approval-recomputation note / stack-blocking note explicit, and keeps proof freshness
valid — the difference is which states are narrowed and why.

## provider_stale_local_continue.json

A three-panel lane where the provider-managed queue truth has gone `provider_stale`
and a local-only-continuation estimate is degraded. Neither panel claims to be
authoritative while its truth is degraded; each preserves a `local_continue_fallback`
and offers `continue_local_review`, so ordinary triage never forces raw-provider
navigation. A repo-policy-managed panel remains authoritative and ready alongside
them.

## stack_blocked_and_approval_recompute.json

A three-panel lane covering the two heaviest block reasons: a `blocked_on_stack_parent`
panel whose `stack_parent_blocked` chip carries an explicit blocking note, a
`blocked_on_approval_recomputation` panel whose recomputation note explains the reset,
and an `aureline_local_estimate` panel whose provider is `provider_unreachable` — it
carries an explicit `browser_handoff_boundary` and a `local_continue_fallback`. Every
blocked state keeps its reason explicit rather than collapsing into a generic warning.
