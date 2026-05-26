# Knowledge-plane evidence packet

This is the stable contract every claimed knowledge-plane surface
must speak: topology canvases, list/table fallbacks, breadcrumbs,
AI inspectors, onboarding tours, review explainer cards, the
headless CLI emitter, support exports, and the release proof index.
The runtime owner is the
`aureline_graph::knowledge_evidence_packet` module.

The packet binds four hardened v24 invariants on top of the
[freshness propagation contract](freshness_propagation_packet.md):

1. **Shared identity model.** Every consumer surface reads the same
   node/edge identity space, the same freshness/confidence
   vocabulary, and the same out-of-scope count model. The packet
   pins `identity_model_ref`, the visible `node_ids`/`edge_ids`,
   `freshness_vocabulary_ref`, `confidence_vocabulary_ref`, and
   `out_of_scope_count_model_ref`.
2. **No-impact honesty.** Impact explorer cards may never collapse
   `outside current slice` into `no impact found`. The closed
   `no_impact_state` token (`visible_impact_present`,
   `no_impact_in_workspace_or_slice`, or
   `impact_outside_slice_disclosed`) must match the card's
   `hidden_or_out_of_scope_count`. A card with non-zero hidden or
   out-of-scope impact may not declare
   `no_impact_in_workspace_or_slice`; a card disclosing outside-slice
   impact must pin a non-empty `widen_action_ref`.
3. **Ownership taxonomy.** Ownership cards distinguish
   `curated_first_party`, `policy_derived`,
   `imported_provider_metadata`, and `heuristic_inferred` instead of
   collapsing every owner into one generic role. Non-curated classes
   must pin a `partiality_note` explaining how the owner was derived.
4. **Citation-ready explainers.** Each explainer snapshot pins a
   `source_class` (`curated` or `generated`), a cited refs list, an
   evidence-opening action list, and freshness/confidence tokens from
   the shared vocabulary. Generated prose must either ship with at
   least one citation or pin an explicit `inference_label`.

## Out-of-scope count model

Every surface that paints knowledge-plane truth reads the same count
model:

- `visible_*_count` — items visible inside the active slice.
- `out_of_scope_*_count` (topology) and `hidden_or_out_of_scope_count`
  (impact card) — items hidden by scope, policy, or the loaded slice.
- `widen_scope_action_ref` (topology) and `widen_action_ref` (impact
  card) — the action a user invokes to widen the slice and compute a
  broader answer.

A non-zero out-of-scope count without a widen action fires
`topology_missing_widen_action` or `impact_card_missing_widen_action`.

## Required consumer projections

Every stable knowledge-plane packet preserves these nine consumer
projections:

- `topology_canvas` — topology map view.
- `list_table_fallback` — non-canvas list/table projection for
  keyboard and screen-reader users.
- `breadcrumb` — parent/ancestor breadcrumb projection.
- `ai_inspector` — AI inspector / context picker.
- `onboarding_tour` — onboarding tour projection.
- `review_explainer_card` — review or PR-style explainer card.
- `cli_headless` — headless CLI emitter for knowledge-plane queries.
- `support_export` — support export bundle.
- `release_proof_index` — release proof index entry.

A projection must preserve the same packet id, the node/edge identity
binding, the freshness vocabulary, the confidence vocabulary, the
out-of-scope count model, the impact-card widen action, the ownership
class taxonomy, and the explainer citations and inference label
verbatim. Dropping any of these fires the matching finding kind:
`node_edge_identity_dropped`, `freshness_vocabulary_dropped`,
`confidence_vocabulary_dropped`, `out_of_scope_count_model_dropped`,
or `consumer_projection_drift`. A missing projection fires
`missing_consumer_projection`.

## Closed validation-finding vocabulary

When the packet fails an invariant, the validator emits one or more
of these closed finding kinds:

`wrong_record_kind`, `wrong_schema_version`,
`missing_packet_identity`, `missing_shared_identity_model`,
`missing_topology_view`, `node_edge_identity_mismatch`,
`out_of_scope_count_dropped`, `topology_missing_widen_action`,
`no_impact_collapsed_on_out_of_scope_impact`,
`impact_card_missing_widen_action`, `ownership_class_missing`,
`ownership_partiality_note_missing`, `explainer_source_class_missing`,
`generated_explainer_missing_citations_or_inference`,
`explainer_missing_evidence_opening_actions`,
`missing_consumer_projection`, `consumer_projection_drift`,
`node_edge_identity_dropped`, `freshness_vocabulary_dropped`,
`confidence_vocabulary_dropped`, `out_of_scope_count_model_dropped`,
`raw_boundary_material_present`, `promotion_state_mismatch`.

## Boundary safety

The packet is intentionally metadata-only. The validator emits
`raw_boundary_material_present` if any row admits raw query text, raw
node bodies, secrets, provider payloads, or ambient credentials. The
support-export wrapper preserves the product packet verbatim and is
considered export-safe only when every row, card, snapshot, and
projection passes validation.

## Hard-dependency narrowing

A packet that fails any of the v24 invariants above must auto-narrow
below stable. Promotion state is one of:

- `stable` — no findings.
- `narrowed_below_stable` — warning findings only.
- `blocks_stable` — at least one blocker finding.

The packet stores the derived promotion state inline; the validator
fires `promotion_state_mismatch` if the stored state disagrees with
the derived findings on re-validation.

## Fixture corpus

The fixture corpus at
`fixtures/search/m4/knowledge_evidence_packet/` exercises:

- `baseline_stable.json` — every closed surface, ownership class, and
  no-impact state, with citation-backed and inference-labelled
  explainers.
- `no_impact_collapsed_on_out_of_scope_blocks_stable.json` — an
  impact card that collapses outside-slice impact into
  `no_impact_in_workspace_or_slice`.
- `ownership_partiality_note_missing_blocks_stable.json` — a
  heuristic-inferred ownership card that drops its partiality note.
- `generated_explainer_missing_citations_or_inference_blocks_stable.json`
  — a generated explainer that ships without citations or an
  inference label.
- `consumer_projection_drops_node_edge_identity_blocks_stable.json` —
  a consumer projection that drops the node/edge identity binding.

The checked-in canonical packet at
`artifacts/search/m4/knowledge_evidence_packet.json` covers every
required consumer surface against every closed ownership class
(`curated_first_party`, `policy_derived`,
`imported_provider_metadata`, `heuristic_inferred`) and every closed
no-impact state, and is the stable truth source consumers ingest
verbatim.
