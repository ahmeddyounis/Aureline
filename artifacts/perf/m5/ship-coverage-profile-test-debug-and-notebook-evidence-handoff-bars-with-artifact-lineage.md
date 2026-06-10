# Ship Coverage, Profile, Test, Debug, and Notebook Evidence Handoff Bars with Artifact Lineage

**Artifact type:** Performance evidence qualification packet (M5)
**Packet id:** m5_051_evidence_handoff_qualification:v1
**As of:** 2026-06-10

## Summary

- Handoff bar rows: 7
- Artifact lineage rows: 7
- Capture source rows: 6
- Save/share scope rows: 5
- Stable surfaces: 5
- Below-stable surfaces: 2
- All below-stable surfaces have disclosure: yes
- Usable lineage rows: 7
- Honest save/share scope rows: 5

## Claims

| Surface | Claim | Status |
|---|---|---|
| Coverage handoff bar | Stable | Certified |
| Profile handoff bar | Stable | Certified |
| Test handoff bar | Stable | Certified |
| Debug handoff bar | Stable | Certified |
| Notebook handoff bar | Stable | Certified |
| Export review | Preview | Under qualification |
| Support export | Preview | Under qualification |

## Evidence

- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_coverage_local_live.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_profile_sampled.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_test_ci_provided.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_debug_local_live.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_notebook_provider_supplied.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_profile_imported.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/handoff_coverage_cached.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_coverage_local_live.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_profile_sampled.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_test_ci_provided.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_debug_local_live.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_notebook_provider_supplied.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_profile_imported.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/lineage_coverage_cached.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/capture_source_local_live.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/capture_source_provider_supplied.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/scope_local_only.json`
- `fixtures/performance/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage/scope_exportable.json`

## Schema and Implementation

- Schema: `schemas/perf/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.schema.json`
- Implementation: `crates/aureline-profiler/src/ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage/`

## Downgrade Rules

1. If a stable surface is missing a required guard, it is narrowed to preview.
2. If a handoff-bar row does not show origin, build ID, commit, capture source,
   save/share scope, and lineage state, the row is flagged as a validation
   violation.
3. If an artifact lineage row does not show build identity, environment
   fingerprint, capture mode, mapping quality, and freshness, the row is flagged
   as a validation violation.
4. If a save/share scope row does not show redaction mode and destination class,
   the row is flagged as a validation violation.
5. If a handoff bar references an unknown capture source, save/share scope, or
   artifact lineage, the row is flagged as a validation violation.
