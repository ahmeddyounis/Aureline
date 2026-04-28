# Placeholder-state honesty fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/empty_loading_placeholder_contract.md`](../../../docs/ux/empty_loading_placeholder_contract.md)
and the schema at
[`/schemas/ux/placeholder_state.schema.json`](../../../schemas/ux/placeholder_state.schema.json).

Each JSON file is a single `placeholder_state_record`. The fixture
metadata describes the scenario, but the canonical vocabulary lives in
the record fields. The corpus covers exact no-results, first-run,
indexing in progress, provider unavailable, and stale cached content.

## Cases

| Fixture | State class | Scenario axis |
|---|---|---|
| [`no_results_exact_search.json`](./no_results_exact_search.json) | `empty_no_results` | Search completed the current scope and found no matches, so the surface renders a neutral empty state rather than a loading placeholder. |
| [`first_run_start_center.json`](./first_run_start_center.json) | `empty_first_run` | First-run Start Center has no durable work yet and exposes local work-resume actions before account or service prompts. |
| [`indexing_in_progress_search.json`](./indexing_in_progress_search.json) | `indexing` | Hot-set search rows remain usable while semantic indexing continues and all partial cues stay visible. |
| [`provider_unavailable_docs_help.json`](./provider_unavailable_docs_help.json) | `missing_extension_or_provider` | Docs/help provider is unavailable, so cached and local docs remain visible with provider-unavailable copy. |
| [`stale_cached_content_shell.json`](./stale_cached_content_shell.json) | `stale_cached_content` | Shell restore keeps prior content visible as stale cached content and blocks ready/green treatment. |
