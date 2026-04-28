# List-row, result-row, and card-row fixtures

Seed corpus for the shared row contract frozen in
[`/docs/ux/list_and_card_row_contract.md`](../../../docs/ux/list_and_card_row_contract.md)
and the schema at
[`/schemas/ux/list_row.schema.json`](../../../schemas/ux/list_row.schema.json).

The corpus exercises:

- blocked support rows with safe inspect/export actions;
- stale review result rows with freshness-gated provider actions;
- provider-backed result rows that merge duplicate sources without
  losing provenance;
- local-only run rows that keep draft state separate from provider
  publication; and
- incident rows that promote to a richer card because terse rows
  cannot preserve counts, evidence, and recovery truth.

## Cases

| Fixture | Record kind | Scenario axis |
| --- | --- | --- |
| [`support_blocked_row.json`](./support_blocked_row.json) | `list_row_record` | Support triage row blocked by policy while read-only details and metadata export remain available. |
| [`review_stale_result_row.json`](./review_stale_result_row.json) | `result_row_record` | Hosted-review row whose provider state is stale relative to the local basis; mutation requires refresh. |
| [`provider_backed_merged_result_row.json`](./provider_backed_merged_result_row.json) | `result_row_record` | Provider inventory row merged from local mirror and provider result while preserving member provenance. |
| [`run_local_only_draft_row.json`](./run_local_only_draft_row.json) | `list_row_record` | Local-only run draft queued for publication without implying provider acceptance. |
| [`incident_promoted_card_row.json`](./incident_promoted_card_row.json) | `card_row_record` | Incident summary promoted to a card to keep failed state, evidence counts, freshness, and recovery actions visible. |
