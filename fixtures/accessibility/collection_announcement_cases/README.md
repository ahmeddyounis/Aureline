# Collection Announcement Cases

Seed fixtures for the accessible collection announcement contract.

These records validate the projection frozen in
[`/docs/accessibility/collection_announcement_contract.md`](../../../docs/accessibility/collection_announcement_contract.md)
and shaped by
[`/schemas/accessibility/collection_announcement.schema.json`](../../../schemas/accessibility/collection_announcement.schema.json).

| Fixture | Purpose |
|---|---|
| `virtualized_search_grid_all_matching.yaml` | A virtualized search grid announces logical position, selected count, hidden members, and all-matching batch scope while mounted rows recycle. |
| `filtered_review_tree_hidden_selected_read_only.yaml` | A filtered review tree announces tree level, read-only/blocked state, hidden selected members, and visible-scope action limits. |
| `stale_query_prior_snapshot_batch_scope.yaml` | A provider-backed batch scope announces that execution is pinned to a prior query snapshot after material dataset drift. |
| `keyboard_help_parity_list_grid_actions.yaml` | A package inventory grid exposes high-frequency selection and review commands through non-hover help surfaces and screen-reader actions. |

The YAML files intentionally carry opaque refs, bounded labels, counts,
state classes, message IDs, and schema refs only. Raw source text,
paths, query bodies, provider URLs, prompts, credentials, and user
identifiers are excluded.
