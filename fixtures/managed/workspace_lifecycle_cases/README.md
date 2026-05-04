# Managed-Workspace Lifecycle Cases

Worked fixtures for
[`/docs/managed/managed_workspace_lifecycle_contract.md`](../../../docs/managed/managed_workspace_lifecycle_contract.md)
and the boundary schema at
[`/schemas/managed/workspace_lifecycle_state.schema.json`](../../../schemas/managed/workspace_lifecycle_state.schema.json).

The cases use opaque refs for workspaces, instances, tickets,
evidence artifacts, and policies. Raw user emails, raw tenant names,
raw workspace volume ids, raw container image digests, raw provider
URLs, and raw provider account identifiers do not appear.

| Fixture | Phase | Coverage |
|---|---|---|
| `first_attach_after_provisioning.yaml` | `ready` | First-attach path; workspace_attached posture; persistence posture for ready. |
| `suspend_idle_then_resume.yaml` | `suspended` | Idle-suspend persistence posture; resume vs local-only continuation as distinct retry outcomes. |
| `expired_workspace_session_ticket.yaml` | `expired` | Session-ticket-expired expiry block; reauth and local-only continuation distinct outcomes. |
| `rebuild_review_after_successor_image.yaml` | `rebuild` | Rebuild-review required; committed-vs-uncommitted persistence split; rebuild and resume forbidden to combine. |
| `local_only_continuation_after_service_outage.yaml` | `degraded` | Control-plane outage degraded block; local-only admissible surfaces; typed managed-action blocking. |
