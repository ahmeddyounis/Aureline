# Token / state / reduced-motion audit (reviewer entrypoint)

This page is the reviewer entrypoint for the token-adoption,
component-state, and reduced-motion audit on the protected M1 shell
surfaces. It is the canonical truth source for appearance semantics on
those surfaces — the audit pack, not screenshots or local notes, is what
M1 review cites when checking that Aureline ships token/state/motion
contracts and not ad hoc styling islands.

## What the audit covers

The protected walk runs over four surface families:

- **Shell chrome** — title-context bar, activity rail, sidebars, main
  workspace, status bar, transient overlays. Token-backed background,
  text, border, status, and focus-ring colors; density/space tokens.
- **Start Center** — first-touch surface for new and returning users.
  Same token pipeline plus `ComponentStates::FOCUS_VISIBLE` /
  `SELECTED` mappings on rows, framed by the shared overlay-dialog
  motion preset.
- **Search palette** — keyboard-first command/file palette. Token-backed
  query field and rows, focus ring routed through the shared component
  state registry, enter/exit framed by the shared overlay-dialog motion
  preset.
- **Trust / scope-truth surfaces** — restricted, locked, and warning
  postures. Status colors come from the shared `status.warning.*` /
  `status.danger.*` / `status.success.*` token families so trust
  narrowing is never carried by hue alone.

For each surface the audit verifies, by reading the canonical sources:

1. Every required `require_*` token id is still loaded as a literal call
   somewhere in the surface's source files. Theme, contrast, and density
   switching can only reach a surface that goes through the registry.
2. Every required `ComponentStates::*` symbol still appears in the
   surface's source files. Focus visibility, selection, hover, and
   warning treatments must keep mapping to the shared visual rules
   instead of color-only or per-surface styling.
3. The surface still pulls from `aureline_ui::motion`. The shared
   motion module is what threads the `motion_reduced`,
   `motion_low_motion`, `motion_power_saver`, and
   `motion_critical_hot_path` postures into transitions.
4. Every referenced motion-preset fixture (under
   `fixtures/design/motion_cases/`) still declares
   `reduced_motion_fallbacks` with both `preserves_state_conveyance` and
   `preserves_focus_visibility` set. A motion preset that drops the
   fallback contract gets caught here, not in late design review.
5. Surface code does not match prohibited literal patterns — raw
   `ColorRgba { r: ... }` constructions or hand-coded
   `Duration::from_millis(...)` motion durations that bypass the shared
   contracts.

## Protected walk (run unattended)

```bash
python3 tests/ux/token_state_audit/run_token_state_audit.py --repo-root .
```

The runner emits a durable JSON capture at
`artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json`
and exits non-zero on any regression. The capture records the observed
token calls, state symbols, and motion-preset summaries per surface so a
reviewer can see *what the audit actually saw* rather than just a
pass/fail line.

## Failure drill (proves the lane fails loudly)

Each fixture under `fixtures/ux/reduced_motion_cases/` declares a named
failure drill with a forced input (drop a required token, drop a
required state symbol, or drop a required motion-preset reference) and
the `check_id` the audit must report when that input is forced:

| Drill                                              | Forced input                                               | Expected check                                    |
| -------------------------------------------------- | ----------------------------------------------------------- | ------------------------------------------------- |
| `shell_chrome_drop_focus_ring_token`               | drop `require_color("al.color.focus.ring")`                 | `token_state_audit.required_token.missing`        |
| `start_center_drop_selected_state`                 | drop `ComponentStates::SELECTED` symbol                     | `token_state_audit.required_state.missing`        |
| `search_palette_drop_overlay_motion_preset`        | drop `overlay_dialog_enter.yaml` motion-preset reference    | `token_state_audit.required_motion_preset.missing` |
| `trust_surface_drop_warning_token`                 | drop `require_color("status.warning.border")`               | `token_state_audit.required_token.missing`        |

To replay one:

```bash
python3 tests/ux/token_state_audit/run_token_state_audit.py \
  --repo-root . \
  --force-drill shell_chrome_drop_focus_ring_token
```

The drill exits 0 only if the expected `check_id` was actually reported.
A drill that *fails* to surface its expected check is itself a failure
mode — the runner records it as
`token_state_audit.failure_drill.expected_finding_missing`.

## Surfaces and source files

| Surface         | Sources                                                                                                         |
| --------------- | ---------------------------------------------------------------------------------------------------------------- |
| shell_chrome    | `crates/aureline-shell/src/bootstrap/native_shell.rs`, `crates/aureline-ui/src/components/state_registry.rs`     |
| start_center    | `crates/aureline-shell/src/bootstrap/native_shell.rs`, `crates/aureline-shell/src/start_center/mod.rs`           |
| search_palette  | `crates/aureline-shell/src/bootstrap/native_shell.rs`, `crates/aureline-shell/src/search_shell/state.rs`         |
| trust_surface   | `crates/aureline-shell/src/bootstrap/native_shell.rs`, `crates/aureline-shell/src/scope_truth/card.rs`           |

## Storage / index

- Cases: `fixtures/ux/reduced_motion_cases/`
- Runner: `tests/ux/token_state_audit/run_token_state_audit.py`
- Capture: `artifacts/milestones/m1/captures/token_motion_audit_validation_capture.json`
- Proof packet: `artifacts/milestones/m1/proof_packets/token_motion_audit.md`
- Accessibility review: `artifacts/accessibility/m1/token_motion_review.md`
- Index: `artifacts/milestones/m1/artifact_index.yaml#token_motion_audit`

## Relationship to adjacent lanes

This audit is **complementary** to the existing M1 lanes, not a
replacement:

- `appearance_harness` (token-adoption baseline + screenshot goldens) —
  proves the shell-chrome renderer keeps loading the same set of
  tokens. This audit extends that posture to Start Center, search, and
  trust surfaces and adds state-symbol + motion-preset checks.
- `reduced_motion` (overlay-dialog motion preset tests) — proves the
  reduced-motion plans resolve via the registry. This audit confirms
  the protected M1 surfaces still *consume* those plans through the
  shared module.
- `theme_packs` — proves the registries themselves load. This audit
  proves consumers still call `require_*` to read them.
