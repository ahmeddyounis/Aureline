# Alpha Shared-Terminal Control-Channel with Explicit Grants and Presenter-Handoff Audit

This document is the reviewer-facing landing page for the alpha shared-terminal
control-channel record family: every shared terminal pane that any other actor
can observe or drive is projected through one
[`SharedTerminalAlphaPage`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
that distinguishes view-only, request-control, active-control, and revoked
states; records presenter-handoff and control-change audit events; and
preserves local terminal continuity when control ends or degrades.

The machine-readable boundary lives at
[`/schemas/runtime/shared_terminal_control_alpha.schema.json`](../../../schemas/runtime/shared_terminal_control_alpha.schema.json).
The Rust implementation lives at
[`/crates/aureline-runtime/src/shared_terminal_alpha/`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs).
The protected fixture lives at
[`/fixtures/runtime/shared_terminal_alpha/page.json`](../../../fixtures/runtime/shared_terminal_alpha/page.json).

## The alpha promise

- Every participant on a shared terminal resolves to exactly one
  [`SharedTerminalControlState`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
  in one of four typed states:
  `view_only_observer`, `request_control_pending`,
  `active_control_grantee`, or `control_revoked`. Control is **never**
  inferred from presence or presenter state — an
  `active_control_grantee` row that does not cite a
  `control_grant_ref` on the upstream
  [`control_grant_record`](../../../schemas/collaboration/control_grant.schema.json)
  is refused at validate time, mirroring the
  `control_grant_not_inferable_from_presence_or_presenter_state`
  invariant on the upstream control-grant contract.
- Presenter handoff is recorded as an explicit
  [`PresenterHandoffEvent`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
  whose typed outcome (`presenter_role_accepted`,
  `presenter_role_declined`, `presenter_role_auto_observer`,
  `presenter_role_revoked`, `presenter_role_expired_session_end`) names
  where the role landed. `presenter_role_accepted` requires a
  `destination_actor_ref`; `presenter_role_declined` requires a
  `decline_reason_label`; `presenter_role_revoked` requires a
  `revocation_cause_label`.
- Control-state transitions and presenter-handoff outcomes mint typed
  [`SharedTerminalAuditEvent`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
  rows on the local audit stream so the session UI, support export, and
  admin-collaboration surfaces inspect the same event ids. Denial events
  (`control_request_denied`, `audit_denial_emitted`) MUST cite a
  `denial_reason_label`.
- When control ends or degrades, the bound row records a
  [`LocalTerminalContinuityObservation`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
  using a closed
  [`LocalContinuityClass`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
  vocabulary so local terminal continuity is preserved without silent
  authority widening and without in-flight input replay.

## Control-state vocabulary

| Control state | Meaning | Required refs |
| --- | --- | --- |
| `view_only_observer` | Participant observes the transcript only | none (must NOT cite `control_grant_ref`) |
| `request_control_pending` | Request awaiting grantor admission | `pending_request_ref` |
| `active_control_grantee` | Participant holds an admitted grant | `control_grant_ref` |
| `control_revoked` | Grant revoked; mutation refused, observation continues | `control_grant_ref`, `revocation_ref`, `revocation_cause` |

## Presenter-handoff outcome vocabulary

| Outcome | Required ref |
| --- | --- |
| `presenter_role_accepted` | `destination_actor_ref` |
| `presenter_role_declined` | `decline_reason_label` |
| `presenter_role_auto_observer` | none (presenter stepped away) |
| `presenter_role_revoked` | `revocation_cause_label` |
| `presenter_role_expired_session_end` | none |

Every handoff cites the bound `presenter_state_ref` (the upstream
`presenter_state_record`) so reviewers can trace the role change.

## Audit-event vocabulary

| Event class | When it fires | Required field |
| --- | --- | --- |
| `control_requested` | Participant asked for control | `control_state_ref` |
| `control_request_admitted` | Owner / approver admitted the request | `control_state_ref` |
| `control_request_denied` | Owner / approver denied the request | `control_state_ref`, `denial_reason_label` |
| `control_active_started` | Active control session started | `control_state_ref` |
| `control_active_ended` | Active control session ended | `control_state_ref` |
| `control_revoked` | Control was revoked | `control_state_ref` |
| `control_expired_session_end` | Active control expired at session end (hard cap) | `control_state_ref` |
| `presenter_handoff_initiated` | Handoff invitation minted | `presenter_handoff_ref` |
| `presenter_handoff_resolved` | Handoff invitation resolved | `presenter_handoff_ref` |
| `audit_denial_emitted` | Denial emitted (e.g. replay after revoke) | `denial_reason_label` |

## Local-continuity vocabulary

| Continuity class | When it fires |
| --- | --- |
| `owner_input_preserved_after_grantee_revoked` | Grantee was revoked; owner's local input continues |
| `grantee_demoted_observer_no_input_injection` | Grantee demoted to observer; no silent input injection |
| `local_shell_resumed_after_relay_outage` | Relay outage forced non-replayable hand-back |
| `local_authority_restored_after_session_end` | Session ended; local pane returns to single-user authority |
| `local_authority_restored_after_expiry` | Single-shot grant spent or duration window closed; local authority returned |

Every continuity observation pins three guardrails closed:
`silent_authority_widening_taken=false`,
`local_terminal_continuity_preserved=true`, and
`in_flight_input_replayed=false`.

## Validator invariants

[`SharedTerminalAlphaPage::validate`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
enforces:

- `record_kind`, `schema_version`, and `shared_contract_ref` match the
  alpha constants on every page, state, presenter handoff, audit event,
  and continuity observation;
- state ids, handoff ids, audit-event ids, and observation ids are
  unique within a page;
- every state cites a complete `binding` (session, shared-object,
  terminal pane, execution context, host identity), a participant
  actor ref, and a non-empty rationale;
- `active_control_grantee` rows cite a `control_grant_ref`;
  `control_revoked` rows cite a `control_grant_ref`, `revocation_ref`,
  and `revocation_cause`; `request_control_pending` rows cite a
  `pending_request_ref`; view-only and pending rows MUST NOT cite a
  `control_grant_ref`;
- guardrails are pinned closed on every state and observation:
  `raw_terminal_bytes_present`, `raw_input_payload_present`, and
  `silent_authority_widening_taken` are all `false`;
  `local_terminal_continuity_preserved` is `true`;
  `in_flight_input_replayed` is `false`;
- presenter handoffs cite their required outcome field
  (`destination_actor_ref`, `decline_reason_label`, or
  `revocation_cause_label`) and (when resolved) a `resolved_at`
  timestamp;
- audit events for control changes cite a known `control_state_ref` and
  events for handoffs cite a known `presenter_handoff_ref`; denial
  events cite a `denial_reason_label`;
- continuity observations cite a known `bound_state_ref`;
- the page covers all four control states, the
  `presenter_role_accepted` and `presenter_role_auto_observer`
  handoff outcomes, and the `control_active_started`,
  `control_revoked`, and `presenter_handoff_resolved` audit-event
  classes.

The
[`SharedTerminalAlphaSupportExport`](../../../crates/aureline-runtime/src/shared_terminal_alpha/mod.rs)
projection drops upstream audit-event refs, raw terminal-byte
guardrails, and the in-flight replay guardrail so support bundles
inherit the redaction posture without per-surface review.

## Reviewer fixture

The protected fixture
[`/fixtures/runtime/shared_terminal_alpha/page.json`](../../../fixtures/runtime/shared_terminal_alpha/page.json)
covers:

- all four control states (`view_only_observer`,
  `request_control_pending`, `active_control_grantee` bound to a
  one-shot terminal grant, and `control_revoked` bound to a
  minutes-window grant revoked by the owner);
- two presenter handoffs (`presenter_role_accepted` and
  `presenter_role_auto_observer`);
- audit events for `control_requested`, `control_active_started`,
  `control_revoked`, `presenter_handoff_resolved` (both handoffs), and
  an `audit_denial_emitted` denial that names "Grant replay after
  revocation forbidden";
- continuity observations for
  `owner_input_preserved_after_grantee_revoked` and
  `grantee_demoted_observer_no_input_injection` after the revocation.

Run the fixture validator:

```bash
cargo test -p aureline-runtime --test shared_terminal_alpha
```

## Out of scope

This alpha lane lands the typed record family, the boundary schema, the
fixture, and the first consumer (the runtime crate plus its integration
test and this reviewer doc). Wiring the record into the session UI,
admin-collaboration console, and full-cloud presenter follow surfaces is
later work; this lane keeps the contract honest before those consumers
arrive and refuses to claim parity that is not yet observed.
