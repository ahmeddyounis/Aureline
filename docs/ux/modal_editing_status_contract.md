# Modal editing status and recovery contract

This document freezes the UX contract for modal-editing status strips,
sequence-help access, intentional modeless fallbacks, and mode-loss
recovery. It exists so source editors, rename fields, search fields,
dialogs, terminals, webviews, browser companions, docs/help, settings,
and support exports all expose the same truth when modal editing changes
key meaning or becomes unavailable.

Machine-readable companions:

- [`/schemas/ux/modal_state.schema.json`](../../schemas/ux/modal_state.schema.json)
  defines one `modal_state_record` snapshot for status-strip rendering,
  availability disclosure, macro-capture posture, mode-loss recovery,
  and help/accessibility hooks.
- [`/fixtures/ux/modal_editing_cases/`](../../fixtures/ux/modal_editing_cases/)
  contains worked examples for supported source-editor modal state,
  rename-field fallback, dialog escape recovery, search-field text input,
  and terminal passthrough.

This contract composes with, and does not replace:

- [`/docs/commands/sequence_and_modal_discoverability_contract.md`](../commands/sequence_and_modal_discoverability_contract.md)
  for canonical command graph parity, leader overlays, sequence-help
  rows, pending operators, register cues, and shortcut teaching.
- [`/docs/ux/keybinding_resolver_contract.md`](./keybinding_resolver_contract.md)
  for precedence, conflict review, blocked host shortcuts, imported
  keymap fidelity, and typed disabled reasons.
- [`/docs/ux/status_bar_contract.md`](./status_bar_contract.md) and
  [`/docs/ux/status_strip_family_contract.md`](./status_strip_family_contract.md)
  for stable status placement, compact overflow, screenshot-safe state
  labels, and keyboard-reachable detail routing.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md),
  [`/docs/ux/forms_validation_contract.md`](./forms_validation_contract.md),
  and [`/docs/ux/field_rules_contract.md`](./field_rules_contract.md)
  for focus takeover, form-field safety, validation, and high-risk field
  review.
- [`/docs/accessibility/a11y_ime_packet_template.md`](../accessibility/a11y_ime_packet_template.md)
  and [`/docs/i18n/locale_input_readiness.md`](../i18n/locale_input_readiness.md)
  for screen-reader, IME, dead-key, AltGr, sticky-key, and locale input
  readiness.

If this document conflicts with the command descriptor, keybinding
resolver, sequence-help, status bar, dialog, form, accessibility, or
source UI/UX specification, the upstream owner wins and this contract,
schema, and fixtures must change in the same patch.

## Scope

Frozen here:

- status-strip fields for current mode, pending operator, leader path,
  macro recording or replay state, temporary fallback notes, and visible
  unavailable or narrowed contexts;
- current-surface modal availability classes for full modal editing,
  limited modal editing, modeless fallback, text-input exemption,
  terminal passthrough, host capture, and policy block;
- mode-loss and recovery rules for dialogs, search, rename fields,
  forms, terminal focus, webviews, browser companions, host capture,
  policy narrowing, and IME composition;
- macro-capture rules that block silent capture when a sequence is
  destructive, unresolved, or running in a surface that is not modal
  safe; and
- accessibility and help hooks for keyboard inspection, screen-reader
  announcements, sequence help, leader overlays, fallback explanations,
  docs/help pivots, and focus return.

Out of scope:

- shipping a full modal-editing implementation;
- choosing a final visual design for the strip, overlays, or picker;
- defining every Vim, Neovim, Emacs, Helix, or product-native command;
- changing command ids, preview posture, or approval posture owned by
  command descriptors.

## Canonical State Chain

Modal state is product state. It is not private keymap plugin state and
must be serializable into a `modal_state_record`.

| Source | Owns | Modal status may project |
|---|---|---|
| Command descriptor | command id, canonical verb, preview, approval, disabled reason, docs/help anchor | command identity and safety posture for sequence help |
| Keybinding resolver | sequence resolution, active layer, host/policy block, conflict review | whether a key is waiting, blocked, unsupported, or resolved |
| Sequence and leader overlay records | available next keys, operator-pending help, leader group rows, register cues | inline help and overlay pivots |
| Modal state record | current surface, visible strip fields, modal availability, fallback note, recovery state, capture posture, accessibility hooks | status strip, support export, help bridge, and mode-loss recovery |

