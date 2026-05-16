# Beta token / state / density / motion / theme audit (reviewer entrypoint)

This page is the reviewer entrypoint for the beta promotion of
Aureline's launch-critical visual-system and state vocabulary. It is
the canonical truth source for what the audit projects, what it
rejects, and how to reproduce a defect list locally — the audit
output, not screenshots or local notes, is what M3 review cites when
checking that focus, trust, degraded-state, and action semantics
remain legible across themes, densities, and motion postures.

## What the audit covers

The audit projects the eight launch-critical shell surfaces declared
in
[`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../crates/aureline-shell/src/token_state_audit/mod.rs):

- `shell_chrome` — title-context bar, activity rail, sidebars,
  status bar.
- `start_center` — first-touch surface for new and returning users.
- `command_palette` — keyboard-first command palette.
- `scope_truth_chip` — trust narrowing chip / scope-truth posture.
- `activity_center_row` — durable job row + badge mirror.
- `notification_envelope` — toast / banner / status notification.
- `trust_prompt_sheet` — typed permission / trust prompt sheet.
- `settings_root` — settings root pane.

Every row crosses one (theme × density × posture) cell:

- Themes: `dark_reference`, `light_parity`, `high_contrast_dark`,
  `high_contrast_light`.
- Densities: `compact`, `standard`, `comfortable`.
- Accessibility postures: `motion_standard`, `motion_reduced`,
  `motion_low_motion`, `motion_power_saver`,
  `motion_critical_hot_path`.

For each row the audit verifies, by reading the seeded projection:

1. The row declares the required color tokens for its promised
   semantics (`color.focus.*`, `status.*`, `trust.*`).
2. The row declares the required component state symbols
   (`FocusVisible`, `Selected`, `Warning`, `Restricted`,
   `Destructive`, `Locked`, `PolicyBlocked`, `Degraded`, etc.)
3. The row preserves the density-derived row, control, and
   panel-padding tokens (no `compact` row that collapses geometry to
   zero).
4. The row names a motion preset reference and a typed reduced-motion
   substitution class that preserves state conveyance and focus
   visibility for the row's promised semantics.
5. The row's `canonical_command_id` and `canonical_action_label` match
   every other row for the same surface (action labels do not drift
   across themes, densities, or motion postures).

## Defect list — read it first

The audit's only output is a typed defect list under
[`fixtures/ux/m3/theme_density_motion/defects.json`](../../../fixtures/ux/m3/theme_density_motion/defects.json).
The seeded value is `[]` — every claimed beta row passes. Reviewers
should regenerate the fixture (see below) and confirm the defect list
remains empty before signing off the lane.

The closed defect vocabulary is:

| Defect kind | When the validator emits it |
| --- | --- |
| `missing_focus_color_token` | Row promised `focus_legible` but did not declare a `color.focus.*` token. |
| `missing_focus_visible_state_symbol` | Row promised `focus_legible` but did not declare `ComponentStateClass::FocusVisible`. |
| `missing_trust_token` | Row promised `trust_legible` but did not declare a `status.*` or `trust.*` token. |
| `missing_trust_state_symbol` | Row promised `trust_legible` but trust treatment would be carried by hue alone. |
| `missing_degraded_state_symbol` | Row promised `degraded_legible` but did not declare `ComponentStateClass::Degraded`. |
| `motion_strips_state_conveyance` | Row promised state conveyance but motion would suppress the state marker. |
| `motion_strips_focus_visibility` | Row promised focus legibility but motion would hide the focus ring. |
| `density_collapses_geometry_token` | Density mode would collapse the row, control, or panel-padding token below baseline. |
| `action_label_drifts_across_rows` | Canonical command id or action label drifted across two rows for one surface. |
| `missing_density_geometry_token` | Row did not list both `size.row.*` and `size.control.*` tokens. |
| `missing_motion_preset_reference` | Row did not name a motion preset reference. |

## Reproduce locally

Validate the seeded page (exits non-zero on any defect):

```sh
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- validate
```

Replay the checked-in fixture corpus through the shared types:

```sh
cargo test -q -p aureline-shell --test token_state_audit_beta_fixtures
```

Regenerate the fixture corpus from the seeded page (the inspector is
the only mint-from-truth path):

```sh
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- page           > fixtures/ux/m3/theme_density_motion/page.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- rows           > fixtures/ux/m3/theme_density_motion/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- defects        > fixtures/ux/m3/theme_density_motion/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- support-export > fixtures/ux/m3/theme_density_motion/support_export.json
```

## Failure drill (proves the lane fails loudly)

The unit tests under
[`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../crates/aureline-shell/src/token_state_audit/mod.rs)
include one drill per defect kind. To exercise the lane locally,
patch one of the drill cases (e.g. drop a focus color token in the
seeded builder) and confirm the validator surfaces the matching
defect entry.

A drill that fails to surface its expected defect is itself a
regression — the unit test fails the build.

## Surfaces and source files

| Surface | Audit row source |
| --- | --- |
| `shell_chrome` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `shell-chrome:*`) |
| `start_center` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `start-center:*`) |
| `command_palette` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `command-palette:*`) |
| `scope_truth_chip` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `scope-truth:*`) |
| `activity_center_row` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `activity-row:*`) |
| `notification_envelope` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `notification:*`) |
| `trust_prompt_sheet` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `trust-prompt:*`) |
| `settings_root` | `crates/aureline-shell/src/token_state_audit/mod.rs` (seeded rows: `settings-root:*`) |

## Storage / index

- Module: [`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../crates/aureline-shell/src/token_state_audit/mod.rs)
- Inspector: [`crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs)
- Fixtures: [`fixtures/ux/m3/theme_density_motion/`](../../../fixtures/ux/m3/theme_density_motion/)
- Integration test: [`crates/aureline-shell/tests/token_state_audit_beta_fixtures.rs`](../../../crates/aureline-shell/tests/token_state_audit_beta_fixtures.rs)
- Companion doc: [`docs/ux/m3/design_token_beta_audit.md`](../../../docs/ux/m3/design_token_beta_audit.md)

## Relationship to adjacent lanes

This audit is **complementary** to the M1 token / motion / state
audit ([`artifacts/ux/m1_token_and_motion_audit.md`](../m1_token_and_motion_audit.md)),
not a replacement:

- The M1 audit reads the live shell source files and proves protected
  surfaces still call `require_*` and `ComponentStates::*` symbols
  in code. This beta audit promotes the same posture to the M3
  launch-critical surface set, adds the four-promise vocabulary
  (`focus_legible`, `trust_legible`, `degraded_legible`,
  `action_label_stable`), and projects an enumerable defect list a
  reviewer can quote.
- The beta audit feeds the same support-export pipeline as the
  notification-privacy and activity-center beta projections, so a
  cross-surface review packet quotes the token-state row, the
  activity row, and the notification row from one wrapper.
