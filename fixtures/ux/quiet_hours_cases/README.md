# Quiet-hours cases

Worked snapshots of the shell-level quiet-hours / do-not-disturb posture
applied to typed notification envelopes, routed through
`aureline_shell::notifications::NotificationRouter`, and projected onto a
privacy-safe [`DurableBadgeProjection`].

These fixtures are the truth a reviewer reads to confirm:

1. **Quiet hours narrows interruption without hiding durable truth.** The
   posture flags transient surfaces (toast, OS notification, lock-screen
   summary, companion push) as held; durable surfaces (durable_job_row,
   activity_center_digest_card) keep delivering so the activity center
   never loses an item.
2. **Critical-safety severity bypasses every mode.** A
   `severity_class = critical` envelope is never marked `suppressed=true`
   by the posture, so the router still delivers it onto every requested
   surface — including the OS notification — even when DND or quiet-hours
   are scheduled.
3. **Badge counts are deduped and privacy-safe.** Three repeat emissions
   of the same canonical event collapse to one durable item in the
   projection. The `privacy_safe_summary_label` field carries category-
   class tokens and counts only — never workspace, object, actor, or raw
   summary copy.
4. **OS-bound surfaces redact under policy.** Under quiet-hours-user,
   DND, presentation, screen-share, privacy-mode, and admin-suppression
   the projection sets `os_app_icon_badge_visible = false` and
   `lock_screen_summary_visible = false`. Focus-mode, reduced-attention,
   and power-saver intentionally preserve the OS app-icon badge so
   glanceable truth survives the hold.

## Cases

- `protected_walk_quiet_hours_holds_attention.json` — protected walk:
  scheduled user quiet hours are active, three indexer-warning emissions
  collapse to one durable item, and one critical-safety security
  advisory bypasses the hold. The badge projection counts deduped
  durable items, surfaces a privacy-safe `2 background items, 1 critical`
  label, and refuses the OS app-icon badge under the active mode.
- `failure_drill_dnd_during_sensitive_event.json` — failure drill: do-
  not-disturb is active while a sensitive (critical-safety) security
  advisory arrives. The advisory delivers on the durable_job_row,
  banner, and OS notification surfaces (DND must not gag tier_critical_
  safety). The badge projection holds the OS app-icon badge but
  preserves the in-product durable count and critical-safety subcount,
  and the privacy-safe label carries no raw object identity, actor, or
  summary copy.
- `desktop_notification_badge_reopen_audit.json` — alpha taxonomy proof:
  repeated failed test notifications coalesce under one durable truth
  object while quiet hours holds toast and OS fanout; a critical OS
  notification exposes privacy-safe summary copy, one safe primary
  action, and exact reopen; action-state rows prove dismiss,
  acknowledge, snooze, mute, resolve, and suppress reconcile badge
  counts without deleting durable history; the suppression audit report
  explains shown, held, deduped, and escalated outcomes.

## Layout

Each case carries:

- `__fixture__` — name, scenario, contract sections.
- `__source__` — `active_posture_modes` (the modes the shell-level
  posture had active when the snapshot was captured) and
  `envelope_emissions` (number of upstream envelope mints).
- `expected_routed[]` — one or more routed-notification records as
  emitted by `NotificationRouter`.
- `expected_badge_projection` — the `DurableBadgeProjection` derived
  from those routed records under the same posture.
- `expected_os_payload`, `expected_badge_reconciliation`,
  `expected_action_states`, and `expected_suppression_audit` appear on
  the alpha taxonomy proof case. They are produced by the external
  payload, action-state, badge, and audit modules rather than by
  surface-local heuristics.

## Re-blessing

Routing or posture behavior changes intentionally? Run

```sh
BLESS_QUIET_HOURS_FIXTURES=1 cargo test -p aureline-shell --test quiet_hours_protected_walk
```

to regenerate the fixtures from the current shell output.

Run

```sh
BLESS_NOTIFICATION_TAXONOMY_ALPHA=1 cargo test -p aureline-shell --test notification_taxonomy_alpha_tests
```

to regenerate the alpha taxonomy fixture and its YAML suppression-audit
artifact.
