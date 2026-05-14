# Alpha Keybinding Resolver

This document describes the bounded alpha keybinding path now wired through
the resolver, preset profiles, settings inspection rows, help inspector, and
support-safe artifacts.

Canonical runtime sources:

- `crates/aureline-input/src/keybindings/mod.rs`
- `crates/aureline-input/src/presets/mod.rs`
- `crates/aureline-settings/src/keybindings/mod.rs`
- `crates/aureline-shell/src/keybindings/mod.rs`

Canonical artifacts:

- `schemas/settings/keybindings.schema.json`
- `fixtures/keybindings/alpha_presets/`
- `artifacts/commands/alpha_keybinding_parity_report.json`
- `artifacts/migration/keymap_translation_report_sample.json`

## Scope

The alpha scope is the command set published in
`artifacts/commands/alpha_command_registry.yaml`. The resolver path covers:

- defaults and preset profile bindings;
- user/profile, workspace, extension, admin policy, platform, security, and
  temporary mode layers through one precedence model;
- winning-binding source attribution;
- conflict review packets that remain reopenable from help, settings,
  migration, and support surfaces;
- preset profile rows for VS Code, IntelliJ, Vim, and Emacs; and
- parity rows proving palette, menu, and keybinding projections keep the same
  command id, preview class, approval posture, authority class, and result
  contract.

Out of scope: full modal-editor fidelity, arbitrary third-party keymap
emulation, macro recorder semantics, and runtime execution of incumbent
plugin state.

## Precedence

The resolver keeps the contract order frozen in
`docs/ux/keybinding_resolver_contract.md`:

1. `platform_reserved`
2. `emergency_security_hard_block`
3. `admin_policy_lock`
4. `temporary_mode_overlay`
5. `user_profile_binding`
6. `workspace_recommendation`
7. `extension_binding`
8. `core_default`

The implementation includes fixture coverage for the bounded mode-overlay path:
`fixtures/input/keybinding_cases/mode_overlay_beats_user_profile.json`.

## Preset Fidelity

Preset rows never guess parity. Each command mapping emits one of:

- `exact`
- `translated`
- `partial`
- `shimmed`
- `unsupported`

The checked-in preset fixtures list the current bounded profile rows. The
translation report includes unsupported source-tool behavior so migration and
support surfaces can distinguish "not claimed" from "silently dropped."

## Inspectable Winner Truth

The shell report exposes one winning row per claimed command:

- literal sequence;
- winning source ref;
- resolver layer;
- resolver reason code;
- inherited preview and approval posture;
- authority class; and
- policy/platform narrowing when present.

The help inspector renders a compact summary through
`crates/aureline-shell/src/help/keybinding_inspector.rs`. Settings rows use the
schema in `schemas/settings/keybindings.schema.json`, so a settings/detail
surface can render the same truth without parsing logs.

## Conflict Review

Conflict rows are projected from resolver conflict packets and include:

- the contested literal sequence;
- winning and losing command ids;
- the conflict-review packet ref;
- the retained translation report ref; and
- the product surface that can reopen the conflict.

The Vim preset intentionally keeps a `Ctrl+Shift+Y` conflict between the docs
handoff command and command-trace command so review surfaces exercise duplicate
binding inspection before imported shortcuts are applied.

## Accessibility Evidence

The reachable help surface remains
`crates/aureline-shell/src/help/keybinding_inspector.rs`. It now renders both
the existing keyboard-gap audit and the alpha keybinding truth summary, so
keyboard-only review can inspect active shortcuts, conflicts, and source refs
from the same sheet. The keyboard-gap audit continues to cover focus-return and
keyboard-route requirements for the launch-critical surfaces that consume those
shortcuts.

## Verification

Run:

```sh
cargo test -p aureline-input --test keybinding_cases
cargo test -p aureline-shell --test alpha_keybinding_truth
cargo test -p aureline-shell --test keyboard_gap_audit
```
