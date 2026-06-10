# Topology, Ownership, and Codebase Explainer Cards

- Packet: `packet:m5:understanding_cards:net_retry_region`
- Region: codebase understanding: the networking retry region
- Promotion: `stable` (0 findings)
- Cards: 3 | Degradations: 1

## Cards

- [topology_map] `card:topology:net_retry_region` (Networking retry region topology) — graph_index / exact_build_match / warm_cached / local / medium
  - Confidence reason: derived from the workspace graph index at the active build revision; warm-cached so labelled medium rather than live
  - Provenance: derived_summary / cited=true / evidence=2
- [ownership_surface] `card:ownership:net_retry_region` (Networking retry region ownership) — codeowners_file / exact_build_match / authoritative_live / local / high
  - Confidence reason: owner read verbatim from a matched CODEOWNERS entry at the active revision; authoritative declaration so labelled high
  - Provenance: verbatim_node / cited=true / evidence=1
- [codebase_explainer] `card:explainer:retry_with_backoff` (What retry_with_backoff does) — workspace_code / exact_build_match / warm_cached / local / medium
  - Confidence reason: an inferred explanation generated over the cited symbol and retry-policy guide; labelled medium because it is inferred, not a verbatim doc
  - Provenance: inferred_explanation / cited=true / evidence=2

## Degradations

- [graph_index_stale/advisory]: the workspace graph was indexed before the last two commits; topology edges may lag the working tree
