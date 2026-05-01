# Native-lifecycle drill packet

Reviewer-side drill packet for desktop lifecycle events on the claimed
profiles in
[`/artifacts/platform/claimed_desktop_profiles.yaml`](./claimed_desktop_profiles.yaml).
This packet exists so wake-from-sleep, network transition,
display detach / reattach, DPI change, removable-volume loss / return,
and expired-callback-after-sleep behaviors are recorded as one
inspectable trail rather than as platform-specific folklore.

The packet is review evidence. It does not mint vocabulary. Every axis
is re-exported from the upstream schemas:

- [`/schemas/platform/deep_link_intent.schema.json`](../../schemas/platform/deep_link_intent.schema.json)
  - `system_affordance_case_record`, `lifecycle_state_class`,
    `target_availability_class`, `freshness_class`, `policy_resolution_class`,
    `replay_posture`, `fallback_class`, `outcome_class`, `audit_event_id`.
- [`/schemas/platform/window_state.schema.json`](../../schemas/platform/window_state.schema.json)
  - topology change classes, focus-return rules, restore-history
    adjustment classes, session-execution postures.
- [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  - suppression class, suppression outcome, privacy payload class,
    forbidden shortcut action class, exact-reopen linkage.

If this packet disagrees with
[`/docs/ux/desktop_affordance_contract.md`](../../docs/ux/desktop_affordance_contract.md),
[`/docs/ux/window_display_contract.md`](../../docs/ux/window_display_contract.md),
[`/docs/ux/os_notification_and_quiet_hours_contract.md`](../../docs/ux/os_notification_and_quiet_hours_contract.md),
or
[`/docs/platform/desktop_platform_conformance_matrix.md`](../../docs/platform/desktop_platform_conformance_matrix.md),
those sources win and this packet plus its companion fixtures update in
the same change.

## 1. Scope

A native-lifecycle drill is the closed inspection a reviewer runs when
the OS hands Aureline a wake, sleep, network, display, DPI, removable
volume, or expired-callback transition. The drill answers, for each
case:

- Which canonical command, object identity, and event lineage backed
  the in-product state at the moment the lifecycle transition fired.
- Which `lifecycle_state_class` was exercised.
- Which `expected_state_token` was observed (e.g. `Reconnecting`,
  `Layout adjusted`, `Root unavailable`, `Reopen required`,
  `Local fallback`, `Notification suppressed for privacy`).
- Which `safe_actions` were offered and whether any of them silently
  re-ran a privileged command, widened authority, stranded focus, or
  discarded local context.
- Which audit event id closed the trail.

A drill is conforming when every case row resolves to one fixture
under
[`/fixtures/platform/native_lifecycle_cases/`](../../fixtures/platform/native_lifecycle_cases/),
that fixture binds verbatim to one upstream
`system_affordance_case_record` or `window_display_verification_case`
fixture, and the drill's expected-state token is one of the eight
closed values in §3.

## 2. Out of scope

- Automated platform-lab infrastructure or per-OS adapter code. This
  packet is review evidence; OS adapter code lands in the eventual
  platform-integration crate.
- Final user-facing microcopy. This packet pins state tokens, audit
  event ids, and recovery action classes. Product writing chooses
  final strings inside those limits.
- New lifecycle classes, new replay postures, new fallback classes, new
  privacy payload classes, or new audit event ids. Every axis is
  re-exported from the upstream schemas; widening requires a decision
  row in the owning schema.

## 3. Closed expected-state token vocabulary

Every drill case names exactly one `expected_state_token` from the
closed eight. The token names the user-visible truth the in-product
surface MUST advertise after the transition.

| `expected_state_token` | When the case names it |
|---|---|
| `Reconnecting` | Live remote, callback, debug, or collaboration authority is held until revalidation completes; local edit context survives. |
| `Layout adjusted` | Window, sheet, or pane was moved, resized, or reflowed into safe bounds; pane identity, focus chain, and dialog ownership remain intact. |
| `Root unavailable` | A workspace root, mount point, or required dependency is missing; the pane slot becomes a placeholder with locate / reconnect / cached-context recovery. |
| `Reopen required` | An auth callback, deep-link token, managed-workspace session, or approval ticket has expired or drifted; reopen routes through revalidation, not silent replay. |
| `Local fallback` | Remote authority is unreachable; the user keeps local work with an explicit "continue local" or cached-context affordance. |
| `Notification suppressed for privacy` | Lock-screen, companion, or system-notification surface delivers a redacted summary or holds delivery; the durable in-product trail and exact-reopen linkage are preserved. |
| `Safe bounds restored` | Topology change moved an off-screen window or sheet back onto a reachable display while preserving pane intent and dialog ownership. |
| `Expired session` | An auth, managed, or provider session ended during sleep; in-product state names what paused, what remains local, and which reauth or reconnect action recovers. |

A row that needs a state outside the closed set opens a decision row
under the upstream schema instead of widening this table.

## 4. Required drill case classes

Every release evidence pack and support packet that cites this drill
packet MUST include at least one row per case class below. A claim
that is narrower (e.g. a profile that does not include removable
volumes) records the row as `not_applicable` with a typed reason
rather than omitting it.

| Drill case class | `lifecycle_state_class` | Default expected-state token | Required upstream fixture kinds |
|---|---|---|---|
| `wake_from_sleep_local_context_preserved` | `wake_from_sleep` | `Reconnecting` for live authority; `Local fallback` for offline edit | `system_affordance_case_record` plus `window_display_verification_case` |
| `network_transition_remote_unreachable` | `wake_from_sleep` (or `expired_session` if sessions also lapsed) | `Reconnecting` while reattach is held; `Local fallback` once the user accepts | `system_affordance_case_record` |
| `display_detach_reattach_safe_bounds` | `display_reconnect` | `Layout adjusted` or `Safe bounds restored` | `window_display_verification_case` |
| `dpi_change_layout_adjusted` | `dpi_or_topology_change` | `Layout adjusted` | `window_display_verification_case` |
| `removable_volume_loss_root_unavailable` | `unavailable_target` | `Root unavailable` | `system_affordance_case_record` |
| `removable_volume_return_review_required` | `removable_volume_return` | `Reopen required` then `Local fallback` until review completes | `system_affordance_case_record` |
| `expired_callback_after_sleep_reopen_required` | `expired_session` | `Reopen required` | `system_affordance_case_record` |
| `lock_screen_notification_suppressed_for_privacy` | `lock_screen_privacy` | `Notification suppressed for privacy` | `system_affordance_case_record` plus `notification_suppression_record` |

## 5. Per-case drill row

Reviewers fill one block per case row from §4. Free text is allowed
only in the `notes` field; every other field resolves to a closed
vocabulary or a stable opaque ref.

```yaml
- case_id: native-lifecycle-case:<short-slug>
  drill_case_class: <one of §4>
  lifecycle_state_class: <wake_from_sleep|display_reconnect|dpi_or_topology_change|removable_volume_return|expired_session|blocked_deep_link|unavailable_target|lock_screen_privacy>
  expected_state_token: <one of §3>
  upstream_fixture_refs:
    system_affordance_case_record_ref: <fixtures/platform/system_affordance_cases/...> | null
    window_display_verification_case_ref: <fixtures/platform/window_display_cases/...> | null
    notification_suppression_record_ref: <fixtures/ux/os_notification_cases/...> | null
  canonical_backing_refs:
    command_id_ref: <cmd:...>
    object_identity_ref: <obj:...>
    canonical_event_id_ref: <ux:event:...> | null
    event_lineage_ref: <ux:lineage:...> | null
  policy_context:
    policy_epoch: <pe:...>
    trust_state: <trusted|restricted|pending_evaluation|unknown>
    tenant_or_workspace_scope_ref: <scope:...> | null
  expected_outcome_class: <admitted_exact|review_required_before_execution|denied_with_recovery|degraded_placeholder|privacy_narrowed_delivery|context_preserved_revalidation_required>
  replay_posture: <single_use|bounded_reuse|read_only_resumable|replay_denied_consumed|replay_denied_expired|replay_denied_policy_epoch_changed|replay_denied_target_drifted|replay_denied_origin_mismatch>
  fallback_class: <open_intent_review_sheet|open_read_only_placeholder|open_cached_context|locate_missing_target|continue_local_only|open_activity_center|open_default_browser|deny_with_explanation|export_context|no_fallback_available>
  safe_actions:
    - <short label, no raw paths or URLs>
  must_not_happen:
    - <one assertion tied to authority, privacy, replay, focus, or destructive cleanup>
  audit_event_id: <one of platform.deep_link_intent_admitted | platform.deep_link_intent_review_required | platform.deep_link_intent_denied | platform.system_affordance_case_exercised | platform.notification_clickthrough_resolved | platform.lifecycle_recovery_preserved_context>
  notes: >
    <free-text reviewer rationale; closed-vocab fields above are the
    actual record of truth>
```

## 6. Conformance assertions

Every native-lifecycle drill case is non-conforming unless ALL of the
assertions below hold. Tooling and reviewers compare against this
list; new assertions require a decision row.

1. **Context preserved.** `lifecycle_recovery.preserves_context` is
   `true`. Local edit, transcript, and dirty-buffer state survive the
   transition. A row that loses local context records a typed denial
   reason and routes to crash recovery rather than claiming the case
   passed.
2. **Destructive cleanup forbidden.**
   `lifecycle_recovery.destructive_cleanup_forbidden` is `true`. The
   case never discards cached buffers, recent items, restore history,
   or held suppression records as "cleanup."
3. **Hidden focus steal forbidden.**
   `lifecycle_recovery.hidden_focus_steal_forbidden` is `true`. Wake,
   reconnect, reattach, or reopen never lands focus on a hidden,
   off-screen, or unrelated surface.
4. **Revalidation explicit.** Live remote, debug, callback, managed,
   or provider authority advertises `Reconnecting`, `Reopen required`,
   or `Expired session` until the user-driven revalidation completes.
   Silent replay or silent reauth is non-conforming.
5. **Reopen routes back to the canonical object.** Notification
   click-through, lock-screen quick action, dock / taskbar reopen, or
   companion push reopen MUST resolve to the durable object, durable
   row, or attention item via `exact_reopen_linkage`. Generic
   home-screen reopen is forbidden.
6. **Privacy preserved.** Lock-screen, companion, and OS-notification
   surfaces use `lock_screen_safe_generic`, `lock_screen_safe_scoped`,
   `redacted_metadata_only`, or `policy_forbidden_on_lock_screen` as
   appropriate. The forbidden label kinds and forbidden shortcut
   action classes from
   `schemas/ux/notification_suppression_record.schema.json` are not
   exposed.
7. **Reviewer can distinguish layout from data-loss from
   trust-boundary events.** The case's `expected_state_token` and
   `expected_outcome_class` together name whether the transition was
   shell / layout adjustment (`Layout adjusted`,
   `Safe bounds restored`), data loss or unavailability
   (`Root unavailable`), or trust / authority boundary change
   (`Reconnecting`, `Reopen required`, `Expired session`,
   `Local fallback`, `Notification suppressed for privacy`).

## 7. Reuse rules

A native-lifecycle drill packet is reusable in release evidence and
support packets when:

- every required case class from §4 is filled with a row that resolves
  to an upstream fixture ref;
- every row's `expected_state_token` is one of the closed eight in
  §3;
- the conformance assertions in §6 hold for every row;
- every claimed desktop profile from
  `artifacts/platform/claimed_desktop_profiles.yaml` either fills the
  row or records `not_applicable` with a typed reason; and
- the packet cites
  [`/artifacts/platform/lock_screen_privacy_rows.yaml`](./lock_screen_privacy_rows.yaml)
  for the lock-screen rows used by the OS-notification and
  auth/device-handoff cases.

## 8. Companion artifacts

| Artifact | Role |
|---|---|
| [`/fixtures/platform/native_lifecycle_cases/`](../../fixtures/platform/native_lifecycle_cases/) | Worked native-lifecycle drill cases, one per row in §4. |
| [`/artifacts/platform/lock_screen_privacy_rows.yaml`](./lock_screen_privacy_rows.yaml) | Closed lock-screen privacy row table for OS notifications and auth / device handoff surfaces. |
| [`/fixtures/platform/system_affordance_cases/`](../../fixtures/platform/system_affordance_cases/) | Upstream `system_affordance_case_record` fixtures the drills bind to. |
| [`/fixtures/platform/window_display_cases/`](../../fixtures/platform/window_display_cases/) | Upstream `window_display_verification_case` fixtures the topology and DPI drills bind to. |
| [`/fixtures/ux/os_notification_cases/`](../../fixtures/ux/os_notification_cases/) | Upstream `notification_suppression_record` fixtures the privacy drill binds to. |
| [`/artifacts/qa/window_display_matrix.yaml`](../qa/window_display_matrix.yaml) | Upstream window/display continuity matrix the topology drills cite. |
| [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../ux/quiet_hours_policy_matrix.yaml) | Upstream quiet-hours per-mode policy the privacy drill cites. |

## 9. Review checklist

A change touching a native-lifecycle drill is conforming only if a
reviewer can answer:

1. Which `lifecycle_state_class` was exercised, and which fixture under
   `/fixtures/platform/native_lifecycle_cases/` records the drill?
2. Which closed `expected_state_token` did the in-product surface
   advertise after the transition?
3. Which canonical command, object identity, and event lineage backed
   the in-product state, and which `audit_event_id` closed the trail?
4. Which `safe_actions` were offered, and which `must_not_happen`
   assertion would the row have violated if the platform tried a
   silent rerun, hidden focus steal, destructive cleanup, or authority
   widening?
5. For lock-screen, companion, or auth / device-handoff rows: which
   row in `/artifacts/platform/lock_screen_privacy_rows.yaml` named
   the privacy payload class, allowed labels, and forbidden shortcut
   actions for the surface?
