# M5 Docs and Code-Understanding Certification

- Packet: `m5-docs-certification:stable:0001`
- Label: `M5 Docs and Code-Understanding Certification`
- Surfaces: 10 (10 certified, 0 narrowed/held/blocked)
- Downgrade rules: 6 (auto-enforced)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-09T00:00:00Z)

## Surfaces

- **docs_pack_recall**: `stable` / `certified`
  - Scope: Mirror-aware docs-pack recall with source/version/freshness chips and stale-example findings; pinned signed mirrors outrank live vendor docs
  - Schema: `schemas/docs/implement-mirrored-docs-pack-recall-source-or-version-or-freshness-chips-and-stale-example-findings.schema.json`
- **semantic_recall**: `stable` / `certified`
  - Scope: Docs and code semantic recall with a query-session ledger, explicit ranking reasons, and a cited provenance export
  - Schema: `schemas/docs/implement-docs-and-code-semantic-recall-with-query-session-ledger-ranking-reasons-and-provenance-export.schema.json`
- **codebase_understanding_cards**: `stable` / `certified`
  - Scope: Cited topology, ownership, and codebase-explainer cards that preserve source class and confidence with open-raw/open-source escapes
  - Schema: `schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json`
- **retrieval_debug**: `stable` / `certified`
  - Scope: Retrieval-debug inspector with exact/imported/heuristic labeling and explicit ranking reasons for every docs/code recall result
  - Schema: `schemas/docs/ship-retrieval-debug-surfaces-for-docs-recall-and-ai-context-with-exact-or-imported-or-heuristic-labeling.schema.json`
- **scoped_browser_surface**: `beta` / `certified`
  - Scope: Narrow, attributable docs/review browser surfaces with explicit handoff reason, return-path safety, and trust-class disclosure
  - Schema: `schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json`
- **light_remote_edit**: `beta` / `certified`
  - Scope: Browser-lite light remote-edit surfaces with narrow scope, stale-state honesty, and no hidden authority expansion
  - Schema: `schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json`
- **saved_query_privacy**: `stable` / `certified`
  - Scope: Saved-query privacy controls with local-versus-shared retention truth and support-export-safe search history
  - Schema: `schemas/docs/ship-saved-query-privacy-controls-local-versus-shared-retention-and-support-export-safe-search-history.schema.json`
- **docs_authoring_review**: `beta` / `certified`
  - Scope: Docs authoring suggestions with stale-link/stale-example review verdicts, apply-posture truth, and open-raw/open-source escapes
  - Schema: `schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json`
- **docs_search_link**: `stable` / `certified`
  - Scope: Docs search with symbol-linked reference cards and code-anchor-preserving deep links
  - Schema: `schemas/docs/ship-docs-search-symbol-linked-reference-cards-and-code-anchor-preserving-deep-links.schema.json`
- **recall_matrix**: `stable` / `certified`
  - Scope: The frozen M5 docs and code-recall matrix this certification binds against; no certified surface stays greener than the matrix
  - Schema: `schemas/docs/freeze-the-m5-docs-and-code-recall-matrix-browser-surface-scope-and-retrieval-debug-contract.schema.json`
