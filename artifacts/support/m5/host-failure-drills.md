# M5 Host-Failure Drill Review

This review artifact freezes the seeded M5 host-failure corpus, the support-side
forensic packet, and the export-safety assertions support/release reviewers use
to validate the host-failure readiness lane.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust drill packet | `crates/aureline-support/src/m5_host_failure_drills/mod.rs` |
| Rust forensic packet | `crates/aureline-support/src/m5_forensic_packet/mod.rs` |
| Boundary schema | `schemas/support/m5-host-failure-drills.schema.json` |
| Forensic schema | `schemas/support/m5-forensic-packet.schema.json` |
| Reviewer doc | `docs/help/support/m5-host-failure-drills.md` |
| Fixture corpora | `fixtures/support/m5/host_failure_drills/`, `fixtures/support/m5/forensic_packets/` |
| Runtime forensic contract | `schemas/runtime/forensic_packet.schema.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Host-family coverage | All claimed M5 host families now appear in at least one seeded drill row. |
| Required seeded scenarios | Notebook crash/stall, provider-run failure, preview-server restart, remote connector drift, AI broker circuit-breaker, query/runtime crash, pipeline viewer fault, and connector host mismatch are all present. |
| Scoped failure proof | Every drill row asserts restart-budget enforcement, scoped failure, checkpoint preservation, and no-hidden-rerun behavior. |
| Forensic export posture | Support-side forensic rows distinguish `local_only`, `imported`, `mirrored`, and `uploaded` artifact states without hiding destination or retention changes. |
| No-silent-upload proof | Every drill row carries explicit guard assertions that local preview precedes export and egress requires explicit user or policy action. |

## Coverage Notes

- `provider_run_session_host` carries both `provider_run_failure` and
  `ai_broker_circuit_breaker` because provider-run and broker-breaker failures
  share the `ai_tool_broker` fault domain but exercise different recovery and
  export states.
- `profiler_replay_session_host`, `docs_browser_bridge_host`, and
  `infra_helper_job` extend the minimum scenario list so the drill corpus
  covers the full claimed host-family set already frozen by the M5 governance
  packet.

## Export-Safety Posture

- local preview exists before any managed upload or mirror action;
- uploaded artifacts only appear after explicit reviewed share steps;
- imported and mirrored evidence remains labeled as non-local source material;
- crash/support/telemetry boundaries remain separate and auditable.
