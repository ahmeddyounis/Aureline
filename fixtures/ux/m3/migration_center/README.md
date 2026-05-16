# Migration center / learnability surface (beta) fixture corpus

Reviewable fixtures for the beta migration-center and learnability
projection that lives in
[`crates/aureline-shell/src/migration_center/mod.rs`](../../../../crates/aureline-shell/src/migration_center/mod.rs).

Each JSON file is a literal projection of the seeded
`MigrationCenterPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_migration_center.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_migration_center.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:migration_center_beta:v1` so the shell UI rows, the headless
CLI rows, and the support-export rows pivot to the same `entry_id`
and `support_row_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`page.json`](./page.json) | Full beta page with the learnability claim, upstream wizard/corpus refs, sections, entries, support rows, defect list, and summary banner. |
| [`sections.json`](./sections.json) | Section rows for each required section kind. |
| [`entries.json`](./entries.json) | Entry-point rows for docs/help anchors, glossary packs, migration reports, known limits, recovery routes, and first-run exits. |
| [`support_rows.json`](./support_rows.json) | Support-export rows aligned 1:1 with the live entries by `entry_id`. |
| [`defects.json`](./defects.json) | Typed defect list emitted by the validator. Seeded value is `[]`. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page plus a metadata-safe defect roll-up. Raw private material is excluded by construction. |
| [`summary.json`](./summary.json) | Reviewer-facing summary banner (section count, entry count, recovery-route count, keyboard-first count, defect count). |

## Fixture rules

- Every record carries a stable `entry_id`, `section_id`, and the
  shared contract ref `shell:migration_center_beta:v1`; record kinds
  are stable Rust constants.
- Every row quotes a learnability claim with closed-vocabulary tokens
  for `claim_class`, `lifecycle_class`, and `freshness_class`.
- Claimed beta rows MUST NOT carry the `review_overdue` freshness
  state without an explicit downgrade; the validator rejects it.
- Every row carries a `keyboard_reach` token from the closed set:
  `keyboard_first_command_invocation`, `keyboard_reachable_focus_path`,
  or `keyboard_chord_reference`. Keyboard-first rows MUST quote a
  canonical `command_id` from the command registry; chord-reference
  rows MUST quote a `keyboard_chord_ref`.
- Every row MUST set `requires_account_detour` and
  `requires_marketplace_detour` to `false` on claimed beta rows; the
  validator rejects either being `true`.
- Required section kinds: `docs_help_anchors`, `glossary_packs`,
  `migration_reports`, `known_limits`, `recovery_routes`, and
  `first_run_confusion_exits`. The validator rejects any missing.
- Support rows MUST agree with the matching live row on
  `section_kind_token`, `entry_kind_token`, `keyboard_reach_token`,
  `claim_class_token`, `lifecycle_class_token`,
  `freshness_class_token`, `command_id`, `docs_help_anchor_ref`,
  `migration_report_ref`, `known_limit_ref`, and
  `recovery_route_class_token`. Drift is a contract bug the validator
  rejects.
- Support export wrappers MUST set `raw_private_material_excluded` to
  `true`. Raw bodies are never exported.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- page           > fixtures/ux/m3/migration_center/page.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- sections       > fixtures/ux/m3/migration_center/sections.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- entries        > fixtures/ux/m3/migration_center/entries.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- support-rows   > fixtures/ux/m3/migration_center/support_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- support-export > fixtures/ux/m3/migration_center/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- defects        > fixtures/ux/m3/migration_center/defects.json
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- summary        > fixtures/ux/m3/migration_center/summary.json
```

## Verification

```sh
cargo test -p aureline-shell --test migration_center_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- validate
```

## Failure drills

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-account-detour
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-missing-command-id
cargo run -q -p aureline-shell --bin aureline_shell_migration_center -- drill-review-overdue
```
