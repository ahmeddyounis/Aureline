# Break-glass event example fixtures

These fixtures anchor the two-person-control, break-glass audit, and
post-incident reconciliation contract frozen in
[`/docs/security/high_risk_control_quorum.md`](../../../docs/security/high_risk_control_quorum.md)
and validated by
[`/schemas/security/break_glass_event.schema.json`](../../../schemas/security/break_glass_event.schema.json).

They reuse the same advisory, emergency-action, revocation, and
private-triage packet refs seeded elsewhere under
`fixtures/security/` so one incident resolves into one id graph across
triage, freeze, revocation, and break-glass audit surfaces.

**Scope rules**

- Every fixture validates as `break_glass_event_record`.
- Each fixture exercises the full lifecycle envelope: invocation,
  must-close-by ceiling, scope (subjects plus install-profile-card plus
  deployment profile plus channel), produced emergency-action /
  revocation refs, private-triage packet linkage, and the applicable
  close-out state.
- Raw human identifiers (email, login, pager id, chat handle, hostname),
  raw signing material, raw quorum signatures, raw exploit payloads,
  and raw incident notes never appear. Actor accounts carry opaque refs
  only.
- `must_close_by` is always within 24 hours of `invoked_at`, matching
  the `audited_single_responder_containment` profile ceiling in
  `signing_quorum.yaml`.

**Index**

| Fixture | Lifecycle state | Produced record kind | What it proves |
|---|---|---|---|
| [`signed_channel_freeze_retrospective_cosign.yaml`](./signed_channel_freeze_retrospective_cosign.yaml) | `reconciled_with_retrospective_cosign` | `emergency_action_record` | admitted cause, 24 h envelope, two-person cross-forum cosign closes out the invocation on time |
| [`revocation_superseded_by_signed_action.yaml`](./revocation_superseded_by_signed_action.yaml) | `superseded_by_signed_action` | `revocation_record` | a single-responder revocation is replaced by a co-signed revocation; the break-glass event cites the superseding record and closes out |
| [`expired_without_reconciliation.yaml`](./expired_without_reconciliation.yaml) | `expired_without_reconciliation` | `emergency_action_record` | the invocation passed its 24 h ceiling without cosign or supersedence; the record surfaces a correction signal and names the follow-up plan |
| [`withdrawn_invalid_invocation.yaml`](./withdrawn_invalid_invocation.yaml) | `withdrawn_invalid_invocation` | `emergency_action_record` | an invocation whose underlying cause was not admissible is withdrawn explicitly; no silent rollback |
