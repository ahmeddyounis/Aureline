# M5 Docs and Code-Understanding Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
downgrade behavior the canonical support export keeps green. Each one keeps every
certified surface present, compatibility-report and trust-review invariants
satisfied, and proof freshness valid — the difference is which surfaces are
narrowed or blocked and why. They are regenerated with:

```sh
cargo run -p aureline-docs --bin aureline_docs_certification -- fixture <name>
```

## recall_freshness_expired_narrows.json

The `docs_pack_recall` and `semantic_recall` lanes are narrowed from Stable to
Beta because the pinned, signed mirror's freshness window expired; recall falls
back to last-known-good with explicit freshness labels. The verdict is
`narrowed_to_qualified`, so the surfaces stay promotion-permitting at Beta — the
downgrade narrows the claim, it does not hide the surface.

## browser_scope_expansion_blocked.json

The `scoped_browser_surface` lane is held and blocked from promotion after an
unqualified scope expansion was detected; no browser handoff is offered while
held. The verdict is `blocked_underqualified` with a `held` qualification, so
`promotion_blockers` reports the lane and promotion must fail until it is
re-qualified. The lane stays visible (labeled, not hidden); docs recall, the
codebase-understanding cards, and the retrieval-debug inspector remain Stable.
