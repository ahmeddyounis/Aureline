# M5 docs-search-bar and scope-switcher primitive contract

**Task:** M05-869 — Implement docs search bars and scope switchers with corpus-class,
provider-availability, keyboard-hint, and cached-live state truth across claimed M5
docs browsers.

**Batch:** B102 — documentation-browser component truth.

This lane implements the reusable docs **search bar** and **scope switcher** —
two of the eight governed component families frozen by
[`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`](./freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md)
— as one working primitive with a real resolver, so a user can tell what corpus and
provider Aureline is searching and whether results are live, cached, mirrored, or
narrowed **before** they read any result.

## What this primitive owns

- A resolver — `resolve_docs_search` — that takes one search bar's label, scope
  target, searched corpus classes, source provider, provider availability, retrieval
  mode, version scope, keyboard hint, and freshness reading, and derives:
  - the **search-availability posture** (`M5DocsSearchAvailability`): live-ready,
    cached-ready, mirrored-ready, narrowed-provider-degraded, narrowed-policy-limited,
    degraded-provider-unavailable, degraded-offline-no-corpus, or blocked-unknown;
  - a self-contained **degraded-state banner** (`M5DocsSearchDegradedBanner`)
    whenever the search is narrowed, degraded, or blocked, naming the exact limit
    reason, the corpus in scope, the retrieval mode, and the next action — never an
    empty result list with no explanation.
- A parity matrix — `M5DocsSearchPrimitivePacket` — binding one row per claimed M5
  docs-search consumer (docs-browser search, onboarding / tutorial lookup, AI
  citation-follow, support / help search, CLI docs search) to the same anatomy,
  postures, provider availabilities, retrieval modes, limit reasons, next actions,
  export fields, and non-visual accessibility routes.

## Derivation ladder (blocking-first)

`derive_search_availability(provider, retrieval)` resolves in fixed order:

1. unknown provider availability **or** unknown retrieval mode → `blocked_unknown_state`;
2. no local corpus available → `degraded_offline_no_corpus`;
3. provider unavailable → `degraded_provider_unavailable`;
4. provider policy-limited → `narrowed_policy_limited`;
5. provider degraded → `narrowed_provider_degraded`;
6. mirror-only provider or mirrored retrieval → `search_mirrored_ready`;
7. cached or offline-bundled retrieval → `search_cached_ready`;
8. otherwise (available provider, live retrieval) → `search_live_ready`.

Cached and mirrored retrieval are carried **explicitly** in the resolved model and
in the export; they are never shown as live.

## Reused vs. minted vocabulary

Reused verbatim from the frozen docs-browser component matrix (no parallel grammar):
`M5DocsCorpusClass`, `M5DocsVersionScope`, `M5DocsSourceProvider`,
`M5DocsFreshnessState`, `M5DocsSurfaceFamily`, `M5DocsDeploymentLine`,
`M5DocsConsumerSurface`, `M5DocsAccessibilityRoute`, `M5DocsQualificationClass`,
`M5DocsDowngradeTrigger`.

Minted here (what the matrix left implicit about the bar/switcher themselves):
`M5DocsSearchConsumerSurface`, `M5DocsSearchBarAnatomyPart`,
`M5DocsProviderAvailability`, `M5DocsRetrievalMode`, `M5DocsSearchAvailability`,
`M5DocsSearchLimitReason`, `M5DocsSearchNextAction`, `M5DocsSearchBarExportField`.

## Acceptance-criteria mapping

- **Corpus / provider / live-cached-mirrored-narrowed visible before acting** —
  mandatory anatomy (`corpus_scope_label`, `scope_target_switcher`,
  `search_availability_verdict`) plus the derived posture; proven by the
  `availability_coverage_unproven` lint (≥1 ready and ≥1 not-ready worked example).
- **State model consistent across docs / help / onboarding / AI / CLI** — one
  primitive, one row per consumer, one shared vocabulary set; proven by the
  required-consumer and vocabulary-drift checks.
- **Offline / mirror / policy-blocked stays keyboard complete and explained** —
  mandatory `keyboard_hint` anatomy, required `keyboard_focusable` accessibility
  route, and the self-contained degraded banner; proven by the
  `scope_and_keyboard_explicit_unproven` and `degraded_banner_calm_explicit_unproven`
  lints.

## Boundary and evidence

- Boundary schema: `schemas/docs/m5-docs-search-bar-and-scope-switcher-primitive.schema.json`
- Support export (canonical, `include_str!`-checked):
  `artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/support_export.json`
- Matrix CSV: `artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/matrix.csv`
- Markdown report: `artifacts/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive.md`
- Narrowed fixtures (every consumer stays visible):
  `fixtures/docs/m5/m5-docs-search-bar-and-scope-switcher-primitive/`

Raw URLs, raw tokens, credentials, private endpoints, and user query bodies stay
outside the support boundary; the search-bar label, scope target, and keyboard hint
are carried only as opaque, export-safe representations.

## Reproduce

```sh
cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- support-export
cargo run -q -p aureline-docs --bin aureline_docs_search_bar_scope_switcher_primitive -- validate
cargo test -p aureline-docs --lib implement_docs_search_bars
```
