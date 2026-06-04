# Browser Runtime Inspection Qualification

This contract qualifies browser-runtime inspection rows before product
surfaces, docs, Help, support exports, release packets, or optional-surface
manifests may call them stable.

It composes the existing browser-runtime and browser-inspection contracts:

- [`docs/runtime/browser_runtime_contract.md`](../../browser_runtime_contract.md)
- [`docs/runtime/browser_inspection_contract.md`](../../browser_inspection_contract.md)
- [`schemas/runtime/browser_runtime_session.schema.json`](../../../schemas/runtime/browser_runtime_session.schema.json)
- [`schemas/runtime/console_event.schema.json`](../../../schemas/runtime/console_event.schema.json)
- [`schemas/runtime/network_event_ref.schema.json`](../../../schemas/runtime/network_event_ref.schema.json)
- [`schemas/runtime/storage_object_state.schema.json`](../../../schemas/runtime/storage_object_state.schema.json)

## Stable Qualification Rules

The packet is stable only when it proves all of the following:

- Every browser-runtime target kind has a row: `embedded_preview`,
  `external_browser_tab`, `simulator_webview`, `device_browser`,
  `device_webview`, `remote_preview_session`, and `captured_snapshot`.
- Target rows bind target identity, origin scope, attach/protocol state,
  session freshness, and drift/resync semantics. Cached metadata does not
  imply live control authority.
- Preview and browser-runtime inspection remain separate. Preview answers what
  is rendered; browser-runtime inspection answers why runtime pixels, events,
  requests, and storage behave that way.
- DOM node, framework component, source file/symbol, and cross-origin frame are
  separate object classes with separate affordances and mapping states.
- Source-map quality is explicit: `exact`, `approximate`, `framework_only`,
  `runtime_only`, `stale`, and `unavailable` are separate states.
- Console, network, and storage lanes preserve `data_available`, `no_data`,
  `cross_origin_limited`, `protocol_unavailable`, `attach_required`, and
  `external_browser_only` without collapsing them into a generic unavailable
  label.
- Mutating actions (`clear_storage`, `cookie_override`, `replay_request`,
  `live_style_edit`, `force_reload`, and `protocol_override`) carry explicit
  review, rollback or export lineage, target identity, redaction-safe export,
  and no-hidden-side-effect proof.
- Product labels, docs/Help, support exports, release packets, and
  optional-surface manifests consume packet state verbatim and do not imply
  standalone browser devtools parity.

## Downgrade Behavior

The packet downgrades or blocks stable claims when target identity, source-map
quality, attach protocol, cross-origin access, session freshness, mutation
safety, redaction, or consumer-surface binding is missing. Downgraded rows must
carry a disclosure ref, and consumer surfaces must render that downgraded state
instead of inheriting a nearby stable preview row.

## Boundary

The packet is metadata-only. It excludes raw DOM text, raw selectors tied to
private content, raw URLs, hostnames, headers, cookies, request/response bodies,
storage keys or values, screenshots, source-map bytes, source files, secrets,
and ambient runtime authority. Support exports carry enum tokens, opaque refs,
findings, and redaction-safe summaries only.

## Verification

Run:

```sh
cargo test -p aureline-runtime --test browser_runtime_inspection_qualification
```

The tests materialize the canonical packet from
`crates/aureline-runtime/src/browser_runtime_inspection_qualification/`, check
fixture/doc/schema paths, and assert that missing target coverage, weak runtime
truth, stale source maps, unsafe mutations, collapsed object classes, and unsafe
export material block stable promotion.
