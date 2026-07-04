# M5 Manifest / Build Component Qualification

- Packet: `m5-manifest-build-component-qualification:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Bundle: `artifacts/release/m5-manifest-build-component-qualification-proof/support_export.json`
- Consolidates 7 certified component packets
- Consumers: 8 qualified across 8 / 8 claimed consumers
- Status: 5 green / 3 yellow / 0 red

## Rows

- **qual:infrastructure-surface** (infrastructure_surface) — consumer=infrastructure_surface target_context=certified schema_freshness=certified truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified
- **qual:live-resource-surface** (live_resource_surface) — consumer=live_resource_surface target_context=certified schema_freshness=certified truth_layers=disclosed_narrowed adapter_source=certified accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=truth_layer_labels trigger=drift_from_source — Live-resource surface detected rendered-vs-live divergence; the truth-layer claim narrows to disclosed and shows the divergence before presenting live as current
- **qual:execution-launcher** (execution_launcher) — consumer=execution_launcher target_context=certified schema_freshness=certified truth_layers=certified adapter_source=disclosed_narrowed accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=adapter_source_kind trigger=adapter_unavailable — Build launcher fell back to heuristic adapter discovery; the adapter-source claim narrows to disclosed and keeps native-vs-fallback provenance visible before any run
- **qual:incident-support** (incident_support) — consumer=incident_support target_context=certified schema_freshness=disclosed_narrowed truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified_with_narrowing
  - Narrowed: dimension=schema_freshness trigger=schema_stale — Incident-support export references a stale manifest schema mirror; the schema-freshness claim narrows to disclosed and marks the mirror pending refresh
- **qual:handoff-consumer** (handoff_consumer) — consumer=handoff_consumer target_context=certified schema_freshness=certified truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified
- **qual:support-packet** (support_packet) — consumer=support_packet target_context=certified schema_freshness=certified truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified
- **qual:help-center** (help_center) — consumer=help_center target_context=certified schema_freshness=certified truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified
- **qual:release-evidence** (release_evidence) — consumer=release_evidence target_context=certified schema_freshness=certified truth_layers=certified adapter_source=certified accessibility=certified verdict=qualified
