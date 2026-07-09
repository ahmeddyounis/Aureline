# M5 learning-mode toggles and tip cards

The learning-mode toggle and the tip card are two of the six governed learning components
frozen by the
[M5 learning-component matrix](m5_learning_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`LearningModeToggleTipCardControlsPacket`](../../crates/aureline-learning/src/implement_learning_mode_toggles_and_tip_cards_with_user_workspace_scope_pause_snooze_reset_why_now_context_and_stable_command_file_docs_deep_link_truth_across_claimed_m5_onboarding_and_help_surfaces/mod.rs),
so a claimed M5 onboarding, tour, learning-mode, glossary, or help surface can project a
learning-mode toggle and a tip card that make learnability **explicit, reversible, and
command-backed** — never through an ephemeral coachmark or hidden routing, and never at the
cost of trust or data ownership.

## What the resolvers decide

The module has two derived resolvers so the honesty of each control is computed, never
asserted.

### `resolve_learning_activation`

Given a toggle's learning-mode state, the resolver derives an **activation class**:

- `on` → `active`
- `per_feature_family` → `scoped_active`
- `sandboxed_only` → `sandboxed_active` (must carry an explicit sandboxed note)
- `paused` → `paused` (must carry an explicit paused note), never active
- `off` / `ended` → `inactive` (must carry an explicit inactive note), never active

A user can therefore always tell **when learnability is active** and what its scope changes; a
paused, ended, or off toggle can never read as active learning.

### `resolve_tip_delivery`

Given a tip's dismissal state, the resolver derives a **delivery class**:

- `dismissible` → `delivered`
- `persistent_until_acted` → `delivered_persistent`
- `snoozed` → `snoozed` (must carry an explicit snoozed note), not on screen
- `dismissed` / `auto_expired` / `suppressed_by_preference` → `withheld` (must carry an
  explicit withheld note), never on screen

A dismissed, auto-expired, or suppressed tip can never read as delivered, and every withheld or
snoozed tip stays **reopenable from Help or the command system**.

## Scope, pause / snooze / reset, and deep links

- **Scope** — every toggle names its `global` / `workspace` / `feature_family` / `session` /
  `surface` / `unavailable` scope and an explicit scope label, so the user can tell **what
  learnability changes**.
- **Pause / snooze / reset** — every toggle offers the mandatory `reset_learning` action so a
  user can reset learning **without affecting trust or data ownership**, plus
  `enable_learning`, `pause_learning`, `snooze_learning`, and `change_scope` as appropriate.
- **Optional and dismissible** — every tip offers the mandatory `dismiss_tip` action; delivered
  tips also offer `try_next_action`, `open_deep_link`, and `snooze_tip`, and every tip can be
  reopened with `reopen_from_help` or `open_command_reference`.
- **Stable deep links** — every next step names a stable `command_reference`, `file_location`,
  `docs_anchor`, or `help_topic` deep link with a resolvable reference. A control that offers a
  deep-link action must name a resolvable kind, so a next step is never an ephemeral coachmark
  or hidden route.

## Hard invariants

Every control keeps five bools `false`, and validation flags any that is `true`:

- `masks_privacy_or_offline_state` — cached / offline / local-only state stays visible.
- `hides_activation_or_scope` — whether learning is active and what scope it changes stays
  explicit.
- `implies_hidden_apply_or_mutation` — explain and do stay separate; nothing applies without
  the ordinary preview / approval model.
- `invents_alternate_state_label` — no surface invents a second word for a governed state.
- `depends_on_ephemeral_coachmark_or_hidden_routing` — learnability never depends on ephemeral
  coachmarks or hidden routing.

Progress stays user-owned and default-local; no control widens trust or mutating authority.

## Coverage

The checked-in support export exercises every activation class, every learning-mode state, and
every scope across the six seeded toggles, and every delivery class, every tip trigger class,
and every tip dismissal state across the six seeded tip cards.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-learning-mode-toggle-tip-card-controls.schema.json`](../../schemas/ui/m5-learning-mode-toggle-tip-card-controls.schema.json)
- Support export: [`artifacts/release/m5-learning-mode-toggle-tip-card-proof/support_export.json`](../../artifacts/release/m5-learning-mode-toggle-tip-card-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-learning-mode-toggle-tip-card-proof/matrix.csv`](../../artifacts/release/m5-learning-mode-toggle-tip-card-proof/matrix.csv)
- Design report: [`artifacts/design/m5-learning-mode-toggle-tip-card.md`](../../artifacts/design/m5-learning-mode-toggle-tip-card.md)
- Scenario fixtures: [`fixtures/ui/m5-learning-mode-toggle-tip-card-controls/`](../../fixtures/ui/m5-learning-mode-toggle-tip-card-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- fixture-learning-mode-toggle-paused
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- fixture-tip-card-withheld
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_mode_toggle_tip_card_primitive -- validate
```