Rules:

1. A modal strip, fallback note, or recovery banner must not mint local
   command ids or local enablement rules.
2. If a strip field refers to a command-bearing sequence, it quotes the
   stable command id and command revision from the command graph through
   sequence-help or resolver refs.
3. If a surface cannot honor modal editing faithfully, the state record
   says so with a typed availability class before key meaning changes.
4. Hidden mode, hidden macro recording, and silent destructive capture
   are non-conforming.

## Status-Strip Anatomy

Whenever the current surface changes key meaning, a keyboard-reachable
status strip, status-bar segment, or equivalent persistent cue is
required. Compact layouts may condense the strip, but the backing record
must retain every field below.

| Field | Required content | Non-conforming collapse |
|---|---|---|
| `current_mode` | Canonical mode label such as `normal`, `insert`, `visual`, `replace`, `select`, `command`, `terminal_passthrough`, `modeless`, or `unsupported`; source profile when it changes meaning; non-color visibility cue. | Color-only mode state or private plugin state with no keyboard route. |
| `pending_operator` | Operator key, count if present, expected motion/text object, destructive flag, scope preview ref, cancel hint, and resulting command id when known. | Letting `d`, `3d`, `ci`, or similar destructive prefixes wait invisibly. |
| `leader_path` | Literal prefix, typed keys, waiting/ambiguous/resolved state, sequence-help ref, timeout, and cancel gesture. | A which-key overlay generated from hand-maintained labels or no waiting state. |
| `macro_recording` | Recording/replay state, register ref, visible label, replay scope, and review requirement. | Macro state that disappears while still capturing or replaying. |
| `temporary_fallback_note` | Short note when the active focus is modeless, narrowed, host-blocked, or intentionally exempt from modal semantics. | Failing with no indication in rename, search, form, terminal, or webview focus. |
| `unavailable_contexts` | Surface class, support class, typed reason, visible label, and detail route for each known context where full modal editing is unavailable or narrowed. | Generic "not available" text without scope, owner, or retry path. |

Mode changes must never rely on color alone. The strip must include at
least one non-color cue such as a text label, icon shape, screen-reader
announcement, status-strip segment, or focus-region label.

## Availability Classes

Every focus change resolves modal support for the target surface before
dispatching modal sequences.

| Class | Meaning | Required behavior |
|---|---|---|
| `full_modal` | The surface can honor the active modal profile for navigation, operators, counts, registers, leader sequences, and safe macro capture. | Show mode, pending sequence, macro state, and sequence-help hooks. |
| `limited_modal` | The surface supports some modal semantics but narrows operators, leader groups, registers, or replay. | Show the narrowed scope and supported-surface retry path. |
| `modeless_fallback` | The surface accepts text or UI input and does not reinterpret ordinary keys as modal commands. | Show a temporary fallback note and preserve literal input. |
| `text_input_exempt` | A rename/search/form field intentionally captures text input instead of modal operators. | Block destructive macro capture, keep Escape semantics explicit, and restore prior mode on return when valid. |
| `terminal_passthrough` | A terminal-like surface owns most key input. | Show terminal passthrough, preserve host or shell input, and expose a command-palette/help route. |
| `modal_unavailable` | The surface cannot provide truthful modal semantics. | Explain why, expose a retry path, and do not approximate destructive operations. |
| `policy_blocked` | Policy, trust, managed profile, or emergency state denies modal dispatch or macro capture. | Quote the policy/support owner and deny closed. |
| `host_blocked` | OS, browser, embedded host, or remote latency protection captures or narrows key input. | Name the host boundary and available fallback route. |
| `temporarily_suspended` | Modal state is suspended by dialog focus, IME composition, or another temporary owner. | Preserve a focus-return target and visible recovery note. |

Unsupported surfaces must be visible, not surprising. A terminal,
rendered docs pane, webview, browser companion, property grid, or
restricted input may opt out of full modal editing only if the strip and
help paths say so directly.

