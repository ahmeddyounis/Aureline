# Token / state / density / motion / theme audit (beta) fixture corpus

Reviewable fixtures for the beta token-state audit projection that
lives in
[`crates/aureline-shell/src/token_state_audit/mod.rs`](../../../../crates/aureline-shell/src/token_state_audit/mod.rs).

Each JSON file is a literal projection of the seeded
`TokenStateAuditPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_token_state_audit.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:token_state_audit_beta:v1` so shell rows, badges, the headless
inspector output, and support-export rows pivot to the same `case_id`
and `row_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`rows.json`](./rows.json) | Audited rows for every claimed launch-critical surface across dark, light, and high-contrast themes; compact, standard, and comfortable densities; standard, reduced, low-motion, power-saver, and critical-hot-path postures. |
| [`defects.json`](./defects.json) | Validator defect list. Seeded value is `[]`; the validator emits typed entries when a row drops a required token, drops a required state symbol, degrades a motion substitution, collapses density geometry, or drifts an action label. |
| [`page.json`](./page.json) | Full beta audit page with aggregate summary banner (row, surface, theme, density, and posture counts; defect roll-up). |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page plus a metadata-safe defect roll-up keyed by stable defect-kind tokens. |

## Audit invariants

- Every row promises one of four typed semantics: `focus_legible`,
  `trust_legible`, `degraded_legible`, `action_label_stable`. The
  validator enforces each promise:
  - `focus_legible` requires a `color.focus.*` token, the
    `FocusVisible` component state, and a motion substitution that
    preserves focus visibility.
  - `trust_legible` requires a `status.warning.*` /
    `status.danger.*` / `status.success.*` / `trust.restricted.*` /
    `trust.locked.*` token AND a `Warning`, `Restricted`,
    `PolicyBlocked`, `Locked`, or `Destructive` component state —
    trust must never be carried by hue alone.
  - `degraded_legible` requires the `Degraded` component state AND a
    motion substitution that preserves state conveyance.
  - `action_label_stable` requires the same `canonical_command_id`
    and `canonical_action_label` across every (theme × density ×
    motion) row for one surface — a density or motion switch must
    not change the meaning of an action.
- Every row preserves the row-height, control-height, and
  panel-padding tokens for its density (compact, standard, or
  comfortable).
- Every row names a `motion_preset_ref` and resolves a typed
  substitution class — `suppress_entirely` is rejected on rows that
  promise `degraded_legible` or `trust_legible`.
- The page must cover every theme, density, and posture in the four
  closed enums (`ThemeClass`, `DensityClass`, `AccessibilityPostureClass`).

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- page           > fixtures/ux/m3/theme_density_motion/page.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- rows           > fixtures/ux/m3/theme_density_motion/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- defects        > fixtures/ux/m3/theme_density_motion/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- support-export > fixtures/ux/m3/theme_density_motion/support_export.json
```

## Verification

```sh
cargo test -q -p aureline-shell --test token_state_audit_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_token_state_audit -- validate
```
