# Stable Project Doctor probes, finding codes, explainability, and unsupported-state reporting

This document defines the stable Project Doctor lane that promotes the beta probe-pack catalog to a versioned, attributable, confidence-labeled, and explainable diagnosis system with formal unsupported-state reporting.

## What this row owns

- The stable probe-pack catalog vocabulary (`StableProbePackClass`, `StableProbePackLifecycleStatus` including `stable`).
- The stable finding record (`ProjectDoctorStableFinding`) with mandatory explainability factors.
- The formal unsupported-state report (`ProjectDoctorStableUnsupportedStateReport`) emitted when Doctor refuses to diagnose beyond supported evidence.
- The stable support packet (`ProjectDoctorStableSupportPacket`) including chain-of-custody events for export traceability.
- The closed `ExplainabilityFactorClass` vocabulary and `UnsupportedStateClass` vocabulary.
- The boundary schema at [`/schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json`](../../schemas/support/stabilize_project_doctor_probes_finding_codes_explainability_and.schema.json).
- The protected fixture corpus at [`/fixtures/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and/`](../../fixtures/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and/).

## Acceptance and how this row meets it

- Every stable pack declares a `stable`, `beta`, or `deprecated` lifecycle status; stable packs are read-only by default and admitted under headless or support-guided mode.
- Every stable finding carries at least one `ExplainabilityFactor` so UI, CLI, and support export all render the same explanation dimensions.
- Every unsupported-state report is typed with `UnsupportedStateClass`, cites evidence, and carries explainability factors.
- The stable support packet preserves chain-of-custody events so support exports maintain exact-build traceability.
- The stable evaluator refuses unknown pack refs, unsupported finding codes, mismatched versions, missing explainability, and unsafe redaction classes.

## Failure-drill posture

- Invalid schema versions, wrong record kinds, empty ids, and mis-prefixed finding codes are rejected with typed `ProjectDoctorStableViolation` rows.
- Missing `explainability_factors` or `raw_private_material_excluded = false` are treated as validation failures.
- `UnsupportedStateClass::None` combined with a non-empty `unsupported_finding_code` is rejected.
- Duplicate pack ids or finding codes in a catalog are rejected.

## First consumers

- The implementation lives in [`crates/aureline-doctor/src/stabilize_project_doctor_probes_finding_codes_explainability_and/mod.rs`](../../../crates/aureline-doctor/src/stabilize_project_doctor_probes_finding_codes_explainability_and/mod.rs).
- The primary evaluator is `ProjectDoctorStableEvaluator`.

## Related contracts

- [`project_doctor_beta.md`](../m3/project_doctor_beta.md) — beta probe-pack catalog and finding contract.
- [`doctor_probe_packs_beta.md`](../m3/doctor_probe_packs_beta.md) — beta probe-pack family catalog.
- [`doctor_explanation.schema.json`](../../schemas/support/doctor_explanation.schema.json) — finding explanation boundary schema.
- [`doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json) — finding and probe-catalog contract.

## Out of scope for this row

- Repair transaction execution (owned by repair and repair-transaction lanes).
- Support bundle assembly (owned by aureline-support-bundle).
- Crash symbolication (owned by aureline-crash).
- Headless CLI runtime behavior (owned by the shell lane).
