# M5 Diagnostic Source Descriptors and Collection Snapshots

- Packet: `m5-diagnostic-source-and-collection:stable:0001`
- Label: `M5 Diagnostic Source Descriptors and Collection Snapshots`
- Minted: `2026-06-19T00:00:00Z`
- Source descriptors: 7
- Snapshots: 9
- Claimed snapshots: 9
- Downgraded snapshots: 1

## Source descriptors

| Family | Origin | Confidence | Tool version |
| --- | --- | --- | --- |
| editor_structural | live_local_session | Authoritative | tool-version:editor_structural:1.0.0 |
| language_service | live_local_session | Authoritative | tool-version:language_service:1.0.0 |
| build_or_task | live_local_session | DerivedStructured | tool-version:build_or_task:1.0.0 |
| runtime_or_test | live_local_session | Authoritative | tool-version:runtime_or_test:1.0.0 |
| scanner_import | imported_snapshot | ImportedAuthoritative | tool-version:scanner_import:1.0.0 |
| policy | live_local_session | Authoritative | tool-version:policy:1.0.0 |
| heuristic | live_local_session | HeuristicParsed | tool-version:heuristic:1.0.0 |

## Collection snapshots

| Surface | Scope | Completeness | Freshness | Streaming | Omitted | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- | --- |
| notebook_cell_diagnostics | current_root | complete_enumeration | current | settled | 0 | beta | beta |
| framework_pack_diagnostics | workspace | partial_visible_scan | current | streaming | 1 | beta | beta |
| request_tooling_diagnostics | selected_workset | complete_enumeration | current | settled | 0 | beta | beta |
| data_tooling_diagnostics | workspace | partial_visible_scan | stale | aborted | 1 | beta | held |
| preview_runtime_diagnostics | current_root | filtered_view | current | settled | 1 | beta | beta |
| package_lane_diagnostics | workspace | complete_enumeration | current | settled | 0 | beta | beta |
| language_provider_diagnostics | workspace | complete_enumeration | current | settled | 0 | stable | stable |
| editor_structural_diagnostics | current_root | incremental_since_last | recent | settled | 0 | stable | stable |
| imported_scanner_diagnostics | workspace | imported_snapshot_set | imported_snapshot | settled | 1 | beta | beta |

- Degraded: `data_tooling_diagnostics` — The dataset scan aborted before completing; held below preview until a full scan can re-establish whole-workspace coverage
