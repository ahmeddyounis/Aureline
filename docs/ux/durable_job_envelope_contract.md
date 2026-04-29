# Durable-job envelope contract: actor, target, phase, progress, sensitivity, affordance, badge-source, and notification-fanout grammar

This document freezes the durable-job envelope contract — the upstream
truth record that backs every long-running, queued, reviewable, held, or
suppressed piece of work in Aureline. It is the source the durable-work
row, status strip, OS notification, companion push, badge, and support
export all read from. Toasts, modal spinners, transient banners, and
status-bar flickers are render chrome; they are never the durable
truth.

The contract is normative. Where this document disagrees with the UI /
UX Spec sections it quotes, the source spec wins and this document plus
the schema and fixtures must change in the same patch. Where this
document disagrees with a downstream surface's private progress field,
job row, or notification linkback vocabulary, this document wins and
the surface is non-conforming.

## Companion artifacts

- [`/schemas/ux/durable_job_envelope.schema.json`](../../schemas/ux/durable_job_envelope.schema.json)
  defines the machine-readable `durable_job_envelope_record` carried at
  the cross-tool boundary.
- [`/fixtures/ux/durable_job_cases/`](../../fixtures/ux/durable_job_cases/)
  contains worked examples covering every progress-phase class and the
  badge-source and notification-fanout rules.

## Upstream and sibling contracts

This contract composes with existing owners and does not replace them:

