# Warm-path budget reconciliation examples

Seed fixtures that exercise the reconciliation rules in
`artifacts/perf/warm_path_budget_ledger.yaml`. These files are
reviewable examples, not a benchmark corpus: they freeze the
**shape** of a reconciliation verdict so performance review,
benchmark dashboards, and waiver packets can be tested against one
stable envelope.

Normative sources:

- `artifacts/perf/warm_path_budget_ledger.yaml`
- `docs/perf/startup_budget_breakdown.md`

## Fixture rules

- Every fixture resolves stage ids through the warm-path budget
  ledger; free-text stage labels are not permitted.
- Every fixture names the reconciliation rule ids it exercises.
- Every fixture carries `measured_aggregate_ms` for
  `first_paint_aggregate` and `interactive_aggregate` and a
  `per_stage_measurements[]` list for component stages.
- Fixtures do not set release thresholds; they exercise the
  verdict taxonomy (`within_budget`, `breach`,
  `aggregate_green_with_stage_regression`, `not_measured`).
- Unmeasured or provisional stages carry `verdict: not_measured`
  with one `exception_reason` from the ledger's vocabulary.

## Index

| Fixture | Rule ids exercised | Expected truth |
|---|---|---|
| `within_budget_reference_capture.json` | `rule.reconciliation.trace_span_to_stage_total`, `rule.reconciliation.aggregate_sum_to_aggregate_ceiling`, `rule.reconciliation.provisional_unmeasured_stage` | All wired stages land inside their allocations; both aggregates land inside their ceilings; unmeasured stages surface as `not_measured`, not green. |
| `aggregate_miss_first_paint_over_ceiling.json` | `rule.reconciliation.aggregate_sum_to_aggregate_ceiling`, `rule.reconciliation.trace_span_to_stage_total`, `rule.reconciliation.waiver_or_exception_to_stage_budget` | Warm first paint aggregate exceeds 150 ms; the two critical-path stages are individually flagged with breach values and the packet names the stage ids rather than rolling the miss into a single aggregate number. |
| `single_stage_regression_aggregate_green.json` | `rule.reconciliation.single_stage_regression_vs_aggregate_green`, `rule.reconciliation.trace_span_to_stage_total` | Warm interactive aggregate stays under 700 ms but renderer bootstrap exceeds its 120 ms allocation. Verdict is `aggregate_green_with_stage_regression` and the offending stage id is named. |
