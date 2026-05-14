# Keyboard Mode And Orientation Alpha Contract

This document records the bounded alpha contract for preset keymaps, visible
mode state, register and clipboard routing, macro replay review, and editor
orientation aids. It complements the keybinding resolver and viewport summary
contracts; it does not claim full modal-editor ecosystem parity.

## Canonical Records

- `editor_mode_state_record` from `crates/aureline-editor/src/modes/mod.rs`
  owns current mode, source preset, sequence guides, register routes, pending
  operator state, macro review outcomes, recovery actions, and support-safe
  packet refs.
- `editor_orientation_truth_record` from
  `crates/aureline-editor/src/orientation/mod.rs` owns multi-cursor count, fold
  summary hidden-state counts, breadcrumb continuity, and minimap or overview
  degraded-state messaging.
- `mode_state_settings_inspection_record` from
  `crates/aureline-settings/src/keybindings/mode_state.rs` is the settings and
  support projection. It must not reinterpret mode behavior.
- `alpha_mode_orientation_report` from
  `crates/aureline-shell/src/help/mode_state_orientation.rs` is the shell/help
  consumer used by the keybinding inspector.

## Required Behavior

Preset keymap lanes must expose the source preset, visible mode, partial or
unsupported sequence state, and a keyboard path to keymap diagnostics, command
search, and safe mode reset.

Register and clipboard routes must distinguish local editor registers, system
clipboard, remote clipboard bridge, named registers, search register, macro
register, and policy-blocked routes before paste or replay. Unsupported or
policy-blocked routes fail closed with a visible reason.

Macro replay that crosses files, invokes run-capable commands, mutates
settings, or relies on unstable timing must require review or be rejected. It
must not widen from editor-local automation silently.

Orientation aids must keep critical state available outside optional visual
chrome. Multi-cursor count is visible, fold summaries preserve hidden
diagnostics/conflicts/trust markers, breadcrumbs preserve navigation
continuity, and reduced minimap or overview state names alternate routes.

## Evidence

- `fixtures/editor/mode_and_orientation/alpha_mode_and_orientation_cases.json`
- `artifacts/commands/alpha_mode_state_parity_report.json`

