# Search result row, guarantee, and replace-scope contract

Search result rows are a user-facing projection over the search truth,
planner, saved-query, generated-lineage, collection, and replace-preview
records. They are not a private backend view. Every row must explain what
is guaranteed, what is approximate, and what was not searched before the
user opens, exports, cites, replaces, or batches over it.

Machine-readable companions:

- [`/schemas/search/search_result_row.schema.json`](../../schemas/search/search_result_row.schema.json)
  defines the renderer-facing `search_result_row_record`.
- [`/fixtures/search/result_cases/`](../../fixtures/search/result_cases/)
  contains worked rows for stale index, partial workspace, generated path,
  hidden / ignored file, semantic approximation, and provider-backed cases.

This contract composes with:

- [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  for readiness, result truth, ranking reasons, hidden-scope reasons,
  partial-truth causes, semantic fallback, freshness, and deep-link drift.
- [`/schemas/search/result_fusion_record.schema.json`](../../schemas/search/result_fusion_record.schema.json)
  for planner pass, result set, shard contribution, duplicate collapse,
  and ranking explanation refs.
- [`/schemas/search/saved_query_and_scope_binding.schema.json`](../../schemas/search/saved_query_and_scope_binding.schema.json)
  and
  [`/schemas/search/saved_query_bundle.schema.json`](../../schemas/search/saved_query_bundle.schema.json)
  for query-history, saved-query, scope-binding, search deep-link, and
  collection-snapshot continuity.
- [`/schemas/generated/artifact_edit_posture.schema.json`](../../schemas/generated/artifact_edit_posture.schema.json)
  for generated or mirrored artifact posture.
- [`/schemas/collections/batch_review_packet.schema.json`](../../schemas/collections/batch_review_packet.schema.json)
  and
  [`/schemas/editor/refactor_preview.schema.json`](../../schemas/editor/refactor_preview.schema.json)
  for batch-write and replace-preview gates.

If those source contracts disagree with this document, those contracts win
and this document updates in the same change.

## Row Anatomy

Every rendered row carries these groups:

| Group | Required contents |
|---|---|
| Row identity | row id, family, visibility, schema version, minted time |
| Display projection | primary label, context label, source label, result type |
| Canonical identity | canonical entity ref plus file, text range, symbol, provider, generated lineage, or canonical source refs as applicable |
| Source refs | search packet, planner pass, result set, result fusion, query session, scope binding, saved query, deep-link binding, lineage, and review refs |
| Snippet basis | whether the snippet came from current buffer text, an index snapshot, symbol signature, semantic summary, provider summary, generated lineage summary, or no snippet |
| Freshness and readiness | readiness state, freshness class, truth class, semantic fallback, confidence label, scope class, stale / partial explanation |
| Truth labels | frozen row chips for stale index, partial index, approximate, hidden scope, generated path, ignored path, provider-backed, blocked, not searched, saved query, and deep-linked state |
| Quick actions | typed row actions with disabled reasons, never free-form buttons |
| Replace scope | visible, selected, all-matching, excluded, blocked, and unknown counts before any write path widens |
| Route persistence | query, filters, scope, result set, deep link, saved query, and freshness refs preserved for reopen/export |

The six row families are:

- `file_result` — a workspace file, buffer anchor, or path-backed object.
- `text_match_result` — a text occurrence or range produced by lexical or
  indexed search.
- `symbol_result` — a declaration, definition, implementation, reference,
  route, or graph symbol target.
- `semantic_result` — a semantic or hybrid suggestion that is useful but not
  automatically authoritative.
- `provider_result` — a provider-backed resource or provider-overlay row.
- `generated_source_result` — a generated, mirrored, imported, or derived
  source row whose default edit posture comes from generated-artifact
  posture and lineage records.

## Frozen Row Labels

Rows must use the frozen label tokens in
`search_result_row.schema.json#/$defs/search_row_label_token`.

| Token | User meaning |
|---|---|
| `exact` | The row is guaranteed for the stated scope. |
| `approximate` | The row is useful but ranking or matching is approximate. |
| `partial_index` | Search did not cover the whole requested scope yet. |
| `stale_index` | Search is using an older snapshot. Refresh before trusting broad actions. |
| `hidden_scope` | Matching rows exist or may exist in a scope this surface cannot show. |
| `generated_path` | The row points at generated or derived source. Edit posture must stay visible. |
| `ignored_path` | The row is under ignore or exclusion rules. It was not searched as ordinary source. |
| `provider_backed` | The row depends on a connected provider or provider-hosted index. |
| `blocked` | The row or action is blocked before commit. |
| `not_searched` | The engine did not search that subset. This is different from zero matches. |
| `saved_query` | The row is restored from a saved query or captured collection. |
| `deep_linked` | The row route is preserved by a search deep-link binding. |

Surfaces may render icons, abbreviated chips, or grouped details, but they
must not replace these states with backend terms such as shard, cache
owner, or planner stage in primary row chrome. Those refs belong in the
detail panel.

## Guarantees

`confidence_label` answers the question the user cares about:

- `guaranteed` — exact for the stated scope and freshness.
- `approximate` — heuristic, semantic, provider-limited, or otherwise
  useful but not guaranteed.
- `partial` — only part of the requested scope was searched.
- `stale` — based on a prior snapshot.
- `not_searched` — the subset was never searched under the current query.
- `unknown` — the surface cannot prove a stronger label and must route
  consequential actions through review or denial.

Rows with `approximate`, `partial`, `stale`, `not_searched`, or `unknown`
must expose `stale_or_partial_explanation` or a hidden/not-searched scope
disclosure. The explanation text must quote the frozen search readiness
copy where an existing sentence applies.

## Replace And Batch-Writes

Every row carries a `replace_scope` block, even when writes are not
applicable. Before a replace or batch-write command can move from visible
or selected rows to all matching rows, the surface must show:

- visible count;
- selected count;
- all-matching count and status;
- excluded count;
- blocked count;
- unknown count;
- the proposed write scope;
- the preview or batch-review packet ref that will own the review.

`scope_can_widen_without_review` is always `false`. A write path that
targets `all_matching_results` must resolve
`widening_review_requirement` to `preview_required`,
`batch_review_required`, or `blocked_until_review`; it must not silently
apply to all matches from a visible or selected row command.

Blocked and excluded are separate. Unknown is not a soft zero. If the count
is unknown, stale, provider-limited, or partial, the row must carry that
status to the review packet instead of flattening the number to exact.

## Deep Links And Saved Queries

Rows that can be reopened, exported, shared, cited by AI, or included in
support evidence must preserve:

- query session ref;
- query-history entry ref when available;
- saved-query ref when available;
- filter AST ref when available;
- scope-binding ref;
- result-set ref;
- search deep-link binding ref;
- deep-link drift state;
- freshness class at route mint;
- policy context.

`preserve_query_filters_scope_freshness` is always `true`. A reopen that
cannot prove the same query, filters, scope, and freshness must render the
drift state or refuse the route; it must not repaint captured rows as
current exhaustive truth.

## Fixture Acceptance

A fixture under `fixtures/search/result_cases/` is conforming only when a
reviewer can determine, without backend-specific jargon:

- whether the row is guaranteed, approximate, partial, stale, not searched,
  or unknown;
- which scope was searched and which subset was hidden, excluded, blocked,
  or unknown;
- whether the target is generated, ignored, provider-backed, or ordinary
  source;
- which quick actions remain safe;
- whether replace or batch-write would widen, and which review packet owns
  that widening; and
- which route refs preserve query, filters, scope, saved-query, deep-link,
  and freshness semantics.
