# Boundary Wording Catalog Fixtures

These fixtures are valid, export-safe boundary-wording catalog packets. They are
minted from the same seed builder as the canonical support export by
`aureline_shell_m5_boundary_wording`, and each one passes every validation invariant
(including the cross-surface copy-parity lint). They exercise the two parity
properties the catalog must keep green: locale neutrality and offline-mirror identity.

## localized_overlay.json

A localized overlay of the canonical catalog. Every human-prose field — entry
canonical text, implication disclosures, and alternative disclosures — is rewritten
into a pseudo-localized form (text runs are wrapped in `⟦ ⟧` locale markers), while
every entry id, concept id, boundary term, surface, actual posture, implication
posture, support metadata ref, alternative reference ref, and source ref stays
byte-for-byte identical. The `reference_locale` flips from `en` to `qps-ploc`.
Demonstrates that human prose localizes freely while the machine-facing boundary
facts — the part settings, onboarding, marketplace, help/About, release notes, and
account/upgrade prompts key off — never move, so a translation can never fork a
concept id or a support ref into a different boundary claim.

## offline_mirror.json

An offline-mirror variant of the canonical catalog. The entries are identical; only
the catalog id and the release/mirror ref differ. Demonstrates that the catalog
survives an offline mirror with its boundary claims and provenance intact.
