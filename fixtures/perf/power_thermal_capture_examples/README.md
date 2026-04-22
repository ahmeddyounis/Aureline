# Power / thermal capture examples

Example raw captures for the
[`power_thermal_capture`](../../../schemas/benchmarks/power_thermal_capture.schema.json)
record family. These fixtures are not benchmark thresholds. They exist
so reviewers and the audit scripts can exercise:

- reference-profile and reference-posture comparability;
- drain and average-power summarization;
- efficiency-state transition context;
- workload-budget decision auditing; and
- hidden-pane / off-screen violation detection.

Canonical sources:

- `docs/perf/power_thermal_methodology.md`
- `artifacts/perf/reference_laptop_matrix.yaml`
- `docs/perf/efficiency_state_policy.md`
- `artifacts/perf/worker_budget_rules.yaml`

## Index

| Fixture | Expected outcome |
|---|---|
| `arm64_steady_edit_efficiency_aware_run_a.json` | Passes audit; comparable with `run_b`; ARM64 battery editing posture stays inside the seeded directional target |
| `arm64_steady_edit_efficiency_aware_run_b.json` | Passes audit; comparable with `run_a`; slight drain delta for compare-script output |
| `x86_64_thermal_transition_hidden_pane_violation.json` | Fails audit; hidden preview paints off-screen, a thermal transition is missing source context, and one required workload decision is absent |
