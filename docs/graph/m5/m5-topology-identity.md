# M5 topology-identity packet

This document describes the canonical packet that carries the **M5 topology identities** — the
stable node and edge ids every code-understanding surface points at, so the same graph object
survives canvas, list, table, breadcrumb, review, and support/export views without lossy
translation. Where the [workset-scope packet](m5-workset-scope.md) answers *what slice am I
looking at?* and the [graph-governance matrix](m5-graph-governance.md) freezes *which depth
claim a lane may publish*, this packet answers the question search, review, AI, onboarding,
docs, and support all ask next: **which exact graph object is this, and how do I point at it
again later?**

It is the user-facing companion to the governed artifact at
`artifacts/graph/m5/m5-topology-identity.json` and the typed model in the `aureline-graph`
crate (`m5_topology_identity`).

## What this packet covers

The packet reuses the node and edge identity space rather than letting each view invent its own
transient ids. It carries four things.

### 1. Stable node identities

Each entry in `nodes` is a `TopologyNodeIdentity` with a canonical, stable `node_id`, a `kind`
(`file`, `directory`, `symbol`, `module`, `doc`, `ownership`, `provider_resource`, or
`workset_scope`), a redaction-aware `display_label`, `namespace_ref` and `workspace_ref`, a
`freshness` and `confidence` token, a `source_class`, the `contract_badges` it carries across
views, and an `export_permalink`.

### 2. Stable edge identities

Each entry in `edges` is a `TopologyEdgeIdentity` with a canonical `edge_id`, a `kind`
(`imports`, `calls`, `contains`, `impacts`, `owned_by`, `references`, or `depends_on`), a
`from_node_id` and `to_node_id` that must reference declared nodes, a `relation_fidelity`, a
`freshness`, `confidence`, `source_class`, `contract_badges`, and an `export_permalink`.

The **relation fidelity** is the honesty contract. An edge is `exact` only when direct,
authoritative evidence supports it. Otherwise it is labeled explicitly — `approximate` (heuristic
or inferred), `imported` (hydrated from an imported bundle), `partial` (truncated at the active
workset boundary), `stale` (older than the current revision of an endpoint), or `blocked`
(withheld by policy or a missing connection) — and **must** carry a `fidelity_reason`. A visual
map never implies stronger certainty than the underlying graph carries.

### 3. The active scope anchor

`active_scope` records the `snapshot_id`, `scope_id`, `taken_as_of` date, and `scope_mode` of
the slice this topology was queried under. Every surface binding is stamped with the same
snapshot and scope, so a later support export or replay can reconstruct exactly which topology
the user saw.

### 4. One binding per surface

`surface_bindings` carry the identities into every accessible surface that could otherwise mint
its own transient ids:

1. **`map_canvas`** — the visual map.
2. **`list`** — a flat list.
3. **`table`** — the non-canvas edge/node table.
4. **`breadcrumb`** — the breadcrumb trail.
5. **`explainer`** — the architecture explainer.
6. **`review`** — review explanation.
7. **`support_export`** — the support/export bundle.

Each binding records the `snapshot_id` and `scope_id` it renders, an `is_canvas` flag, the
`resolves_node_ids` and `resolves_edge_ids` it points at, a `consumer_ref`, and a `note`. A
binding may only resolve ids declared in the packet — a transient per-view id is a validation
failure.

## Guardrails

- **Stable identity across surfaces.** A binding that resolves a node or edge id not declared in
  the packet fails validation (`UnresolvedNodeRef`, `UnresolvedEdgeRef`).
- **No canvas-only source of truth.** Every declared node and edge must be resolvable from a
  non-canvas accessible view, and the canvas may not own an identity a non-canvas surface cannot
  resolve (`NodeMissingNonCanvasSurface`, `EdgeMissingNonCanvasSurface`, `CanvasOnlyNodeIdentity`,
  `CanvasOnlyEdgeIdentity`).
- **Approximate relations stay explicit.** A non-exact edge with no `fidelity_reason` fails
  validation (`UnlabeledNonExactRelation`).
- **Export-safe references.** Every node and edge must carry a unique permalink that embeds its
  canonical id (`UnsafeNodePermalink`, `UnsafeEdgePermalink`, `DuplicatePermalink`), so support,
  issue reports, review comments, and evidence packets can point at the exact object without a
  screenshot.
- **Replayable scope.** Every binding is stamped with the active snapshot and scope
  (`SnapshotBindingMismatch`, `ScopeIdMismatch`), and every surface carries exactly one binding
  (`MissingSurfaceBinding`, `DuplicateSurfaceBinding`).

## How the packet narrows surfaces

The typed model recomputes the summary counts and validates the nodes, edges, and bindings.
`export_projection` produces the redaction-safe identity index that downstream surfaces —
release evidence, help/service-health, docs badges, and support exports — render instead of
restating topology objects by hand. The packet binds upstream to the canonical graph-depth
governance matrix (`governance_matrix_ref`) and the workset-scope packet (`scope_packet_ref`) it
extends, so the shared identity model has one provenance root.

## Out of scope

This packet does not add aesthetic map features that bypass the canonical node or edge identity
contract; non-canvas accessible views must always resolve to the same node and edge objects.
