# Notification contract: accountable surfaces, exact reopen, and badge semantics

This document freezes the high-level notification contract for Aureline.
It sits above the delivery-lineage contract so product surfaces can decide
whether an event is a toast, banner, activity-center row, digest group, or
system notification without losing the canonical event id, actor/source,
quiet-hours posture, badge meaning, or exact reopen target.

The contract is normative. Where this document disagrees with the UI / UX
Spec or with the lower-level attention-routing contracts, the source spec
wins and this document, schema, and fixtures must change in the same patch.
Where a downstream surface creates a private notification class, private
badge total, or generic reopen destination, this contract wins and the
surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  defines the `notification_event_record` that every notification,
  badge, digest, and reopen path can be reviewed against.
- [`/fixtures/ux/notification_cases/`](../../fixtures/ux/notification_cases/)
  contains worked cases for deduped toasts, escalated banners, grouped
  provider events, and reopen-after-dismiss paths.

## Upstream contracts

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility tier,
  quiet-hours mode, suppression reason, privacy payload class, dedupe
  scheme, dismissal verb, and reopen-target vocabulary.
- [`toast_contract.md`](./toast_contract.md) owns toast-only posture,
  undo semantics, action budget, and durable rediscovery requirements for
  acknowledgement toasts.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  owns canonical event lineage, delivery steps, release steps, durable
  linkbacks, and action taxonomy.
- [`durable_work_contract.md`](./durable_work_contract.md) owns the
  durable activity row a user returns to after a transient surface
  disappears.
- [`shell_close_reopen_contract.md`](./shell_close_reopen_contract.md)
  owns shell slot, focus-return, placeholder, and no-rerun restore
  behavior.
- [`chronology_row_contract.md`](./chronology_row_contract.md) owns the
  durable history row after an event leaves immediate attention.

## Scope

This revision freezes:

- five notification surface classes: `toast`, `banner`,
  `activity_center_row`, `digest_group`, and `system_notification`;
- severity, durability, grouping, quiet-hours, and badge-count semantics
  for every emitted notification event;
- required actor/source, age, expiry, next-action, visibility-scope,
  canonical-object, event-lineage, and exact-reopen fields;
- dedupe, escalation, snooze, quiet-hours, digest-release, and
  cross-window behavior for repetitive or grouped signals;
- the rule that toasts are acknowledgement affordances or mirrors only,
  never the durable representation of outages, policy denials,
  connectivity loss, or long-running work.

Out of scope:

- platform adapter APIs for macOS, Windows, Linux, browser, or mobile
  notifications;
- final copy, iconography, animation, and layout;
- the eventual Rust notification-router crate.

## Required event anatomy

Every `notification_event_record` carries:

| Field | Required meaning | Non-conforming collapse |
| --- | --- | --- |
| `canonical_event_id` | Same id used by envelopes, lineages, badges, durable rows, system notifications, and history. | Per-surface notification ids that cannot be joined. |
| `event_lineage_id_ref` | Opaque ref to the event-lineage record. | One-off strings with no delivery history. |
| `source_subsystem` and `source_event_ref` | Stable owner and owner-side event ref. | "System" or "Background task" with no source. |
| `actor_identity_ref` | Opaque actor of record, including system actors. | Raw names, raw email, or missing actor. |
| `event_age_millis`, `occurred_at`, `last_updated_at` | Age and absolute time anchors. | Relative copy only. |
| `expires_at` and `expiry_policy_class` | Whether the event expires, waits for a condition, or remains until resolved. | Vanishing state with no rule. |
| `visibility_scope` | One of `local_only`, `workspace_visible`, or `provider_shared`. | Local and provider-visible events mixed in one lane. |
| `next_action` | Command-backed next action and exact target. | A vague `Open` that lands on a generic surface. |
| `surface_instances` | One row per rendered surface instance. | Toast, banner, badge, and OS notification each inventing local truth. |

Raw paths, raw URLs, provider payloads, prompt/completion text, command
bodies, secret material, and customer-owned identifiers do not appear in
the event record. Use opaque refs and privacy-safe labels.

## Surface Classes

