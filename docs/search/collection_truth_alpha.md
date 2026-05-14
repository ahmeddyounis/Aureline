# Collection Truth Alpha

The shared collection runtime contract lives in `aureline-search::collections`
and is consumed by search, review, and package/inventory lanes. It implements
the product-wide contract in
[`docs/ux/collection_view_contract.md`](../ux/collection_view_contract.md) and
keeps compatibility with the canonical collection schemas under
`schemas/collections/`.

## Contract

- `CollectionFilterAst` stores filters as typed expressions with field ids,
  operators, source classes, lock state, redaction posture, and round-trip
  status. Policy, workset, provider, client, and partial-data narrowing is
  represented as visible chips instead of hidden backend state.
- `SavedCollectionView` versions saved views with owner scope, privacy class,
  fallback behavior, visible columns, pinned columns, sort keys, and stale or
  degraded labels. Portable saved views reject transient row selection,
  provider cursors, and local-only literals.
- `CollectionScopeCounters` distinguishes visible, loaded, all-matching,
  selected, blocked, hidden, hidden-by-policy, and hidden-by-filter counts with
  exact, approximate, provider-limited, client-limited, stale, cached, partial,
  or unknown status.
- `CollectionSelectionState` stores stable item identities and announces
  selected count, scope class, hidden-selected count, blocked count, and stale
  selection basis for keyboard and assistive-technology paths.
- `BatchReviewSheet` separates included, excluded, blocked, hidden, and stale
  members before consequential batch actions continue, then preserves mixed
  aftermath summaries.

## First Consumers

- Search result collections project existing `SearchQuerySession` and
  `SearchScopeCountsRecord` state through
  `CollectionViewAlphaRecord::from_search_results`.
- Review workspaces project diff anchors through
  `aureline_review::collections::ReviewCollectionAlphaPacket`.
- Package and extension install-review lanes project package rows through
  `aureline_extensions::collections::ExtensionInstallCollectionAlphaPacket`.

## Schemas And Fixtures

- Canonical boundary schemas:
  `schemas/collections/filter_ast.schema.json`,
  `schemas/collections/saved_view.schema.json`,
  `schemas/collections/selection_state.schema.json`, and
  `schemas/collections/batch_review_packet.schema.json`.
- Narrow alpha implementation schemas:
  `schemas/collections/filter_ast_alpha.schema.json`,
  `schemas/collections/saved_view_alpha.schema.json`, and
  `schemas/collections/batch_review_alpha.schema.json`.
- `fixtures/search/collection_truth_alpha/query_backed_scope_truth.json`
- `fixtures/review/batch_review_alpha/local_review_batch_sheet.yaml`

## Verification

Run the focused checks:

```sh
cargo test -p aureline-search --test collection_truth_alpha
cargo test -p aureline-review --test batch_review_alpha
cargo test -p aureline-extensions marketplace_package_lane_reuses_collection_batch_review_truth
```
