# M5 Docs-Browser Component Surface Certification (M05-875)

This is the closing surface-certification capstone over the frozen M5 docs-browser
component matrix
(`freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`).

Where the freeze matrix defines the eight reusable docs-browser components — the
docs-search-bar, docs-scope-switcher, docs-result-row, symbol-linked-reference-card,
docs-source/version-badge, docs-pack-row, stale-example-finding-row, and
browser-handoff-banner — the M05-869..873 primitive lanes narrow each one and the
M05-874 accessibility lane proves keyboard / screen-reader / CLI-export parity and
per-family auto-narrowing, this capstone **certifies** that the shared component truth
holds on every claimed M5 docs / help / onboarding / AI surface, and auto-narrows any
surface that cannot sustain it.

## What it is keyed on

One `DocsSurfaceCertificationRow` per claimed **surface** — the surface a user actually
searches, opens, compares, cites, or exports documentation through, not the reusable
component family it renders:

| Surface | Token |
| --- | --- |
| Docs browser | `docs_browser` |
| Onboarding / learning tour | `onboarding` |
| Glossary | `glossary` |
| AI citations / evidence panel | `ai_citations` |
| Support / help | `support_help` |
| Mirror / offline docs console | `mirror_offline` |
| CLI / headless | `cli_headless` |
| Support / export bundle | `support_export` |

## Truth axes

Each surface is scored on six truth axes. The `cli_export` axis is **always-on** and
must stay certified on every row so support and automation can reconstruct the certified
corpus / source / version / pack / handoff truth from the same object identity the user
saw.

- `visual` — corpus class, provider/source, version/package scope, symbol anchor,
  project-doc override reason, and freshness are shown on-surface.
- `keyboard` — the same truth and its actions are reachable without a pointer.
- `screen_reader` — the same truth is announced non-visually, never color/badge-only.
- `cli_export` (always-on) — the surface state is reconstructable as text / JSON /
  Markdown.
- `degraded_state` — a cached/mirrored result, a version-adjacent match, an unverified
  symbol linkage, or a stale example honestly downgrades a `current_authoritative` /
  `supported_reference` claim.
- `source_and_handoff_provenance` — source class, version adjacency, mirror freshness,
  pack pin/offline/quarantine state, and the browser-handoff reason stay explicit, never
  inheriting a healthier surface's provenance or flattening a handoff into a bare URL
  jump.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`current_authoritative` / `supported_reference` claim while one of its truth axes is not
current is over-claiming and is blocked (red). A surface that discloses the reduction by
narrowing its docs-support claim — with a bound reason, a frozen downgrade trigger, and a
non-generic visible label — is honestly yellow. Every axis certified with the claim
delivered is green.

The derived status is never authored; it is recomputed from the axis outcomes and the
claim narrowing, and a stored status that disagrees with a fresh derivation is a
validation violation.

## Canonical bundle

Every row cites exactly one canonical proof bundle — the frozen docs-browser component
matrix release proof
(`artifacts/docs/m5/freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix/support_export.json`)
— rather than cloning per-surface evidence. Each row additionally records the M05-874
accessibility support export as supporting evidence. The packet is metadata-only: raw
docs bodies, provider tokens, and mirror cursors never cross this boundary.

## Coverage guarantees

- Every claimed surface is certified exactly once.
- Every frozen docs-browser component family is certified on at least one surface, so the
  full matrix runs across the claimed consumers.
- Every row scores every axis exactly once and keeps `cli_export` certified.

## Seeded certification

The checked-in packet certifies all eight surfaces: **four green** (docs browser,
onboarding, glossary, support export deliver their claim) and **four yellow** (AI
citations, support/help, mirror/offline, CLI/headless auto-narrow a not-current truth
axis to a weaker docs-support ceiling). No surface hides drift (red).

Known compatibility notes captured on the yellow rows cover: keyword-fallback citations
never inheriting an exact-symbol authoritative label, mirror freshness (cached/offline
results never reading as live), and pack quarantine (a quarantined pack never reading as
trusted).

## Artifacts

- Schema: `schemas/docs/m5-docs-browser-component-certification.schema.json`
- Support export (canonical): `artifacts/docs/m5/m5-docs-browser-component-certification/support_export.json`
- Matrix CSV: `artifacts/docs/m5/m5-docs-browser-component-certification/matrix.csv`
- Report: `artifacts/docs/m5/m5-docs-browser-component-certification/report.md`
- Fixtures mirror: `fixtures/docs/m5/m5-docs-browser-component-certification/`

Regenerate the artifacts (after changing the seed) with:

```
GEN_DOCS_CERT_ARTIFACTS=1 cargo test -p aureline-docs \
  certify_docs_search_bar_result_row_symbol_linked_reference_card_source_badge_pack_row_stale_example_row_and_handoff_banner_component_truth_on_every_claimed_m5_docs_surface::tests::generate_artifacts
```
