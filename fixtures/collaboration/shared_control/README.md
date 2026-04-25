# Collaboration follow, presenter-state, and shared-control grant worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/collaboration/shared_control_contract.md`](../../../docs/collaboration/shared_control_contract.md)
and the schemas at
[`/schemas/collaboration/follow_and_presenter_state.schema.json`](../../../schemas/collaboration/follow_and_presenter_state.schema.json)
and
[`/schemas/collaboration/control_grant.schema.json`](../../../schemas/collaboration/control_grant.schema.json).

Every file is a YAML document carrying a `__fixture__` prelude
summarising the scenario, the contract sections it exercises, and
the record kinds it produces, plus a `records` array containing
individual `follow_target_record`, `presenter_state_record`,
`presenter_handoff_record`, `follow_and_presenter_audit_event_record`,
`control_grant_record`, `control_grant_revocation_record`, and
`control_grant_audit_event_record` instances that conform to the
schemas. The `record_kind` discriminator on each record names
which schema validates it (records from both schemas coexist in
the same `records` list; downstream tooling discriminates by kind).

No fixture embeds raw buffer text, raw terminal bytes, raw debug
payloads, raw URLs, raw absolute paths, raw user identifiers, raw
billing-account ids, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, raw pack bytes, or raw provider
payloads. Every such field is an opaque ref, a reviewable label,
or a coarse bucket.

## Cases

- [`presenter_broadcast_view_only_no_control.yaml`](./presenter_broadcast_view_only_no_control.yaml)
  — A presenter broadcasts focus to two observers. Each observer
  follows the broadcast channel and receives a view-only control
  grant over the `presenter_focus_lane` with
  `ceiling_read_only_no_mutation`. A denial event demonstrates
  that shared-terminal control CANNOT be inferred from the
  presenter or co-presenter badge (both the
  `control_grant_not_inferable_from_presence_or_presenter_state`
  and the `shared_terminal_or_debug_control_requires_explicit_grant`
  denial reasons fire on the respective audit streams). Acceptance
  bullet 3.
- [`terminal_grant_revoked_immediately.yaml`](./terminal_grant_revoked_immediately.yaml)
  — A `terminal_lane` grant minted under
  `grant_scope_single_action_one_shot` +
  `duration_single_action_one_shot` +
  `ceiling_mutation_within_named_action_set` with
  `approval_ticket_required`, admitting only
  `terminal_send_signal_allowed`. The grantee spends the one-shot
  action; a follow-up attempt denies with
  `single_action_grant_already_spent`; the owner revokes the grant
  with `owner_revoked`, `revocation_immediate_authority_expired_on_mint`,
  and `non_replayable_authority_expired_on_revocation`. Acceptance
  bullet 1.
- [`debug_grant_admin_signed_retained.yaml`](./debug_grant_admin_signed_retained.yaml)
  — A `debug_lane` grant minted under
  `admin_signed_required` + `approval_ticket_required` with
  `duration_bounded_minutes_window` and `expires_at`. Durability is
  `retained_full_payload_admin_signed_opt_in_only` citing the
  session-policy-manifest ref; full-payload retention is opt-in per
  admin signature and cannot be inferred from ordinary
  collaboration. Mid-grant workspace-trust narrows; the revocation
  cites `workspace_trust_narrowed` with the immediate-and-non-
  replayable invariants and discards an in-flight
  `debug_continue_allowed` action. Acceptance bullet 2.
- [`degraded_follow_no_input_injection.yaml`](./degraded_follow_no_input_injection.yaml)
  — A presenter broadcast session loses live state. The observer's
  `follow_target_record` is superseded with
  `follow_mode_class = follow_unavailable_live_state_missing` and
  `degraded_follow_posture_class = follow_only_summary_readout_offered`
  (then `invitation_or_handoff_fallback_offered`). A hidden-input-
  injection attempt denies with
  `hidden_input_injection_forbidden_during_degraded_follow`. No
  degraded-follow posture admits input injection. Acceptance
  bullet 3.
- [`replay_after_revocation_denied.yaml`](./replay_after_revocation_denied.yaml)
  — A `terminal_lane` grant under
  `duration_until_revoked_bounded_by_session` is revoked mid-session
  by `relay_outage_non_replayable`. A buffered
  `terminal_send_input_allowed` request that arrives after the
  revocation record denies with
  `grant_replay_after_revocation_forbidden`. Re-admission requires a
  fresh grant with a fresh approval ticket. Acceptance bullet 1.
- [`presenter_step_away_auto_observer.yaml`](./presenter_step_away_auto_observer.yaml)
  — A presenter steps away. A `presenter_handoff_record` is minted
  with `presenter_handoff_kind_class = presenter_step_away_auto_observer`,
  citing `admitting_owner_actor_ref`. The prior presenter is
  auto-downgraded to observer; the broadcast is paused until the
  owner admits a new presenter. No control grants flip as a side
  effect of the handoff; any shared-terminal or shared-debug grant
  in flight remains revoke-or-expire-only.
