# M5 docs-browser component matrix (design QA)

Shared design / schema / QA / release matrix for the reusable M5 documentation
browser components (row **M05-868**, batch B102). Design, schema, QA, and release
owners consume this one matrix instead of rewording docs truth per surface.

**Canonical truth (do not re-key):**

- Contract doc:
  `docs/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix.md`
- Schema:
  `schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json`
- Support export + CSV + report:
  `artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_browser_component_matrix -- report`

## Component families and the truth each must always show

| Family | Must always show | Never allowed to |
| --- | --- | --- |
| `docs_search_bar` | corpus class + source provider searched | leave corpus/origin implicit |
| `docs_scope_switcher` | version / package scope in effect | hide which version is being read |
| `docs_result_row` | match state + project-doc override reason | show nearby/cached as exact-live; reorder silently |
| `symbol_linked_reference_card` | symbol anchor + resolution | show an unresolved anchor as a resolved deep link |
| `docs_source_version_badge` | source provider + freshness | show mirrored/cached as live first-party truth |
| `docs_pack_row` | pin / mirror / offline / quarantine state | present a quarantined/offline pack as freely trusted |
| `stale_example_finding_row` | stale-example status | show a drifted/broken example as current guidance |
| `docs_handoff_banner` | exact browser-handoff reason | dead-end a handoff without stating why |

## Design acceptance gates

1. **One vocabulary.** Corpus class, source provider, version scope, match state,
   override reason, symbol anchor, freshness, pack state, stale-example status, and
   handoff reason use only the frozen tokens in the schema/contract. No surface
   invents parallel labels.
2. **Mandatory labels.** Every component exposes `identity`, `state`, and
   `keyboard_route`, plus the truth labels relevant to it (`corpus_class`,
   `source_provider`, `freshness`).
3. **Non-visual parity.** Every component is keyboard-focusable, screen-reader
   announced, non-hover reachable, pointer-optional, high-contrast safe, and
   support-exportable. Nothing is hover-only or browser-only.
4. **Deployment parity.** The same truth survives local-OSS, self-hosted, managed,
   air-gapped, and mirror/offline lines.
5. **Auto-narrowing.** When a downgrade trigger fires the component drops below
   Stable while staying visible (fixtures: `stale_example_finding_row` → Beta,
   `docs_handoff_banner` → Preview).

See `matrix.csv` in the canonical artifact directory for the per-family
surface-family / deployment-line / required-label / consumer-surface /
downgrade-trigger grid.
