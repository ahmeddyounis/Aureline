# Microcopy Grammar Fixtures

Worked YAML fixtures for
[`/docs/copy/count_scope_freshness_grammar.md`](../../../docs/copy/count_scope_freshness_grammar.md)
and
[`/schemas/copy/microcopy_term.schema.json`](../../../schemas/copy/microcopy_term.schema.json).

Each file is a `microcopy_case_record` showing the input truth, required
controlled terms, forbidden claims, approved compact copy, accessible
label, and export-safe label for one dense-surface scenario.

## Cases

- [`loaded_vs_all_matching_search.yaml`](./loaded_vs_all_matching_search.yaml)
  - Search summary where loaded results differ from all matching
    results while the index warms.
- [`hidden_by_policy_selection.yaml`](./hidden_by_policy_selection.yaml)
  - Selection bar and batch action where selected findings include
    hidden-by-policy and outside-workset members.
- [`approximate_queue_count.yaml`](./approximate_queue_count.yaml)
  - Queue row with an approximate provider backlog and exact loaded
    count.
- [`stale_health_check.yaml`](./stale_health_check.yaml)
  - Dashboard health card rendering a stale cached health snapshot.
- [`imported_historical_event.yaml`](./imported_historical_event.yaml)
  - Chronology row for an imported historical failed event with exact
    original and import timestamps.
