# Efficiency-state policy

This document freezes the shared efficiency-state policy Aureline uses
when battery, thermal, or power-saver pressure narrows background work
and render behavior. It exists so laptop-safe behavior is a product
contract, not a best-effort optimization pass, and so later latency
wins cannot hide fan, battery, or worker-budget regressions behind
off-screen work users cannot see.

The document is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document must be updated in the same change.

## Companion artifacts

- [`/artifacts/perf/worker_budget_rules.yaml`](../../artifacts/perf/worker_budget_rules.yaml)
  — machine-readable efficiency states, workload families, render-
  suppression rules, instrumentation points, and visible explanation
  contracts.
- [`/fixtures/perf/hidden_pane_cases/`](../../fixtures/perf/hidden_pane_cases/)
  — reviewable hidden-pane and off-screen scenarios that freeze what
  must be suppressed, what remains protected, and what users should
  see.
- [`/docs/runtime/resource_governor_contract.md`](../runtime/resource_governor_contract.md)
  — shared work classes, queue lanes, and visible health-state
  vocabulary this policy reuses rather than renaming.
- [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — admission-control thresholds, overload scenarios, and governor
  state transitions the efficiency policy specializes.
- [`/docs/benchmarks/spike_metric_names.md`](../benchmarks/spike_metric_names.md)
  — protected render-path counters this policy names for hidden-pane,
  visible-pane, off-screen, and frame-miss evidence.
- [`/docs/design/design_token_component_state_vocabulary.md`](../design/design_token_component_state_vocabulary.md)
  — motion and state-token vocabulary for power-saver and critical-
  hot-path posture changes.

## Scope and authority

This policy is a specialization of the runtime resource governor for
power, battery, and thermal pressure. It does not create a second
scheduler. The governor remains the canonical owner of:

- admission decisions;
- shed, defer, pause, cancel, and deny decisions;
- runtime health-state transitions; and
- the visible explanation contract projected to shell, support,
  benchmark, and future scheduler surfaces.

This policy freezes four additional questions that the general
resource-governor contract leaves at the power/performance layer:

1. Which work must throttle first when the device becomes battery- or
   thermal-constrained.
2. Which hot-path behaviors remain protected in every efficiency
   state.
3. Which hidden-pane and off-screen render behaviors are forbidden.
4. Which user-visible cues and truth labels must accompany each
   throttling decision.

## Shared vocabulary

The efficiency policy reuses the work classes from
[`resource_governor_contract.md`](../runtime/resource_governor_contract.md):

- `core_interaction`
- `core_navigation`
- `short_foreground_task`
- `background_knowledge_work`
- `optional_assistance`
- `upload_and_replication`

The user-facing capability states also remain the same:

- `ready`
- `warming`
- `partial`
- `degraded`
- `offline`
- `unsupported`
- `overloaded`

The efficiency-state vocabulary is:

- `Nominal`
- `EfficiencyAware`
- `ThermalConstrained`
- `ProtectCore`
- `Recovery`

`Recovery` is a governed runtime state even when the primary user cue
is a `warming`, `partial`, or `degraded` capability row rather than a
distinct state pill.

## Non-negotiable rules

1. Typing, save, undo, local navigation, terminal correctness, and
   current-task visibility remain protected in every efficiency state.
2. Optional assistance, speculative prefetch, background uploads, and
   non-essential motion shed before Aureline narrows the protected hot
   path.
3. Hidden panes and off-screen surfaces perform no committed paint,
   animation, or speculative refresh work. Correctness-preserving data
   ingestion is allowed only when it does not consume render budget and
   only when a visible surface depends on it.
4. Power-state changes are inspectable. The product may not say only
   "optimized" or silently slow features without naming the state,
   cause, affected capability, and recovery path.
5. Partial or stale results are explicit. Search, graph, preview, AI,
   extension, and upload surfaces may not imply full freshness when the
   governor has reduced scope or paused work.
6. Explicit user actions outrank speculative work, but explicit action
   does not grant unbounded background budgets. A user may request
   "Continue anyway" or an explicit refresh, but the runtime still
   protects the hot path and preserves truthful messaging.
7. Recovery is staged. Clearing the power or thermal signal does not
   authorize a deferred-work stampede that recreates the same battery
   or heat spike.

## Efficiency-state model

| State | Typical trigger | Must throttle first | Must preserve | Required user cue |
|---|---|---|---|---|
| **Nominal** | AC power or no sustained power / thermal pressure | no additional throttling beyond ordinary governor rules | typing, save, navigation, current-task visibility | no dedicated cue required |
| **EfficiencyAware** | on battery, OS battery saver, user low-power mode, or low-battery threshold | AI warmups, speculative prefetch, background uploads, non-essential animation | typing, save, local navigation, visible editor / terminal truth | status item or pill naming `EfficiencyAware` and the source; affected capabilities explain any narrowed scope |
| **ThermalConstrained** | OS thermal-pressure signal, sustained CPU saturation with hot-path miss, or fan / noise cap reached | non-essential animation, indexing parallelism, extension polling, rich preview refresh, graph enrichment beyond the hot set | editor latency, terminal correctness, current task visibility, explicit user commands | status item or pill naming `ThermalConstrained` and the source, plus per-capability throttled rows where freshness or completeness changed |
| **ProtectCore** | critical battery, repeated frame misses under pressure, or severe thermal throttling | all optional assistance, speculative background work, non-critical uploads, hidden preview refresh, non-visible graph/index work, and non-essential animation | current buffer integrity, save, undo, explicit user commands, visible task control | status item or pill naming `ProtectCore`; contextual banner when new work is denied or when visible results become partial or stale |
| **Recovery** | pressure clears after a constrained interval | deferred queues remain capped and resume in stages | protected hot path remains reserved until queues and frame metrics restabilize | no dedicated pill required; affected capabilities show `warming`, `partial`, or a recovery note until checkpoints catch up |

## Workload families and truthful narrowing

The efficiency policy names one workload-family set so future runtime
code, diagnostics, and benchmarks can map a throttled behavior to the
same work class and the same visible explanation contract.

| Workload family | Shared class / lane | First constrained action | Visible explanation contract | Truth contract when narrowed |
|---|---|---|---|---|
| **AI warmups** | `optional_assistance` / `maintenance` | defer or cancel speculative model warmups | efficiency-state pill + throttled-capability row | assistant may stay cold or unavailable; no cached result may be relabeled current because warmup was paused |
| **Prefetch** | `background_knowledge_work` / `maintenance` | stop non-visible prefetch and hot-cold cache widening | efficiency-state pill + throttled-capability row | hot-local or cached data remains usable; full-scope freshness becomes `warming` or `partial` rather than silently stale |
| **Non-essential animation** | `core_interaction` / `foreground` | collapse decorative motion, cursor embellishment, shimmer, and hidden blink loops to the current power posture | efficiency-state pill | focus visibility and state conveyance remain intact while decorative motion disappears |
| **Uploads and replication** | `upload_and_replication` / `upload_replication` | batch, defer, or require explicit foreground confirmation | durable deferred-job row or throttled-capability row | queued bytes, destination, and replay posture stay visible; a deferred upload is not "sent" |
| **Indexing refresh** | `background_knowledge_work` / `interactive_background` | reduce from whole-workspace to open-file and hot-set scope; cap parallelism | throttled-capability row + partial-scope chip | open files remain current; whole-workspace search, symbol, and graph answers become `partial` until the wider scope resumes |
| **Extension polling** | `background_knowledge_work` / `provider_overlay` | lower poll cadence, then pause optional timers and refresh loops | throttled-capability row | user-invoked extension commands remain attributable; background pause must appear as throttled, not broken |
| **Preview refresh** | `background_knowledge_work` / `interactive_background` for background refresh; explicit refresh remains foreground-bounded | freeze hidden or non-focused refresh work at the last truthful snapshot | stale-snapshot badge + throttled-capability row | snapshot age, source, and paused-refresh cause remain visible; hidden preview activity may not continue rendering |
| **Graph enrichment** | `background_knowledge_work` / `interactive_background` | pause non-hot-set enrichment and imported refresh | throttled-capability row + partial-scope chip | graph, search, AI, and impact surfaces label omitted scope or reduced freshness explicitly |

## State-specific budget rules

The machine-readable contract lives in
[`worker_budget_rules.yaml`](../../artifacts/perf/worker_budget_rules.yaml).
The normative narrative is:

### Nominal

- all workload families may run within their published budgets;
- speculative work remains collapse-aware and checkpointed;
- hidden panes still do no paint or animation work; Nominal is not an
  exemption from hidden-surface suppression.

### EfficiencyAware

- AI warmups and speculative prefetch move first to explicit-invoke or
  long-idle-only posture;
- low-priority uploads batch and wait behind visible work;
- non-editor motion collapses to the power-saver posture;
- indexing, graph, and preview work continue only where they preserve
  hot-local usefulness for visible or recently-active scope.

### ThermalConstrained

- indexing and graph work shrink to open-file and hot-set scope with
  lower parallelism;
- extension polling, hidden preview refresh, and non-visible imported
  data refresh pause before editor or terminal latency degrades;
- visible previews may refresh at a lower cadence, but hidden previews
  freeze at the last truthful snapshot;
- rich motion and embellishment are suppressed before any correctness
  or current-task surface is narrowed.

### ProtectCore

- all speculative background work pauses or denies admission;
- new AI warmups, non-critical uploads, marketplace refresh, preview
  warming, non-visible graph enrichment, and non-visible index scans
  do not start;
- visible non-core surfaces may remain readable from the last truthful
  local or cached snapshot, but they must surface `partial`,
  `degraded`, `stale`, or `overloaded` rather than appearing current;
- explicit user actions may still run if they are local-safe and
  bounded, but the runtime may deny heavy expansion with `overloaded`
  and a retry path.

### Recovery

- resumption is queue-age and checkpoint aware rather than FIFO flood;
- open-file and hot-set work resumes before whole-workspace widening;
- preview refresh, extension polling, and speculative prefetch resume
  only after hot-path metrics remain stable for the bounded window;
- hidden or still-occluded panes remain suppressed even during
  recovery.

## Hidden-pane and off-screen suppression

The render and polling policy is visibility-aware. The policy uses the
following visibility states:

- `visible_focused`
- `visible_background`
- `occluded_window`
- `hidden_tab`
- `collapsed_split`
- `detached_offscreen`

The claimed protected pane families are:

- editor viewports;
- terminal viewports;
- diff and review viewports;
- search and result-list viewports;
- task and log viewports;
- preview viewports; and
- graph panels.

The rules are:

1. `visible_focused` panes may consume paint budget for truthful
   visible work. Non-essential motion follows the current motion
   posture.
2. `visible_background` panes may paint only on actual content change.
   In `EfficiencyAware` and above, non-essential animation collapses.
3. `occluded_window`, `hidden_tab`, `collapsed_split`, and
   `detached_offscreen` panes perform no committed paint and no
   animation. They may record invalidation bookkeeping for diagnostics,
   but they may not spend render budget on frames the user cannot see.
4. Hidden preview, browser-runtime, and graph panes may not continue
   speculative refresh while hidden. They freeze at the last truthful
   snapshot and surface freshness or partiality when reopened.
5. Hidden terminal panes preserve PTY correctness and scrollback state
   without cursor-blink, shader, or scroll-embellishment work.
6. Hidden extension webviews and companion surfaces follow host policy,
   not extension-local policy. Optional timers, polling, and keepalive
   refresh pause when the host says the pane is hidden or off-screen.
7. In `EfficiencyAware`, `ThermalConstrained`, and `ProtectCore`,
   hidden-pane render waste on claimed protected panes is a policy
   violation. The target is zero `hidden_pane_work` on claimed stable
   surfaces in these states.
8. `offscreen_suppression_eligible` exists as an audit hook. A non-zero
   value is only acceptable while the harness is proving suppression
   logic; release-bearing evidence treats any committed off-screen work
   on claimed protected panes as non-conforming.

## User-visible cue requirements

### Runtime state cue

`EfficiencyAware`, `ThermalConstrained`, and `ProtectCore` must project
the shared efficiency-state pill from the UI / UX templates:

- state label;
- source, such as `OS battery saver`, `user low-power mode`, or
  `thermal pressure`;
- open-details action; and
- accessibility label.

Ordinary power-state changes use a status item or lightweight pill,
not a modal interruption.

### Capability cue

Whenever the governor changes freshness, completeness, cadence, or
admission for a named capability, the affected surface must project a
throttled-capability row or equivalent inline explanation that names:

- the affected capability;
- the current state or pause reason;
- the user impact in truth terms, such as `results may be partial` or
  `snapshot may be stale`;
- whether recovery is automatic or requires explicit retry; and
- any explicit actions, such as `Open details`, `Continue anyway`, or
  `Keep throttled`.

### Partial and stale result rules

- narrowed indexing or graph scope projects `partial`, not `ready`;
- frozen previews project freshness age or `stale snapshot`, not
  `ready`;
- denied background assistance projects `degraded` or `overloaded`,
  depending on whether explicit user action is still allowed;
- deferred uploads project a durable queued/deferred state, not
  completion;
- recovery catch-up projects `warming` or `partial` until checkpoints
  complete.

## Instrumentation and evidence

The policy requires the following instrumentation points. Future code
may change event shapes, but it may not rename the semantic fields
without updating this document and the companion artifact together.

### Runtime events

- `efficiency_state_transition`
  — previous state, new state, source signal, top throttled
  contributors, and any user or admin override.
- `workload_budget_decision`
  — workload family, work class, queue lane, current efficiency state,
  action (`admit`, `defer`, `pause`, `deny`, `staged_resume`),
  checkpoint state, and visible explanation contract.
- `render_visibility_decision`
  — surface class, visibility state, efficiency state, whether paint
  or animation was suppressed, and whether polling was kept for
  correctness only.

### Protected counters

The render-path evidence must align to the counter names frozen in
[`spike_metric_names.md`](../benchmarks/spike_metric_names.md):

- `visible_pane_work`
- `hidden_pane_work`
- `offscreen_suppression_eligible`
- `frame_misses`

These counters are the minimum evidence required to argue that a power
or thermal optimization preserved the hot path honestly rather than
moving waste off-screen.

### Support and benchmark evidence

Support bundles and future benchmark packets must capture:

- a bounded efficiency-state timeline;
- the top throttled workload families by time and queue age;
- deferred upload or replication summary;
- hidden-pane suppression violations;
- user or admin overrides; and
- the visible explanation contract each narrowed capability projected.

This is the evidence layer future `ff.power_thermal_posture` harness
work will consume.

## Change discipline

Any new background worker, speculative refresh path, preview surface,
or hidden-pane-capable shell surface is non-conforming until the same
change does all of the following:

1. maps the behavior to an existing workload family or adds a new row
   in [`worker_budget_rules.yaml`](../../artifacts/perf/worker_budget_rules.yaml);
2. reuses the shared visible explanation contracts;
3. declares how hidden or off-screen states suppress its work; and
4. adds or updates a fixture in
   [`/fixtures/perf/hidden_pane_cases/`](../../fixtures/perf/hidden_pane_cases/).
