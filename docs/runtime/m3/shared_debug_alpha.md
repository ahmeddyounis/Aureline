# Alpha Shared-Debug Control-Channel with Explicit Grants, Follow State, and Presenter-Handoff Audit

This document is the reviewer-facing landing page for the alpha shared-debug
control-channel record family: every shared debug session that any other
actor can observe, follow, or drive is projected through one
[`SharedDebugAlphaPage`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
that distinguishes view-only, follow, request-control, active-control, and
revoked states; records presenter-handoff, follow-engage / follow-release,
and control-change audit events; and preserves local debug authority when
control ends or degrades.

The machine-readable boundary lives at
[`/schemas/runtime/shared_debug_control_alpha.schema.json`](../../../schemas/runtime/shared_debug_control_alpha.schema.json).
The Rust implementation lives at
[`/crates/aureline-runtime/src/shared_debug_alpha/`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs).
The protected fixture lives at
[`/fixtures/runtime/shared_debug_alpha/page.json`](../../../fixtures/runtime/shared_debug_alpha/page.json).

This lane is the debug-session parallel of the
[shared-terminal control-channel alpha](./shared_terminal_alpha.md): the same
control-grant boundary applies, with debug-specific vocabulary for follow
state, audit events, and local-pane continuity.

## The alpha promise

- Every participant on a shared debug session resolves to exactly one
  [`SharedDebugControlState`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
  in one of five typed states:
  `view_only_observer`, `follow_presenter_observer`,
  `request_control_pending`, `active_control_grantee`, or `control_revoked`.
  Control is **never** inferred from presence, follow state, or presenter
  state — an `active_control_grantee` row that does not cite a
  `control_grant_ref` on the upstream
  [`control_grant_record`](../../../schemas/collaboration/control_grant.schema.json)
  is refused at validate time, mirroring the
  `control_grant_not_inferable_from_presence_or_presenter_state`
  invariant on the upstream control-grant contract.
- A `follow_presenter_observer` row MUST cite the followed actor with
  `follow_target_actor_ref` and never carries input authority or
  breakpoint-mutation authority; switching from follow to active control
  requires a typed grant audit-event chain.
- Presenter handoff is recorded as an explicit
  [`PresenterHandoffEvent`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
  whose typed outcome (`presenter_role_accepted`,
  `presenter_role_declined`, `presenter_role_auto_observer`,
  `presenter_role_revoked`, `presenter_role_expired_session_end`) names
  where the role landed. `presenter_role_accepted` requires a
  `destination_actor_ref`; `presenter_role_declined` requires a
  `decline_reason_label`; `presenter_role_revoked` requires a
  `revocation_cause_label`.
- Control-state transitions, follow engage / release, and presenter-
  handoff outcomes mint typed
  [`SharedDebugAuditEvent`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
  rows on the local audit stream so the session UI, support export, and
  admin-collaboration surfaces inspect the same event ids. Denial events
  (`control_request_denied`, `audit_denial_emitted`) MUST cite a
  `denial_reason_label`.
- When control ends or degrades, the bound row records a
  [`LocalDebugContinuityObservation`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
  using a closed
  [`LocalDebugContinuityClass`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
  vocabulary so local debug authority is preserved without silent
  authority widening and without in-flight stepping, breakpoint, or
  evaluate-input replay.

## Control-state vocabulary

| Control state | Meaning | Required refs |
| --- | --- | --- |
| `view_only_observer` | Participant observes the debugger transcript only; not following | none (must NOT cite `control_grant_ref` or `follow_target_actor_ref`) |
| `follow_presenter_observer` | Participant follows another actor's debugger view (stepping, caret, breakpoint focus) | `follow_target_actor_ref` (must NOT cite `control_grant_ref`) |
| `request_control_pending` | Request awaiting grantor admission | `pending_request_ref` |
| `active_control_grantee` | Participant holds an admitted debug grant | `control_grant_ref` |
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
| `follow_engaged` | Participant engaged follow mode against a presenter | `control_state_ref` |
| `follow_released` | Participant released follow mode back to passive view | `control_state_ref` |
| `presenter_handoff_initiated` | Handoff invitation minted | `presenter_handoff_ref` |
| `presenter_handoff_resolved` | Handoff invitation resolved | `presenter_handoff_ref` |
| `audit_denial_emitted` | Denial emitted (e.g. replay after revoke) | `denial_reason_label` |

## Local-continuity vocabulary

| Continuity class | When it fires |
| --- | --- |
| `owner_debug_authority_preserved_after_grantee_revoked` | Grantee was revoked; owner's local debug authority continues |
| `grantee_demoted_observer_no_step_injection` | Grantee demoted to observer; no silent step or breakpoint injection |
| `follow_observer_decoupled_after_relay_outage` | Relay outage decoupled follow; observer kept its last observed frame |
| `local_debug_authority_restored_after_session_end` | Session ended; local pane returns to single-user authority |
| `local_debug_authority_restored_after_expiry` | Single-shot grant spent or duration window closed; local authority returned |

Every continuity observation pins three guardrails closed:
`silent_authority_widening_taken=false`,
`local_debug_continuity_preserved=true`, and
`in_flight_debug_input_replayed=false`.

## Validator invariants

[`SharedDebugAlphaPage::validate`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
enforces:

- `record_kind`, `schema_version`, and `shared_contract_ref` match the
  alpha constants on every page, state, presenter handoff, audit event,
  and continuity observation;
- state ids, handoff ids, audit-event ids, and observation ids are
  unique within a page;
- every state cites a complete `binding` (session, shared-object,
  debug-session, execution context, host identity), a participant
  actor ref, and a non-empty rationale;
- `active_control_grantee` rows cite a `control_grant_ref`;
  `control_revoked` rows cite a `control_grant_ref`, `revocation_ref`,
  and `revocation_cause`; `request_control_pending` rows cite a
  `pending_request_ref`; `follow_presenter_observer` rows cite a
  `follow_target_actor_ref`; view-only, follow, and pending rows MUST
  NOT cite a `control_grant_ref`; non-follow rows MUST NOT cite a
  `follow_target_actor_ref`;
- guardrails are pinned closed on every state and observation:
  `raw_stack_frames_present`, `raw_variable_payload_present`,
  `raw_breakpoint_expression_present`, and
  `silent_authority_widening_taken` are all `false`;
  `local_debug_continuity_preserved` is `true`;
  `in_flight_debug_input_replayed` is `false`;
- presenter handoffs cite their required outcome field
  (`destination_actor_ref`, `decline_reason_label`, or
  `revocation_cause_label`) and (when resolved) a `resolved_at`
  timestamp;
- audit events for control changes and follow engage / release cite a
  known `control_state_ref` and events for handoffs cite a known
  `presenter_handoff_ref`; denial events cite a `denial_reason_label`;
- continuity observations cite a known `bound_state_ref`;
- the page covers all five control states, the
  `presenter_role_accepted` and `presenter_role_auto_observer`
  handoff outcomes, and the `control_active_started`,
  `control_revoked`, `follow_engaged`, and
  `presenter_handoff_resolved` audit-event classes.

The
[`SharedDebugAlphaSupportExport`](../../../crates/aureline-runtime/src/shared_debug_alpha/mod.rs)
projection drops upstream audit-event refs, raw stack-frame /
variable-payload / breakpoint-expression guardrails, and the in-flight
replay guardrail so support bundles inherit the redaction posture
without per-surface review.

## Reviewer fixture

The protected fixture
[`/fixtures/runtime/shared_debug_alpha/page.json`](../../../fixtures/runtime/shared_debug_alpha/page.json)
covers:

- all five control states (`view_only_observer`,
  `follow_presenter_observer` bound to the active driver,
  `request_control_pending`, `active_control_grantee` bound to a
  one-shot debug grant, and `control_revoked` bound to a
  minutes-window grant revoked by the owner);
- two presenter handoffs (`presenter_role_accepted` and
  `presenter_role_auto_observer`);
- audit events for `control_requested`, `control_active_started`,
  `control_revoked`, `follow_engaged`, `follow_released`,
  `presenter_handoff_resolved` (both handoffs), and an
  `audit_denial_emitted` denial that names "Debug grant replay after
  revocation forbidden";
- continuity observations for
  `owner_debug_authority_preserved_after_grantee_revoked` and
  `grantee_demoted_observer_no_step_injection` after the revocation.

Run the fixture validator:

```bash
cargo test -p aureline-runtime --test shared_debug_alpha
```

## Acceptance posture

- Shared debug sessions declare view, follow, control, and revocation
  states; presenter handoffs and revocations are preserved on the local
  audit stream.
- No shared-debug lane silently escalates from viewing (or following) to
  controlling a session: the validator refuses any transition that lacks
  a typed control_grant_ref and audit-event chain.
- Degradation to local-only debugging is preserved and recoverable via
  the closed `LocalDebugContinuityClass` vocabulary, with the no-replay
  invariant on in-flight stepping, breakpoints, and evaluate-input
  pinned closed.

## Out of scope

This alpha lane lands the typed record family, the boundary schema, the
fixture, and the first consumer (the runtime crate plus its integration
test and this reviewer doc). Wiring the record into the session UI,
admin-collaboration console, and full-cloud presenter follow surfaces is
later work; this lane keeps the contract honest before those consumers
arrive and refuses to claim parity that is not yet observed.
