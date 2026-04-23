# Warm-path startup budget breakdown

This document is the **normative** companion to
[`/artifacts/perf/warm_path_budget_ledger.yaml`](../../artifacts/perf/warm_path_budget_ledger.yaml).

It turns warm-start performance language from "the shell paints in
150 ms" into a per-stage ledger. Performance review, benchmark
dashboards, and waiver packets resolve through the same stage ids so
a regression can be localised to the stage that moved instead of
vanishing inside a single aggregate number.

If this document disagrees with the PRD, Technical Architecture
Document, or Milestones document, those documents win and this file
updates in the same change.

Companion artifacts:

- [`/artifacts/perf/latency_budget_ledger.yaml`](../../artifacts/perf/latency_budget_ledger.yaml)
  — path-level budgets (`path.shell.launch`,
  `path.shell.first_useful_chrome`, `path.workspace.restore`, etc.)
  that the per-stage ledger cross-references.
- [`/artifacts/perf/protected_path_ledger.yaml`](../../artifacts/perf/protected_path_ledger.yaml)
  — protected-path identities and segment ids.
- [`/artifacts/perf/evidence_linkage_seed.yaml`](../../artifacts/perf/evidence_linkage_seed.yaml)
  — joins from paths to journey traces, benchmark rows, and packet
  families.
- [`/artifacts/bench/fitness_function_catalog.yaml`](../../artifacts/bench/fitness_function_catalog.yaml)
  — protected fitness functions (`ff.warm_start_to_first_paint`,
  `ff.first_paint`, `ff.command_parity`, `ff.restore_fidelity`) used
  by stage reconciliation rows.
- [`/schemas/traces/spike_timing.schema.json`](../../schemas/traces/spike_timing.schema.json)
  — shell-spike trace family that supplies most warm-start spans.
- [`/schemas/traces/journey_trace.schema.json`](../../schemas/traces/journey_trace.schema.json)
  — journey trace family that supplies restore and first-useful-
  chrome spans.
- [`/fixtures/perf/budget_reconciliation_examples/`](../../fixtures/perf/budget_reconciliation_examples/)
  — seed fixtures exercising the reconciliation rules.

## Why this ledger exists

The warm first paint target is published as a single number
(≤ 150 ms) and the warm interactive target as another single number
(≤ 700 ms). The single numbers are necessary for public claims, but
they are not sufficient for engineering review. Breadth work across
renderer, command system, project detection, snapshot restore, and
cache hydration can silently trade time between stages:

- renderer bootstrap slips by 30 ms;
- snapshot restore shaves 30 ms off by dropping fidelity;
- the aggregate stays green;
- the ledger should still say "renderer bootstrap regressed" and
  "snapshot restore under-delivered."

The warm-path ledger exists so that review surface, benchmark
dashboard, and waiver packets all quote the same stage ids. A run
within the aggregate budget cannot silently hide a per-stage
regression; a run past the aggregate ceiling must still name which
stage overran.

## Frozen stage set at this revision

Stage ids are stable. Adding a stage is additive-minor; renaming or
silently dropping a stage is breaking and requires a named change
record under the same performance-council path that governs the
latency-budget ledger.

| Stage id | Allocation | Critical-path posture |
|---|---:|---|
| `budget.warm_path.process_startup_and_config_load` | 80 ms | on warm first paint critical path |
| `budget.warm_path.renderer_bootstrap_and_font_atlas` | 120 ms | on warm first paint critical path |
| `budget.warm_path.workspace_snapshot_restore` | 120 ms | on warm interactive critical path after first paint |
| `budget.warm_path.file_tree_and_git_status_async_kick` | 0 ms | off critical path, async after first paint |
| `budget.warm_path.built_in_activation` | 40 ms | on warm interactive critical path after first paint |
| `budget.warm_path.project_detection_and_task_sniffing` | 80 ms | off critical path, async after first paint |
| `budget.warm_path.cache_hydration` | 80 ms | off critical path, async after first paint |
| `budget.warm_path.first_paint_aggregate` | ≤ 150 ms | aggregate ceiling |
| `budget.warm_path.interactive_aggregate` | ≤ 700 ms | aggregate ceiling |

