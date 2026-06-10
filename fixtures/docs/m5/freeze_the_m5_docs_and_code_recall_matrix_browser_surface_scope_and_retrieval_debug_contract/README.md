# M5 Docs and Code-Recall Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every lane
present, trust-review and consumer-projection invariants satisfied, and proof
freshness valid — the difference is which lanes are narrowed and why.

## mirror_offline_recall_narrowed.json

The docs semantic-recall lane is narrowed to Beta because the pinned, signed
mirror is offline; recall falls back to last-known-good with explicit freshness
and offline labels. Demonstrates the `mirror_offline` and `freshness_expired`
downgrade triggers narrowing a recall claim rather than hiding the lane.

## browser_surface_held.json

The scoped browser-surface lane is held pending upstream browser-companion
handoff-eligibility graduation. Held lanes do not require evidence packets and
use the `not_applicable` rollback posture; no browser handoff is offered while
held. Docs recall, codebase explainers, and the retrieval-debug inspector remain
Stable.
