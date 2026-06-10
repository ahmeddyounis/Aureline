# Build the Regression Baseline Store, Baseline Selection UX, and Comparable-Environment Guards

**Artifact type:** Performance evidence qualification packet (M5)
**Packet id:** m5_049_regression_baseline_qualification:v1
**As of:** 2026-06-09

## Summary

- Baseline-store rows: 4
- Baseline-selection UX rows: 4
- Comparable-environment guard rows: 3
- Environment fingerprint rows: 3
- Stable surfaces: 4
- Below-stable surfaces: 2
- All below-stable surfaces have disclosure: yes
- Usable baselines: 4
- Narrowing guards: 3

## Claims

| Surface | Claim | Status |
|---|---|---|
| Baseline store | Stable | Certified |
| Baseline selection UX | Stable | Certified |
| Comparison report | Stable | Certified |
| Environment guard inspector | Stable | Certified |
| Export review | Preview | Under qualification |
| Support export | Preview | Under qualification |

## Evidence

- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/baseline_cpu_sampling.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/baseline_memory_sampling.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/baseline_picker.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/baseline_recent.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/comparison_report_cpu.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/environment_guard_strict.json`
- `fixtures/performance/m5/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards/environment_guard_freshness.json`

## Schema and Implementation

- Schema: `schemas/perf/build-the-regression-baseline-store-baseline-selection-ux-and-comparable-environment-guards.schema.json`
- Implementation: `crates/aureline-profiler/src/build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards/`

## Downgrade Rules

1. If a stable surface is missing a required guard, it is narrowed to preview.
2. If a baseline-store row does not show freshness state and build identity,
   the row is flagged as a validation violation.
3. If a baseline-selection UX row does not show a warning when the environment
   match state warns, the row is flagged as a validation violation.
4. If a comparable-environment guard row does not show criteria breakdown or
   narrow on mismatch, the row is flagged as a validation violation.
5. If a baseline store, selection UX, or guard references an unknown baseline,
   fingerprint, or guard, the row is flagged as a validation violation.
