# Fixtures: M5 adapter-parity-and-health matrix

This directory contains fixture metadata for the `m5_adapter_parity_and_health_matrix`
packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-adapter-parity-and-health.json`

## Coverage

- `pipeline_build_run`, `preview_route`, `notebook_execution`, `framework_tooling_action`,
  `incident_replay`, and `support_bundle_join` are the only claimed adapter flows, and each
  carries exactly one row — no flow inherits a live authoritative claim from an adjacent
  one.
- Each flow carries its own adapter, target-context, health-strip, health-receipt,
  execution, and support-export ref; every structured-import, imported, or heuristic source
  also carries a source-snapshot ref, and every incident or support join carries a
  support-bundle ref, so a narrowed flow never drops its snapshot or support-join semantics.
- Adapter source covers `native`, `protocol_backed`, `structured_import`, `imported`, and
  `heuristic`; freshness covers `live`, `recent`, `stale`, and `expired`; coverage covers
  `complete`, `partial`, `degraded`, and `absent`; connection covers `connected`,
  `reconnecting`, `bridged`, and `disconnected`; verification covers `verified`, `attested`,
  `unverified`, and `unverifiable`. The recovery path covers `await_live_adapter`,
  `reimport_artifact`, `open_in_provider`, and `none`.
- Published health covers `live_authoritative`, `import_qualified`, `heuristic_provisional`,
  and `unavailable`, and the health decision covers `publish`, `qualify`, `provisional`, and
  `withhold`.
- The six canonical fallback reasons — `imported_artifact`, `heuristic_inference`,
  `stale_snapshot`, `partial_coverage`, `connection_unstable`, and `unverified_source` — are
  each exercised by at least one flow.
- The health gate is exercised in every direction: the live native `pipeline_build_run` and
  the protocol-backed `preview_route` publish live authoritative health; the
  structured-import `notebook_execution` narrows to an import-qualified claim; the imported
  `framework_tooling_action` and the heuristic `incident_replay` narrow to
  heuristic-provisional; and the expired, absent, disconnected, unverifiable
  `support_bundle_join` is withheld entirely. The `framework_tooling_action` row is the
  automatic-downgrade case — a stale imported snapshot on a reconnecting connection is
  dropped from its declared import-qualified claim to heuristic-provisional rather than left
  green — while the `pipeline_build_run` and `preview_route` rows prove the parity model is
  not a blanket downgrade: a live native or protocol-backed adapter that is fresh, complete,
  connected, and verified publishes a clean authoritative claim. The `support_bundle_join`
  row proves a dead imported snapshot is withheld with an await-live-adapter recovery rather
  than masquerading as usable execution truth. Each row's `published_health`,
  `health_decision`, and `fallback_reasons` equal the recomputed gate decision, so release
  and desktop/CLI tooling can prove underqualified flows narrow before they are trusted.
