# Convention-diagnostic rows and derived-relationship banners

- Packet: `m5-convention-diagnostic-derived-relationship-controls:stable:0001`
- Surface: `M5 convention-diagnostic rows and derived-relationship banners: diagnostic class, affected entity / file, confidence / severity, detected source, suggested fix / open-docs action, support-class caveat, source of inference, last refresh, exact-versus-heuristic-versus-runtime-confirmed state, and canonical proving-source truth across claimed framework-diagnostic surfaces`
- Convention-diagnostic rows: 6 (5 heuristic or partial)
- Derived-relationship banners: 6 (4 heuristic or partial)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Convention-diagnostic rows

- **Route file is missing its required loader export** — class `hard_contract_violation`, confidence `verified`, severity `error`, certainty `exact_from_source`, caveat `fully_supported`, proving source `source_file`
- **Component uses an API removed in the pinned framework version** — class `version_mismatch`, confidence `high_confidence`, severity `warning`, certainty `heuristic`, caveat `version_mismatch`, proving source `source_symbol`
- **Handler name does not follow the framework naming convention** — class `heuristic_suspicion`, confidence `heuristic_convention`, severity `hint`, certainty `heuristic`, caveat `heuristic_only`, proving source `docs_anchor`
- **Pack cannot analyze this dynamic route; a convention is assumed** — class `pack_limitation`, confidence `derived_by_convention`, severity `info`, certainty `heuristic`, caveat `pack_limited`, proving source `source_file`
- **Bridged adapter reports a possibly deprecated lifecycle hook** — class `deprecation_notice`, confidence `low_confidence`, severity `suppressed`, certainty `partial_or_unresolved`, caveat `bridged_behavior`, proving source `runtime_trace`
- **Unclassified framework diagnostic from a stale scan** — class `unknown_diagnostic`, confidence `unknown`, severity `stale`, certainty `partial_or_unresolved`, caveat `unsupported`, proving source `no_proving_source`

## Derived-relationship banners

- **GET /users → users loader** — class `exact_from_source`, proving `proving_source_linked`, certainty `exact_from_source`, inference `static_source`, refresh `current`, proving source `source_file`
- **Checkout page → Payments service** — class `inferred_from_runtime`, proving `runtime_evidence_only`, certainty `runtime_confirmed`, inference `runtime_observation`, refresh `imported`, proving source `runtime_trace`
- **AuthModule → Session store (assumed)** — class `heuristic_link`, proving `source_linked_partial`, certainty `heuristic`, inference `naming_convention`, refresh `stale`, proving source `source_symbol`
- **DI container → provider (by convention)** — class `derived_by_convention`, proving `convention_only`, certainty `heuristic`, inference `dependency_graph`, refresh `never_refreshed`, proving source `docs_anchor`
- **External billing → unknown consumer (partial)** — class `partial_link`, proving `no_proving_source`, certainty `partial_or_unresolved`, inference `manifest_declaration`, refresh `unknown`, proving source `no_proving_source`
- **Unresolved relationship** — class `unresolved_link`, proving `unknown_proving`, certainty `partial_or_unresolved`, inference `static_source`, refresh `unknown`, proving source `no_proving_source`
