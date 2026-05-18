# Docs Knowledge Surface Beta

The docs knowledge-surface model is implemented in `aureline-docs::evidence_model`.
It sits above the existing citation alpha records and gives every docs-backed
surface the same provenance shape:

- source class, source pack, source revision, source build date, and running build identity
- freshness, version match, locale, locality, and mirror/offline posture
- citation anchors, citation drawer refs, keyboard inspection action refs, and source-open fallback
- truth downgrade labels for stale, unverified, mismatched, missing-anchor, and illustrative states
- curated/generated labels for derived explanations

## Surface Contract

Docs browser rows, glossary cards, architecture maps, topology/codebase
explainers, quick help, docs-backed search, docs-backed AI answers, and support
exports consume `DocsKnowledgeSurfaceProjection`. The `source_strip` is the
compact row every surface renders or exports; it repeats the same source,
version, freshness, locality, citation, external-open, and truth-label tokens.

Derived explanations use `DocsDerivedExplanation`. Generated or curated status
is mandatory, and every claim must either carry citation anchors or be marked as
inference with confidence. A generated explanation over stale docs downgrades to
`Retest pending`; illustrative-only or missing-anchor states downgrade to
`Illustrative`.

## Offline And Mirror Truth

Offline and mirrored packs do not get a lighter model. They still carry
`source_build_at`, `source_pack_revision_ref`, `running_build_identity_ref`,
`freshness_class`, `locale_overlay_state`, `mirror_offline_posture`, and exact
citation anchor refs. This keeps pinned glossary/tour packs useful without
letting cached teaching content look like live upstream truth.

## Proof Corpus

- Fixture manifest: `fixtures/docs/provenance_and_citation_truth/manifest.yaml`
- Evidence packet: `artifacts/docs/docs_evidence_packets/provenance_and_citation_truth_packet.json`
- Schemas:
  - `schemas/docs/docs_node_provenance.schema.json`
  - `schemas/docs/docs_derived_explanation.schema.json`

Run:

```sh
cargo test -p aureline-docs --test knowledge_surface_evidence_beta
```
