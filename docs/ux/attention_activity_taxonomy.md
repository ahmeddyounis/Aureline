# Attention-routing, durable job-row, activity-center, quiet-hours, and reopen-semantics taxonomy

This document is the **shell-wide taxonomy** for how events reach
a user across toast, contextual banner, status item, durable job
row, attention item, activity-center digest card, OS notification,
OS badge, lock-screen summary, and companion-push surfaces. It
exists so every owning subsystem — editor, terminal, review /
diff, palette / search, install / update / attach, AI apply,
collaboration, provider-bearing, docs / help / service-health,
support / export, build system, test runner, debug session, task
runner, indexer, VFS save, sync / mirror, notebook kernel, remote
agent, extension host, workspace trust, policy resolver, admin
policy, secret broker, runtime power manager, shell — uses **one
canonical event id, one envelope shape, and one set of routing
rules** when it delivers, dedupes, holds, releases, and reopens a
user-visible event.

The taxonomy is normative. Where this document disagrees with the
UI / UX Spec sections it quotes, the source spec wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's mint of its own delivery
vocabulary, this document wins and the surface is non-conforming.

The companion artifacts are:

- [`/docs/ux/notification_contract.md`](./notification_contract.md)
  and
  [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  — the surface-class contract for toast, banner,
  activity-center row, digest group, and system notification
  records. It reuses this taxonomy's axes and adds required
  actor/source, age, expiry, next-action, visibility-scope,
  badge-count, quiet-hours, and exact-reopen fields.
- [`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
  — boundary schema every non-owning surface reads. Freezes the
  `activity_event_envelope_record`, the `durable_job_record`,
  the `attention_item_record`, the `badge_record`, the
  `grouped_burst_record`, the `reopen_target_record`, the
  `quiet_hours_policy_row_record`, and the
  `activity_event_audit_event_record`.
- [`/docs/ux/durable_work_contract.md`](./durable_work_contract.md)
  and [`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json)
  define the render-level durable row contract for state classes,
  progress forms, activity-center partitions, notification linkbacks,
  and support-export fields. They compose with this taxonomy rather
  than re-minting event-envelope vocabulary.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  — one row per frozen `quiet_hours_mode` declaring what delivery
  surfaces are suppressed, what is preserved, and that durable
  history and the critical-safety tier always render.
- [`/artifacts/ux/interruptibility_escalation_seed.yaml`](../../artifacts/ux/interruptibility_escalation_seed.yaml)
  — per-tier escalation rules, required triggers, and protected-
  path rules.

This taxonomy rides alongside — it does not re-mint — the
vocabularies frozen in:

- [`docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — authority, consequence, revert, representation, focus-return,
  and responsive-fallback vocabularies. An envelope that reports a
  consequence-bearing interaction quotes its
  `interaction_safety_packet_record` by ref; this taxonomy adds
  routing, durability, and quiet-hours posture without re-minting
  those axes.
- `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
  — authority class and freshness hint on every rendered row.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — redaction pass runs before bytes reach any persistent or
  exportable sink, including OS notification payloads and lock-
  screen summaries.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow; policy MAY NOT silently widen.
- `docs/adr/0009-execution-context-and-scope.md`
  — `execution_context_id` is re-exported; routing never invents
  a parallel scope vocabulary.
- `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
  — provider-bearing deliveries quote the browser-handoff packet
  by ref rather than launching raw URLs from a notification.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class` are
  re-exported without modification.
- `docs/adr/0013-docs-help-service-health-truth.md`
  — docs-health deliveries attribute bytes to citation anchors
  when the row quotes authoritative material.

## Who reads this document

- **Shell / notification / activity-center authors** minting the
  first-party delivery surfaces. They honour one vocabulary, one
  envelope shape, one dedupe lineage, and one reopen contract
  across every surface.
- **Owning subsystems** (editor, terminal, review / diff, install /
  update / attach, AI apply, collaboration, provider-bearing,
  docs / help / service-health, support / export, build system,
  test runner, debug session, task runner, indexer, VFS save,
  sync / mirror, notebook kernel, remote agent, extension host,
  workspace trust, policy resolver, admin policy, secret broker,
  runtime power manager) minting envelopes for their events.
- **Support, parity-audit, and cross-client routing tooling**
  reading each axis mechanically — every axis is separately
  addressable even when the surface folds several into one
  notification line.

## One taxonomy, four kinds of surfaces, one envelope

The taxonomy applies uniformly to four kinds of surfaces. A
surface that mints a private delivery vocabulary, its own attention
class set, its own interruptibility tier names, its own quiet-
hours semantics, its own badge class set, or its own reopen verb
set is non-conforming.

| Surface kind                       | Delivery-surface classes it owns                                                                                                                                |
|------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Transient in-product surface**   | `toast`, `contextual_banner`, `status_item`                                                                                                                      |
| **Durable in-product surface**     | `durable_job_row`, `attention_item`, `activity_center_digest_card`, `digest_group_row`                                                                           |
| **OS-level and companion surface** | `os_notification`, `os_badge_app_icon`, `lock_screen_summary`, `companion_push`                                                                                  |
| **Held surface**                   | `not_delivered_held` (still emits an envelope so audit and durable history are preserved)                                                                        |

Every delivery — and every future delivery inherited by a
protected surface — emits an `activity_event_envelope_record` for
every user-visible event. The envelope is the cross-surface
contract; per-surface rendering freedoms MUST collapse back into
the envelope's addressable axes on every boundary (support
export, mutation journal, AI evidence, claim manifest, crash
dump, evidence packet).

## Long-running work MUST live on a durable surface

The taxonomy names **where long-running work must live** and
**what cannot be toast-only**. The following classes of work have
a durable home; a surface that renders them only as a toast is
non-conforming (`denial_reason = toast_only_forbidden_for_durable_work`
or `durable_job_row_missing_for_long_running_work`):

- `build`, `test`, `debug_session`, `task_run`
- `indexer_pass`, `save_or_sync`
- `update_install`, `restore_or_recovery`
- `download_or_upload`
- `notebook_execution`, `ai_apply_execution`, `review_apply_execution`
- `provider_handoff`, `remote_attach`
- `collaboration_session`
- `admin_policy_refresh`

Each of the above is a `durable_job_subject_class` in the schema.
Taskbar, dock, and app-icon progress indicators reflect **these
classes only** — never transient background polls.

Acknowledgement, follow-up, digest, snooze, and mute items are
additionally required to live on a durable surface whenever they
carry logs, evidence, or recovery actions. A toast with no
durable mirror for work that has recoverable evidence is
non-conforming.

## Core axes (frozen)

Every `activity_event_envelope_record` names exactly one value
from each axis below. Adding a value is additive-minor and bumps
`activity_event_envelope_schema_version`; repurposing a value is
breaking and requires a new decision row.

### Delivery-surface class

- `toast`
- `contextual_banner`
- `status_item`
- `durable_job_row`
- `attention_item`
- `activity_center_digest_card`
- `digest_group_row`
- `os_notification`
- `os_badge_app_icon`
- `lock_screen_summary`
- `companion_push`
- `not_delivered_held`

Rules (frozen):

1. A surface that cannot resolve its delivery-surface class MUST
   deny with `delivery_surface_unresolved` rather than defaulting
   to `toast`.
2. The same event across multiple surfaces emits one envelope per
   (canonical_event_id, delivery_surface_class, client_scope)
   triple. All envelopes for one event share the same
   `canonical_event_id` and, where applicable, the same
   `grouped_burst_id` (§9.3 of the UI / UX Spec).
3. `not_delivered_held` still emits an envelope so durable history
   is preserved. A held event that did not emit an envelope is
   non-conforming (`denial_reason = held_event_missing_audit_trail`).

### Attention class

Re-exported from UI / UX Spec §9.15 verbatim. Acknowledgement,
Follow-up, Digest, Muted / Snoozed, Durable running, Suppressed
policy, Held quiet-hours.

- `acknowledgement`
- `follow_up_needed`
- `digest_only`
- `muted_or_snoozed`
- `durable_running`
- `suppressed_policy`
- `held_quiet_hours`

Rules (frozen):

1. A surface that cannot resolve its attention class MUST deny
   with `attention_class_unresolved` rather than defaulting to
   `acknowledgement`.
2. `acknowledge` MAY clear a badge but MUST NOT silently change
   the underlying object. `resolve` requires the underlying
   object to change or an explicit mark-done; a `resolve` that
   silently mutates the source object is non-conforming
   (`denial_reason = resolve_silently_mutated_source_object`).
3. `muted_or_snoozed` envelopes MUST name the resume condition
   on the `attention_item_record.snooze_resume_condition_label`.
   A snooze without a resume condition is non-conforming
   (`denial_reason = snooze_without_resume_condition`).

### Badge class

Badges are per-class counts. Combining classes into one overloaded
count is non-conforming (`denial_reason = badge_count_inflation_mixed_classes`).

- `needs_review`
- `failed_runs`
- `mentions`
- `security_notices`
- `session_requests`
- `offline_publish_pending`
- `durable_running_count`
- `held_or_suppressed_count`
- `completion_unread`

Rules (frozen):

1. `0` means no current items for the class; the badge rendering
   MUST clear. A badge that persists at zero is non-conforming
   (`denial_reason = badge_persisted_after_state_cleared`).
2. A badge whose class cannot be named MUST deny with
   `badge_class_unlabelled` rather than render as a generic total.
3. `held_or_suppressed_count` is rendered alongside `count` in
   compact-shell overflow so suppression is never silent; a held
   badge total that hides its held count is non-conforming.

### Interruptibility tier

Six tiers, closed set. Escalation requires a typed trigger
(severity change, duration threshold crossed, required authority
change). Decorative escalation is non-conforming
(`denial_reason = interruption_tier_escalated_without_trigger`).

- `tier_ambient` — passive chrome only (status item, badge).
- `tier_transient` — toast allowed.
- `tier_durable` — MUST hit a durable surface (durable_job_row or
  attention_item).
- `tier_actionable` — requires a named next action; cannot be
  toast-only.
- `tier_blocking_trust` — routes through a review sheet or modal
  when the active workflow cannot continue safely.
- `tier_critical_safety` — cannot be suppressed by quiet-hours,
  focus mode, presentation mode, screen share, privacy mode,
  power-saver, or reduced-attention policy. Admin suppression
  MAY narrow but MAY NOT silently block
  (`denial_reason = admin_suppression_blocked_critical_safety`).

### Quiet-hours mode

Ten modes, closed set. The active set of modes at mint time is
recorded on every envelope (`quiet_hours_mode_at_mint`). A user
who is not under any hold has `mode_none`.

- `mode_none`
- `mode_quiet_hours_user`
- `mode_do_not_disturb_user`
- `mode_focus_mode_user`
- `mode_presentation`
- `mode_screen_share`
- `mode_privacy_mode`
- `mode_reduced_attention_policy`
- `mode_power_saver_runtime`
- `mode_admin_suppression`

Rules (frozen):

1. Every mode preserves durable history. A mode that silently
   discards durable activity-center rows, attention items,
   history-lane entries, or audit events is non-conforming
   (`denial_reason = quiet_hours_hold_discarded_durable_history`).
2. Every mode preserves `tier_critical_safety`. Trust-, policy-,
   or recovery-critical items always render regardless of hold
   (`denial_reason = quiet_hours_hold_discarded_critical_safety_tier`).
3. Admin overrides MAY narrow; they MAY NOT silently widen
   beyond the frozen exclusion rules (ADR-0008).

### Suppression reason

Closed set. A held envelope MUST name at least one reason.

- `quiet_hours_user_policy`
- `do_not_disturb_user_policy`
- `focus_mode_user_policy`
- `presentation_mode_active`
- `screen_share_active`
- `privacy_mode_active`
- `admin_policy_suppression`
- `reduced_attention_posture`
- `power_saver_background_pause`
- `dedupe_same_canonical_event`
- `dedupe_same_grouped_burst`
- `class_muted_by_user`
- `class_snoozed_by_user`
- `release_pending_next_unsuppressed_surface`

Rules (frozen):

1. Dedupe reasons preserve lineage. Two bursts with the same
   dedupe scheme must share a single `grouped_burst_id`; a split
   is non-conforming (`denial_reason = grouped_burst_split_without_new_lineage`).
2. `release_pending_next_unsuppressed_surface` envelopes release
   as one grouped digest when the mode exits, grouped by source
   and severity (§9.14 of the UI / UX Spec).

### Privacy payload class

Closed set. Every envelope names one class.

- `lock_screen_safe_generic`
- `lock_screen_safe_scoped`
- `in_product_only`
- `redacted_metadata_only`
- `policy_forbidden_on_lock_screen`

Rules (frozen):

1. Deliveries on `lock_screen_summary` MUST NOT carry
   `in_product_only` payloads
   (`denial_reason = privacy_payload_class_missing_on_lock_screen`).
2. OS notifications MUST NOT embed raw bodies, raw paths, raw
   URLs, raw secret material, raw customer-owned identifiers, or
   raw AI prompt / completion text
   (`denial_reason = raw_body_forbidden_on_os_notification` /
   `raw_prompt_text_forbidden_on_os_notification` /
   `raw_secret_forbidden_on_any_surface`).
3. Lock-screen summaries degrade to privacy-safe summaries unless
   the user or admin explicitly chose more disclosure (§6.7 /
   §9.14 of the UI / UX Spec).

### Dedupe-key scheme

Closed set. Repeated deliveries collapse under one scheme.

- `canonical_event_id`
- `canonical_object_target_plus_event_class`
- `grouped_burst_id`
- `subsystem_plus_object_plus_phase`
- `cross_client_canonical_event_id`

Rules (frozen):

1. Ten retries against one pipeline become one evolving item,
   not ten independent notifications. A surface that stacks
   toasts is non-conforming
   (`denial_reason = dedupe_violated_stacked_toasts`).
2. `cross_client_canonical_event_id` collapses the same event
   across desktop, companion, remote agent, and managed admin
   surfaces into one canonical item; duplicate deliveries across
   clients split their `cross_client_lineage_refs` into sibling
   envelopes rather than renewing per-client events
   (`denial_reason = companion_cross_client_divergence`).

### Reopen-target kind

Closed set. Invoking a notification lands on the narrowest
truthful destination.

- `canonical_object_target_exact`
- `review_sheet`
- `diff_view`
- `evidence_packet_row`
- `activity_center_item`
- `history_lane_row`
- `attention_item_exact`
- `durable_job_row_exact`
- `placeholder_announced`
- `reopen_denied_requires_revalidation`

Rules (frozen):

1. Reopening to a generic home screen is forbidden
   (`denial_reason = reopen_to_generic_home_forbidden`).
2. When the originating target requires fresh user intent (wake-
   from-sleep, display reconnect, policy-epoch change, provider
   grant narrowed), reopen denies with
   `reopen_denied_requires_revalidation` rather than silently
   replaying a mutating action (§6.7 of the UI / UX Spec).
3. When the originating target is missing or moved, the reopen
   falls back to `placeholder_announced` with an announcement
   explaining why reopen could not return to the exact target.
   Silent fallback to a home screen is non-conforming.

### Dismissal verb

Closed set. `Acknowledge`, `Resolve`, `Dismiss`, `Snooze`,
`Mute`, and `Suppress` are different states and MUST never be
used as interchangeable labels
(`denial_reason = dismissal_verb_used_as_alias`).

- `acknowledge` — removes the badge, not the underlying state.
- `resolve` — underlying object changes (or explicit mark-done);
  MUST NOT silently mutate the source object.
- `dismiss` — removes the transient delivery; the durable row
  stays intact in the activity center, inbox, inspected object,
  or history lane.
- `snooze` — defers with a named resume condition.
- `mute` — silences future deliveries of the class.
- `suppress` — system-side policy hold (admin / quiet-hours /
  presentation); not a user verb.

Rules (frozen):

1. `Mark done` in Aureline MUST NOT silently close an incident,
   dismiss a provider alert, or publish a review unless that
   mutation is separately confirmed (§9.15).
2. Dismissing a toast MUST NOT erase the underlying state from
   the activity center, inbox, inspected object, or history lane.

### Durable-job lifecycle state

Closed set. Re-exported from UI / UX Spec §10.5.

- `queued`
- `running`
- `needs_approval`
- `attention_required`
- `completed`
- `partially_completed`
- `failed`
- `quiet_hours_held`
- `policy_suppressed`
- `cancelled`
- `restored_evidence_only`

`restored_evidence_only` matches the interaction-safety contract's
`evidence_only_no_rerun` revert class (restored terminals, debug
sessions, notebooks, API requests, remote shells); a surface that
silently re-runs evidence-only work is non-conforming.

### Source subsystem

Every envelope names the owning subsystem; the set is closed. Adding
a subsystem is additive-minor and bumps the schema version. See the
schema (`source_subsystem`) for the complete list.

## Envelope shape (summary)

The schema at
[`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
is authoritative. A minimal `activity_event_envelope_record`
names:

- `envelope_id`
- `canonical_event_id` — one id backs every repeated delivery
  across surfaces and across clients.
- `source_subsystem`
- `actor_identity_ref`
- `canonical_object_target_ref`
- `attention_class`
- `interruptibility_tier`
- `delivery_surface_class`
- `privacy_payload_class`
- `quiet_hours_mode_at_mint` (one or more modes; contains
  `mode_none` when the user is unheld)
- `suppression_reasons`
- `dedupe_key_scheme`
- `grouped_burst_id_ref` (nullable)
- `reopen_target_ref` (nullable only when
  `interruptibility_tier = tier_ambient` and no reopen is
  meaningful)
- `summary_label` (short, privacy-safe)
- `next_action_label` (required when
  `interruptibility_tier ∈ { tier_actionable, tier_blocking_trust,
  tier_critical_safety }`)
- `dismissal_verbs_available`
- `interaction_safety_packet_id_ref` (nullable; quotes the
  interaction-safety packet when the event reports a
  consequence-bearing interaction — never re-mints its axes)
- `cross_client_lineage_refs`
- `client_scopes`, `freshness_class`, `redaction_class`,
  `policy_context`, `minted_at`

### Related records

Envelope-adjacent records freeze the addressable shape of each
durable surface:

- `durable_job_record` — one row per long-running unit of work,
  lifecycle state, phase label, progress, cancellability,
  evidence ref.
- `attention_item_record` — one row per triage item, with a
  required `why_it_matters_label` and `primary_next_action_label`
  when `attention_class = follow_up_needed`, and a required
  `snooze_resume_condition_label` when `attention_class =
  muted_or_snoozed`.
- `badge_record` — one row per badge class per source subsystem;
  count plus held-or-suppressed count.
- `grouped_burst_record` — lineage for collapsed bursts,
  including whether the digest was released after a hold.
- `reopen_target_record` — the narrowest truthful destination
  for an invoked notification.
- `quiet_hours_policy_row_record` — one row per
  `quiet_hours_mode` declaring suppressed / preserved delivery
  surfaces, suppressed tiers (never `tier_critical_safety`),
  durable-history preservation (always true), and exit-digest
  behaviour. The seeded rows are in
  [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml).

## Lock-screen, presentation, privacy, and admin-suppression posture

The taxonomy names the initial posture for each protected OS-level
path:

### Lock-screen-safe summaries

- Default `privacy_payload_class` for `lock_screen_summary` is
  `lock_screen_safe_generic`. A workspace- or session-scoped
  preview may upgrade to `lock_screen_safe_scoped`.
- `in_product_only` payloads MUST NOT deliver on
  `lock_screen_summary`.
- `policy_forbidden_on_lock_screen` denies OS-surface delivery
  entirely and preserves only the in-product durable row.
- Secret-bearing, AI prompt / completion, customer-owned
  identifiers, and raw review titles are `redacted_metadata_only`
  by default and never appear on lock-screen surfaces.

### Presentation / privacy mode

- `mode_presentation` and `mode_privacy_mode` suppress
  `toast`, `os_notification`, `companion_push`, and
  `lock_screen_summary` deliveries for
  tiers `tier_transient` through `tier_actionable`.
- They preserve `status_item`, `durable_job_row`,
  `attention_item`, `activity_center_digest_card`,
  `digest_group_row`.
- `tier_blocking_trust` and `tier_critical_safety` continue to
  render regardless of mode, because the active workflow cannot
  continue safely without the interruption.

### Admin suppression

- `mode_admin_suppression` is NOT user-overridable.
- Admin suppression MAY narrow delivery further; it MAY NOT
  silently block `tier_critical_safety` deliveries
  (`denial_reason = admin_suppression_blocked_critical_safety`).
- Admin suppression preserves audit trail; blocked high-
  importance events remain visible in durable chronology where
  policy requires it.

### Focus-steal prevention

- A delivery on a protected path (save, recovery, trust review,
  debugging, AI apply) that would interrupt the active workflow
  is refused with `focus_steal_on_protected_path`.
- `tier_critical_safety` deliveries are the only exception; they
  escalate through the interaction-safety contract's review-sheet
  or modal path rather than inventing a new focus-stealing
  surface.

## Routing rules for transient vs durable attention

| Source of event                                | Default delivery surface                        | Escalate to                                                       | Suppress when                                                                 | Required carry-over                                                                        |
|------------------------------------------------|-------------------------------------------------|-------------------------------------------------------------------|-------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------|
| Direct user success with no durable consequence| `toast` (tier_transient)                        | `activity_center_item` only when logs / evidence exist            | Never blocked; visually reduced in `mode_presentation`                        | Action label, `canonical_object_target_ref`, `Undo` / open-details path when reversible     |
| Background completion with durable output      | `durable_job_row` + optional `toast`            | `attention_item` when review required or mixed outcome            | Toast MAY be suppressed under user focus / quiet modes                        | `canonical_event_id`, `canonical_object_target_ref`, output location, next action          |
| Degraded capability or reconnecting state      | `status_item` + `contextual_banner`             | `attention_item` when prolonged, repeated, or partially blocking  | Toast-only treatment is prohibited                                            | Last-known-good state, last failure reason, retry / open-details action                    |
| Trust / policy / auth / entitlement change     | `contextual_banner` or review sheet             | `attention_item` and modal only when active workflow cannot continue | Companion / OS notifications obey redaction policy                        | Affected scope, authority source, expiry / reauth path, what still works                   |
| Collaboration / human-request event            | `attention_item` or inline collaboration surface| `os_notification` only for direct mentions, reviews, invited joins| `mode_quiet_hours_user`, `mode_presentation`, `mode_privacy_mode` route to digest | Actor, workspace / session id ref, reply / open path                                       |
| Repeated low-severity burst                    | `digest_group_row` (grouped)                    | None unless severity increases                                    | Toasts and sounds collapse into one grouped event                             | `grouped_burst_id`, member_count, latest event time, open-group action                     |
| Long-running job progress                      | `durable_job_row` + `os_badge_app_icon`         | `attention_item` on failure / mixed outcome                       | Toast MAY be suppressed under user focus / quiet modes                        | `durable_job_id`, `lifecycle_state`, phase label, cancel / open-details action             |

## Repeated-event dedupe

Repeated deliveries of the same event collapse under one lineage
(`dedupe_key_scheme`). The following collapse rules are frozen:

1. **Exact event repeat** → `canonical_event_id` scheme. One
   evolving envelope; new deliveries update `summary_label`,
   progress, and phase rather than minting a new envelope.
2. **Same object, new event class** → `canonical_object_target_plus_event_class`
   scheme. Two distinct lineages, each with one envelope and
   one grouped-burst id if applicable.
3. **Retry burst on one pipeline** → `subsystem_plus_object_plus_phase`
   scheme. N retries become one `grouped_burst_record` with
   `member_count = N`.
4. **Low-severity burst** → `grouped_burst_id` scheme. Digest
   card card surfaced as one `digest_group_row`; dismissing the
   digest entry does not rewrite source events.
5. **Cross-client duplicate** → `cross_client_canonical_event_id`
   scheme. Desktop, companion, remote agent, and managed admin
   surfaces share one canonical envelope; per-client delivery
   history is preserved in `cross_client_lineage_refs`.

## Exact-reopen semantics

Invoking a delivery lands on a typed reopen target:

1. **`canonical_object_target_exact`** — the canonical object
   row or detail panel (e.g. the review item, the failing test,
   the branch row, the artifact row).
2. **`review_sheet` / `diff_view` / `evidence_packet_row`** — the
   interaction-safety surface that owns the consequence-bearing
   interaction.
3. **`activity_center_item` / `history_lane_row`** — the durable
   row for background work or cleared history.
4. **`attention_item_exact` / `durable_job_row_exact`** — the
   canonical attention item or durable job row; used when the
   delivery was a toast mirror of a durable surface.
5. **`placeholder_announced`** — target missing / moved / policy-
   blocked / extension unavailable / display topology lost; the
   surface renders a placeholder whose announcement explains
   why reopen could not return to the exact target.
6. **`reopen_denied_requires_revalidation`** — target requires
   fresh user intent (wake-from-sleep, display reconnect, policy-
   epoch change, provider-grant narrowing); reopen denies rather
   than silently replaying a mutating action.

A delivery that reopens to a generic home screen is non-conforming
(`denial_reason = reopen_to_generic_home_forbidden`).

## Grouped-burst lineage

`grouped_burst_record` preserves the **one evolving item rather
than N independent notifications** contract:

- `member_count` names the number of envelopes collapsed into
  this burst.
- `first_event_id_ref` and `latest_event_id_ref` bracket the
  lineage.
- `grouping_reason_label` is short and privacy-safe.
- `digest_released_after_hold` is true when the burst was held
  across a `quiet_hours_mode` and released on mode exit as one
  grouped digest grouped by source and severity.
- Splitting a lineage into two bursts requires a new
  `dedupe_key_scheme`; an unjustified split is non-conforming
  (`denial_reason = grouped_burst_split_without_new_lineage`).

## When quiet-hours holds still preserve durable history

Every `quiet_hours_mode` (except `mode_none`) may delay
interruption. **No mode discards durable history.** The
`quiet_hours_policy_row_record` for every mode asserts:

- `preserves_durable_history: true`
- `preserves_critical_safety_tier: true`

Held envelopes carry:

- `delivery_surface_class: not_delivered_held`
- at least one `suppression_reason`
- the same `canonical_event_id`, `canonical_object_target_ref`,
  `attention_class`, and (where applicable) `grouped_burst_id_ref`
  they would have carried if delivered

When the mode exits, held envelopes release. Bursty holds release
as one grouped digest grouped by source and severity so the user
can triage intentionally rather than replay a toast backlog one
item at a time (§9.14). Blocked high-importance events under
`mode_admin_suppression` retain a durable audit trail where
policy requires it (§6.7).

## Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `activity_event_envelope_record`, `durable_job_record`,
   `attention_item_record`, `badge_record`,
   `grouped_burst_record`, `reopen_target_record`,
   `quiet_hours_policy_row_record`, and
   `activity_event_audit_event_record` cross the RPC boundary as
   typed payloads (ADR-0004). Raw bodies, raw paths, raw URLs,
   raw prompt text, and raw credential material never cross.
2. OS notification payloads, companion push payloads, and lock-
   screen summaries go through the broker-owned redaction pass
   (ADR-0007) before bytes reach the sink. The
   `activity_event_envelope_record.summary_label` is already
   redacted to the declared `privacy_payload_class`.
3. Mutation-journal entries, support bundles, and evidence
   packets name `envelope_id`, `durable_job_id`,
   `attention_item_id`, `badge_id`, `grouped_burst_id`,
   `reopen_target_id`, and `policy_row_id` only.
4. Crash dumps and core files MUST NOT inherit unresolved
   envelopes; a crash that lands mid-delivery discards the
   envelope rather than persisting a partial axis set.

Redaction defaults (frozen):

| Sink                                 | Default inclusion                                                                                                                                                                  |
|--------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | Envelope / job / attention / badge / burst / reopen / policy ids, source subsystem, attention class, interruptibility tier, delivery surface, quiet-hours modes, suppression reasons, audit-event ids. No raw bodies, paths, URLs, or prompt text. |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw bodies or paths.                                                                                                              |
| `support_bundle`                     | Full per-axis values, full grouped-burst lineage, full durable-job lifecycle history, full reopen-target enumeration. Raw bodies excluded.                                         |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, consequence class (from the referenced interaction-safety packet), durable-job lifecycle state. Raw bodies never included.   |
| `ai_context_capture`                 | Envelope / job / attention ids, attention class, interruptibility tier. Raw bodies and prompt text never captured.                                                                   |
| `os_notification_payload`            | `envelope_id`, `canonical_event_id`, `summary_label` (already privacy-safe), `next_action_label` (when present), `reopen_target_ref`. No raw bodies, paths, URLs, or prompt text.   |
| `lock_screen_summary_payload`        | Category-only for `lock_screen_safe_generic`; bounded category plus workspace / session label (no object identity) for `lock_screen_safe_scoped`; denied for `policy_forbidden_on_lock_screen`. |
| `companion_push_payload`             | Same as `os_notification_payload`; cross-client collapse via `cross_client_canonical_event_id`.                                                                                     |
| `recipe_manifest`                    | `envelope_id`, `durable_job_id`, `running_build_identity_ref`. Raw bodies forbidden.                                                                                                 |
| `mutation_journal_entry`             | Ids, attention class, interruptibility tier, lifecycle state, delivery surface. No raw bodies or URLs.                                                                               |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                                    |
| `claim_manifest`                     | Full per-axis values, full grouped-burst lineage. Raw bodies never included.                                                                                                         |
| `terminal_transcript`                | `envelope_id` and `source_subsystem` only; raw URLs require boundary-labelled confirmation before capture.                                                                           |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

## Schema-of-record posture

The eventual shell / attention-routing crate's Rust types are the
source of truth. The JSON Schema export at
`schemas/ux/activity_event_envelope.schema.json` is the cross-tool
boundary every non-owning surface reads. Adding a new delivery-
surface class, attention class, badge class, interruptibility
tier, quiet-hours mode, suppression reason, privacy payload
class, dedupe-key scheme, reopen-target kind, durable-job
lifecycle state, dismissal verb, audit-event id, or denial reason
is additive-minor and bumps
`activity_event_envelope_schema_version`; repurposing any
existing value is breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- Full activity-center UI implementation.
- Platform notification adapters (macOS User Notifications, Linux
  XDG `org.freedesktop.Notifications`, Windows Toast, companion
  push providers). The envelope reserves the shape; adapters wire
  it later.
- The eventual shell / attention-routing crate's Rust types. The
  JSON Schema export reserves the boundary shape until the crate
  lands.
- Per-surface rendering logic (toast spacing, badge animation,
  digest card visual design). The design-token vocabulary
  (`docs/design/design_token_component_state_vocabulary.md`)
  owns rendering.

These lines move only by opening a new decision row, not by
editing this taxonomy.

## Reuse guarantee

This taxonomy is reusable by every owning subsystem without
redefining core delivery semantics. A subsystem that mints an
event MUST:

1. Quote the delivery / attention / interruptibility / quiet-hours /
   suppression / privacy / dedupe / reopen / dismissal /
   lifecycle vocabularies above verbatim.
2. Emit one `activity_event_envelope_record` per
   (canonical_event_id, delivery_surface_class, client_scope)
   triple; share `canonical_event_id` and, where applicable,
   `grouped_burst_id` across all related envelopes.
3. Emit a `durable_job_record` for every long-running unit of
   work; never render long-running work as toast-only.
4. Emit an `attention_item_record` for every Needs-follow-up
   item; never conflate `acknowledge`, `resolve`, `dismiss`,
   `snooze`, `mute`, or `suppress`.
5. Emit a `badge_record` per class; never inflate a badge with
   mixed classes.
6. Emit a `grouped_burst_record` when collapsing repeated events;
   never split lineage without a new `dedupe_key_scheme`.
7. Emit a `reopen_target_record` on every delivery whose
   `interruptibility_tier` is not `tier_ambient`; never reopen
   to a generic home screen.
8. Honour the quiet-hours / focus / presentation / screen-share /
   privacy / admin-suppression / reduced-attention / power-saver
   matrix; preserve durable history and the critical-safety tier
   in every mode.
