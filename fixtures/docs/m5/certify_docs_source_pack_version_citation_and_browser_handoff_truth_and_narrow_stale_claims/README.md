# M5 Documentation-Claim Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
auto-narrow behavior the canonical support export keeps green. Each one keeps
every claimed profile present, the compatibility-report and trust-review
invariants satisfied, every documentation-evidence class covered, and proof
freshness valid — the difference is which profiles are narrowed, retest-pending,
or blocked, and why. They are regenerated with:

```sh
cargo run -p aureline-docs --bin aureline_docs_claim_certification -- fixture <name>
```

## source_class_evidence_stale_retest_pending.json

Every profile depends on the docs source-class evidence, so when that evidence
exceeds the freshness SLO every profile is narrowed to Preview and marked
`retest_pending`, with the `source_class_evidence_stale` trigger recorded. The
profiles stay present and labeled — `retest_pending_profiles` reports all five
and `publication_blockers` is empty, because retest-pending narrows the public
claim rather than hiding the profile or hard-failing publication.

## browser_handoff_evidence_stale_blocks_publication.json

The browser-handoff-bearing profiles (`docs_browser`, `help_about`,
`ai_explanation`, and `support_export`) are held and `blocked_underqualified`
after their browser-handoff evidence went stale; no handoff is offered until it
is re-proven, because a handoff must not silently share context or impersonate a
governed docs surface. `publication_blockers` reports those four profiles and
publication must fail until they are re-certified. The `onboarding_learning`
profile does not touch browser handoff, so it stays certified and visible.