- [`attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility tier,
  durable-job lifecycle state, quiet-hours mode, suppression reason,
  badge class, and source-subsystem vocabulary.
- [`schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
  owns the per-event delivery envelope, durable-job lifecycle record,
  attention-item record, badge record, grouped-burst record, reopen
  target record, quiet-hours policy row, and audit-event vocabulary.
- [`durable_work_contract.md`](./durable_work_contract.md) owns the
  render-level durable row (`job_row_record`); the row consumes the
  envelope and projects it into activity-center, task surface, and
  compact status-strip layouts.
- [`notification_contract.md`](./notification_contract.md) and
  [`notification_delivery_contract.md`](./notification_delivery_contract.md)
  own surface-class routing, dedupe, dismissal verbs, privacy payloads,
  and exact-reopen rules.
- [`status_strip_family_contract.md`](./status_strip_family_contract.md)
  owns the compact persistent strip for in-flight work.

The `durable_job_envelope_record` carries refs to those owners rather
than re-minting their fields.

## Scope

Frozen at this revision:

- one envelope shape carrying actor, subsystem, target identity, phase,
  progress, queue reason, privacy / cost / policy / network / trust /
  provider / recovery sensitivity, cancel / retry / open-details
  affordances, and canonical object linkage;
- the ten-class progress-phase grammar;
- badge-source partition rules so badge counts derive from durable
  rows or authoritative objects, never from surface-local spin loops;
- notification-envelope linkage and fanout-receipt fields so status
  strip, OS notifications, lock-screen summaries, and companion push
  share one canonical event/object lineage with the envelope.

Out of scope:

- the full activity-center backend, queue scheduler, or progress
  emitter implementation;
- notification transport, OS adapters, or companion sync wire formats;
- subsystem-specific run logs, evidence packets, or output viewers
  beyond opaque refs.

## Durable-job invariant

A durable-job envelope is the only legitimate truth for work that:

- can outlive the surface that started it (build, test, debug session,
  task run, indexer pass, save/sync, AI apply, review apply, package
  action, update install, download/upload, notebook execution,
  collaboration session, restore/recovery, remote attach, provider
  handoff, admin policy refresh);
- can affect cost, policy, network, trust, provider state, or recovery
  posture;
- can produce evidence, history, or follow-up burden.

If any of those apply, work must mint exactly one envelope. Render
surfaces — toast, banner, status item, durable row, OS notification,
lock-screen summary, companion push — must derive their state from the
envelope's `durable_job_id` plus `canonical_event_id`, never from a
private surface-local counter.

A surface that renders progress, queue, completion, failure, hold, or
suppression for durable work without naming an upstream envelope is
non-conforming.

## Required envelope anatomy

Every `durable_job_envelope_record` carries the same top-level anatomy.
Renderers may project subsets of the fields, but they may not drop the
backing fields or replace them with prose-only state.

| Field | Required content | Non-conforming collapse |
| --- | --- | --- |
| `envelope_id` | Opaque id of this envelope record. | A surface-local progress key with no durable identity. |
| `durable_job_id` | Stable durable-job identity shared across phase transitions, phase splits, and retries. | Minting a new id for each phase update. |
| `canonical_event_id` | Canonical event id used by toasts, badges, status strip, OS notifications, companion push, durable rows, and exported history. | A per-surface notification id with no durable lineage. |
| `canonical_object_target_ref` | Opaque ref to the canonical object the work concerns (review item, build, branch, session, artifact, provider grant, package). | Raw paths, raw URLs, or display text as identity. |
| `actor` | Source subsystem, actor identity, execution origin, client scope. | Copy that says "system" with no stable owner. |
| `target` | Target identity ref, target kind, scope label, object label, redaction class, source record refs. | Raw paths or display text used as the target ref. |
| `subject_class` | One of the frozen durable-job subject classes (build, test, debug session, save/sync, AI apply, etc.). | Surface-local subject names not in the frozen taxonomy. |
| `phase` | Phase class plus the phase fields the class requires (queue reason, waiting-for, approval source, failure summary, completion summary, hold reason). | Generic `Working` / `Done` / `Failed` strings with no class meaning. |
| `progress` | Progress-form class plus the companion fields the form requires (progress bar, indeterminate reason, counts). | Spinner-only progress with no phase, indeterminate reason, or actor label. |
| `sensitivity` | Privacy payload class and the cost / policy / network / trust / provider state / recovery posture impact axes. | Hiding billable, remote, policy, trust, or provider effects until completion. |
| `affordances` | Open-details affordance (required), cancel and retry affordance class, dismissal verbs available, additional action kinds. | Pointer-only controls, destructive shortcuts, or `Dismiss` as the only path on consequential work. |
| `badge_source` | Badge-source class, emitted badge classes, dedupe-key scheme, grouped-burst lineage, partition rule. | Badges whose counts derive from local surface counters rather than envelopes or canonical objects. |
| `notification_linkage` | Notification, status-strip, OS notification, lock-screen, and companion-push envelope refs plus fanout receipts. | Per-surface durable identity that does not collapse onto the canonical event/object lineage. |
| `lineage` | Event lineage ref, source envelope refs, cross-client lineage refs. | Re-minting lineage per device or per delivery. |
| `audit` | Audit-event refs covering create, phase change, completion, attention-required, cancel, hold, suppress, release. | Held or suppressed events with no audit trail. |
| `support_export` | Exportability, redaction class, export field refs, notification dedupe ref, raw-private-material exclusion flag. | Exports that cannot explain source, target, suppression, dedupe, or fanout. |
| `policy_context` | Policy epoch, trust state, execution context id when applicable. | Stale policy epoch implied by lack of revalidation. |
| `timestamps` | Minted, queued, started, last phase changed, held, finished as applicable. | Single mutable timestamp that loses lifecycle history. |
| `narrative_refs` | Refs to documents the envelope embodies. | Free-text without doc anchors. |

## Progress-phase grammar

Every envelope resolves to exactly one `phase_class`. These are display,
activity-center, status-strip, support-export, and audit classes — not
localized copy. The grammar is closed; new phases are additive-minor and
require a `durable_job_envelope_schema_version` bump.

| `phase_class` | Meaning | Required companion fields |
| --- | --- | --- |
| `preparing` | Pre-flight work has started but no executable phase has begun (resolution, capability probe, plan synthesis, manifest fetch). | `phase.phase_label`, `progress.progress_form_class` of `phase_only` or `spinner_only` with `indeterminate_reason_label`. |
| `queued` | Work is accepted but not executing yet. | `phase.queue_reason_label`, `phase.expected_boundary_class`. |
| `running` | Work is actively executing in foreground or background. | `phase.phase_label`, `progress.progress_form_class` of `labeled_progress_bar` or `spinner_only` with `indeterminate_reason_label`. |
| `waiting_input` | Work is paused on a user, policy, trust, provider, or admin decision. | `phase.waiting_for_label`, `phase.approval_source_label`, an `affordances.additional_action_kinds` entry containing `review_approval` (or another typed handoff). |
| `partially_complete` | Some targets, phases, or items succeeded while others failed, were skipped, hidden, or excluded. | `phase.failure_or_partial_summary_label`, `progress.included_count`, `progress.excluded_count`, `progress.failed_count`. |
| `completed` | Work completed successfully and is historical but reviewable. | `phase.completion_summary_label`, `timestamps.finished_at`. |
| `failed` | Work terminated unsuccessfully and requires follow-up. | `phase.failure_or_partial_summary_label`, `timestamps.finished_at`, an `affordances.additional_action_kinds` entry containing `retry` or a typed reason if retry is denied. |
| `cancelled` | A user, system, policy, or admin actor cancelled the work before completion. | `phase.failure_or_partial_summary_label` naming the cancellation actor and reason, `timestamps.finished_at`. |
| `held` | Interruption was intentionally delayed by a quiet-hours, focus, presentation, screen-share, privacy, power-saver, or reduced-attention mode; durable history is preserved. | `phase.hold_or_suppression_reason_label`, suppression reasons referencing the active mode, `audit.audit_event_refs` non-empty. |
| `suppressed` | Policy blocks full display, fanout, or action while preserving the audit trail. | `phase.hold_or_suppression_reason_label`, `phase.policy_source_ref` non-null, `audit.audit_event_refs` non-empty. |

Rules:

- An envelope must never carry two `phase_class` values; phase splits
  emit a new envelope under the same `durable_job_id` and
  `canonical_event_id`.
- `phase_age_label` is required on every envelope so a status strip,
  badge tooltip, or OS notification can show an authoritative age
  without re-deriving it locally.
- A `running` envelope without either a `labeled_progress_bar` or a
  `spinner_only` progress form with an `indeterminate_reason_label` is
  non-conforming.
- A `queued` envelope must name an `expected_boundary_class` so
  downstream surfaces can render a faithful waiting summary.
- `held` and `suppressed` envelopes must preserve durable history;
  silent drop is non-conforming.
- `completed`, `failed`, and `cancelled` envelopes must carry
  `timestamps.finished_at`.
- `waiting_input` envelopes must name the approval source even when the
  product cannot show the approver's identity (e.g., admin policy
  source, provider tenancy authority).

