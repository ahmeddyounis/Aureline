# Records, Chronology, and Policy Governance Matrix

Checked-in artifact: `artifacts/governance/records_policy_simulation_matrix.yaml`

Human-readable companion: `docs/governance/records_policy_simulation_matrix.md`

## Purpose

This packet freezes one governed row per durable managed/provider/support
artifact family that now needs explicit record class, delete/export honesty,
chronology, and policy-simulation coverage.

## Current state

- Seven rows are fully backed.
- One non-release-blocking row (`browser_handoff_manifest`) is narrowed to
  `needs_review` because its proof packet is stale.
- No release-blocking row is currently narrowed, so publication may proceed
  without widening the stale browser-handoff claim.
- Every row also binds a family-specific record class, retention assignment,
  chronology row id, and one or more concrete producer `record_kind`s.

## Rows

- `ai_evidence_packet` uses `ai_retained_evidence_packet` and validates
  `ai_evidence_packet_finalization`.
- `provider_linked_work_item` uses
  `provider_linked_work_item_record` and validates the work-item mutation
  review packet record kind.
- `companion_continuity_packet` uses `companion_continuity_packet` and
  validates the companion continuity packet record kind.
- `incident_support_packet` uses `incident_support_packet` and validates the
  incident workspace surface packet record kind.
- `sync_mirror_ledger` uses `sync_mirror_ledger` and validates the managed sync
  maturity packet record kind.
- `offboarding_record` uses `offboarding_exit_packet` and preserves local
  downloaded copies versus managed retained copies.
- `browser_handoff_manifest` uses `browser_handoff_manifest`, validates the
  browser/provider handoff continuity packet record kind, and is local-only.
- `support_export_packet` uses `support_export_packet` and validates the
  records-policy governance support-export packet record kind.
