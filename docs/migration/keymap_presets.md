# Keymap presets (built-in)

This document describes the built-in keymap presets shipped as **profile-style
bindings**. Presets exist so migration, help, and support surfaces can refer to
one stable shortcut vocabulary that binds to the same canonical command IDs used
by the command registry and palette.

## Canonical sources

- Runtime preset definitions: `crates/aureline-input/src/presets/mod.rs`
- Resolver contract: `docs/ux/keybinding_resolver_contract.md`
- Resolver schema: `schemas/commands/keybinding_resolver.schema.json`
- Seeded command registry: `artifacts/commands/command_registry_seed.yaml`

## Presets available

The current preset IDs are:

- `preset:keymap:vs_code`
- `preset:keymap:intellij`
- `preset:keymap:vim`
- `preset:keymap:emacs`

Each preset seeds the resolver’s `user_profile_binding` layer with bindings that
target stable `cmd:*` command IDs.

## Inspecting presets in the live shell

Run:

`cargo run -p aureline-shell --bin aureline_shell`

Then:

- Press `Ctrl+I` (or `Cmd+I`) to open the keybinding inspector sheet.
- Use `Left` / `Right` arrow keys to switch presets.
- The inspector lists bindings as `title — command_id => shortcut(s)` and lists
  any detected same-layer collisions as conflicts requiring review.

## Notes on current limitations

- The desktop shell input path currently recognizes letter keys (A–Z) and
  `Space` for resolver-driven shortcuts; additional keys will be added as the
  input router matures.
- A conflict review is emitted when multiple equally-specific candidates contend
  for the same sequence in the same resolver layer; help surfaces should prefer
  quoting the conflict-review packet fields over recreating local summaries.

