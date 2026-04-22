# Power and thermal benchmark methodology

This document is the normative methodology Aureline uses for any
machine-readable claim about laptop power draw, battery drain, thermal
state transitions, hidden-pane render suppression, and background
worker-budget behavior. It exists so power and thermal claims remain
reproducible, reviewable, and separately governed from feature work
rather than living as anecdotal screenshots or one-off local notes.

If this document disagrees with the PRD, Technical Architecture
Document, Technical Design Document, UI / UX Spec, or Design System
Style Guide, those source documents win and this file must be updated in
the same change.

## Companion artifacts

- [`/artifacts/perf/reference_laptop_matrix.yaml`](../../artifacts/perf/reference_laptop_matrix.yaml)
  — machine-readable reference-profile roster, capture postures,
  ambient assumptions, and provisional budgets.
- [`/schemas/benchmarks/power_thermal_capture.schema.json`](../../schemas/benchmarks/power_thermal_capture.schema.json)
  — boundary schema for raw power / thermal capture records.
- [`/fixtures/perf/power_thermal_capture_examples/`](../../fixtures/perf/power_thermal_capture_examples/)
  — example raw captures consumed by the audit tools.
- [`/tools/perf/power_thermal_audit/`](../../tools/perf/power_thermal_audit/)
  — summary, audit, and run-comparison scripts for the raw capture
  family.
- [`/docs/perf/efficiency_state_policy.md`](./efficiency_state_policy.md)
  — efficiency-state, hidden-pane, and truthful-throttling policy this
  methodology measures.
- [`/artifacts/perf/worker_budget_rules.yaml`](../../artifacts/perf/worker_budget_rules.yaml)
  — machine-readable workload and visibility vocabulary this
  methodology reuses.
- [`/docs/runtime/resource_governor_contract.md`](../runtime/resource_governor_contract.md)
  — shared work classes, queue lanes, and visible health-state
  semantics that power / thermal captures must project against.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — reserved `ff.power_thermal_posture` row this methodology exists to
  wire cleanly once the benchmark harness lands.
