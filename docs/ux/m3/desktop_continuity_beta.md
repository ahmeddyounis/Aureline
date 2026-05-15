# Desktop continuity (beta)

This page describes the beta-grade runtime-adaptation projection that
lives in `aureline-shell`. It freezes the acceptance states for
suspend-resume, battery/thermal pressure, and multi-monitor continuity
so the live shell, the review packets in
`fixtures/ux/m3/desktop_continuity_matrix/`, and the support export
report the same runtime-adaptation truth.

The projection is the page-level surface above the existing
efficiency-state hooks
([`crates/aureline-shell/src/efficiency/`](../../../crates/aureline-shell/src/efficiency/)),
the desktop continuity alpha packet
([`docs/ux/desktop_continuity_interruption_alpha.md`](../desktop_continuity_interruption_alpha.md)),
and the workspace-management beta projection
([`docs/ux/m3/multi_window_beta.md`](./multi_window_beta.md)). It does
not re-derive any of that truth; it projects the five record classes a
beta reviewer needs to inspect daily.

## Contract surface

The beta projection ships five record kinds, all under the shared
contract ref `shell:runtime_adaptation_beta:v1`:

- `shell_runtime_adaptation_beta_power_posture_record` — power/thermal
  posture row. Carries the posture token, the per-workload pause /
  degrade / deny / keep-running decisions, the protected foreground
  actions retained at this posture, the foreground responsiveness
  promise, and whether the user sees an explicit "adapted posture"
  summary in the activity center.
- `shell_runtime_adaptation_beta_suspend_resume_record` — suspend-resume
  cycle. Carries the host OS, the lifecycle event class
  (`sleep`, `wake_from_sleep`, `hibernate`, `resume_from_hibernate`,
  `lock_unlock_cycle`), the explicit continuity summary
  (`reconnect`, `stale`, `resumed_work`), the local-work-continues /
  privileged-paused / no-silent-rerun invariants, the recovery action
  tokens surfaced to the user, and the requirement that the resume
  summary stays user-visible.
- `shell_runtime_adaptation_beta_monitor_continuity_record` —
  multi-monitor topology event. Carries the host OS, the event class
  (`display_disconnected`, `display_reconnected`, `scale_changed`,
  `arrangement_changed`, `dock_undock_cycle`), the continuity summary
  tokens, the resulting restore fidelity (mirroring the
  `windows_beta` vocabulary), the visible-bounds / focus-intent
  preservation invariants, the topology summary requirement, and an
  optional ref to the matching `windows_beta` restore outcome so a
  reviewer can pivot to the workspace-management page on the same
  topology event.
- `shell_runtime_adaptation_beta_foreground_protection_record` — pinned
  foreground-action protection. Carries the host OS, the posture, the
  three protected actions (`edit_active_buffer`,
  `open_command_palette`, `direct_navigation`), the
  remains-responsive / no-input-event-drops / no-blocking-dialog
  invariants, and the narrative for the action-protection promise.
- `shell_runtime_adaptation_beta_desktop_matrix_record` — per-OS beta
  matrix row. Carries the host OS, the suspend-resume / monitor /
  posture row refs exercised on that OS, the **named downgrade
  behaviors** (e.g. `speculative_prefetch_paused`,
  `extension_polling_denied`, `stranded_window_snapped_to_primary`,
  `cached_view_marked_stale_on_unlock`), and the
  foreground-responsiveness / no-silent-rerun promises that must hold
  across the row.

A `shell_runtime_adaptation_beta_page_record` aggregates the five
record lists together with a summary banner (counts, pressured
postures with degraded background, continuity events that required a
user-visible summary, and OS rows that carry a named downgrade). A
`shell_runtime_adaptation_beta_support_export_record` wrapper quotes
the page plus every `case_id` in stable order so support reviewers
can pivot from a row to the page without separate query plumbing.

## Acceptance posture

