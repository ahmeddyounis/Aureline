# Search Result Row Cases

Worked `search_result_row_record` fixtures for
[`/docs/ux/search_result_contract.md`](../../../docs/ux/search_result_contract.md)
and
[`/schemas/search/search_result_row.schema.json`](../../../schemas/search/search_result_row.schema.json).

| Fixture | Coverage |
|---|---|
| `stale_index_text_result.json` | stale index label, stale all-matching count, replace preview requirement |
| `partial_workspace_symbol_result.json` | partial workspace coverage, hidden scope, unknown matching count before rename |
| `generated_source_result.json` | generated path label, lineage refs, canonical-source and regenerate actions |
| `hidden_ignored_file_result.json` | hidden / ignored paths, not-searched disclosure, excluded write subset |
| `semantic_approximate_result.json` | semantic supplement, approximate confidence, no authoritative write set |
| `provider_backed_result.json` | provider-backed saved query, deep-link continuity, provider-limited batch review |

Every fixture must preserve query/session/scope/freshness refs and show
visible, selected, all-matching, excluded, blocked, and unknown counts
before any write path can widen.
