# Browser Inspection Fixtures

Worked examples for
[`/docs/runtime/browser_inspection_contract.md`](../../../docs/runtime/browser_inspection_contract.md)
and the boundary schemas:

- [`/schemas/runtime/console_event.schema.json`](../../../schemas/runtime/console_event.schema.json)
- [`/schemas/runtime/network_event_ref.schema.json`](../../../schemas/runtime/network_event_ref.schema.json)
- [`/schemas/runtime/storage_object_state.schema.json`](../../../schemas/runtime/storage_object_state.schema.json)

The fixtures use only opaque runtime, route, source-map, evidence,
task-channel, support-bundle, exact-build, origin, key, digest, and
policy refs. They do not contain raw URLs, hostnames, paths, query
strings, headers, request bodies, response bodies, cookie values,
storage values, console message bodies, stack frames, tokens, or
provider payloads.

| Fixture | Record | Scenario |
|---|---|---|
| `console_live_exact_mapping.yaml` | `console_event_record` | Live console event with exact mapping, redacted-message export, and full viewer linkage. |
| `console_stale_source_map_disclosed.yaml` | `console_event_record` | Stale source map blocks exact source jump and exposes the stale-map disclosure at action time. |
| `network_replayed_request_export_review.yaml` | `network_event_ref_record` | Replayed request remains read-only; copied request and HAR-like evidence stay redacted. |
| `network_cached_service_worker_state.yaml` | `network_event_ref_record` | Stale service-worker cache state blocks replay and is disclosed on the request row. |
| `network_partner_provider_export_blocked.yaml` | `network_event_ref_record` | Partner/provider request evidence stays metadata-only or by-reference under export policy. |
| `storage_cookie_sensitive_blocked.yaml` | `storage_object_state_record` | Cookie storage keeps names/values redacted and requires reviewed local action before mutation. |
| `storage_weak_runtime_inspect_only.yaml` | `storage_object_state_record` | Weak runtime identity forces storage evidence into inspect-only fallback. |

Acceptance coverage:

- Browser inspection surfaces disclose live/runtime provenance,
  mapping fidelity, imported/replayed state, and redaction posture.
- Exports cannot silently include sensitive request, cookie, storage,
  or provider data.
- Drift between source, preview route, service-worker cache, and
  runtime state is represented by typed disclosure values on the row.
- Viewer linkage preserves preview route, preview snapshot, task
  channel, support bundle, exact-build identity, and evidence refs
  without replacing them with surface-local aliases.
