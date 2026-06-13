# Export/Delete Lifecycle Packet — Artifact Summary

Canonical fixture: `fixtures/governance/records_export_delete_lifecycle/canonical_packet.yaml`

Schema: `schemas/governance/records_export_delete_lifecycle.schema.json`

Human-readable companion: `docs/governance/records_export_delete_lifecycle.md`

Producer: `aureline-records::export_delete_lifecycle`
(`seeded_records_export_delete_lifecycle_packet`).

## Purpose

This artifact freezes the canonical export-job, privacy/delete-case, and
destruction-receipt truth source for claimed M5 managed, provider-linked,
AI-evidence, sync, incident, and offboarding rows. It is metadata-only and
carries no credential bodies or raw provider payloads.

## Invariants enforced by `validate()`

- Schema version and record kind match the frozen constants.
- Every export job carries a non-empty manifest bundle id; completed/partial
  jobs carry bundle refs; partial jobs carry partial/omission reasons.
- Every request case links at least one export job or delete case.
- Completed/partial delete cases carry a destruction receipt; blocked,
  policy-retained, outside-scope, manual-local-capture, not-found, and
  omitted-by-redaction delete cases carry a typed blocker state.
- Every required governed family is covered, and each family link resolves to a
  known request case, export job, and delete case whose outcomes agree.

## Frozen rows

- `ai_evidence_packet` — export `completed`, delete `partial` with a receipt
  that discloses policy-retained evidence copies; local prompt/result caches
  remain on-device.
- `provider_linked_work_item` — export `outside_platform_scope`, delete
  `not_found`; only local linkage metadata was ever possessed.
- `sync_mirror_ledger` — export `manual_local_capture_required`, delete
  `blocked_by_hold`; per-device local snapshots are outside managed scope.
- `incident_support_packet` — export `omitted_by_redaction`, delete `completed`
  with a destruction receipt.
- `offboarding_record` — export `partial`, delete `policy_retained` citing the
  retention-floor horizon; downloaded local exports stay user-controlled.

## Consumers

- `aureline-support::records_export_delete_governance` — first support/export
  consumer; embeds the packet, projects support rows, and inherits validation.
