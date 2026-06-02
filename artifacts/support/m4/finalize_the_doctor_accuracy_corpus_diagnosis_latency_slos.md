# Finalized Doctor Accuracy Corpus, Diagnosis-Latency SLOs, and Headless/UI Parity Artifact

## Summary

This artifact documents the finalized Project Doctor accuracy corpus,
diagnosis-latency SLO catalog, and headless/UI parity audit delivered
under the M04 stable lane.

## Schema

- [`schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json`](../../schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json)

## Crate consumer

- [`crates/aureline-doctor/src/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/mod.rs`](../../crates/aureline-doctor/src/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/mod.rs)

## Fixture corpus

- [`fixtures/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/`](../../fixtures/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/)

## Key finalized contracts

| Contract | Record kind | Purpose |
|---|---|---|
| Accuracy corpus | `project_doctor_accuracy_corpus_record` | Ground truth for all eight seeded scenarios |
| Latency SLO catalog | `project_doctor_latency_slo_catalog_record` | p50/p95 budgets per scenario and surface |
| Stable profile parity audit | `project_doctor_stable_profile_parity_audit_record` | Four-row parity audit per stable profile |
| Finalized support packet | `project_doctor_finalize_support_packet_record` | Combined export projection with corpus metadata and benchmark traces |

## Vocabulary additions

- `AccuracyCorpusScenarioClass`: `missing_toolchain`, `blocked_trust_state`, `broken_watcher`, `incompatible_cache_profile`, `extension_regression`, `wrong_target_environment`, `failed_helper_attach`, `degraded_docs_mirror`
- `MeasurementSurfaceClass`: `doctor_probe_run_local`, `doctor_probe_run_headless`, `doctor_probe_run_inspector`, `doctor_probe_run_managed`
- `LatencyPercentileClass`: `p50`, `p90`, `p95`
- `ThresholdStateClass`: `to_be_set_by_benchmark_council`, `must_complete_under_diagnosis_latency_budget`, `must_not_exceed_false_positive_budget`, `must_not_claim_exact_rollback_without_evidence`, `must_export_complete_escalation_packet`
- `ParityClass`: `full_parity`, `machine_readable_only_no_ui`, `ui_suppressed_consent_required`, `ui_suppressed_unsupported`, `ui_suppressed_managed_authority_required`, `unavailable_in_context`
- `HeadlessExitCodeClass`: `exit_clean_no_findings`, `exit_findings_advisory_only`, `exit_findings_actionable`, `exit_unsupported_context`, `exit_blocked_consent_required`, `exit_probe_runtime_error`
- `UnimplementedCapabilityClass`: `implemented`, `not_yet_implemented_planned`, `not_yet_implemented_descoped`, `deprecated_will_remove`, `permanently_unsupported`
