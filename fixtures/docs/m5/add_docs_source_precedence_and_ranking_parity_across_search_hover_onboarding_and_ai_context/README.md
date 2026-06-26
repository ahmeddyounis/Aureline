# Docs-Source Precedence and Ranking Parity Fixtures

Each fixture is a case file with a `record_kind` of
`docs_source_precedence_and_ranking_parity_case`, a `scenario` describing what
the case proves, a packet `input`, and an `expect` block naming the derived
promotion state and the validation finding kinds the validator must raise. The
integration test materializes each `input` and asserts the promotion state and
expected findings, so these fixtures pin the guardrails the canonical support
export keeps green.

Regenerate any case with the headless emitter, e.g.:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_source_precedence_ranking -- fixture baseline_stable
```

## baseline_stable.json

The baseline packet certifies `stable`: two ranking sets cover all seven
distinguishable source lanes. For the repo-specific question project docs outrank
the mirrored and live-external alternatives, but both stay visible and are
referenced; every candidate carries a precedence reason and note; the derived
explanation never ranks primary; and docs search, hover/peek, onboarding, AI
context, and support export project the same ranking explanation with one shared
vocabulary.

## source_lane_flattened_blocks_stable.json

The derived-explanation lane is dropped, flattening the source set. The validator
raises `source_class_distinguishability_missing` and blocks stable because the
seven source lanes must each stay distinguishable.

## project_masquerades_as_vendor_blocks_stable.json

A project-docs candidate is labelled with a live-provider trust class so it would
masquerade as vendor docs. The validator raises `candidate_lane_unresolved` and
blocks stable because that source/trust pair resolves to no distinguishable lane.

## unexplained_rank_inversion_blocks_stable.json

Project docs keep the top rank above the more-authoritative mirrored docs but no
longer carry a reason that justifies outranking them. The validator raises
`unexplained_rank_inversion` and blocks stable because a less-authoritative
source may outrank a more-authoritative one only with an explicit justifying
reason.

## outrank_without_visible_alternative_blocks_stable.json

Project docs claim to outrank vendor docs but the mirrored and live-external
alternatives are removed from the set. The validator raises
`outrank_without_visible_alternative` and blocks stable because project docs may
outrank vendor docs only while keeping the vendor difference visible.

## derived_ranked_as_primary_blocks_stable.json

A derived explanation is promoted to rank 1. The validator raises
`derived_explanation_ranked_as_primary` and blocks stable because a derived
explanation never claims primary authority.

## reason_class_mismatch_blocks_stable.json

A candidate claims a vendor-override reason while declaring a non-override
precedence class. The validator raises `precedence_reason_class_mismatch` and
blocks stable because the precedence reason must stay consistent with the
precedence class.

## hidden_ranking_model_blocks_stable.json

The AI-context surface stops reusing the shared ranking vocabulary and mints a
hidden ranking model. The validator raises `hidden_ranking_model` and blocks
stable because no surface may run a second ranking model that ignores
source-class, version-match, or freshness truth.

## offline_unavailable_reason_missing_blocks_stable.json

A candidate is unavailable in an offline profile but gives no reason. The
validator raises `offline_unavailable_reason_missing` and blocks stable because
an offline / air-gapped profile must keep candidates inspectable with an explicit
unavailable reason rather than silently dropping them.

## missing_rank_explanation_surface_blocks_stable.json

The onboarding surface loses its ranking-explanation projection. The validator
raises `missing_rank_explanation_surface` and blocks stable because the ranking
explanation must stay inspectable across docs search, hover/peek, onboarding, AI
context, and support export.

## air_gapped_candidate_narrows_below_stable.json

A candidate is unavailable in an air-gapped profile but honestly discloses why.
The packet narrows below stable rather than blocking (via
`air_gapped_candidate_narrowed`), because offline inspectability with an explicit
unavailable reason is degraded but honest.
