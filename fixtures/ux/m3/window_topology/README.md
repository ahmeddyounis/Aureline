# Multi-window beta fixture corpus

Reviewable fixtures for the beta workspace-management projection that
lives in
[`crates/aureline-shell/src/windows/`](../../../../crates/aureline-shell/src/windows/).

Each JSON file is a literal projection of the seeded `WindowsBetaPage`
produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_windows.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_windows.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref `shell:windows_beta:v1` so
shell UI rows, headless CLI rows, and support-export rows pivot to the
same case id.

## Index

| Fixture | Coverage |
| --- | --- |
| [`split_intents.json`](./split_intents.json) | Split-window intents (axis, weights, focus-after-split, no silent buffer fork). |
| [`detach_intents.json`](./detach_intents.json) | Pane detach into a new window (live authority preserved, source close attributed to detach). |
| [`move_intents.json`](./move_intents.json) | Cross-window move for tabs, diff/review, and inspector surfaces with workspace truth preserved. |
| [`restore_outcomes.json`](./restore_outcomes.json) | Restore after mixed-DPI normalization, display detach, and dock/undock changes. |
| [`page.json`](./page.json) | Full beta workspace-management page with aggregate summary banner. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page plus every case id. |

## Fixture rules

- Every record carries a stable `case_id` and the shared contract ref
  `shell:windows_beta:v1`; record kinds are stable Rust constants.
- Cross-window move records always set
  `canonical_workspace_truth_preserved` to `true` — a `false` value is
  a contract bug that the validator rejects.
- Restore outcomes always include the user-visible "layout adjusted"
  note whenever fidelity drops below `exact_restore`.
- Restore outcomes always promise that recovery-critical chrome (title
  context, restore details, command palette, keyboard focus, activity
  center) stays reachable; the validator rejects a hidden assurance.
- Live-capability surfaces (terminal, notebook) restored as
  `evidence_only_placeholder` must require an explicit user rerun.
  Silent rerun is a contract bug that the validator rejects.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_windows -- split        > fixtures/ux/m3/window_topology/split_intents.json
cargo run -q -p aureline-shell --bin aureline_shell_windows -- detach       > fixtures/ux/m3/window_topology/detach_intents.json
cargo run -q -p aureline-shell --bin aureline_shell_windows -- move         > fixtures/ux/m3/window_topology/move_intents.json
cargo run -q -p aureline-shell --bin aureline_shell_windows -- restore      > fixtures/ux/m3/window_topology/restore_outcomes.json
cargo run -q -p aureline-shell --bin aureline_shell_windows -- page         > fixtures/ux/m3/window_topology/page.json
cargo run -q -p aureline-shell --bin aureline_shell_windows -- support-export > fixtures/ux/m3/window_topology/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test windows_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_windows -- validate
```
