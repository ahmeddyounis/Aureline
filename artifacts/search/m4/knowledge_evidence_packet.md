# Knowledge-plane evidence — stable artifact

This is the human-readable narrative for the stable knowledge-plane
lane that finalises topology maps, impact explorer cards, ownership
cards, and architecture explainer snapshots with citation-ready
evidence. The canonical truth source is the checked-in evidence
packet at `artifacts/search/m4/knowledge_evidence_packet.json`;
later dashboards, docs, Help/About surfaces, and support exports
should ingest that file instead of cloning status text.

## What the artifact certifies

The artifact certifies that every claimed stable knowledge-plane
row:

- **Shares one identity model.** Topology canvases, list/table
  fallbacks, breadcrumbs, AI inspectors, onboarding tours, and review
  explainer cards reference the same `node_ids`, `edge_ids`,
  freshness vocabulary, confidence vocabulary, and out-of-scope count
  model.
- **Is honest about no-impact vs. outside-slice.** Each impact card
  carries one of `visible_impact_present`,
  `no_impact_in_workspace_or_slice`, or
  `impact_outside_slice_disclosed`. Cards with outside-slice impact
  pin the exact `widen_action_ref`. The packet never lets an
  outside-slice card masquerade as `no_impact_found`.
- **Distinguishes ownership classes.** Ownership cards split into
  `curated_first_party`, `policy_derived`,
  `imported_provider_metadata`, and `heuristic_inferred`. Non-curated
  ownership carries a partiality note explaining how the owner was
  derived.
- **Ships citation-ready explainers.** Every explainer snapshot pins
  a `source_class` (`curated` or `generated`), at least one citation
  or an explicit `inference_label` when generated, freshness,
  confidence, and at least one evidence-opening action.

## Consumer surfaces covered

The checked-in canonical packet binds projections to every required
consumer surface:

| Surface | Why it ships |
|---|---|
| `topology_canvas` | Topology map view. |
| `list_table_fallback` | Non-canvas list / table for keyboard and screen-reader users. |
| `breadcrumb` | Parent / ancestor breadcrumb projection. |
| `ai_inspector` | AI inspector / context picker. |
| `onboarding_tour` | Onboarding tour projection. |
| `review_explainer_card` | Review or PR-style explainer card. |
| `cli_headless` | Headless CLI emitter for knowledge-plane queries. |
| `support_export` | Support export bundle. |
| `release_proof_index` | Release proof index entry. |

A projection that drops the node/edge identity binding, the
freshness vocabulary, the confidence vocabulary, the out-of-scope
count model, the impact widen action, the ownership class, or the
explainer citations is auto-narrowed below stable.

## Closed finding vocabulary

When the packet fails an invariant the validator emits one or more
of: `wrong_record_kind`, `wrong_schema_version`,
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

The fixture corpus drills the most likely failure modes (impact card
collapsing outside-slice impact, heuristic ownership without a
partiality note, generated explainer without citations or an
inference label, consumer projection dropping the node/edge identity
binding).

## Hard-dependency narrowing

A row that fails any of the v24 invariants above is auto-narrowed
below stable. The canonical packet refuses to publish a stable claim
until every card, snapshot, and projection binds to one of the closed
tokens above.

## How to read the artifact

The canonical packet at
`artifacts/search/m4/knowledge_evidence_packet.json` is parsed by
`current_stable_knowledge_evidence_packet()` in
`crates/aureline-graph/src/knowledge_evidence_packet/mod.rs`, which
re-validates the file at module load time. Consumer projections cite
the canonical packet by id and preserve every closed token verbatim.

## Verification

```
cargo test -p aureline-graph --lib knowledge_evidence_packet
cargo test -p aureline-graph --test knowledge_evidence_packet
```
