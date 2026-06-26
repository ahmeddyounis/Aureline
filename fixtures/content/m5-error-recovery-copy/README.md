# Error/Recovery Copy Catalog Fixtures

These fixtures are valid, export-safe error/recovery copy catalog packets. They are
minted from the same seed builder as the canonical support export by
`aureline_shell_m5_error_recovery_copy`, and each one passes every validation
invariant. They exercise the two parity properties the catalog must keep green:
locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every recovery-block copy-line
template and recovery-link label is rewritten into a pseudo-localized form (human
text runs are wrapped in `⟦ ⟧` locale markers), while every block id, chip id,
machine token, variable name, and `{chip:...}` / `{var:...}` placeholder stays
byte-for-byte identical. The verb-first next-action label and its verb stay fixed —
they resolve from the controlled verb register, not free prose. The
`reference_locale` flips from `en` to `qps-ploc`. Demonstrates that human prose
localizes freely while the machine-facing identity — the part banners, exports, and
screen readers key off — never moves, so a translation can never fork the meaning of
a failure or a degraded state.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The chips and blocks are
identical; only the catalog id and the release/mirror refs differ. Demonstrates that
the catalog survives an offline mirror with its reason chips and recovery-block
identities intact.
