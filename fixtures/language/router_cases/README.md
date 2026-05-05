# Language protocol-router worked fixtures

These YAML fixtures exercise the router contract frozen in
[`/docs/architecture/language_protocol_router_adr.md`](../../../docs/architecture/language_protocol_router_adr.md)
and the decision packet frozen in
[`/docs/language/language_router_contract.md`](../../../docs/language/language_router_contract.md)
and the boundary schemas at:

- [`/schemas/language/provider_capability.schema.json`](../../../schemas/language/provider_capability.schema.json)
- [`/schemas/language/provider_resolution.schema.json`](../../../schemas/language/provider_resolution.schema.json)
- [`/schemas/language/router_decision.schema.json`](../../../schemas/language/router_decision.schema.json)

Each fixture is a single record of one of these shapes:

- `provider_capability_record`
- `provider_resolution_record`
- `router_decision_record`

The corpus uses only opaque provider / host / epoch / execution-context
/ subject / mapping / policy handles plus typed vocabulary and reviewable
summaries. No fixture carries raw source text, raw provider logs, raw
hostnames, raw URLs, raw process arguments, raw debug values, raw test
payloads, or raw secret material.

## Cases

| Fixture | Record kind | Scenario it freezes |
|---|---|---|
| `completion_lsp_wins_text_fallback.yaml` | `provider_resolution_record` | LSP completion wins over syntax text fallback and AI advisory rows with coordinate translation disclosed. |
| `diagnostics_merge_build_lsp_native.yaml` | `provider_resolution_record` | Build, LSP, and native analyzer diagnostics coexist without losing provider origin or conflict state. |
| `formatting_native_beats_lsp.yaml` | `provider_resolution_record` | Native formatter wins formatting precedence while LSP formatting remains visible as lower precedence. |
| `test_discovery_native_unavailable_heuristic.yaml` | `provider_resolution_record` | Test discovery falls back from an unavailable native adapter to a heuristic parser with degraded state. |
| `debug_adapter_quarantined_capability.yaml` | `provider_capability_record` | DAP adapter descriptor is quarantined after crash loop and marks debug launch/attach/control unsupported. |
| `debug_launch_quarantine_resolution.yaml` | `provider_resolution_record` | Debug launch request becomes inspect-only because the preferred adapter is quarantined and no fallback can own the session. |
| `framework_hover_overlay_conflict.yaml` | `provider_resolution_record` | Framework hover overlays LSP facts while preserving a framework/language disagreement. |
| `definition_missing_providers_text_fallback.yaml` | `router_decision_record` | Definition/navigation request binds to a config root and toolchain identity, then falls back to syntax-only because semantic providers are missing. |
| `diagnostics_semantic_service_crash_loop_cached_graph_fallback.yaml` | `router_decision_record` | Crash-loop-quarantined semantic service forces explicit cached-graph diagnostics fallback with restart-budget and freshness disclosure. |
| `hover_hybrid_remote_graph_local_fallback.yaml` | `router_decision_record` | Hybrid routing picks a remote graph lane as authoritative while keeping local lanes non-authoritative and policy-bounded. |
| `rename_semantics_stale_text_preview_fallback.yaml` | `router_decision_record` | Deliberate downgrade from stale/warming semantics to a narrow file-local text preview for rename, with explicit safety limits. |

## Cross-walk

- Completion covers provider origin labels, coordinate translation, and
  advisory AI not satisfying authoritative requests.
- Diagnostics covers merged coexistence and source conflict disclosure.
- Formatting covers exclusive precedence and safe alternate disclosure.
- Test discovery covers structured-to-heuristic fallback with explicit
  degraded state.
- Debug covers provider quarantine isolation without collapsing unrelated
  language features.
- Framework hover covers overlay routing and conflict visibility.
- Router decision cases cover config/package/workspace root binding, toolchain
  identity disclosure, hybrid lane placement honesty, restart-budget/crash-loop
  supervision vocabulary reuse, and explicit semantic-to-narrower fallback
  summaries.
