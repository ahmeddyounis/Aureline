# Deep-link replay-deny cases

Worked `deep_link_intent_record` fixtures that prove every
replay-denied posture in
[`/schemas/platform/deep_link_intent.schema.json`](../../../schemas/platform/deep_link_intent.schema.json)
fails closed with a typed denial reason and a fallback that either
preserves user intent (cached context, locate, review) or denies with
explanation.

The cases are the upstream contract for the replay-deny matrix in
[`/artifacts/platform/system_affordance_route_audit.md`](../../../artifacts/platform/system_affordance_route_audit.md)
§6 and the protocol-handler / browser-callback rows in
[`/artifacts/platform/file_association_ownership_matrix.yaml`](../../../artifacts/platform/file_association_ownership_matrix.yaml).

Every fixture binds to `deep_link_intent_record` (not
`system_affordance_case_record`) so the replay-deny posture is the
record's primary axis. A fixture that needs the broader system-
affordance shape lives under
[`/fixtures/platform/system_affordance_cases/`](../system_affordance_cases/)
instead.

## Required closure rules

For every fixture in this directory:

- `record_kind` is `deep_link_intent_record`.
- `replay_posture` is one of the closed `replay_denied_*` values.
- `policy_resolution_class` is `denied_replay`.
- `audit_event_id` is `platform.deep_link_intent_denied`.
- `degraded_reasons` includes `replay_denied`.
- `trust_review_requirement` is anything except `no_review_required`
  whenever `authority_delta_class` is non-`none`.
- `fallback.preserves_user_intent` is `true` for cached-context,
  locate, or review-sheet fallbacks; `false` only when the fallback
  is `deny_with_explanation` and intent preservation is not
  possible.
- `redaction_class`, `privacy_payload_class`, and the policy /
  handler-ownership context resolve to opaque refs only — raw URLs,
  raw paths, raw callback bodies, and raw secrets never appear in
  these fixtures.

## Coverage

| Replay-denied posture | Fixture |
|---|---|
| `replay_denied_consumed` | `auth_callback_replay_denied_consumed.yaml` |
| `replay_denied_expired` | `workspace_open_replay_denied_expired.yaml` |
| `replay_denied_policy_epoch_changed` | `managed_resume_replay_denied_policy_epoch_changed.yaml` |
| `replay_denied_target_drifted` | `review_link_replay_denied_target_drifted.yaml`, `dock_recent_replay_denied_target_drifted.yaml` |
| `replay_denied_origin_mismatch` | `browser_return_replay_denied_origin_mismatch.yaml`, `command_invocation_widened_authority_denied.yaml` |

A previously-existing `system_affordance_case_record` fixture for
`replay_denied_consumed` lives at
[`/fixtures/platform/system_affordance_cases/deep_link_remote_review_replay_denied.json`](../system_affordance_cases/deep_link_remote_review_replay_denied.json);
it complements the auth-callback case here.
