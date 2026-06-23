# Envelope-routing bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-envelope-routing/canonical_bundle.json`](../../fixtures/activity/m5-envelope-routing/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-envelope-routing.schema.json`](../../schemas/activity/m5-envelope-routing.schema.json).
It gives reviewers the frozen producer, envelope, context, and invariant tables
without reading the JSON. The contract narrative lives in
[`/docs/activity/m5-envelope-routing.md`](../../docs/activity/m5-envelope-routing.md).

- Bundle id: `m5-envelope-routing:bundle:0001`
- Record kind: `m5_envelope_routing_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Producers: 12 · Envelopes: 12 · Contexts: 6 · Decisions: 72 · Invariants: 15

## Producer registry

Every claimed M5 producer routes through the typed envelope path; none retains
surface-local toast/banner/badge logic.

| Subsystem | Producer | Emits envelope | Routes typed | Surface-local logic |
| --- | --- | --- | --- | --- |
| `shell` | `producer:shell.command_result` | `shell.command_result` | yes | no |
| `notebook` | `producer:notebook.cell_run` | `notebook.cell_run` | yes | no |
| `task_runner` | `producer:task.run_failed` | `task.run_failed` | yes | no |
| `ai` | `producer:ai.awaiting_approval` | `ai.awaiting_approval` | yes | no |
| `collaboration` | `producer:collab.review_requested` | `collab.review_requested` | yes | no |
| `incident` | `producer:incident.thread_opened` | `incident.thread_opened` | yes | no |
| `operator` | `producer:operator.fleet_alert` | `operator.fleet_alert` | yes | no |
| `managed_policy` | `producer:managed.policy_changed` | `managed.policy_changed` | yes | no |
| `companion` | `producer:companion.fanout_status` | `companion.fanout_status` | yes | no |
| `security` | `producer:security.credential_revoked` | `security.credential_revoked` | yes | no |
| `sync` | `producer:sync.restore_complete` | `sync.restore_complete` | yes | no |
| `support` | `producer:support.export_ready` | `support.export_ready` | yes | no |

## Envelope corpus

Each envelope carries a source subsystem, scope, privacy class, severity, dedupe
strategy, and a stable action target. A `*` marks an action that routes through the
in-product preview/approval flow.

| Subsystem | Severity | Scope | Privacy | Dedupe | Action |
| --- | --- | --- | --- | --- | --- |
| `shell` | minor_success | window | summary_safe | latest_supersedes | open |
| `notebook` | progress | session | workspace_sensitive | canonical_key_coalesce | open |
| `task_runner` | degraded | session | workspace_sensitive | root_cause_collapse | retry |
| `ai` | handoff_actionable | session | workspace_sensitive | latest_supersedes | review_approve\* |
| `collaboration` | handoff_actionable | collaboration | workspace_sensitive | canonical_key_coalesce | review_approve\* |
| `incident` | handoff_actionable | workspace | security_critical | root_cause_collapse | open |
| `operator` | security_advisory | tenant_org | managed_sensitive | count_rollup | open |
| `managed_policy` | informational | tenant_org | managed_sensitive | latest_supersedes | review_approve\* |
| `companion` | informational | app_global | summary_safe | latest_supersedes | open |
| `security` | security_advisory | app_global | security_critical | no_dedupe | open |
| `sync` | minor_success | workspace | workspace_sensitive | latest_supersedes | open |
| `support` | minor_success | app_global | summary_safe | latest_supersedes | open |

## Routing contexts

| Context | Active window | DND / present | Screen reader | Role | User / admin policy | Quiet-hours |
| --- | --- | --- | --- | --- | --- | --- |
| `default_focused` | app_foreground_focused | off / off | off | solo | all_allowed / unmanaged | no |
| `background_quiet_hours` | app_background | off / off | off | solo | all_allowed / unmanaged | yes |
| `presenting_dnd` | app_foreground_unfocused | on / presenting | off | owner | all_allowed / managed_default | no |
| `managed_locked_owner` | app_foreground_focused | off / off | off | owner | all_allowed / managed_locked | no |
| `screen_reader_reviewer` | app_foreground_unfocused | off / off | on | reviewer | important_only / managed_default | no |
| `guest_muted` | app_background | off / off | off | guest | muted / managed_restricted | no |

## Worked routing — `ai.awaiting_approval` (preview/approval-gated)

The same envelope and action target on every surface; the in-app activity center
is always a durable delivery, and the gated action never executes inline on
fanout.

| Context | in-app | OS | browser | mobile |
| --- | --- | --- | --- | --- |
| `default_focused` | deliver | deliver_redacted | deliver_redacted | deliver_redacted |
| `background_quiet_hours` | deliver | defer_quiet_hours | defer_quiet_hours | defer_quiet_hours |
| `presenting_dnd` | deliver | defer_focus | defer_focus | defer_focus |
| `managed_locked_owner` | deliver | deliver_redacted | suppressed_by_admin_policy | suppressed_by_admin_policy |
| `guest_muted` | deliver | suppressed_by_user_policy | suppressed_by_user_policy | suppressed_by_user_policy |

A `security_advisory` envelope (e.g. `security.credential_revoked`) instead breaks
through quiet-hours, focus, and mute with a redacted summary on every recommended
surface, while its full payload stays in-product.

## Computed invariants (all hold)

| Invariant |
| --- |
| `envelope.every_producer_routes_typed` |
| `envelope.required_fields_present` |
| `envelope.stable_action_target_shared` |
| `envelope.durable_record_always` |
| `envelope.suppression_separate_from_durable` |
| `envelope.privacy_never_widens_on_fanout` |
| `envelope.fanout_cannot_bypass_preview_approval` |
| `envelope.routing_reproducible` |
| `envelope.context_inputs_complete` |
| `envelope.copy_localizable_not_contract` |
| `envelope.support_export_safe` |
| `envelope.consumer_parity` |
| `envelope.matrix_bound` |
| `envelope.action_target_reopens_authoritative` |
| `envelope.recommended_surfaces_handled` |

The freeze gate `crates/aureline-activity/tests/m5_envelope_routing.rs` rebuilds
the bundle in code and asserts it equals this fixture byte-for-byte; an
inconsistent edit flips an invariant or fails the round-trip.
