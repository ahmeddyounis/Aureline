# Responsive shell rules (runtime wiring)

This document is an implementation-facing entrypoint for how the live desktop
shell exercises the responsive-fallback and split-layout contracts.

Normative sources (contract-first):

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.5 (window classes and responsive behavior)
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.1 (shell zones + minimum editor width)
- `docs/ux/shell_zone_and_density_contract.md`
- `docs/ux/shell_responsive_fallback_contract.md`
- `docs/ux/tabs_editor_groups_contract.md` (minimum useful widths + compare fallback)
- `docs/ux/splitter_contract.md` (splitters, collapse/restore, focus return)

Machine-readable companions and rehearsals:

- `artifacts/ux/shell_metrics.yaml`
- `artifacts/ux/zone_priority_rules.yaml`
- `fixtures/ux/shell_layout_classes/`
- `fixtures/ux/responsive_fallback_cases/`

## Live shell implementation touchpoints

- `crates/aureline-shell/src/layout/zone_registry.rs`
  - Owns canonical shell-zone geometry.
  - Collapses optional zones before violating the main-workspace minimum useful
    width.
  - Accepts split-heavy + minimum-width overrides so editor-group rules drive
    collapse consistently (no per-surface heuristics).

- `crates/aureline-shell/src/layout/split_tree.rs`
  - Owns a stable-id split tree used to represent editor-group splits.
  - Enforces minimum useful widths during layout; emits a typed “too narrow”
    decision instead of silently producing unusable panes.

- `crates/aureline-shell/src/app_frame/desktop_frame.rs`
  - Wires zone registry + editor-group split tree into one shared state object.
  - Tracks focus and preserves focus validity across relayouts.
  - Exposes the active `responsive_fallback_modes` for truthful placeholder
    behavior and reviewer-visible debugging.

- `crates/aureline-shell/src/bin/aureline_shell.rs`
  - Renders a placeholder desktop frame (zones + editor groups) via `winit` +
    `softbuffer`.
  - Includes keyboard-first actions for splitting editor groups, exercising
    narrow-width fallback, and opening sheet placeholders for secondary surfaces.

## How to exercise (dogfood path)

Run:

- `cargo run -p aureline-shell --bin aureline_shell`

Within the shell window:

- Resize the window across compact/standard/expanded widths and observe that the
  main workspace remains usable and focus stays visible.
- `Ctrl+\\` to split the focused editor group; `Ctrl+G` to cycle editor-group focus.
- Shrink until a new split would violate the 420 px per-group minimum; the shell
  surfaces an explicit fallback choice instead of creating unusable narrow panes.
- `Ctrl+I` opens the inspector as a sheet placeholder when the inspector is not
  docked; `Esc` closes the sheet and returns focus.

