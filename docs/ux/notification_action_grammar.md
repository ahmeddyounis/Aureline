# Notification action grammar: stable verbs, suppression ledger, and reopen-without-replay rules

This contract freezes the **action semantics** for notification-adjacent
surfaces so the activity center, OS notifications, lock-screen summaries,
dock/taskbar mirrors, system-tray summaries, and companion push alerts never
invent private meanings for the same user intent.

The goal is narrow: the verbs **dismiss**, **snooze**, **acknowledge**,
**resolve**, **archive (dismiss to history)**, **mute source**, and
**reopen** mean the same thing everywhere, preserve one canonical object
identity, and never enable high-impact “shortcut replay” from summary-only
surfaces.

The contract is normative. Where this document disagrees with the UI / UX
Spec, the source spec wins and this document plus its schema and fixtures
must change in the same patch. Where a downstream surface invents a private
dismissal alias, a private snooze rule, a private reopen target, a private
archive-as-delete behavior, or a privileged mutation shortcut, this contract
wins and the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/notification_suppression_ledger.schema.json`](../../schemas/ux/notification_suppression_ledger.schema.json)
  — boundary schema for the suppression-history ledger that records *why*
  a delivery was delayed, coalesced, muted, or withheld **without implying**
  the underlying event was deleted.
- [`/fixtures/ux/notification_action_cases/`](../../fixtures/ux/notification_action_cases/)
  — worked YAML fixtures covering dismiss vs archive, snooze/mute ledger
  entries, dedupe coalescing, and reopen-without-replay from OS/companion
  surfaces.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface, suppression-reason, quiet-hours, badge-class,
  dismissal-verb, and reopen-target vocabularies.
- [`notification_contract.md`](./notification_contract.md) and
  [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  own notification surface-class, durability, badge policy, and exact-reopen
  requirements per event.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  and [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  own canonical event lineage, routing, dedupe collapse, delivery/dismissal/
  reopen/release steps, and the action taxonomy record fields.
- [`os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  and [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  own per-surface suppression records, privacy-safe payload rules, and the
  forbidden shortcut-action classes for OS/companion surfaces.
- [`durable_work_contract.md`](./durable_work_contract.md),
  [`exact_reopen_drill_map.md`](./exact_reopen_drill_map.md), and
  [`/schemas/ux/job_row_retention.schema.json`](../../schemas/ux/job_row_retention.schema.json)
  own durable-row retention, dismiss-to-history transitions, and the audit
  events that prove a durable row was preserved even when transient chrome
  was dismissed.

## 1. Objects and identity (frozen)

The verbs in this contract apply to two *related* objects that must never
diverge in identity:

1. **Canonical event / notification identity**
   - `canonical_event_id` is the single id shared by toast, banner, activity
     center row, digest group, OS notification, lock-screen summary, and
     companion push.
   - `event_lineage_id_ref` binds every delivery step and every dismissal /
     reopen step to the same trail.
2. **Canonical durable target identity**
   - Durable work and reviewable attention routes resolve to a durable
     identity (`durable_job_id_ref` / durable row id / attention item id).
   - Transient surfaces are permitted to disappear; the durable target is
     not.

Any action implementation that **mints a new canonical event id** or
**splits the canonical object identity** as a side effect of a surface-local
gesture is non-conforming.

## 2. Action grammar (meaning must not drift)

The verbs below are **not synonyms**. If a surface needs new microcopy it
MUST still map back to one of these verbs and preserve the semantics below.

### 2.1 `dismiss` (close transient chrome only)

Meaning:

- removes a transient delivery surface (toast, banner, OS notification
  presentation) from view;
- MAY clear a badge/unread marker for the *delivery instance*; and
- MUST NOT mutate the underlying canonical object or durable row.

Non-conforming behavior:

- treating dismiss as delete/resolve/archive of the durable row;
- dismissing one surface but minting a new `canonical_event_id` so the item
  appears “new” elsewhere.

### 2.2 `acknowledge` (clear attention without changing the source object)

Meaning:

- clears unread/attention and the associated badge-class count; and
- MUST NOT change the underlying canonical object state.

`acknowledge` is permitted as a **privacy-safe shortcut action** on OS and
companion surfaces because it does not mutate the source object
(`allowed_action_kind = acknowledge_clears_badge_only`).

### 2.3 `snooze` (deferral with an explicit resume condition)

Meaning:

- defers re-interruption and clears the badge temporarily; and
- MUST record a *resume condition* (“when it comes back”) that is stable and
  inspectable.

Rules (frozen):

- `snooze` MUST record a non-empty resume condition label (see
  `snooze_resume_condition_label` on the lineage dismissal step record).
- `snooze` MUST write a suppression-ledger entry that names
  `suppression_reason = class_snoozed_by_user` and preserves the canonical
  event id and durable linkback.
- OS/companion shortcut snooze is permitted only for the bounded action
  kinds frozen by the OS notification contract
  (`allowed_action_kind = snooze_until_unlocked` or
  `snooze_until_resume_condition`).

### 2.4 `mute source` (silence a class or source without deleting history)

Meaning:

- silences one badge/notification class (or a narrower source scope) so
  future deliveries are suppressed; and
- MUST preserve durable history and audit truth for already-minted events.

Rules (frozen):

- `mute source` MUST write a suppression-ledger entry that names
  `suppression_reason = class_muted_by_user`.
- A muted class MUST still coalesce/dedupe into the same canonical objects;
  muting is not a license to mint surface-local duplicates.

### 2.5 `resolve` (source-object mutation)

Meaning:

- records that the underlying canonical object changed state (fixed,
  approved, completed, or explicitly marked done); and
- MUST be backed by a *real* source-object mutation or an explicit mark-done
  action admitted by the source object model.

Rules (frozen):

- `resolve` MUST be recorded as a mutating action on the lineage dismissal
  step (`mutates_source_object = true`).
- OS/companion summary surfaces MUST NOT complete `resolve` directly when it
  is high-impact; they may only **reopen into** the owning in-product
  surface that performs the same preview/approval/revalidation requirements
  as the original workflow.

### 2.6 `archive` (dismiss to history; never delete)

Archive is a durable-row lifecycle operation, not a synonym for dismiss:

- archive removes an item from the “active” activity-center partitions by
  moving it to history-lane or an equivalent retained view; and
- archive MUST preserve durable identity, audit linkage, and exportability.

Rules (frozen):

- Archive MUST be represented as a **dismiss-to-history** retention
  transition (see `release_trigger_class = user_dismiss_to_history` and the
  retention audit event kind `job_row_dismissed_to_history`).
- Archive MUST NOT imply the underlying event “disappeared”; the canonical
  event and its durable linkback remain reachable by id.

### 2.7 `reopen` (exact reopen, without shortcut replay)

Reopen means “take me back to the narrowest truthful owning surface”:

- reopen resolves through the durable linkback contract and the exact-reopen
  drill map; and
- reopen is distinct from “replay the original action”.

Rules (frozen):

1. **Reopen preserves identity.** `canonical_event_id` and the canonical
   object target are invariant under reopen, including after quiet-hours
   hold, dedupe collapse, dismiss, snooze, mute, or archive.
2. **Reopen never lands on a generic home screen.** If the exact target is
   missing, moved, policy-blocked, or unavailable, reopen MUST land on an
   announced placeholder or a typed revalidation denial.
3. **Shortcut replay is forbidden.** OS-level actions, lock-screen quick
   actions, dock/taskbar shortcuts, system-tray actions, and companion push
   actions MUST NOT execute any action that matches a forbidden shortcut
   action class (for example `irreversible_high_blast`,
   `bypass_review_sheet`, `provider_grant_change_from_os_shortcut`).
   Instead, they MUST reopen into the authoritative product surface where
   preview, approval, and revalidation gates are enforced.

