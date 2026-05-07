# Background-work queue contract

This document freezes the shared runtime contract Aureline uses to
decide how background jobs enter, collapse, checkpoint, and cancel
inside the five shared queue lanes (`foreground`,
`interactive_background`, `maintenance`, `provider_overlay`,
`upload_replication`). It exists so typing, save, undo, and
current-task visibility stay ahead of speculative or optional work
no matter which capability discovers a new reason to do work in the
background.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document must be updated in the same change.

## Companion artifacts

- [`/artifacts/runtime/queue_lane_matrix.yaml`](../../artifacts/runtime/queue_lane_matrix.yaml)
  — machine-readable lane / job-kind / collapse-policy / checkpoint
  / staleness / cancellation / budget-domain matrix, the shell-ready
  budget-protection flag per workload lane, and the seeded
  coalesce / replace / restart diagnostics.
- [`/artifacts/runtime/lane_arbitration_rules.md`](../../artifacts/runtime/lane_arbitration_rules.md)
  — stable arbitration names and collapse-key templates used across scheduler,
  benchmarks, and support exports.
- [`/artifacts/runtime/checkpoint_resume_examples.yaml`](../../artifacts/runtime/checkpoint_resume_examples.yaml)
  — checkpoint/resume proof corpus for pressure, suspend/resume, and cancellation.
- [`/schemas/runtime/background_job.schema.json`](../../schemas/runtime/background_job.schema.json)
  — boundary schema for the background-job record, the queue-
  diagnostics record, and the collapse-event record every surface,
  support export, and future scheduler projects.
- [`/fixtures/runtime/queue_cases/`](../../fixtures/runtime/queue_cases/)
  — worked fixtures that freeze coalesce, replace, restart,
  startup-restore protection, provider-refresh replacement, and
  protected-foreground-not-starved scenarios.
- [`/docs/runtime/resource_governor_contract.md`](./resource_governor_contract.md)
  — shared work-class, lane, governor-state, and visible health-
  state vocabulary this contract reuses rather than renaming.
