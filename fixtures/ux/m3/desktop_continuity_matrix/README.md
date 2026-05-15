# Desktop continuity (beta) fixture corpus

Reviewable fixtures for the beta runtime-adaptation projection that
lives in
[`crates/aureline-shell/src/runtime_adaptation/`](../../../../crates/aureline-shell/src/runtime_adaptation/).

Each JSON file is a literal projection of the seeded
`RuntimeAdaptationPage` produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_runtime_adaptation.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_runtime_adaptation.rs)).
The inspector is the only mint-from-truth path for these fixtures, so
the checked-in JSON cannot drift from the Rust types.

All records carry the shared contract ref
`shell:runtime_adaptation_beta:v1` so shell UI rows, headless CLI
rows, and support-export rows pivot to the same `case_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`power_posture_rows.json`](./power_posture_rows.json) | Power/thermal postures with background pause/degrade and foreground-protected actions. |
| [`suspend_resume_rows.json`](./suspend_resume_rows.json) | Suspend-resume cycles per OS with explicit reconnect/stale/resumed-work summaries. |
| [`monitor_continuity_rows.json`](./monitor_continuity_rows.json) | Multi-monitor reconnect / detach / mixed-DPI events with named recovery actions. |
| [`foreground_protection_rows.json`](./foreground_protection_rows.json) | Editing, palette, and direct-navigation responsiveness per posture and OS. |
| [`desktop_matrix_rows.json`](./desktop_matrix_rows.json) | Per-OS beta rows (macOS, Windows, Linux) with named downgrade behaviors. |
| [`page.json`](./page.json) | Full beta runtime-adaptation page with aggregate summary banner. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes the page plus every case id. |

## Fixture rules

- Every record carries a stable `case_id` and the shared contract ref
  `shell:runtime_adaptation_beta:v1`; record kinds are stable Rust
  constants.
- Pressured postures (`low_battery`, `critical_battery`,
  `os_battery_saver`, `thermal_pressure`) must admit at least one
  `pause`, `degrade`, or `deny` workload decision. The validator
  rejects a pressured posture with no background degradation.
- Every posture row must carry the three protected foreground actions
  (`edit_active_buffer`, `open_command_palette`, `direct_navigation`)
  and `foreground_responsiveness_preserved=true`. The validator
  rejects a posture that drops a protected action.
- Suspend-resume rows must set `no_silent_rerun_or_authority_reuse`
  and `user_visible_resume_summary_required` to `true`. Silent rerun
  or a missing summary is a contract bug the validator rejects.
- Monitor continuity rows must preserve `visible_bounds_preserved`
  and `focus_intent_preserved`, and must require a user-visible
  topology summary. Hidden adjustments are a contract bug.
- The desktop matrix must include one row per supported OS
  (`macos`, `windows`, `linux`), and each row must carry at least
  one named downgrade behavior.

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- posture        > fixtures/ux/m3/desktop_continuity_matrix/power_posture_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- suspend        > fixtures/ux/m3/desktop_continuity_matrix/suspend_resume_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- monitor        > fixtures/ux/m3/desktop_continuity_matrix/monitor_continuity_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- foreground     > fixtures/ux/m3/desktop_continuity_matrix/foreground_protection_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- matrix         > fixtures/ux/m3/desktop_continuity_matrix/desktop_matrix_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- page           > fixtures/ux/m3/desktop_continuity_matrix/page.json
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- support-export > fixtures/ux/m3/desktop_continuity_matrix/support_export.json
```

## Verification

```sh
cargo test -p aureline-shell --test runtime_adaptation_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- validate
```
