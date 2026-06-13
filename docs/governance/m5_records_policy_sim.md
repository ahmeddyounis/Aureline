# M5 Legal-Hold, Retention, and Pre-Action Truth

This page is the human-readable companion to the canonical M5 hold/retention
packet emitted by `aureline-records` and mirrored at
`artifacts/governance/m5_records_policy_sim.md`. A frozen example of the packet
lives at `fixtures/governance/m5_records_policy_sim/canonical_packet.yaml` and
its shape is validated by
`schemas/governance/m5_records_policy_sim.schema.json`.

## Why this packet exists

The M5 surfaces add AI evidence, provider-linked work items, companion and
incident packets, managed sync/offboarding records, browser handoff manifests,
and richer support/export surfaces. These rows create or reference durable
artifacts that users can be asked to delete, export, or archive. Collapsing
their real boundaries into generic denial copy hides whether an artifact is
held, retained, or simply outside the platform's reach.

This packet turns the legal-hold, retention, archive, and outside-platform-scope
boundaries into typed product objects so product, CLI/headless, and
support/export surfaces all read the same truth before a destructive or
support-sensitive action commits. It is metadata-only and carries no credential
bodies or raw provider payloads.

## Object model

Each governed family contributes one row binding:

- **Legal-hold notice** — the fail-closed hold status (`active`,
  `unknown_indeterminate`, `cleared`), the scope the hold reaches, the backing
  hold refs, the accountable retention owner, and the user-visible notice text.
  An active or indeterminate hold blocks destruction.
- **Hold selector scope** — the exact selector expression and the record classes
  it covers, with explicit flags for whether a managed and/or a local copy is in
  scope. A hold that only reaches the managed copy says so.
- **Retention inspector** — the retention owner, retention label, retention rule,
  delete action, grace rule, and the local and managed owners.
- **Archive inspector** — the archive state (`active_local`, `archived_local`,
  `managed_archive`, `no_archive`) and an explicit note when the only archive is
  local.
- **Pre-action truth** — for both delete and export, the outcome the user would
  actually get if they committed the action now, a plain-language reason, the
  confirmation copy shown before committing, and the outside-platform-scope flag.

## Outcome vocabulary

Pre-action delete/export truth distinguishes the same outcome vocabulary the
records lifecycle uses:

`requested`, `queued`, `completed`, `partial`, `blocked_by_hold`,
`policy_retained`, `outside_platform_scope`, `manual_local_capture_required`,
`not_found`, and `omitted_by_redaction`.

## Required contract

- An `active` or `unknown_indeterminate` hold fails closed: the notice blocks
  destruction and the pre-delete truth reports `blocked_by_hold`.
- Local-only artifacts never claim managed hold, managed export, or managed
  delete; the local-only boundary is stated explicitly.
- The `outside_platform_scope` flag and the projected outcome agree.
- A managed-archive claim is never made for a local-only family.
- Remembered decisions never widen authority; the policy exception/expiry lane
  (`policy:m5_exception_expiry_truth:v1`) pins every exception across actor,
  object, target, policy epoch, and environment and revalidates it on drift.
- Delete/export honesty outranks cosmetically simple "done" copy.

## Governed families

| Family | Hold status | Pre-delete outcome | Pre-export outcome |
| --- | --- | --- | --- |
| AI evidence packet | cleared | `policy_retained` | `completed` |
| Provider-linked work item | cleared (local-only) | `not_found` | `outside_platform_scope` |
| Companion continuity packet | indeterminate | `blocked_by_hold` | `completed` |
| Incident support packet | cleared | `completed` | `omitted_by_redaction` |
| Sync mirror ledger | active | `blocked_by_hold` | `manual_local_capture_required` |
| Browser handoff manifest | cleared (local-only) | `completed` | `outside_platform_scope` |
| Offboarding record | cleared | `policy_retained` | `partial` |
| Support export packet | cleared | `completed` | `completed` |

These illustrative rows are exactly the ones frozen in the canonical fixture;
they exercise every hold status, hold scope, and pre-action outcome the contract
requires.

## Consumers

- `aureline-records::m5_records_policy` is the authoritative producer and
  validator, and exposes product, CLI/headless, and support/export projections
  that share one hold/retention/blocker vocabulary.
- `aureline-policy::m5_exception_expiry` is the policy companion that gates the
  hold/retention claims with time-bounded, actor-scoped exceptions revalidated
  on drift.
- `aureline-support::m5_records_policy_governance` is the first support consumer;
  it embeds both authoritative packets, exposes the metadata-only support/export
  projection, and proves every records-side exception reference resolves to a
  live, bounded policy exception.
