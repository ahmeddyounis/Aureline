# Activity-center / durable job-row seed

The activity center is the durable truth surface for long-running task
progress in the live shell. This document is the reviewer-facing entry
point for the M1 seed: what is real, what is reserved, how to exercise
the protected walk and failure drill, and where to find the source.

## Companion artifacts

- [`/crates/aureline-shell/src/activity_center/mod.rs`](../../crates/aureline-shell/src/activity_center/mod.rs)
  — `ActivityCenterRow`, `ActivityCenterSnapshot`, and the
  `ActivityCenterStore` (in-memory or file-backed home for rows).
- [`/crates/aureline-shell/src/activity_center/restore_job.rs`](../../crates/aureline-shell/src/activity_center/restore_job.rs)
  — restore-job source that mints typed notification envelopes per
  lifecycle phase and converts them to durable observations.
- [`/fixtures/ux/activity_center_cases/`](../../fixtures/ux/activity_center_cases/)
  — worked examples covering the protected walk, restart survival,
  failed-with-retry, and dedupe drills.

## Upstream contracts the seed consumes (does not fork)

- [`docs/ux/notification_envelope_contract.md`](./notification_envelope_contract.md)
  and the typed envelope at
  [`/schemas/ux/notification_envelope.schema.json`](../../schemas/ux/notification_envelope.schema.json)
  own privacy classes, severity, dedupe schemes, reopen targets, and
  stable command-backed actions. The activity center reads these
  verbatim.
- The
  [`crate::notifications::router::NotificationRouter`](../../crates/aureline-shell/src/notifications/router.rs)
  owns dedupe-aware routing onto the `durable_job_row` surface (and
  every other recommended surface). The activity center reads
  `RoutedNotification` records — it never invents a parallel routing
  vocabulary.
- [`aureline_recovery::session_restore::proposal::RestoreProposal`](../../crates/aureline-recovery/src/session_restore/proposal.rs)
  is the upstream task object behind the first wired long-running
  task class (restore). The restore-job source derives lifecycle
  observations from the proposal counts; it does not read state from a
  local widget timer.

## What the seed delivers in M1

The seed wires **one** real long-running task class — restore — through
the durable activity-center lane end-to-end:

1. The recovery flow builds a `RestoreProposal` from the persisted
   workspace-authority checkpoint, window-topology snapshot, and
   crash-journal entries.
2. The restore-job source mints typed notification envelopes for each
   lifecycle phase (`Preparing`, `Running` with progress, terminal
   `Completed` / `Failed`).
3. The shared `NotificationRouter` routes the envelope onto the
   `durable_job_row` surface (and `status_item`).
4. The `ActivityCenterStore` records the `(routed, observation)` pair
   into a row keyed by `canonical_event_id`. Lifecycle progression
   updates the row in place; `minted_at` is preserved across updates,
   `last_observed_at` is bumped on every observation.
5. When file-backed, the store rewrites a single JSON file on every
   observation. Reopening the same path returns the same rows — a
   completed or failed row remains reviewable after a process
   restart.

Other long-running task classes (index warmup, build, support export)
are explicitly reserved. The seed-scope notice on every snapshot says
so verbatim so a reviewer can see the lane's shape without inferring
depth from row counts.

## Protected walk

1. Trigger a real restore pass (e.g., relaunch after the recovery flow
   builds a proposal).
2. Watch the durable row update through `Preparing` → `Running` (with
   numerator / denominator surfaces from the proposal counts) →
   `Completed`.
3. Lose focus / close the toast / restart the process.
4. Reopen the activity center on the same workspace and confirm the
   row is still there with the preserved canonical event id, reopen
   target, summary label, and lifecycle.

The fixtures exercise this end-to-end:

- `restore_running_progress.json` shows steps 1–2 (in-flight row).
- `restore_completed_after_focus_loss.json` shows step 4 (terminal
  row remains reopenable).

## Failure drill

1. Start a restore pass.
2. Force the underlying snapshot read to fail (or simulate it via the
   `RestoreLifecyclePhase::Failed` source path).
3. Confirm the activity row flips to `Failed` with
   `retryability=available` and the typed detail label, and that the
   snapshot lights `honesty_marker_present=true`.
4. Restart the process; confirm the row reloads with the same terminal
   lifecycle so the failure does not silently disappear.

The `restore_failed_retry_available.json` fixture captures the row
shape that step 3 produces.

## Dedupe drill

The `NotificationRouter` collapses repeats on the dedupe key. The
activity center honors that dedupe and increments `occurrence_count`
on the row instead of spawning a duplicate row. The reopen target
remains exact across deduped emissions so a reviewer who clicks the
deduped row is led back to the same canonical object.

The `restore_dedupe_repeat.json` fixture captures one row with
`occurrence_count=2`.

## What the seed does NOT own

- A render layer (chrome / pixels). The chrome consumes the snapshot
  and the routed surface row but is not part of this seed.
- A queue scheduler, a global progress emitter, or a parallel job
  store. The activity center is one home; richer subsystem
  productization is M2/M3 work.
- New lifecycle classes beyond the seed-relevant subset
  (`preparing`, `running`, `completed`, `failed`, `cancelled`).
  Additions require a schema-version bump on `ActivityCenterRow`.

## Privacy and support-export posture

The row carries the upstream `privacy_class` and `redaction_class`
verbatim, plus the reopen target and a single primary action bound
to a stable `command_id` (`cmd:activity.open_job_details`). Support
exports can quote the snapshot directly: row identity, lifecycle,
retryability, progress, detail, and reopen target are all
serializable, with no raw paths or raw payloads.