## Sensitivity and affordances

Sensitivity captures whether the work touches axes that change the
required render and disclosure rules:

- `affects_cost`, `affects_policy`, `affects_network`, `affects_trust`,
  `affects_provider_state`, `affects_recovery_posture` are independent
  booleans. Any `true` axis requires `detail_or_evidence_required = true`
  and a non-null `detail_or_evidence_ref`.
- `privacy_payload_class` controls what the envelope's summary label may
  carry on lock-screen-bound deliveries.
- A `sensitivity_summary_sentence` carries one short reviewable sentence
  describing the impact in product (never a raw payload).

Affordances expose the actions every render must offer or explain why
they are unavailable:

- `open_details` is **always required**; a durable envelope without an
  open-details affordance is non-conforming.
- `cancel` is required when the work is cancellable; a non-cancellable
  envelope must say so via `cancel.cancellability_class`.
- `retry` is required when the phase is `failed` or `partially_complete`
  and retry is allowed by policy; a denied retry must carry a typed
  reason rather than a missing button.
- `dismissal_verbs_available` enumerates which of `acknowledge`,
  `resolve`, `dismiss`, `snooze`, `mute`, `suppress` apply; verbs are
  not aliases of one another.
- Additional action kinds (`review_approval`, `acknowledge`,
  `export_support`, `open_history`, `release_hold`, `inspect_policy`,
  `open_evidence`) extend the affordance set without inventing
  surface-specific verbs.

## Badge-source partition rules

Badges and counts derive from the envelope or the canonical object
target — never from a surface-local counter. The envelope names how it
participates:

| `badge_source_class` | Meaning | Required companion content |
| --- | --- | --- |
| `derived_from_envelope_state` | The badge count is incremented once when this envelope enters the contributing phase and decremented once when it leaves it. | `emitted_badge_classes` non-empty; `dedupe_key_scheme`; `partition_rule_label` describing the contributing phase. |
| `derived_from_canonical_object` | The badge count is sourced from the canonical object's authoritative state (e.g., review-needs-attention count from the review object), not from envelope phase. | `emitted_badge_classes` non-empty; `partition_rule_label` referencing the object class; `canonical_object_target_ref` non-null. |
| `aggregated_grouped_burst` | The badge count reflects a grouped burst that collapses N envelopes into one durable row. | `grouped_burst_id_ref` non-null; `dedupe_key_scheme = grouped_burst_id`; `partition_rule_label` describing the burst class. |
| `not_a_badge_source` | This envelope contributes to no badges. | `emitted_badge_classes` is empty; downstream surfaces must not infer counts from this envelope. |

Rules:

- A surface that increments a badge count without resolving an envelope
  whose `badge_source_class ≠ not_a_badge_source` and whose
  `emitted_badge_classes` includes the badge is non-conforming.
- Mixing badge classes into one count is non-conforming. Each badge
  count must trace to one `badge_class` from the frozen taxonomy.
- Dedupe must use the named `dedupe_key_scheme`; ten retries against
  one pipeline collapse onto one envelope under
  `subsystem_plus_object_plus_phase` or `grouped_burst_id`, not ten
  contributions to a count.
- A held or suppressed envelope still emits to a badge with the
  `held_or_suppressed_count` class so suppression is never silent.
- Clearing a badge requires the envelope to leave the contributing
  phase or the canonical object to stop reporting the contributing
  state. Acknowledge removes the badge but not the underlying state.

## Notification-envelope linkage and fanout receipts

