# Docs-browser source/result truth — contract

The docs-browser truth packet is the stable knowledge-plane contract that
certifies docs-browser source descriptors, docs-result objects, machine-readable
version-match / source-class / freshness enums, and symbol-linked docs flows.
Its checked-in canonical form lives at
`artifacts/search/m4/docs_browser_truth_packet.json`; the implementation lives
at `crates/aureline-docs/src/docs_browser_truth_packet/mod.rs`.

## What the packet certifies

The packet certifies that every claimed stable docs-browser row:

- **Pins a [`DocsBrowserSourceDescriptor`].** Each descriptor carries
  `source_class`, `provider_or_pack_id`, `provider_or_pack_revision_ref`,
  `locale`, `trust_class`, and `browser_handoff` capability. The closed
  `DocsBrowserSourceClass` taxonomy covers `project_docs`,
  `mirrored_official_docs`, `extension_docs_pack`, `live_external_docs`,
  `curated_knowledge_pack`, `generated_reference`, and `derived_explanation`.
- **Pins a [`DocsBrowserResultObject`].** Each result carries `result_id`,
  `title`, `docs_source_ref`, machine-readable `version_match_state` and
  `freshness_state`, a `symbol_link_class`, optional `symbol_refs`, citation
  anchors, snippet anchor, and a downgrade note when the row lowers certainty.
- **Reuses the closed enums on every consumer surface.** The same
  `source_class`, `version_match_state`, and `freshness_state` tokens are
  consumed verbatim by the docs-browser shell, peek overlay, split pane,
  browser-handoff packet, AI context inspector, extension API, onboarding tour,
  support export, and the release proof index. Surface-local prose is forbidden.
- **Preserves the docs-result object identity through every symbol-linked
  flow.** Each [`DocsBrowserSymbolFlow`] records that the docs-result identity
  was preserved through peek, split, browser handoff, support export, and AI
  handoff. A flow that loses identity at any step blocks promotion with
  `symbol_flow_identity_lost`.

## Consumer surfaces covered

| Surface | Why it ships |
|---|---|
| `docs_browser_shell` | In-product docs-browser row, table, and detail. |
| `peek_overlay` | Symbol-peek overlay opened from source. |
| `split_pane` | Split-pane companion view next to source. |
| `browser_handoff_packet` | Open-in-browser packet for live external docs. |
| `ai_context_inspector` | AI context inspector / context-picker projection. |
| `extension_api` | Extension API consumer that reads docs-browser truth. |
| `onboarding_tour` | Onboarding tour and learning projection. |
| `support_export` | Support export bundle. |
| `release_proof_index` | Release proof index entry. |

A consumer projection that drops the source-class taxonomy, the version-match
vocabulary, the freshness vocabulary, the docs-result object identity, or the
symbol-flow identity is auto-narrowed below stable.

## Closed finding vocabulary

When the packet fails an invariant the validator emits one or more of:
`wrong_record_kind`, `wrong_schema_version`, `missing_packet_identity`,
`missing_source_descriptors`, `missing_result_objects`,
`source_descriptor_incomplete`, `result_source_ref_unpinned`,
`result_identity_incomplete`, `result_disclosure_missing`, `symbol_refs_missing`,
`symbol_flow_identity_lost`, `symbol_flow_result_ref_unpinned`,
`browser_handoff_packet_missing`, `missing_consumer_projection`,
`consumer_projection_drift`, `source_class_taxonomy_dropped`,
`version_match_vocabulary_dropped`, `freshness_vocabulary_dropped`,
`symbol_flow_identity_dropped`, `raw_boundary_material_present`,
`required_source_class_coverage_missing`, `derived_explanation_not_downgraded`,
`promotion_state_mismatch`.

The fixture corpus drills the most likely failure modes:

- `missing_required_source_class_blocks_stable.json` — the packet drops the
  extension-docs-pack source descriptor.
- `symbol_flow_drops_split_step_blocks_stable.json` — a symbol-linked flow drops
  the split step from its preserved-identity list.
- `result_source_ref_unpinned_blocks_stable.json` — a docs-result row points at
  a source id that no descriptor declared.
- `consumer_projection_drops_source_class_blocks_stable.json` — the support
  export projection sets `preserves_source_class = false`.
- `live_external_handoff_missing_packet_blocks_stable.json` — a live external
  docs descriptor declares `available_explicit` handoff but drops the packet
  ref.

## How to regenerate the packet

The seed lives in code; the artifact and fixtures are regenerated with the
crate's headless emitter:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- packet \
  > artifacts/search/m4/docs_browser_truth_packet.json
cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- fixture baseline_stable \
  > fixtures/search/m4/docs_browser_truth_packet/baseline_stable.json
```

The replay test
`crates/aureline-docs/tests/docs_browser_truth_packet.rs` ingests the
artifact and every fixture; CI fails if the emitter, the artifact, or a fixture
drifts.
