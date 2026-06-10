# Retrieval-Debug Surfaces (docs, recall, AI context)

- Packet: `packet:m5:retrieval_debug:net_retry_query`
- Query: retrieval debug: how does the networking retry backoff work
- Promotion: `stable` (0 findings)
- Entries: 3 | Degradations: 1

## Entries

- [docs_search] `entry:docs:retry_with_backoff_symbol` (retry_with_backoff (symbol reference)) — label `exact` — workspace_code / exact_build_match / authoritative_live / local / high
  - Derivation reason: exact symbol-name match against the local workspace index at the active build revision; labelled exact and high
  - Cited: true | Ranking signals: 2
- [semantic_recall] `entry:recall:backoff_policy_guide` (Exponential backoff guidance (imported pack)) — label `imported` — imported_pack / compatible_minor_drift / warm_cached / mirrored_pack / medium
  - Derivation reason: came in through a pinned imported pack rather than the workspace; labelled imported and held to medium because it is not workspace-verified
  - Cited: true | Ranking signals: 2
- [ai_context] `entry:ai_context:retry_explanation_fragment` (Retry/backoff context fragment) — label `heuristic` — ai_assembled_context / exact_build_match / warm_cached / local / low
  - Derivation reason: assembled by a heuristic chunk-selection pass over the cited symbol and guide; labelled heuristic and held to low confidence
  - Cited: true | Ranking signals: 2

## Degradations

- [index_stale/advisory]: the recall index was built before the last two commits; recall entries may lag the working tree