The envelope is the join key between durable rows and the
notification-delivery family. It carries explicit refs to delivery
envelopes on each fanout surface plus a typed receipt for each delivery
attempt. The transport itself remains owned by the notification and
delivery contracts.

Required linkage fields:

- `notification_linkage.shared_canonical_event_id` mirrors the envelope's
  `canonical_event_id`. It is repeated explicitly so an exporter or
  cross-tool reader does not need to walk the envelope to recover the
  shared id.
- `notification_linkage.shared_canonical_object_target_ref` mirrors the
  envelope's `canonical_object_target_ref` for the same reason.
- `notification_linkage.notification_envelope_refs` lists opaque refs to
  `notification_event_record` envelopes that mirror this durable job.
- `notification_linkage.status_strip_envelope_refs` lists refs to the
  compact persistent strip's surface envelopes.
- `notification_linkage.os_notification_envelope_refs` lists refs to OS
  notification deliveries.
- `notification_linkage.lock_screen_summary_envelope_refs` lists refs to
  lock-screen summaries.
- `notification_linkage.companion_push_envelope_refs` lists refs to
  companion-surface push deliveries.
- `notification_linkage.fanout_receipts` enumerates one receipt per
  delivery attempt, each carrying `fanout_surface_class`,
  `delivery_envelope_ref`, `receipt_state`, `dedupe_key_scheme`,
  `redaction_class`, and `client_scope`.

`receipt_state` values:

- `delivered` — a render-bearing surface presented the event;
- `held_quiet_hours` — held by a quiet-hours / focus / presentation /
  screen-share / privacy / power-saver / admin-suppression mode;
- `suppressed_policy` — admin or workspace policy blocked delivery;
- `deduped_canonical_event` — collapsed onto an existing envelope;
- `deduped_grouped_burst` — folded into a grouped burst;
- `released_from_hold` — the original held delivery surfaced via the
  release digest;
- `not_attempted_no_route` — no surface in the matrix matched the tier
  and client scope; the durable row remains the only render path.

Rules:

- Every fanout surface that mirrors a durable job must record exactly
  one receipt per attempt. Re-delivery emits a new receipt under the
  same envelope, not a new envelope.
- `dedupe_key_scheme` on a receipt must match the envelope's lineage
  scheme; cross-client divergence is non-conforming.
- A receipt carrying `delivered` for a tier that requires a durable
  surface (durable, actionable, blocking trust, critical safety) must
  reference an envelope that itself names the durable row reopen
  target.
- Activation of any fanout surface must reopen the narrowest truthful
  destination; a receipt whose `delivery_envelope_ref` resolves to a
  generic home screen is non-conforming.

## Support export and audit fields

Envelopes carry enough structured data for support export, notification
dedupe, and later audit without raw private material.

Required export/audit facts:

- `envelope_id`, `durable_job_id`, `canonical_event_id`,
  `canonical_object_target_ref`, and the subject and target classes;
- actor subsystem, actor identity ref, execution origin, client scope;
- phase class, phase label, age label, expected boundary class, and the
  phase-specific companion labels (queue reason, waiting-for, failure
  summary, completion summary, hold/suppression reason);
- progress form class, progress bar fields when present, indeterminate
  reason when present, included/excluded/failed counts when applicable;
- sensitivity axes, privacy payload class, and detail/evidence ref when
  required;
- affordance availability classes and disabled-reason labels;
- badge-source class, emitted badge classes, dedupe-key scheme, grouped-
  burst lineage, partition rule;
- notification, status-strip, OS notification, lock-screen, and
  companion-push envelope refs and fanout receipts;
- audit-event refs covering create, phase change, completion, attention,
  cancel, hold, suppress, and release.

Raw bodies, raw paths, raw URLs, raw host names, raw provider payloads,
raw prompt or completion text, raw command bodies, raw logs, tokens,
credential material, and secret values never appear in an envelope.

## Non-conforming cases

The following are contract violations:

- a long-running action exists only as a toast, status flicker, or
  modal spinner with no envelope;
- an envelope mints a fresh `canonical_event_id` per phase update;
- a `running` envelope carries no progress form;
- a `queued` envelope does not name `queue_reason_label` or
  `expected_boundary_class`;
- a `waiting_input` envelope does not name an approval source or a
  typed handoff action;
- a `failed` or `partially_complete` envelope omits the failure or
  partial summary;
- a `held` or `suppressed` envelope has no audit trail;
- a badge count derives from a local surface counter instead of an
  envelope or canonical object;
- a fanout surface mirrors a durable job without a typed receipt;
- a fanout receipt resolves to a generic home screen;
- a sensitive-axis envelope (cost, policy, network, trust, provider,
  recovery) lacks a detail or evidence ref;
- support export cannot explain source, target, phase, progress,
  sensitivity, affordances, badges, fanout receipts, suppression, or
  audit.
