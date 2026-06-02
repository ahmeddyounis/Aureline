# Stable Project Doctor Artifact

## Summary

This artifact documents the stable Project Doctor probe-pack catalog, finding contract, explainability, and unsupported-state reporting delivered under the M04 stable lane.

## Schema

- [`schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json`](../../schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json)

## Crate consumer

- [`crates/aureline-doctor/src/stabilize_project_doctor_probes_finding_codes_explainability_and/mod.rs`](../../crates/aureline-doctor/src/stabilize_project_doctor_probes_finding_codes_explainability_and/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and/`](../../fixtures/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and/)

## Key stable contracts

| Contract | Record kind | Purpose |
|---|---|---|
| Stable probe-pack catalog | `project_doctor_stable_probe_pack_catalog_record` | Closed list of stable/beta/deprecated packs |
| Stable finding | `project_doctor_stable_finding_record` | Typed finding with explainability factors |
| Unsupported-state report | `project_doctor_stable_unsupported_state_report_record` | Formal refusal-to-diagnose report |
| Stable support packet | `project_doctor_stable_support_packet_record` | Metadata-safe export projection with chain-of-custody |

## Vocabulary additions

- `StableProbePackLifecycleStatus`: `stable`, `beta`, `deprecated`
- `ExplainabilityFactorClass`: `expected_state`, `observed_state`, `belief_basis`, `counter_evidence`, `limitation`, `user_impact`, `safety_reason`, `next_action_reason`
- `UnsupportedStateClass`: `none`, `unsupported_profile`, `unsupported_probe_target`, `unsupported_offline_state`, `unsupported_managed_policy_state`, `insufficient_permissions`, `unsupported_high_risk_capture`, `evidence_unavailable`, `scope_out_of_bounds`, `dependency_missing`, `context_not_admitted`, `unsupported_platform`
