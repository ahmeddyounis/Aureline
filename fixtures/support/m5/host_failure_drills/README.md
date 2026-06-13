# M5 Host-Failure Drill Fixtures

This corpus snapshots the canonical M5 host-failure drill packet plus per-case
evidence slices for every seeded scenario.

Files:

- `packet.json` — canonical packet emitted by `seeded_m5_host_failure_drill_packet()`
- `*.json` — per-scenario evidence slices containing the drill row and its
  matching forensic row

Required scenarios:

- `notebook_kernel_crash_stall`
- `provider_run_failure`
- `preview_server_restart`
- `remote_connector_drift`
- `ai_broker_circuit_breaker`
- `query_runtime_crash`
- `pipeline_viewer_fault`
- `connector_host_mismatch`

Extended host-family coverage:

- `docs_browser_bridge_route_drift`
- `profiler_replay_imported_gap`
- `infra_helper_signature_failure`
