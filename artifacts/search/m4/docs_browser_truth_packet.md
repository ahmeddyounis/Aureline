# Docs-browser source/result truth — stable artifact

This is the human-readable narrative for the stable docs-browser truth lane
that certifies docs-browser source descriptors, docs-result objects,
machine-readable version-match / source-class / freshness enums, and
symbol-linked docs flows. The canonical truth source is the checked-in packet
at `artifacts/search/m4/docs_browser_truth_packet.json`; later dashboards,
docs, Help/About surfaces, and support exports should ingest that file instead
of cloning status text.

## What the artifact certifies

The artifact certifies that every claimed stable docs-browser row:

- **Pins a docs-source descriptor.** Source class, provider or pack id,
  provider or pack revision ref, locale, trust class, and browser-handoff
  capability are present on every row, and the closed source-class taxonomy
  covers `project_docs`, `mirrored_official_docs`, `extension_docs_pack`,
  `live_external_docs`, and `derived_explanation` on the same stable build.
- **Pins a docs-result object.** Every row carries a stable `result_id`,
  `title`, `docs_source_ref`, machine-readable `version_match_state` and
  `freshness_state`, a `symbol_link_class`, optional `symbol_refs`, citation
  anchors, and a downgrade note when the row lowers certainty.
- **Reuses the closed enums on every consumer surface.** Source-class,
  version-match, and freshness tokens are read verbatim by the docs-browser
  shell, AI context inspector, onboarding tour, support export, and the
  extension API. Surface-local prose is forbidden.
- **Preserves docs-result identity through symbol-linked flows.** Each
  symbol-linked flow records that the docs-result object identity was preserved
  through peek, split, browser handoff, support export, and AI handoff
  fixtures, rather than regenerating anonymous rows at each step.

## Consumer surfaces covered

The checked-in canonical packet binds projections to every required consumer
surface:

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

A projection that drops the source-class taxonomy, the version-match
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
