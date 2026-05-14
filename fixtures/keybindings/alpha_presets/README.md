# Alpha Keymap Preset Fixtures

These fixtures protect the bounded alpha preset rows consumed by the
keybinding truth report and help inspector. Each preset lists the claimed
command set, the literal shortcut supplied by that preset, and the controlled
translation outcome.

Runtime source:

- `crates/aureline-input/src/presets/mod.rs`
- `crates/aureline-shell/src/keybindings/mod.rs`

Verification:

```sh
cargo test -p aureline-shell --test alpha_keybinding_truth
```
