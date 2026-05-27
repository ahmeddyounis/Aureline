# Migration-wizard import-fidelity fixtures

Scope: M04-104 — Stabilize migration-wizard import fidelity for VS Code, IntelliJ, Vim, and Emacs launch paths.

These fixtures are worked cases for the bounded beta contract that governs how imported settings, keybindings, snippets, themes, and workflow hints map to Aureline-native records.

## Fixture index

| Fixture | Editor | Scenario |
|---|---|---|
| `vs_code_settings_keybindings_exact.json` | VS Code / Code-OSS | High-fidelity exact mappings for settings and keybindings with no diagnostics. |
| `jetbrains_partial_with_diagnostics.json` | JetBrains family | Partial migration with diagnostics for code-style hints and run configs, showing translated and unsupported outcomes. |
| `vim_modal_editing_translated.json` | Vim / Neovim | Modal-editing profile translated to Aureline-native modal presets with shimmed clipboard defaults. |
| `emacs_keyboard_navigation_mixed.json` | Emacs | Mixed outcome with exact keybindings, partial command aliases, and unsupported Elisp package state. |

## Invariants checked by tests

1. Every fixture parses, validates, and projects without error.
2. Outcome labels are one of: exact, translated, partial, shimmed, unsupported.
3. Rollback checkpoint states are surfaced as separable inspectable truths.
4. Support/export records keep every `raw_*_export_allowed` flag false.
5. Consumer-surface lists include both `support_export` and `audit_lane`.
6. Diagnostics carry reason classes and suggested actions when mapping fails.
7. Launch path states are previewable or checkpointed before destructive apply.