- [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  — change-control and disclosure rules for any power / thermal claim
  that leaves the internal dashboard context.

## Scope and authority

This methodology defines:

1. The reference laptop and capture-posture metadata every run must
   carry.
2. The minimum raw capture envelope a power / thermal run must emit.
3. The measurement sequence for idle draw, active interactive draw,
   thermal-pressure transitions, hidden-pane render suppression, and
   worker-budget behavior.
4. The audit checks a reviewer or future CI lane must be able to run on
   committed raw captures.

This methodology does not:

- set final release-gating thresholds for every claimed laptop;
- replace the benchmark-lab run-result schema;
- widen the release-evidence posture of `ff.power_thermal_posture`
  beyond its current provisional state; or
- permit hardware, lab-image, or threshold recalibration inside the
  same feature change that benefits from it.

## Non-negotiable rules

1. Every capture MUST resolve to one `reference_profile_id` and one
   `reference_posture_id` from the reference-laptop matrix.
2. A run without explicit power source, battery mode, user power mode,
   ambient band, display brightness, refresh rate, and network posture
   is non-comparable.
3. Hidden-pane and off-screen work MUST remain auditable from raw
   capture bytes. A human-readable summary alone is insufficient.
4. Efficiency-state transitions MUST preserve their source and reason in
   the raw capture. A trace that says only "optimized" or
   "thermal-constrained" without context is non-conforming.
5. The methodology, reference laptop matrix, and audit scripts MUST
   land separately from feature work when they recalibrate hardware,
   ambient assumptions, or thresholds. A faster feature may not hide its
   own battery or fan regressions by changing the measuring stick.

## Reference profile contract

The reference matrix defines one profile family per claimed hardware
class and one or more capture postures under each profile. A comparable
run requires:

- matching `reference_profile_id` and `reference_profile_revision`;
- matching `reference_posture_id`;
- matching host architecture class and host platform class;
- matching power source and battery mode;
- ambient temperature within the declared tolerance band;
- matching brightness and refresh-rate settings;
- matching network posture; and
- a capture duration appropriate to the measured class.

If any of those drift, the run may still be useful for local inspection,
but it is not comparable to a reference capture.

## Raw capture contract

Every raw capture record uses
[`power_thermal_capture.schema.json`](../../schemas/benchmarks/power_thermal_capture.schema.json)
and MUST include at least:

| Field | Why it exists |
|---|---|
| `reference_profile_id`, `reference_profile_revision`, `reference_posture_id` | pins hardware class, OS image, ambient assumptions, and power posture |
| `run_context_class` | distinguishes reference capture from provisional or self capture |
| `capture_class` and `scenario_id` | prevents "idle draw" and "thermal transition" runs from being compared as if they were the same workload |
| `build_identity_ref` and `corpus_refs` | ties the run back to exact build and protected benchmark corpus inputs |
| `environment` | freezes power source, battery mode, display, network, and ambient details |
| `efficiency_state_timeline` | makes state changes inspectable instead of inferred from prose |
| `samples[]` | records battery, power, thermal, hidden-pane, off-screen, and worker-budget counters over time |
| `events[]` | records state transitions and workload-budget decisions with source and reason |

The raw capture is intentionally lower level than the benchmark-lab
run-result record. It is the auditable input the future
`ff.power_thermal_posture` row can cite; it is not a replacement for the
rolled-up dashboard report.

## Measurement classes

The closed capture-class set is:

- `idle_draw`
- `active_interactive_draw`
- `thermal_pressure_transition`
- `hidden_pane_suppression`
- `worker_budget_behavior`

### Idle draw

Purpose: show what Aureline costs to keep open when no user-visible work
is progressing.

Method:

1. Boot the reference image and wait for background OS update and
   indexing noise to quiesce.
2. Launch Aureline with the declared corpus or empty-shell scenario.
3. Warm for 120 seconds so cold-start transients do not dominate the
   sample.
4. Record a 300-second capture window at the declared sample cadence.
5. Confirm there is no hidden-pane paint loop, no speculative refresh
   loop, and no background worker that consumes a full logical core
   without a declared reason.

Required evidence:

- average system power or battery-energy delta over the window;
- CPU or worker-budget quiescence summary;
- hidden-pane and off-screen counters at zero on claimed stable
  surfaces; and
- explicit efficiency-state context, even when the state remains
  `Nominal`.

### Active interactive draw

Purpose: measure battery cost during a steady, realistic foreground
workflow rather than only during idle or synthetic CPU saturation.

Method:

1. Use a declared workflow corpus such as
   `corpus.workflow.first_useful_edit_rust_self_host`.
2. Start from the posture declared in the reference matrix, usually a
   battery-backed balanced mode with fixed brightness and Wi-Fi state.
3. Warm for 180 seconds to settle caches and initial scans.
4. Run a 900-second scripted session containing edit, navigation,
   search, and save actions on the protected workflow.
5. Normalize battery delta to drain-per-hour for comparison across runs
   with the same posture.

Required evidence:

- initial and final battery percent and battery energy;
- average system power over the run;
- hot-path latency summary;
- worker-budget snapshot by shared work class; and
- hidden-surface counters proving energy was not "saved" by pushing work
  off-screen.

### Thermal-pressure transition

Purpose: verify that state transitions are correct and that shedding
order follows the efficiency-state policy when thermal pressure rises.

Method:

1. Begin in a declared battery or AC posture with the expected initial
   efficiency state.
2. Induce sustained load with the declared corpus and auxiliary load
   recipe until the OS or runtime reports thermal pressure.
3. Record the full state sequence, usually
   `EfficiencyAware -> ThermalConstrained -> Recovery`, with exact
   timestamps, source, and reason.
4. Audit that the declared workloads throttle within the allowed settle
   window after each constrained-state entry.
5. Hold the run through recovery long enough to show staged resume
   rather than a queue stampede.

Required evidence:

- explicit transition events with source and reason;
- workload-budget decisions for the required workload families;
- hot-path latency still preserved through the constrained interval; and
- a bounded recovery window with gradual resumption.

### Hidden-pane suppression

Purpose: prove that hidden or off-screen surfaces do not keep spending
 paint, animation, or refresh budget.

Method:

1. Open at least one claimed stable surface in a hidden-tab,
   collapsed-split, or detached-offscreen posture.
2. Keep a visible current-task surface active so the workload remains
   realistic.
3. Record paint, hidden-pane, and off-screen counters while the hidden
   surface would otherwise be tempted to refresh.
4. Re-open the surface and confirm it reports truthful freshness or
   partiality rather than silently claiming continuous currency.

Required evidence:

- `hidden_pane_work = 0` on claimed stable surfaces;
- `committed_paint_count = 0` for hidden or detached-offscreen samples;
- any `offscreen_suppression_eligible` activity surfaced as an audit
  finding rather than ignored; and
- visible user-facing state when freshness narrowed.

### Worker-budget behavior

Purpose: show that optional and speculative work sheds before core
interaction degrades.

Method:

1. Run the workload under a constrained posture or while inducing a
   constrained posture.
2. Capture queued and running worker counts by shared work class.
3. Emit workload-budget decision events for AI warmups, speculative
   prefetch, uploads, indexing refresh, extension polling, preview
   refresh, and graph enrichment as applicable.
4. Confirm that optional assistance and speculative work reduce before
   the hot path exceeds its protected budget.

Required evidence:

- worker counts by shared work class in `samples[]`;
- `workload_budget_decision` events naming the work class and queue
  lane; and
- a visible explanation path that could later surface the same truth to
  users.

## Sample cadence and timing rules

- `samples[]` SHOULD be recorded every 5 seconds or faster.
- `events[]` MUST preserve second-level ordering even when multiple
  events share the same second.
- Idle windows MUST record at least 300 seconds after warmup.
- Active interactive windows MUST record at least 900 seconds after
  warmup.
- Thermal-transition captures MUST include at least one constrained
  state and one recovery interval.
- Hidden-pane captures MUST hold the hidden posture for at least 180
  seconds or long enough to demonstrate the suppressed workload would
  otherwise have refreshed.

## Provisional budgets and operating targets

The following targets are seeded now so later releases do not invent a
new metric identity when the harness lands. Targets marked
`subject_to_calibration` are draft operating targets, not final release
gates.

| Budget or target | Value | Status |
|---|---|---|
| Idle shell CPU on reference hardware | `<= 1%` over 5 minutes | seeded from PRD |
| Open fully indexed workspace CPU with no active typing | `<= 2%` over 5 minutes | seeded from PRD |
| Active interactive battery drain on ARM64 reference laptop | `<= 10%` per hour | seeded from PRD; compare only on the same posture |
| Active interactive battery drain on x86_64 reference laptop | `<= 12%` per hour | seeded from PRD; compare only on the same posture |
| `hidden_pane_work` on claimed stable surfaces | `0` | hard policy floor |
| Hidden or detached-offscreen `committed_paint_count` | `0` | hard policy floor |
| `offscreen_suppression_eligible` | `audit_only_then_converge_zero` | subject to calibration |
| State-entry settle window for workload-budget decisions | `30s` | subject to calibration |
| Recovery quiet window before speculative resume | `60s` | subject to calibration |
| Thermal hot-path p95 under `ThermalConstrained` | `subject_to_calibration` | reserved for future gating |
| Protect-core hot-path p95 | `subject_to_calibration` | reserved for future gating |

The machine-readable version of these assumptions lives in
[`reference_laptop_matrix.yaml`](../../artifacts/perf/reference_laptop_matrix.yaml).

## Audit expectations

The audit tools are intentionally simple and deterministic. They do not
re-derive policy from prose. They read the raw capture and answer three
questions:

1. Is the run comparable to another run on the same reference posture?
2. Does the capture preserve enough context to explain efficiency-state
   and worker-budget behavior?
3. Did hidden-pane or off-screen work violate the declared suppression
   policy?

The minimum script set is:

- `summarize.py` — prints one capture summary with power, drain,
  thermal, and worker-budget highlights.
- `audit_capture.py` — fails when required context is missing, when
  transitions lack source or reason, when required workload-budget
  decisions do not appear, or when hidden surfaces spend paint budget.
- `compare_runs.py` — checks comparability for two captures on the same
  reference posture and prints deltas for drain, power, and hot-path
  behavior.

## Change control

Any change to:

- a reference profile id or posture id;
- the OS image or patch level for a reference profile;
- the ambient, display, or network assumptions of a reference posture;
- a provisional budget value; or
- the capture schema itself

must land as an explicit methodology or benchmark-governance change, not
as an incidental detail inside a feature PR. Hardware or image
recalibration resets comparability and must be called out as such in the
same change.

## Current integration boundary

This repository now freezes the methodology, capture schema, reference
profiles, example raw captures, and audit tools. The benchmark-lab row
`ff.power_thermal_posture` remains provisional until a future change
wires these raw captures into the run-result and dashboard pipeline.
