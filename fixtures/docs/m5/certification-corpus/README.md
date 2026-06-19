# M5 Docs Authoring Certification Corpus

These fixtures are valid, export-safe docs-authoring certification reports that
exercise the automatic narrowing the canonical support export keeps green. Each
one keeps every profile present, the compatibility report and trust-review
invariants satisfied, the certification index consistent, and proof freshness
valid — the difference is which profiles are narrowed or blocked and why. They are
regenerated with:

```sh
cargo run -p aureline-docs --bin aureline_docs_authoring_certification -- fixture <name>
```

## mirror_offline_narrows_recall.json

The `mirrored` and `pinned_pack` profiles drop their source/version/freshness
truth gate after the pinned, signed mirror goes offline, so the report narrows
both from Stable to Beta with a `narrowed_to_qualified` verdict. The profiles stay
promotion-permitting at Beta — the downgrade narrows the claim, it does not hide
the profile — and the derived waiver-and-downgrade log gains two `auto_downgrade`
entries.

## unsafe_preview_blocks_handoff.json

The `browser_handoff` profile loses its safe rendered-preview boundaries, so the
report blocks it from promotion with a `blocked_underqualified` verdict and a
`held` qualification. `promotion_blockers` reports the profile and promotion must
fail until preview is re-sanitized. The profile stays visible (labeled, not
hidden); the desktop, mirrored, cached, pinned-pack, and extension-owned profiles
remain certified.
