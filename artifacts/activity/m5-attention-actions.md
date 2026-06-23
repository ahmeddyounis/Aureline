# Attention-actions bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-attention-actions/canonical_bundle.json`](../../fixtures/activity/m5-attention-actions/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-attention-actions.schema.json`](../../schemas/activity/m5-attention-actions.schema.json).
It gives reviewers the frozen action, item, and outcome tables without reading the
JSON. The contract narrative lives in
[`/docs/activity/m5-attention-actions.md`](../../docs/activity/m5-attention-actions.md).

- Bundle id: `m5-attention-actions:bundle:0001`
- Record kind: `m5_attention_actions_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Actions: 5 · Items: 8 · Outcomes: 26 · Invariants: 16

## Action grammar

Each action carries a distinct resulting state, retention class, badge effect, resume
kind, and effect scope — they never collapse into one generic close. Snooze and mute
are the only actions that record a deferral / suppression marker, stored separately
from audit history.

| Action | Resulting state | Retention | Badge effect | Resume | Scope | Suppression-separate |
| --- | --- | --- | --- | --- | --- | --- |
| `dismiss` | dismissed | `durable_until_archived` | `clear_keep_record` | none | this_item | false |
| `snooze` | snoozed | `suppression_state_separate` | `clear_until_resume` | timer_or_predicate | this_item | true |
| `acknowledge` | acknowledged | `durable_until_resolved` | `clear_mark_read` | none | this_item | false |
| `mute` | suppressed | `suppression_state_separate` | `clear_and_suppress_source` | until_unmuted | this_source | true |
| `resolve` | resolved | `durable_until_archived` | `clear_on_resolve` | none | this_item | false |

Every action keeps the underlying durable record, is audit-append-only, stays
reopenable, and replays no side effect.

## Attention-item corpus

A representative set of durable attention objects across object families and
subsystems. A security advisory supports only acknowledge and resolve — it can never
be silenced.

| Subsystem | Object family | Reopen target | Privacy | Supported actions |
| --- | --- | --- | --- | --- |
| `task_runner` | activity_object | `activity_job_row` | workspace_sensitive | dismiss, snooze, acknowledge, resolve |
| `ai` | notification_envelope | `review_request` | workspace_sensitive | snooze, acknowledge, mute, resolve |
| `collaboration` | notification_envelope | `review_request` | workspace_sensitive | dismiss, snooze, acknowledge, mute, resolve |
| `incident` | notification_envelope | `incident_thread` | security_critical | snooze, acknowledge, resolve |
| `managed_policy` | routing_context | `policy_diff` | managed_sensitive | acknowledge, resolve |
| `security` | notification_envelope | `audit_event` | security_critical | acknowledge, resolve |
| `shell` | notification_envelope | `route_object` | summary_safe | dismiss, acknowledge |
| `support` | notification_envelope | `evidence_packet` | summary_safe | dismiss, snooze, acknowledge, resolve |

## Worked outcomes — `collab.review_requested` (the full grammar)

The one item that supports all five actions. Each action transitions the same durable
object to a distinct state with distinct retention and badge behavior; the badge
always clears, the record is always kept, and the reopen route to the same
`review_request` is preserved without replaying a side effect.

| Action | Resulting state | Retention | Badge effect | Badge | Resume | Reopen target |
| --- | --- | --- | --- | --- | --- | --- |
| `dismiss` | dismissed | `durable_until_archived` | `clear_keep_record` | 6→5 | no | `review_request` |
| `snooze` | snoozed | `suppression_state_separate` | `clear_until_resume` | 6→5 | yes | `review_request` |
| `acknowledge` | acknowledged | `durable_until_resolved` | `clear_mark_read` | 6→5 | no | `review_request` |
| `mute` | suppressed | `suppression_state_separate` | `clear_and_suppress_source` | 6→5 | yes | `review_request` |
| `resolve` | resolved | `durable_until_archived` | `clear_on_resolve` | 6→5 | no | `review_request` |

Each outcome propagates the same resulting state across the in-app activity center
(authoritative), the dock / taskbar badge (clear count), the OS notification
(withdraw, no replay), and the browser / mobile companions (reflect state, no replay)
— one action model, never a local variant.

## Computed invariants (all hold)

| Invariant |
| --- |
| `action.five_distinct_actions` |
| `action.badge_effects_distinct` |
| `action.semantics_distinct` |
| `action.keeps_underlying_record` |
| `action.exact_reopen_continuity` |
| `action.no_side_effect_replay` |
| `action.surface_parity` |
| `action.suppression_separate_from_audit` |
| `action.resume_condition_present_iff_required` |
| `action.badge_clears_never_negative` |
| `action.support_export_explains_without_replay` |
| `action.security_not_silenceable` |
| `action.all_actions_exercised` |
| `action.outcomes_reproducible` |
| `action.matrix_bound` |
| `action.support_export_safe` |

The freeze gate `crates/aureline-activity/tests/m5_attention_actions.rs` rebuilds the
bundle in code and asserts it equals this fixture byte-for-byte; an inconsistent edit
flips an invariant or fails the round-trip.
