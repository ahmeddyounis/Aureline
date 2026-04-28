# Overview-surface fixtures

Worked fixtures for the overview and triage contract frozen in
[`/docs/ux/overview_surface_contract.md`](../../../docs/ux/overview_surface_contract.md)
and the schema at
[`/schemas/ux/dashboard_tile.schema.json`](../../../schemas/ux/dashboard_tile.schema.json).

The fixtures exercise stale green summary downgrade behavior, grouped
digests, mixed-ownership decision queues, exported triage views,
readiness banners, and inbox rows with unread and local draft state.

Each JSON file is a single overview record. The `__fixture__` prelude is
reviewer metadata; the canonical vocabulary lives in the record itself.

## Cases

| Fixture | Record kind | Scenario axis |
| --- | --- | --- |
| [`stale_green_summary_downgraded.json`](./stale_green_summary_downgraded.json) | `dashboard_tile_record` | A last-known passing tile downgrades to evidence stale once the freshness window expires. |
| [`grouped_digest_release_shift.json`](./grouped_digest_release_shift.json) | `digest_group_record` | A grouped digest keeps count, grouping reason, latest change, and evidence open path visible. |
| [`mixed_ownership_decision_queue.json`](./mixed_ownership_decision_queue.json) | `decision_queue_record` | A queue mixes provider, team, policy, unread, and draft states while exposing ranking reasons and typed counts. |
| [`exported_triage_view_preserves_queue_semantics.json`](./exported_triage_view_preserves_queue_semantics.json) | `exported_queue_view_record` | A captured queue export preserves visible/all-matching/blocked/hidden/unread/draft semantics, ranking, filters, and evidence refs. |
| [`readiness_banner_partial_indexes.json`](./readiness_banner_partial_indexes.json) | `readiness_banner_record` | A readiness banner refuses a ready claim while index evidence is partial. |
| [`inbox_row_unread_local_draft.json`](./inbox_row_unread_local_draft.json) | `inbox_row_record` | An inbox row keeps unread and local draft state distinct from provider-authoritative review truth. |
