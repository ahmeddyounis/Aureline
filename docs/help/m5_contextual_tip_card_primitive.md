# M5 contextual-tip-card primitive

The contextual tip card is one of the five governed contextual-teaching / migration-bridge
component families frozen by the
[M5 contextual-teaching / migration-bridge component matrix](m5_contextual_teaching_migration_bridge_component_matrix.md).
This primitive narrows that family into a single reusable resolver,
[`resolve_contextual_tip_card`](../../crates/aureline-learning/src/implement_contextual_tip_cards_with_why_now_relevance_concrete_next_action_stable_command_reference_and_try_open_docs_snooze_dismiss_actions_that_respect_quiet_hours_presentation_mode_and_recent_dismissals_across_claimed_m5_learnability_surfaces/mod.rs),
so a user can learn a nearby action **in place** — from the card alone — without ever leaving
the task or reopening a detached tutorial.

## What the resolver decides

Given one tip's trigger class, command-backing state, dismissal state, live delivery context
(quiet hours, presentation mode, whether a like tip was recently dismissed, whether the
underlying action requires approval), and its opaque why-now relevance, stable next-action
command reference, and stable tip identity, the resolver derives:

- **Delivery posture** — a suppression-first ladder so a tip always respects its delivery
  limits *before* it is ever shown:
  1. `withheld_for_quiet_hours` — quiet hours are active.
  2. `withheld_for_presentation_mode` — presentation mode is active.
  3. `withheld_already_resolved` — the tip was already dismissed / auto-expired /
     suppressed-by-preference, or a like tip was recently dismissed (non-spammy guard).
  4. `snoozed_for_later` — the user snoozed it.
  5. `delivered_actionable` — clear to show and backed by a command.
  6. `delivered_informational` — clear to show but with no command backing (a pure hint).
- **Bounded actions** — a delivered tip always offers `open_docs`, `snooze_tip`, and
  `dismiss_tip` so it stays reversible in place. An actionable tip also offers `try_next_action`
  — or `request_approval` instead whenever the underlying action requires approval, so the tip
  never bypasses the trust boundary of the action it teaches. A snoozed tip offers only
  `dismiss_tip`; a withheld tip is off screen and offers nothing.

Every resolved card also asserts the acceptance-criterion invariants: it `teaches_in_place`,
never `hijacks_workflow`, always `respects_quiet_hours` / `respects_presentation_mode` /
`respects_recent_dismissals`, stays `is_reversible`, and `honors_underlying_trust_limits`.

## Reused vs minted vocabulary

The tip trigger class, tip dismissal state, command-backing state, surface family, deployment
line, teaching consumer surface, accessibility route, qualification class, and downgrade
triggers are reused verbatim from the frozen component matrix. This primitive mints new
vocabulary only for what that matrix left implicit about the tip card itself: its learnability
consumers, its anatomy parts, its derived delivery posture, its bounded actions, and its export
fields. No M5 learnability surface invents a second tip-card grammar.

## Learnability consumers

One parity row is bound per claimed M5 learnability consumer so the why-now / next-action /
command-reference / delivery vocabulary stays identical across desktop, headless/export, and
support consumers:

- First-Run Onboarding Panel
- Guided-Tour Overlay
- Command-Palette Hint
- Inline Editor Tip
- Support Tip Export

## Source contracts

- `schemas/ui/m5-contextual-tip-card.schema.json` — this primitive's boundary schema.
- `docs/help/m5_contextual_tip_card_primitive.md` — this contract doc.
- `schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json` — the frozen
  component matrix this primitive narrows from.
- `schemas/commands/command_descriptor.schema.json` — the stable command reference behind the
  tip's next action.
- `schemas/ux/presentation_mode_state.schema.json` — the presentation-mode signal the tip's
  delivery respects.

## Checked-in evidence

- Support export: `artifacts/release/m5-contextual-tip-card-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-contextual-tip-card-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-contextual-tip-card-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-contextual-tip-card-primitive/`

All evidence is minted from one source of truth by the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_contextual_tip_card_primitive -- validate
```