| Surface class | Use | Durability | Grouping | Quiet-hours behavior | Badge semantics |
| --- | --- | --- | --- | --- | --- |
| `toast` | Immediate acknowledgement, reversible success, or a transient mirror of a durable row. | `ephemeral_acknowledgement` or `ephemeral_mirror_with_durable_link`. | May dedupe by `canonical_event_id`; never stack repeated copies. | May be held or visually reduced. Held copies release as a digest only when a durable record exists. | Does not count raw deliveries. May clear an acknowledgement badge or mirror a deduped durable item. |
| `banner` | Workspace, trust, policy, outage, connectivity, degraded, or migration state. | `persistent_until_condition_clears` unless a review surface supersedes it. | One banner per canonical object / event class. Repetition updates age and cause. | Blocking or critical banners render immediately or use an explicit admin-narrowed durable-only rule. | May increment `security_notices`, `failed_runs`, or `held_or_suppressed_count` only from deduped durable items. |
| `activity_center_row` | Durable job, retryable failure, queued review item, long-running progress, or reviewable completion. | `durable_until_resolved_or_archived`. | Rows dedupe by canonical event or grouped burst. | Always preserved. Transient interruptions may be held, but the row remains. | Badge counts derive from durable rows, not notification deliveries. |
| `digest_group` | Collapsed burst, quiet-hours release, or grouped provider updates. | `digest_until_reviewed`. | Must name group key, member count, first/latest lineage refs where available, and release rule. | Held events release as one digest grouped by source and severity. Urgent or blocking members must escape the digest through an explicit escalation. | `member_count` may display, but app badges count the digest or durable items by class, not every raw event. |
| `system_notification` | OS notification, lock-screen summary, companion push, or platform badge mirror. | `os_managed_mirror`. | Mirrors product lineage; never mints a separate provider/device event. | Respects quiet hours, privacy mode, lock-screen redaction, and admin narrowing. Critical safety cannot be silently blocked. | Mirrors deduped product badges only. It does not become the source of truth. |

## Toast Prohibitions

A toast is non-conforming when it is the only surface for any of these
state classes:

- `durable_outage`
- `policy_denial`
- `connectivity_loss`
- `long_running_work`
- `review_required`

Those states require a banner, activity-center row, attention item,
review sheet, or another durable owner. A toast may mirror the durable
surface only when it carries the same `canonical_event_id`, the same
canonical object target, and a non-empty durable linkback.

## Dedupe, Grouping, and Escalation

Repeated deliveries collapse into one lineage. Producers must choose one
dedupe scheme from the attention taxonomy and preserve it in
`dedupe_policy`:

- exact repeat: `canonical_event_id`;
- same object and event class: `canonical_object_target_plus_event_class`;
- burst by subsystem/object/phase: `subsystem_plus_object_plus_phase`;
- digest: `grouped_burst_id`;
- cross-client duplicate: `cross_client_canonical_event_id`.

Escalation requires a typed reason. A digest that later contains a
blocking member must record the escalation and move the blocking member
to a banner, activity row, review surface, or redacted system
notification. Decorative escalation is non-conforming.

## Quiet Hours and Digest Release

Quiet hours may delay interruption; they may not discard durable
history. Every surface instance records:

- active quiet-hours modes at mint time;
- suppression reasons when any delivery is held;
- whether delivery is held, durable-only, released as a digest, rendered
  immediately, or admin-narrowed;
- whether the event is urgent or blocking and which explicit exception
  rule applies.

Blocking or critical notifications cannot be hidden in a digest with no
rule. If admin policy narrows a blocking event to durable-only delivery,
the record must say so through `urgent_or_blocking_rule =
admin_narrowed_durable_only`.

## Exact Reopen

Clicking a notification or invoking its command reopens the narrowest
truthful destination:

- `canonical_object` for the exact review, build, branch, artifact,
  session, provider grant, or route object;
- `canonical_route` for a stable product route with target arguments;
- `review_context` for review sheet, diff, approval, or evidence
  context;
- `durable_activity_row` for the durable activity-center row;
- `digest_group` for the digest with member list and source filters;
- `placeholder_announced` when the target is missing, moved, blocked by
  policy, or unavailable;
- `denied_requires_revalidation` when fresh user intent is required
  before reopening.

Generic home-screen reopen is not a valid target. Reopen after dismiss,
quiet-hours release, cross-window activation, or OS notification
activation must resolve through the same canonical object target or an
announced placeholder.

## Dismiss, Acknowledge, Snooze, Mute, Resolve

The verbs remain distinct:

- `dismiss` removes a transient surface and preserves durable history;
- `acknowledge` clears unread/badge attention without changing the
  source object;
- `snooze` requires a resume condition and does not erase history;
- `mute` silences one class and preserves audit;
- `resolve` requires a real source-object change or explicit mark-done.

The event record makes this mechanically reviewable through
`dismissal_policy`. Dismissing a toast cannot erase, acknowledge,
resolve, suppress, archive, or mutate the durable row.

## Cross-Window and Cross-Client Rules

Every event records an owning window and a focus-transfer rule:

- return to the origin window when the canonical target still belongs
  there;
- preserve an existing owner window for durable job/detail surfaces;
- open in the invoking window only when the canonical object is visible
  and policy admits it;
- show an announced placeholder in the invoking window when the exact
  target cannot be reopened there;
- deny cross-window reopen when trust, provider authority, or
  revalidation rules require it.

Provider-shared and companion-visible notifications reuse the same
canonical event id and lineage. They do not mint per-device or
per-provider notification truth.

## Fixture Coverage

The fixtures under `/fixtures/ux/notification_cases/` cover:

- deduped toasts that remain acknowledgement-only;
- a connectivity-loss event escalated into a persistent banner and
  activity row instead of a toast;
- grouped provider events released from quiet hours as one digest with
  provider-shared visibility;
- a dismissed toast whose reopen path still lands on the durable review
  context.
