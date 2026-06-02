# Finalized diagnosis and evidence packets for wrong-target writes, stale truth, policy denial, route drift, and install failures

This document defines the finalized diagnosis and evidence packet contracts
that promote five blocked-user failure families into typed, export-safe,
versioned contracts.

## What this row owns

- The [`DiagnosisPacket`] for each of the five failure scenarios, containing
  finding codes, severity, confidence, explainability factors, no-touch
  boundaries, and ordered recovery-ladder rungs.
- The [`EvidencePacket`] paired with each diagnosis packet, containing
  redaction-safe evidence items with exact-build identity.
- The [`FinalizedDiagnosisSupportPacket`] that combines diagnosis and evidence
  refs into one metadata-safe export projection.
- The boundary schema at
  [`/schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json`](../../schemas/support/finalize_diagnosis_and_evidence_packets_for_wrong_target.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target/`](../../fixtures/support/m4/finalize_diagnosis_and_evidence_packets_for_wrong_target/).

## Failure families

| Scenario | Probe pack | Typical finding code | Narrowest repair |
|---|---|---|---|
| Wrong-target write | `trust_policy` / `entry_open_readiness` | `doctor.finding.wrong_target_write` | Reapprove target or route |
| Stale truth | `search_index_readiness` / `trust_policy` | `doctor.finding.stale_truth` | Reset ephemeral cache |
| Policy denial | `trust_policy` / `provider_auth` | `doctor.finding.policy_denial` | Reacquire trust approval |
| Route drift | `entry_open_readiness` / `trust_policy` | `doctor.finding.route_drift` | Refresh route manifest |
| Install failure | `toolchain_resolution` / `restore_continuity` | `doctor.finding.install_failure` | Reinstall toolchain |

## Acceptance and how this row meets it

- Every diagnosis packet cites a scenario from the closed
  [`FailureScenarioClass`] vocabulary, declares a finding code starting with
  `doctor.finding.`, and lists at least one no-touch boundary and one
  explainability factor.
- Every diagnosis packet declares at least one recovery-ladder rung ordered
  from narrowest to widest blast radius.
- Every evidence packet contains at least one evidence item with a
  [`RedactionClass`] and an exact-build identity.
- Evidence packets exclude raw private material and ambient authority by
  default.
- The finalized support packet includes exact-build identity and covers all
  five failure scenarios.

## Failure-drill posture

- Missing finding-code prefix, empty explainability, missing no-touch
  boundaries, and missing recovery-ladder rungs are rejected with typed
  [`DiagnosisEvidenceViolation`] rows.
- Evidence packets that mismatch their paired diagnosis packet by scenario or
  packet id are rejected.
- Duplicate evidence item ids or recovery-ladder repair classes are rejected.
- Missing exact-build identity is rejected.

## First consumers

- The implementation lives in
  [`crates/aureline-doctor/src/finalize_diagnosis_and_evidence_packets_for_wrong_target/mod.rs`](../../../crates/aureline-doctor/src/finalize_diagnosis_and_evidence_packets_for_wrong_target/mod.rs).
- The primary evaluator is `DiagnosisEvidenceEvaluator`.

## Related contracts

- [`stabilize_project_doctor_probes_finding_codes_explainability_and.md`](./stabilize_project_doctor_probes_finding_codes_explainability_and.md) — stable probe-pack catalog and finding contract.
- [`project_doctor_packet.md`](../project_doctor_packet.md) — scenario matrix and scoreboard source.
- [`recovery_ladder_packet.md`](../recovery_ladder_packet.md) — recovery-ladder rung definitions.

## Out of scope for this row

- Live evidence collection (owned by runtime and probe lanes).
- Repair transaction execution (owned by repair and repair-transaction lanes).
- Support bundle assembly (owned by aureline-support-bundle).
- Crash symbolication (owned by aureline-crash).
