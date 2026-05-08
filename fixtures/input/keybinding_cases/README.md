# Keybinding resolver input cases

These fixtures are **input-driven** resolver cases used by the Rust implementation
in `crates/aureline-input/`.

They complement (but do not replace) the contract-first resolver packets in:

- `docs/ux/keybinding_resolver_contract.md`
- `fixtures/commands/keybinding_conflict_examples/`

## Format

Each `*.json` file describes:

- an inspected key sequence (string form like `Ctrl+Shift+P` or `Ctrl+K S`);
- a minimal inspection scope (platform + surface + focus context);
- a set of candidate bindings grouped by resolver layer; and
- the expected winning outcome (winner kind/layer/state + command id when resolved).

The fixtures intentionally keep values opaque and stable (no raw paths, no raw
workspace content, no secrets).

