# Framework-certainty and source-sync chip cases

Worked fixtures for the framework-certainty and source-sync chip
contract frozen in
[`/docs/framework/framework_certainty_and_source_sync_contract.md`](../../../docs/framework/framework_certainty_and_source_sync_contract.md).
Every fixture conforms to one of the two schemas:

- [`/schemas/framework/framework_certainty_row.schema.json`](../../../schemas/framework/framework_certainty_row.schema.json)
- [`/schemas/framework/source_sync_chip.schema.json`](../../../schemas/framework/source_sync_chip.schema.json)

The fixtures collectively exercise:

- a fully-proven framework route on the route explorer surface backed
  by a framework-pack analysis chain;
- an inferred component-tree node from project-graph and syntax
  evidence only;
- a stale preview source-synchronization row whose framework pack ran
  against an older revision;
- a notebook-bound framework-context row carrying a notebook-binding
  block with `kernel_runtime_not_consulted`;
- a no-framework-certainty fallback / inference reason row blocking
  any framework-specific claim when only generic language and build
  evidence exists;
- the in-sync source-sync chip composed with the proven route;
- the drifted source-sync chip composed with the stale preview
  mapping;
- the runtime-unattached source-sync chip composed with the no-
  framework-certainty row.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against the
  schema referenced in its `# yaml-language-server` header.
- **Vocabulary parity.** Fixtures use the same framework-certainty,
  framework-primary-source, framework-adapter, certainty-source,
  source-sync, hot-reload, and target-device vocabularies as the
  language provider-graph contract and the preview-runtime contract.
  They do not mint fixture-local labels.
- **Composition.** Fixtures cite preview-snapshot, preview-runtime
  strip, hot-reload state, result-provenance, capability-negotiation,
  notebook, and execution-context records by opaque ref only; raw
  bodies do not appear.

## Fixtures

- [`fully_proven_framework_route.yaml`](./fully_proven_framework_route.yaml)
  — `framework_proven` `route_mapping_subject` row backed by framework-
  pack analysis with `route_mapped_to_handler_proven`.
- [`inferred_component_tree_partial_sources.yaml`](./inferred_component_tree_partial_sources.yaml)
  — `framework_inferred` `component_tree_node_subject` row with
  `component_node_inferred_from_graph` and a `project_graph_only_evidence`
  fallback reason.
- [`stale_preview_mapping.yaml`](./stale_preview_mapping.yaml)
  — `framework_stale` `source_synchronization_subject` row paired with
  the drifted source-sync chip; `proving_artifact_stale` fallback
  reason.
- [`notebook_bound_framework_context.yaml`](./notebook_bound_framework_context.yaml)
  — `framework_inferred` `detected_framework_subject` row on the
  notebook framework-context surface with a notebook-binding block
  citing `kernel_runtime_not_consulted`.
- [`no_framework_certainty.yaml`](./no_framework_certainty.yaml)
  — `framework_unavailable` `fallback_inference_reason_subject` row on
  the diagnostics lane citing `generic_language_signal_only`.
- [`source_sync_chip_in_sync.yaml`](./source_sync_chip_in_sync.yaml)
  — `source_in_sync` chip composed with the proven route fixture.
- [`source_sync_chip_drifted.yaml`](./source_sync_chip_drifted.yaml)
  — `source_drifted_since_render` chip composed with the stale
  preview-mapping fixture; `restart_required` hot-reload posture.
- [`source_sync_chip_runtime_unattached.yaml`](./source_sync_chip_runtime_unattached.yaml)
  — `no_preview_lane_attached` chip composed with the no-framework-
  certainty fixture; demonstrates that a diagnostics surface still
  emits at least one fallback-open action.

## Related artifacts

- [`/schemas/language/result_provenance.schema.json`](../../../schemas/language/result_provenance.schema.json)
  — language result-provenance and framework-certainty vocabulary
  re-exported by the certainty row.
- [`/schemas/preview/preview_snapshot.schema.json`](../../../schemas/preview/preview_snapshot.schema.json)
  — preview-snapshot record the source-sync chip projects from.
- [`/schemas/preview/preview_runtime_strip.schema.json`](../../../schemas/preview/preview_runtime_strip.schema.json)
  — preview-runtime strip record the chip composes with.
- [`/schemas/preview/hot_reload_state.schema.json`](../../../schemas/preview/hot_reload_state.schema.json)
  — hot-reload state record the chip composes with.
