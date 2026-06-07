# Navigation Target And Hierarchy Contract

Stable navigation surfaces must exchange canonical objects instead of private row shapes or best-guess jumps.

The language platform owns five object families for cross-surface use:

- `NavigationTarget` carries `target_id`, `relation_kind`, `object_ref`, `anchor_ref`, provider/source class, confidence, freshness, ambiguity, scope completeness, fallback mode, and optional disambiguation ref.
- `ReferenceOccurrence` carries `occurrence_id`, target and anchor refs, `access_kind`, scope, provider/source class, confidence, freshness, scope completeness, fallback mode, and body-redaction state.
- `NavigationDisambiguationSet` carries the requested relation, every candidate target ref, a selection policy, created timestamp, ambiguity class, and fallback state.
- `HierarchyEdge` carries source and target refs, edge class, proof class, depth, scope completeness, freshness, confidence, fallback mode, and proof refs for non-direct edges.
- `RenamePreviewSet` carries the root target, candidate occurrence refs, blocked/generated/readonly/sparse/partial/redacted candidate refs, conflict notes, sparse or partial reasons, generated-scope notes, and fallback state.

Definition, declaration, implementation, reference, type, call, route-binding, owner-link, and doc-link are distinct `relation_kind` values. `Go to Definition`, `Go to Declaration`, `Go to Implementation`, and `Find References` must not alias one another silently; unsupported or downgraded behavior must use a non-`none` fallback mode with a support-safe reason.

References and rename candidates preserve the closed `access_kind` vocabulary: `read`, `write`, `call`, `inherit`, `import`, `export`, `route-binding`, `test-only`, `generated`, and `runtime-observed`.

Hierarchy views preserve `direct`, `transitive`, `inferred`, `framework_generated`, and `runtime_observed` edge classes. Runtime-observed and framework-generated edges can enrich navigation but cannot replace direct static proof.

Rename preview is governed by candidate identities. Blocked, generated, readonly, sparse-scope, partially loaded, and redacted candidates remain visible even when source bodies are hidden.

The boundary schema is `schemas/search/navigation-targets.schema.json`; the fixture corpus is `fixtures/search/m4/navigation-target-and-hierarchy-contract/`.
