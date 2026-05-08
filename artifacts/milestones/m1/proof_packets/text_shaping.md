# Proof packet: text shaping lane

Purpose: anchor proof captures for production text shaping, font fallback, and
glyph-atlas caching in one indexed location.

Canonical sources (non-exhaustive):

- `docs/adr/0002-renderer-text-stack-and-shaping-fallback.md`
- `crates/aureline-text/src/shaping/mod.rs`
- `crates/aureline-render/src/glyph_atlas.rs`
- `fixtures/text/font_fallback_cases/`
- `crates/aureline-shell/src/bootstrap/native_shell.rs` (Start Center consumer)

## Latest validation capture

- Capture: `artifacts/milestones/m1/captures/text_shaping_validation_capture.json`
- Command: `cargo test -p aureline-text -p aureline-render`
