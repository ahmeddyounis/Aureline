# Fixtures: M5-profile Doctor/repair/container certification

This directory contains fixture metadata for the
`m5_profile_doctor_repair_container_certification` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/m5-profile-doctor-repair-container-certification.json`

## Coverage

- `notebook`, `request_api`, `database`, `profiler`, `remote_preview`, `sync`,
  `companion`, and `incident` are the only claimed marketed profiles, and each
  carries exactly one row — no profile inherits trust from an adjacent one.
- Each profile carries its own current qualification packet, diagnosis-latency
  corpus, rollback path, compatibility/downgrade story, and support-export ref.
- Published qualification covers `certified`, `provisional`, `underqualified`, and
  `unsupported`, and the certification decision covers `promote`,
  `narrow_to_provisional`, `narrow_to_underqualified`, and `fail_promotion`.
- Evidence freshness covers `current`, `stale`, `expired`, and `unknown`; the
  diagnosis-latency state covers `green`, `amber`, `red`, and `unmeasured`; engine
  reachability covers `reachable`, `degraded`, `blocked`, and `not_applicable`; and
  boundary proof covers `verified`, `partial`, `unverified`, and `not_applicable`.
- The five canonical narrowing reasons — `stale`, `diagnosis_latency_red`,
  `repair_underqualified`, `engine_blocked`, and `boundary_proof_missing` — are each
  exercised by at least one profile.
- The promotion gate is exercised in both directions: the clean `notebook` profile
  promotes to a full certification, while stale, latency-red, repair-underqualified,
  engine-blocked, and boundary-missing profiles narrow automatically and the
  unqualified `incident` profile fails promotion. Each row's
  `published_qualification`, `certification_decision`, and `narrowing_reasons` equal
  the recomputed gate decision, so release tooling can prove underqualified profiles
  narrow before publication.
