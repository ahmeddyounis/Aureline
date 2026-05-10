# Interruptibility seed: quiet-hours, do-not-disturb, and privacy-safe badge behavior

This document is the **reviewer-facing entry** to the shell-level
quiet-hours / do-not-disturb / privacy-safe-badge posture frozen on the
notification truth lane.

Its job is narrow: show a reviewer where the seed lives, how the shell's
runtime posture flows through the notification envelope into the
[`NotificationRouter`](./notification_routing_seed.md), and how the
failure drill — a sensitive event arriving under DND — keeps durable
truth, the **critical-safety** tier, and the **privacy-safe** badge
honest. The upstream interruptibility, attention-taxonomy, OS-
notification / quiet-hours, and notification-routing contracts are
normative; this seed is the entry point and does **not** invent
additional vocabulary.

Where this document disagrees with the UI / UX Spec or with the upstream
attention-activity-taxonomy, interruptibility-arbitration, OS-
notification / quiet-hours, notification-envelope, or notification-
routing-seed contracts, the source spec wins and this seed plus its
companion artifacts must change in the same patch.

## Companion artifacts

- [`/crates/aureline-shell/src/notifications/quiet_hours.rs`](../../crates/aureline-shell/src/notifications/quiet_hours.rs)
  — the live `QuietHoursPosture` and `DurableBadgeProjection` types the
  shell consumes.
- [`/crates/aureline-shell/tests/quiet_hours_protected_walk.rs`](../../crates/aureline-shell/tests/quiet_hours_protected_walk.rs)
  — protected walk + failure drill integration tests.
- [`/fixtures/ux/quiet_hours_cases/`](../../fixtures/ux/quiet_hours_cases/)
  — worked posture + routing + badge fixtures.

## Upstream contracts (composed, not replaced)

This seed composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns the `quiet_hours_mode`, `interruptibility_tier`,
  `suppression_reason`, `delivery_surface_class`, `privacy_payload_class`,
  `dedupe_key_scheme`, and `reopen_target_kind` vocabularies the posture
  re-uses.
- [`interruptibility_arbitration_contract.md`](./interruptibility_arbitration_contract.md)
  owns the arbitration rules that decide whether an event interrupts,
  including the no-modal-during-typing posture, the protected-flow set,
  and the focus-steal-prevention rule.
