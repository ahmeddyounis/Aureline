# Toast contract: acknowledgement-only posture, undo, and durable linkback

This document freezes Aureline’s toast contract: when a toast is
permitted, what toasts must **never** be used for, and which invariants
keep a toast from becoming the sole durable record of meaningful product
state.

The contract is normative. Where this document disagrees with the UI /
UX Spec or with upstream notification contracts, the source spec wins
and this document plus the schema and fixtures MUST change in the same
patch. Where a downstream surface uses a toast to carry durable degraded
state, policy denial, connectivity loss, blocking errors, or long-running
work without a durable owner, this contract wins and the surface is
non-conforming.

## Companion artifacts

- [`/schemas/ux/toast_event.schema.json`](../../schemas/ux/toast_event.schema.json)
  defines the `toast_event_record` and `toast_request_review_record`
  frozen by this document.
- [`/fixtures/ux/toast_cases/`](../../fixtures/ux/toast_cases/)
  contains worked cases for acknowledgement-only toasts, undo-capable
  toasts with durable linkback, deduped / grouped toast bursts, and a
  rejected attempt to model durable degraded state as toast-only.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`notification_contract.md`](./notification_contract.md) owns
  notification surface classes, canonical event identity, exact-reopen
  posture, and toast-only prohibitions at the notification layer.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  owns event lineage, delivery steps, action taxonomy, dismissal verbs,
  and the “no bypass on high-risk shortcuts” rule.
- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns interruptibility tiers, quiet-hours modes, dedupe schemes, and
  delivery-surface vocabulary.
- [`durable_work_contract.md`](./durable_work_contract.md) and
  [`chronology_row_contract.md`](./chronology_row_contract.md) own the
  durable activity / history surfaces to which a toast links back when
  the result remains meaningful after dismissal.
