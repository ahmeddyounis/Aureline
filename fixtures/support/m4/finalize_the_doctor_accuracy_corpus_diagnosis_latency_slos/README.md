# Finalized Doctor accuracy corpus, diagnosis-latency SLOs, and headless/UI parity fixtures

This directory contains the protected fixture corpus for M04-158.

## Files

- `manifest.yaml` — fixture manifest with required assertions and source refs.
- `accuracy_corpus.yaml` — ground truth records for all eight seeded support scenarios.
- `latency_slo_catalog.yaml` — p50 and p95 latency budgets per scenario and surface.
- `parity_audit_stable.yaml` — headless/UI parity audit with exactly four rows.

## Usage

The test file `crates/aureline-doctor/tests/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.rs` loads these fixtures and validates them through the `ProjectDoctorFinalizeEvaluator`.