- [`os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  owns suppression-record anatomy, payload-redaction rules, exact-reopen
  linkage on OS-bound surfaces, and the no-bypass posture for high-risk
  shortcuts.
- [`notification_envelope_contract.md`](./notification_envelope_contract.md)
  and [`notification_routing_seed.md`](./notification_routing_seed.md)
  own the envelope schema, the typed surface set, dedupe by
  `dedupe_key_scheme + dedupe_key_ref`, and the routing rules the
  posture's output flows through.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  owns per-`quiet_hours_mode` suppressed / preserved surface lists. The
  shell posture quotes that matrix when it decides whether the OS app-
  icon badge or the lock-screen summary may render.
- [`/artifacts/ux/quiet_hours_override_matrix.yaml`](../../artifacts/ux/quiet_hours_override_matrix.yaml)
  owns the four override event classes (security advisory, trust
  downgrade, approval expiry, route warning) under which a hold may
  break.

## Who reads this seed

- **Shell, activity-center, and OS-shim authors** wiring posture into
  the notification truth lane. Apply `QuietHoursPosture::apply_to_envelope`
  before routing; consume `DurableBadgeProjection` for badge surfaces.
- **Support, evidence, and parity-audit tooling** classifying the
  posture's effect on routed notifications by `active_modes`,
  `held_under_posture_count`, and `os_app_icon_badge_visible`.
- **Reviewers** confirming that durable history, critical-safety truth,
  and badge privacy survive every mode in the policy matrix.

## The posture at a glance

A [`QuietHoursPosture`](../../crates/aureline-shell/src/notifications/quiet_hours.rs)
carries the set of `quiet_hours_mode` values currently active on the
shell. The seed exposes:

- **`apply_to_envelope(&mut env)`** — write the posture's mode set into
  the envelope's `suppression_state` block before the router sees it.
  The posture **unions** with any modes already on the envelope (so an
  upstream subsystem that knew about `mode_admin_suppression` cannot have
  its modes silently dropped) and recomputes `suppressed`. Critical-
  safety severity always lands on `suppressed = false`; blocking-trust
  severity passes every user-scheduled mode but is held by
  `mode_admin_suppression`.
- **`holds_attention_for(severity)`** — predicate the activity center
  uses to decide whether a non-critical envelope routes to a quiet-
  hours digest under the current posture.
- **`suppresses_os_app_icon_badge()`** / **`suppresses_lock_screen_summary()`** —
  predicates the platform adapter reads when it decides whether to paint
  a count on the dock / taskbar / lock-screen surface. They quote the
  upstream policy matrix verbatim.

A [`DurableBadgeProjection`](../../crates/aureline-shell/src/notifications/quiet_hours.rs)
is built from a slice of `RoutedNotification`s plus the current posture.
Its required fields:

1. **Counts.** `durable_count` is the number of unique
   `canonical_event_id`s that delivered onto a durable surface. Three
   repeat emissions of the same canonical event count once.
   `severity_counts` buckets per severity. `critical_safety_count` is
   the number of critical items — always rendered, even under quiet
   hours. `held_under_posture_count` is the subcount currently held by
   the posture, useful for the exit-mode digest.
2. **Visibility flags.** `os_app_icon_badge_visible` and
   `lock_screen_summary_visible` mirror the upstream policy matrix.
3. **Privacy-safe summary.** `privacy_safe_summary_label` is a
   category-and-count string like `"2 background items, 1 critical"`.
   Raw object identifiers, actor identities, workspace labels, and raw
   summary copy never appear here.

## The protected walk

> **Enable quiet hours → trigger notifications → verify badges and
> suppression stay privacy-safe.**

1. **Enable.** The shell sets `QuietHoursPosture::quiet_hours_user()`.
2. **Trigger.** Three indexer-warning envelopes mint with the same
   `canonical_event_id`. The posture is applied to each; the router sees
   `suppressed = true` with `mode_quiet_hours_user` recorded.
3. **Route.** The first emission delivers on `durable_job_row`,
   holds the toast and OS notification (the receipt records
   `held_quiet_hours` so the held truth is auditable). The next two
   emissions emit `deduped_canonical_event` receipts on every surface;
   the reopen target ref is preserved across all three.
4. **Critical-safety bypass.** A security advisory then mints with
   `severity_class = critical`. The posture refuses to mark it
   suppressed; the router delivers on `durable_job_row`,
   `contextual_banner`, and `os_notification` even with the user's
   quiet-hours mode active.
5. **Project the badge.** The `DurableBadgeProjection` reports
   `durable_count = 2`, `critical_safety_count = 1`,
   `held_under_posture_count = 1`, `os_app_icon_badge_visible = false`,
   and `privacy_safe_summary_label = "2 background items, 1 critical"`.
   The badge does not inflate from the three repeat emissions; the
   privacy-safe label never names the indexer shard or the security
   advisory.

The worked example backing this walk is
[`/fixtures/ux/quiet_hours_cases/protected_walk_quiet_hours_holds_attention.json`](../../fixtures/ux/quiet_hours_cases/protected_walk_quiet_hours_holds_attention.json).

## The failure drill

> **Trigger do-not-disturb or quiet hours during a sensitive event and
> confirm badge behavior respects privacy and interruptibility policy.**

The drill exercises the posture's hardest invariant: a critical-safety
event arriving under DND must interrupt, and the badge derived from it
must remain privacy-safe.

1. **Pre-condition.** `QuietHoursPosture::do_not_disturb()` is active.
2. **Trigger.** A `severity_class = critical` security advisory arrives
   with `privacy_class = security_critical` and `privacy_payload_class =
   redacted_metadata_only`.
3. **Invariant.** The posture MUST NOT set `suppressed = true`. The
   active modes are still recorded for audit, but
   `suppression_reasons` is empty so the router knows the envelope is
   not held.
4. **Routing.** Every recommended surface delivers on the same
   `reopen_target_ref` so the activity center, the in-product banner,
   and the OS notification all reopen the same security review canvas.
5. **Privacy-safe badge.** The projection reports `durable_count = 1`,
   `critical_safety_count = 1`, `held_under_posture_count = 0`, and
   `os_app_icon_badge_visible = false` (DND policy still holds the OS
   app-icon badge for the wider stream — an OS-level redaction, not a
   suppression of the underlying truth). The
   `privacy_safe_summary_label` is `"1 background item, 1 critical"`;
   the raw advisory copy, the credential broker actor, and the
   canonical object identity are never echoed.

The worked example is
[`/fixtures/ux/quiet_hours_cases/failure_drill_dnd_during_sensitive_event.json`](../../fixtures/ux/quiet_hours_cases/failure_drill_dnd_during_sensitive_event.json).

## Mode-by-mode posture summary

The shell posture quotes the policy matrix verbatim; the table below
collapses the matrix to the slots the seed exposes.

| Mode | Holds non-critical attention surfaces | OS app-icon badge | Lock-screen summary |
| --- | --- | --- | --- |
| `mode_none` | no | shown | shown |
| `mode_quiet_hours_user` | yes | suppressed | suppressed |
| `mode_do_not_disturb_user` | yes | suppressed | suppressed |
| `mode_focus_mode_user` | yes | shown | suppressed |
| `mode_presentation` | yes | suppressed | suppressed |
| `mode_screen_share` | yes | suppressed | suppressed |
| `mode_privacy_mode` | yes | suppressed | suppressed |
| `mode_reduced_attention_policy` | yes | shown | shown |
| `mode_power_saver_runtime` | yes | shown | shown |
| `mode_admin_suppression` | yes | suppressed | suppressed |

Critical-safety severity bypasses every row above. Blocking-trust
severity bypasses every row except `mode_admin_suppression`.

## Non-conforming patterns

The following remain non-conforming under this seed:

- a posture that sets `suppressed = true` on a `severity_class = critical`
  envelope;
- a badge surface that counts raw deliveries instead of deduped durable
  items, inflating the count under retry storms;
- a privacy-safe summary label that echoes a raw path, raw URL, actor
  name, workspace label, or summary copy;
- a held envelope whose durable surfaces are stripped to zero (durable
  truth must always preserve);
- a posture that **widens** the envelope's mode set (only narrowing /
  unioning is conforming);
- an OS app-icon badge that paints a count under
  `mode_quiet_hours_user`, `mode_do_not_disturb_user`,
  `mode_presentation`, `mode_screen_share`, `mode_privacy_mode`, or
  `mode_admin_suppression` without explicit per-mode override.

## Evidence

Validation-lane evidence for this seed lives under
[`/artifacts/milestones/m1/`](../../artifacts/milestones/m1/) and is
registered in
[`artifact_index.yaml`](../../artifacts/milestones/m1/artifact_index.yaml)
under the `interruptibility_seed` lane. Refresh whenever the
`QuietHoursPosture` API, the badge projection shape, the protected-walk
fixture, the failure-drill fixture, or the policy-matrix slice the seed
quotes change.
