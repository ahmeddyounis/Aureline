# M5 Collection-Scope-and-Streaming Fixtures

Conformance fixtures for the M5 diagnostic source-descriptor and
collection-snapshot contract.

## source_and_collection_set.json

The full `DiagnosticSourceAndCollectionPacket`: one source descriptor per claimed
source family (`editor_structural`, `language_service`, `build_or_task`,
`runtime_or_test`, `scanner_import`, `policy`, `heuristic`) and one collection
snapshot per claimed M5 surface (notebook cell, framework pack, request/API
tooling, data tooling, preview runtime, package lane, language provider,
editor-structural guard, imported scanner).

Each source descriptor reuses the canonical `diagnostic_source` record and keeps
its producer identity, tool version, target/environment fingerprint, confidence,
and imported-versus-live origin rather than a second source store. Each snapshot
names the workspace/workset/target scope analyzed, a completeness label,
freshness, a streaming state, the materialized diagnostic refs and/or a resumable
streaming cursor, and the omitted scopes and reasons:

- the framework-pack snapshot is still **streaming** with a resumable cursor and a
  `not_yet_scanned` omitted scope;
- the preview-runtime snapshot is a suppression-**filtered view** with a
  `filtered_by_suppression` omitted scope;
- the editor-structural snapshot is **incremental since the last save**;
- the imported-scanner snapshot is an **imported snapshot** held read-only and
  never rendered as live local truth;
- the data-tooling snapshot **aborted** before completion, so it auto-downgrades
  from `beta` to `held` with an `aborted_collection` trigger and a precise
  degraded label.

This fixture validates against
[`schemas/quality/diagnostic-source-and-collection.schema.json`](../../../../schemas/quality/diagnostic-source-and-collection.schema.json)
and is byte-identical to the checked support export at
[`artifacts/m5/diagnostics/source-collection-proof/support_export.json`](../../../../artifacts/m5/diagnostics/source-collection-proof/support_export.json).

## source_descriptor.example.json

A single `diagnostic_source` descriptor (the imported-scanner family) that
validates against
[`schemas/quality/diagnostic-source-descriptor.schema.json`](../../../../schemas/quality/diagnostic-source-descriptor.schema.json).

## collection_snapshot.example.json

A single `diagnostic_collection_snapshot` (the streaming framework-pack snapshot,
with a resumable cursor and a named omitted scope) that validates against
[`schemas/quality/diagnostic-collection-snapshot.schema.json`](../../../../schemas/quality/diagnostic-collection-snapshot.schema.json).
