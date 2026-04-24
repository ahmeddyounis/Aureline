# Restricted-mode case fixtures

These fixtures anchor the workspace-trust, restricted-mode, and
permission-propagation contract seeded in
[`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../../../docs/adr/0018-workspace-trust-and-restricted-mode.md)
and validated by
[`/schemas/security/trust_decision_packet.schema.json`](../../../schemas/security/trust_decision_packet.schema.json).

They bind the vocabulary frozen in the ADR (trust states, entry-flow
transitions, remembered-decision scopes, reason classes, escalation
cues, audit-event ids, and per-state authority kinds) to concrete
worked cases so later tasks, terminal, debug, notebook, AI,
extension, connected-provider, remote-attach, and recovery lanes
can anchor on one shared packet instead of inventing parallel
surface-local trust conventions.

**Scope rules**

- Every fixture is one or more records validating against
  `trust_decision_record`, `trust_audit_event_record`, or
  `trust_matrix_inspection_record` in the boundary schema.
- Every fixture that narrows, denies, or forces a transition on
  policy / emergency / quarantine / recovery authority carries at
  least one `source_reason_refs` entry of the matching kind.
- `recovery_action_ref` is present only for
  `safe_mode_workspace_restricted` and
  `extension_quarantine_restricted` transitions; it is an opaque
  stand-in for a row in
  `schemas/support/recovery_action.schema.json` the recovery-ladder
  packet will publish.
- Raw filesystem paths, raw policy-bundle bytes, raw signing-key
  material, raw consent-capture bodies, and raw secret material
  MUST NOT appear anywhere.

**Index**

| Fixture | Transition | Resulting trust state | What it proves |
|---|---|---|---|
| [`first_open_untrusted_unknown.json`](./first_open_untrusted_unknown.json) | `initial_open_untrusted` | `untrusted_unknown` | First open of an unknown workspace resolves restricted posture with `request_trust_grant` offered as a recommendation; editor / search / save floor stays admitted. |
| [`open_in_restricted_mode.json`](./open_in_restricted_mode.json) | `open_in_restricted_mode` | `restricted` | Explicit user choice to open restricted; layout restore permitted; activators and tasks stay gated. |
| [`continue_in_restricted_mode_after_decline.json`](./continue_in_restricted_mode_after_decline.json) | `continue_in_restricted_mode` | `restricted` | User declined trust; session continues restricted with a `never_remembered` decline recorded; matrix-row audit event shows `blocked_pending_trust` for tasks. |
| [`open_without_restore.json`](./open_without_restore.json) | `open_without_restore` | `restricted` | Restricted posture with layout restore suspended; previously-open tasks / notebooks do not re-execute. |
| [`safe_mode_workspace_restricted.json`](./safe_mode_workspace_restricted.json) | `safe_mode_workspace_restricted` | `restricted_recovery_fallback` | Recovery-ladder forced restricted fallback after a crash loop; `recovery_action_ref` cites the recovery-ladder row; third-party extensions disabled. |
| [`grant_trust_session_bounded.json`](./grant_trust_session_bounded.json) | `grant_trust_session` | `trusted_time_bounded` | Explicit session-only grant with `expires_at`; matrix widens for tasks / debug / notebook under ticketed authority where policy narrows. |
| [`grant_trust_remembered_workspace_scope.json`](./grant_trust_remembered_workspace_scope.json) | `grant_trust_remembered` | `trusted` | Remembered-decision bound to the workspace root for the current user profile; survives restart; admin-policy ceiling holds. |
| [`policy_narrow_trusted_to_degraded.json`](./policy_narrow_trusted_to_degraded.json) | `policy_narrow_to_degraded` | `trusted_policy_degraded` | Admin policy bundle narrowed a trusted workspace; AI apply and extension activation downgrade from `allowed` to `approval_required_per_invocation`. |
| [`emergency_action_force_restricted.json`](./emergency_action_force_restricted.json) | `emergency_action_force_restricted` | `restricted` | Emergency-action bundle forced restricted posture; `source_reason_refs` cites the emergency-action record; escalation routes to recovery ladder. |
| [`identity_gate_unavailable.json`](./identity_gate_unavailable.json) | `identity_gate_unavailable` | `trust_unavailable_identity_gate` | Managed identity unreachable; workspace awaits identity; admin export shows stale-but-last-known-good posture; local editing continues. |
| [`matrix_inspection_snapshot.json`](./matrix_inspection_snapshot.json) | n/a | `trusted_policy_degraded` | `trust_matrix_inspection_record` snapshotting per-surface authority for a policy-degraded workspace; used by admin export, doctor probes, governance packets. |
