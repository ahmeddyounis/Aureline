# M5 Fault / Crash Certification Review

This review packet certifies the claimed M5 host families on the claimed M5
profiles using one shared certification index.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-support/src/m5_fault_crash_certification/mod.rs` |
| Boundary schema | `schemas/support/m5-fault-crash-certification.schema.json` |
| Reviewer doc | `docs/help/support/m5-fault-crash-certification.md` |
| Canonical fixture | `fixtures/support/m5/fault_crash_certification/packet.json` |
| Governance baseline | `fixtures/support/m5/fault_crash_governance/packet.json` |
| Crash store | `fixtures/support/m5/crash_store/packet.json` |
| Schema registry | `fixtures/support/m5/depth_surface_schema_registry/packet.json` |
| Forensic packet | `fixtures/support/m5/forensic_packets/packet.json` |
| Host-failure drills | `fixtures/support/m5/host_failure_drills/packet.json` |
| Symbolication packet | `fixtures/debug/symbolication/packet.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Canonical certification index | Every claimed host/profile row now binds restart, crash, symbolication, schema-governance, and field-readiness proof in one checked packet. |
| Profile-specific truth | Air-gapped rows narrow or withhold provider, browser-bridge, and live pipeline claims instead of inheriting a generic crash-support badge. |
| Downgrade automation | Stale restart, crash, symbolication, schema, drill, or consumer-binding proof can no longer keep a broad claim green. |
| Shared consumer contract | Help/About, service health, support export, and release manifest bindings all point to the same packet id and preserve the same row fields verbatim. |
| Export safety | The certification remains metadata-only and by-reference; raw dumps, raw logs, and secrets stay outside this boundary. |

## Current posture

- `desktop_local_first`, `hybrid_remote_attach`, `managed_cloud`, and
  `self_hosted_sovereign` keep broad qualification for the host families whose
  underlying packets already prove exact-build or labeled-forensics truth.
- `air_gapped_mirror_only` narrows connector, pipeline, provider, and
  browser-bridge claims where the live plane is absent.
- Degraded fixtures prove that stale symbolication narrows to
  `experimental_local_only` and stale schema governance blocks managed-export
  claims as `blocked_unverified`.

## Follow-ups

- If live Help/About, service-health, or release-manifest renderers are added
  for this lane, they should ingest `packet.json` directly rather than copying
  its labels into a new local model.
- If future M5 host families are added, they should extend this certification
  row set in the same change as their governance, symbolication, and drill
  packets.
