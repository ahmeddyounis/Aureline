# Translated Pack Parity Fixtures

This corpus proves that translated docs, help, glossary, onboarding, guided-tour, and troubleshooting packs preserve source revision, overlay revision, freshness, skew, citations, command ids, keyboard paths, scope labels, mirror/offline posture, and the `Open in source language` action.

Regenerate with:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- manifest > fixtures/docs/m3/translated_pack_parity/manifest.json
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- surfaces > fixtures/docs/m3/translated_pack_parity/surface_projection.json
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- support-export > fixtures/docs/m3/translated_pack_parity/support_export.json
```

Validate with:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_locale_overlay_beta -- validate
cargo test -p aureline-docs --test locale_overlay_beta
```
