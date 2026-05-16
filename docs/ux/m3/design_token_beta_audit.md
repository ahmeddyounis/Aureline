# Design token / state / density / motion / theme beta audit (companion)

This companion document is the reviewer-facing contract for the beta
audit that promotes Aureline's launch-critical visual-system and
state vocabulary. It does not mint a parallel design system; it pins
what the audit projects, what it rejects, and how a reviewer reads
the checked-in defect list.

## What the audit promises

Launch-critical shell surfaces ship the same focus, trust,
degraded-state, and action semantics across dark, light, and
high-contrast themes; compact, standard, and comfortable densities;
and standard, reduced, low-motion, power-saver, and critical-hot-path
motion postures.

The four typed semantics are:

- **`focus_legible`** — the focus ring resolves through a
  `color.focus.*` token AND the `FocusVisible` component state. A
  motion substitution may collapse the transition but must not hide
  the ring.
- **`trust_legible`** — trust narrowing (warning / restricted /
  locked / policy-blocked / destructive) resolves through a
  `status.*` or `trust.*` token AND a matching component state
  symbol. Trust is never carried by hue alone, even on
  high-contrast themes.
- **`degraded_legible`** — degraded posture resolves through the
  `Degraded` component state AND a non-motion state marker. Motion
  may collapse but must not be the only carrier of the state.
- **`action_label_stable`** — the same `canonical_command_id` and
  `canonical_action_label` reappear across every (theme × density ×
  motion) row for one surface. A density or motion switch must not
  change the meaning of an action.

## Surfaces under audit

The audit covers eight launch-critical shell surfaces:

| Surface | Why it's launch-critical |
| --- | --- |
| `shell_chrome` | Title-context bar, activity rail, sidebars, and status bar — focus, trust, and degraded posture must remain legible across themes and densities. |
| `start_center` | First-touch surface for new and returning users; row focus and selected state must remain legible. |
| `command_palette` | Keyboard-first command surface; the focus ring, selected row, and enter/exit motion must keep state visible under reduced motion. |
| `scope_truth_chip` | Trust narrowing chip / scope-truth posture; warning and restricted treatments must come from token families, not hue alone. |
| `activity_center_row` | Durable job row + badge; lifecycle and resolution state must stay legible across density and motion. |
| `notification_envelope` | Toast / banner / status notification; severity and privacy treatments must stay legible across theme switches. |
| `trust_prompt_sheet` | Typed permission / trust prompt sheet; warning and destructive treatments must keep their state symbols across themes. |
| `settings_root` | Settings root pane; navigation focus, current-pane indicator, and policy-locked rows must keep their state legible. |

## Defect vocabulary

The audit's only output is a typed defect list. The seeded page seeds
zero defects; the validator emits the following kinds when a row
regresses:

- `missing_focus_color_token`
- `missing_focus_visible_state_symbol`
- `missing_trust_token`
- `missing_trust_state_symbol`
- `missing_degraded_state_symbol`
- `motion_strips_state_conveyance`
- `motion_strips_focus_visibility`
- `density_collapses_geometry_token`
- `action_label_drifts_across_rows`
- `missing_density_geometry_token`
- `missing_motion_preset_reference`

Each defect carries the surface, row id, drifted field, and a
reviewer-facing note. The list is enumerable (no free-text bug
reports) so cross-row drift is comparable in support exports.

## Inputs / outputs

- **Module**:
  [`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../crates/aureline-shell/src/token_state_audit/mod.rs)
- **Headless inspector**:
  [`crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs)
- **Fixture corpus**:
  [`fixtures/ux/m3/theme_density_motion/`](../../../fixtures/ux/m3/theme_density_motion/)
- **Integration test**:
  [`crates/aureline-shell/tests/token_state_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/token_state_audit_beta_fixtures.rs)
- **Reviewer entrypoint**:
  [`artifacts/ux/m3/token_state_audit.md`](../../../artifacts/ux/m3/token_state_audit.md)

## Run the audit

```sh
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- validate
cargo test -q -p aureline-shell --test token_state_audit_beta_fixtures
```

The `validate` subcommand exits with code `0` when the seeded page
seeds zero defects and exits with code `3` after writing each typed
defect to stderr when it doesn't. That non-zero exit is what the
review gate watches.

## Failure-drill posture

The unit tests under
[`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../crates/aureline-shell/src/token_state_audit/mod.rs)
include one drill per defect kind (drop the focus token, drop the
focus-visible state symbol, drop a trust token, strip motion
substitution, collapse density geometry, drift an action label, etc.).
A drill that fails to surface the expected defect is itself a
regression — the unit test fails the build.

## Relationship to adjacent lanes

This audit is **complementary** to the M1 token / motion / state
audit ([`artifacts/ux/m1_token_and_motion_audit.md`](../../../artifacts/ux/m1_token_and_motion_audit.md)),
not a replacement:

- The M1 audit walks the live source files and proves protected M1
  surfaces still call `require_*` and `ComponentStates::*` symbols.
  This beta audit promotes the same posture to the M3 launch-critical
  surface set and adds the `focus_legible` / `trust_legible` /
  `degraded_legible` / `action_label_stable` promise vocabulary so a
  reviewer can quote the contract a row breaks instead of a free-text
  observation.
- The beta audit feeds the same support-export pipeline as the
  notification-privacy and activity-center beta projections, so a
  cross-surface review packet can quote the token-state audit row,
  the activity row, and the notification row from one wrapper.
