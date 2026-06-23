# Fanout-receipts bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-fanout-receipts/canonical_bundle.json`](../../fixtures/activity/m5-fanout-receipts/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-fanout-receipts.schema.json`](../../schemas/activity/m5-fanout-receipts.schema.json).
It gives reviewers the frozen destination, source, condition, and dispatch tables without
reading the JSON. The contract narrative lives in
[`/docs/activity/m5-fanout-receipts.md`](../../docs/activity/m5-fanout-receipts.md).

- Bundle id: `m5-fanout-receipts:bundle:0001`
- Record kind: `m5_fanout_receipts_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Destinations: 3 · Sources: 5 · Conditions: 8 · Dispatches: 40 · Receipts: 120 · Invariants: 16

## Governed destinations

The three out-of-window mirror surfaces this lane mints receipts for. The in-app activity
center is the authoritative durable record, not a fanout copy, and is out of scope here.

| Destination | Client scope | Durable authoritative |
| --- | --- | --- |
| `os_native_notification` | `os_primary_endpoint` | no |
| `browser_companion` | `browser_companion_session` | no |
| `mobile_companion` | `mobile_companion_device` | no |

## Source corpus

A representative set of attention sources across subsystems, severities, privacy classes,
reopen targets, and preview/approval postures.

| Source | Subsystem | Severity | Privacy | Reopen target | Preview/approval |
| --- | --- | --- | --- | --- | --- |
| `task.completed` | task_runner | minor_success | summary_safe | activity_job_row | no |
| `ai.awaiting_approval` | ai | handoff_actionable | workspace_sensitive | review_request | yes |
| `incident.flagged` | incident | handoff_actionable | workspace_sensitive | incident_thread | no |
| `route.policy_warning` | managed_policy | handoff_actionable | managed_sensitive | policy_diff | yes |
| `security.credential_revoked` | security | security_advisory | security_critical | audit_event | no |

## Transport-condition corpus

Each condition fixes the per-destination transport context for a dispatch, so the corpus
exercises every delivery state, reason, and posture.

| Condition | OS notification | Browser companion | Mobile companion |
| --- | --- | --- | --- |
| `all_delivered` | delivered | delivered | delivered |
| `mobile_stale` | delivered | delivered | stale |
| `companion_undelivered` | delivered | undelivered | delivered |
| `os_timed_out` | undelivered | delivered | delivered |
| `locked_screen` | delivered (lock-screen-safe) | delivered (lock-screen-safe) | delivered (lock-screen-safe) |
| `managed_endpoint_blocked` | undelivered | undelivered | undelivered |
| `policy_withheld` | suppressed | suppressed | suppressed |
| `transport_unknown` | unknown | delivered | delivered |

## Worked dispatches — OS notification delivery state per condition

The OS notification delivery state for each source under each condition. The companion
destinations follow the same engine.

| Source \\ Condition | all_delivered | mobile_stale | companion_undelivered | os_timed_out | locked_screen | managed_blocked | policy_withheld | transport_unknown |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `task.completed` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `ai.awaiting_approval` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `incident.flagged` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `route.policy_warning` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |
| `security.credential_revoked` | delivered | delivered | delivered | undelivered | delivered | undelivered | suppressed | unknown |

## Worked postures — OS notification summary posture per condition

The OS notification summary posture for each source under each condition. A delivered
managed-sensitive source renders `open_app_only`; a delivered sensitive source renders
`redacted_summary`; under `locked_screen` every above-summary-safe source renders
`lock_screen_safe`; every `undelivered` and `suppressed` copy renders `no_summary`.

| Source \\ Condition | all_delivered | locked_screen | os_timed_out | policy_withheld |
| --- | --- | --- | --- | --- |
| `task.completed` | clear_summary | clear_summary | no_summary | no_summary |
| `ai.awaiting_approval` | redacted_summary | lock_screen_safe | no_summary | no_summary |
| `incident.flagged` | redacted_summary | lock_screen_safe | no_summary | no_summary |
| `route.policy_warning` | open_app_only | lock_screen_safe | no_summary | no_summary |
| `security.credential_revoked` | redacted_summary | lock_screen_safe | no_summary | no_summary |

A failed (`os_timed_out`, `managed_endpoint_blocked`) copy is **undelivered** with an
explicit reason, never counted as delivered; a `policy_withheld` copy is **suppressed** with
a suppression reason kept distinct from a transport failure. In both cases the durable
in-product record still holds the attention, and every copy still reopens the source's exact
authoritative object — an approval-gated source (`ai.awaiting_approval`,
`route.policy_warning`) never executes its action inline.

## Computed invariants (all hold)

| Invariant |
| --- |
| `fanout.receipt_per_destination` |
| `fanout.binds_source_and_canonical_event` |
| `fanout.failures_labeled_never_counted_delivered` |
| `fanout.stale_undelivered_have_reason` |
| `fanout.privacy_safe_summary_default` |
| `fanout.lock_screen_safe` |
| `fanout.managed_endpoint_blocks_payload` |
| `fanout.reopen_parity` |
| `fanout.no_preview_approval_bypass` |
| `fanout.durable_record_present` |
| `fanout.suppressed_separate_from_failure` |
| `fanout.every_state_exercised` |
| `fanout.every_posture_exercised` |
| `fanout.dispatches_reproducible` |
| `fanout.matrix_bound` |
| `fanout.support_export_safe` |

The freeze gate `crates/aureline-activity/tests/m5_fanout_receipts.rs` rebuilds the bundle
in code and asserts it equals this fixture byte-for-byte; an inconsistent edit flips an
invariant or fails the round-trip.
