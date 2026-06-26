# Docs-Source Precedence and Ranking Parity

- Packet: `packet:docs_source_precedence_and_ranking_parity:001`
- Schema: `schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json`
- Support export: `artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/support_export.json`
- Contract doc: `docs/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md`
- Fixtures: `fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/`
- Producer: `aureline_docs::current_stable_docs_precedence_ranking_export`

## Coverage

- A `DocsRankingSet` is a ranked answer set for one subject (a search query, a hovered symbol, a natural-language question, an onboarding topic, or an AI-context subject). Each candidate carries a stable `DocsSourceLane`, a `SourcePrecedenceClass`, a closed `PrecedenceReason` plus a human-readable note, its version-match and freshness state, its mirror/offline posture, and the project-specific / override cues a surface shows.
- The seven distinguishable lanes — project docs, generated docs, mirrored official docs, curated knowledge packs, extension-contributed docs, live external docs, and derived explanations — are derived from a candidate's source class and trust class so precedence ranking never flattens them. The curated-knowledge-pack source class splits into the curated and extension-contributed lanes by trust class, which is the only way extension-contributed docs stay distinguishable from a first-party curated pack.
- A `RankExplanationProjection` reuses the same ranking explanation on docs search, hover/peek, onboarding, AI context, and support export — each projection keeps the source class, precedence reason, version match, freshness, and project/override cue visible, stays inspectable on demand, and reuses the shared ranking vocabulary instead of a hidden, parallel ranking model. The support-export projection reconstructs every ranking set.
- The support export preserves the exact packet identity without exporting full content, raw private material, or the ranking explanation flattened away.

## Ranking guardrails

The packet proves precedence stays typed and inspectable. Project docs may outrank vendor docs for a repo-specific question, but a project-outranks-vendor candidate must keep at least one vendor / mirrored alternative visible in the same set and reference it (otherwise promotion blocks). A less-authoritative source may rank above a more-authoritative one only when it carries a precedence reason that justifies the inversion. A derived explanation never claims primary authority (it may not rank first and must declare a not-applicable precedence class). A candidate's precedence reason must stay consistent with its precedence class, project docs labelled with a vendor trust class resolve to no distinguishable lane, and no consumer surface may mint a hidden ranking model that ignores source-class, version-match, or freshness truth. An offline / air-gapped profile keeps a candidate inspectable with an explicit unavailable reason rather than dropping it or substituting generic web search; an honestly disclosed offline candidate and a surfaced project/vendor disagreement narrow below stable rather than blocking. Raw document bodies, raw source files, raw URLs, raw provider payloads, and credentials never cross the boundary.
