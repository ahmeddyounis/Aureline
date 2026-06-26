# Stable Docs Source/Result Object Reuse Fixtures

Each fixture is a case file with a `record_kind` of `docs_source_result_reuse_case`,
a `scenario` describing what the case proves, a packet `input`, and an `expect`
block naming the derived promotion state and the finding kinds the validator must
raise. The integration test materializes each `input` and asserts the promotion
state and expected findings, so these fixtures pin the guardrails the canonical
support export keeps green.

## baseline_stable.json

The baseline packet certifies `stable`: one canonical docs-source descriptor and
docs-result object per source class are reused across docs search, symbol-linked
reference cards, hover/peek docs, AI citations, glossary cards, and support exports
without re-deriving source, version, or freshness truth.

## project_docs_relabeled_as_vendor_blocks_stable.json

Project documentation is relabeled with a live-provider trust class. The validator
raises `source_trust_class_mismatch` and blocks stable because project docs must
never masquerade as vendor docs on any surface.

## derived_explanation_claims_precedence_blocks_stable.json

A derived explanation claims source precedence over primary docs. The validator
raises `derived_explanation_masquerades_as_primary` and blocks stable because
derived explanations are never primary documentation authority.

## live_external_inlined_without_handoff_blocks_stable.json

Live external docs are treated as a local cache instead of requiring an explicit
browser handoff. The validator raises `live_external_docs_handoff_missing` and
blocks stable because live external docs must resolve only through an explicit,
isolated handoff.

## result_freshness_drift_blocks_stable.json

A result silently changes freshness from its source descriptor. The validator
raises `source_result_truth_mismatch` and blocks stable because every surface must
read one source/version/freshness truth for the same object.

## consumer_projection_drops_truth_blocks_stable.json

A consumer-surface projection stops showing the source class. The validator raises
`consumer_surface_projection_drift` and blocks stable because every surface must
keep source class, version match, freshness, trust class, and symbol/citation
linkage visible.
