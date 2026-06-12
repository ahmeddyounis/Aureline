# Retrieval-Debug Surfaces (docs, recall, AI context)

- Packet: `packet:m5:retrieval_debug:checkout_infra_query`
- Query: retrieval debug: why does checkout drift from the cluster deployment
- Promotion: `stable` (0 findings)
- Entries: 3 | Degradations: 1

## Entries

- [docs_search] `entry:docs:checkout-rendered-manifest` (checkout rendered Deployment manifest) — label `exact` — project_docs / exact_build_match / warm_cached / local / high
  - Derivation reason: exact manifest-path and object-name match against the local infra docs index; the answer explicitly cites authored, rendered, and planned layers.
  - Cited: true | Ranking signals: 2
  - Infrastructure truth layers: authored_desired, rendered_expanded, planned_validated
  - Infrastructure limits: Provider overlay was unavailable, so the answer degraded to authored, rendered, and planned checkout intelligence.
- [semantic_recall] `entry:recall:checkout-runbook-drift` (Checkout Kubernetes drift runbook) — label `imported` — imported_pack / compatible_minor_drift / warm_cached / mirrored_pack / medium
  - Derivation reason: came in through a pinned imported infra runbook pack rather than the workspace; labelled imported and held to medium because only the observed-layer mapping is mirrored.
  - Cited: true | Ranking signals: 2
  - Infrastructure truth layers: rendered_expanded, observed_live
- [ai_context] `entry:ai_context:checkout-drift-fragment` (Checkout drift context fragment) — label `heuristic` — ai_assembled_context / exact_build_match / warm_cached / local / low
  - Derivation reason: assembled by a heuristic chunk-selection pass over authored manifests, rendered output, and the imported drift runbook; labelled heuristic and held to low confidence.
  - Cited: true | Ranking signals: 2
  - Infrastructure truth layers: authored_desired, rendered_expanded
  - Infrastructure limits: Live cluster and provider overlays were unavailable, so AI context degraded to authored and rendered checkout facts instead of claiming runtime certainty.

## Degradations

- [index_stale/advisory]: the infra recall index was built before the latest manifest regeneration; retrieval results may lag the working tree
