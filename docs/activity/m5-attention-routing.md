# Attention-routing matrix contract

This document freezes the object model behind Aureline's attention routing: the
notification envelope, durable activity object, badge aggregate, fanout receipt,
routing context, privacy class, and action/retention semantics. These are
governed product contracts, not ad hoc toast polish.

The matrix does not re-implement those objects. Each one already has a boundary
schema (under [`/schemas/ux/`](../../schemas/ux/) plus the sibling
[`/schemas/events/activity_row.schema.json`](../../schemas/events/activity_row.schema.json))
and at least one producing crate in the shell. The matrix is the single place
that **names the attention object families**, **freezes their stable identifiers
and required fields**, **maps each one to the proof packet that keeps it
current**, **pins one shared state vocabulary**, **defines the controlled
vocabulary** the attention plane reuses, **covers every fanout channel**, and
**states the invariants** every attention surface must hold — so docs, Help/About,
support, activity, and companion surfaces point at the same underlying objects
rather than re-expressing notification truth ad hoc.

The track invariant this lane protects: **attention is routed, typed,
privacy-aware, and reopen-safe.** No long-running or reviewable work lives only
in a toast; badges derive from deduped durable items; OS and companion fanout
cannot bypass the in-product preview/approval flow; suppression and quiet-hours
state stays separate from audit history; and every attention surface can reopen
the authoritative object instead of reissuing a blind side effect.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/activity/m5-attention-routing.schema.json`](../../schemas/activity/m5-attention-routing.schema.json)
  — boundary schema for `m5_attention_routing_matrix`.
- [`/fixtures/activity/m5-attention-routing/canonical_matrix.json`](../../fixtures/activity/m5-attention-routing/canonical_matrix.json)
  — the published canonical matrix; the freeze gate asserts the in-code builder
  equals it byte-for-byte.
- [`/artifacts/activity/m5-attention-routing.md`](../../artifacts/activity/m5-attention-routing.md)
  — the human-readable companion (object, channel, state, and invariant tables).
- `crates/aureline-activity/src/m5_attention_routing/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-activity --example dump_m5_attention_routing` — the
  headless emitter (JSON, or `-- --lines` for the projection).

## Attention object families

Each family cites the canonical boundary schema(s) it binds, the crate(s) that
already produce that truth, and the proof packet that keeps it current.

| Object token | Family | Bound schemas | Proof packet |
| --- | --- | --- | --- |
| `notification_envelope` | Notification envelope | notification_envelope, m5-os-notification-envelope | `docs/ux/notification_envelope_contract.md` |
| `activity_object` | Durable activity object | m5-activity-object, activity_row | `docs/ux/activity_center_alpha.md` |
| `badge_aggregate` | Badge aggregate | finalize-badge-semantics-cross-client-dedupe-admin-suppression | `docs/m5/notification-privacy-and-badges.md` |
| `fanout_receipt` | Fanout receipt | fanout_receipt | `docs/ux/notification_delivery_contract.md` |
| `routing_context` | Routing context | notification_route_outcome, notification_event | `docs/ux/notification_routing_seed.md` |
| `privacy_class` | Privacy class | notification_envelope, notification_suppression_record | `docs/ux/notification_privacy_dedupe_audit.md` |
| `action_retention_semantics` | Action / retention semantics | notification_suppression_record, notification_suppression_ledger, attention_inbox_item | `docs/ux/notification_action_grammar.md` |

Each object entry additionally carries: a stable `object_id`
(`attention_object.<token>`), the consumers that render it, the applicable states
from the shared vocabulary, the controlled-vocabulary axes it binds, its required
fields, a retention rule (including whether suppression state stays separate from
audit history), its default privacy class, redaction posture, and scope, whether
it is a durable authoritative record, the authoritative objects it can reopen,
and an attention-routing honesty note.

## Controlled vocabulary

The matrix defines eight controlled-vocabulary axes; every object declares which
it binds, and the `attention.controlled_vocabulary_complete` invariant fails if
any axis is bound by no object.

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

The shared vocabulary additionally pins the action semantics
(dismiss, snooze, acknowledge, resolve, mute), the retention classes, the
redaction classes, and the consumer classes, plus the union of bound source
schemas.

## Shared state vocabulary

One state vocabulary spans the notification lifecycle, durable-job progress,
cross-client delivery, and suppression / quiet-hours states. Each term carries
three computed flags — `requires_durable_object`, `is_delivery_gap`, and
`is_suppression` — and the upstream schema enum tokens it derives from.

`pending`, `routed`, `shown`, `acknowledged`, `snoozed`, `quiet_hours_deferred`,
`suppressed`, `running`, `queued_waiting`, `partially_completed`, `failed`,
`completed`, `resolved`, `dismissed`, `archived`, `fanout_stale`,
`fanout_undelivered`, `unknown_requires_review`.

## Fanout channels

The matrix covers the channels an attention object must stay truthful across. No
channel may bypass the in-product preview/approval flow
(`can_bypass_preview_approval` is `false` everywhere) and every channel respects
quiet-hours.

| Channel token | Delivery posture | Privacy ceiling | Mirrors authoritative |
| --- | --- | --- | --- |
| `in_app_activity_center` | durable_in_product | managed_sensitive | no |
| `os_native_notification` | out_of_window_mirror | summary_safe | yes |
| `dock_taskbar_badge` | coarse_count_only | summary_safe | yes |
| `browser_companion` | scoped_mirror | workspace_sensitive | yes |
| `mobile_companion` | scoped_mirror | summary_safe | yes |
| `operator_dashboard` | read_only_operator | managed_sensitive | yes |

## Invariants and release-automation binding

[`attention_routing_matrix`](../../crates/aureline-activity/src/m5_attention_routing/mod.rs)
computes each invariant's `holds` flag from the built objects, channels, and
states, so the checked-in fixture and the freeze gate freeze the contract
byte-for-byte and an inconsistent edit flips an invariant and fails CI.

The release-automation binding is the freeze gate
[`crates/aureline-activity/tests/m5_attention_routing.rs`](../../crates/aureline-activity/tests/m5_attention_routing.rs),
which runs under `cargo test --workspace`. The invariant
`attention.proof_packet_mapped` flips false the moment a claimed attention object
lacks a mapped proof packet, so stable promotion cannot harden an attention claim
without current proof on every named attention object.

The frozen invariants:

- `attention.canonical_object_identity` — every object cites a canonical schema
  and a producer.
- `attention.proof_packet_mapped` — every object maps to a non-empty proof
  packet.
- `attention.no_toast_only_truth` — no long-running or reviewable work lives only
  in a toast.
- `attention.badges_from_deduped_durable` — badges derive from deduped durable
  items, not raw event spam.
- `attention.fanout_cannot_bypass_preview_approval` — no fanout channel bypasses
  preview/approval.
- `attention.fanout_no_silent_failure` — stale and undelivered fanout are labeled,
  never silently dropped.
- `attention.suppression_separate_from_history` — suppression / quiet-hours state
  stays separate from audit history.
- `attention.reopen_authoritative` — every object reopens an authoritative target
  rather than reissuing a blind side effect.
- `attention.privacy_class_governed` — every object is privacy-class aware.
- `attention.envelope_routed_and_typed` — the envelope binds severity, scope,
  privacy class, and dedupe rule.
- `attention.controlled_vocabulary_complete` — every named controlled vocabulary
  is bound by an object.
- `attention.stable_ids_unique` — object ids, channel ids, and state tokens are
  unique.
- `attention.all_channels_covered` — every fanout channel is present.
- `attention.all_objects_present` — every object family is present exactly once.
- `attention.typed_not_toast_only` — every object is typed and locally
  inspectable.

## Export safety

The record carries no message bodies, credentials, raw provider payloads,
hostnames, or absolute paths — only opaque object refs, stable tokens, and short
reviewable sentences. `raw_payload_excluded` is always `true` and every ref is a
repo-relative object ref, so the matrix is safe to embed in a support export
verbatim.
