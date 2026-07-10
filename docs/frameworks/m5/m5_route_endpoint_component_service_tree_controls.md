# M5 route / endpoint rows and component / service tree nodes

This contract implements the frozen `route_endpoint_row` and `component_service_tree_node` component
families from the [M5 framework-component matrix](m5_framework_component_matrix.md) as two reusable,
co-equal control vectors — the **route / endpoint row** and the **component / service tree node** —
so a user can inspect a framework topology row without hiding its evidence basis.

The Rust validator in
`crates/aureline-templates/src/implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity`
is the authoritative gate; the
[boundary schema](../../../schemas/ui/m5-route-endpoint-component-service-tree-controls.schema.json)
documents the export shape.

## What the route / endpoint row names

A route / endpoint row names, before a user trusts it:

- **Route / path or matcher** — the route label or matcher.
- **Source file / symbol** — the proving file and symbol.
- **HTTP / UI / runtime kind** — `http_route`, `ui_route`, `websocket_route`, `rpc_endpoint`,
  `runtime_binding`, or `unknown_kind`.
- **Owning framework / app** — which framework and app own the route.
- **Params / guards notes** — the path params and the guards.
- **Freshness** — whether the route signal is `current`, `imported`, `stale`, `never_scanned`, or
  `unknown`.
- **Evidence source** — where the row's knowledge came from.

## What the component / service tree node preserves

A component / service tree node preserves:

- The **entity kind** — `component_node`, `service_node`, `module_node`, `dependency_edge`,
  `external_boundary`, or `unknown_node`.
- The **source file / symbol** — the proving file and symbol.
- The **parent / child or provider / consumer relation** — `parent_child`, `provider_consumer`,
  `dependency`, `root_node`, or `none`.
- The **related test / story / doc links**.
- The **partial or derived notes**.

## Derived truth (never asserted)

Both components carry a derived **certainty posture** computed by `resolve_route_evidence_posture`
and `resolve_topology_evidence_posture` from the frozen evidence classes:

- **Certainty posture** — `exact_from_source`, `runtime_confirmed`, `heuristic`, or
  `partial_or_unresolved`. This is the acceptance-criteria axis: a user can tell at a glance whether
  the row is exact, runtime confirmed, a heuristic guess, or only partial. Only `exact_from_source`
  reads as exact; `heuristic` and `partial_or_unresolved` may never read as exact.
- **Authorship posture** (route rows) — `authored`, `generated`, `framework_provided`,
  `runtime_only`, or `unknown_origin`. The authored-versus-generated boundary stays visible at row
  level, never buried in a detail panel.

Because these are derived, a heuristic route or inferred relationship can never read as an exact
one, and a generated route can never leave the authored-versus-generated boundary implicit.

## Proving source (never a hidden parallel model)

Every row and node links back to a canonical proving source — one of `source_file`,
`source_symbol`, `runtime_trace`, or `docs_anchor` — rather than acting like a hidden parallel
model. A component with a static source form must link to a resolvable proving source; a
`runtime_only` or `unresolved` component (which has no source form) must set `no_proving_source` and
name why, so it can never pretend to link to a source file it does not have.

## Hard invariants

Every route row keeps these `false`: `lets_heuristic_masquerade_as_exact`,
`hides_authored_versus_generated_state`, `acts_as_hidden_parallel_model`, and
`invents_alternate_state_label`. Every tree node keeps these `false`:
`lets_heuristic_masquerade_as_exact`, `hides_partial_or_derived_state`,
`acts_as_hidden_parallel_model`, and `invents_alternate_state_label`.

The validator additionally rejects any component whose heuristic or partial posture claims
exact-from-source (`heuristic_claims_exact`).

## Export safety

Raw file bodies, raw source trees, pasted local paths, repository URLs, credentials, and secrets
never cross the export boundary. The canonical proof bundle lives at
`artifacts/release/m5-route-endpoint-tree-node-proof/` and the scenario fixtures at
`fixtures/ui/m5-route-endpoint-component-service-tree-controls/`, both regenerated deterministically
from the seed builders via
`cargo run -p aureline-templates --example dump_route_endpoint_tree_node_controls`.
