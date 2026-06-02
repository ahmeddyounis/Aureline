# Hardened Crash Capture, Exact-Build Symbolication, Crash-Loop Detection, and Evidence Preview/Export Artifact

## Summary

This artifact documents the hardened crash-capture, exact-build symbolication,
crash-loop detection, and evidence preview/export lane delivered under the M04
stable line.

## Schema

- [`schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json`](../../schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json)

## Crate consumer

- [`crates/aureline-crash/src/harden_crash_capture_exact_build_symbolication_crash_loop/mod.rs`](../../crates/aureline-crash/src/harden_crash_capture_exact_build_symbolication_crash_loop/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/harden_crash_capture_exact_build_symbolication_crash_loop/`](../../fixtures/support/m4/harden_crash_capture_exact_build_symbolication_crash_loop/)

## Key contracts

| Contract | Record kind | Purpose |
|---|---|---|
| Crash-loop signal | `crash_loop_signal_record` | Emitted when restart budgets are breached |
| Evidence preview | `crash_evidence_preview_record` | Pre-export preview with included/omitted items |
| Evidence export packet | `crash_evidence_export_packet_record` | Metadata-safe export with chain-of-custody |

## Vocabulary additions

- `CrashLoopDetectionState`: `no_loop`, `emerging`, `confirmed`, `escalating`
- `CrashLoopScenarioClass`: `startup_restart_budget_exceeded`, `reopen_restart_budget_exceeded`, `runtime_host_restart_budget_exceeded`, `extension_host_restart_budget_exceeded`, `restore_replay_failed_repeatedly`
- `RecoveryLadderHookClass`: `safe_mode_minimal_profile`, `open_without_restore`, `export_evidence`, `retry_fault_domain`, `disable_recent_extension`, `reset_ephemeral_cache`
- `ExportRedactionClass`: `metadata_safe_default`, `operator_only_restricted`, `local_only`
- `EvidenceInclusionState`: `embedded_metadata`, `by_reference`, `omitted`
