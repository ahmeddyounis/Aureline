# Notification Envelope Routing Beta Contract

This contract binds notification-envelope routing, stable action-target
semantics, and lock-screen-safe OS/companion handoff into one governed
routing system. It composes existing contracts instead of minting a second
notification vocabulary:

- [`../notification_envelope_contract.md`](../notification_envelope_contract.md)
- [`../notification_action_grammar.md`](../notification_action_grammar.md)
- [`../notification_delivery_contract.md`](../notification_delivery_contract.md)
- [`../notification_privacy_dedupe_audit.md`](../notification_privacy_dedupe_audit.md)
- [`../os_notification_and_quiet_hours_contract.md`](../os_notification_and_quiet_hours_contract.md)
- [`durable_attention_beta_contract.md`](durable_attention_beta_contract.md)
- [`notification_privacy_and_quiet_hours.md`](notification_privacy_and_quiet_hours.md)
- [`companion_scope_beta_contract.md`](companion_scope_beta_contract.md)

The schema of record for the routed object is
[`/schemas/ux/notification_route_outcome.schema.json`](../../../schemas/ux/notification_route_outcome.schema.json),
a governed projection over the frozen
[`notification_envelope`](../../../schemas/ux/notification_envelope.schema.json)
and [`fanout_receipt`](../../../schemas/ux/fanout_receipt.schema.json)
contracts.

## Contract Surface

The shell projection lives in
[`crates/aureline-shell/src/attention_router/`](../../../crates/aureline-shell/src/attention_router/).
It is a thin governance layer over the existing notification primitives in
[`crates/aureline-shell/src/notifications/`](../../../crates/aureline-shell/src/notifications/),
exported under the shared contract ref `shell:attention_router_beta:v1`:

- `notification_route_outcome_record` — the single governed object emitted
  per envelope under one live channel context. It resolves the same alert
  consistently across in-app toasts, banners, status overflow, the activity
  center, native OS notifications, and companion fanout.
- `shell_attention_routing_case_record` — one seeded routing case (scenario,
  emission count, and the resulting outcome).
- `shell_attention_routing_corpus_record` — the coverage corpus over every
  fanout surface and channel-resolution class.
- `shell_attention_route_support_export_record` /
  `shell_attention_route_support_export_row_record` — support-safe export of
  class, route, suppression, and outcome through structured enums rather than
  raw user-facing message text.

## The route outcome anatomy

A `notification_route_outcome_record` carries:

- The envelope identity it resolves (`source_notification_envelope_id_ref`,
  `canonical_event_id`, `event_lineage_id_ref`) plus severity, privacy,
  payload, redaction, and dedupe class copied verbatim from the envelope.
- A `channel_context` snapshot: `active_window_state`,
  `screen_reader_posture`, `companion_availability`,
  `presentation_follow_state`, and the effective `active_quiet_hours_modes`.
- The single `reopen_target` shared by every surface.
- `resolved_surface_routes[]`: per surface the `core_receipt_state` (the
  dedupe + suppression core's decision), the `resolved_receipt_state` (after
  live-channel narrowing), the `channel_resolution_class` explaining the
  difference, the stale/undelivered reason, suppression reasons, the reopen
  target ref, and the redaction class.
- `available_lifecycle_actions[]`: the six governed user verbs with badge,
  retention, and export semantics.
- `safe_action_target`: at most one open-only, non-destructive action.
- `companion_handoff`: summary-first posture with the forbidden shortcut
  classes the summary refuses to complete.
- Proof booleans: `durable_truth_preserved`,
  `all_routes_preserve_reopen_target`, `screen_reader_announce_required`,
  `screen_reader_navigable_surface_present`, and `no_generic_home_reopen`.

## Routing rules

The router never widens authority. It folds the live channel context into an
effective quiet posture, applies it to a clone of the envelope, routes through
the dedupe + suppression core, then only narrows each surface:

