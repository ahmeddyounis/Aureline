# Checks-Summary Card Fixtures

These fixtures are valid, export-safe checks-summary card packets that exercise the
check-class labeling, evidence-identity, and narrowing behavior the canonical
support export keeps green. Each one keeps the trust-review and consumer-projection
invariants satisfied, covers every check class (`required`, `optional`, `skipped`,
`suppressed`, `timed_out`, `stale`, `not_evaluated_here`) so all seven are
distinguishable from the card alone, keeps every log/artifact/annotation link
anchored to its originating review and check identity, and keeps proof freshness
valid — the difference is which states are narrowed and why.

## provider_stale_local_continue.json

A provider-backed pull request whose provider truth has gone `provider_stale`. The
card never collapses into one pass/fail number (`presents_single_verdict: false`) —
the stale provider is **not** allowed to hide the richer per-check evidence — but it
preserves a `local_continue_fallback` and offers `continue_local_review` so ordinary
triage never forces raw-provider navigation.

## timed_out_and_not_evaluated.json

A `provider_unreachable` card whose integration check `timed_out` and whose
deploy-preview is `not_evaluated_here`. It carries an explicit
`browser_handoff_boundary` and a `local_continue_fallback`, keeps the timed-out and
not-evaluated states visibly distinct from a plain pass, and preserves cached
log/annotation evidence so review can continue while the provider is unreachable.
