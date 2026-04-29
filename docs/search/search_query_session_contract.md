# Canonical search query-session, result-identity, and explanation contract

Search surfaces share one user intent even when they render that intent with
different density, provider mix, or client capabilities. Quick open, global
search, symbol search, docs search, AI context retrieval, CLI search, saved
query reopen, and support export must therefore cite one canonical query
session instead of copying query text, scope, ranking mode, result ids, and
explanations into surface-local records.

Machine-readable companions:

- [`/schemas/search/query_session.schema.json`](../../schemas/search/query_session.schema.json)
  defines the canonical `query_session_record`.
- [`/schemas/search/search_result_identity.schema.json`](../../schemas/search/search_result_identity.schema.json)
  defines the stable `search_result_identity_record` that tree/list/CLI/export
  surfaces cite.
- [`/schemas/search/search_explanation_capture.schema.json`](../../schemas/search/search_explanation_capture.schema.json)
  defines ranking and omission explanation captures with visibility tiers.
- [`/fixtures/search/query_session_cases/`](../../fixtures/search/query_session_cases/)
  contains worked cases for partial-index, policy-limited, provider-limited,
  hidden-by-filter, and recentness-boosted sessions.

This contract composes with, and does not replace:

- [`search_readiness_vocabulary.md`](./search_readiness_vocabulary.md) for
  readiness, result-truth, ranking-reason, hidden-scope, partial-truth,
  semantic-fallback, and deep-link drift tokens.
- [`query_planner_contract_seed.md`](./query_planner_contract_seed.md) for
  planner-pass, result-set, shard-snapshot, duplicate-merge, and streaming
  frame identity.
- [`../ux/search_result_contract.md`](../ux/search_result_contract.md) for
  renderer-facing search rows and replace-scope counts.
- [`../ux/quick_open_contract.md`](../ux/quick_open_contract.md) for quick-open
  row and same-surface explanation projection.
