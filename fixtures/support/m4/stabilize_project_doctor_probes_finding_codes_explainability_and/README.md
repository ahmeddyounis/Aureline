# Stable Project Doctor Fixtures

This directory holds the protected fixture corpus for the stable Project Doctor lane.

## Files

| File | Record kind | Purpose |
|---|---|---|
| `catalog.yaml` | `project_doctor_stable_probe_pack_catalog_record` | Stable catalog with 4 packs (entry, toolchain, provider, support_bundle_integrity) |
| `finding_entry_target_unavailable.yaml` | `project_doctor_stable_finding_record` | Stable finding for entry target unavailable with explainability factors |
| `finding_toolchain_missing_component.yaml` | `project_doctor_stable_finding_record` | Stable finding for toolchain missing component with explainability factors |
| `finding_provider_credential_expired.yaml` | `project_doctor_stable_finding_record` | Stable finding for provider credential expired with explainability factors |
| `finding_support_bundle_integrity_unsupported.yaml` | `project_doctor_stable_finding_record` | Stable finding for unsupported support-bundle integrity state |
| `unsupported_state_report_scope_out_of_bounds.yaml` | `project_doctor_stable_unsupported_state_report_record` | Formal unsupported-state report for scope out of bounds |

## Consumers

- `crates/aureline-doctor/tests/stabilize_project_doctor_probes_finding_codes_explainability_and.rs`
