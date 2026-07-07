# M5 symbol-linked reference-card primitive

Row **M05-871** — batch B102 (documentation-browser component lane).

This lane implements the reusable **symbol-linked reference card** named in the frozen
M5 docs-browser component matrix
(`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`)
as one governed primitive, so that when Aureline jumps from code to docs the card keeps
its **initiating file/symbol code anchor** visible and always says **how strong the
symbol linkage actually is** — an exact symbol match, a nearby version match, a
project-specific override, or a keyword fallback — instead of blending every case into
one "docs found" card.

## Two halves

1. **Resolver** — `resolve_reference_card(&M5DocsReferenceCardResolutionInput)
   -> Result<M5ResolvedDocsReferenceCard, M5DocsReferenceCardResolutionError>`.
   From one card's title, initiating file and symbol, symbol anchor, corpus class,
   source provider, match state, override reason, version scope, freshness, cited
   source revision, and open action it derives:
   - the **symbol-linkage strength** (`M5DocsSymbolLinkageStrength`) — honesty-first:
     an unresolved anchor never reads as an exact symbol match (it reads as a keyword
     fallback, or — when even the match is stale — an unresolved no-linkage stub); then
     a project-pinned/project-specific card reads as project-specific linkage, an exact
     match at a non-nearby version reads as exact-symbol linkage, a nearby version or
     nearby match reads as nearby-version linkage, and a mirror/cache/stale-served match
     reads as heuristic linkage;
   - the **freshness posture** (`M5DocsCardFreshnessPosture`) — a cached, mirrored, or
     stale cited revision is never shown as live even when its declared freshness would
     suggest it;
   - a **linkage disclosure** (`M5DocsReferenceCardLinkageDisclosure`) — always present
     — naming why the card appeared and how strong the linkage is.
2. **Parity matrix** — `M5DocsReferenceCardPrimitivePacket` — binds one row per claimed
   reference-card consumer (editor hover/peek, docs-browser card, AI-explanation card,
   onboarding reference card, support evidence card) to the shared card anatomy, the
   same linkage strengths, freshness postures, symbol anchors, match states, override
   reasons, export fields, and non-visual accessibility routes.

## Reused vs minted vocabulary

Reused verbatim from the frozen component matrix: `M5DocsSymbolAnchor`,
`M5DocsCorpusClass`, `M5DocsVersionScope`, `M5DocsSourceProvider`,
`M5DocsFreshnessState`, `M5DocsMatchState`, `M5DocsOverrideReason`,
`M5DocsSurfaceFamily`, `M5DocsDeploymentLine`, `M5DocsConsumerSurface`,
`M5DocsAccessibilityRoute`, `M5DocsQualificationClass`, `M5DocsDowngradeTrigger`.

Minted here (only what the matrix left implicit about the card itself):
`M5DocsReferenceCardConsumerSurface`, `M5DocsReferenceCardAnatomyPart`,
`M5DocsSymbolLinkageStrength`, `M5DocsCardFreshnessPosture`,
`M5DocsReferenceCardExportField`.

## Acceptance criteria mapped to lints

- **A user can tell why a card appeared and how strong the linkage is** — every card
  carries a self-contained `linkage_disclosure`; the four named states
  (`exact_symbol_linkage`, `nearby_version_linkage`, `project_specific_linkage`,
  `keyword_fallback_linkage`) must each be proven by a worked resolution
  (`LinkageStateCoverageUnproven`).
- **Exact / nearby / project-specific / keyword-fallback stay explicit** — the linkage
  strength is derived on a fixed honesty-first ladder and never collapses to a single
  "docs found" tag.
- **Reference-card identity survives export/support/AI evidence paths** — every worked
  resolution preserves the initiating file/symbol anchor and the matrix proves both a
  resolved and an unresolved anchor (`AnchorIdentityUnproven`); the mandatory export
  fields carry the linkage strength, symbol anchor, initiating anchor, source provider,
  version scope, and freshness state.

Freshness visibility is also proven across the matrix (`FreshnessVisibilityUnproven`):
at least one live and one not-live cited revision.

## Artifacts

- Schema: `schemas/docs/m5-symbol-linked-reference-card-primitive.schema.json`
- Support export / matrix CSV:
  `artifacts/docs/m5/m5-symbol-linked-reference-card-primitive/`
- Markdown report: `artifacts/docs/m5/m5-symbol-linked-reference-card-primitive.md`
- Narrowed fixtures: `fixtures/docs/m5/m5-symbol-linked-reference-card-primitive/`

All four are minted only by the headless emitter
`aureline_docs_symbol_linked_reference_card_primitive` from the seed builders, so the
in-code matrix, the schema, and the checked-in artifacts never drift.
