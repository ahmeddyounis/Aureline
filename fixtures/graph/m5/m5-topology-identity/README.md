# Fixtures: M5 topology-identity packet

This directory contains fixture metadata for the `m5_topology_identity_packet`.

The canonical packet is checked in at:

`artifacts/graph/m5/m5-topology-identity.json`

and validated by the typed model in the `aureline-graph` crate (`m5_topology_identity`) and the
JSON Schema at `schemas/graph/m5-topology-identity.schema.json`.

## Coverage

- **Stable node and edge identities.** The packet declares one node of every kind (`file`,
  `directory`, `symbol`, `module`, `doc`, `ownership`, `provider_resource`, `workset_scope`) and
  one edge of every kind (`imports`, `calls`, `contains`, `impacts`, `owned_by`, `references`,
  `depends_on`). Each carries a canonical, stable id, namespace and workspace refs, freshness,
  confidence, a source class, and contract badges that travel with it across views.
- **Explicit relation fidelity.** The seven edges exercise every fidelity — `exact` (twice),
  `approximate`, `imported`, `partial`, `stale`, and `blocked`. Every non-exact edge carries an
  explicit `fidelity_reason`, so presentation never implies stronger certainty than the graph.
- **Export-safe permalinks.** Every node and edge carries a unique `export_permalink` that
  embeds its canonical id, so support, issue reports, review comments, and evidence packets can
  point at the exact object without a screenshot.
- **Shared identity across surfaces.** Each of `map_canvas`, `list`, `table`, `breadcrumb`,
  `explainer`, `review`, and `support_export` carries exactly one binding, stamped with the
  active snapshot and scope. The `table` and `support_export` surfaces resolve every declared
  node and edge, so the canvas owns no identity a non-canvas accessible view cannot resolve.
- **Upstream provenance.** The packet binds to the canonical graph-depth governance matrix
  (`artifacts/graph/m5/m5-graph-governance.json`) and the workset-scope packet
  (`artifacts/graph/m5/m5-workset-scope.json`) it extends.

## Guardrails proven

- A non-exact edge with no `fidelity_reason` fails validation (`UnlabeledNonExactRelation`).
- A node or edge permalink that is empty, does not embed its id, or collides with another object
  fails validation (`UnsafeNodePermalink`, `UnsafeEdgePermalink`, `DuplicatePermalink`).
- An edge endpoint that references an undeclared node fails validation (`DanglingEdgeEndpoint`).
- A surface binding that resolves an undeclared id fails validation (`UnresolvedNodeRef`,
  `UnresolvedEdgeRef`).
- A declared node or edge that no non-canvas surface resolves fails validation
  (`NodeMissingNonCanvasSurface`, `EdgeMissingNonCanvasSurface`, `CanvasOnlyNodeIdentity`,
  `CanvasOnlyEdgeIdentity`).
- A binding not stamped with the active snapshot or scope, or one whose `is_canvas` flag
  disagrees with its surface, fails validation (`SnapshotBindingMismatch`, `ScopeIdMismatch`,
  `SurfaceCanvasFlagMismatch`).
