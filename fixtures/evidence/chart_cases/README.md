# Evidence chart-card fixtures

Worked fixtures for
[`/docs/ux/evidence_chart_contract.md`](../../../docs/ux/evidence_chart_contract.md)
and
[`/schemas/evidence/chart_card.schema.json`](../../../schemas/evidence/chart_card.schema.json).

Each JSON file is one `evidence_chart_card_record`. Fixtures use opaque
ids and redaction-aware labels only; raw benchmark traces, logs,
screenshots, private repository names, absolute paths, unrestricted
provider URLs, and credentials stay outside this boundary.

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`fresh_pass_benchmark_chart.json`](./fresh_pass_benchmark_chart.json) | Fresh passing benchmark row with exact-build and public-proof packet refs. |
| [`stale_green_benchmark_chart.json`](./stale_green_benchmark_chart.json) | Last green benchmark chart downgraded after freshness expiry. |
| [`incomparable_baseline_chart.json`](./incomparable_baseline_chart.json) | Compatibility claim whose baseline cannot be compared after environment drift. |
| [`waived_threshold_chart.json`](./waived_threshold_chart.json) | Migration score card with an active threshold waiver and visible marker. |
| [`scope_changed_chart.json`](./scope_changed_chart.json) | Certified-archetype chart whose captured scope no longer matches the current claim scope. |
