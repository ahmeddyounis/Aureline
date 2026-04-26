# Admin policy history row fixtures

These fixtures exercise `policy_decision_history_row_record` from
[`/schemas/admin/effective_policy_card.schema.json`](../../../schemas/admin/effective_policy_card.schema.json).
They are deliberately state-transition records, not generic event dumps.

| Fixture | Scenario | Transition preserved |
|---|---|---|
| [`settings_ai_provider_constrained.json`](./settings_ai_provider_constrained.json) | A current signed managed policy narrows a user-selected AI provider setting. | User-owned broad setting -> organization-owned constrained setting, with lock explanation and audit links. |
| [`remote_port_forwarding_offline_last_known_good.json`](./remote_port_forwarding_offline_last_known_good.json) | A managed mirror is offline, so the resolver uses last-known-good policy and pauses a new remote action. | Current mirrored action -> offline last-known-good pause, with endpoint posture and handoff links. |
| [`support_bundle_delete_blocked_by_hold.json`](./support_bundle_delete_blocked_by_hold.json) | A support-bundle delete request is denied because a support-investigation hold is active. | User-delete eligible archive -> policy-locked retained archive, with retention/deletion matrix and audit links. |

All examples use opaque refs, reviewable summaries, and redaction
summaries. They do not embed raw policy bodies, signatures, tenant
directory data, provider payloads, hostnames, paths, logs, or secret
material.
