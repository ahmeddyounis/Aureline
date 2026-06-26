# Derived-Explanation Citation Sets

This document is the contract for the citation sets that bind every claimed M5
derived explanation to the evidence it actually depended on. A derived
explanation is prose the product generates rather than authors: a docs-browser
explanation, an AI answer, a glossary card, a guided-tour step, an architecture
explainer, or a support-export note. Each one must attach exactly one
`DerivedExplanationCitationSet` or explicitly label itself an inference, so
generated prose never becomes the only durable record of repo truth.

- Record kind: `derived_explanation_citation_sets_packet`
- Support-export record kind: `derived_explanation_citation_sets_support_export`
- Schema: [`schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json`](../../../schemas/docs/implement-derived-explanation-citation-sets-binding-docs-ai-glossary-tours-and-support-exports.schema.json)
- Canonical support export: [`artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/support_export.json`](../../../artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/support_export.json)
- Summary artifact: [`artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md`](../../../artifacts/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports.md)
- Fixtures: [`fixtures/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/`](../../../fixtures/docs/m5/implement_derived_explanation_citation_sets_binding_docs_ai_glossary_tours_and_support_exports/)
- Producer: `aureline_docs::current_stable_derived_explanation_citation_export`
- Headless emitter: `aureline_docs_derived_explanation_citation_sets`

## The surfaces

`DerivedExplanationSurface` enumerates exactly the surfaces that publish derived
explanations and must each attach a citation set:

| Surface | Meaning |
| --- | --- |
| `docs_browser_explanation` | A docs-browser peek/hover/explainer prose block. |
| `ai_answer` | An AI answer rendered in the assistant surface. |
| `glossary_card` | A glossary card surfaced in learning/onboarding packs. |
| `guided_tour_step` | A guided-tour step. |
| `architecture_explainer` | An architecture / topology explainer card. |
| `support_export_note` | A support-export note carried into a redacted support packet. |

Every surface in `DerivedExplanationSurface::REQUIRED` must carry at least one
citation set **and** one consumer projection in a stable packet.

## The citation set

A `DerivedExplanationCitationSet` binds one explanation to its evidence basis:

- **Identity** — `citation_set_id` and `explanation_id` give the set a stable,
  export-safe identity; the explanation is bound to the set and never outlives
  it.
- **Cited evidence** — `cited_files` (path ref + content-digest ref, never a raw
  body), `cited_symbols` (symbol + graph-node ref), and `cited_docs` (docs-node
  ref with its own source class, version match, freshness, locale, and trust
  class).
- **Graph epoch** — `graph_epoch` names the code-graph epoch, workspace
  revision, and capture time the explanation was produced against.
- **Derivation** — `derivation` records the tool ref, tool version, and optional
  model ref. It never carries prompt text or raw provider payloads.
- **Vocabularies** — `source_class`, `trust_class`, `freshness`, and `locale`
  reuse the canonical docs-contracts matrix vocabularies frozen by
  [`freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix`](freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md)
  rather than minting parallel tokens.

## Direct citation vs labeled inference

`CitationBasis` is the heart of the contract:

- `direct_citation` — the explanation cites at least one concrete file, symbol,
  or docs node, and carries no inference label. Its trust class is never
  `derived_inference_only`.
- `labeled_inference` — no direct citation exists, so the explanation is an
  explicitly labeled inference. It carries an `InferenceLabel` (a `reason`, an
  `inferred_from_summary`, and a `confidence`), cites no direct evidence, and
  **never claims primary authority** — its `source_class` is `derived_explanation`
  and its `trust_class` is `derived_inference_only`.

This is the guardrail: citations or explicit inference labels are mandatory.
Prose may never outrun its evidence basis.

## Redaction preserves the basis

`CitationRedactionState` lets support and export flows withhold content without
losing the citation basis:

- `content_inline_preserved` — cited refs are carried with no content withheld.
- `content_redacted_basis_preserved` — cited content is redacted, but the
  citation refs, graph epoch, and derivation stay.
- `content_omitted_basis_preserved` — cited content is omitted entirely so the
  source corpus is not forced into every export, but the basis stays.

When content is withheld the set must still name its basis (cited refs or an
inference label) plus a well-formed graph epoch and derivation. A redaction that
strips the basis blocks the stable claim.

## Consumer projections

`CitationConsumerProjection` records that every surface reuses the *same*
citation object instead of inventing prose-only private explanation state. Each
projection names the surface, the packet it belongs to, the citation sets it
reuses, and three preservation flags (`reuses_shared_citation_object`,
`preserves_inference_label`, `preserves_citation_basis_on_export`). The
support-export projection must reference **every** citation set, so an export
never silently drops a derived explanation's evidence basis.

## Promotion and validation

`DerivedExplanationCitationPacket::materialize` computes the validation findings
and the promotion state from the input:

- `stable` — all invariants hold.
- `narrowed_below_stable` — a non-fatal narrowing applies: a direct citation
  rests on `stale`/`unverified` freshness, or a labeled inference is flagged
  `speculative`.
- `blocks_stable` — a blocking invariant failed: prose claiming a direct citation
  cites nothing, an inference hides behind authoritative trust, a redaction
  strips the basis, a required surface lacks a set or projection, or the support
  export drops a citation set.

The packet is metadata-only: it carries no raw document bodies, raw source files,
raw URLs, raw provider payloads, prompt text, or credentials.

## Consumers

AI, onboarding, glossary, docs-browser, support-export, and extension surfaces
consume this packet directly. They project the shared citation object rather than
re-deriving a private explanation state, so any claimed M5 derived explanation
can be traced back to one citation set naming the files, symbols, docs refs,
graph epoch, locale, and derivation tool/version it depended on.
