# M5 docs-result-row and source-version-badge primitive contract

**Task:** M05-870 — Ship docs result rows and source or version badges with
result-kind, provider, version-scope, symbol-match-confidence, freshness, and
rank-reason truth across claimed M5 knowledge surfaces.

**Batch:** B102 — documentation-browser component truth.

This lane implements the reusable docs **result row** and **source/version badge** —
two of the eight governed component families frozen by
[`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`](./freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md)
— as one working primitive with a real resolver, so documentation answers stop
inheriting hidden certainty from lower-level search objects. A user can tell, from
the row and its badge alone, what kind of result it is, whether it is a local/project
doc or an upstream/vendor doc, whether its freshness reads as current or is
explicitly cached, mirrored, or stale, and why a project doc outranked vendor docs —
**before** they open it.

## What this primitive owns

- A resolver — `resolve_docs_result_row` — that takes one result's title, kind,
  corpus class, source provider, match state, override reason, symbol-match
  confidence, version scope, freshness, and open-action target, and derives:
  - the **source-badge class** (`M5DocsSourceBadgeClass`): local-project-docs,
    workspace-spec, first-party-reference, cached/mirrored-reference,
    live-vendor-upstream, extension-contributed, or ai-derived-explanation — each
    with a color-independent `glyph_label`, so a user distinguishes local/project
    from upstream/vendor at row level without relying on color;
  - the **freshness posture** (`M5DocsResultFreshnessPosture`): current-live,
    recently-synced-current, cached-explicit-not-live, mirrored-explicit-not-live,
    stale-flagged, or freshness-unknown — a cached, mirrored, or stale result is
    never shown as live even when its declared freshness would suggest it;
  - a self-contained **rank-reason disclosure** (`M5DocsRankReasonDisclosure`)
    whenever project-doc precedence, version adjacency, or mirror freshness
    materially decides the ranking, naming the exact rank factor and override reason
    — never a silent reorder.
- A parity matrix — `M5DocsResultRowPrimitivePacket` — binding one row per claimed M5
  docs-result consumer (docs-browser result, AI-answer citation, onboarding step
  reference, support answer result, CLI result list) to the same anatomy, badge
  classes, postures, match states, override reasons, rank factors, symbol-match
  confidences, export fields, and non-visual accessibility routes.

## Derivation ladders

`derive_source_badge_class(provider, corpus, scope)` resolves in fixed, specific-first
order: ai-derived provider → `ai_derived_explanation`; project-specific scope →
`local_project_docs`; codebase-symbol corpus → `workspace_spec`;
community-contributed corpus → `extension_contributed`; vendor-dependency corpus or
third-party provider → `live_vendor_upstream`; mirrored / offline-import /
bundled-local provider → `cached_mirrored_reference`; otherwise (first-party hosted)
→ `first_party_reference`.

`derive_freshness_posture(freshness, match_state)` keeps a cached, mirrored, or stale
match explicit even when the declared freshness is `live_current`, so the result is
never shown as live.

`derive_rank_factor(override_reason, version_scope)` maps each project-doc override
reason to its rank factor, reads a no-override nearby-version match as
`version_adjacency`, and produces no disclosure for a plain default ranking.

## Reused vs. minted vocabulary

Reused verbatim from the frozen docs-browser component matrix (no parallel grammar):
`M5DocsCorpusClass`, `M5DocsVersionScope`, `M5DocsSourceProvider`,
`M5DocsFreshnessState`, `M5DocsMatchState`, `M5DocsOverrideReason`,
`M5DocsSurfaceFamily`, `M5DocsDeploymentLine`, `M5DocsConsumerSurface`,
`M5DocsAccessibilityRoute`, `M5DocsQualificationClass`, `M5DocsDowngradeTrigger`.

Minted here (what the matrix left implicit about the row/badge themselves):
`M5DocsResultConsumerSurface`, `M5DocsResultRowAnatomyPart`, `M5DocsResultKind`,
`M5DocsSourceBadgeClass`, `M5DocsSymbolMatchConfidence`,
`M5DocsResultFreshnessPosture`, `M5DocsRankFactor`, `M5DocsResultRowExportField`.

## Acceptance-criteria mapping

- **Distinguish local/project docs from upstream/vendor at row level before opening**
  — the derived `source_badge_class` and its `is_local_or_project` predicate, plus
  the mandatory `source_provider_badge` anatomy; proven by the
  `local_vs_upstream_coverage_unproven` lint (≥1 local and ≥1 upstream worked
  example).
- **Version / freshness state visible wherever a result is reused (docs browser, AI
  answer, onboarding, support)** — the derived `freshness_posture`, mandatory
  `version_scope_badge` and `freshness_badge` anatomy, and mandatory export fields;
  proven by the `freshness_visibility_unproven` lint (≥1 live and ≥1 not-live worked
  example).
- **Badge / state vocabulary stable across UI, docs/help, exports, and support
  packets** — one primitive, one row per consumer, one shared vocabulary set, one
  mandatory export-field set; proven by the required-consumer, vocabulary-drift, and
  mandatory-export-field checks. The `rank_reason_inspectable_unproven` lint proves a
  materially overridden ranking always ships an inspectable disclosure.

## Boundary and evidence

- Boundary schema: `schemas/docs/m5-docs-result-row-and-source-version-badge-primitive.schema.json`
- Support export (canonical, `include_str!`-checked):
  `artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/support_export.json`
- Matrix CSV: `artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/matrix.csv`
- Markdown report: `artifacts/docs/m5/m5-docs-result-row-and-source-version-badge-primitive.md`
- Narrowed fixtures (every consumer stays visible):
  `fixtures/docs/m5/m5-docs-result-row-and-source-version-badge-primitive/`

Raw URLs, raw tokens, credentials, private endpoints, and result bodies stay outside
the support boundary; the result title and open-action target are carried only as
opaque, export-safe representations.

## Reproduce

```sh
cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- support-export
cargo run -q -p aureline-docs --bin aureline_docs_result_row_source_version_badge_primitive -- validate
cargo test -p aureline-docs --lib implement_docs_result_rows
```