| Live context | Effect on routing |
| --- | --- |
| `foreground_focused` | The redundant OS notification is dropped (`suppressed_foreground_redundant`); in-app surfaces deliver. |
| `foreground_unfocused` / `background_hidden` | The OS notification delivers; the lock-screen summary is not applicable. |
| `locked_or_away` | The lock-screen summary is the external path, subject to the envelope's payload class. |
| `screen_reader = active` | A navigable durable surface must be present and the announcement is required. |
| `companion = paired_available` | The companion push delivers summary-first. |
| `companion = unpaired` / `paired_unavailable` | The companion push is not attempted but stays a visible receipt. |
| `companion = policy_blocked` | The companion push is suppressed by policy. |
| `presenting` / `following_presenter` | Folds into presentation mode; audience-visible surfaces are held while durable truth flows. |
| quiet hours / DND / admin suppression | Attention surfaces are held or suppressed; durable surfaces still deliver. |

A held, suppressed, or deduped core decision is **never** upgraded back to
delivered. Dedupe always wins: a repeat of an already-delivered surface
coalesces and keeps the same reopen target ref.

## Action grammar

Dismiss, snooze, acknowledge, mute, clear, and resolve are distinct verbs
with stable badge, retention, and export semantics. The behavioral
transitions live on
[`crate::notifications::actions::NotificationAttentionState`](../../../crates/aureline-shell/src/notifications/actions.rs);
the router publishes the descriptor grammar:

| Verb | Badge effect | Retention effect | Mutates source | Notes |
| --- | --- | --- | --- | --- |
| `dismiss` | leaves active badge | retains active row | no | Closes transient chrome only. |
| `acknowledge` | clears active badge | retains active row | no | Clears attention without touching the source. |
| `snooze` | moves to held count | retains active row | no | Requires a resume condition. |
| `mute` | clears active badge | retains active row | no | Requires a muted class/source. |
| `clear` | clears active badge | retains history only | no | Leaves the active inbox; reachable in history. |
| `resolve` | clears active badge | retains resolved record | yes | Resolves the source through its owning model. |

`suppress` is a system-side hold, not a user verb, and is excluded from the
published action set.

## Acceptance Rules

1. The same alert can be reasoned about consistently across toast, banner,
   status overflow, activity center, native OS notification, and companion
   surfaces — one `notification_route_outcome_record` per envelope.
2. No claimed beta notification path opens a generic home view or triggers a
   blind side effect: every reopen resolves to the exact object or a truthful
   placeholder, and `all_routes_preserve_reopen_target` plus
   `no_generic_home_reopen` hold for every outcome.
3. Lock-screen and compact-shell summaries stay privacy-safe by default and
   honor managed endpoint policy; OS and companion fanout is summary-first
   with `reopen_into_durable_object_required`.
4. Quiet hours and admin suppression delay interruption without erasing
   durable history, badge truth, or support-export lineage:
   `durable_truth_preserved` stays true.
5. The support export carries class, route, suppression, and outcome through
   support-safe enums and structured packets, never raw user-facing message
   text.

## Guardrails

Alerting stays subordinate to durable truth. The router does not create
shortcut actions that bypass preview, approval, or trust fences just because
the alert originated outside the main shell — the companion handoff publishes
the forbidden shortcut classes it refuses to complete, and the only external
`safe_action_target` is open-only and non-destructive.

## Conformance

The corpus, support export, and validation are minted from truth by the
headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_attention_router.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_attention_router.rs))
and replayed by
[`crates/aureline-shell/tests/attention_router_fixtures.rs`](../../../crates/aureline-shell/tests/attention_router_fixtures.rs)
against the fixtures under
[`fixtures/ux/m3/notification_routing/`](../../../fixtures/ux/m3/notification_routing/).
The route matrix is summarized in
[`/artifacts/ux/m3/notification_route_matrix.md`](../../../artifacts/ux/m3/notification_route_matrix.md).
