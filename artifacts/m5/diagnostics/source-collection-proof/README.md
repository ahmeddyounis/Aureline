# M5 Diagnostic Source-and-Collection Proof

`support_export.json` is the checked support export of the M5 diagnostic
source-descriptor and collection-snapshot packet
(`DiagnosticSourceAndCollectionPacket`). It is the canonical artifact downstream
Problems, review, saved-view, CLI/headless, and support surfaces ingest through
`aureline_runtime::m5_diagnostic_source_descriptors_and_collection_snapshots::current_m5_source_and_collection_export`
instead of cloning provider-local scan state.

The packet carries:

- one **source descriptor** per claimed source family — `editor_structural`,
  `language_service`, `build_or_task`, `runtime_or_test`, `scanner_import`,
  `policy`, and `heuristic` — each naming a producer identity, tool and tool
  version, target/environment fingerprint, confidence, and imported-versus-live
  origin class, reusing the canonical `diagnostic_source` record rather than a
  second source store;
- one **collection snapshot** per claimed M5 surface — notebook cell, framework
  pack, request/API tooling, data tooling, preview runtime, package lane,
  language provider, editor-structural guard, and imported scanner — each naming
  the workspace/workset/target scope analyzed, a completeness label, freshness, a
  streaming state, the materialized diagnostic refs and/or a resumable streaming
  cursor, and the omitted scopes and reasons.

It proves the honesty guarantees the surfaces depend on:

- users can tell whether a set is **settled, streaming, partial, filtered,
  incremental, imported, current, recent, stale, or aborted** — the framework-pack
  snapshot is still streaming with a resumable cursor, the preview-runtime snapshot
  is a suppression-filtered view, the editor-structural snapshot is incremental
  since the last save, and the imported-scanner snapshot is an imported snapshot
  held read-only and never shown as live local truth;
- partial, filtered, streaming, and aborted snapshots name at least one **omitted
  scope** with its reason, so an empty or tiny set cannot pose as whole-workspace
  truth.

The data-tooling snapshot is the auto-downgrade demonstration: its scan aborted
before completing, so it auto-downgrades from `beta` to `held` with an
`aborted_collection` trigger and a precise degraded label, while every other
snapshot's effective qualification equals its claim.

`support_export.md` is the deterministic Markdown summary of the same packet.

## Regenerate

```bash
cargo run -p aureline-runtime --example dump_m5_diagnostic_source_and_collection > \
  artifacts/m5/diagnostics/source-collection-proof/support_export.json
cargo run -p aureline-runtime --example dump_m5_diagnostic_source_and_collection summary > \
  artifacts/m5/diagnostics/source-collection-proof/support_export.md
```

The artifact validates against
[`schemas/quality/diagnostic-source-and-collection.schema.json`](../../../../schemas/quality/diagnostic-source-and-collection.schema.json)
(composed from
[`schemas/quality/diagnostic-source-descriptor.schema.json`](../../../../schemas/quality/diagnostic-source-descriptor.schema.json)
and
[`schemas/quality/diagnostic-collection-snapshot.schema.json`](../../../../schemas/quality/diagnostic-collection-snapshot.schema.json))
and is byte-identical to the protected fixture at
[`fixtures/quality/m5/collection-scope-and-streaming/source_and_collection_set.json`](../../../../fixtures/quality/m5/collection-scope-and-streaming/source_and_collection_set.json).
