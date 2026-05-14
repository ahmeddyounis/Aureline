# Efficiency-State Alpha Runtime Evidence

This document describes the first checked-in runtime consumer for power,
thermal, and hidden-pane efficiency truth on launch-wedge shell flows.

## Canonical Inputs

- Runtime policy: `docs/perf/efficiency_state_policy.md`
- Worker-budget rules: `artifacts/perf/worker_budget_rules.yaml`
- Power/thermal methodology: `docs/perf/power_thermal_methodology.md`
- Raw capture schema: `schemas/benchmarks/power_thermal_capture.schema.json`
- Protected fixtures: `fixtures/performance/efficiency_state_alpha/`

## Runtime Surface

The shell crate exposes `aureline_shell::efficiency` with typed hooks for:

- `efficiency_state_transition`
- `workload_budget_decision`
- `render_visibility_decision`

The status bar consumes `EfficiencyStatusSnapshot` through the
`status.slot.efficiency.state` slot. The slot paints only when the runtime
reports that power, thermal, battery, or policy pressure changed behavior.

## Activity Consumer

Indexing budget decisions can be converted into the existing durable
activity-center alpha row model. The row opens the authoritative activity
object, preserves support-export fields, and states that open files and hot-set
navigation remain truthful while wider indexing is reduced or paused.

## Hidden-Pane Suppression

`RenderVisibilityDecision` suppresses committed paint and animation ticks for:

- `occluded_window`
- `hidden_tab`
- `collapsed_split`
- `detached_offscreen`

`HiddenPaneRenderAudit` then audits committed samples. The protected fixture and
raw capture both expect:

- `hidden_pane_work = 0`
- hidden-surface committed paint count `= 0`
- hidden-pane render violations `= 0`

## Verification

```sh
cargo test -p aureline-shell --test efficiency_state_alpha
python3 tools/perf/power_thermal_audit/audit_capture.py fixtures/performance/efficiency_state_alpha/thermal_pressure_hidden_pane_capture.json
```

The fixture covers thermal pressure while indexing, AI warmups, uploads,
remote/session helpers, and hidden previews are present. It verifies that
typing, save, undo, local navigation, terminal correctness, and current task
visibility stay protected, and that efficiency adaptation is not used as an
explanation for skipped durability or missing user-owned artifacts.

