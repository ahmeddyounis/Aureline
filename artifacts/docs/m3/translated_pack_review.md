# Translated Pack Review

Review date: 2026-05-18

## Scope

This packet covers translated docs, help, glossary, onboarding, guided-tour, and troubleshooting overlays for the beta learnability surfaces. The contract treats translation as a locale overlay on top of source-language pack truth.

## Evidence Reviewed

- `schemas/docs/locale_overlay.schema.json`
- `fixtures/docs/m3/translated_pack_parity/manifest.json`
- `fixtures/docs/m3/translated_pack_parity/surface_projection.json`
- `fixtures/docs/m3/translated_pack_parity/support_export.json`
- `docs/ux/m3/translated_help_and_onboarding_pack_contract.md`

## Findings

- Complete translated rows preserve source revision, overlay revision, citations, command ids, keyboard paths, and scope labels.
- Partial glossary overlays render `Partial translation` and keep `Open in source language`.
- Stale onboarding overlays render `Stale translation` with source-revision skew and stale freshness.
- Guided-tour and troubleshooting rows that cannot render the requested locale fall back to source language with the same source-language action and metadata-only support rows.
- Support export reconstructs the overlay and source revision shown to the user without exporting raw translated or source bodies.

## Verification

```sh
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- validate
cargo test -p aureline-docs --test locale_overlay_beta
```
