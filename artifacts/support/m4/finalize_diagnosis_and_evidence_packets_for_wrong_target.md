# Finalized Diagnosis and Evidence Packets for Wrong-Target Writes, Stale Truth, Policy Denial, Route Drift, and Install Failures

## Summary

This artifact documents the finalized diagnosis and evidence packet contracts
for five blocked-user failure families delivered under the M04 stable lane.

## Schema

- [`schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json`](../../schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json)

## Crate consumer

- [`crates/aureline-doctor/src/finalize_diagnosis_and_evidence_packets_for_wrong_target/mod.rs`](../../crates/aureline-doctor/src/finalize_diagnosis_and_evidence_packets_for_wrong_target/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target/`](../../fixtures/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target/)

## Key finalized contracts

| Contract | Record kind | Purpose |
|---|---|---|
| Diagnosis packet | `diagnosis_packet_record` | Typed diagnosis for one failure scenario with finding code, confidence, and recovery ladder |
| Evidence packet | `evidence_packet_record` | Redaction-safe evidence items tied to a diagnosis packet |
| Finalized support packet | `finalized_diagnosis_support_packet_record` | Combined export projection with exact-build identity and scenario coverage |

## Failure families covered

- `wrong_target_write` — write operations that land on unexpected targets
- `stale_truth` — cached or mirrored truth treated as current when diverged
- `policy_denial` — security or policy gates blocking expected operations
- `route_drift` — commands or events routed to unexpected handlers
- `install_failure` — toolchain, extension, or dependency install failures

## Vocabulary additions

- `FailureScenarioClass`: `wrong_target_write`, `stale_truth`, `policy_denial`, `route_drift`, `install_failure`
- `EvidenceClass`: `log_trace`, `state_snapshot`, `config_diff`, `route_manifest`, `policy_audit`, `install_artifact`, `symbol_trace`
- `DiagnosisConfidenceClass`: `observed_authoritative`, `inferred_from_evidence`, `unknown_requires_probe`
- `BlastRadiusClass`: `single_disposable_state`, `same_family_state_classes`, `cross_family_state_classes`, `escalation_only`
- `RepairClass`: `observe_only_no_repair`, `reapprove_target_or_route`, `reset_ephemeral_cache`, `reacquire_trust_approval`, `reinstall_toolchain`, `refresh_route_manifest`, `reset_targeted_durable_state`, `defer_to_escalation_packet`
- `RedactionClass`: `metadata_safe_default`, `opt_in_only`, `prohibited`