The per-stage allocations are **not additive**. Post-first-paint
stages are allowed to run in parallel, which is why the sum of stage
allocations deliberately exceeds the aggregate ceilings. See
"Reconciliation rules" below for the exact check.

## Frozen vocabularies

The closed vocabularies live in the YAML under
`stage_status_vocabulary`, `stage_role_vocabulary`,
`critical_path_posture_vocabulary`, `allocation_kind_vocabulary`,
`reconciliation_kind_vocabulary`, and
`exception_reason_vocabulary`. Reviewers MUST resolve against these
ids; packets MAY NOT invent synonyms for stage labels or exception
reasons.

### Stage status

- `seeded` — stage id is stable and backed by at least one cited
  source document.
- `provisional` — stage id is reserved so later wiring cannot invent
  a second identity; it does not yet carry a measured verdict.

### Critical-path posture

- `on_warm_first_paint_critical_path` — stage duration lands
  directly in the warm first paint aggregate.
- `on_warm_interactive_critical_path_after_first_paint` — stage runs
  after first paint and before the warm interactive aggregate
  closes.
- `off_critical_path_async_after_first_paint` — stage must not block
  first paint; its allocation bounds the window in which it may
  contend with other post-paint work.
- `aggregate_budget_not_an_individual_stage` — row is an aggregate
  ceiling, not a component stage.

### Allocation kinds

- `fixed_ms_allocation` — numeric cap on the stage's measured
  duration on the claimed hardware class.
- `aggregate_ceiling_ms` — numeric cap on the aggregate stage's
  measured total.
- `must_be_zero_on_critical_path` — stage MUST NOT land any non-zero
  blocking time on the relevant critical path.
- `provisional_engineering_allocation` — allocation is reserved so
  packets do not invent a new cap; it is not yet ratified.

### Exception reasons

- `provisional_unmeasured` — the stage is provisional and has no
  measured verdict yet.
- `hardware_class_pending_calibration` — the allocation is published
  but the hardware row needed to interpret it is still being
  calibrated.
- `build_identity_pending` — the run is measured but the build
  identity it was captured against has not been promoted.
- `approved_waiver_active` — a performance-council waiver is in
  force for the stage.
- `degraded_posture_fallback` — the stage is running in a declared
  degraded posture and the fallback contract, not the numeric cap,
  applies.
- `span_family_not_yet_emitted` — the canonical trace span for the
  stage has not been emitted by the trace family yet.
- `stage_merged_into_another_span_under_review` — the stage is
  currently measured as part of a wider span pending a hook-list
  split.

## Reconciliation rules

The YAML defines six reconciliation rules; this section states what
they mean for review surfaces.

### 1. Trace-span-to-stage-total

Each stage names exactly one canonical span (or named sum of spans)
in `reconciliation.trace_span_refs`. The review surface MUST compare
`measured_duration_ms` against `allocation_ms`. A stage with no wired
span is reported `not_measured`; it never reads from an unrelated
span.

### 2. Benchmark-row-to-stage-total

Benchmark rows cited under a stage resolve through the
fitness-function catalog. They provide the distribution (p50, p95,
release bar) against which the stage's allocation is interpreted.
Rows marked `status: reserved_until_*` are placeholders — they do
not make a stage green.

### 3. Aggregate-sum-to-aggregate-ceiling

The aggregate rows (`first_paint_aggregate`,
`interactive_aggregate`) compare measured totals to their
`allocation_ms` ceilings. Aggregate totals are **not** the sum of
component `allocation_ms` values; they are independent caps. The
ledger's component-stage links exist for attribution, not arithmetic.

### 4. Single-stage regression when aggregate stays green

This is the rule that stops breadth work from hiding behind
aggregate totals. If any component stage's measured duration exceeds
its `allocation_ms`, the review surface MUST name the offending
stage id, even when the aggregate lands under its ceiling. The
overall verdict MAY NOT read `green`; it reads
`aggregate_green_with_stage_regression` and names the offending
stages. The seed fixture for this case is
[`single_stage_regression_aggregate_green.json`](../../fixtures/perf/budget_reconciliation_examples/single_stage_regression_aggregate_green.json).

