# Citation Truth Alpha

The citation alpha model is implemented in `aureline-docs` and is shared by
docs/help rows, search-linked docs rows, graph explainers, onboarding/help-pack
exports, and AI evidence packets.

## Records

- `DocsNodeIdentity` records stable docs-node identity, source class, source pack
  and revision, locale fallback, freshness, locality, and exact reopen refs.
- `CitationAnchorAlpha` records citation-anchor identity, exact-anchor
  availability, freshness, locality, confidence, and inference markers.
- `CitationDrawerEvidenceView` is the drawer/evidence projection used by UI
  surfaces and non-canvas fallbacks.
- `CitationEvidenceExport` preserves docs nodes, citation drawers, help-pack
  items, and AI/explainer packet refs for support reconstruction without raw
  document bodies, raw URLs, prompt text, or provider payloads.

The bounded schemas are checked in at:

- `schemas/docs/docs_node_alpha.schema.json`
- `schemas/docs/citation_anchor_alpha.schema.json`

## Protected Proof Path

The protected fixture manifest is
`fixtures/docs/citation_truth_alpha/manifest.yaml`. It points at
`artifacts/docs/citation_export_sample_alpha.json`, which exercises:

- exact anchor preservation for a product help row
- disclosed missing-anchor downgrade for a not-installed onboarding pack
- visible project-doc versus vendor-doc precedence
- derived explainer inference marking
- onboarding locale fallback with pack id, item id, citation refs, and
  source-language fallback ref

## Verification

Run the citation proof tests with:

```sh
cargo test -p aureline-docs --test citation_truth_alpha
```

Consumer coverage is exercised through the existing docs-linked search, graph
explainer, AI evidence, and shell docs-pack tests.
