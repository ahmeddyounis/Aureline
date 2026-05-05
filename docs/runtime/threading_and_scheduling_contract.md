# Threading and scheduling contract

This document freezes Aureline's **runtime worker-class and scheduling-lane
model**: where work is allowed to run, what it may block on, how it yields or
is cancelled, and what diagnostics must exist so benchmark traces and support
exports can explain *why* something ran when it did.

The contract is normative. If it disagrees with the PRD, Technical Architecture
Document, Technical Design Document, UI / UX spec, or Design System Style Guide,
those source documents win and this document must be updated in the same
change.

## Companion artifacts

- [`/docs/architecture/service_topology_and_process_placement.md`](../architecture/service_topology_and_process_placement.md)
  — which planes and processes own work and the baseline "do not block the hot
  path" rules.
- [`/artifacts/architecture/process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
  — process roles plus the existing repo-wide scheduling-class vocabulary.
- [`/docs/runtime/resource_governor_contract.md`](./resource_governor_contract.md)
  — governor health states, work classes, visible health-state projection, and
  the policy owner of admission / shedding decisions.
- [`/docs/runtime/background_queue_contract.md`](./background_queue_contract.md)
  — background queue lanes, collapse/checkpoint/cancellation contracts, and the
  queue-diagnostics boundary.
- [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — queue-depth / oldest-age thresholds, shed order, and protect-core posture.
- [`/artifacts/runtime/queue_lane_matrix.yaml`](../../artifacts/runtime/queue_lane_matrix.yaml)
  — frozen queue-lane + workload-lane vocabulary for background work.
- [`/schemas/runtime/worker_class.schema.json`](../../schemas/runtime/worker_class.schema.json)
  — boundary schema for worker-class scheduling decisions, yields, and lane
  diagnostics.
- [`/fixtures/runtime/scheduling_cases/`](../../fixtures/runtime/scheduling_cases/)
  — worked fixtures that show indexing, extensions, AI jobs, and sync refresh
  degrading before hot paths are harmed.

## Scope and non-goals

This contract:

- defines the worker-class vocabulary and the minimum set of scheduling fields
  every work item must declare;
- defines what *must never* happen on the hot path (blocking I/O, unbounded
  waits, silent priority inversion); and
- defines the diagnostics surfaces and exported record shapes used to explain
  queueing, yielding, starvation, and shedding.

This contract does **not** ship a production scheduler implementation. Any
runtime may implement these rules via OS thread priorities, cooperative task
executors, work-stealing pools, or a later centralized scheduler; but it must
project back to the fields and vocabularies frozen here.

## Definitions

- **Worker class** — a CPU scheduling placement and behavioral contract. It
  names *where* work is allowed to execute (UI thread, render thread, bounded
  foreground pool, yieldable maintenance pool, helper process group), and the
  prohibitions and yield/cancel rules that come with that placement.
- **Queue lane** — the background-job lane vocabulary frozen by the background
  queue contract (`foreground`, `interactive_background`, `maintenance`,
  `provider_overlay`, `upload_replication`). Queue lanes are about *admission*
  and *collapse*; worker classes are about *execution placement and priority*.
- **Protected hot path** — the work that must remain responsive under pressure:
  UI input dispatch, render submission, visible edit state updates, undo/redo,
  save intent capture, and visible task control.
- **Priority inversion** — lower-priority work indirectly blocks higher-priority
  work (typically via contended locks, blocking waits, or synchronous cross-lane
  calls). Silent priority inversion is non-conforming.

## Required scheduling fields (every work item)

Every unit of work that can be delayed, queued, throttled, cancelled, or
diagnosed MUST declare the fields below on its scheduling envelope, trace span,
or support-export record:

1. **`worker_class`** — execution placement class (closed vocabulary, below).
2. **`priority_policy`** — the class's priority band inside the host.
3. **`yieldability`** — whether and how the work yields under pressure.
4. **`admission_control`** — how the governor may defer/deny this work.
5. **`concurrency_cap`** — the effective cap in effect for this class at the
   moment the work was admitted (may vary by device, profile, and governor /
   efficiency state, but it must be explicit in diagnostics).
6. **`cancellation_contract`** — whether cancellation is immediate, deferred to
   safe points, deferred to checkpoints, or forbidden in-flight.
7. **`latency_budget_owner`** — which plane/owner is accountable for meeting the
   declared latency budget (or explicitly `no_latency_budget` for best-effort
   work).
8. **Instrumentation labels** — stable labels used across logs/traces/support
   exports: at minimum `worker_class`, `priority_policy`, `queue_lane` (when
   applicable), `work_class` and `budget_domain` (when applicable), plus a
   per-capability `task_kind` or `job_kind` identifier.

The `schemas/runtime/worker_class.schema.json` boundary schema is the concrete
shape that support exports and diagnostic fixtures conform to.

## Worker-class vocabulary (frozen)

Worker classes are a closed vocabulary. Adding a new worker class is an
additive change that MUST update:

- this document;
- `schemas/runtime/worker_class.schema.json`; and
- at least one fixture in `fixtures/runtime/scheduling_cases/` demonstrating the
  new class under pressure.

Repurposing an existing worker class id is breaking.

### Worker-class matrix

Each row below is the **default** contract for the class. The governor and the
efficiency-state policy may narrow concurrency and admission, but they may not
weaken the forbidden-work rules.

| `worker_class` | Purpose | Default `priority_policy` | `yieldability` | Default placement | Forbidden work (non-exhaustive) |
|---|---|---|---|---|---|
| `ui_input` | UI event dispatch, command routing entry, visible edit state | `highest_priority` | `non_yieldable_hot_path` | shell process main/UI thread | blocking filesystem/network I/O, process launch, waits on background completion, unbounded locks |
| `render_submission` | scene close, damage classification, frame submit/present | `high_priority_bounded` | `yield_at_safe_points_only` | shell process render thread | blocking filesystem/network I/O, process launch, waits on background completion, unbounded locks |
| `foreground_user_intent` | visible navigation, quick-open/search refinement, save handoff | `high` | `yield_at_safe_points_only` | shell process bounded foreground pool | long-running workspace scans, unbounded retries, durable export work |
| `interactive_language_actions` | hover/completion/rename-format requests, language host RPC | `high` | `yield_with_checkpoint` | language host / bounded interactive pool | full-repo scans, long "best effort" crawls, blocking shell waits |
| `task_debug_control` | task/test/debug control paths, PTY/Git helper routing | `medium_high` | `yield_with_checkpoint` | helper control path (supervisor + helpers) | anything that blocks shell input/render waiting for completion |
| `indexing_search_maintenance` | watcher normalization, indexing, search/graph maintenance | `medium_yieldable` | `yield_with_checkpoint` | knowledge worker group | holding locks needed by hot path; synchronous calls back into UI/render paths |
| `extension_background_work` | extension timers/polls, background commands, previews | `low_capped` | `yield_with_checkpoint` | extension host pool | unbounded CPU, unbounded I/O, silent background mutation, blocking hot path |
| `ai_background_jobs` | embeddings, context expansion, model warmup/refresh | `low_to_medium_budgeted` | `yield_with_checkpoint` | AI helper pool / managed boundary | unbounded token/CPU burn; background work that delays input/render |
| `sync_metadata_refresh` | sync pulls/pushes, metadata refresh, replication | `low` | `yield_with_checkpoint` | replication/upload workers | any work that consumes reserved interactive capacity |

#### Allowed work and escalation path (required)

Every worker class MUST have an explicit escalation path when it is asked to do
forbidden work.

- `ui_input` and `render_submission` escalation is always **handoff**: capture
  intent and enqueue work onto a non-hot-path class; never block waiting.
- `foreground_user_intent` escalation is **handoff or degrade**: return partial
  truth, stale labels, or "warming" state rather than blocking the UI.
- Yieldable background classes (`indexing_search_maintenance`,
  `extension_background_work`, `ai_background_jobs`, `sync_metadata_refresh`)
  escalate by **yielding, checkpointing, collapsing, deferring, or shedding**
  under governor pressure. They must never escalate by borrowing hot-path
  capacity.

#### Instrumentation labels (required)

Every trace span, log event, and support export record that represents work in a
worker class MUST carry:

- `worker_class` — one of the ids above
- `priority_policy` — one of the policy ids below
- `yieldability` — one of the yieldability ids below
- `admission_decision` — admitted/deferred/paused/shed/denied for the work item
- `concurrency_cap` — effective cap at admission time
- `task_kind` — dotted lower-snake identifier for the work kind (for example
  `index.full_scan`, `extension.poll`, `ai.embedding_refresh`, `sync.pull`)

When the work is also a background job, it MUST additionally carry the
background-queue labels: `queue_lane`, `work_class`, `budget_domain`,
`workload_lane`, and `priority_hint` as defined by the queue contract.

## Priority policies (frozen)

`priority_policy` is closed vocabulary:

- `highest_priority`
- `high_priority_bounded`
- `high`
- `medium_high`
- `medium_yieldable`
- `low_capped`
- `low_to_medium_budgeted`
- `low`

Priority policies may map to OS priority, cooperative executor weights, or a
later centralized scheduler. Regardless of implementation, the mapping must
preserve the ordering above under nominal conditions and must never allow a
lower policy to silently starve a higher one.

## Yieldability (frozen)

`yieldability` is closed vocabulary:

- `non_yieldable_hot_path` — never yield; only cancels stale duplicate work.
- `yield_at_safe_points_only` — may yield only at well-defined safe points
  (between events, between frame submits, between request phases).
- `yield_with_checkpoint` — must implement checkpoint/resume and yield under
  governor pressure; may be paused without correctness loss.

Any work item marked `yield_with_checkpoint` is non-conforming if it cannot
resume from a declared boundary after cancellation, pause, restart, or policy
change.

## Admission control and concurrency caps

The resource governor is the canonical owner of admission decisions. This
contract freezes *how* admission is explained:

- Work is admitted or denied by an explicit **decision**, never by silent
  starvation.
- Every class publishes its **effective concurrency cap** and current usage in
  diagnostics.
- Degraded/protect-core posture MUST narrow concurrency by class (background and
  optional classes first) before it narrows core interaction.

The specific numeric caps may be device/profile dependent, but they MUST be
explicit in the exported diagnostics and referenced by a stable id (for example,
`cap:extension_background_work:low_power_mode`), so support packets can explain
why a task did not run.

## Cancellation semantics

The cancellation contract vocabulary is aligned with the background-queue
contract:

- `cancel_immediate`
- `checkpoint_then_cancel`
- `cancel_after_phase`
- `no_inflight_cancel`

Hot-path classes typically use `no_inflight_cancel` for *in-flight* work and
instead cancel by dropping *stale* events before they begin. Yieldable classes
prefer `checkpoint_then_cancel` or `cancel_after_phase`.

## Starvation detection, backpressure, and shedding

The runtime MUST detect and report starvation using the diagnostic fields in
`schemas/runtime/worker_class.schema.json`:

- per worker class: runnable depth, running count, blocked count, oldest
  runnable age, yield count, cancellation count;
- per queue lane (when applicable): lane depth and oldest age (as already
  required by the queue contract).

### Starvation alarms

- A starvation alarm fires when a higher-priority worker class cannot make
  forward progress within its published budget window and lower-priority work is
  still running or runnable.
- Starvation alarms MUST include a cause classification that distinguishes:
  - CPU saturation (too many runnable tasks),
  - lock contention / priority inversion,
  - blocking I/O mistakenly on a protected class,
  - unbounded retry loops.

### Backpressure and burst shedding

When pressure rises, the governor must prefer this order:

1. collapse/replace superseded background work (queue contract),
2. reduce concurrency caps on low-priority worker classes,
3. defer admission to best-effort classes (`extension_background_work`,
   `ai_background_jobs`, `sync_metadata_refresh`),
4. pause yieldable maintenance (`indexing_search_maintenance`) to hot-set or
   visible-scope only,
5. only then narrow visible features, and only with explicit visible degraded
   labels.

Burst shedding is never silent: every shed/denied decision must mint a
diagnosable record that cites the active governor/efficiency state and the
effective cap/threshold that triggered it.

## Diagnostics and support export contract

Any support bundle, benchmark trace packet, or "service health" dashboard MUST
be able to answer:

- Which worker class did the work run on, and why?
- What was the governor state at the time?
- Was the work yieldable, and did it yield or checkpoint?
- Was admission deferred/denied because of a cap, a threshold, or forbidden work?
- Did a starvation alarm fire, and was priority inversion suspected?

The boundary records in `schemas/runtime/worker_class.schema.json` are the
minimum set needed to explain those answers.