- [`shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  owns command parity, undo/revert safety posture, and evidence-carrying
  review requirements for consequential actions.

If this document conflicts with any upstream owner above, the upstream
owner wins and this document plus the schema and fixtures must change in
the same patch.

## Scope

Frozen at this revision:

- four toast posture classes: `simple_acknowledgement`,
  `toast_with_action`, `burst_grouped`, `reversible_mutation`;
- when a toast is allowed (acknowledgement, quick confirmation, undo
  opportunity, and brief mirrors only);
- forbidden toast-only uses (connectivity loss, policy denial, remote
  failure, blocking errors, and long-running work);
- timeout, dismissibility, and pause-on-hover/focus/screen-reader rules;
- action budget (`primary_action` + optional `undo_action`) and undo
  semantics;
- grouping / dedupe posture for repeated toasts; and
- durable rediscovery requirements and durable linkback structure when
  the result matters after dismissal.

Out of scope:

- OS-level notification adapters and lock-screen payload rules (see
  `os_notification_and_quiet_hours_contract.md`);
- motion, placement, z-order, and focus-trap rules (see
  `overlay_layer_contract.md`); and
- final copywriting, iconography, and component layout.

## 1. Toast posture classes

Toasts are categorized by **posture**, not by styling:

| Toast posture class | What it is | What it is not |
| --- | --- | --- |
| `simple_acknowledgement` | Auto-dismiss acknowledgement after a direct user action. | A durable record of state. |
| `toast_with_action` | Acknowledgement with one clear follow-up action. | A multi-action menu or a review surface. |
| `burst_grouped` | A single toast representing a deduped burst (counted, not stacked). | A stack of many near-identical toasts. |
| `reversible_mutation` | Acknowledgement of a small reversible mutation with `undo_action`. | The only route to recover from a consequential mutation. |

Posture is a contract axis: a toast that cannot be classified into one
of these four classes is non-conforming and must route through a banner,
status/task surface, activity row, review sheet, or another durable
owner.

## 2. Allowed uses

A toast is permitted only for:

1. **Short-lived success acknowledgement** after a direct user action
   where the result is immediately observable on a durable surface
   (for example: the affected object’s state indicator changes) and no
   follow-up is required.
2. **Undo opportunity** for a small reversible change when undo remains
   available through the same command graph path as the rest of the
   product’s undo/revert machinery.
3. **Quick confirmation** after direct user action (copy, toggle, pin,
   local-only intent) where the toast is not the only durable record of a
   change that matters later.
4. **A brief mirror of a durable surface** (job row, history row, object
   row, or banner) when the toast carries a durable linkback and does not
   become the source of truth.

Toasts are acknowledgement affordances; they are not a durable status
lane.

## 3. Forbidden toast-only uses

Toasts MUST NOT be the **only** representation of any of the following
state classes:

- connectivity loss or reconnecting loops;
- policy denial, trust boundary changes, or restricted-mode transitions;
- remote failures or provider-side denials that still matter after the
  toast disappears;
- blocking or repair-required errors; and
- long-running work, queued work, or work that requires review (these
  belong in durable job rows / task surfaces and the activity center).

A toast MAY mirror a durable owner for the above states only when the
toast record includes a durable linkback and the durable owner is
simultaneously reachable through non-toast navigation.

## 4. Timeout, dismissibility, and pause rules

Every toast is dismissible. A non-dismissible toast is non-conforming.

### 4.1 Timeout defaults and budgets

Toasts MUST auto-dismiss; a toast that is intended to remain until
resolved is a banner, a job row, or another durable surface.

Default timeouts (tunable by accessibility posture and content length):

- `simple_acknowledgement`: target ~4–6 seconds.
- `toast_with_action`: target ~8–12 seconds, with pause on hover/focus.
- `burst_grouped`: target ~8–12 seconds (grouped count must be readable).
- `reversible_mutation`: target ~10–15 seconds to support undo without
  hurry, with pause on hover/focus.

### 4.2 Pause conditions

When a toast includes `primary_action` or `undo_action`, the auto-dismiss
timer MUST pause during:

- pointer hover,
- keyboard focus within the toast, and
- screen-reader announcement.

Action-bearing toasts that dismiss while hovered, focused, or being
announced are non-conforming.

## 5. Actions and undo semantics

### 5.1 Action budget

A toast MUST expose at most:

- one `primary_action`, and
- one `undo_action` (only when posture is `reversible_mutation`).

Additional actions belong in a menu, banner, row, sheet, or a durable
detail surface.

### 5.2 Command-backed actions (no bypass)

All toast actions MUST route through a stable `command_id_ref`. A toast
that triggers hidden mutations, bypasses permission checks, bypasses
review/evidence paths, or mutates objects through a private callback is
non-conforming.

### 5.3 Undo posture

`undo_action` is only permitted when:

- the mutation is reversible without loss of user intent;
- undo uses the same command/evidence path as non-toast undo and records
  reversal in durable history where history exists; and
- undo does not bypass trust/policy checks that would apply to the same
  reversal invoked from a durable surface.

If the change is not reliably reversible, do not offer undo in a toast;
route to a durable surface and offer an explicit revert/review path.

## 6. Grouping and dedupe

Repeated toasts must not stack into spam. Producers MUST either:

- dedupe repeats into one toast using a stable key, or
- group a burst into one toast that shows `collapsed_toast_count` and
  offers an open-details path to the durable destination.

A grouped toast MUST be explicit about what was grouped
(`group_summary_label`) and MUST preserve canonical object identity
through `canonical_object_target_ref` and the durable linkback.

## 7. Durable rediscovery and linkback

When the toast describes a result that remains meaningful after the toast
dismisses (mutation, durable job completion, warning, policy boundary,
remote change), the user MUST be able to rediscover the same outcome
through a durable surface **with the same object identity and the same
summary wording**.

This is enforced mechanically via:

- `durable_linkback_policy.durable_link_required = true`, and
- a non-null `durable_linkback_policy.durable_linkback_target` carrying:
  `durable_target_kind`, `target_identity_ref`, and
  `durable_summary_label`.

## 8. Schema mapping

The schema binds this contract to two record shapes:

- `toast_event_record` — a conforming toast instance with posture,
  timer/dismissal/action policy, optional grouping/dedupe metadata, and
  a durable linkback when required.
- `toast_request_review_record` — a routing / conformance review record
  showing that a toast-only request was rejected (or downgraded to a
  mirror) because a durable owner is required.

