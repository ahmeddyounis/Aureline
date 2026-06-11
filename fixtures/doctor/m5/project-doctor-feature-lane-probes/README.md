# Fixtures: Project Doctor feature-lane probes

This directory contains fixture metadata for the
`project_doctor_feature_lane_probes` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-feature-lane-probes.json`

## Coverage

- `notebook_kernel`, `request_api`, `database_target`, `profiler_replay`,
  `preview_route`, `sync_device_registry`, `companion_handoff`, and
  `incident_packet` are the only claimed lanes, and every lane carries exactly one
  read-only probe family — no lane inherits another lane's findings.
- Each family's `finding_code_prefix` matches its lane (`doctor.finding.<lane>.`),
  and every finding's code is in its family's supported set.
- Diagnosis state covers `healthy`, `partial`, `stale`, `unsupported`,
  `policy_blocked`, and `target_mismatch`, so unsupported and partial conditions
  are reported explicitly rather than as generic "unavailable" text.
- Severity covers `info`, `degraded`, `blocking`, and `unsupported`; an
  `unsupported` state always pairs with an `unsupported` severity.
- Confidence covers `observed_authoritative`, `observed_with_gap`,
  `inferred_from_evidence`, and `unknown_requires_probe`.
- Affected scope covers all eight scope-kind identities (`kernel_engine`,
  `api_route`, `database_target`, `profiler_session`, `preview_route`,
  `device_registry`, `companion_session`, `incident_packet`).
- Repair candidates appear only in lanes whose family sets
  `emits_repair_candidates` (request/API, database target, preview route), and
  every repair-candidate id uses the `repair.` prefix.
- Every finding is stable across the desktop card, headless JSON row, and
  support-export row, so support, automation, and users reason about the same
  finding ids and codes across desktop and headless contexts.
