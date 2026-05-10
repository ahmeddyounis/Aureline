# Activity-center seed fixtures

Worked examples for the durable activity-center / job-row seed.
Reviewer-facing entry point:
[`docs/ux/activity_center_seed.md`](../../../docs/ux/activity_center_seed.md).

Each fixture is a serialized
[`ActivityCenterSnapshot`](../../../crates/aureline-shell/src/activity_center/mod.rs)
record (the projection the chrome reads when it draws the activity-center
pane). The corpus exercises the protected-walk lifecycle, the
restart-survival failure drill, the failed-with-retry drill, and the
notification-router dedupe behavior on the durable row.

- `restore_running_progress.json` — `running` lifecycle with a labeled
  progress descriptor; the row carries the typed reopen target and the
  shared open-job-details command.
- `restore_completed_after_focus_loss.json` — `completed` terminal
  lifecycle. The row keeps `is_terminal=true` and remains reopenable
  after focus loss or a process restart (the file-backed store reloads
  it verbatim).
- `restore_failed_retry_available.json` — `failed` terminal lifecycle
  with `retryability=available`; the snapshot lights
  `honesty_marker_present=true` so the chrome cannot fabricate a
  green row.
- `restore_dedupe_repeat.json` — the same canonical event id observed
  twice; one row, `occurrence_count=2`, the reopen target is
  preserved verbatim.
