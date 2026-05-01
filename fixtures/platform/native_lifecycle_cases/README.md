# Native-lifecycle drill cases

Reviewer-side worked fixtures for
[`/artifacts/platform/native_lifecycle_drill_packet.md`](../../../artifacts/platform/native_lifecycle_drill_packet.md).
Each fixture binds one drill row from the packet's §4 to:

- one upstream `system_affordance_case_record` from
  [`/fixtures/platform/system_affordance_cases/`](../system_affordance_cases/),
- one upstream `window_display_verification_case` from
  [`/fixtures/platform/window_display_cases/`](../window_display_cases/),
  and / or
- one upstream `notification_suppression_record` /
  `desktop_summary_affordance_record` from
  [`/fixtures/ux/notification_privacy_cases/`](../../ux/notification_privacy_cases/)
  or
  [`/fixtures/ux/os_notification_cases/`](../../ux/os_notification_cases/).

These fixtures are review evidence. They do not mint vocabulary; every
axis is a re-export of the upstream schemas frozen in
[`/schemas/platform/deep_link_intent.schema.json`](../../../schemas/platform/deep_link_intent.schema.json),
[`/schemas/platform/window_state.schema.json`](../../../schemas/platform/window_state.schema.json),
and
[`/schemas/ux/notification_suppression_record.schema.json`](../../../schemas/ux/notification_suppression_record.schema.json).

## Fixture rules

- Every fixture names exactly one `drill_case_class` from the closed
  set in the drill packet's §4.
- Every fixture names exactly one `lifecycle_state_class` from the
  upstream `deep_link_intent.schema.json` enumeration.
- Every fixture names exactly one `expected_state_token` from the
  closed eight in the drill packet's §3.
- Every fixture cites at least one upstream fixture ref under
  `upstream_fixture_refs`. A fixture that does not bind to an upstream
  record is non-conforming.
- Every fixture asserts `lifecycle_recovery_proof.preserves_context`,
  `destructive_cleanup_forbidden`, and `hidden_focus_steal_forbidden`
  are `true`. Lock-screen rows additionally enumerate
  `forbidden_shortcut_action_classes` from the upstream notification
  schema.
- Free text appears only in `__fixture__.scenario`, in `safe_actions`
  short labels, and in `must_not_happen` short labels. Every other
  field resolves to a closed vocabulary or a stable opaque ref.
- Raw URLs, raw paths, raw provider payloads, raw secret material,
  raw prompt or completion text, and customer-owned identifiers never
  appear in any field.

## Cases

| Fixture | Drill case class | Lifecycle state | Expected state token |
|---|---|---|---|
| [`wake_from_sleep_local_context_preserved.yaml`](./wake_from_sleep_local_context_preserved.yaml) | `wake_from_sleep_local_context_preserved` | `wake_from_sleep` | `Reconnecting` |
| [`network_transition_remote_unreachable.yaml`](./network_transition_remote_unreachable.yaml) | `network_transition_remote_unreachable` | `wake_from_sleep` | `Reconnecting` then `Local fallback` |
| [`display_detach_reattach_safe_bounds.yaml`](./display_detach_reattach_safe_bounds.yaml) | `display_detach_reattach_safe_bounds` | `display_reconnect` | `Layout adjusted` (`Safe bounds restored`) |
| [`dpi_change_layout_adjusted.yaml`](./dpi_change_layout_adjusted.yaml) | `dpi_change_layout_adjusted` | `dpi_or_topology_change` | `Layout adjusted` |
| [`removable_volume_loss_root_unavailable.yaml`](./removable_volume_loss_root_unavailable.yaml) | `removable_volume_loss_root_unavailable` | `unavailable_target` | `Root unavailable` |
| [`removable_volume_return_review_required.yaml`](./removable_volume_return_review_required.yaml) | `removable_volume_return_review_required` | `removable_volume_return` | `Reopen required` then `Local fallback` |
| [`expired_callback_after_sleep_reopen_required.yaml`](./expired_callback_after_sleep_reopen_required.yaml) | `expired_callback_after_sleep_reopen_required` | `expired_session` | `Reopen required` (`Expired session`) |
| [`lock_screen_notification_suppressed_for_privacy.yaml`](./lock_screen_notification_suppressed_for_privacy.yaml) | `lock_screen_notification_suppressed_for_privacy` | `lock_screen_privacy` | `Notification suppressed for privacy` |

## Coverage contract

The seeded fixture set keeps:

- at least one wake-from-sleep case that proves local context survives
  and live authority advertises Reconnecting;
- at least one network-transition case that proves Local fallback is
  explicit and pending remote writes are not replayed;
- at least one display-topology case that proves windows return to
  safe bounds and dialog ownership survives;
- at least one DPI / scale-bucket case that proves readable scale,
  sheet ownership, and keyboard recovery remain intact;
- at least one removable-volume loss case that proves the missing
  pane degrades to a placeholder without destructive cleanup;
- at least one removable-volume return case that proves identity is
  reconciled and writes stay blocked until review;
- at least one expired-callback case that proves replay is denied
  and reauth routes through the in-product surface;
- at least one lock-screen privacy case that proves OS payloads are
  redacted and high-risk shortcut actions are forbidden.
