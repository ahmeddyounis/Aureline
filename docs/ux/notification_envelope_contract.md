# Notification-envelope contract: privacy class and fanout receipts

This document freezes the **notification-envelope** contract for Aureline.
It exists so toasts, banners, status items, durable activity rows, native OS
notifications, and companion fanout all derive from one typed object model
with stable ids, stable enums, explicit privacy gating, explicit dedupe, and
explicit reopen targets.

The contract is normative. Where this document disagrees with the UI / UX
Spec, the source spec wins and this document, schemas, and fixtures must be
updated in the same change.

## Companion artifacts

- [`/schemas/ux/notification_envelope.schema.json`](../../schemas/ux/notification_envelope.schema.json)
  defines the `notification_envelope_record` and the frozen privacy-class rule
  rows.
- [`/schemas/ux/fanout_receipt.schema.json`](../../schemas/ux/fanout_receipt.schema.json)
  defines the `fanout_receipt_record` carried by notification envelopes and
  other durable truth objects.
- [`/fixtures/ux/notification_envelope_cases/`](../../fixtures/ux/notification_envelope_cases/)
  contains worked examples for cross-surface reconstruction, privacy gating,
  and suppressed fanout visibility.

## Upstream contracts (composed, not replaced)

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md) owns
  delivery-surface class, attention class, interruptibility tier, quiet-hours
  mode, suppression reason, payload redaction posture, dedupe-key scheme,
  reopen-target kind, dismissal verbs, and source-subsystem vocabulary.
- [`notification_contract.md`](./notification_contract.md) and
  [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  own the surface-class routing and exact-reopen requirements for the
  rendered notification family.
- [`notification_delivery_contract.md`](./notification_delivery_contract.md) and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  own canonical event lineage and cross-surface delivery steps.
- [`durable_job_envelope_contract.md`](./durable_job_envelope_contract.md) owns
  durable work envelopes and their linkage to notification deliveries.

## Scope

This contract freezes:

- the envelope identity fields used to join in-product, OS, and companion
  fanout records without minting per-surface ids;
- the privacy class and its rule rows so lock-screen/native payloads, support
  exports, and companion summaries remain consistent;
- the stable action identity fields so surfaces share one action vocabulary
  while localized message copy remains revisable;
- the fanout-receipt shape so suppressed, deduped, stale, or dropped fanout
  remains visible truth and stays joinable to the originating envelope.

Out of scope:

- platform adapter implementation (macOS/Windows/Linux notification APIs),
  companion transport code, or retry scheduling logic;
- UI layout and final copy decisions beyond the privacy-safe label limits.

## Required envelope anatomy

Every `notification_envelope_record` carries:

| Field | Required meaning | Non-conforming collapse |
| --- | --- | --- |
| `notification_envelope_id` | Stable envelope id for this notification object. | Per-surface ids with no join key. |
| `canonical_event_id` | Stable join id shared across all deliveries and clients. | Re-minting event ids on every surface. |
| `event_lineage_id_ref` | Ref to the lineage trail for this canonical event. | Dropping lineage so suppression is invisible. |
| `source_subsystem` + `source_event_ref` | Stable producer owner + producer-side ref. | “System” with no owning subsystem. |
| `severity_class` | Stable severity vocabulary for routing and escalation. | Free-text severity or per-surface synonyms. |
| `privacy_class` | Coarse privacy gate for OS payload, companion fanout, and export posture. | Implied privacy from message copy. |
| `privacy_payload_class` + `redaction_class` | Concrete enforcement posture for lock-screen/OS surfaces and export sinks. | OS payloads guessing what is safe. |
| `dedupe_key_scheme` + `dedupe_key_ref` | Stable dedupe join key for collapse across retries and clients. | Spamming repeated toasts/badges per retry. |
| `canonical_object_target_ref` | Opaque ref to the durable object of truth. | Raw paths/URLs or missing object linkage. |
| `recommended_surfaces` | Typed surface set used by routers and mirrors. | Private routing logic per surface. |
| `reopen_target` | Stable reopen target kind + id/ref. | Generic “Open” landing on a home screen. |
| `actions[]` | Stable action ids + targets; labels remain localizable. | Surfaces inventing local action semantics. |
| `suppression_state` | Quiet-hours/admin suppression facts and reasons. | Silent drop on hold or policy suppression. |
| `fanout_receipts[]` | One receipt per delivery attempt or hold state. | Best-effort transport with no visible outcome. |

## Privacy-class rules

The envelope carries both a coarse `privacy_class` and concrete enforcement
fields. The coarse class is the stable policy label; the concrete fields are
the enforcement knobs used by adapters and exports.

Rules:

1. `privacy_class` is stable and must not be repurposed. Adding a new value is
   additive; changing meaning of an existing value is breaking.
2. `privacy_payload_class` governs what the OS/lock-screen/companion payload is
   allowed to render. A payload must never widen beyond the class.
3. `redaction_class` governs what can appear in support exports and other
   durable sinks. A surface must never export beyond the class.

## Fanout receipts

Every fanout surface that mirrors a notification envelope records one receipt
per attempt (including holds and no-route outcomes) so “failed” and
“suppressed” remain visible truth.

Each `fanout_receipt_record` carries:

| Field | Meaning |
| --- | --- |
| `source_notification_envelope_id_ref` | Join back to the originating envelope. |
| `canonical_event_id` + `event_lineage_id_ref` | Join keys for cross-surface reconstruction. |
| `fanout_surface_class` | Which surface was attempted (toast, OS, companion, etc.). |
| `client_scope` | Destination client class (desktop, companion, managed admin, etc.). |
| `receipt_state` | Delivered/held/suppressed/deduped/no-route outcome. |
| `stale_or_undelivered_reason` | Typed reason + optional label when not delivered or stale. |
| `delivery_envelope_ref` | Ref to the per-surface delivery envelope when one exists. |
| `reopen_target_ref` | The canonical reopen target used on activation. |

Rules:

- A fanout receipt must exist even when the surface does not render
  (`receipt_state = held_quiet_hours`, `suppressed_policy`, or
  `not_attempted_no_route`).
- Receipts must preserve dedupe identity: `dedupe_key_scheme` on receipts must
  match the envelope’s scheme; cross-client divergence is non-conforming.
- Activation must resolve to the same canonical reopen target; a receipt whose
  reopen target is a generic home surface is non-conforming.

## Stable ids and localized copy

The contract keeps stable ids and enums as the joinable truth:

- ids: `notification_envelope_id`, `canonical_event_id`, `event_lineage_id_ref`,
  `dedupe_key_ref`, `reopen_target_ref`, `command_id`;
- enums: `source_subsystem`, `severity_class`, `privacy_class`,
  `privacy_payload_class`, `redaction_class`, `dedupe_key_scheme`,
  `fanout_surface_class`, `fanout_receipt_state`.

User-facing copy remains localizable and revisable:

- `summary_label` and `actions[].label` are not stable keys; they are localized
  strings constrained only by privacy rules.
- Surfaces must bind user-facing copy to stable ids and enums; they must not
  infer semantics from copy.

## Non-conforming cases

The following are contract violations:

- per-surface notification ids that cannot be joined across desktop/OS/companion;
- OS/lock-screen payloads that widen beyond the envelope’s privacy class;
- suppressed or dropped fanout with no receipt trail;
- repeated deliveries that mint new canonical event ids instead of deduping;
- reopen activation that lands on a generic home screen rather than the
  canonical object/route target;
- actions that are not command-backed by stable ids.

