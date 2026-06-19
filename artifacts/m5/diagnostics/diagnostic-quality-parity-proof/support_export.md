# M5 Diagnostic Quality Snapshots and Imported-versus-Live Deltas

- Packet: `m5-diagnostic-quality-parity:stable:0001`
- Label: `M5 Diagnostic Quality Snapshots and Imported-versus-Live Deltas`
- Minted: `2026-06-19T00:00:00Z`
- Snapshots: 4
- Claimed snapshots: 4
- Downgraded snapshots: 1
- Delta packets: 3
- Blocked / incomparable deltas: 1

## Quality snapshots

| Snapshot | Scope | Origin | Freshness | Tools | Collections | Debt | Claimed | Effective |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| snapshot:m5:language-provider:0001 | workspace | live_local_session | current | 1 | 1 | 0 | stable | stable |
| snapshot:m5:runtime-test:0001 | selected_workset | live_local_session | recent | 1 | 1 | 2 | beta | beta |
| snapshot:m5:imported-scanner:0001 | workspace | imported_snapshot | imported_snapshot | 1 | 1 | 3 | beta | beta |
| snapshot:m5:ci-import:0007 | baseline_family | imported_snapshot | stale | 1 | 1 | 5 | beta | held |

## Imported-versus-live deltas

| Delta | Basis | Base origin | Compare origin | Compatibility | Notes |
| --- | --- | --- | --- | --- | --- |
| delta:imported-vs-live:0001 | imported_vs_live_rerun | imported_snapshot | live_local_session | compatible_with_local_confirmation | 1 |
| delta:ci-vs-local:0007 | ci_vs_local_rerun | imported_snapshot | live_local_session | blocked_rule_pack_mismatch | 2 |
| delta:runtime-vs-static:0001 | runtime_vs_static_analysis | live_local_session | live_local_session | compatible_exact | 0 |

- Degraded: `snapshot:m5:ci-import:0007` — The imported CI scan predates the current rule-pack epoch and is held below preview until a fresh import or local rerun re-establishes current governance state

- Release-visible debt: 10 (assembled from snapshots: true)
