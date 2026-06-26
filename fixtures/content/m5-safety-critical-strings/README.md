# Safety-Critical String Catalog Fixtures

These fixtures are valid, export-safe safety-critical string catalog packets. They
are minted from the same seed builder as the canonical support export by
`aureline_shell_m5_safety_critical_strings`, and each one passes every validation
invariant. They exercise the two parity properties the catalog must keep green:
locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every reference template is rewritten
into a pseudo-localized form (human text runs are wrapped in `⟦ ⟧` locale markers),
while every message id, term id, machine token, variable name, and `{term:...}` /
`{var:...}` placeholder stays byte-for-byte identical. The `reference_locale` flips
from `en` to `qps-ploc`. Demonstrates that human prose localizes freely while the
machine-facing identity — the part the product, exports, and screen readers key
off — never moves, so a translation can never fork the meaning of a state.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The terms and messages are
identical; only the catalog id and the release/mirror refs differ. Demonstrates that
the catalog survives an offline mirror with its controlled terms and message
identities intact.
