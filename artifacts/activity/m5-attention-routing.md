# Attention-routing matrix — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-attention-routing/canonical_matrix.json`](../../fixtures/activity/m5-attention-routing/canonical_matrix.json)
and its boundary schema
[`/schemas/activity/m5-attention-routing.schema.json`](../../schemas/activity/m5-attention-routing.schema.json).
It gives reviewers the frozen object, channel, state, and invariant tables
without reading the JSON. The contract narrative lives in
[`/docs/activity/m5-attention-routing.md`](../../docs/activity/m5-attention-routing.md).

- Matrix id: `m5-attention-routing:matrix:0001`
- Record kind: `m5_attention_routing_matrix`
- Objects: 7 · Channels: 6 · States: 18 · Invariants: 15

## Attention object families

| Object | Bound schemas | Default privacy | Default redaction | Durable | Proof packet |
| --- | --- | --- | --- | --- | --- |
| `notification_envelope` | notification_envelope, m5-os-notification-envelope | summary_safe | summary_only | yes | `docs/ux/notification_envelope_contract.md` |
| `activity_object` | m5-activity-object, activity_row | workspace_sensitive | metadata_safe_default | yes | `docs/ux/activity_center_alpha.md` |
| `badge_aggregate` | finalize-badge-semantics-cross-client-dedupe-admin-suppression | summary_safe | count_only | yes | `docs/m5/notification-privacy-and-badges.md` |
| `fanout_receipt` | fanout_receipt | summary_safe | redacted_payload | yes | `docs/ux/notification_delivery_contract.md` |
| `routing_context` | notification_route_outcome, notification_event | workspace_sensitive | summary_only | yes | `docs/ux/notification_routing_seed.md` |
| `privacy_class` | notification_envelope, notification_suppression_record | security_critical | redacted_payload | yes | `docs/ux/notification_privacy_dedupe_audit.md` |
| `action_retention_semantics` | notification_suppression_record, notification_suppression_ledger, attention_inbox_item | workspace_sensitive | metadata_safe_default | yes | `docs/ux/notification_action_grammar.md` |

## Controlled vocabulary (each axis bound by ≥1 object)

| Axis | Tokens |
| --- | --- |
| `severity` | minor_success, informational, degraded, progress, handoff_actionable, security_advisory |
| `scope` | app_global, window, workspace, session, collaboration, tenant_org |
| `privacy_class` | summary_safe, workspace_sensitive, security_critical, managed_sensitive |
| `dedupe_rule` | canonical_key_coalesce, root_cause_collapse, latest_supersedes, count_rollup, no_dedupe |
| `suppression` | user_muted_source, policy_suppressed, already_acknowledged, superseded, rate_limited |
| `quiet_hours` | defer_unless_opted_in, defer_to_in_product, may_bypass_with_policy, follow_admin_policy, always_defer |
| `fanout_delivery` | delivered, pending, stale, undelivered, superseded, suppressed_by_policy |
| `reopen_routing` | activity_job_row, evidence_packet, policy_diff, review_request, incident_thread, route_object, audit_event |

## Fanout channels

| Channel | Delivery posture | Privacy ceiling | Bypass preview/approval | Quiet-hours respected |
| --- | --- | --- | --- | --- |
| `in_app_activity_center` | durable_in_product | managed_sensitive | false | true |
| `os_native_notification` | out_of_window_mirror | summary_safe | false | true |
| `dock_taskbar_badge` | coarse_count_only | summary_safe | false | true |
| `browser_companion` | scoped_mirror | workspace_sensitive | false | true |
| `mobile_companion` | scoped_mirror | summary_safe | false | true |
| `operator_dashboard` | read_only_operator | managed_sensitive | false | true |

## Shared state vocabulary

| State | Requires durable object | Delivery gap | Suppression |
| --- | --- | --- | --- |
| `pending` | false | false | false |
| `routed` | false | false | false |
| `shown` | false | false | false |
| `acknowledged` | false | false | false |
| `snoozed` | true | false | false |
| `quiet_hours_deferred` | false | false | true |
| `suppressed` | false | false | true |
| `running` | true | false | false |
| `queued_waiting` | true | false | false |
| `partially_completed` | true | false | false |
| `failed` | true | false | false |
| `completed` | true | false | false |
| `resolved` | true | false | false |
| `dismissed` | false | false | false |
| `archived` | true | false | false |
| `fanout_stale` | false | true | false |
| `fanout_undelivered` | false | true | false |
| `unknown_requires_review` | false | false | false |

## Invariants

All 15 invariants compute their `holds` flag from the built matrix; the freeze
gate ([`crates/aureline-activity/tests/m5_attention_routing.rs`](../../crates/aureline-activity/tests/m5_attention_routing.rs))
fails if any flips false.

| Invariant | Holds |
| --- | --- |
| `attention.canonical_object_identity` | true |
| `attention.proof_packet_mapped` | true |
| `attention.no_toast_only_truth` | true |
| `attention.badges_from_deduped_durable` | true |
| `attention.fanout_cannot_bypass_preview_approval` | true |
| `attention.fanout_no_silent_failure` | true |
| `attention.suppression_separate_from_history` | true |
| `attention.reopen_authoritative` | true |
| `attention.privacy_class_governed` | true |
| `attention.envelope_routed_and_typed` | true |
| `attention.controlled_vocabulary_complete` | true |
| `attention.stable_ids_unique` | true |
| `attention.all_channels_covered` | true |
| `attention.all_objects_present` | true |
| `attention.typed_not_toast_only` | true |

## Regenerate

```sh
cargo run -p aureline-activity --example dump_m5_attention_routing \
  > fixtures/activity/m5-attention-routing/canonical_matrix.json
```
