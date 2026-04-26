# Language protocol-router worked fixtures

These YAML fixtures exercise the router contract frozen in
[`/docs/architecture/language_protocol_router_adr.md`](../../../docs/architecture/language_protocol_router_adr.md)
and the boundary schemas at
[`/schemas/language/provider_capability.schema.json`](../../../schemas/language/provider_capability.schema.json)
and
[`/schemas/language/provider_resolution.schema.json`](../../../schemas/language/provider_resolution.schema.json).

Each fixture is a single record of one of these shapes:

- `provider_capability_record`
- `provider_resolution_record`

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
