# View-freshness contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/view_freshness_contract.md`](../../../docs/ux/view_freshness_contract.md)
and the schema at
[`/schemas/ux/view_freshness.schema.json`](../../../schemas/ux/view_freshness.schema.json).

The fixtures exist so search, docs, graph, log, review, notebook,
dashboard, and support surfaces can compare against the same badge
table and captured-versus-live export rules.

Each JSON file is a single `view_freshness_record`. The `__fixture__`
prelude is reviewer metadata; the canonical vocabulary lives in the
record itself.

## Cases

- [`search_results_live_exact.json`](./search_results_live_exact.json)
  - live exact search results with current scope, full completeness,
  intact snapshot/delta parity, and live-reference export.
- [`docs_pack_snapshot_exact.json`](./docs_pack_snapshot_exact.json)
  - exact docs-pack view captured from a fixed docs pack; export stays
  snapshot-only and does not imply current live docs.
- [`graph_partial_remote_shard_missing.json`](./graph_partial_remote_shard_missing.json)
  - partial graph view that names the missing remote shard and the
  provider-connectivity reason instead of hiding it under a stale label.
- [`log_stream_stale_cached_tail.json`](./log_stream_stale_cached_tail.json)
  - stale cached log tail with last-known-good time, retention-window
  invalidation, and requery action.
- [`dashboard_approximate_derived.json`](./dashboard_approximate_derived.json)
  - approximate dashboard aggregate from sampled telemetry with
  derivation inputs and approximation reason.
- [`review_pack_captured_scope_vs_live.json`](./review_pack_captured_scope_vs_live.json)
  - review pack whose captured provider scope differs from current live
  scope; export preserves both refs.
- [`notebook_variable_snapshot_exact.json`](./notebook_variable_snapshot_exact.json)
  - notebook variable explorer captured from a fixed kernel run; export
  stays snapshot-only and requery is explicit.
