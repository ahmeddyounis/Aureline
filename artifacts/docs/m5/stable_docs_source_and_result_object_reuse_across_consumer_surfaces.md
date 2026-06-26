# Stable Docs Source/Result Object Reuse Across Consumer Surfaces

- Packet: `packet:stable_docs_source_and_result_object_reuse:001`
- Schema: `schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json`
- Support export: `artifacts/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/support_export.json`
- Contract doc: `docs/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces.md`
- Fixtures: `fixtures/docs/m5/stable_docs_source_and_result_object_reuse_across_consumer_surfaces/`

## Coverage

- One canonical `DocsSourceDescriptor` per source class carries source class, provider or pack identity, BCP-47 locale, trust class, browser-handoff capability, mirror/offline posture, version-match state, and freshness state.
- One canonical `DocsResult` per source carries a stable result id, a title, a ref to its source descriptor, the version-match and freshness state it observed, symbol refs or citation anchors, snippet metadata that never forces full-content export, and a support/export-safe identity.
- The packet keeps the project-documentation, mirrored-official-docs, extension-contributed-docs, live-external-docs, and derived-explanation source classes all represented so they stay distinguishable across every consuming surface.
- A `DocsObjectSurfaceProjection` reuses the same source/result objects on docs search, symbol-linked reference cards, hover/peek docs, AI citations, glossary cards, and support exports — each projection keeps the source class, version match, freshness, trust class, and symbol/citation linkage visible and preserves result identity without forcing full content.
- The support export preserves the exact packet identity without exporting full content, raw private material, or ambient authority.

## Trust guardrails

The packet proves documentation truth stays typed and inspectable: source class, locale, version match, freshness, mirror/offline state, and trust class stay visible; project documentation never masquerades as vendor docs (a project source labeled with a vendor or provider trust class blocks promotion); derived explanations never claim primary authority (a derived source that claims precedence blocks promotion); live external docs always resolve through an explicit, isolated browser handoff; a result never silently upgrades the version-match or freshness state of its source; no surface — including support export and browser handoff — forces full-content export; and no consuming surface mints a private badge vocabulary instead of reusing the shared one. Raw document bodies, raw source files, raw URLs, raw provider payloads, and credentials never cross the boundary.
