# M5 filter-AST, saved-view, column-preset, selection-scope, result-counter, and batch-action-descriptor matrix

This document is the contract for the dense-collection freeze. It binds **every
claimed M5 dense collection surface** to a single bounded qualification matrix, so
Milestone 5 can ship this depth area with canonical implementation, proof,
downgrade behavior, and operator-facing truth instead of ad hoc prototypes, side
spreadsheets, or feature copy that outruns evidence.

M5 operational surfaces are increasingly dense collections: pipeline run lists,
review queues, incident lists, graph lists, marketplace results, activity rows,
provider/admin tables, and query-backed result sets. Those lanes only stay
trustworthy if their filter grammar, saved views, selection scope, result
counters, and batch-action scope are canonical product objects rather than
surface-local heuristics.

The matrix is canonical: no product, docs/help, diagnostics, accessibility, or
release-control surface may present a greener claim than this matrix, and any row
that cannot identify its filter-AST, selection-scope, result-counter, or
batch-action class auto-downgrades before it publishes.

Durable selection survives by stable identity, never a row highlight. Provider or
policy narrowing is always visible, never hidden inside a generic filter chip.
Visible, loaded, matching, and selected counts never blur. Visible rows are never
treated as all matching rows without an explicit step. Broad batch actions never
bypass preview because the list is virtualized or provider-backed.

## Source of truth

- Packet type: `CollectionQualificationMatrixPacket`
  (`crates/aureline-collections/src/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/`).
- Boundary schema:
  `schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json`.
- Checked support export:
  `artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/support_export.json`.
- Markdown summary:
  `artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix.md`.
- Protected fixtures:
  `fixtures/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/`.
- Conformance dump: `cargo run -p aureline-collections --example dump_m5_collection_qualification_matrix [support|summary]`.

The matrix reuses the frozen cross-surface collection vocabulary rather than
minting synonyms: the selection-scope class comes from
`stabilize_selection_scope_and_batch_result_truth::SelectionScopeClass`, and the
scope-counter vocabulary comes from
`stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm`.

## Claimed surfaces

Each claimed dense collection surface carries one matrix row:

`pipeline_run_list`, `review_queue`, `incident_list`, `graph_list`,
`marketplace_results`, `activity_rows`, `provider_admin_table`,
`query_backed_result_set`, and `support_export_projection`.

Every required surface MUST be represented; a missing surface blocks the matrix.

## Per-row dimensions

Each row identifies the four semantics a claimed surface must own:

- **filter-AST class** — `typed_clause_ast`, `saved_query_snapshot`,
  `provider_delegated_query`, or `free_text_scoped`.
- **selection-scope class** — `current_item_only`, `visible_range`, `loaded_set`,
  `all_matching_query`, or `explicit_custom_set`.
- **result-counter class** — `exact_count`, `approximate_count`,
  `provider_limited_count`, or `partial_streaming_count`.
- **batch-action scope class** — `local_reversible_batch`,
  `mixed_client_provider_batch`, `provider_authoritative_batch`,
  `destructive_gated_batch`, or `inspect_only_no_batch`.

A row also declares its scope-counter vocabulary terms (which must cover the full
canonical set when a result-counter class is identified), its saved-view contract,
its column-preset contract, and its batch-action descriptors.

### Batch-action descriptors

Each broad action a surface offers declares its kind (`export`, `copy`,
`suppress`, `install`, `update`, `delete`, `rerun`, `share`, `approve`) and its
preview / scope-receipt requirements. Any action that mutates state, is
provider-backed, is irreversible, or is an export / copy / share **must** preview
the included / excluded / blocked / skipped / hidden members and emit a reviewable
scope receipt before commit — regardless of how the list is rendered.

## Auto-downgrade

When every identity dimension (filter-AST, selection-scope, result-counter,
batch-action) is identified, the row's `effective_qualification` equals its
`claimed_qualification`. When any dimension cannot be identified, the row
auto-downgrades: its effective qualification ranks strictly below its claim
(`stable` > `beta` > `preview` > `experimental` > `held` > `unavailable`), it
records a downgrade trigger (`unidentified_filter_ast`,
`unidentified_selection_scope`, `unidentified_result_counter`,
`unidentified_batch_action`, `provider_narrowed`, `policy_narrowed`,
`partial_data_limited`, or `upstream_dependency_narrowed`), and it carries a
precise degraded label. A generic provider error never stands in for a precise
downgrade truth.

## Guardrails

The matrix refuses to publish unless:

- durable selection survives by stable identity, not a row highlight;
- provider or policy narrowing is always visible, never hidden in a generic chip;
- visible, loaded, and matching counts never blur;
- visible rows are never treated as all matching rows without an explicit step;
- broad batch actions never bypass preview because the list is virtualized or
  provider-backed; and
- any row lacking an identified filter / scope / count / batch class
  auto-downgrades below its claim.

## Consumer projection

Product, docs/help, diagnostics, accessibility, and release-control surfaces
ingest this matrix directly instead of cloning collection semantics by surface.
Downgraded rows are visibly labeled below current in every consumer surface.

## Boundary safety

Raw query text, raw filter literal bytes, provider cursors, credentials, and raw
row bodies never cross this boundary. The packet carries only typed class tokens,
booleans, opaque ids, and redaction-aware reviewable labels.
