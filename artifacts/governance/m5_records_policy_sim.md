# M5 Hold/Retention Truth Packet — Artifact Summary

Canonical fixture: `fixtures/governance/m5_records_policy_sim/canonical_packet.yaml`

Schema: `schemas/governance/m5_records_policy_sim.schema.json`

Human-readable companion: `docs/governance/m5_records_policy_sim.md`

Producer: `aureline-records::m5_records_policy`
(`seeded_m5_records_policy_packet`).

Policy companion: `aureline-policy::m5_exception_expiry`
(`seeded_m5_exception_expiry_packet`).

## Purpose

This artifact freezes the canonical legal-hold, retention, archive, and
pre-action delete/export truth for the durable M5 artifact families. It is
metadata-only and carries no credential bodies or raw provider payloads. It is
the canonical legal-hold and retention truth source for new M5
managed/support rows.

## Invariants enforced by `validate()`

- Schema version and record kind match the frozen constants.
- An `active` or `unknown_indeterminate` hold fails closed: the notice blocks
  destruction and the pre-delete truth reports `blocked_by_hold`.
- Local-only families never claim managed hold, managed export, or managed
  delete.
- Every pre-action truth carries a reason and confirmation copy, and its
  `outside_platform_scope` flag agrees with its projected outcome.
- Every retention inspector names a retention owner.
- A managed-hold claim requires a managed-reaching selector scope, and a
  managed-archive claim is never made for a local-only family.
- Every governed M5 family is covered.

## Frozen rows

- `ai_evidence_packet` — delete `policy_retained` (managed evidence copies
  survive a local delete); export `completed`.
- `provider_linked_work_item` — local-only; delete `not_found`, export
  `outside_platform_scope`.
- `companion_continuity_packet` — indeterminate hold; delete `blocked_by_hold`
  fail-closed, export `completed` (reads are unaffected).
- `incident_support_packet` — delete `completed` with a receipt, export
  `omitted_by_redaction`.
- `sync_mirror_ledger` — active managed hold; delete `blocked_by_hold`, export
  `manual_local_capture_required` for per-device local snapshots.
- `browser_handoff_manifest` — local-only; delete `completed`, export
  `outside_platform_scope`.
- `offboarding_record` — delete `policy_retained` citing the retention floor,
  export `partial`.
- `support_export_packet` — delete `completed`, export `completed`.

## Policy exception/expiry companion

The `aureline-policy::m5_exception_expiry` packet pins every gating exception
across actor, object, target, policy epoch, and environment, bounds it with an
explicit expiry, and lists the reapproval triggers that revalidate it on drift.
The records rows reference these exceptions by id.

## Consumers

- `aureline-support::m5_records_policy_governance` — first support/export
  consumer; embeds both packets, projects support rows, inherits validation, and
  proves every exception reference resolves.
