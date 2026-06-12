# Fixtures: M5 execution-certification matrix

This directory contains fixture metadata for the `m5_execution_certification_matrix`
packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-execution-certification.json`

## Coverage

- `build_intelligence`, `target_context_discovery`, `host_boundary`,
  `managed_workspace_lifecycle`, `cluster_context_infrastructure`, `mutation_handoff_review`,
  and `live_resource_context` are the only claimed lanes, and each carries exactly one row —
  no lane inherits a certified claim from an adjacent one.
- Each row binds to the canonical execution-truth packet it certifies via `packet_ref`
  (validated against `CertifiedLane::source_packet`), so the certification aggregates the five
  landed B16 execution packets rather than a parallel spreadsheet, and each row carries its
  own drill, evidence, certification-receipt, release-evidence, service-health, docs-badge,
  and support-export refs.
- Evidence freshness covers `fresh`, `recent`, `stale`, and `expired`; profile coverage
  covers `full`, `partial`, `minimal`, and `absent`; drill outcome covers `passed`,
  `partially_passed`, `inconclusive`, and `failed`; evidence provenance covers `verified`,
  `attested`, `unverified`, and `unverifiable`. The downgrade path covers `refresh_evidence`,
  `narrow_profile`, `narrow_lifecycle`, `withdraw_claim`, and `none`.
- Published qualification covers `certified`, `profile_qualified`, `lifecycle_provisional`,
  and `withdrawn`, and the certification decision covers `certify`, `qualify_profile`,
  `provision_lifecycle`, and `withdraw`.
- The four downgrade reasons — `stale_evidence`, `partial_profile_coverage`,
  `drill_regression`, and `unverified_evidence` — are each exercised by at least one lane.
- The certification gate is exercised in every direction: the clean `build_intelligence` and
  `target_context_discovery` lanes publish certified claims; the partial, attested
  `host_boundary` lane narrows to a profile-qualified claim; the stale
  `managed_workspace_lifecycle`, the minimal/partially-passed `cluster_context_infrastructure`,
  and the inconclusive/unverified `mutation_handoff_review` lanes narrow to
  lifecycle-provisional; and the expired, absent, drill-failing, unverifiable
  `live_resource_context` lane is withdrawn entirely. The `managed_workspace_lifecycle` lane
  is the automatic-downgrade case — a stale drill snapshot is dropped from its declared
  certified claim to lifecycle-provisional rather than left green — while
  `build_intelligence` and `target_context_discovery` prove the gate is not a blanket
  downgrade. The ops-adjacent `managed_workspace_lifecycle`, `cluster_context_infrastructure`,
  `mutation_handoff_review`, and `live_resource_context` lanes narrow safely instead of
  inheriting a broader local or desktop claim. Each lane's `published_qualification`,
  `certification_decision`, and `downgrade_reasons` equal the recomputed gate decision, so
  release, help/service-health, docs, and support-export surfaces ingest one packet and a
  downgraded lane cannot stay green by inertia.