- [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — admission matrix, lane depth / oldest-age thresholds, governor
  state transitions, and shed order the queue contract reads in
  tandem with this matrix.
- [`/docs/perf/efficiency_state_policy.md`](../perf/efficiency_state_policy.md)
  — battery, thermal, and hidden-pane suppression policy. The
  efficiency policy points at the same lane and budget-domain ids
  this matrix declares.
- [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — scope, workset, and authority vocabulary jobs quote when they
  name a collapse key, a staleness reason, or a cancellation
  contract.

## Scope and authority

This contract does not introduce a second scheduler. The resource
governor remains the canonical owner of:

- admission decisions;
- shed, defer, pause, cancel, and deny decisions;
- governor health-state transitions; and
- the projected visible health state that shell surfaces, support
  exports, and future scheduler dashboards explain.

This contract freezes four additional questions the general
resource-governor contract leaves at the queue layer:

1. Which job kinds may enter which lane, with which work class and
   which budget domain.
2. How duplicate or superseded background jobs collapse so queue
   depth stays bounded.
3. What checkpoint, staleness, and cancellation contracts every job
   kind MUST declare so pausing, resuming, and cancelling is safe.
4. What diagnostics every lane and every job MUST expose so support
   bundles, About / service-health panels, and the later scheduler
   reason about queue posture from one schema.

The eventual queue engine and the full runtime scheduler
implementation remain out of scope. This contract is the shape they
implement against.

## Shared vocabulary

The queue contract reuses the five shared queue lanes from
[`resource_governor_contract.md`](./resource_governor_contract.md)
without renaming:

- `foreground`
- `interactive_background`
- `maintenance`
- `provider_overlay`
- `upload_replication`

It reuses the six shared work classes from the same contract:

- `core_interaction`
- `core_navigation`
- `short_foreground_task`
- `background_knowledge_work`
- `optional_assistance`
- `upload_and_replication`

It reuses the five governor health states:

`nominal -> constrained -> degraded -> protect_core -> recovery`

Surfaces project queue posture through the same visible health-
state vocabulary the governor owns: `ready`, `warming`, `partial`,
`degraded`, `offline`, `unsupported`, `overloaded`.

## Budget domains

Every lane declares one budget domain. A budget domain is the
counter the governor and the efficiency-state policy both read and
narrow. Budget domains are frozen here so later scheduler or
efficiency code references one name, not a local synonym:

| Budget domain | Narrowed by governor state | Narrowed by efficiency state | Notes |
|---|---|---|---|
| `hot_path_interactive_budget` | reserved; never shrunk below the `core_interaction` / `core_navigation` floor | never shrunk below protected hot-path floor | typing, save, undo, current-task visibility, visible paint |
| `foreground_task_budget` | shrunk in degraded; shrunk hard in protect-core | shrunk on battery saver and thermal critical | explicit user-run commands, debug step, Git status |
| `knowledge_refresh_budget` | collapsed in constrained; hot-set only in degraded; paused in protect-core | paused on battery saver and thermal critical | index scan, graph warmup, semantic refresh |
| `maintenance_budget` | throttled in constrained; deferred in degraded; denied in protect-core | paused on battery saver; disabled on thermal critical | hidden preview warmers, extension polling, marketplace refresh |
| `provider_overlay_budget` | separate retry / circuit-breaker budget; narrowed when the remote impairment thresholds trip | trimmed on battery saver | remote attach auth / probe retries, provider overlay refresh |
| `replication_budget` | batched in nominal; deferred in constrained; paused in degraded | batched on battery saver; deferred on thermal critical | telemetry forward, crash upload, support bundle upload, sync publish |

Every workload lane (next section) binds to exactly one budget
domain. No lane may borrow hot-path interactive budget.

## Workload lanes

Workload lanes name the specific background jobs Aureline already
knows it will run. Each workload lane binds to one of the five
shared queue lanes, to one work class, and to one budget domain.
Each workload lane declares whether it may `steal_shell_ready_budget`
— that flag is `false` for every workload that could plausibly
compete with typing, save, undo, or current-task visibility.

| Workload lane | Queue lane | Work class | Budget domain | `steal_shell_ready_budget` | Typical jobs |
|---|---|---|---|---|---|
| `startup_restore` | `interactive_background` | `background_knowledge_work` | `knowledge_refresh_budget` | **false** | rehydrating the last-opened workset, replaying pinned tabs and cursors, restoring split layout, resuming warming indexers from checkpoint |
| `hot_set_scan` | `interactive_background` | `background_knowledge_work` | `knowledge_refresh_budget` | **false** | open-file and visible-symbol freshness, hot-set search priming, outline / diagnostics first-paint |
| `graph_warmup` | `maintenance` | `background_knowledge_work` | `knowledge_refresh_budget` | **false** | semantic graph build, references catalog, cross-file link resolution |
| `provider_refresh` | `provider_overlay` | `background_knowledge_work` | `provider_overlay_budget` | **false** | remote attach warm, provider overlay refresh, tenant policy / feature flag refresh, docs/model catalog refresh |
| `restore_rebind` | `provider_overlay` | `background_knowledge_work` | `provider_overlay_budget` | **false** | rebinding remote session handles after reconnect, replaying resync-required subscriptions, re-issuing capability envelopes and subscription handles |
| `workspace_index_full` | `maintenance` | `background_knowledge_work` | `knowledge_refresh_budget` | **false** | full-workspace index, full-workspace graph rebuild |
| `extension_timer` | `maintenance` | `optional_assistance` | `maintenance_budget` | **false** | extension background polling, speculative warmup, non-visible previews |
| `ai_context_expansion` | `maintenance` | `optional_assistance` | `maintenance_budget` | **false** | AI embeddings, context expansion, marketplace refresh, model warmup |
| `telemetry_forward` | `upload_replication` | `upload_and_replication` | `replication_budget` | **false** | opt-in telemetry, crash upload, support-bundle upload, sync publish |

Only `core_interaction` and `core_navigation` work — the shell's
protected hot path — is allowed to consume the
`hot_path_interactive_budget` reserved capacity. No workload lane
above may preempt a frame, delay a keystroke, stall save, or defer
undo. Startup restore, hot-set scan, graph warmup, provider refresh,
and restore-rebind specifically disclaim that capacity because they
are the typical early-session jobs that could otherwise race the
first keystroke.

## Job identity fields

Every background job carries the identity fields below. The schema
in `schemas/runtime/background_job.schema.json` is the boundary
shape; surfaces, support bundles, replay artifacts, and the future
scheduler all project back to these names.

- **`job_id`** — opaque id allocated at enqueue. Safe to log and
  safe on support exports.
- **`job_kind`** — canonical dotted lower-snake kind (for example
  `knowledge.hot_set_scan`, `provider.overlay_refresh`,
  `startup.workspace_restore`). Frozen by the job-kind vocabulary
  in `queue_lane_matrix.yaml`.
- **`workspace_id`** — workspace the job runs under. Null is
  admitted only for unscoped ambient workloads (for example,
  crash-upload for a previous workspace that is no longer open).
- **`profile_id`** — active profile id when an Aureline profile is
  bound. Null when no profile is active.
- **`slice_id`** — workset / sparse-slice / review-workspace /
  companion-surface id. Null when the job is workspace-wide.
- **`scope`** — discriminated scope descriptor reusing the
  `scope_class` vocabulary from the execution-context schema
  (`current_root`, `named_workset`, `sparse_slice`,
  `full_workspace`, `policy_limited_view`, `review_workspace`,
  `companion_surface`, `ambient`). The `ambient` scope is reserved
  for jobs like crash upload that are not bound to a workspace.
- **`initiating_source`** — why this job exists:
  `user_action`, `session_startup`, `workspace_open`,
  `profile_change`, `policy_change`, `focus_change`,
  `file_change_notification`, `remote_reconnect`,
  `scheduler_timer`, `extension_request`, `ai_tool_call`,
  `support_export_request`, `sync_trigger`, `recovery_resume`.
- **`collapse_key`** — structured key used by the queue to
  identify duplicate or superseded work. Every collapse_key
  includes `job_kind` and at least one of `workspace_id`,
  `slice_id`, or `scope_ref`. Collapse keys may include a
  `phase` field when the job declares phases.
- **`collapse_policy`** — frozen collapse behavior:
  - `coalesce_stale_duplicates` — an in-flight job of the same key
    is kept; new arrivals fold into its next output.
  - `replace_superseded` — the newest arrival wins; any in-flight
    job with the same key is cancelled at its next checkpoint.
  - `restart_after_supersede` — like `replace_superseded`, but the
    new job restarts from the last-good checkpoint rather than
    from scratch.
  - `serialize_exact_duplicates` — only exact-duplicate re-enqueue
    is suppressed; otherwise the job runs normally.
  - `none` — no collapse; every enqueue runs. Reserved for jobs
    that cannot be merged without correctness loss.
- **`checkpoint_policy`** — frozen checkpoint behavior:
  - `none` — the job has no resume boundary and must run to
    completion or be cancelled outright. Forbidden on lanes whose
    governor admission includes pause.
  - `explicit_phase_boundary` — the job declares phases; resume
    happens at phase boundaries.
  - `item_or_time_boundary` — the job checkpoints per processed
    item or on a declared time interval.
  - `resumable_chunk_boundary` — the job processes chunks and may
    resume at any chunk boundary. Used by uploads and replication.
- **`staleness_policy`** — what to do when a job that was paused,
  deferred, or queued reaches the front of the lane:
  - `drop_if_stale` — drop the job if its inputs are no longer
    current or its initiating scope no longer exists.
  - `re_queue_if_still_relevant` — re-enqueue with a fresh
    `created_at` if its scope is still live.
  - `refresh_on_resume` — re-fetch the inputs before running.
  - `never_stale` — the job remains valid regardless of elapsed
    time (for example, crash upload).
- **`cancellation_contract`** — what cancellation means for this
  job:
  - `cancel_immediate` — the job cooperates with immediate
    cancellation at the next safe point; any partial output is
    discarded.
  - `checkpoint_then_cancel` — the job must reach the next
    checkpoint before acknowledging cancellation; partial output
    is preserved under the declared checkpoint.
  - `cancel_after_phase` — cancellation is acknowledged at the
    next phase boundary.
  - `no_inflight_cancel` — the job cannot be cancelled once
    started. Forbidden outside `core_interaction` work and
    `core_navigation` work; never valid for background lanes.
- **`priority_hint`** — bounded ordering hint inside the lane:
  `protected_foreground`, `visible_work_support`, `normal`,
  `background_idle`, `deferred`. The lane's admission matrix, not
  the priority hint, decides whether the job runs; the hint only
  orders work within an already-admitted lane.

## Collapse, checkpoint, and cancel semantics

1. **Duplicates collapse before queue depth grows.** An arriving
   enqueue whose `collapse_key` matches an in-flight or queued job
   MUST be folded under the declared `collapse_policy`. Unbounded
   duplicate accumulation is non-conforming.
2. **Replace and restart are distinct.** `replace_superseded`
   cancels the prior job at its next checkpoint and starts a new
   job from scratch. `restart_after_supersede` cancels the prior
   job at its next checkpoint and starts from its last-good
   checkpoint. Surfaces and support exports MUST record which of
   the two happened; calling a replace a restart is
   non-conforming.
3. **Every pauseable job has a resume boundary.** A job with
   `checkpoint_policy = none` MAY NOT be paused by the governor;
   pressure in its lane must shed other jobs first. A job that
   enters the governor's `degraded` or `protect_core` posture
   without a resume boundary is non-conforming.
4. **Cancellation preserves truth.** A job's
   `cancellation_contract` MUST be honored. `cancel_immediate`
   jobs discard partial output; `checkpoint_then_cancel` jobs
   preserve the checkpoint they reach; `cancel_after_phase` jobs
   wait for the phase boundary. Silently converting between
   contracts is non-conforming.
5. **Explicit user cancellation outranks the queue.** A user
   pressing cancel, closing a task, saving, or restoring always
   wins over background collapse, checkpointing, or replay.
6. **Startup lanes do not steal shell-ready budget.** Jobs in the
   `startup_restore`, `hot_set_scan`, `graph_warmup`,
   `provider_refresh`, `restore_rebind`, `workspace_index_full`,
   `extension_timer`, `ai_context_expansion`, and
   `telemetry_forward` workload lanes MUST NOT delay a keystroke,
   frame, save, undo, or current-task visibility. Their
   `steal_shell_ready_budget` flag is `false` and their admission
   narrows before the shell's `hot_path_interactive_budget` is
   touched.

## Diagnostics

Every lane MUST expose the diagnostic fields below. They are the
fields the support-bundle exporter, the About / service-health
panel, the later scheduler, and the QA fixture harness read when
they reason about queue posture. The boundary schema names them
with these exact ids.

- **`lane`** — one of the five shared queue lanes.
- **`lane_depth`** — current enqueued + in-flight job count in the
  lane.
- **`oldest_age_seconds`** — seconds since the oldest enqueued job
  was created. Null when the lane is empty.
- **`collapse_count`** — cumulative count of enqueues folded by
  the collapse policy since the supervisor session started. This
  is the counter that makes queue pressure relief visible.
- **`coalesce_count`** — subcount of `collapse_count` under the
  `coalesce_stale_duplicates` policy.
- **`replace_count`** — subcount of `collapse_count` under the
  `replace_superseded` policy.
- **`restart_count`** — subcount of `collapse_count` under the
  `restart_after_supersede` policy.
- **`cancellation_lag_ms`** — p95 milliseconds between a
  cancellation request and the target job acknowledging (or
  reaching its next checkpoint when
  `checkpoint_then_cancel` applies). Null when no cancellation
  has occurred in the current window.
- **`last_checkpoint`** — last-good checkpoint for any job that
  declared a checkpoint policy: `{ checkpoint_class,
  checkpoint_ref, epoch, captured_at }`. Null when the lane has
  no checkpointed job.
- **`budget_domain`** — the budget domain the lane binds to.
  Copied from the workload lane of the dominant in-flight job;
  defaults to the lane's primary domain when idle.
- **`governor_state_at_sample`** — the governor health state at
  sample time. Joins back to
  `resource_governor_thresholds.yaml`.
- **`visible_projection`** — the visible health state surfaces
  project for this lane (`ready`, `warming`, `partial`,
  `degraded`, `offline`, `unsupported`, `overloaded`).

Per-job diagnostics carried on the background-job record are:

- **`enqueued_at`** and **`started_at`**.
- **`current_phase`** — phase id for jobs with phase boundaries;
  null otherwise.
- **`last_checkpoint_ref`** — opaque ref to the last persisted
  checkpoint.
- **`collapse_origin_job_id`** — set when the job is the survivor
  of a collapse; identifies the first job in the collapse chain.
- **`superseded_job_ids`** — set when the job superseded earlier
  enqueues under `replace_superseded` or
  `restart_after_supersede`.

## Governor and efficiency-state pointers

Every lane id and every budget-domain id declared here is valid as a
reference target from
[`resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
and from
[`docs/perf/efficiency_state_policy.md`](../perf/efficiency_state_policy.md).
Those surfaces read the matrix in
[`queue_lane_matrix.yaml`](../../artifacts/runtime/queue_lane_matrix.yaml)
by id and do not mint parallel lane or budget-domain names. A
later resource-governor or efficiency-state rule referencing an
unknown lane or domain id is non-conforming.

## Seeded diagnostics scenarios

The companion YAML carries the machine-readable rows. The families
are frozen here so later implementations map into one shape:

- **Duplicate index scan — coalesced.** A file-change notification
  re-enqueues `knowledge.hot_set_scan` while a scan is in flight.
  The in-flight job absorbs the new work; `collapse_count` and
  `coalesce_count` both increment. The lane does not grow.
- **Provider overlay refresh — replaced.** A new provider-overlay
  refresh arrives while a stale refresh is running. The stale
  refresh cancels at its next checkpoint and the new refresh
  starts from scratch; `collapse_count` and `replace_count`
  increment. Surfaces project `warming` on the provider overlay
  until the new refresh acknowledges.
- **Graph warmup — restarted from checkpoint.** A workset change
  supersedes an in-flight graph warmup. The warmup cancels at its
  phase boundary and a new warmup starts from the last-good
  checkpoint; `collapse_count` and `restart_count` increment. The
  governor lane depth stays flat.
- **Startup restore — protected foreground.** At session startup,
  `startup.workspace_restore` and `provider.overlay_refresh`
  enqueue together with the user's first keystroke. The keystroke
  runs on the `hot_path_interactive_budget`; the background
  lanes do not acquire any of that budget. Lane diagnostics show
  the background lanes `warming`; the editor stays `ready`.
- **Restore rebind after reconnect.** A remote connector reconnects
  and `restore_rebind` jobs re-issue subscription handles. The
  lane uses `provider_overlay_budget`, not hot-path budget;
  surfaces project `warming` until rebind completes.
- **Telemetry backlog paused under protect-core.** The governor
  enters `protect_core`. The `upload_replication` lane pauses;
  `telemetry.forward` jobs preserve queued state, and
  `cancellation_lag_ms` stays null because pausing, not
  cancelling, is the contract.

The important property is not the exact diagnostic values. The
property is that every lane, every job kind, every collapse, and
every cancellation resolves one frozen vocabulary — not a per-
subsystem invention — so support, recovery, and later scheduler
work reason about the same shape.
