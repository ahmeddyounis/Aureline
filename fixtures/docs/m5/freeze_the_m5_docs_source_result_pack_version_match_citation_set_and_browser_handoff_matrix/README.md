# M5 Docs-Contracts Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every governed
object present, the frozen vocabulary set intact, and the trust-review,
consumer-projection, and release-posture invariants satisfied — the difference is
which object is narrowed and why.

## browser_handoff_held.json

The browser-handoff object is held pending upstream browser-companion
handoff-eligibility graduation. Held objects do not require proof packets, use the
`not_applicable` evidence requirement and rollback posture, and keep no proof refs
— no browser handoff is claimed while held. The docs source descriptor, docs
result object, docs-pack manifest, derived-explanation citation set, version-match
state, and stale-example finding remain Stable. Demonstrates that an unproven
handoff object narrows to held rather than claiming an unbacked surface.

## mirror_offline_pack_narrowed.json

The docs-pack manifest is narrowed to Beta because the pinned, signed mirror went
offline. The object keeps all of its declared vocabularies and the
`mirror_offline` downgrade trigger, so mirror/offline truth stays disclosed while
the claim narrows. Demonstrates that a mirror-offline docs pack narrows the claim
rather than silently presenting stale pack content as current.
