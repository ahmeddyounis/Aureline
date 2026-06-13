# M5 Record-Governance Certification Packet — Artifact Summary

Canonical fixture: `fixtures/governance/m5_records_policy_certification/canonical_packet.yaml`

Schema: `schemas/governance/m5_records_policy_certification.schema.json`

Human-readable companion: `docs/governance/m5_records_policy_certification.md`

Producer: `aureline-records::m5_records_policy_certification`
(`seeded_m5_records_policy_certification_packet`).

CI validator: `tools/check_m5_records_policy_certification.py`.

## Purpose

This artifact is the canonical certification and shiproom/public-claim truth
source for the M5 record-governance lane. It aggregates the five lane proofs for
each governed M5 managed/enterprise family into one verdict:

- **record class** — the governed record-class descriptor
  (`records_policy_simulation_matrix`, record-class registry);
- **hold/delete** — legal-hold and delete/export honesty
  (`m5_records_policy`, contract `records:m5_hold_retention_truth:v1`);
- **chronology** — timezone-aware chronology and lineage
  (`m5_evidence_chronology_packet`, contract `chronology:m5_evidence_time_lineage:v1`);
- **policy simulation** — pre-apply policy simulation
  (`m5_policy_impact_simulation_packet`, contract `records:m5_policy_impact_simulation:v1`);
- **exception/expiry** — time-bounded exceptions and remembered-decision
  revalidation (`m5_exception_expiry_packet`, contract `policy:m5_exception_expiry_truth:v1`).

The packet is metadata-only and carries no credential bodies or raw provider
payloads.

## Invariants enforced by `validate()`

- A family certifies only when every proof cell is observed and current; a
  missing or stale cell auto-narrows the family.
- A local-only family that claims a managed hold, export, or delete is narrowed
  via `managed_claim_unproven` and rejected.
- The stored verdict and narrow reasons match the recomputed values.
- A shiproom or public-claim label must read "certified" for a certified row and
  "narrowed" for a narrowed row, so a surface can never render cosmetically clean
  "done" copy over a narrowed claim.
- The promotion decision is `hold` whenever any release-blocking family is
  narrowed, and the blocking list matches the narrowed release-blocking rows.
- Every governed M5 family is covered exactly once and every consumer surface
  (shiproom, public claim, CLI/headless, support export) is bound.
- The summary roll-up matches the computed counts.

## Live cross-check

`verify_against_live_packets()` re-checks the certification against current
same-crate truth: it loads the records/policy matrix and the seeded
hold/retention and policy-simulation packets, runs their own `validate()`, and
confirms each proof cell's source record kind matches live truth. A regression in
any upstream packet — or a tampered proof source — surfaces a cross-check finding
so the affected claim re-certifies instead of silently holding stale "certified"
copy.

## Consumers

Shiproom gate review ingests `shiproom_projection()`; public status surfaces
ingest `public_claim_projection()`; CLI/headless ingests
`cli_headless_projection()`; support export ingests
`support_export_projection()`. All four render the same certification result
rather than cloning their own status text.
