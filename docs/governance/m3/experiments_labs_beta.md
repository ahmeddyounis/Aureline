# M3 experiments / flags / Labs governance beta projection

This page is the reviewer-facing landing for the beta experiments, flags,
and Labs governance UI projection. It does **not** mint a parallel
inventory or kill-switch precedence — those still live in the settings
crate
([`crates/aureline-settings/src/experiments/mod.rs`](../../../crates/aureline-settings/src/experiments/mod.rs))
reading the canonical artifact
[`artifacts/governance/experiments_inventory_alpha.yaml`](../../../artifacts/governance/experiments_inventory_alpha.yaml).
The beta projection is the page-level surface that pins three promises
on every claimed row so the settings root pane, Help / About panel,
diagnostics panel, support export packet, release center, and command
palette all read the same truth.

## Companion control artifacts

- Source inventory: `artifacts/governance/experiments_inventory_alpha.yaml`
- Beta projection module:
  [`crates/aureline-settings/src/experiments/labs_governance_beta.rs`](../../../crates/aureline-settings/src/experiments/labs_governance_beta.rs)
- Shell consumer:
  [`crates/aureline-shell/src/experiments_governance/mod.rs`](../../../crates/aureline-shell/src/experiments_governance/mod.rs)
- Headless inspector: `aureline_shell_experiments_governance`
- JSON Schema:
  [`schemas/governance/experiments_labs_governance_beta.schema.json`](../../../schemas/governance/experiments_labs_governance_beta.schema.json)
- Reviewer fixtures:
  [`fixtures/settings/experiments_labs_governance_beta/`](../../../fixtures/settings/experiments_labs_governance_beta/)
- Integration test:
  [`crates/aureline-settings/tests/experiments_labs_governance_beta.rs`](../../../crates/aureline-settings/tests/experiments_labs_governance_beta.rs)

## What the projection pins

1. **Per-row alignment.** Every projection row carries an `owner`, a
   `cohort_or_ring`, a `review_or_expiry_date`, and a
   `kill_switch_path.summary` that quotes the highest-precedence
   inventory source. The validator rejects a row that loses any of
   these. A row whose effective lifecycle state is `DisabledByPolicy`
   MUST also carry an active source class.
2. **Visible markers on stable surfaces.** Every host surface enumerates
   whether it claims `claims_stable_posture` truth. A stable-claiming
   host that renders a non-stable row MUST carry a `visible_marker_token`
   (`labs_chip` / `preview_chip` / `beta_chip` / `deprecated_chip` /
   `policy_disabled_chip` / `retired_tombstone`) AND a non-empty
   `visible_marker_disclosure`. This is how the projection enforces:
   *Stable-looking surfaces do not silently depend on hidden experiment
   state without a visible marker.*
3. **Shared vocabulary.** UI badges, CLI rows, support-export rows, and
   this doc all read the same closed lifecycle vocabulary
   (`Labs`, `Preview`, `Beta`, `Stable`, `Deprecated`, `DisabledByPolicy`,
   `Retired`). The validator rejects a row whose UI badge token disagrees
   with the CLI or support-export token or the inventory's effective
   lifecycle state.

## Host surface vocabulary

| Surface token             | Claims stable posture |
|---------------------------|-----------------------|
| `settings_root`           | yes                   |
| `settings_labs_tab`       | no                    |
| `help_about_panel`        | yes                   |
| `diagnostics_panel`       | yes                   |
| `support_export_packet`   | yes                   |
| `release_center`          | yes                   |
| `command_palette`         | yes                   |

A `settings_labs_tab` row does not need a visible marker because the
tab itself is the marker. Every other host claims stable posture; the
projection requires a visible marker for every non-stable row those
hosts render.

## Lifecycle marker mapping

| Effective lifecycle state | Visible marker token  | Counts toward attention chip |
|---------------------------|-----------------------|------------------------------|
| `Labs`                    | `labs_chip`           | yes                          |
| `Preview`                 | `preview_chip`        | yes                          |
| `Beta`                    | `beta_chip`           | yes                          |
| `Stable`                  | none (no marker)      | no                           |
| `Deprecated`              | `deprecated_chip`     | yes                          |
| `DisabledByPolicy`        | `policy_disabled_chip`| yes                          |
| `Retired`                 | `retired_tombstone`   | no                           |

## Verifying the lane

Run the headless inspector to mint or validate the page:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- page
cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- cli
cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- validate
```

Replay the failure drills (each fixture is a known regression that the
validator MUST flag):

- `fixtures/settings/experiments_labs_governance_beta/drill_hidden_experiment_on_stable_surface.json`
  — a Labs row stripped of `visible_marker_token`. The validator MUST
  emit `visible_marker_missing` AND
  `stable_host_renders_hidden_experiment`.
- `fixtures/settings/experiments_labs_governance_beta/drill_missing_alignment_field.json`
  — a Beta row stripped of its `owner` ref. The validator MUST emit
  `alignment_field_missing { field: "owner" }`.

The integration test under
`crates/aureline-settings/tests/experiments_labs_governance_beta.rs`
replays both drills and asserts the validator surfaces the expected
errors.

## How the projection links to source policy

- Owner / cohort / expiry / kill-switch alignment is enforced by the
  validator on every row, mirroring
  `docs/governance/feature_flag_policy.md` and
  `docs/governance/experiment_expiry_and_schema_review_contract.md`.
- The visible-marker rule mirrors the alpha capability-lifecycle badge
  contract under `docs/ux/capability_lifecycle_badge_contract.md`.
- Kill-switch precedence (`emergency_security_response` >
  `admin_policy_ceiling` > `release_channel_or_rollout_override` >
  `cohort_or_ring_assignment` > `user_opt_in_or_local_preview_toggle`)
  is consumed verbatim from the canonical inventory; the projection
  never re-orders or shortens the ladder.
