# Route / endpoint rows and component / service tree nodes

- Packet: `m5-route-endpoint-component-service-tree-controls:stable:0001`
- Surface: `M5 route / endpoint rows and component / service tree nodes: route / matcher, source file / symbol, HTTP / UI / runtime kind, owning framework / app, params / guards, freshness, evidence source, authored-versus-generated state, exact-versus-heuristic-versus-runtime-confirmed certainty, and canonical proving-source truth across claimed topology explorers`
- Route / endpoint rows: 6 (2 generated)
- Component / service tree nodes: 6 (4 heuristic or partial)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Route / endpoint rows

- **GET /users** — kind `http_route`, evidence `exact_from_source`, certainty `exact_from_source`, authorship `authored`, freshness `current`, proving source `source_file`
- **/_app** — kind `ui_route`, evidence `runtime_confirmed`, certainty `runtime_confirmed`, authorship `framework_provided`, freshness `imported`, proving source `runtime_trace`
- **rpc: SyncService.Push** — kind `rpc_endpoint`, evidence `heuristic_convention`, certainty `heuristic`, authorship `generated`, freshness `stale`, proving source `source_symbol`
- **ws: /events** — kind `websocket_route`, evidence `derived_by_convention`, certainty `heuristic`, authorship `generated`, freshness `never_scanned`, proving source `docs_anchor`
- **runtime: /internal/probe** — kind `runtime_binding`, evidence `partial_evidence`, certainty `partial_or_unresolved`, authorship `runtime_only`, freshness `unknown`, proving source `no_proving_source`
- **unresolved route** — kind `unknown_kind`, evidence `unresolved`, certainty `partial_or_unresolved`, authorship `unknown_origin`, freshness `stale`, proving source `no_proving_source`

## Component / service tree nodes

- **CartView component** — kind `component_node`, evidence `exact_from_source`, certainty `exact_from_source`, relation `parent_child`, proving source `source_file`
- **Payments service** — kind `service_node`, evidence `runtime_confirmed`, certainty `runtime_confirmed`, relation `provider_consumer`, proving source `runtime_trace`
- **Auth module** — kind `module_node`, evidence `heuristic_inferred`, certainty `heuristic`, relation `dependency`, proving source `source_symbol`
- **DI dependency edge** — kind `dependency_edge`, evidence `derived_by_convention`, certainty `heuristic`, relation `root_node`, proving source `docs_anchor`
- **External billing boundary** — kind `external_boundary`, evidence `partial_evidence`, certainty `partial_or_unresolved`, relation `none`, proving source `source_file`
- **Unresolved node** — kind `unknown_node`, evidence `unresolved`, certainty `partial_or_unresolved`, relation `none`, proving source `no_proving_source`
