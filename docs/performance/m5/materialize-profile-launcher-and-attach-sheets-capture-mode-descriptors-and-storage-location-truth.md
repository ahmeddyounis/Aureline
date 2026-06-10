# Materialize Profile Launcher and Attach Sheets, Capture-Mode Descriptors, and Storage-Location Truth

This document is the reviewer-facing landing page for the M5 profile-launcher,
attach-sheet, capture-mode descriptor, and storage-location truth lane.

## Scope

This lane governs how profiler and trace surfaces:

- launch new capture sessions with explicit mode, target, and build identity;
- attach to running targets via typed attach sheets that preserve attach
  semantics instead of silently downgrading to launch;
- describe capture modes with honest overhead warnings, sampling parameters,
  and mapping quality states;
- record where evidence is stored with class, retention, freshness, provenance,
  and policy posture so users never view profiles without knowing their origin
  and lifecycle.

## Canonical Artifacts

- **Implementation:** `crates/aureline-profiler/src/materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth/`
- **Packet:** `artifacts/perf/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.json`
- **Schema:** `schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json`
- **Fixtures:** `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/`

## Surfaces

| Surface | Claim | Rationale |
|---|---|---|
| Profile launcher | Stable | Shows launch mode, target identity, capture mode, storage location, build/runtime identity, degraded-state labels, export posture, and retention policy. |
| Attach sheet | Stable | Shows attach kind, target process identity, capture mode, storage location, and degraded-state labels for attach-specific limitations. |
| Capture-mode inspector | Stable | Shows mode class, sampling parameters, overhead class, mapping quality, and honest overhead warnings. |
| Storage-location browser | Stable | Shows location class, path or URI, retention class, freshness state, provenance chain, and policy posture with degraded-state warnings. |
| Export review | Preview | Redaction-safe export flows for profiler evidence are still under qualification. |
| Support export | Preview | Support-bundle redaction for profiler payloads is still under qualification. |

## Capture-Mode Descriptors

The module carries closed capture-mode classes:

- `time_sampling` — time-based sampling with a fixed interval;
- `event_sampling` — event-based sampling triggered by hardware counters;
- `instrumentation` — full instrumentation with probe insertion;
- `hybrid` — hybrid sampling plus selective instrumentation;
- `allocation_sampling` — allocation or heap sampling;
- `render_timeline` — render or frame timeline capture;
- `trace_span_collection` — trace span collection from a backend;
- `replay_recording` — replay sidecar recording.

Every selectable descriptor MUST show an overhead warning and an unavailable
reason when the mode cannot be used for the current target or build.

## Storage-Location Truth

Storage locations carry a closed class vocabulary:

- `local_temp` — local temporary directory;
- `local_cache` — local cache directory with retention policy;
- `workspace_relative` — workspace-relative storage under version control exclusion;
- `remote_store` — remote object store or managed service;
- `support_bundle` — support-retained artifact bundle;
- `imported` — imported from an external file or bundle;
- `archived` — evidence that has been moved or archived;
- `policy_blocked` — location blocked by policy.

Every location row MUST show its class, retention, and policy posture and MUST
warn when the freshness state is `stale`, `expired`, `missing`, `imported`,
`policy_blocked`, or `unverified`.

## Downgrade and Rollback

- Any surface that claims `stable` with an incomplete guard set is narrowed
  automatically by the validator.
- Attach sheets MUST show a degraded-state label; missing labels trigger a
  validation violation.
- Capture-mode descriptors MUST show overhead warnings; missing warnings trigger
  a validation violation.
- Storage-location rows MUST show class, retention, and degraded-state warnings;
  missing truth labels trigger a validation violation.
- Cross-reference failures (unknown capture-mode or storage-location refs)
  trigger validation violations.

## Invariants

- Raw command lines, raw process environment bytes, raw payload bytes, secrets,
  and ambient credentials do not cross this boundary.
- Every launcher and attach sheet points to exactly one capture-mode descriptor
  and one storage-location truth row.
- Every capture-mode descriptor explains its overhead and why it may be
  unavailable.
- Every storage-location row carries freshness, retention, and policy posture.
