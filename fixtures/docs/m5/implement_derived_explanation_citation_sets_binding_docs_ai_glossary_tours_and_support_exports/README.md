# Derived-Explanation Citation-Sets Fixtures

Each fixture is a case file with a `record_kind` of
`derived_explanation_citation_sets_case`, a `scenario` describing what the case
proves, a packet `input`, and an `expect` block naming the derived promotion
state and the validation finding kinds the validator must raise. The integration
test materializes each `input` and asserts the promotion state and expected
findings, so these fixtures pin the guardrails the canonical support export keeps
green.

Regenerate any case with the headless emitter, e.g.:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_derived_explanation_citation_sets -- fixture baseline_stable
```

## baseline_stable.json

The baseline packet certifies `stable`: one citation set binds every claimed
surface (docs browser, AI answer, glossary card, guided tour, architecture
explainer, support export), direct citations name cited files/symbols/docs
nodes, the architecture explainer is an explicitly labeled inference, and the
support export reuses every citation set.

## direct_citation_without_evidence_blocks_stable.json

A direct-citation AI answer drops every cited file, symbol, and docs node. The
validator raises `citation_basis_missing` and blocks stable because prose
claiming a direct citation must name the evidence it depended on.

## inference_without_label_blocks_stable.json

A labeled-inference architecture explainer drops its inference label. The
validator raises `inference_label_missing` and blocks stable because an
explanation with no direct citation must explicitly label itself an inference.

## inference_claims_authority_blocks_stable.json

A labeled inference records first-party authoritative trust. The validator raises
`basis_trust_inconsistent` and blocks stable because a derived inference never
claims primary authority.

## redaction_drops_basis_blocks_stable.json

A support-export note omits its content and also drops every citation ref. The
validator raises `redaction_drops_citation_basis` and blocks stable because
redaction may withhold content but must always preserve the citation basis.

## surface_coverage_missing_blocks_stable.json

The glossary card surface loses its citation set. The validator raises
`surface_coverage_missing` and blocks stable because every claimed surface must
attach one citation set.

## support_export_drops_basis_blocks_stable.json

The support-export projection stops referencing one citation set. The validator
raises `support_export_drops_citation_basis` and blocks stable because a support
export must preserve the citation basis of every derived explanation.

## projection_drops_reuse_blocks_stable.json

The AI surface stops reusing the shared citation object. The validator raises
`consumer_projection_drops_reuse` and blocks stable because surfaces must reuse
the same citation object instead of inventing prose-only private state.

## stale_citation_narrows_below_stable.json

A direct citation rests on stale freshness. The validator raises
`citation_freshness_narrowed` and narrows below stable — the basis still exists,
but the explanation must not claim current authority.

## speculative_inference_narrows_below_stable.json

The architecture explainer marks its inference speculative. The validator raises
`speculative_inference_narrowed` and narrows below stable — the inference is
still labeled, but reads as low confidence.
