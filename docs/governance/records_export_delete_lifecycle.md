# Export, Delete, and Destruction-Receipt Lifecycle

This page is the human-readable companion to the canonical lifecycle packet
emitted by `aureline-records` and mirrored at
`artifacts/governance/records_export_delete_lifecycle.md`. A frozen example of
the packet lives at
`fixtures/governance/records_export_delete_lifecycle/canonical_packet.yaml` and
its shape is validated by
`schemas/governance/records_export_delete_lifecycle.schema.json`.

## Why this packet exists

Managed, provider-linked, AI-evidence, sync, incident, and offboarding rows now
create or reference durable artifacts whose export and delete behavior must be
inspectable rather than asserted in prose. This packet turns that behavior into
typed product objects so product, CLI/headless, help/docs, and support-export
surfaces all read the same truth.

The packet is the canonical export/delete truth source for those families. It
carries metadata only — no credential bodies and no raw provider payloads.

## Object model

- **Export job** — a resumable export with a mandatory
  [export bundle manifest]. Every export job emits a manifest even when the
  outcome is partial, outside platform scope, or omitted by redaction.
- **Privacy request case** — the durable case that tracks an access-export,
  privacy-delete, or offboarding-delete request. It links its export jobs and
  delete cases and carries a typed blocker when it cannot finish cleanly.
- **Delete case** — a delete with a planning-time and an execution-time hold
  evaluation. A terminal delete either carries a durable destruction receipt or
  a typed blocker state; it never reports a bare "done".
- **Family link** — one authoritative row per governed artifact family binding
  the family's record class, first consumer packet, request case, export job,
  and delete case, plus an explicit local-only boundary note where relevant.

## Outcome vocabulary

Export and delete flows distinguish:

`requested`, `queued`, `completed`, `partial`, `blocked_by_hold`,
`policy_retained`, `outside_platform_scope`, `manual_local_capture_required`,
`not_found`, and `omitted_by_redaction`.

## Required contract

- Export jobs always emit a manifest; delete flows always emit a receipt or a
  typed blocker state.
- Local-only artifacts never masquerade as remotely exported or remotely
  deleted. Local-only boundaries are stated explicitly and disclosed to
  product, CLI, and support surfaces.
- A remembered decision never widens authority across actor, object, target,
  policy epoch, or environment drift.
- Delete/export honesty outranks cosmetically simple "done" copy.

## Governed families

| Family | Example export outcome | Example delete outcome |
| --- | --- | --- |
| AI evidence packet | `completed` | `partial` (policy-retained copies disclosed) |
| Provider-linked work item | `outside_platform_scope` | `not_found` (no managed copy) |
| Sync mirror ledger | `manual_local_capture_required` | `blocked_by_hold` |
| Incident support packet | `omitted_by_redaction` | `completed` (receipt emitted) |
| Offboarding record | `partial` | `policy_retained` (retention floor cited) |

These illustrative rows are exactly the ones frozen in the canonical fixture;
they exercise every export/delete outcome the contract requires.

## Consumers

- `aureline-records::export_delete_lifecycle` is the authoritative producer and
  validator.
- `aureline-support::records_export_delete_governance` is the first support
  consumer; it embeds the authoritative packet, exposes the metadata-only
  support/export projection, and inherits the packet's validation results.
