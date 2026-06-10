# Ship the Hotspot Workspace with Flamegraph, Call Tree, Mapping-Quality Labels, and Source Navigation

**Artifact type:** Performance evidence qualification packet (M5)
**Packet id:** m5_046_hotspot_workspace_qualification:v1
**As of:** 2026-06-09

## Summary

- Flamegraph rows: 4
- Call-tree rows: 5
- Session-strip rows: 2
- Mapping-quality badge rows: 6
- Source-navigation rows: 5
- Stable surfaces: 5
- Below-stable surfaces: 2
- All below-stable surfaces have disclosure: yes

## Claims

| Surface | Claim | Status |
|---|---|---|
| Flamegraph view | Stable | Certified |
| Call tree view | Stable | Certified |
| Session strip | Stable | Certified |
| Mapping-quality badge | Stable | Certified |
| Source navigation | Stable | Certified |
| Export review | Preview | Under qualification |
| Support export | Preview | Under qualification |

## Evidence

- `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/flamegraph_cpu_sampling.json`
- `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/call_tree_cpu_sampling.json`
- `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/session_strip_live_capture.json`
- `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/mapping_quality_exact.json`
- `fixtures/performance/m5/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation/source_navigation_open_source.json`

## Schema and Implementation

- Schema: `schemas/perf/ship-the-hotspot-workspace-with-flamegraph-call-tree-mapping-quality-labels-and-source-navigation.schema.json`
- Implementation: `crates/aureline-profiler/src/ship_the_hotspot_workspace_with_flamegraph_call_tree_mapping_quality_labels_and_source_navigation/`

## Downgrade Rules

1. If a stable surface is missing a required guard, it is narrowed to preview.
2. If a flamegraph row does not show mapping quality, the row is flagged as a validation violation.
3. If a call-tree row does not show symbolization state, the row is flagged as a validation violation.
4. If a session-strip row does not show a degraded-state label, the row is flagged as a validation violation.
5. If a mapping-quality badge does not show its mapping quality, the badge row is flagged as a validation violation.
6. If a source-navigation row does not show mapping quality before the jump, the row is flagged as a validation violation.
7. If a source navigation references an unknown frame, or a mapping-quality badge references an unknown context, the row is flagged as a validation violation.
