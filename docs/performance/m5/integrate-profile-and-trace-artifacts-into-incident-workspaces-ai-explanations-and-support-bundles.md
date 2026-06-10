# Integrate Profile and Trace Artifacts into Incident Workspaces, AI Explanations, and Support Bundles

This document is the reviewer-facing landing page for the M5 profile and trace
integration lane.

## Scope

This lane governs how profile and trace artifacts are integrated into:

- **Incident workspaces** — attaching profiles, traces, memory snapshots, and
  related evidence to incident runbooks with preserved build identity, environment
  fingerprint, capture mode, mapping quality, and freshness;
- **AI explanations** — generating AI-derived explanations (hotspot summaries,
  trace narratives, regression comparisons, anomaly explanations) with explicit
  confidence levels and provenance so users know what the explanation is derived
  from;
- **Support bundles** — including profile and trace artifacts in support exports
  with honest inclusion kind, redaction profile, and export posture so bundle
  contents are never opaque.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles/`
- **Packet:** `artifacts/perf/m5/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.json`
- **Schema:** `schemas/perf/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles.schema.json`
- **Fixtures:** `fixtures/performance/m5/integrate-profile-and-trace-artifacts-into-incident-workspaces-ai-explanations-and-support-bundles/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Incident workspace attachment | Stable | Shows artifact origin, build identity, environment fingerprint, capture mode, mapping quality, freshness, and incident workspace link. |
| AI explanation | Stable | Shows confidence, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, and comparison basis. |
| Support bundle inclusion | Stable | Shows inclusion kind, artifact origin, build identity, environment fingerprint, capture mode, mapping quality, redaction profile, and export posture. |
| Export review | Preview | Cross-surface export review for integrated profile and trace artifacts is still under qualification. |
| Cross-reference integrity | Preview | Automated cross-surface integrity checks for profile and trace integration are still under qualification. |

## Incident Workspace Attachment Rows

Incident workspace attachment rows carry:

- `artifact_kind` — `profile`, `trace`, `replay_timeline`, `memory_snapshot`, `coverage`, `test_result`, `debug_session`, or `notebook_output`;
- `incident_workspace_ref` — reference to the incident workspace;
- `artifact_ref` — reference to the artifact being attached;
- `lineage_ref` — reference to the artifact lineage;
- `build_identity_ref` — build identity at capture time;
- `environment_fingerprint_ref` — normalized environment;
- `capture_mode_ref` — capture mode descriptor;
- `mapping_quality_ref` — mapping quality descriptor;
- `freshness` — `current`, `stale`, `expired`, `missing`, `imported`, or `unverified`.

Every attachment row MUST show artifact origin, build identity, environment
fingerprint, capture mode, mapping quality, freshness, and incident workspace
link.

## AI Explanation Rows

AI explanation rows carry:

- `explanation_kind` — `summary`, `hotspot_explanation`, `regression_explanation`, `trace_narrative`, `comparison_narrative`, or `anomaly_explanation`;
- `artifact_ref` — reference to the explained artifact;
- `lineage_ref` — reference to the artifact lineage;
- `confidence` — `high`, `medium`, `low`, or `uncertain`;
- `build_identity_ref` — build identity;
- `environment_fingerprint_ref` — normalized environment;
- `capture_mode_ref` — capture mode descriptor;
- `mapping_quality_ref` — mapping quality descriptor;
- `comparison_basis_ref` — basis for comparison when applicable.

Every explanation row MUST show confidence, artifact origin, build identity,
environment fingerprint, capture mode, mapping quality, and comparison basis.

## Support Bundle Inclusion Rows

Support bundle inclusion rows carry:

- `inclusion_kind` — `full_artifact`, `metadata_only`, `redacted_summary`, or `reference_only`;
- `artifact_ref` — reference to the included artifact;
- `lineage_ref` — reference to the artifact lineage;
- `bundle_ref` — reference to the support bundle;
- `build_identity_ref` — build identity;
- `environment_fingerprint_ref` — normalized environment;
- `capture_mode_ref` — capture mode descriptor;
- `mapping_quality_ref` — mapping quality descriptor;
- `redaction_profile_ref` — redaction profile applied;
- `export_posture_ref` — export posture classification.

Every inclusion row MUST show inclusion kind, artifact origin, build identity,
environment fingerprint, capture mode, mapping quality, redaction profile, and
export posture.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Incident workspace attachment rows MUST show artifact origin, build identity,
  environment fingerprint, capture mode, mapping quality, freshness, and incident
  workspace link; missing truth labels trigger a validation violation.
- AI explanation rows MUST show confidence, artifact origin, build identity,
  environment fingerprint, capture mode, mapping quality, and comparison basis;
  missing truth labels trigger a validation violation.
- Support bundle inclusion rows MUST show inclusion kind, artifact origin, build
  identity, environment fingerprint, capture mode, mapping quality, redaction
  profile, and export posture; missing truth labels trigger a validation
  violation.
- Summary mismatch triggers a validation violation.

## Invariants

- Raw payload bytes, raw command lines, secrets, and ambient credentials do not
  cross this boundary.
- Every attachment, explanation, and inclusion row preserves build identity,
  environment fingerprint, capture mode, and mapping quality so attribution
  never gets lost.
- AI explanation confidence narrows automatically when mapping fidelity or
  artifact identity are weak.
- Support bundle inclusion kind must be honest; generic `artifact included`
  badges are insufficient.
- Export posture must remain explicit across incident workspace, AI explanation,
  and support bundle surfaces.
