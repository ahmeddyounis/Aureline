# Stabilize modal editing, leader-key discovery, register routing, macro-safe replay, and keyboard-mode downgrade truth

## Scope

This document defines the M04-191 stable contract for modal-editing safety in the Aureline editor. It covers the mode strip, leader/sequence guide, register/clipboard picker, operator-pending overlay, macro recording/replay cues, surface downgrade truth, and keymap import regression reporting.

## Goals

- Modal state is **product state**, not plugin-private state.
- Users adopting Vim-, Neovim-, Emacs-, Helix-, or product-native keyboard modes have immediate clarity on current mode, pending operator, count, register, recording state, and valid next keys.
- Destructive paste or replay differentiates local clipboard, remote clipboard bridge, named register, search register, macro register, and policy-blocked routes.
- Unsupported sequences **fail closed with explanation** rather than approximate silently.
- Unsafe or broad macro replay promotes to a **reviewable recipe** or is **rejected with a reason**.
- Modal discovery is **unified with the command graph**: leader overlays, palette search, docs, and `:command` entry all reference the same canonical command IDs and aliases.

## Surfaces

| Surface | Purpose | Minimum state |
|---|---|---|
| **Mode strip** | Declare when keys have changed meaning | Current mode, source preset, read/write posture, macro-recording cue |
| **Leader / sequence guide** | Keep partial or ambiguous key sequences inspectable | Typed prefix, timeout posture, available next keys, conflict or unsupported note |
| **Register / clipboard picker** | Show where yanks, pastes, macros, or search state will land | Register class, local vs system vs remote clipboard route, policy locks |
| **Operator-pending overlay** | Expose scope-changing commands before the target is chosen | Operator, count, object class, cross-file or replay implication |
| **Macro replay review** | Bound unsafe or broad replay before execution | Source register, target scope, write classes touched, preview or downgrade path |

## Honesty invariants

1. **Register routes fail closed.** A blocked or unsupported route must not silently fall back to a nearby route. The user sees a visible reason and a diagnostics path.
2. **Macro replays are bounded.** Replays that cross files, mutate settings, invoke run-capable commands, or depend on unstable timing must require review or be rejected.
3. **Sequence states are visible.** Partial, ambiguous, blocked, and unsupported sequence guides carry a visible note and an accessibility announcement.
4. **Recovery paths exist.** Every modal surface must expose routes to keymap diagnostics, command palette search, and safe-mode reset.
5. **Surface downgrades are labeled and reversible.** IME, accessibility, browser-companion, restricted-mode, and large-file surfaces that narrow modal fidelity must label the gap before key meaning changes and must provide a reversible restore path.
6. **Import regressions are closed vocabulary.** Imported keymap outcomes are labeled `exact`, `translated`, `partial`, `shimmed`, or `unsupported`, never simply `imported`.

## Surface downgrade matrix

| Downgrade kind | When it applies | Visible behavior | Restore path |
|---|---|---|---|
| `ime` | IME preedit is active | Modal commands deferred until composition ends | Exit IME preedit, resume modal input |
| `accessibility` | Screen-reader focus mode | Leader overlays simplified to single-stroke announcements | Toggle screen-reader focus mode in settings |
| `browser_companion` | Running in browser or companion host | Certain chords intercepted; some leader sequences unavailable | Open in desktop Aureline for full fidelity |
| `restricted_mode` | Restricted workspace policy | Run-capable commands and macro replay disabled | Trust workspace through trust gate |
| `large_file` | Large-file posture active | Multi-cursor operators and macro replay disabled for performance | Switch to normal editing after file is safe |

## Keymap import regression vocabulary

| Outcome | Meaning | Example |
|---|---|---|
| `exact` | One-to-one exact match | `gg` → `editor.go_to_first_line` |
| `translated` | Equivalent Aureline command exists | `Ctrl+W h` → `editor.navigate_to_left_group` |
| `partial` | Subset of original behavior | `gd` → `editor.go_to_definition` (no preview split) |
| `shimmed` | Approximation with known limits | `Ctrl+A` → `editor.increment_number` (decimal only) |
| `unsupported` | No faithful mapping; fails closed | `:q` quit semantics unsupported |

## Schema

The boundary schema is `schemas/editor/mode_state_record.schema.json`.

The canonical record is [`ModalEditingSafetyPacket`]:
- `record_kind`: `"modal_editing_safety_packet"`
- `schema_version`: `1`
- `schema_ref`: `"schemas/editor/mode_state_record.schema.json"`

It composes:
- `mode_state`: an [`EditorModeStateRecord`] with mode, sequence guides, register routes, pending operator, macro replay reviews, and recovery actions.
- `surface_downgrades`: active [`SurfaceDowngradeRecord`]s.
- `import_regressions`: [`KeymapImportRegressionRecord`]s for imported keymaps.
- `command_graph_unified`: `true` when modal discovery references the canonical command graph.
- `latency_budget_micros`: claimed latency budget for modal cue rendering (≤ 1,000 µs on stable rows).

## Fixtures

Deterministic claimed-stable fixtures live under:
`fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/`

The corpus is generated by:
```sh
cargo run --bin aureline_modal_editing_safety
```

Scenarios cover:
- `vim_normal_full_fidelity.json` — baseline full-fidelity surface.
- `ime_downgrade.json` — IME composition narrows modal fidelity.
- `accessibility_downgrade.json` — screen-reader mode simplifies overlays.
- `browser_companion_downgrade.json` — browser host intercepts chords.
- `restricted_mode_downgrade.json` — restricted policy disables run-capable commands.
- `large_file_downgrade.json` — large-file posture disables operators.
- `import_exact.json` — exact keymap import mapping.
- `import_translated.json` — translated keymap import mapping.
- `import_partial.json` — partial keymap import mapping.
- `import_shimmed.json` — shimmed keymap import mapping.
- `import_unsupported.json` — unsupported keymap import mapping.
- `macro_cross_file_review.json` — cross-file macro replay requires review.
- `macro_run_settings_rejected.json` — run-capable/settings-mutating macro rejected.

## Integration touchpoints

- **crates/aureline-editor** — owns the mode-state record and safety packet.
- **crates/aureline-command** — provides the canonical command graph unified with modal discovery.
- **crates/aureline-settings** — stores the transport-safe projection shape for settings and support surfaces.
- **docs/migration** — surfaces import regression outcomes during onboarding.
- **docs/help** — exposes recovery actions and keymap diagnostics.
- **fixtures/editor** — pins deterministic corpus for regression testing.
- **fixtures/accessibility** — verifies screen-reader announcements for every downgrade and sequence state.

## Performance

Modal cues and leader overlays must not add noticeable latency tax to typing or command entry on claimed stable rows. The current budget is **1,000 microseconds** per cue render.
