# Interruptibility arbitration, focus-steal prevention, and escalation-surface contract

This document is the **arbitration contract** that decides — for every
event the attention router considers — whether the event MAY interrupt
the user, the surface class it MAY appear on, and whether it MUST
remain durable-only instead of escalating to a modal, banner, OS
notification, or companion alert. It exists so daily operation stays
calm, keyboard-safe, and reviewable, and so trust, auth, update, and
repair flows can request attention without bypassing the escalation
model.

The contract is normative. Where this document disagrees with the UI /
UX Spec or the upstream attention-routing, notification, navigation,
shell-interaction-safety, dialog / sheet, voice / dictation, OS-
notification / quiet-hours, durable-work, or quiet-hours override
contracts, those sources win and this document, the companion matrix,
and the fixtures update in the same change. Where this document
disagrees with a downstream surface's private interrupt rule, this
document wins and the surface is non-conforming.

## Companion artifacts

- [`/artifacts/ux/escalation_matrix.yaml`](../../artifacts/ux/escalation_matrix.yaml)
  — machine-readable matrix binding each `active_flow_class` to the
  `interruptibility_tier` rows that may interrupt, the
  `delivery_surface_class` set that may render, the
  `arbitration_outcome_class` produced, the `focus_steal_attempt_class`
  set that is denied, and the typed escalation triggers required.
- [`/fixtures/ux/interruptibility_cases/`](../../fixtures/ux/interruptibility_cases/)
  — worked YAML fixtures for trust, auth, update, repair, voice
  capture, presentation, screen share, assistive-tech, and OS
  notification reopen flows.

## Upstream contracts this contract rides on

This contract mints **no** new attention class, interruptibility tier,
quiet-hours mode, suppression reason, delivery-surface class, badge
class, dedupe scheme, reopen-target kind, dismissal verb, durable-job
lifecycle state, escalation tier, focus-return state, responsive-
fallback mode, required-visible-field class, escalation reason, or
denial reason. Every value is re-exported from the upstream sources:

