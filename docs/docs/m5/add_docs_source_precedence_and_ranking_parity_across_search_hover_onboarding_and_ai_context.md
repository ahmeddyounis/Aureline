# Docs-Source Precedence and Ranking Parity

This document is the contract for docs-source precedence and ranking parity:
the rules that let a reader tell *why* one documentation answer outranked
another and *what authority* the winning answer carries, and the proof that the
same ranking explanation is reused across docs search, hover/peek, onboarding,
AI context, and support export. The packet ranks over the canonical docs-source
descriptors and docs-result objects rather than re-deriving source, version, or
freshness truth ad hoc.

- Record kind: `docs_source_precedence_and_ranking_parity_packet`
- Schema: [`schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json`](../../../schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json)
- Canonical support export: [`artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/support_export.json`](../../../artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/support_export.json)
- Summary artifact: [`artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md`](../../../artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md)
- Fixtures: [`fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/`](../../../fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/)
- Producer: `aureline_docs::current_stable_docs_precedence_ranking_export`

## The seven distinguishable source lanes

Precedence ranking must never flatten the documentation source classes into one
list. A `DocsSourceLane` is derived from a candidate's source class and trust
class:

| Lane | Source class + trust class |
| --- | --- |
| `project_docs` | project docs, first-party authoritative |
| `generated_docs` | generated reference, first-party authoritative |
| `mirrored_official_docs` | mirrored official docs, signed mirror verified |
| `curated_knowledge_pack` | curated knowledge pack, curated/supported |
| `extension_contributed_docs` | curated knowledge pack, extension-pack signed |
| `live_external_docs` | vendor/provider docs, live provider handoff |
| `derived_explanation` | derived explanation, derived inference only |

The curated-knowledge-pack source class splits into two lanes by trust class,
which is the only way extension-contributed docs stay distinguishable from a
first-party curated pack. A source/trust pair that does not resolve to a lane
(for example project docs labelled with a live-provider trust class) has no
distinguishable lane and blocks promotion — that is how project docs are kept
from masquerading as vendor docs.

## The ranked candidate

A `RankedDocsCandidate` is one answer in a ranking set. Every candidate carries:

| Field | Meaning |
| --- | --- |
| `candidate_id` / `docs_source_ref` / `result_ref` | Stable id and refs to the docs-source descriptor and docs-result object it ranks. |
| `source_class` / `trust_class` / `lane` | The canonical class, trust class, and the distinguishable lane they resolve to. |
| `precedence_class` | The project/vendor `SourcePrecedenceClass` for this candidate. |
| `precedence_reason` / `precedence_reason_note` | A closed reason for the rank, plus a human-readable note. |
| `version_match_state` / `freshness_state` | Version-match and freshness truth. |
| `mirror_offline_posture` | Live-online, local project pack, generated-local, mirrored, offline-pinned, cached-local, not-installed, or support pack. |
| `rank_position` | 1-based rank within the set. |
| `project_specific_cue` / `override_cue` | The override / project-specific cues a surface shows. |
| `outranks_refs` | Candidate ids this candidate outranks (used to keep the vendor difference visible). |
| `available_in_offline_profile` / `unavailable_reason` | Whether the candidate stays inspectable offline, and the explicit reason when it does not. |
| `disclosure_note` | Required for a derived, drifted, stale, override, disagreement, or unavailable candidate. |

## Precedence reasons

`PrecedenceReason` is the closed vocabulary for *why* a candidate carries its
rank: `project_scope_match`, `exact_version_match`, `freshness_preferred`,
`official_upstream_authority`, `curated_pack_relevance`,
`extension_contributed_scope`, `live_external_fallback`,
`vendor_override_policy`, `disagreement_both_shown`, and
`derived_inference_only`. Each reason is admissible only for certain precedence
classes, and `project_scope_match`, `exact_version_match`,
`freshness_preferred`, `vendor_override_policy`, and `disagreement_both_shown`
are the reasons that can justify ranking a less-authoritative source above a
more-authoritative one.

## Ranking invariants

`DocsPrecedenceRankingPacket::materialize` derives the validation findings and a
promotion state of `stable`, `narrowed_below_stable`, or `blocks_stable`.

Blocking invariants:

- The seven source lanes must each stay distinguishable.
- A candidate's declared lane must agree with its source class and trust class,
  and the pair must resolve to a lane.
- A candidate must explain its rank with a non-empty note, and its precedence
  reason must stay admissible for its precedence class.
- A less-authoritative source may rank above a more-authoritative one only with
  a justifying precedence reason.
- A project-outranks-vendor candidate must keep at least one vendor / mirrored
  alternative visible in the same set and reference it.
- A derived explanation may not rank first or claim a precedence class.
- A required disclosure note must be present.
- An offline-unavailable candidate must carry an explicit unavailable reason.
- Docs search, hover/peek, onboarding, AI context, and support export must each
  project the ranking explanation, keep every shared-explanation field visible,
  stay inspectable on demand, and reuse the shared ranking vocabulary instead of
  a hidden ranking model.
- The support-export projection must reconstruct every ranking set.

Narrowing (valid but degraded) states:

- A candidate honestly disclosed as unavailable in an offline / air-gapped
  profile narrows below stable.
- A surfaced project/vendor disagreement (both answers shown) narrows below
  stable.

## Acceptance criteria mapping

- *Project docs may outrank vendor docs for repo-specific questions, but the
  product always keeps the source difference visible* — the seed's repo-specific
  ranking set ranks project docs first with a `project_scope_match` reason and a
  `project_outranks_vendor_default` precedence class, while the mirrored and
  live-external alternatives stay in the set and are referenced; dropping them
  raises `outrank_without_visible_alternative`.
- *Docs ranking explanations remain inspectable on demand across docs browser,
  hover/peek, onboarding, and AI context surfaces using one stable vocabulary* —
  the per-surface projections keep one shared vocabulary; a surface that mints a
  hidden ranking model raises `hidden_ranking_model`.
- *Offline and air-gapped profiles preserve docs-pack inspectability and explicit
  unavailable reasons rather than pretending generic web search is equivalent* —
  an offline-unavailable candidate without a reason blocks; with a reason it
  narrows below stable and stays inspectable.

## Boundary safety

The packet is a metadata-only truth packet. It carries no raw document bodies,
raw source files, raw URLs, raw provider payloads, or credentials — only
metadata, opaque refs, the controlled precedence / lane / reason vocabulary, and
contract refs. The support export preserves the exact packet without exporting
full content or flattening the ranking explanation.
