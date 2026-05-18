# Topology, Impact, and Explainer Beta Surface

The graph-understanding surface projects one semantic graph packet into topology, impact, and cited-explainer views without changing identity.

## Required Parity

- Every visible map node has a node table row with the same graph node id, freshness, confidence, and selection state.
- Every visible map edge has an edge table row with the same graph edge id, endpoints, relation class, freshness, confidence, and selection state.
- Impact rows use one controlled reason vocabulary across UI, CLI/headless, exports, and support packets: `exact_edge`, `shared_target`, `ownership_rule`, `generated_linkage`, `heuristic_similarity`, `policy_coupling`.
- Explainer claims show `curated` or `generated`, preserve citations, and carry omissions such as `outside_current_scope` rather than flattening them into generic partial text.

## Scope Vocabulary

Graph-backed topology maps, impact explorers, AI context inspectors, and review semantic hints use the same five labels:

- `Current repo`
- `Selected workset`
- `Full workspace`
- `Remote cache`
- `Outside current scope`

Partial graph, imported fact, parser-only fallback, policy-hidden, and outside-scope states remain visible in live surface packets and in support exports.

