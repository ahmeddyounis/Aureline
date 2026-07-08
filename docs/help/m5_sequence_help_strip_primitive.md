# M5 sequence-help-strip primitive

The sequence-help strip is one of the five governed contextual-teaching / migration-bridge
component families frozen by the
[M5 contextual-teaching / migration-bridge component matrix](m5_contextual_teaching_migration_bridge_component_matrix.md).
This primitive narrows that family into a single reusable resolver,
[`resolve_sequence_help_strip`](../../crates/aureline-learning/src/implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces/mod.rs),
so a keyboard-first user can understand — from the strip alone — exactly where a partial or
ambiguous key sequence stands **before** it fails or surprises them, **without** detached docs or
tribal knowledge, and can learn command-language pathways entirely in-product.

## What the resolver decides

Given one sequence's help state, current step kind, command-backing state, opaque
current-mode-or-leader reference, valid next keys, cancel key, optional opaque example-command
reference, screen-reader announcement text, opaque full-cheat-sheet reference, and opaque stable
strip identity, the resolver derives:

- **Help posture** — derived one-to-one from the frozen sequence-help state so a partial,
  dead-end, ambiguous, or disabled sequence is always named for exactly what it is:
  1. `ready_for_input` — ready to accept the first key (`ready`).
  2. `awaiting_next_key` — awaiting the next key of a multi-key sequence (`awaiting_next_key`).
  3. `partial_sequence` — a partial match that can still continue (`partial_match`).
  4. `unbound_dead_end` — no binding for the entered keys (`no_binding`).
  5. `conflicting_binding` — an ambiguous binding needs resolution (`conflicting_binding`).
  6. `disabled_in_context` — disabled in the current context (`disabled_in_context`).
- **Bounded actions** — every strip offers `cancel_sequence` and `open_full_cheat_sheet` so a
  keyboard-first user can always back out or reach the full cheat sheet. A strip with valid next
  keys also offers `show_valid_next_keys`; a command-backed sequence with an example offers
  `run_example_command`; and an ambiguous sequence offers `resolve_conflicting_binding`.

Every resolved strip also asserts the acceptance-criterion invariants: it
`shows_current_mode_or_leader`, `explains_next_keys_or_dead_end`, `shows_cancel_key`,
`never_requires_pointer_hover`, `provides_screen_reader_announcement`,
`keeps_full_cheat_sheet_reachable`, and `preserves_command_backing_honestly`.

An open sequence (ready, awaiting a next key, or a partial match) with **no** valid next keys is
rejected outright (`missing_next_keys_for_open_sequence`), so an ambiguous or partial sequence
never leaves the user with nothing to press and no way to interpret it. A command-backed sequence
must name an example command (`missing_example_for_backed_state`), and a sequence with no command
backing may not declare one (`example_command_on_unbacked_state`).

## Reused vs minted vocabulary

The sequence-help state, sequence step kind, command-backing state, surface family, deployment
line, teaching consumer surface, accessibility route, qualification class, and downgrade triggers
are reused verbatim from the frozen component matrix. This primitive mints new vocabulary only
for what that matrix left implicit about the strip itself: its modal / command-language
consumers, its anatomy parts, its derived help posture, its bounded actions, and its export
fields. No M5 command-language surface invents a second sequence-help grammar.

## Modal / command-language consumers

One parity row is bound per claimed M5 modal / command-language consumer so the current-mode /
next-keys / cancel-key / example-command / cheat-sheet vocabulary stays identical across desktop,
headless/export, and support consumers, and so the same strip works for leader sequences, modal
operators, partial keyboard commands, and every related command-language teaching moment:

- Leader-Sequence Overlay
- Modal-Operator Strip
- Partial-Command Hint
- Command-Palette Sequence Hint
- Support Sequence Export

## Source contracts

- `schemas/ui/m5-sequence-help-strip.schema.json` — this primitive's boundary schema.
- `docs/help/m5_sequence_help_strip_primitive.md` — this contract doc.
- `schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json` — the frozen
  component matrix this primitive narrows from.
- `schemas/commands/keybinding_resolver.schema.json` — the keybinding-resolver contract the
  strip's next-key guidance binds against.
- `schemas/commands/command_descriptor.schema.json` — the command-descriptor contract the strip's
  example-command backing binds against.

## Checked-in evidence

- Support export: `artifacts/release/m5-sequence-help-strip-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-sequence-help-strip-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-sequence-help-strip-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-sequence-help-strip-primitive/`

All evidence is minted from one source of truth by the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- validate
```
