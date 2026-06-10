# Materialize Profile Launcher and Attach Sheets, Capture-Mode Descriptors, and Storage-Location Truth

**Artifact type:** Performance evidence qualification packet (M5)
**Packet id:** m5_045_profile_launcher_qualification:v1
**As of:** 2026-06-09

## Summary

- Launcher rows: 5
- Attach-sheet rows: 4
- Capture-mode descriptors: 9
- Storage-location truth rows: 7
- Stable surfaces: 4
- Below-stable surfaces: 2
- All below-stable surfaces have disclosure: yes

## Claims

| Surface | Claim | Status |
|---|---|---|
| Profile launcher | Stable | Certified |
| Attach sheet | Stable | Certified |
| Capture-mode inspector | Stable | Certified |
| Storage-location browser | Stable | Certified |
| Export review | Preview | Under qualification |
| Support export | Preview | Under qualification |

## Evidence

- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/cpu_sampling_launch.json`
- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/attach_process_picker.json`
- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/time_sampling_descriptor.json`
- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/instrumentation_descriptor.json`
- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/local_cache_location.json`
- `fixtures/performance/m5/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth/workspace_relative_location.json`

## Schema and Implementation

- Schema: `schemas/perf/materialize-profile-launcher-and-attach-sheets-capture-mode-descriptors-and-storage-location-truth.schema.json`
- Implementation: `crates/aureline-profiler/src/materialize_profile_launcher_and_attach_sheets_capture_mode_descriptors_and_storage_location_truth/`

## Downgrade Rules

1. If a stable surface is missing a required guard, it is narrowed to preview.
2. If an attach sheet does not show a degraded-state label, the sheet row is
   flagged as a validation violation.
3. If a capture-mode descriptor does not show an overhead warning, the
   descriptor row is flagged as a validation violation.
4. If a storage-location row does not show class, retention, and degraded-state
   warnings, the location row is flagged as a validation violation.
5. If a launcher or attach sheet references an unknown capture mode or storage
   location, the row is flagged as a validation violation.
