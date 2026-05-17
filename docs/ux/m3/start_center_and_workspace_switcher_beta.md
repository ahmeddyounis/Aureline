# Start Center and workspace switcher beta

The beta Start Center is a productivity launcher for the no-workspace
state. It keeps the five work-start actions first, then renders pinned
and recent work from the same recent-work registry used by the
workspace switcher. Account, marketplace, and release content can be
present only after the work-resume surface has already been offered.

The implementation lives in
[`crates/aureline-shell/src/start_center/beta.rs`](../../../crates/aureline-shell/src/start_center/beta.rs).
It reads the workspace recent-work model in
[`crates/aureline-workspace/src/recent_work/`](../../../crates/aureline-workspace/src/recent_work/)
and the shell switcher projection in
[`crates/aureline-shell/src/workspace_switcher/mod.rs`](../../../crates/aureline-shell/src/workspace_switcher/mod.rs).

## Contract Surface

The projection ships these record families under
`shell:start_center_workspace_switcher_beta:v1`:

- `shell_start_center_beta_primary_action_record` for `Open folder`,
  `Open workspace`, `Clone repository`, `Restore last session`, and
  `Import from...`. Each row is keyboard reachable and can render
  before sign-in or network readiness.
- `shell_start_center_workspace_switcher_beta_work_row_record` for
  Start Center and switcher rows. Rows carry target kind, target
  state, failure state, trust state, restore availability,
  last-opened time, pinned/recent section, actions, and the shared
  activation contract.
- `shell_start_center_workspace_switcher_beta_privacy_mode_record`
  for privacy-reduced modes. Hiding paths or recent-work metadata
  never removes the ability to open a folder or workspace file.
- `shell_start_center_workspace_switcher_beta_support_row_record`
  for support-export parity. The row mirrors the Start Center and
  switcher action ids and state tokens for the same recent-work id.
- `shell_start_center_workspace_switcher_beta_support_export_record`
  for the support-export wrapper.

## Acceptance Posture

The validator rejects drift in the core beta promises:

- Primary actions must be present in the required order, in the
  `primary_work_resume` zone, and must not require sign-in or network
  readiness before rendering.
- Start Center rows and switcher rows must agree on the recent-work
  id, target kind, target state, failure state, trust state, restore
  availability, and action ids.
- Missing and moved local rows must keep `Locate`, `Open anyway`, and
  `Remove from list` available. Remote-unavailable rows must keep
  `Reconnect` or `Reauthorize`, `Open anyway`, and `Remove from list`
  available.
- Workspace-switcher rows must preserve `cancel_switch` and
  `reopen_previous_workspace` so a failed switch does not strand the
  prior workspace.
- Support-export rows must match the live row state and keep cleanup
  scoped to recent-work metadata only.

## Headless Inspector

Use the inspector as the mint-from-truth path for fixtures:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- page
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- primary-actions
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- rows
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- switcher-rows
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- privacy-modes
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- validate
```

## Fixtures

Reviewable fixtures live under
[`fixtures/ux/m3/start_center/`](../../../fixtures/ux/m3/start_center/):

- `primary_actions.json`
- `rows.json`
- `switcher_rows.json`
- `privacy_modes.json`
- `page.json`
- `support_export.json`

The fixture test in
[`crates/aureline-shell/tests/start_center_switcher_beta_fixtures.rs`](../../../crates/aureline-shell/tests/start_center_switcher_beta_fixtures.rs)
round-trips those files through the Rust types, validates the page
and support export, and checks that `page.json` exactly matches the
seeded builder.

## Verification

```sh
cargo test -p aureline-workspace recent_work --lib
cargo test -p aureline-shell start_center::beta --lib
cargo test -p aureline-shell --test start_center_switcher_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_start_center -- validate
```
