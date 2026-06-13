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

## Rows

- `ai_evidence_packet` uses `ai_retained_evidence_packet` and keeps managed
  retention visible.
- `provider_linked_work_item` uses `operational_audit_record` and keeps local
  draft versus managed retained truth distinct.
- `companion_continuity_packet` uses `managed_workspace_metadata` and treats the
  local copy as a cache rather than the authority source.
- `incident_support_packet` and `support_export_packet` use
  `support_bundle_archive` and preserve held/queued/retained-evidence honesty.
- `sync_mirror_ledger` uses `managed_copy_index_entry` and keeps imported/live
  chronology distinct.
- `offboarding_record` uses `offboarding_exit_packet` and preserves local
  downloaded copies versus managed retained copies.
- `browser_handoff_manifest` is local-only and explicitly does not claim remote
  delete, export, or hold authority.