- [`docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — interruptibility tier, attention class, delivery-surface class,
  quiet-hours mode, suppression reason, privacy payload class, dedupe
  scheme, reopen-target kind, dismissal verb, durable-job lifecycle
  state, source subsystem.
- [`/artifacts/ux/interruptibility_escalation_seed.yaml`](../../artifacts/ux/interruptibility_escalation_seed.yaml)
  — per-tier default and forbidden delivery surfaces, escalation
  required-trigger set, and the seed `protected_paths` (`save`,
  `recovery`, `trust_review`, `debugging`, `ai_apply`,
  `patch_approval`, `restore_from_crash`).
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  — per-`quiet_hours_mode` suppressed / preserved surfaces, suppressed
  tiers, durable-history preservation rule, exit-digest behaviour.
- [`/artifacts/ux/quiet_hours_override_matrix.yaml`](../../artifacts/ux/quiet_hours_override_matrix.yaml)
  — closed override event classes (security advisory, trust downgrade,
  approval expiry, route warning) that may break a hold.
- [`docs/ux/notification_contract.md`](./notification_contract.md) and
  [`docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  — notification surface classes, dedupe / grouping / escalation
  rules, exact-reopen targets, and forbidden-shortcut posture.
- [`docs/ux/os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  — OS notification redaction, lock-screen-safe summary, companion
  fanout, and forbidden-shortcut classes.
- [`docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — review-sheet / modal posture, focus-return state, responsive-
  fallback mode, required-visible-field set, authority class,
  consequence class, preview / apply / revert phase, and the typed
  permission-renewal triggers.
- [`docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md) —
  surface-class selection, no product-owned nested overlay, platform-
  auth-dialog exception.
- [`docs/ux/navigation_and_escalation_contract.md`](./navigation_and_escalation_contract.md)
  and [`/artifacts/ux/navigation_hierarchy.yaml`](../../artifacts/ux/navigation_hierarchy.yaml)
  — `escalation_tier` ladder
  (`tier.inline_in_target_surface` →
  `tier.contextual_inline_overlay` →
  `tier.panel_attached_to_surface` →
  `tier.sheet_attached_to_window` →
  `tier.dialog_modal` →
  `tier.full_surface_takeover`) and the inline-first / panel-second /
  modal-last rule.
- [`docs/ux/voice_and_dictation_contract.md`](./voice_and_dictation_contract.md)
  — voice-mode session, mic-indicator state, command-preview posture,
  transcript correction, and the rule that voice-issued privileged
  actions land on the same preview / apply / revert phase a
  keyboard-issued action lands on.
- [`docs/ux/exact_reopen_drill_map.md`](./exact_reopen_drill_map.md)
  — eight reopen source surfaces collapsing to one canonical durable
  identity.

## Who reads this document

- **Shell, attention-router, notification-router, and command-router
  authors** deciding whether an emitted event renders inline, panels,
  sheets, modals, durable rows, OS notifications, or companion
  alerts — and whether it must hold for a protected flow.
- **Owning subsystems** (editor, terminal, review / diff, palette /
  search, install / update / attach, AI apply, collaboration,
  provider-bearing, docs / help / service-health, support / export,
  build system, test runner, debug session, task runner, indexer,
  VFS save, sync / mirror, notebook kernel, remote agent, extension
  host, workspace trust, policy resolver, admin policy, secret broker,
  runtime power manager) confirming their events resolve to one
  arbitration outcome before reaching a protected user flow.
- **Trust / auth / update / repair flow authors** confirming their
  attention requests stay inside the escalation model rather than
  inventing a private modal, replay path, or focus-stealing OS
  notification.
- **Support, parity-audit, and review tooling** mechanically joining
  `canonical_event_id`, `interaction_safety_packet_id_ref`,
  `event_lineage_id_ref`, `suppression_record_id`, `reopen_target_id`,
  and `interruptibility_arbitration_decision_id` across the contracts
  above.

## Why this exists

Without one arbitration contract, daily-use surfaces drift fast:

- a modal asks the user to confirm a benign update while they are
  actively typing, focus-steals the editor, and loses keystrokes;
- a connectivity-loss banner pre-empts a review-and-diff canvas mid-
  apply, hiding the consequence block behind a notification;
- an OS notification re-asks the user to approve a credential
  callback after they already dismissed it, replaying focus on every
  display reconnect;
- a presentation with screen-share active surfaces a personal toast
  that exposes a workspace identifier on the projected screen;
- an extension repair card flashes a modal during voice dictation
  and the user's spoken sentence becomes a button press;
- a badge for an unrelated event class climbs while a high-risk
  approval is on screen, pulling attention off the consequence;
- an admin-policy refresh stacks itself as a second product-owned
  overlay over an active permission sheet rather than updating the
  parent surface in place;
- a low-severity burst escalates to tier_blocking_trust without a
  typed trigger so reviewers cannot see why the modal appeared;
- companion push replays the same event the desktop already showed,
  inflating the user's attention budget across clients.

This contract closes those gaps. It freezes one closed
`active_flow_class` set, one closed `arbitration_outcome_class` set,
one closed `focus_steal_attempt_class` set, and the rule that every
arbitration decision quotes an upstream tier, escalation trigger,
quiet-hours mode, and reopen target by reference rather than minting
a parallel vocabulary.

## 1. Scope

In scope:

- the **arbitration rules** that decide, for each emitted event,
  whether the event may interrupt the user given the user's currently
  active flow (typing, review, full-screen presentation, screen share,
  assistive-tech use, voice capture, save, recovery, trust review,
  debugging, AI apply, patch approval, restore from crash, auth
  callback pending, update install pending, repair pending) and the
  active `quiet_hours_mode` set;
- the **focus-steal prevention rules** for product-owned modals,
  banners, activity items, auth callbacks, update prompts, OS
  notification reopen paths, companion replays, and voice-issued
  privileged actions;
- the **escalation-surface matrix** binding each event tier to the
  set of surface classes (`inline_in_target` / `contextual_banner` /
  `attached_sheet` / `modal_dialog` / `full_surface_takeover` /
  `durable_job_row` / `attention_item` / `os_notification` /
  `companion_push`) it may render on under the active flow and quiet-
  hours mode;
- the **denial set** (`focus_steal_on_protected_path`,
  `interruption_tier_escalated_without_trigger`,
  `product_owned_nested_overlay_forbidden`,
  `reopen_to_generic_home_forbidden`,
  `reopen_requires_revalidation`,
  `quiet_hours_hold_discarded_durable_history`,
  `admin_suppression_blocked_critical_safety`) used when arbitration
  refuses an interruption.

Out of scope:

- window-manager integration, OS-level focus follow-mouse rules, OS-
  level notification adapters, and the eventual attention-router /
  notification-router crates' Rust types. The matrix reserves the
  arbitration shape; adapters wire it later.
- per-user preference UI for hold thresholds, snooze defaults, mute
  granularity, and presentation auto-detect heuristics.
- per-surface rendering freedoms (toast spacing, modal entrance
  animation, banner collapse). Layout-token vocabularies own
  rendering.

## 2. Frozen vocabulary (re-exported)

Every value below is re-exported. New values are additive-minor and
require an upstream schema bump and a decision row; repurposing a
value is breaking.

- **`interruptibility_tier`** —
  `tier_ambient` / `tier_transient` / `tier_durable` /
  `tier_actionable` / `tier_blocking_trust` / `tier_critical_safety`.
- **`delivery_surface_class`** —
  `toast` / `contextual_banner` / `status_item` / `durable_job_row` /
  `attention_item` / `activity_center_digest_card` /
  `digest_group_row` / `os_notification` / `os_badge_app_icon` /
  `lock_screen_summary` / `companion_push` / `not_delivered_held`.
- **`escalation_tier`** —
  `tier.inline_in_target_surface` /
  `tier.contextual_inline_overlay` /
  `tier.panel_attached_to_surface` /
  `tier.sheet_attached_to_window` /
  `tier.dialog_modal` /
  `tier.full_surface_takeover`.
- **`quiet_hours_mode`** —
  `mode_none` / `mode_quiet_hours_user` /
  `mode_do_not_disturb_user` / `mode_focus_mode_user` /
  `mode_presentation` / `mode_screen_share` /
  `mode_privacy_mode` / `mode_reduced_attention_policy` /
  `mode_power_saver_runtime` / `mode_admin_suppression`.
- **`escalation_required_triggers`** —
  `severity_increased` / `duration_threshold_crossed` /
  `required_authority_changed` / `consequence_class_escalated` /
  `basis_snapshot_drifted` / `policy_epoch_changed` /
  `trust_state_changed` / `provider_grant_narrowed` /
  `recovery_class_downgraded_to_no_recovery` /
  `repeated_dedupe_threshold_crossed`.
- **upstream `protected_paths`** (from the interruptibility
  escalation seed) — `save` / `recovery` / `trust_review` /
  `debugging` / `ai_apply` / `patch_approval` / `restore_from_crash`.
- **`reopen_target_kind`** —
  `canonical_object_target_exact` / `review_sheet` / `diff_view` /
  `evidence_packet_row` / `activity_center_item` /
  `history_lane_row` / `attention_item_exact` /
  `durable_job_row_exact` / `placeholder_announced` /
  `reopen_denied_requires_revalidation`.
- **`override_event_class`** (from the quiet-hours override matrix) —
  `security_advisory_break` / `trust_downgrade_break` /
  `approval_expiry_break` / `route_warning_break`.
- **`focus_return_state`** —
  `returned_exact` / `returned_nearest_safe_ancestor` /
  `returned_current_batch_or_detail_owner` /
  `returned_placeholder_announced` / `focus_loss_denied` /
  `focus_not_applicable_non_interactive`.

## 3. Arbitration-local vocabulary (closed, additive-minor)

This contract introduces three small arbitration-local vocabularies.
They are scoped to arbitration rows and never substitute for any
frozen upstream vocabulary; each value cites the upstream axis it
binds against.

### 3.1 `active_flow_class`

The user's currently active protected flow at the moment an event
arrives. The set is closed.

| `active_flow_class`                  | Binds upstream                                                                             | Default posture                                                                                                                                                          |
|--------------------------------------|--------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `editor_typing_active`               | none upstream; arbitration-local                                                           | Keystrokes are reaching a buffer. `tier_transient`, `tier_durable`, `tier_actionable` hold or route durable-only.                                                        |
| `review_canvas_active`               | upstream `patch_approval` and `ai_apply`                                                   | Review / diff / approval canvas is on screen. Modal interruptions deny.                                                                                                  |
| `full_screen_presentation_active`    | `mode_presentation`                                                                        | Audience-visible surface. Toast / OS notification / companion push / lock-screen summary suppressed; durable rows update silently.                                        |
| `screen_share_active`                | `mode_screen_share`                                                                        | As `full_screen_presentation_active`; collaboration pings route to session digest.                                                                                       |
| `assistive_tech_active`              | `mode_reduced_attention_policy`                                                            | Toasts route to `status_item`; announcements use the same screen-reader lane the keyboard route uses.                                                                    |
| `voice_capture_active`               | none upstream; arbitration-local                                                           | Mic indicator is hot. Voice-issued privileged actions land on the keyboard preview / apply / revert posture; transient toasts route to `status_item`.                    |
| `voice_command_preview_active`       | upstream `ai_apply` (preview phase)                                                        | A voice-issued command is in `preview` awaiting confirmation. Modal interruptions deny.                                                                                  |
| `save_in_flight`                     | upstream `save`                                                                            | Save lifecycle is mid-write. Non-critical notices hold; durable rows update silently.                                                                                    |
| `recovery_in_flight`                 | upstream `recovery`                                                                        | Restore / repair active. Only `tier_durable` repair updates and `tier_blocking_trust` / `tier_critical_safety` may render.                                              |
| `restore_from_crash_active`          | upstream `restore_from_crash`                                                              | First surface after a crash is recovery. Ambient / transient / actionable chrome MUST NOT interpose.                                                                     |
| `trust_review_active`                | upstream `trust_review`                                                                    | Trust / policy / authority decision on screen. Only `tier_blocking_trust` and `tier_critical_safety` render.                                                             |
| `debugging_active`                   | upstream `debugging`                                                                       | Step / watch / breakpoint focus preserved. Transient and actionable chrome routes to activity center.                                                                    |
| `ai_apply_active`                    | upstream `ai_apply`                                                                        | AI apply mid-evidence-review. Other tiers compete for focus and MUST NOT interrupt.                                                                                      |
| `patch_approval_active`              | upstream `patch_approval`                                                                  | Approve / deny consequence-bearing decision on screen. Unrelated notices hold.                                                                                           |
| `auth_callback_pending`              | quiet-hours override `active_provider_handoff_session`                                     | Provider handoff or browser-handoff approval is mid-flight. Reopen denies until revalidation; lock-screen summary stays scoped.                                          |
| `update_install_pending`             | upstream `tier_durable` `update_install`                                                   | Update is downloading or staging. Durable row + status item; modal forbidden until reboot-required tier_blocking_trust escalation.                                       |
| `repair_pending`                     | upstream `tier_durable` `restore_or_recovery` and extension repair                         | Extension crash / restart / repair card on screen. Modal interruptions deny; companion fanout collapses to one canonical row.                                            |
| `none`                               | `mode_none`                                                                                | No protected flow. Default escalation matrix applies.                                                                                                                    |

Rules (frozen):

1. A surface that cannot resolve its `active_flow_class` MUST default
   to the strictest matching row that is **at least as restrictive**
   as any candidate row, never to `none`. A surface that defaults to
   `none` while a protected flow is observable (mic hot, save
   pending, presentation engaged, review canvas focused) is non-
   conforming.
2. Multiple `active_flow_class` values MAY be active concurrently
   (e.g. `voice_capture_active` + `editor_typing_active`). The
   arbitration row is the **intersection** of the allowed surface
   sets, not the union.
3. Active-flow detection runs on the first-party shell. Extension-
   contributed surfaces inherit the active-flow posture of the
   embedding zone; an extension that overrides active-flow detection
   to render a modal is non-conforming
   (`focus_steal_on_protected_path`).

### 3.2 `arbitration_outcome_class`

The decision the arbitration layer recorded. The set is closed.

- `delivered_inline_in_target_surface`
- `delivered_contextual_banner`
- `delivered_status_item`
- `delivered_durable_job_row`
- `delivered_attention_item`
- `delivered_activity_center_digest_card`
- `delivered_digest_group_row`
- `delivered_os_notification_lock_screen_safe`
- `delivered_companion_push_lock_screen_safe`
- `routed_via_review_sheet`
- `routed_via_attached_sheet`
- `routed_via_modal_dialog`
- `routed_via_full_surface_takeover`
- `held_not_delivered_release_pending`
- `denied_focus_steal_on_protected_path`
- `denied_product_owned_nested_overlay`
- `denied_admin_suppression_blocked_critical_safety`
- `denied_interruption_tier_escalated_without_trigger`
- `denied_reopen_to_generic_home`
- `denied_reopen_requires_revalidation`

Rules (frozen):

1. Every arbitration decision records exactly one
   `arbitration_outcome_class` and either the
   `delivery_surface_class` chosen (when delivered) or the
   `denial_reason` recorded (when denied).
2. A `denied_*` outcome MUST emit an
   `activity_event_envelope_record` with
   `delivery_surface_class = not_delivered_held` (or the equivalent
   review-sheet / sheet routing record) so durable history is
   preserved. Silent denial is non-conforming
   (`held_event_missing_audit_trail`).
3. A `routed_via_*` outcome MUST quote the upstream
   `interaction_safety_packet_id_ref` (review sheet / attached sheet)
   or the navigation `escalation_tier` ladder advance reason. A
   `routed_via_modal_dialog` outcome that does not name the
   `consequence_class_label`, `preview_class`,
   `approval_posture_class`, or attention-tier value that justified
   the escalation is non-conforming (per
   `navigation_and_escalation_contract.md` §4.5).

### 3.3 `focus_steal_attempt_class`

The closed set of forbidden interruption forms. Every row in the
matrix names the focus-steal attempts it denies. The set is closed.

- `product_owned_modal_attempted_during_typing`
- `product_owned_modal_attempted_during_review`
- `product_owned_modal_attempted_during_presentation`
- `product_owned_modal_attempted_during_voice_capture`
- `product_owned_modal_attempted_during_repair`
- `product_owned_banner_replaced_typing_focus`
- `product_owned_banner_replaced_review_focus`
- `activity_item_burst_attempted_during_typing`
- `auth_callback_window_focus_grab`
- `update_install_modal_attempted_during_typing`
- `update_install_modal_attempted_during_presentation`
- `os_notification_replay_after_dismiss`
- `companion_push_replay_after_dismiss`
- `voice_command_inline_apply_attempted`
- `extension_repair_modal_attempted`
- `nested_product_overlay_stacked_on_sheet`

Rules (frozen):

1. Every `focus_steal_attempt_class` value resolves to one upstream
   denial reason: `focus_steal_on_protected_path`,
   `product_owned_nested_overlay_forbidden`,
   `reopen_to_generic_home_forbidden`,
   `reopen_requires_revalidation`,
   `interruption_tier_escalated_without_trigger`,
   `admin_suppression_blocked_critical_safety`, or
   `held_event_missing_audit_trail`. A focus-steal denial that
   cannot resolve to one of those is non-conforming.
2. A focus-steal attempt that lands on `tier_critical_safety` does
   **not** route as a focus steal; it routes via the interaction-
   safety review sheet or modal path
   (`shell_interaction_safety_contract.md`) and records the
   `tier_critical_safety` outcome rather than denying.

## 4. Arbitration rules (normative)

Every rule is normative. A surface that violates any rule is non-
conforming regardless of how the violation is painted.

### 4.1 Events that may never steal focus on protected typing or review paths

The following classes of events MUST route durable-only when the
active flow is `editor_typing_active`, `review_canvas_active`,
`patch_approval_active`, `ai_apply_active`,
`voice_command_preview_active`, `save_in_flight`,
`debugging_active`, or `restore_from_crash_active`:

- `tier_transient` deliveries (any `attention_class`).
- `tier_durable` deliveries except those whose `source_subsystem`
  matches the active flow's owning subsystem (e.g. a build under
  review may update its own row; an unrelated indexer pass holds).
- `tier_actionable` deliveries from a different
  `canonical_object_target_ref` than the active flow's target.
- `attention_item_record`s arriving as a **burst** (more than one
  member sharing a `grouped_burst_id`) — the burst routes to a
  digest until the active flow ends, no toast.
- Update-install staging notices below `tier_blocking_trust`
  (`update_install_pending` may render durable_job_row + status_item
  but never a modal until reboot is required at
  `tier_blocking_trust`).
- Repair-flow reconnect or restart prompts below
  `tier_blocking_trust` (extension repair card lands on
  `attention_item`, never modal).
- Auth-callback "your sign-in completed" toasts; the durable
  provider-handoff row updates instead.
- OS notification or companion push **replays** of an event the user
  already dismissed in-product (`os_notification_replay_after_dismiss`
  / `companion_push_replay_after_dismiss`) — the dismissal verb on
  the durable row is authoritative.

`tier_blocking_trust` and `tier_critical_safety` are the only tiers
that may interrupt these flows, and only via the interaction-safety
review-sheet or modal path. A toast or banner that pre-empts these
flows is non-conforming (`focus_steal_on_protected_path`).

### 4.2 States that MUST remain durable-only instead of escalating

The following classes of state remain on `durable_job_row`,
`attention_item`, `activity_center_digest_card`, or `digest_group_row`
and MUST NOT escalate to `modal_dialog`, `full_surface_takeover`,
`os_notification`, `lock_screen_summary`, or `companion_push` unless
the upstream tier rules and the active-flow row both admit the
escalation:

| Durable-only state                                                     | May render on                                                            | MUST NOT escalate to                                                                              |
|------------------------------------------------------------------------|--------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------|
| Long-running build / test / debug session / task run                   | `durable_job_row` + `os_badge_app_icon` + optional `toast` mirror        | `modal_dialog`; OS notification only on completion / failure if user is away                      |
| Indexer pass, save / sync, transport reconnect, admin policy refresh   | `durable_job_row` + `status_item`                                        | `modal_dialog`, `os_notification` while user is in-product, `companion_push`                       |
| Held quiet-hours bursts                                                | `not_delivered_held` envelope + activity center digest on mode exit       | OS notification / companion push during the hold; release as one grouped digest                   |
| Suppressed security notice (admin policy narrows OS surface)           | `attention_item` + durable audit trail                                   | OS notification; `tier_critical_safety` never silenced — it routes via review sheet                |
| Update download / staging                                              | `durable_job_row` + `status_item`                                        | Modal until reboot required (tier_blocking_trust); companion push only on completion              |
| Extension crash repair card                                            | `attention_item` + `contextual_banner` in the embedding zone             | Modal; companion fanout collapses to one canonical row                                            |
| Voice-mode mic state                                                   | persistent `status_item` + `os_badge_app_icon`                           | Modal; voice-issued privileged action lands on review-sheet preview, never inline apply           |
| Auth callback pending revalidation                                     | `durable_job_row` + `attention_item`; reopen denies until user re-enters | Modal; OS notification carries `lock_screen_safe_scoped` summary only                             |
| Companion-delivered alert already dismissed in-product                 | history_lane_row preserves dismissal                                     | OS notification replay; companion push replay; cross-client divergence                            |

A surface that escalates one of those states past the cell on the
right denies with `focus_steal_on_protected_path`,
`interruption_tier_escalated_without_trigger`, or
`reopen_requires_revalidation` (whichever applies first).

### 4.3 Trust, auth, update, and repair flows STAY inside the escalation model

These flows often carry the only events that legitimately interrupt;
the contract names exactly **how** they request attention:

- **Trust downgrade.** A workspace trust state moving from `trusted`
  to `restricted` mid-session is `tier_blocking_trust` with
  `trust_state_changed` as the typed escalation reason. It routes
  through the interaction-safety review sheet on the
  `trust_prompt_canvas`; the OS notification carries
  `lock_screen_safe_scoped`; reopen denies with
  `reopen_denied_requires_revalidation` until the user re-enters
  the trust review canvas. Modal-attempting an inline trust grant
  is non-conforming.
- **Auth callback / approval expiry.** A provider handoff or
  browser-handoff approval ticket nearing expiry is
  `tier_blocking_trust` with `duration_threshold_crossed` or
  `required_authority_changed`. It routes via the review sheet
  and the durable provider-handoff row. The OS notification stays
  `lock_screen_safe_scoped`; raw URLs and raw provider payloads
  never reach the OS sink. Replaying an OS notification after the
  user already dismissed it in-product denies with
  `companion_push_replay_after_dismiss` /
  `os_notification_replay_after_dismiss`.
- **Update install / reboot required.** Update download is
  `tier_durable` (durable row + status item). Reboot-required
  promotion is `tier_blocking_trust` with `severity_increased` or
  `consequence_class_escalated` and routes via the review sheet,
  not a focus-stealing modal. A modal during typing or presentation
  denies with `update_install_modal_attempted_during_typing` /
  `update_install_modal_attempted_during_presentation`.
- **Repair (extension crash, transport reconnect, restore-from-
  crash).** Repair updates are `tier_durable` durable rows.
  Promotion to `tier_blocking_trust` requires
  `consequence_class_escalated` or
  `recovery_class_downgraded_to_no_recovery` and routes via the
  review sheet. A repair-card modal during voice capture or
  presentation denies with `extension_repair_modal_attempted`.

### 4.4 Critical-safety always renders, but never as a generic modal

`tier_critical_safety` events render regardless of `quiet_hours_mode`.
The arbitration outcome is one of:

- `routed_via_review_sheet` — the in-product authoritative surface;
- `delivered_contextual_banner` — when the review sheet is already
  open on the canonical target;
- `delivered_attention_item` — when the user is away from the
  canonical target;
- `delivered_os_notification_lock_screen_safe` /
  `delivered_companion_push_lock_screen_safe` — when the user is
  away from the product entirely.

A `tier_critical_safety` event that renders as a product-owned
generic modal with no review-sheet origin denies with
`focus_steal_on_protected_path` and the
`security_advisory_break` / `trust_downgrade_break` /
`approval_expiry_break` / `route_warning_break` row from the quiet-
hours override matrix MUST be quoted on the lineage.

`mode_admin_suppression` MAY narrow but MAY NOT silently block
`tier_critical_safety`. A managed policy that silently blocks a
critical-safety delivery denies with
`admin_suppression_blocked_critical_safety`.

### 4.5 No product-owned nested overlay, ever

A product-owned modal MUST NOT stack on top of a product-owned sheet,
modal, or full-surface takeover. Stacking denies with
`product_owned_nested_overlay_forbidden`
(per `dialog_sheet_contract.md`). The parent surface MUST update in
place, replace itself with a larger surface, or hand off to a
durable target.

The only exception is a platform-owned overlay (auth dialog, file
picker), which is host-owned and not product-stacked.

### 4.6 Reopen lands on the canonical durable identity

Invoking an OS notification, companion push, lock-screen summary,
status item, badge, toast, contextual banner, activity-center row,
history-lane row, or digest entry reopens to the **narrowest
truthful destination** named on the `reopen_target_record`.
Reopening to a generic home screen denies with
`reopen_to_generic_home_forbidden`.

When the originating target requires fresh user intent (wake from
sleep, display reconnect, policy-epoch change, provider grant
narrowed), reopen denies with `reopen_denied_requires_revalidation`
rather than silently replaying a mutating action. The arbitration
records `denied_reopen_requires_revalidation` and the durable row
exposes the revalidation route.

### 4.7 Voice-issued privileged actions ride the keyboard preview lane

A voice-issued privileged action (apply, install, publish, delete,
revoke) lands on the same `preview` / `apply` / `revert` posture a
keyboard-issued action lands on. Voice cannot collapse the preview
phase, downgrade a consequence class, or mint a private "I just said
it" approval. A voice command that triggers an inline apply outside
the review-sheet preview lane denies with
`voice_command_inline_apply_attempted` and routes back to the
review sheet.

### 4.8 Dismiss is authoritative across clients

Dismissing a durable row is recorded on
`cross_client_canonical_event_id` lineage. An OS notification or
companion push that replays the same `canonical_event_id` after the
user dismissed it in-product denies with
`os_notification_replay_after_dismiss` /
`companion_push_replay_after_dismiss`. Cross-client divergence
denies with `companion_cross_client_divergence`.

### 4.9 Durable history is always preserved

Every arbitration outcome (delivered, held, denied) emits an
`activity_event_envelope_record` so durable history, audit trail,
and reopen targets remain reviewable. A delivery that bypassed the
audit trail denies with `held_event_missing_audit_trail`.

## 5. Escalation-surface matrix (summary; the YAML is authoritative)

The matrix in
[`/artifacts/ux/escalation_matrix.yaml`](../../artifacts/ux/escalation_matrix.yaml)
binds, for each `(active_flow_class, interruptibility_tier,
quiet_hours_mode)` triple:

- the allowed `delivery_surface_class` set;
- the allowed `escalation_tier` (inline / contextual overlay / panel
  / sheet / modal / full takeover);
- the required `escalation_required_triggers` to advance past
  `tier.contextual_inline_overlay`;
- the `arbitration_outcome_class` produced;
- the `focus_steal_attempt_class` set the row denies;
- the upstream `denial_reason` the row resolves to when denied;
- the `reopen_target_kind` and revalidation posture for the
  produced delivery (where applicable).

The summary below is illustrative; reviewers MUST consult the YAML
for the authoritative rows.

| Active flow                                | tier_ambient                | tier_transient                  | tier_durable                                  | tier_actionable                               | tier_blocking_trust                          | tier_critical_safety                         |
|--------------------------------------------|-----------------------------|---------------------------------|-----------------------------------------------|-----------------------------------------------|----------------------------------------------|----------------------------------------------|
| `editor_typing_active`                     | status_item / badge         | held → digest                   | durable_job_row only (no toast)               | attention_item only                           | review_sheet (no inline modal)               | review_sheet + banner                        |
| `review_canvas_active`                     | status_item / badge         | held → digest                   | durable_job_row only                          | activity_center_item only                     | review_sheet (parent updates in place)       | review_sheet + banner                        |
| `full_screen_presentation_active`          | status_item                 | held → digest                   | durable_job_row only                          | held → digest                                 | review_sheet (banner only, no modal)         | review_sheet + banner                        |
| `screen_share_active`                      | status_item                 | held → digest                   | durable_job_row only                          | held → digest                                 | review_sheet (banner only)                   | review_sheet + banner                        |
| `assistive_tech_active`                    | status_item / badge         | status_item only                | durable_job_row + status_item                 | attention_item                                | review_sheet                                 | review_sheet + banner + os_notification      |
| `voice_capture_active`                     | status_item / mic indicator | held → digest                   | durable_job_row only                          | attention_item only                           | review_sheet (no inline apply)               | review_sheet + banner                        |
| `voice_command_preview_active`             | status_item                 | held                            | durable_job_row only                          | held                                          | review_sheet                                 | review_sheet + banner                        |
| `save_in_flight`                           | status_item                 | held                            | durable_job_row (own subsystem)               | held                                          | review_sheet                                 | review_sheet + banner                        |
| `debugging_active`                         | status_item                 | held                            | durable_job_row (own session)                 | activity_center_item                          | review_sheet                                 | review_sheet + banner                        |
| `ai_apply_active` / `patch_approval_active`| status_item                 | held                            | durable_job_row (own apply)                   | activity_center_item                          | review_sheet                                 | review_sheet + banner                        |
| `auth_callback_pending`                    | status_item / badge         | held                            | durable_job_row (provider handoff)            | attention_item (revalidate route)             | review_sheet (provider handoff)              | review_sheet + os_notification scoped        |
| `update_install_pending`                   | status_item / badge         | held                            | durable_job_row (download / staging)          | attention_item (reboot ready)                 | review_sheet (reboot required)               | review_sheet + os_notification               |
| `repair_pending`                           | status_item / badge         | held                            | durable_job_row (recovery)                    | attention_item (next action)                  | review_sheet (recovery downgrade)            | review_sheet + os_notification               |
| `none`                                     | per upstream tier rules     | toast OK                        | durable_job_row + optional toast              | attention_item + optional toast               | review_sheet                                 | review_sheet + banner + os_notification      |

## 6. Audit and boundary posture

Process-boundary constraints (frozen):

1. Every arbitration decision crosses the RPC boundary as an
   `interruptibility_arbitration_decision_record` (a typed view over
   the upstream `activity_event_envelope_record`,
   `notification_suppression_record`, and
   `event_lineage_record` shapes). Raw bodies, raw paths, raw URLs,
   raw prompt text, and raw credential material never cross.
2. The matrix is read-only at runtime. New rows are additive-minor
   and require a decision row in
   `artifacts/governance/decision_index.yaml`; repurposing a row is
   breaking.
3. Support exports include the full
   `(active_flow_class, interruptibility_tier, quiet_hours_mode)`
   row, the chosen `arbitration_outcome_class`, the cited
   `escalation_required_triggers`, the
   `interaction_safety_packet_id_ref`, and any
   `focus_steal_attempt_class` denials. Raw bodies are excluded.
4. Crash dumps and core files MUST NOT inherit unresolved
   arbitration decisions; a crash that lands mid-arbitration
   discards the decision rather than persisting a partial axis set.

Redaction defaults inherit from the attention-routing taxonomy and
the OS-notification / quiet-hours contract; this contract adds no
parallel redaction posture.

## 7. Schema-of-record posture

The eventual attention-router / notification-router crate's Rust
types are the source of truth. The matrix YAML at
`artifacts/ux/escalation_matrix.yaml` and the fixtures at
`fixtures/ux/interruptibility_cases/` are the cross-tool boundary
every non-owning surface reads. Adding an `active_flow_class`,
`arbitration_outcome_class`, or `focus_steal_attempt_class` value is
additive-minor and requires a decision row; repurposing any
existing value is breaking.

There is no external IDL or code-generator toolchain at this stage;
this mirrors the upstream attention-routing posture.

## 8. Non-goals

Out of scope until a superseding decision row opens:

- Window-manager integration, OS-level focus-follow-mouse rules, OS-
  level notification adapter implementations, and the eventual
  attention-router / notification-router / command-router crates'
  Rust types.
- Per-user preference UI for hold thresholds, snooze defaults, mute
  granularity, presentation auto-detect heuristics, and assistive-
  tech posture toggles.
- Per-surface rendering freedoms (toast spacing, modal entrance
  animation, banner collapse). Layout-token vocabularies own
  rendering.
- Any new `protected_path`, `quiet_hours_mode`, `interruptibility_tier`,
  `delivery_surface_class`, `attention_class`, `escalation_required_trigger`,
  or `denial_reason` value. New values require an upstream schema
  bump.

These lines move only by opening a new decision row, not by editing
this contract.

## 9. Reuse guarantee

This contract is reusable by every owning subsystem without
redefining attention semantics. A subsystem that mints an event MUST:

1. Resolve the event's `interruptibility_tier`,
   `delivery_surface_class`, `attention_class`,
   `quiet_hours_mode_at_mint`, `dedupe_key_scheme`,
   `reopen_target_kind`, and `dismissal_verbs_available` from the
   upstream taxonomy verbatim.
2. Resolve the `active_flow_class` from this contract before
   choosing a surface; default to the strictest matching row if
   detection is ambiguous.
3. Quote the matrix row by `arbitration_row_id` rather than
   re-deriving the surface set; a surface that mints a private
   surface set is non-conforming.
4. Record the chosen `arbitration_outcome_class` and either the
   delivered `delivery_surface_class` or the `denial_reason` on the
   activity-event envelope and the lineage record.
5. Cite the `escalation_required_triggers` value when escalating a
   tier; decorative escalation denies with
   `interruption_tier_escalated_without_trigger`.
6. Quote the upstream `interaction_safety_packet_id_ref` when the
   arbitration routes via a review sheet, modal, or full-surface
   takeover; the consequence class, preview class, approval posture,
   and required visible fields all come from upstream.
7. Honour the lock-screen-safe / scoped redaction posture on every
   OS-bound or companion-bound surface; raw bodies, raw paths, raw
   URLs, raw secret material, and raw prompt / completion text never
   reach the OS sink.
8. Preserve durable history under every `arbitration_outcome_class`,
   including denials and holds. A delivery that bypassed the audit
   trail denies with `held_event_missing_audit_trail`.
