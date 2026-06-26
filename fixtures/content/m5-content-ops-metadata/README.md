# Content-Ops Metadata Catalog Fixtures

These fixtures are valid, export-safe content-ops metadata catalog packets. They are
minted from the same seed builder as the canonical support export by
`aureline_shell_m5_content_ops_metadata`, and each one passes every validation
invariant. They exercise the two parity properties the catalog must keep green:
locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every human-prose field — entry
canonical text and placeholder semantic/fallback prose — is rewritten into a
pseudo-localized form (text runs are wrapped in `⟦ ⟧` locale markers), while every
entry id, machine field name, command ref, source ref, placeholder literal and token
id, glossary/plural refs, locale tags, capture posture, and caption-sync state stays
byte-for-byte identical. The `reference_locale` flips from `en` to `qps-ploc`.
Demonstrates that human prose localizes freely while the machine-facing identity —
the part exports, report columns, screenshot pipelines, and screen readers key off —
never moves, so a translation can never fork a command id, an export field id, or a
placeholder token into machine identity.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The entries are identical; only
the catalog id and the release/mirror ref differ. Demonstrates that the catalog
survives an offline mirror with its entry identities and provenance intact.
