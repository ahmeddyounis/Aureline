# Approval ticket lifecycle, audit packet, and export-safe event vocabulary contract

This document freezes the approval-ticket lifecycle contract Aureline
uses to preserve **who authorized what**, **for which target**, and
**under which policy/trust posture** across local mutation, remote
attach, credential use, provider mutations, and support/export packets.

Approval tickets are **auditable product truth**, not internal security
plumbing: the ticket fields and event vocabulary are designed to survive
copy/export and reconstruct intent and scope without leaking raw
payloads.

Companion artifacts:

- [`/schemas/trust/approval_ticket.schema.json`](../../schemas/trust/approval_ticket.schema.json)
  — machine-readable boundary for one `approval_ticket_record`.
- [`/schemas/trust/approval_event.schema.json`](../../schemas/trust/approval_event.schema.json)
  — machine-readable boundary for one `approval_event_record`.
- [`/fixtures/trust/approval_ticket_cases/`](../../fixtures/trust/approval_ticket_cases/)
  — worked lifecycle fixtures covering local mutation, publish-later,
  delegated credential use, remote attach, stale target, org switch,
  browser return, and denied reapproval.

This contract composes with (and does not replace):

- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
  — authority tickets, invalidation, and external-effect lineage packets.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — provider-plane vocabulary and browser-handoff constraints.
- [`/docs/work_items/change_intent_and_publish_preview_contract.md`](../work_items/change_intent_and_publish_preview_contract.md)
  — `preview_hash` semantics (hash over structured fields, not raw bodies).

If this document disagrees with those sources, those sources win and the
ticket + event schemas update in the same change.

Out of scope: building the enforcement backend, provider adapters, or a
full audit pipeline implementation. This document freezes the contract
those implementations will read and write.

## Scope

Approval tickets apply to any action that could cause irreversible
consequence or boundary change, including:

- destructive or high-risk local workspace mutations,
- remote attach and remote agent operations that elevate capability,
- credential projection and delegated credential use,
- provider mutations (publish-now, deferred publish drain, browser handoff),
- trust/policy/admin changes.

Tickets MUST NOT be treated as ambient session privilege. Every ticket
admits only the declared action family and target binding; widening
mints a new ticket.

## Ticket record (export-safe by design)

The durable ticket shape is `approval_ticket_record`:

- Actor binding: `issuer_class`, `actor_subject`, `actor_class`.
- Action family: `action_class`, `command_family_or_action_ref`.
- Target binding: `workspace_or_workset_scope_ref`, `target_identity_ref`,
  `execution_context_id`, `sandbox_profile_or_capability_hash`.
- Policy/trust posture at issue: `policy_epoch`, `trust_state`.
- Time bounds: `issued_at`, `expires_at`, `revoke_epoch`.
- Replay and use bounds: `use_posture`, `use_count`, optional
  `bounded_reuse_counter`.
- Export-safe intent linkage:
  - `intent_digest` (required) — hash over the canonical structured
    intent descriptor.
  - `preview_hash` (nullable) — hash over the canonical structured
    preview fields when a preview is required.
- Export-safe evidence linkage: `evidence_refs` (audit/support/evidence
  refs as opaque ids).

### Preview hash vs local preview refs

`preview_hash` is the export-safe fingerprint for the preview the user
confirmed. It is a hash over structured preview fields (for example
projected provider field/action changes), not a hash of any raw provider
body.

`preview_ref` (when present) is a local-only pointer to a durable preview
record. Exports SHOULD prefer `preview_hash` and MAY omit or redact
`preview_ref`.

## Lifecycle events (stable vocabulary)

Approval lifecycle events are emitted as `approval_event_record`
instances using the frozen `audit_event_id` vocabulary:

| `audit_event_id` | Meaning |
|---|---|
| `approval_ticket_preview_shown` | A surface showed the user a preview or intent summary and asserted the no-hidden-target-switch posture for the decision moment. |
| `approval_ticket_approved` | The approval was granted and a ticket id is now admissible (subject to expiry/revocation/drift). |
| `approval_ticket_denied` | The approval was denied. MUST carry `denial_dimension` and `denial_reason`. |
| `approval_ticket_used` | The ticket id was spent successfully. Updates `use_count_observed`. |
| `approval_ticket_widened` | A new ticket id was minted because the requested action/target/scope widened. MUST cite both `prior_ticket_ref` and `widened_to_ticket_ref`. |
| `approval_ticket_revoked` | The ticket id was revoked. MUST carry `revoke_reason`. |
| `approval_ticket_expired` | The ticket id passed `expires_at` before (or during) a spend attempt. MUST carry `denial_dimension` + `denial_reason`. |
| `approval_ticket_replay_blocked` | A spend attempt was blocked due to replay, drift, or invalidation. MUST carry `denial_dimension` + `denial_reason`. |
| `approval_ticket_scope_downgraded` | The admitted action continued only in a narrower or local-only mode. MUST carry `scope_downgrade_reason`. |

### Stable reason codes (required)

Events that deny, revoke, expire, replay-block, or downgrade MUST carry
typed reason codes:

- `denial_dimension` + `denial_reason` explain *why a ticket could not
  be approved or spent* (trust, policy, network scope, credential scope,
  profile enforcement, runtime health).
- `revoke_reason` explains *why a previously valid ticket was revoked*.
- `scope_downgrade_reason` explains *why a previously admissible intent
  continued only in a narrower mode* (for example provider unreachable,
  policy epoch rolled, trust state narrowed, user chose local draft).

Surfaces MUST render these typed reasons (or their controlled
vocabulary projections) visibly. Silent “try again” is non-conforming.

## Export and redaction rules

Tickets and events are designed to be export-safe without leaking raw
bodies. The following export rules are normative:

- Always export-safe fields:
  `ticket_id`, `issuer_class`, `actor_class`, `action_class`,
  `command_family_or_action_ref`, `workspace_or_workset_scope_ref`,
  `target_identity_ref`, `sandbox_profile_or_capability_hash`,
  `execution_context_id`, `policy_epoch`, `trust_state`, `issued_at`,
  `expires_at`, `use_posture`, `use_count`, `ticket_lineage`,
  `intent_digest`, `preview_hash`, `revoke_epoch`, `revoke_reason`,
  `redaction_class`, `evidence_refs`.
- Redaction-required fields (policy-dependent):
  `actor_subject`, `issuing_surface`, and the human-facing prose fields
  (`original_intent.human_summary`, `approval_event_record.event_note`).
  Exports MAY pseudonymize these, but MUST preserve the typed classes
  and opaque ids so lineage remains reconstructable.
- Local-only (or export-redacted) fields:
  `original_intent.machine_descriptor` and `preview_ref` MAY exist
  locally for internal reconstruction, but exports MUST NOT include raw
  secret bytes, raw tokens, raw URLs, raw file bodies, or raw provider
  payloads in these fields. When exported at all, these fields MUST be
  stripped to opaque refs and/or empty objects while preserving
  `intent_digest` and `preview_hash` so lineage semantics survive.

## Worked fixtures

The fixture corpus in `/fixtures/trust/approval_ticket_cases/` covers:

- local destructive workspace mutation gated by `preview_hash`,
- provider publish-later queue admission then drain downgrade on drift,
- delegated credential use that widens scope (new ticket minted),
- remote attach retargeting / stale target requiring reissue,
- org/tenant switch causing host mismatch and replay-block,
- browser return that fails origin correlation and is replay-blocked,
- denied reapproval where a subsequent attempt is also denied with a
  stable policy/trust reason.

These fixtures exist to keep the vocabulary stable across UI, CLI,
support packets, and audit streams.

