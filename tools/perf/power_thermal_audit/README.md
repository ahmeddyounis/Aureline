# Power / thermal audit tools

Stdlib-only helpers for the raw
[`power_thermal_capture`](../../../schemas/benchmarks/power_thermal_capture.schema.json)
record family.

## Scripts

- `summarize.py <capture.json>`
  - Prints the capture posture, drain, average power, hot-path summary,
    and hidden-pane counters.
- `audit_capture.py <capture.json>`
  - Fails if required efficiency-state context is missing, if required
    workload-budget decisions are absent, or if hidden surfaces spend
    paint budget.
- `compare_runs.py <baseline.json> <candidate.json>`
  - Fails if the two runs differ in reference profile, posture, OS
    image, capture class, scenario, power source, battery mode, or
    other comparability-critical fields. Prints deltas when comparable.

## Example commands

```bash
python3 tools/perf/power_thermal_audit/summarize.py \
  fixtures/perf/power_thermal_capture_examples/arm64_steady_edit_efficiency_aware_run_a.json

python3 tools/perf/power_thermal_audit/audit_capture.py \
  fixtures/perf/power_thermal_capture_examples/arm64_steady_edit_efficiency_aware_run_a.json

python3 tools/perf/power_thermal_audit/compare_runs.py \
  fixtures/perf/power_thermal_capture_examples/arm64_steady_edit_efficiency_aware_run_a.json \
  fixtures/perf/power_thermal_capture_examples/arm64_steady_edit_efficiency_aware_run_b.json
```

The intentionally failing fixture
`x86_64_thermal_transition_hidden_pane_violation.json` is useful for
verifying the negative path:

```bash
python3 tools/perf/power_thermal_audit/audit_capture.py \
  fixtures/perf/power_thermal_capture_examples/x86_64_thermal_transition_hidden_pane_violation.json
```
