# Finalized Doctor accuracy corpus, diagnosis-latency SLOs, and headless/UI parity on stable profiles

This document defines the finalized Project Doctor accuracy corpus,
diagnosis-latency SLO catalog, and headless/UI parity audit that promote
the stable lane into a measurable, benchmark-gated system.

## What this row owns

- The [`DoctorAccuracyCorpus`] with seeded ground-truth records for all
eight scenario families.
- The [`DiagnosisLatencySloCatalog`] with percentile-based budgets (p50,
p95) per scenario and measurement surface.
- The [`StableProfileParityAudit`] with exactly four parity rows per
stable profile (desktop, cli_headless, remote_managed, offline_local).
- The [`ProjectDoctorFinalizeSupportPacket`] that combines corpus
metadata, latency row summaries, parity audit refs, and benchmark-lab
trace refs into one metadata-safe export projection.
- The boundary schema at
[`/schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json`](../../schemas/support/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos.schema.json).
- The protected fixture corpus at
[`/fixtures/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/`](../../fixtures/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/).

## Acceptance and how this row meets it

- Every ground-truth record cites a scenario from the closed
[`AccuracyCorpusScenarioClass`] vocabulary, declares an expected finding
code starting with `doctor.finding.`, and lists at least one no-touch
boundary and one explainability factor.
- The accuracy corpus covers all eight scenario families so no stable
scenario is untested.
- Every latency budget row declares p50 and p95 targets with ordered
yellow/red thresholds, or carries `to_be_set_by_benchmark_council` until
measurement completes.
- Every parity audit contains exactly four rows, one per support context,
with the six mandatory machine-readable fields so findings are replayable
without semantic drift.
- The finalized support packet excludes raw private material and ambient
authority by default.

## Failure-drill posture

- Missing scenarios, duplicate row ids, and finding codes without the
`doctor.finding.` prefix are rejected with typed
[`ProjectDoctorFinalizeViolation`] rows.
- Latency budgets with zero targets or non-ordered thresholds are rejected.
- Parity audits with fewer or more than four rows, duplicate contexts, or
missing mandatory machine-readable fields are rejected.
- Benchmark-lab traces that disagree with the corpus or latency catalog
are surfaced as validation warnings.

## First consumers

- The implementation lives in
[`crates/aureline-doctor/src/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/mod.rs`](../../../crates/aureline-doctor/src/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos/mod.rs).
- The primary evaluator is `ProjectDoctorFinalizeEvaluator`.

## Related contracts

- [`stabilize_project_doctor_probes_finding_codes_explainability_and.md`](./stabilize_project_doctor_probes_finding_codes_explainability_and.md) — stable probe-pack catalog and finding contract.
- [`project_doctor_packet.md`](../project_doctor_packet.md) — scenario matrix and scoreboard source.
- [`diagnosis_latency_scoreboard.yaml`](../../artifacts/support/diagnosis_latency_scoreboard.yaml) — scoreboard row registry.
- [`diagnosis_slo_targets.yaml`](../../artifacts/support/diagnosis_slo_targets.yaml) — numeric target catalog.

## Out of scope for this row

- Live benchmark-lab execution (owned by the benchmark and CI lanes).
- Repair transaction execution (owned by repair and repair-transaction lanes).
- Support bundle assembly (owned by aureline-support-bundle).
- Crash symbolication (owned by aureline-crash).
