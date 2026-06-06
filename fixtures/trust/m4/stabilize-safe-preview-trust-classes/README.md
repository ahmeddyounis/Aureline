# Stable safe-preview trust-class fixtures

`canonical_packet.json` is emitted by
`aureline_content_safety::stable_safe_preview_trust_packet()` and validated by
`crates/aureline-content-safety/tests/stable_safe_preview_trust.rs`.

Run:

```sh
cargo test -p aureline-content-safety --test stable_safe_preview_trust
cargo run -q -p aureline-content-safety --bin stable_safe_preview_trust -- validate
```
