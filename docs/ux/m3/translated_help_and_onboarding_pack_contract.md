# Translated Help And Onboarding Pack Contract

Translated learnability content is an overlay on top of source-language pack truth. It must not create a second authority for docs, help, glossary, onboarding, guided-tour, or troubleshooting content.

## Runtime Contract

- Every translated row carries `pack_id`, `pack_kind`, owner, source locale, requested locale, effective locale, current source revision, overlay revision, and the source revision the overlay was translated from.
- Every row renders one of the controlled badges: `Translated`, `Partial translation`, `Stale translation`, or `Source-language fallback`.
- Citation anchors, command ids, keyboard path refs, and scope label refs are copied unchanged from the source pack into the overlay metadata.
- Partial, stale, missing, and fallback overlays keep a keyboard-reachable `Open in source language` action.
- Mirror/offline posture is explicit, using local, cached, mirrored, offline, live, or not-installed states. A translated pack may not silently depend on live vendor docs when it claims offline or mirror continuity.
- Support export is metadata-only. It records overlay id, source revision, overlay revision, source-revision basis, coverage state, freshness/skew state, badge class, source-language action id, and integrity refs while omitting raw translated and source bodies.

## Evidence

- Schema: `schemas/docs/locale_overlay.schema.json`
- Runtime model: `crates/aureline-docs/src/locale_overlay/mod.rs`
- Fixtures: `fixtures/docs/m3/translated_pack_parity/`
- Review packet: `artifacts/docs/m3/translated_pack_review.md`

## Verification

```sh
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- validate
cargo test -p aureline-docs --test locale_overlay_beta
```
