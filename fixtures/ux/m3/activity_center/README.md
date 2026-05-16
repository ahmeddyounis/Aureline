# Activity center (beta) fixture corpus

Reviewable fixtures for the beta activity-center projection that lives
in
[`crates/aureline-shell/src/activity_center/beta.rs`](../../../../crates/aureline-shell/src/activity_center/beta.rs).

Each JSON file is a literal projection of the seeded
`ActivityCenterBetaPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_activity_center.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_activity_center.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:activity_center_beta:v1` so shell UI rows, headless CLI rows,
the row badges, and support-export rows pivot to the same `row_id`
and `case_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`rows.json`](./rows.json) | Durable activity rows across indexing, restore, install/update, task, test, and git/review job families with exact / placeholder / denial reopen classes. |
| [`badges.json`](./badges.json) | Row-aligned badge mirror that echoes the row's job family, state, resolution, and partition. |
| [`page.json`](./page.json) | Full beta activity-center page with aggregate summary banner (row, reopen, retry, and attention counts). |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page, per-row export rows, and every `case_id` in stable page order. |

## Fixture rules

- Every record carries a stable `case_id`, `row_id`, and the shared
  contract ref `shell:activity_center_beta:v1`; record kinds are
  stable Rust constants.
- Every row promises one of three authoritative reopen classes:
  `exact_durable_object` (requires `exact_target_identity_ref`),
  `truthful_placeholder` (requires `placeholder_reason_label`), or
  `denied_and_explained` (requires `denial_reason_label`). A generic
  home fallback is rejected by the validator.
- Every row carries `recoverable_without_toast=true` and
  `reopenable_after_toast_expiry=true`; the validator rejects a row
  that admits a toast-only recovery path.
- Long-running or retryable rows must expose a durable affordance:
  either `durable_acknowledge_available=true` or a non-`not_applicable`
  retry posture. Durable retry, when offered, must be bound to a
  typed `retry_command_id` and not to a toast button.
- Badges, rows, and support-export rows must agree on `job_family`,
  `state_class`, `resolution_class`, `activity_partition`, and
  `exact_reopen_identity_ref`. Drift is a contract bug the validator
  rejects.
- The page must cover the six claimed beta job families
  (`indexing`, `restore`, `install_update`, `task_run`, `test_run`,
  `git_review`).

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- page           > fixtures/ux/m3/activity_center/page.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- rows           > fixtures/ux/m3/activity_center/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- badges         > fixtures/ux/m3/activity_center/badges.json
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- support-export > fixtures/ux/m3/activity_center/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test activity_center_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_activity_center -- validate
```
