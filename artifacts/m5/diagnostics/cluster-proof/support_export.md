# M5 Diagnostic-Cluster Set

- Packet: `m5-diagnostic-clusters:stable:0001`
- Label: `M5 Diagnostic-Cluster Set`
- Workspace: `workspace:m5:diagnostic-clusters`
- Minted: `2026-06-19T00:00:00Z`
- Clusters: 4
- Cross-source clusters: 2

| Cluster | Dedupe reason | Members | Sources | Dominant severity | Dominant freshness | Imported | Disclosure |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `cluster:m5:cross-source:0001` | cross_source_corroboration | 3 | 3 | error | imported_snapshot | true | true |
| `cluster:m5:exact-duplicate:0001` | exact_duplicate | 2 | 1 | error | current | false | false |
| `cluster:m5:related-location:0001` | related_by_location | 2 | 2 | warning | current | false | false |
| `cluster:m5:related-cause:0001` | related_by_cause | 2 | 1 | warning | recent | false | true |

- `cluster:m5:cross-source:0001` — Three distinct sources flagged the same anchor family; grouped for display while each member keeps its own provenance, freshness, remap state, and imported-versus-live class. (Same issue corroborated by a language service, an imported scanner, and a build task)
  - `diagnostic:m5:cross-source:language-service:0001` — language_service / live_local / current
  - `diagnostic:m5:cross-source:imported-scanner:0001` — scanner_import / imported / imported_snapshot
  - `diagnostic:m5:cross-source:build-task:0001` — build_or_task / live_local / current
- `cluster:m5:exact-duplicate:0001` — The same notebook runner emitted the same finding on two runs; grouped to one row while both contributing records stay recoverable. (Notebook cell error reported twice by the same runner)
  - `diagnostic:m5:notebook-cell:0001` — runtime_or_test / live_local / current
  - `diagnostic:m5:notebook-cell:0002` — runtime_or_test / live_local / current
- `cluster:m5:related-location:0001` — Two findings from different sources share one location; grouped for display while each keeps its own source kind and policy state. (Editor-structural hint and package-lane policy finding share a location)
  - `diagnostic:m5:editor-structural:0001` — editor_structural / live_local / current
  - `diagnostic:m5:package-lane:0001` — policy / live_local / current
- `cluster:m5:related-cause:0001` — Both findings trace to one causal origin; grouped for display while each keeps its own freshness and remap state. (Preview render notice and request-tooling assertion share one cause)
  - `diagnostic:m5:preview-runtime:0001` — build_or_task / live_local / recent
  - `diagnostic:m5:request-tooling:0001` — build_or_task / live_local / current
