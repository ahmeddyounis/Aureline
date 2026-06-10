# Browser/Provider Handoff Continuity Fixtures

These fixtures are valid, export-safe packets that exercise the deep-link,
target-identity, truth-freshness, safe-preview, and handoff-action narrowing
behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## stale_truth_blocked.json

A CI-run handoff whose underlying truth is from a prior base, so the freshness is
`stale_prior_truth`, the handoff action is `unsupported_no_continuity` and
`blocked_stale_truth_review_required`, and the handoff carries explicit attention
reasons. Demonstrates that a stale truth narrows the handoff action rather than
jumping to a possibly-wrong provider state.

## untrusted_target_blocked.json

An artifact deep-link handoff whose target host is untrusted and external, so the
trust class is `untrusted_external`, the safe-preview is `unsafe_preview_blocked`,
the handoff action is `unsupported_no_continuity` and `blocked_untrusted_target`,
and the handoff carries explicit attention reasons. Demonstrates that an untrusted
target identity is disclosed rather than assumed safe, and that an unsafe-preview
target holds its handoff action.
