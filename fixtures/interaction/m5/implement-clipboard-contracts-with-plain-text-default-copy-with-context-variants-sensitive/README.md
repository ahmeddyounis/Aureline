# M5 clipboard-contract fixtures

## stale_proof_forces_label.json

A drill fixture for the clipboard-contract packet. It keeps the full seeded
record set — including the clean plain-text-default baselines (editor source,
preview command id, docs relative path) and one record for each non-default
resolution (notebook diagnostic → copy-with-context, data/API permalink →
copy-with-context, review evidence ref → rich-with-plain-fallback context
variant, runtime support link → sensitive label, data/API fingerprint → sensitive
label, editor private path → relativize/redact, docs rich-only → reject,
provider-linked companion permalink → copy-with-context with imported proof) — and
adds one extra drill record.

The drill record (`clipboard:docs:stale-proof:0001`) is a docs relative-path copy
with no bundled context and non-sensitive content. On its own it would qualify for
`plain_text_default_copy`. But its copy proof has aged outside the freshness
window (`proof_currency` is `stale_expired`), so the `stale_or_missing_copy_proof`
trigger fires, the required floor rises to `copy_with_context_variant`, and the
record correctly resolves to an explicit copy-with-context variant with a precise
`context_label`. The fixture demonstrates that stale evidence — not just bundled
context or sensitivity — forces a copy off the silent default lane, and that the
record stays valid because it records the trigger and the matching non-default
resolution rather than pushing silently.

The fixture validates against
`schemas/interaction/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive.schema.json`
and shares its seeded record set with the checked support export at
`artifacts/interaction/m5/implement-clipboard-contracts-with-plain-text-default-copy-with-context-variants-sensitive/support_export.json`.
