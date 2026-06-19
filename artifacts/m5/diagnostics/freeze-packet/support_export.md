# M5 Diagnostic-Truth Lane Matrix

- Packet: `m5-diagnostic-truth-lane:stable:0001`
- Label: `M5 Diagnostic-Truth Lane Matrix`
- Minted: `2026-06-19T00:00:00Z`
- Rows: 9
- Claimed rows: 9
- Downgraded rows: 1

| Surface | Source | Freshness | Completeness | Session | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- |
| notebook_cell_diagnostics | runtime_or_test | current | partial_visible_scan | applied | beta | beta |
| framework_pack_diagnostics | language_service | current | complete_enumeration | applied | beta | beta |
| request_tooling_diagnostics | build_or_task | current | complete_enumeration | preview_required | beta | beta |
| data_tooling_diagnostics | build_or_task | recent | complete_enumeration | unlinked | beta | held |
| preview_runtime_diagnostics | build_or_task | current | complete_enumeration | applied | beta | beta |
| package_lane_diagnostics | policy | current | complete_enumeration | applied | beta | beta |
| language_provider_diagnostics | language_service | current | complete_enumeration | applied | stable | stable |
| editor_structural_diagnostics | editor_structural | current | complete_enumeration | applied | stable | stable |
| imported_scanner_diagnostics | scanner_import | imported_snapshot | imported_snapshot_set | applied | beta | beta |

- Degraded: `data_tooling_diagnostics` — No governing quality session yet binds the data-tooling fix routes; held below preview until a quality-session outcome and rollback boundary are published
