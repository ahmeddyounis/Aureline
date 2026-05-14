# Efficiency-State Alpha Benchmark Packet

This packet is the benchmark-facing evidence for the alpha desktop-efficiency
lane. It is a smoke/audit packet, not a published battery-life claim.

## Packet Header

| Field | Value |
|---|---|
| Packet id | `benchmark.efficiency_state.alpha` |
| Packet state | `fixture_drill_passed` |
| Captured at | `2026-05-14T08:55:00Z` |
| Runtime implementation | `crates/aureline-shell/src/efficiency/mod.rs` |
| Fixture directory | `fixtures/performance/efficiency_state_alpha/` |
| Raw capture | `fixtures/performance/efficiency_state_alpha/thermal_pressure_hidden_pane_capture.json` |
| Methodology | `docs/perf/power_thermal_methodology.md` |

## Drill Results

| Drill result id | Fixture | States exercised | Result |
|---|---|---|---|
| `benchmark.drill.efficiency.thermal_hidden_pane` | `thermal_pressure_hidden_pane_case.json` | `ThermalConstrained`, hidden tab, detached off-screen preview | `pass` |
| `benchmark.drill.efficiency.raw_capture_audit` | `thermal_pressure_hidden_pane_capture.json` | `EfficiencyAware -> ThermalConstrained -> Recovery` | `pass` |

## Acceptance Coverage

| Acceptance state | Evidence |
|---|---|
| Active efficiency state and pressure source are visible when behavior changes | status projection from `EfficiencyStateSnapshot` |
| Paused or reduced work is named | workload rows for indexing, AI warmup, upload transfer, remote/session helpers, and preview refresh |
| Hidden panes perform no render or animation work | `HiddenPaneRenderAudit.hidden_pane_render_violation_count = 0` |
| Indexing can surface in durable activity rows | `WorkloadBudgetDecision::indexing_activity_row` |
| Durability is not silently narrowed | `EfficiencyStateSnapshot::preserves_durability_truth()` |

## Verification

```sh
cargo test -p aureline-shell --test efficiency_state_alpha
python3 tools/perf/power_thermal_audit/audit_capture.py fixtures/performance/efficiency_state_alpha/thermal_pressure_hidden_pane_capture.json
```