## Mode Loss And Recovery

Mode loss is any transition where the user's previous modal mode would
otherwise become ambiguous after focus moves, Escape is pressed, a
dialog opens, a text field takes input, a surface is destroyed, host
capture starts, policy narrows authority, or IME composition owns input.

Rules:

1. Before opening a dialog, sheet, palette, rename field, search field,
   form field, terminal, webview, or browser-companion surface that
   changes modal dispatch, record the previous mode and focus-return
   target.
2. If a pending operator or leader prefix is active, the transition
   either preserves it with visible waiting state or cancels it with a
   visible recovery note. Destructive pending sequences must be cleared
   before a modeless fallback can capture text.
3. `Escape` must have one declared behavior in the active record:
   cancel pending sequence, close current surface, return to previous
   mode, exit to modeless input, pass through as literal host input, or
   do nothing while IME composition owns it.
4. Rename fields, search fields, and ordinary form fields are
   text-input exemptions by default. They must preserve literal input
   and may not treat `d`, `c`, `q`, leader prefixes, or counts as modal
   operations unless the field explicitly declares modal-safe editing.
5. When focus returns to a modal-safe editor, restore the previous mode
   only when the original surface and buffer identity still match. If
   identity changed, reset to the profile's safe entry mode and explain
   the reset.
6. If policy or host capture caused mode loss, recovery must name the
   owner and cannot silently replay the user's previous sequence.

## Macro Capture Safety

Macros are local editor automation unless a review record says
otherwise. Capture must fail closed when key meaning is unavailable,
ambiguous, or no longer editor-local.

Rules:

1. Recording or replay state remains visible until it ends, blocks, or
   enters review.
2. Text entered into rename, search, password/secret, dialog, terminal,
   browser-hosted, or unsupported surfaces is not captured as modal
   command input by default.
3. A destructive sequence may be captured only when the current surface
   is modal safe, the pending operator is visible, the command identity
   is known, and the command descriptor's preview/approval posture is
   preserved.
4. If a recording crosses files, surfaces, project commands, external
   tools, networked operations, or managed/policy boundaries, capture
   switches to review or to a recipe/automation path instead of silently
   extending macro power.
5. Macro replay into a narrowed or unavailable surface is blocked until
   the user reviews scope, fallback, and unavailable-context notes.

## Accessibility And Help Hooks

The modal state record is the accessibility and help bridge. It must
make the same state reachable through keyboard, screen-reader, docs/help,
support export, and command palette routes.

Required hooks:

- keyboard focus target for the status strip or equivalent mode cue;
- screen-reader announcements for mode change, pending operator,
  leader waiting state, macro recording/replay, fallback entry/exit,
  unsupported surface, blocked sequence, and recovered mode where
  applicable;
- sequence-help and leader-overlay refs when a prefix or pending
  operator exists;
- fallback explanation ref when full modal editing is unavailable,
  narrowed, or intentionally disabled for text entry;
- docs/help anchor for the active modal profile or current fallback;
- focus-order refs and focus-return target for recovery; and
- pivots to palette, keybinding settings, command docs, migration
  guidance, conflict review, supported-surface retry, or mode reset.

Screen-reader copy should be short and stable. State labels must not
depend on punctuation timing or US-keyboard assumptions; IME
composition, dead keys, AltGr, sticky keys, and non-US layouts keep the
right to own input while composition is active.

## Fixture Expectations

Fixtures under
[`/fixtures/ux/modal_editing_cases/`](../../fixtures/ux/modal_editing_cases/)
exercise:

- full modal support in a source editor with visible mode, pending
  operator, leader path, macro recording, and command-graph help refs;
- rename-field fallback where destructive modal input is not silently
  captured;
- dialog focus and Escape recovery where prior mode and focus target
  remain explicit;
- search-field text input exemption during macro recording;
- form-field modeless fallback where review posture and macro capture
  stay explicit; and
- terminal passthrough where full modal editing is visibly unavailable
  and a supported-surface retry path is present.

Each fixture validates against
[`/schemas/ux/modal_state.schema.json`](../../schemas/ux/modal_state.schema.json).
