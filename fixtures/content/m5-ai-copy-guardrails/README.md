# AI Copy Guardrail Catalog Fixtures

These fixtures are valid, export-safe AI copy guardrail catalog packets. They are
minted from the same seed builder as the canonical support export by
`aureline_shell_m5_ai_copy_guardrails`, and each one passes every validation
invariant. They exercise the two parity properties the catalog must keep green:
locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every human-prose field — term labels,
reserved meanings, required-context and forbidden-use lines, and forbidden-phrase
rejection reasons — is rewritten into a pseudo-localized form (text runs are wrapped
in `⟦ ⟧` locale markers), while every term id, machine token, concept, domain,
surface, consumer, phrase id, class, lowercase pattern, and replacement ref stays
byte-for-byte identical. The `reference_locale` flips from `en` to `qps-ploc`.
Demonstrates that human prose localizes freely while the machine-facing identity —
the part the lint, exports, and screen readers key off — never moves, so a
translation can never fork the meaning of a proposal, a confidence label, a
validation state, or a forbidden pattern.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The controlled terms and
forbidden phrases are identical; only the catalog id and the release/mirror ref
differ. Demonstrates that the catalog survives an offline mirror with its term and
forbidden-phrase identities intact.