The beta projection delivers the M3 desktop-continuity acceptance gates:

- **Foreground editing, palette, and direct navigation remain
  protected.** Every power-posture row carries the three protected
  foreground actions and `foreground_responsiveness_preserved=true`;
  every foreground-protection row carries
  `no_input_event_drops` and `no_blocking_dialog`. The validator
  (`validate_runtime_adaptation_page`) rejects a posture that drops a
  protected action and a foreground row that admits a blocking
  dialog.
- **Background work pauses or degrades under power/thermal pressure.**
  Pressured postures (`low_battery`, `critical_battery`,
  `os_battery_saver`, `thermal_pressure`) must admit at least one
  `pause`, `degrade`, or `deny` decision. The validator rejects a
  pressured posture with no background degradation.
- **Suspend-resume produces explicit reconnect, stale, or
  resumed-work summaries.** Every suspend-resume row sets
  `user_visible_resume_summary_required` and
  `no_silent_rerun_or_authority_reuse`; the validator rejects a
  silent rerun or a missing summary.
- **Monitor changes produce explicit topology summaries.** Every
  monitor row sets `user_visible_topology_summary_required`,
  preserves visible bounds and focus intent, and links optionally to
  the matching `windows_beta` outcome. The validator rejects hidden
  adjustments or a missing summary.
- **The beta desktop matrix is exercised on claimed macOS, Windows,
  and Linux rows with named downgrade behaviors.** The matrix list
  must include one row per supported OS, and each row must carry at
  least one named downgrade behavior. The validator rejects an
  incomplete matrix or an OS row without named downgrades.

## Headless consumers

The beta projection is exercised through the
`aureline_shell_runtime_adaptation` binary. The bin is the only
mint-from-truth path for the JSON checked in under
`fixtures/ux/m3/desktop_continuity_matrix/`, so the live shell records,
the CLI rows, and the support-export rows cannot drift.

```sh
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- page
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- posture
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- suspend
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- monitor
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- foreground
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- matrix
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- validate
```

`validate` exits non-zero (status `3`) if any acceptance invariant is
violated; it is wired so CI can fail closed on a regression in any of
the five record kinds.

## Fixtures

Reviewable fixtures live under
[`fixtures/ux/m3/desktop_continuity_matrix/`](../../../fixtures/ux/m3/desktop_continuity_matrix/):

- `power_posture_rows.json` — AC, low-battery, thermal-pressure, and
  critical-battery postures with named background decisions.
- `suspend_resume_rows.json` — macOS wake-from-sleep, Windows
  lock/unlock, and Linux resume-from-hibernate cycles with named
  continuity summaries.
- `monitor_continuity_rows.json` — macOS display detach, Windows
  dock/undock, and Linux mixed-DPI events with named recovery
  actions.
- `foreground_protection_rows.json` — pinned editing / palette /
  navigation responsiveness across macOS / Windows / Linux at
  pressured postures.
- `desktop_matrix_rows.json` — per-OS beta rows that bundle the
  suspend-resume, monitor, and posture refs exercised on that OS with
  the named downgrade behaviors surfaced to beta users.
- `page.json` — full beta page with the aggregate summary banner.
- `support_export.json` — support-export wrapper that quotes the page
  and every case id.

## Verification

```sh
cargo test -p aureline-shell --test runtime_adaptation_fixtures
cargo test -p aureline-shell --lib runtime_adaptation
cargo run -q -p aureline-shell --bin aureline_shell_runtime_adaptation -- validate
```

The fixture test in
[`crates/aureline-shell/tests/runtime_adaptation_fixtures.rs`](../../../crates/aureline-shell/tests/runtime_adaptation_fixtures.rs)
replays every JSON fixture through the Rust types and asserts the
contract invariants. The fixture test also asserts that the
checked-in `page.json` is bit-for-bit equal to the page returned by
the seeded builder, so regenerating with the headless bin is the only
mint-from-truth path.
