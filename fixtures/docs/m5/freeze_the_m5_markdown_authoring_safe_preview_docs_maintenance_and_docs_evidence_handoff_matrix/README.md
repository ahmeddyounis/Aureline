# M5 Docs-Authoring Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every surface
present, trust-review, consumer-projection, and release-posture invariants
satisfied, and proof freshness valid — the difference is which surfaces are
narrowed and why.

## unsafe_preview_narrowed.json

The CommonMark preview surface is narrowed to Beta because a document carried raw
embedded HTML that the renderer blocked. The preview safety class becomes
`raw_html_blocked` and the `unsafe_preview_blocked` downgrade trigger fires, so the
rendered view stays available only with a blocked-HTML disclosure while the source
escape remains primary. Demonstrates that an unsafe rendered preview narrows the
claim rather than silently rendering unsafe content.

## evidence_handoff_held.json

The docs evidence-handoff surface is held pending upstream browser-companion
handoff-eligibility graduation. Held surfaces do not require evidence packets, use
the `not_applicable` evidence requirement and rollback posture, and keep evidence
`local_only` — no prose-to-release handoff is offered while held. The Markdown
authoring workspace, CommonMark preview, docs-maintenance suggestions, and docs
validation remain Stable.
