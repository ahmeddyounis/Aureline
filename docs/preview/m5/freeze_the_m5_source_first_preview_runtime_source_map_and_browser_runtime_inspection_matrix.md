# M5 source-first preview, preview-runtime, source-map, and browser-runtime inspection matrix

This document is the contract for the source-first preview, visual-surface
mapping, and browser-runtime inspection freeze. It binds **every claimed M5
preview/runtime surface** to a single bounded qualification matrix, so Milestone 5
can ship this depth area with canonical implementation, proof, downgrade behavior,
and operator-facing truth instead of ad hoc prototypes, side spreadsheets, or
feature copy that outruns evidence.

The matrix is canonical: no product, docs/help, diagnostics, extension/provider
conformance, or release-control surface may present a greener claim than this
matrix, and any row that cannot identify its source-sync state, target kind, or
mapping-quality class auto-narrows before it publishes.

Source remains canonical; the preview is derivative. Runtime-backed inspection
never masquerades as saved source state. Exact versus approximate versus
inspect-only versus source-only-fallback mapping stays visible. High-impact visual
edits preview the real source diff before commit.

## Source of truth

- Packet type: `PreviewInspectionMatrixPacket`
  (`crates/aureline-preview/src/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/`).
- Boundary schema:
  `schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json`.
- Checked support export:
  `artifacts/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md`.
- Protected fixtures:
  `fixtures/preview/m5/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_preview_inspection_matrix [support|summary]`.

## Claimed surfaces

Each claimed preview/runtime surface carries one matrix row:

`source_first_framework_preview`, `visual_surface_mapping`,
`browser_runtime_inspection`, `device_or_simulator_preview`,
`full_stack_preview_loop`, `embedded_webview_preview`, `visual_edit_transform`,
and `support_export_projection`.

## Frozen vocabulary

Each row reuses the frozen per-view vocabularies rather than minting synonyms, and
adds the matrix-level dimensions this freeze owns:

| Dimension | Vocabulary | Source |
| --- | --- | --- |
| Target kind | `PreviewTargetClass` | `preview_target_descriptor` (reused) |
| Mapping quality | `SourceMappingQualityClass` | `source_mapping` descriptor (reused) |
| Preview session | `PreviewSessionClass` | this matrix |
| Source sync | `SourceSyncClass` | this matrix |
| Attach depth | `AttachDepthClass` | this matrix |
| Round-trip capability | `RoundTripCapabilityClass` | this matrix |
| Qualification | `PreviewMatrixQualificationClass` | this matrix |
| Downgrade trigger | `PreviewMatrixDowngradeTrigger` | this matrix |

- **Source sync** keeps the canonical-vs-derivative relationship honest:
  `in_sync_from_source`, `pending_rebuild`, `drifted_from_source`,
  `runtime_only_no_source`, or `unidentified_source_sync`.
- **Attach depth** keeps a shallow browser attach from claiming full inspection by
  silence: `no_attach`, `dom_only`, `dom_and_styles`, `dom_styles_network`,
  `dom_styles_network_storage`, or `not_applicable_non_browser`.
- **Round-trip capability** keeps a visual action's honesty visible:
  `exact_source_round_trip`, `approximate_source_round_trip`,
  `inspect_only_no_write`, `source_only_fallback`, or `no_round_trip`.

## Auto-narrowing gate

A claimed row may not outrun identified evidence:

- If the row identifies its source-sync state, target kind, and mapping-quality
  class, the `effective_qualification` equals the `claimed_qualification`.
- If any of those three is unidentified (an `unidentified_source_sync` class, or a
  missing `target_kind` / `mapping_quality`), the `effective_qualification` must
  rank strictly below the claim, the row records a `narrow_trigger`, and it carries
  a precise, non-generic `degraded_label`.

`PreviewInspectionMatrixPacket::validate` rejects a matrix that:

- omits a required preview/runtime surface, or demonstrates no auto-narrowing case;
- keeps a public claim while an identity dimension is unidentified without
  narrowing the row below its claim, or narrows without a precise label/trigger;
- lets a `runtime_only_no_source` view claim to be saved source state;
- auto-upgrades an inspect-only or no-round-trip row into a write-capable flow;
- lets a write-capable visual edit skip previewing the real source diff;
- declares an attach depth inconsistent with the identified target kind, or claims
  an exact round-trip without an exact mapping quality;
- fails any guardrail, consumer-projection, or evidence-freshness invariant; or
- carries raw boundary material in the export.

## Guardrails

- Source remains canonical; the preview is derivative — never a second writable
  truth model.
- Runtime-backed inspection never masquerades as saved source state.
- Mapping uncertainty is never hidden by runtime or extension-private wording.
- Inspect-only rows are never auto-upgraded into write-capable designer flows.
- High-impact visual edits preview the real source diff before commit.
- Embedded preview/browser boundaries are not blurred into product authority.
- Any row lacking an identified source-sync, target kind, or mapping-quality class
  auto-narrows below its claim.

## Consumers

Product, docs/help, diagnostics, extension/provider conformance, and
release-control surfaces ingest this matrix directly instead of cloning
preview/runtime terminology by hand, and they label narrowed rows below current in
every surface.
