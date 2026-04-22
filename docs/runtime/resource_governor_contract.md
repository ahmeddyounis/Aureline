# Resource-governor contract

This document freezes the shared runtime contract Aureline uses
when local pressure or upstream impairment forces tradeoffs between
core interaction, visible foreground work, background knowledge
work, remote helpers, extensions, sync, preview, and optional
assistance. It exists so typing, save, undo, and current-task
visibility stay ahead of speculative or optional work without each
subsystem inventing its own notion of urgency, overload, or
degradation.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, or UI / UX Spec,
those source documents win and this document must be updated in the
same change.

## Companion artifacts

- [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — machine-readable threshold families, admission policies,
  shed order, visible-state projection rules, example policy
  fixtures, and overload scenarios.
- [`/docs/ux/attention_activity_taxonomy.md`](../ux/attention_activity_taxonomy.md)
  — durable job row, status item, contextual banner, quiet-hours,
  and reopen rules the governor's visible health surfaces use.
- [`/docs/benchmarks/spike_metric_names.md`](../benchmarks/spike_metric_names.md)
  — protected hot-path metric names the governor consumes for
  key-to-paint, frame-budget, and render-path pressure.
- [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — scope and authority vocabulary the governor quotes when it
  narrows or defers work by workspace, workset, or slice.
- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  — shared service-health truth model this contract reuses when
  remote or optional-service impairment projects as user-visible
  `degraded`, `offline`, or `unsupported`.

## Canonical owner

The shared runtime resource governor and its health publisher are
the canonical owners of:

- admission decisions;
- shed, defer, pause, cancel, and deny decisions;
- governor health-state transitions; and
- the projected runtime health state a shell surface, support
  export, or future scheduler explains.

Individual subsystems report budgets, queue state, retry state,
checkpoint state, and visible-work classification. They do not
reinterpret raw counters into their own overload model.

## Shared rules

1. One product-level governor decides pressure posture across shell,
   indexing, extensions, remote helpers, preview, sync, and
   optional assistance. Per-subsystem overload vocabularies are
   non-conforming.
2. Core interaction wins over every non-local subsystem state.
   Typing, save, undo, redo, and current-task visibility stay
   admitted unless a direct local impossibility exists, such as
   disk full or permission denial.
3. Shedding happens before correctness loss. Optional assistance,
   uploads, prefetch, and speculative maintenance are reduced
   before visible local-core workflows lose truth or control.
4. Duplicate or superseded background work collapses before queue
   depth grows. Unbounded queue growth is non-conforming.
5. Every degradable workload has a checkpoint or coalescing rule.
   A pauseable workload without a resume boundary is
   non-conforming.
6. Every degraded state is user-visible. Internal enums without a
   visible semantic, reason, and exit condition are
   non-conforming.
7. `overloaded`, `offline`, and `unsupported` are not synonyms.
   `overloaded` means the local governor is actively shedding or
   denying work to protect the hot path. `offline` means a remote
   or managed dependency is unavailable but a truthful local-safe
   path remains. `unsupported` means no truthful safe mapping
   exists on the current profile or target.
8. Warming is not indefinite waiting. A capability may not remain
   `warming` after progress stalls past the longest threshold
   window; it must move to `partial`, `degraded`, `offline`, or
   `unsupported` with a visible reason.
9. Partial truth must be named. Any surface that cannot see full
   relevant scope uses `partial`; it may not claim `ready`.
10. Explicit user cancellation, save, restore, and visible task
    control outrank speculative work and retry loops.

## Work classes

The governor uses one shared work-class set. Later schedulers may
subdivide internally, but they must project back to this set on
every boundary.

| Work class | Examples | Admission priority | Pressure posture |
|---|---|---|---|
| **Core interaction** | typing, selection, cursor movement, undo/redo, save, visible paint | highest / reserved | never queued behind background work; deny only on direct local impossibility |
| **Core navigation** | quick open, symbol jump, currently available diagnostics, outline, local diff | high | serve from hot local state first; refine asynchronously |
| **Short foreground task** | explicit user-run command, debug step, test rerun, Git status, search request | high | bounded queue; stale duplicates collapse |
| **Background knowledge work** | indexing, graph refresh, semantic refresh, cache rebuild, repo scan, prefetchable preview preparation | medium | throttle or pause on pressure; resume from checkpoint |
| **Optional assistance** | AI context expansion, embeddings, model warmup, marketplace refresh, remote prebuild warmup | low | first shed on pressure or outage; never reserve hot-path budget |
| **Upload and replication** | opt-in telemetry forward, crash upload, support-bundle upload, background sync publish | lowest | batch, defer, or disable before user workflows are affected |

Interactive extension or language actions triggered directly by the
user belong to `core_navigation` or `short foreground task`, not to
`optional assistance`. Extension timers, polling, and speculative
warmups belong to `background knowledge work` or `optional
assistance`.

## Queue lanes

The governor also reuses one queue-lane set. Admission is decided
by work class, but queue behavior is decided by lane.

| Lane | Collapse rule | Checkpoint expectation | Required diagnostics |
|---|---|---|---|
| **Foreground** | collapse only exact stale duplicates | none or explicit phase boundary | active request id, cancellation lag |
| **Interactive background** | coalesce by workspace, workset, slice, and phase | item or time boundary | oldest age, last checkpoint, collapse count |
| **Maintenance** | replace superseded jobs aggressively | explicit phase boundary | lane depth, pause reason, resume owner |
| **Provider overlay** | separate retry budget from local lanes | phase boundary before remote mutation | retry budget, circuit-breaker state |
| **Upload / replication** | batch by destination and retention class | resumable time or chunk boundary | queued bytes, pause reason, retention class |

## Threshold families

The exact seeded thresholds live in the companion YAML artifact.
The families are fixed here so later runtime code and QA fixtures
measure the same categories:

- **Hot-path pressure** — editor key-to-paint latency and frame
  miss rate. These are the fastest triggers because they directly
  measure the protected path.
- **CPU pressure** — hidden background logical-core consumption and
  runnable background-worker share. Background work may not consume
  a full logical core for 30 seconds without visible status and
  cancellation.
- **I/O pressure** — save-flush latency and background I/O queue
  age. Save is protected; background reads, cache population, and
  preview warmers yield first.
- **Memory pressure** — service-class soft and hard budget
  breaches. Disposable caches, inactive extension state, previews,
  and AI working sets trim before dirty buffers or user-owned
  recovery state.
- **Disk pressure** — low-disk floors and the derived-data cap. The
  baseline cap is the lesser of 20 GiB or 10 percent of free disk,
  with lower device-specific or admin-defined ceilings allowed.
- **Queue pressure** — lane depth and oldest age. Duplicate
  collapse and checkpoint-aware pausing occur before new queue
  admission widens.
- **Remote impairment** — remote RTT, error rate, and disconnect
  duration. Local editing and cached reads stay available wherever
  the contract allows.
- **Optional-service impairment** — latency, error rate, and
  circuit-breaker thresholds for AI, marketplace, docs/model
  refresh, and other optional managed-service clients.

## Governor health states

The runtime resource governor has one visible internal state
machine:

`Nominal -> Constrained -> Degraded -> ProtectCore -> Recovery`

These states govern admission and shed order. They are not the same
as the user-facing surface tokens in the next section.

| Governor state | Typical trigger family | Required admission posture | Required visible effect | Exit condition |
|---|---|---|---|---|
| **Nominal** | no sustained budget violation | normal scheduling and cache policy | none beyond ordinary progress and readiness | no action |
| **Constrained** | brief frame misses, moderate queue growth, rising resource pressure, early remote retry storm | collapse duplicates, trim speculative work, lower optional-service concurrency | status item when sustained; primary surface state changes only if scope or correctness narrows | protected metrics recover within the bounded window, or pressure escalates |
| **Degraded** | persistent latency miss, repeated retries, memory or disk pressure, prolonged queue growth | pause optional assistance, throttle maintenance, reduce work to hot-local or visible scope | affected surfaces move to `degraded`, `partial`, `offline`, or `overloaded` with reason and recovery path | pressure clears and replay or rebuild is healthy, or pressure escalates |
| **ProtectCore** | severe hot-path miss, thermal clamp, low disk, crash-loop pressure, or sustained background saturation affecting interaction | preserve typing, save, undo, navigation, and visible task control only; gate background and provider-overlay work | runtime status item plus contextual banner; new non-core work denies or defers with `overloaded` and a retry path | explicit recovery or automatic restoration of protected metrics |
| **Recovery** | pressure cleared after a degraded or protect-core interval | staged re-enable with bounded ramp-up; do not stampede deferred queues | recovering capabilities show `warming` or `partial` until checkpoints catch up | queues, latency, and pressure metrics return to nominal bands |

## Visible health-state vocabulary

Every surfaced capability projects exactly one primary visible
state. Surfaces may add secondary details, but they may not invent a
parallel primary-state vocabulary.

| Visible state | Meaning | Required visible copy | Typical entry rule | Typical exit rule |
|---|---|---|---|---|
| **Ready** | Capability is meeting its published scope and the governor is not actively narrowing it | ordinary readiness only | no missing scope, no active shed or impairment affecting this capability | transition to any other state on warmup, narrowing, fault, or pressure |
| **Warming** | Progress toward `ready` is expected and measurable; some utility is already safe to expose | current phase or checkpoint, what is already usable, next milestone | initial load, reconnect, rebuild, or recovery catch-up | `ready`, `partial`, `degraded`, `offline`, or `unsupported` |
| **Partial** | A truthful subset is available; the missing scope or omitted behavior is known | exact omitted scope or behavior, confidence/freshness cue, expansion or rebuild path | workset or index incomplete, hot-set-only answer, paused full-scope rebuild | `ready`, `warming`, `degraded`, `offline`, or `unsupported` |
| **Degraded** | Capability still works, but with reduced freshness, fidelity, performance, or authority | what still works, affected surfaces, last failure reason, repair or retry path | fallback path remains truthful after local or upstream impairment | `ready`, `warming`, `partial`, `offline`, `unsupported`, or `overloaded` |
| **Offline** | Required network or control-plane dependency is unavailable; a local-safe path remains | missing dependency plane, local-safe alternative, deferred-action queue where applicable | remote or managed dependency passes the offline threshold or circuit breaker | `warming`, `ready`, `degraded`, or `unsupported` |
| **Unsupported** | No truthful safe mapping exists on this target, profile, data class, or capability contract | exact unsupported class, why safe fallback is absent, handoff/export/repair path | platform gap, contract mismatch, unsupported transform, or denied claim surface | environment or target changes; no silent retry loop |
| **Overloaded** | The local governor is actively shedding, deferring, pausing, or denying work to protect core interaction | shed or denied work, protected path being preserved, whether recovery is automatic or manual | governor reaches `Degraded` or `ProtectCore` and this capability is narrowed by that action | `warming`, `partial`, `degraded`, or `ready` after pressure clears |

Specialized display copy may refine a state, but it may not rename
it. For example, a remote session may display **Read-only degraded**
while the projected primary state remains `degraded`.

## Projection precedence and transition rules

When multiple conditions apply, the governor projects one primary
visible state using the precedence below:

`unsupported > offline > overloaded > degraded > partial > warming > ready`

The rules are:

1. `unsupported` wins only when no truthful fallback exists. It may
   not hide a recoverable remote outage or local overload.
2. `offline` wins when a remote or managed dependency crosses the
   offline threshold and the contract still allows a local-safe or
   cached path.
3. `overloaded` wins when the local governor is actively pausing,
   denying, or deferring this capability to protect core
   interaction. High CPU alone is insufficient; an actual governor
   action must exist.
4. `degraded` wins when the capability still works but through a
   narrowed or fallback path that is not merely incomplete scope.
5. `partial` wins when the capability's main limitation is missing
   scope rather than fault or explicit shedding.
6. `warming` is valid only while progress remains active and the
   next checkpoint is reachable within the declared threshold
   windows.
7. A capability may move `warming -> overloaded` when recovery
   warmup itself is paused to protect the hot path.
8. A capability may move `offline -> warming -> ready` after
   reconnect. It may not jump silently from `offline` to `ready`
   when cache or authority refresh is still pending.
9. A capability may move `degraded -> partial` when a truthful
   hot-set-only answer exists but the broader degraded path is
   still rebuilding.
10. A capability that remains stalled after repeated retry budgets
    or a contract mismatch must leave `degraded` for `offline` or
    `unsupported`; it may not spin indefinitely.

## Admission control and shed order

The governor uses the same action order across all subsystems:

1. Batch, defer, or disable uploads and fleet signals.
2. Pause optional assistance and speculative provider overlays.
3. Collapse duplicate background knowledge work and reduce it to
   open-file and hot-set scope.
4. Throttle or pause extension timers, polling, hidden preview
   warmers, and inactive overlays.
5. Back off retrying remote, sync, and managed-service clients
   within bounded freshness rules.
6. Deny or defer new heavy foreground launches only after the steps
   above have failed to protect typing, save, undo, navigation, and
   current-task control.

The per-work-class admission posture is:

| Work class | Nominal | Constrained | Degraded | ProtectCore | Recovery |
|---|---|---|---|---|---|
| **Core interaction** | admit | admit | admit | admit | admit |
| **Core navigation** | admit | admit from hot local state first | admit from hot local state; refine asynchronously | admit bounded hot-local path; defer non-local refinement | admit and refill gradually |
| **Short foreground task** | admit | admit with bounded queue and duplicate collapse | admit current visible work first; defer non-visible or heavy widener tasks | allow current task control and narrow local-safe actions; deny new heavy or remote-expanding work with `overloaded` | readmit gradually by queue age and cost |
| **Background knowledge work** | admit with budgeted concurrency | collapse by workspace or phase; throttle | pause non-hot-set work; preserve checkpoints; mark affected surfaces `warming` or `partial` | pause | resume from checkpoint with bounded ramp-up |
| **Optional assistance** | admit with low reserved budget | throttle hard | cancel stale work and defer new work | deny new and pause active speculative work | re-enable only after hot-path metrics are stable |
| **Upload and replication** | batch | defer | pause except explicit foreground export | deny new background transfer; preserve queued state | staged resume with backoff |

## Example policy families

The companion YAML carries the machine-readable rows. The policy
families are frozen here so later implementations map into one
shape:

- **Editor shell** — typing, selection, cursor movement, undo/redo,
  save, and visible paint are `core interaction`. Background
  pressure does not change the editor's primary state from `ready`
  by itself; the shell instead surfaces runtime `overloaded`
  status while preserving the core path. The editor itself becomes
  `degraded` only on direct local impossibility or a truthful
  reduced-fidelity mode.
- **Indexing and graph refresh** — open-file freshness and hot-set
  symbols outrank whole-workspace completeness. Indexing starts as
  `warming`, projects `partial` when only the hot set or visible
  scope is current, and projects `overloaded` when the governor
  pauses non-hot-set work to protect typing and save.
- **Extensions** — user-triggered interactive extension work maps
  to `core navigation` or `short foreground task`; background
  timers, polling, and speculative warmup map to
  `background knowledge work` or `optional assistance`. Repeated
  budget breach or crash-loop behavior moves the extension to
  `degraded` or `unsupported` rather than leaving it flapping
  invisibly.
- **Remote attach** — attach auth and probing are short foreground
  work; sync warming is background knowledge work over the
  provider-overlay lane. High latency or transient loss projects
  `warming` or `degraded`; sustained disconnect projects
  `offline`. Local editing and cached reads remain available where
  the remote contract allows them.
- **Sync and replication** — idempotent publish or pull work uses
  the upload / replication lane, backs off aggressively on
  pressure, and never replays destructive work silently after
  overload or offline recovery.
- **Preview and auxiliary renderers** — visible preview refresh is
  foreground-bounded; hidden preview warmers are speculative and
  first shed. When preview cannot stay truthful, it projects
  `degraded` or `unsupported`; when it is merely waiting for a
  safe checkpoint it projects `warming`.
- **AI and optional assistance** — context expansion, embeddings,
  model warmup, marketplace refresh, docs/model refresh, and other
  auxiliary work are low-priority. They move quickly to
  `degraded`, `offline`, or `overloaded` rather than consuming
  reserved interaction budget.

## End-to-end overload scenario

The following scenario is the reference reasoning path the YAML
artifact also carries:

1. A large workspace opens on battery saver. Indexing, graph
   refresh, extension scan, AI embeddings, and a marketplace
   refresh all start within the same minute.
2. The user begins typing in an already-open editor. Key-to-paint
   and frame-miss thresholds enter `Constrained`.
3. The governor collapses duplicate index phases, pauses
   marketplace refresh, and batches uploads. The shell surfaces a
   status item; indexing remains `warming`.
4. Thermal pressure rises and the protected hot path crosses the
   degraded thresholds. The governor enters `Degraded`, throttles
   extension timers, pauses embeddings and docs/model refresh, and
   reduces indexing to open-file plus hot-set scope.
5. Search and graph surfaces now show `partial` because the hot set
   is current but whole-workspace freshness is not. AI assistance
   shows `degraded` or `offline` depending on the provider state.
6. Typing latency continues to worsen and the hidden-background CPU
   threshold stays breached for the full window. The governor
   enters `ProtectCore`.
7. New preview warmers, AI context expansion, marketplace refresh,
   and non-critical sync are denied or paused with `overloaded`.
   Typing, save, undo, navigation, and current-task visibility
   remain admitted.
8. Once key-to-paint, queue age, and thermal pressure recover, the
   governor enters `Recovery`. Indexing resumes from checkpoint,
   sync restarts at a low rate, and affected surfaces move from
   `overloaded` to `warming` or `partial`.
9. When hot-set and whole-workspace queues drain below the seeded
   thresholds, the governor returns to `Nominal` and the affected
   surfaces return to `ready`.

The important property is not the exact number of paused jobs. The
property is that the trigger, state transition, user-visible
semantic, and shed order can all be reasoned about without hidden
per-subsystem exceptions.
