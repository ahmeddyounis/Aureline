# M5 Normalized Diagnostic-Record Set

- Packet: `m5-normalized-diagnostic-records:stable:0001`
- Label: `M5 Normalized Diagnostic-Record Set`
- Minted: `2026-06-19T00:00:00Z`
- Entries: 9
- Claimed entries: 9
- Downgraded entries: 1

| Surface | Diagnostic id | Source | Freshness | Remap | Reopen | Suppr | Baseline | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| notebook_cell_diagnostics | `diagnostic:m5:notebook-cell:0001` | runtime_or_test | current | exact | 6/6 | 0 | 0 | beta | beta |
| framework_pack_diagnostics | `diagnostic:m5:framework-pack:0001` | language_service | current | exact | 6/6 | 0 | 1 | beta | beta |
| request_tooling_diagnostics | `diagnostic:m5:request-tooling:0001` | build_or_task | current | exact | 6/6 | 0 | 0 | beta | beta |
| data_tooling_diagnostics | `diagnostic:m5:data-tooling:0001` | build_or_task | recent | exact | 5/6 | 0 | 0 | beta | held |
| preview_runtime_diagnostics | `diagnostic:m5:preview-runtime:0001` | build_or_task | current | contextual | 6/6 | 0 | 0 | beta | beta |
| package_lane_diagnostics | `diagnostic:m5:package-lane:0001` | policy | current | unmapped | 6/6 | 0 | 0 | beta | beta |
| language_provider_diagnostics | `diagnostic:m5:language-provider:0001` | language_service | current | exact | 6/6 | 0 | 0 | stable | stable |
| editor_structural_diagnostics | `diagnostic:m5:editor-structural:0001` | editor_structural | current | exact | 6/6 | 0 | 0 | stable | stable |
| imported_scanner_diagnostics | `diagnostic:m5:imported-scanner:0001` | scanner_import | imported_snapshot | imported_static | 6/6 | 1 | 0 | beta | beta |

- Degraded: `diagnostic:m5:data-tooling:0001` — No AI-evidence reopen handle yet resolves this record; held below preview until the AI-evidence surface can reopen the same canonical diagnostic id
