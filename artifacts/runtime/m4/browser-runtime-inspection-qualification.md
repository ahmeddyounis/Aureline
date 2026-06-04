# Browser Runtime Inspection Qualification Artifact

This artifact summarizes the checked-in browser-runtime inspection
qualification packet.

Canonical implementation:
[`crates/aureline-runtime/src/browser_runtime_inspection_qualification/`](../../../crates/aureline-runtime/src/browser_runtime_inspection_qualification/)

Boundary schema:
[`schemas/runtime/browser-runtime-inspection-qualification.schema.json`](../../../schemas/runtime/browser-runtime-inspection-qualification.schema.json)

Reviewer contract:
[`docs/runtime/m4/browser-runtime-inspection-qualification.md`](../../../docs/runtime/m4/browser-runtime-inspection-qualification.md)

Fixture pack:
[`fixtures/runtime/m4/browser-runtime-inspection-qualification/`](../../../fixtures/runtime/m4/browser-runtime-inspection-qualification/)

## Packet Summary

The packet qualifies stable browser-runtime inspection by requiring:

- seven browser-runtime target kinds with target identity, origin scope,
  attach/protocol state, freshness, and drift/resync semantics;
- separate DOM node, framework component, source symbol, and cross-origin frame
  object classes;
- explicit source-map quality states including stale and unavailable mapping;
- distinct console/network/storage states for no data, cross-origin limits,
  protocol unavailable, attach required, and external-browser-only handoff;
- reviewed mutation lineage for storage clear, cookie override, request replay,
  live style edit, force reload, and protocol override actions;
- docs/Help, support, release, optional manifest, and product-label consumers
  that read packet state verbatim and downgrade labels from packet findings.

## Stable Result

The canonical packet built by
`current_stable_browser_runtime_inspection_qualification_packet()` has
`promotion_state: stable` and emits no validation findings. Targets that are not
yet fully live, such as captured snapshots or attach-required device/browser
rows, are explicitly labeled `inspect_only` or `downgraded_below_stable` with
disclosure refs, so they do not overclaim devtools-class depth.

## How to Verify

```sh
cargo test -p aureline-runtime --test browser_runtime_inspection_qualification
```

## Export Posture

The support export is redaction-safe by default. It carries target-kind,
source-map, inspection-state, mutation-action, consumer-surface, and finding
tokens. It does not carry raw DOM, URL, cookie, storage, request/response,
source-map, screenshot, source file, secret, or live-authority material.