### 5. Waiver or exception to stage budget

A stage whose measured duration exceeds `allocation_ms` MAY be
reported as non-blocking only if an approved waiver or exception
record is cited. The waiver must resolve through
`waiver_authority_forum` (`performance_council`) and carry one
reason from `exception_reason_vocabulary`. An unsourced breach is a
regression; a sourced breach still appears in the ledger with the
waiver id and an expiry date.

### 6. Provisional or unmeasured stage

A provisional stage — or a seeded stage whose trace span family has
not landed yet — is reported with `verdict: not_measured` and the
relevant exception reason. Downstream packets MAY NOT silently treat
a provisional stage as green; the seed reconciliation example under
`within_budget_reference_capture.json` shows the expected shape.

## How review surfaces consume the ledger

1. Join by stage id.
   Review tables, waiver packets, and dashboards resolve stages by
   id (`budget.warm_path.renderer_bootstrap_and_font_atlas`), not by
   free-text label.

2. Cite the stage verdict.
   Every packet that quotes warm-path timing MUST surface the stage
   verdicts alongside any aggregate verdict. A dashboard card may
   lead with `first_paint_aggregate`, but it MAY NOT omit the
   component-stage verdicts.

3. Quote the benchmark row and the trace span.
   A stage verdict without a `fitness_function_ref` and a
   `trace_span_ref` is not reviewable. Both sides MUST appear in the
   export so the reader can cross-check.

4. Name the waiver.
   Breach verdicts cite a `waiver_ref` and an `exception_reason`.
   The waiver row is the primary record; the ledger only carries the
   default expiry window and the escalation forums.

5. Never widen to an aggregate claim.
   If a packet wants to say "warm first paint is within budget," the
   aggregate stage MUST be `within_budget` **and** every component
   critical-path stage MUST be `within_budget`. A packet MAY NOT
   widen from an aggregate pass to a per-stage pass implicitly.

## Relationship to path-level budgets

The warm-path stage ledger does not replace the path-level budgets
in `artifacts/perf/latency_budget_ledger.yaml`. The two ledgers have
different shapes:

- path-level budgets (e.g. `budget.path.shell.launch`) own the
  user-visible protected path and its claim scope;
- per-stage budgets (e.g.
  `budget.warm_path.renderer_bootstrap_and_font_atlas`) own the
  internal stage that contributes to one or more paths.

Stage rows declare which path-level budgets they feed via
`latency_budget_ledger_links`. A path's first-paint latency rolls up
several stage contributions; the stage ledger makes those
contributions explicit so performance review can see which one
changed.

## Exclusions from this ledger

The following are explicitly **out of scope** at this revision:

- final release thresholds for subsystems that do not yet appear in
  the warm-path table (search, AI, rename preview, remote reconnect,
  save pipeline). Those rows are governed by their own path-level
  budgets in `latency_budget_ledger.yaml` and their own fitness
  rows in the catalog;
- cold-start stage breakdown. Cold-start targets are published in
  the PRD §8.2 table and remain out of scope until a cold-start
  ledger lands;
- third-party extension activation. Third-party extension budgets
  are governed by Appendix BQ of the Technical Architecture Document
  (`startup_activation_median ≤ 50 ms`, `startup_activation_p95 ≤
  150 ms`). The `budget.warm_path.built_in_activation` stage covers
  built-in extensions only.

## Change control

The warm-path budget ledger is change-controlled under the same
performance-council path that governs the latency-budget ledger and
the protected-path ledger. A stage addition, retirement, scope
split, or allocation change requires:

- a named change record in this document,
- a synchronized update to
  `artifacts/perf/warm_path_budget_ledger.yaml`,
- a synchronized update to the relevant
  `artifacts/perf/latency_budget_ledger.yaml` rows and
  `artifacts/perf/evidence_linkage_seed.yaml` joins, and
- a comparability note on any claim that cites a changed stage id.

Unsourced or accidental renames of a stage id are a validation
failure; packets MUST break loudly rather than silently rewire a
protected stage.
