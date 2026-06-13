# M5 Host-Failure Drills

This packet family is the support-side readiness proof for M5 host failure,
crash forensics, and no-silent-upload behavior. It binds seeded host-failure
corpora to the existing M5 governance, crash-store, depth-surface registry, and
recovery-review packets so support and release surfaces can validate one
repeatable drill set instead of anecdotal incident notes.

## What the packet proves

- every claimed M5 host family has at least one seeded failure drill;
- restart-budget enforcement stays scoped to the failing host family;
- checkpoints, reviewed metadata restore boundaries, or clean-state restart
  limits stay visible and export-safe;
- mutating, privileged, or provider-backed lanes never silently rerun after
  failure; and
- crash, support, and telemetry flows keep local preview, explicit review, and
  retention/export-scope disclosure ahead of any upload or mirror path.

## Covered scenarios

| Scenario | Host family | Primary proof |
| --- | --- | --- |
| `notebook_kernel_crash_stall` | Notebook kernel session | crash/stall stays local to the kernel lane and preserves the cell-run checkpoint |
| `provider_run_failure` | Provider-backed run session | provider failure revokes authority and requires reviewed export before any upload |
| `preview_server_restart` | Preview dev server | restart budget exhausts visibly and keeps mirrored symbol use explicit |
| `remote_connector_drift` | Data/API connector and query runtime | imported drift evidence never masquerades as live route truth |
| `ai_broker_circuit_breaker` | Provider-backed run session | breaker-open state keeps replay blocked and export local-first |
| `query_runtime_crash` | Query/request runtime | request-lane crash preserves metadata checkpoint and no-hidden-rerun posture |
| `pipeline_viewer_fault` | Pipeline viewer session | imported event lineage stays labeled in recovery and export |
| `connector_host_mismatch` | Registry or database connector | target mismatch forces visible fail-closed state and explicit upload review |
| `docs_browser_bridge_route_drift` | Docs and browser bridge | imported route facts remain distinct from local reviewed export |
| `profiler_replay_imported_gap` | Profiler and replay session | partial mapping quality stays explicit and export-safe |
| `infra_helper_signature_failure` | Infrastructure helper | signature failure fails closed and preserves signed-cache checkpoint lineage |

## Forensic packet posture

The support-side forensic packet is intentionally narrower than the runtime
forensic packet. It reuses exact-build identity, trigger, checkpoint, and
restart-lineage truth, then adds export-facing state:

- artifact locality/state: `local_only`, `imported`, `mirrored`, `uploaded`
- redaction posture: shared M5 support/export redaction classes
- retention posture: local user-owned, local manifest until sent, or managed
  contract window
- reviewed share actions: local preview, local export, mirror copy, managed
  upload, or imported handoff

Every row must expose a local preview before any egress path. Any uploaded or
mirrored path must remain behind explicit user or policy action and a visible
retention/scope review step.

## No-silent-upload rules

- crash envelopes, dumps, symbolication inputs, and support manifests are
  local-first by default;
- imported or mirrored evidence remains labeled as such in support/export
  packets;
- managed upload is a reviewed action, not a consequence of a crash;
- mirror or upload paths disclose destination and retention scope explicitly;
- support export remains a separate explicit share flow, not ambient telemetry.

## Companion files

- `schemas/support/m5-host-failure-drills.schema.json`
- `schemas/support/m5-forensic-packet.schema.json`
- `fixtures/support/m5/host_failure_drills/`
- `fixtures/support/m5/forensic_packets/`
- `artifacts/support/m5/host-failure-drills.md`