- [`../navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md)
  for durable saved-query, scope-binding, deep-link, and collection-snapshot
  reopen behavior.

If a vocabulary axis is already frozen by one of those contracts, records in
this family quote it by id or enum value. They do not mint synonyms.

## Canonical Session Rule

A `query_session_record` opens once per user-visible search intent. Related
surfaces may translate that intent into different planner passes, lane
groupings, or result projections, but they keep the same `query_session_id`
while the user expects continuity.

Examples:

| User flow | Session behavior |
|---|---|
| Quick open narrows from files to symbols | Same session; updated parse and planner-pass refs. |
| Global search opens a docs-result detail sheet | Same session; docs result identity cites the session and its own result-set ref. |
| AI context picker reuses a docs/code search | Same session when the query text and scope are inherited; new session when AI creates a materially different query. |
| CLI repeats a saved query | New session that cites the saved-query and captured-session refs; it must disclose current-vs-captured drift. |
| Support export captures a result bundle | Export quotes the original session and result identities; it does not claim captured rows are current live truth. |

Surfaces may close a session when the user commits an action, clears the query,
changes to an unrelated intent, or discards the route. Session close does not
delete durable query-history, saved-query, or support-capture refs whose
retention policy admits them.

## Query Session Fields

Every session record carries these field groups:

| Field group | Required contents |
|---|---|
| Session identity | `query_session_id`, source surface, projected result-surface classes, schema version, timestamps. |
| Query text posture | raw-local, redacted, hashed, support-redacted, or classification-only material class; secret-scan state; local literal-store ref when present. |
| Normalized parse | parser id/version, query language, query classification, normalized terms, boosted/excluded/scope terms, filters, parse warnings. |
| Scope binding | scope-binding ref, scope source, scope class, lane grouping, workset/root refs, included/excluded scope refs, hidden-result disclosure. |
| Ranking profile | ranking mode, lexical-first posture, stable-ordering policy, visible/debug reason posture, allowed ranking-reason classes, ranker epoch. |
| History policy | history class, retention class, sensitive-literal handling, saved-query/history refs, sync/share eligibility. |
| Freshness state | readiness, freshness, semantic fallback, stale/partial explanation, hidden-result disclosure, index refresh timestamps. |
| Retrieval/index epoch | retrieval epoch, index epoch, graph/docs/embedding epochs, locality class, shard-snapshot refs. |
| Result snapshots | result-set/planner-pass refs, captured result identity refs, visible/hidden counts, snapshot truth claim. |
| Export/sharing posture | local-by-default flag, share policy, support capture posture, included snapshot fields, explicit no-current-live-truth class. |

No surface may fork one of these groups into an incompatible local structure.
Surface-specific rows can cache a projection for rendering, but the row must
cite the session and reconcile against it when reopened, exported, or explained.

## Query Material And Privacy

Query material is local by default.

- `raw_local_only` material is admitted only when the share policy is
  `local_only_private`, support capture is `not_captured`, export material is
  `metadata_only`, raw-query export is false, and synced history is false.
- Exportable, syncable, CLI, support, managed, or shared sessions must use
  `redacted_text`, `hashed_terms_only`, `support_redacted`, or
  `classification_only`.
- Redacted text is reviewable copy, not proof that the raw query is safe.
  Secret detection state remains explicit.
- Salted query hashes are comparison aids. The salt is local-only and must not
  be reused across tenants, workspaces, or support packets.
- A session that cannot prove the query-material posture denies export rather
  than silently dropping text and pretending the packet is complete.

Normalized parse records follow the same rule. A boosted term may carry a
redacted term label or local-only literal ref, but exported explanations cite
the term id or hash ref.

## Scope And Snapshot Truth

A query session is always bound to a scope object. The scope binding states
whether the query came from current context, explicit user selection, a saved
query, policy narrowing, support capture, CLI flags, or AI context inheritance.

Counts stay separate:

- visible result count;
- matching count, when known;
- hidden count and hidden reasons;
- blocked count and blocked reasons;
- unknown count, when a provider or index cannot prove a count.

Captured result snapshots are not current truth. A `result_snapshot_ref` must
state whether it is `current_at_capture_only`, requires live revalidation, is a
captured snapshot that is not current truth, or is replay-only history. Support
exports and shared bundles must preserve this state even when they include the
same visible rows the user saw.

## Stable Result Identity

`search_result_identity_record` is the row identity that survives different
rendering surfaces. It is not a display label, path string, sort position, or
provider payload.

Every identity records:

- object kind and canonical entity ref;
- optional path, symbol, docs, graph, provider, generated-lineage, command, or
  coordinate anchors;
- access kind and current access state;
- source classes that contributed to the row;
- result-truth class, readiness, freshness, semantic fallback, and ranking
  reasons copied from the owning packet/fusion record;
- visible/matching/hidden/blocked count disclosures;
- drift and remap state for reopen/export/support flows; and
- projection keys for the UI, CLI, export, or support rows that render it.

Hidden placeholders are identities too, but they disclose only count and reason.
They must not carry hidden labels, paths, symbols, provider payloads, or raw
content.

## Explanation Capture

`search_explanation_capture_record` answers "why did this result appear, rank,
or disappear?" with reusable explanation signals rather than free-form,
surface-local text.

The core signal vocabulary covers:

- boosted term match;
- exact match and prefix/fuzzy match;
- recentness and frequent-use boost;
- semantic match;
- docs anchor and graph-neighborhood match;
- scope filter match or exclusion;
- hidden-by-filter, policy-hidden, and provider-limited omissions;
- stale, degraded, and partial index state;
- freshness floor; and
- imported-source or structural-fallback match.

Visibility tiers are part of the record:

| Tier | Meaning |
|---|---|
| `always_visible` | Material truth that ordinary rows must expose when relevant: result truth, readiness, freshness, source class, scope, primary reasons, hidden count, blocked count, semantic fallback. |
| `detail_visible` | Inspectable row/detail-sheet facts such as all contributing reason tokens and source refs. |
| `opt_in_debug` | Power-user debug facts such as shard refs, normalized term hash refs, planner stage refs, and weight buckets. |
| `support_export_redacted` | Support-safe metadata with raw material stripped. |
| `forbidden_secret` | A refused axis, used only to say a field was not captured because it would expose raw or secret material. |

Numeric private ranking weights, raw query text, raw document bodies, raw
symbol definitions, raw absolute paths, raw provider payloads, raw URLs, raw
secrets, and support-only unexplained heuristics are forbidden explanation
fields. A support-only debug artifact may supplement an explanation, but it
must never be the only reason a result ranked, disappeared, or became blocked.

## Surface Mapping

| Source surface | Primary search-result projection |
|---|---|
| `quick_open` | quick-open row plus `full_search` or `symbol_jump` packet refs, depending on lane. |
| `global_search` / `file_search` | `full_search`. |
| `symbol_search` | `symbol_jump`. |
| `docs_search` | `docs_search`. |
| `graph_search` | `graph_overlay`. |
| `ai_context_search` | `ai_explanation_overlay` for derived context rows, with cited source identities. |
| `cli_search` | CLI projection of the same session, result identities, and explanation captures. |
| `support_export` | Quoted owner packets, identities, and explanation captures only. |

CLI and support export are presentation/client scopes, not new search truths.
They cite the same session and identity refs the desktop surface would cite.

## Export And Support Rules

Export and support packets must:

- cite `query_session_id`, `result_identity_id`, planner-pass, result-set, and
  result-snapshot refs instead of re-minting row truth;
- preserve ranking reasons, hidden-result disclosures, blocked-result
  disclosures, stale/partial explanations, source classes, and drift/remap
  state;
- include no raw query text unless the user explicitly opted into a raw local
  export posture and policy allows it;
- include no raw document bodies, raw symbol definitions, raw URLs, provider
  payloads, or secrets;
- state that captured results are a snapshot or replay basis, not current live
  truth, unless the export explicitly records live revalidation; and
- fail closed when a required field is missing or a redaction rule cannot be
  proven.

## Fixture Acceptance

A query-session fixture is conforming when a reviewer can determine, without
backend-specific jargon:

- which single session object owns the query, scope, ranking mode, freshness,
  retrieval epoch, and snapshot refs;
- whether query text is raw-local, redacted, hashed, support-redacted, or
  classification-only;
- which stable result identities would render in UI, CLI, export, and support
  packets;
- whether hidden, blocked, provider-limited, policy-limited, stale, degraded,
  or partial states are explicit;
- which reusable explanation signals appear and which are always visible versus
  debug-only; and
- whether captured result bundles avoid claiming current live truth.
