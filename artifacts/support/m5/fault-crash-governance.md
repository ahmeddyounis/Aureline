# M5 Fault / Crash Governance Review

This review packet freezes the canonical M5 host-failure, crash-forensics, and
diagnostics-schema contract for long-lived hosts added by the M5 depth lanes.

## Evidence

| Evidence | Path |
| --- | --- |
| Rust packet | `crates/aureline-support/src/m5_fault_crash_governance/mod.rs` |
| Boundary schema | `schemas/support/m5-fault-crash-governance.schema.json` |
| Reviewer doc | `docs/help/support/m5-fault-crash-governance.md` |
| Fixture corpus | `fixtures/support/m5/fault_crash_governance/` |
| Crash contract | `schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json` |
| Restart contract | `schemas/support/supervised-restart-evidence-pipeline.schema.json` |
| Schema/consent registry | `artifacts/governance/telemetry_support_usage_schema_registry.json` |

## Review Findings

| Area | Result |
| --- | --- |
| Fault domains frozen | Seven architecture-defined fault-domain classes are carried as typed rows with isolation unit, restart class, checkpoint source, quarantine triggers, and minimum diagnostic exports. |
| Restart budgets frozen | Five restart classes expose strike window, automatic restart budget, and escalation posture; no retry path widens authority silently. |
| Crash vocabulary frozen | Crash envelope, dump/core artifact, symbol/source-map manifest, local symbolication report, and mirrored symbol service are all typed, local-first, and exact-build governed. |
| Host-family coverage | Notebook, data/API, preview, provider, profiler/replay, and pipeline hosts all consume one canonical contract instead of ad hoc restart or crash wording. |
| Diagnostic schema rows frozen | Crash, performance, usage, and support signals now carry schema id, purpose, data class, opt-in scope, prohibited content classes, retention class, and redaction profile in one packet. |
| Downgrade rules explicit | Restart-evidence staleness, crash-artifact proof staleness, symbolication mismatch, and schema/consent drift all narrow claims automatically instead of leaving stale green rows in place. |

## Host Coverage

| Host family | Fault domain | Restart class | Shared crash/signal posture |
| --- | --- | --- | --- |
| Notebook kernel | `session_execution_host` | `session_scoped` | crash + performance + support, exact-build symbolication |
| Data/API connector | `remote_connector` | `privileged_externally_mutating` | crash + support, explicit approval and no silent rerun |
| Preview dev server | `session_execution_host` | `session_scoped` | crash + performance + support, session drift visible |
| Provider-run session | `ai_tool_broker` | `privileged_externally_mutating` | crash + usage + support, breaker/ticket lineage preserved |
| Profiler/replay session | `session_execution_host` | `session_scoped` | crash + performance + support, mapping quality preserved |
| Pipeline viewer | `remote_connector` | `stateless_helper` | crash + support, reconnect lineage visible |

## Downgrade Posture

- `restart_evidence_stale_narrows_host_claim` drops host rows below qualified
  when restart or quarantine proof ages out.
- `crash_artifact_proof_stale_narrows_crash_claim` prevents stale forensics from
  staying green.
- `symbolication_gap_forces_local_only_forensics` forbids shared exact-build
  conclusions when symbolication is absent or mismatched.
- `diagnostic_schema_stale_blocks_managed_export_claim` blocks managed/shareable
  export claims until schema, consent, retention, and redaction proof is fresh.

## Follow-Ups

- Wire this packet into the support-bundle writer so M5 host families export the
  same failure vocabulary by default.
- Reuse the diagnostic-schema rows when the broader telemetry/support registry
  grows a first-class performance entry.
