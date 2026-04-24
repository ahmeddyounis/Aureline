# Notification-delivery contract: canonical event id, routing, redaction, linkback, and action taxonomy

This document is the **cross-surface notification-delivery contract**
for Aureline. It exists so one canonical event id, one routing
matrix, one redaction posture, one durable-linkback contract, and
one action taxonomy connect every surface that can deliver a
user-visible event — toast, contextual banner, status item,
activity center (durable job row, attention item, digest card),
OS notification, OS badge, lock-screen summary, companion push,
grouped digest, and exported history — without minting parallel
event models. A surface that mints a private event id, a private
routing rule for one of the frozen event classes, a private
redaction fallback, a private linkback verb, or a private
dismissal alias is non-conforming.

The contract is normative. Where this document disagrees with the
UI / UX Spec sections it quotes, the source spec wins and this
document MUST be updated in the same change. Where this document
disagrees with a downstream surface's private delivery, dedupe,
linkback, or redaction story, this document wins and the surface
is non-conforming.

The companion artifacts are:

- [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  — boundary schema every non-owning surface reads. Freezes the
  `event_lineage_record` (one record per `canonical_event_id`,
  tracking originating / current delivery surface, escalation
  reason, suppression reasons, privacy payload class, dedupe
  scheme, grouped-burst lineage, delivery / dismissal / reopen /
  release steps, and durable linkback records) and the
  `notification_route_rule_record` (one row per frozen
  `event_class`).
- [`/fixtures/ux/notification_routes/`](../../fixtures/ux/notification_routes/)
  — worked fixtures covering grouped-burst escalation, privacy-
  safe lock-screen payloads, quiet-hours suppression audit,
  exact reopen after delay or dedupe, no-bypass rules for high-
  risk shortcuts, and dismissal that preserves durable history.

This contract rides alongside — it does **not** re-mint — the
vocabularies frozen in:

- [`/docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  — delivery-surface class, attention class, interruptibility
  tier, quiet-hours mode, suppression reason, privacy payload
  class, dedupe-key scheme, reopen-target kind, dismissal verb,
  durable-job lifecycle state, badge class, and source subsystem.
  Every axis above is re-exported from the taxonomy schema; this
  contract adds **routing, dedupe collapse, carry-over, linkback,
  and action-taxonomy rules** without re-minting the axes.
- [`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
  — the per-delivery envelope. One
  `activity_event_envelope_record` emits for every
  (`canonical_event_id`, `delivery_surface_class`, `client_scope`)
  triple. This contract's `event_lineage_record` keeps one trail
  across those envelopes.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  — per-`quiet_hours_mode` suppression / preservation rules. This
  contract does not re-declare which modes suppress which
  surfaces.
- [`/artifacts/ux/interruptibility_escalation_seed.yaml`](../../artifacts/ux/interruptibility_escalation_seed.yaml)
  — per-tier escalation triggers, default delivery surfaces, and
  protected-path rules. Escalation reasons in this contract
  quote the `escalation_triggers` set seeded there verbatim.
- [`/docs/ux/shell_interaction_safety_contract.md`](./shell_interaction_safety_contract.md)
  — authority, consequence, revert, representation, focus-return,
  and responsive-fallback vocabularies. A delivery whose lineage
  reports a consequence-bearing interaction quotes its
  `interaction_safety_packet_record` by ref.
- `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
  — redaction pass runs before bytes reach any persistent or
  exportable sink, including OS notification payloads, lock-
  screen summaries, and companion push payloads.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
  — admin policy MAY narrow; policy MAY NOT silently widen.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  — `freshness_class`, `client_scope`, `redaction_class` are
  re-exported without modification.

## Who reads this document

- **Shell / notification / activity-center / digest / companion
  authors** minting the first-party delivery surfaces. They honour
  one canonical event id, one routing matrix, one redaction
  posture, one linkback contract, and one action taxonomy across
  every surface.
- **Owning subsystems** (editor, terminal, review / diff, palette
  / search, install / update / attach, AI apply, collaboration,
  provider-bearing, docs / help / service-health, support /
  export, build system, test runner, debug session, task runner,
  indexer, VFS save, sync / mirror, notebook kernel, remote
  agent, extension host, workspace trust, policy resolver, admin
  policy, secret broker, runtime power manager) minting events
  that reach users.
- **Support, parity-audit, and cross-client routing tooling**
  reading the lineage and routing rows mechanically — every axis
  is separately addressable even when the surface folds several
  into one notification line.

## 1. Scope

- Freeze the **canonical event-lineage record** covering event
  id, event class, grouped-burst id, escalation reason,
  suppression reasons, privacy class, originating surface,
  current delivery surface, canonical object target, dismissal
  metadata, reopen metadata, release-from-hold metadata, and
  durable linkback records.
- Freeze the **routing matrix** for the seven launch event
  classes — direct user success with no durable consequence,
  background completion with durable output, degraded or
  reconnecting state, trust / policy / auth boundary change,
  collaboration or human-request event, repeated low-severity
  burst, and long-running job progress — plus the
  **high-risk-shortcut-no-bypass** event class for irreversible
  or high-blast-radius shortcuts. Each row names default surface,
  allowed escalation surfaces, forbidden surfaces, badge classes,
  dedupe key scheme, required carry-over fields, and the
  quiet-hours modes that MAY suppress the class.
- Freeze the **redaction and delivery rules** for OS
  notifications, companion push, lock-screen summaries, digest
  release, quiet hours, focus mode, presentation / privacy mode,
  grouped delivery, and duplicate-collapse across desktop /
  browser / mobile clients.
- Freeze the **durable linkback contract** so every toast, OS
  notification, digest card, and companion alert resolves to a
  canonical durable row, history item, or audit-trail entry;
  delayed or suppressed deliveries preserve an auditable trail.
- Freeze the **action taxonomy** for dismiss, snooze,
  acknowledge, mute, resolve, and suppress so notifications
  cannot invent surface-local side effects.

## 2. Out of scope

- Platform notification adapters (macOS User Notifications,
  Linux XDG `org.freedesktop.Notifications`, Windows Toast,
  companion push providers, email, SMS). Those bind to
  `os_notification_payload`, `companion_push_payload`, and
  `lock_screen_summary_payload` through the taxonomy's redaction
  defaults; adapters wire them later.
- Full activity-center UI implementation. The lineage record
  reserves the shape; rendering logic lives on the component
  contracts.
- Per-subsystem delivery authoring (copy choice, icon selection,
  banner placement). Owning subsystems emit lineages against the
  routing rows; product writers own the user-facing strings
  within the rules below.
- The eventual shell / attention-routing crate's Rust types. The
  JSON Schema export reserves the boundary shape until the crate
  lands.

## 3. One canonical event id, one lineage, many envelopes

Every user-visible event — transient or durable, held or released
— shares one `canonical_event_id`. The id backs every repeated
delivery across:

- toast → contextual banner → status item (in-product transient
  chrome),
- durable_job_row, attention_item, activity_center_digest_card,
  digest_group_row (in-product durable surfaces),
- os_notification, os_badge_app_icon, lock_screen_summary
  (OS-level surfaces),
- companion_push (browser / mobile companion surfaces),
- `not_delivered_held` (held-for-suppression envelopes still
  preserve audit),
- and exported history (support bundle, evidence packet, claim
  manifest, mutation journal).

The `event_lineage_record` is the **single trail** that unifies
those envelopes. One record per `canonical_event_id` records:

- the event class (`event_class`), the source subsystem, the
  actor, and the canonical object target;
- the originating delivery surface (where the lineage first
  surfaced) and the current delivery surface;
- the attention class, interruptibility tier, privacy payload
  class, dedupe key scheme, and grouped-burst id;
- the most recent escalation reason plus an ordered
  `delivery_steps` array naming every stage (minted, dedupe
  collapsed, delivered, escalated, held for suppression,
  released from hold, grouped into digest, dropped by policy);
- applied dismissal verbs, applied reopen invocations, and
  release-from-hold steps;
- the durable linkback records the transient deliveries resolve
  back to;
- the policy context the lineage opened under;
- and timestamps for open, last update, and close.

Rules (frozen):

1. **One canonical object target per canonical event id.** Two
   lineages that share a `canonical_event_id` but disagree on
   `canonical_object_target_ref` are non-conforming
   (`denial_reason = duplicate_object_identity_on_shared_event_id`).
   The contract keeps one event backing every repeated delivery;
   splitting an object identity under the shared id is a
   deliberate reopen, not a parallel event.
2. **Held events still open a lineage.** A delivery held under
   any `quiet_hours_mode` still mints an envelope with
   `delivery_surface_class = not_delivered_held` and MUST
   open a lineage; the lineage carries the intended
   `originating_surface_class` and a non-empty
   `suppression_reasons` array so durable chronology is
   preserved (`denial_reason = held_event_missing_linkback`).
3. **Cross-client duplicates collapse.** Desktop, companion,
   remote-agent, and managed-admin deliveries of the same event
   share one `canonical_event_id` and cross-link through
   `cross_client_lineage_refs`; per-client duplicate deliveries
   that renew events are non-conforming
   (`denial_reason = cross_client_divergence`).
4. **The same id links transient and durable surfaces.** Toast
   → durable_job_row → evidence-packet export all carry the
   same `canonical_event_id` and resolve to one
   `canonical_object_target_ref`. Export previews that surface a
   lineage name an `export_preview_row` linkback without
   duplicating the underlying object identity.

## 4. Frozen event classes and routing matrix

Every event that reaches a user surface names exactly one
`event_class`. The table below names the default surface, the
allowed escalation surfaces, the forbidden surfaces, the badge
classes the event class MAY increment, the dedupe scheme, the
required carry-over fields, and the quiet-hours modes that MAY
suppress the event class. Rows are frozen; adding an event class
is additive-minor and bumps `event_lineage_schema_version`.

| Event class                                         | Default surface                              | Escalates to                                                       | Forbidden surfaces                                   | Badge classes                                | Dedupe scheme                                    | Required carry-over                                                                                                      | Suppressible under                                                                 |
|-----------------------------------------------------|----------------------------------------------|--------------------------------------------------------------------|------------------------------------------------------|----------------------------------------------|--------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|
| `direct_user_success_no_durable_consequence`        | `toast`                                      | `activity_center_digest_card` when logs or evidence exist          | `os_badge_app_icon` (toasts do not inflate badges)   | `completion_unread`                          | `canonical_event_id`                             | `canonical_event_id`, `canonical_object_target_ref`, `undo_or_open_details_path` when reversible, `linkback_records`     | Visually reduced in `mode_presentation`; never blocked                             |
| `background_completion_with_durable_output`         | `durable_job_row` + optional mirror `toast`  | `attention_item` when review required, `os_notification` when away | `lock_screen_summary` (durable state lives in product) | `completion_unread`, `durable_running_count` | `canonical_event_id`                             | `canonical_event_id`, `canonical_object_target_ref`, `durable_job_id_ref`, `lifecycle_state`, `next_action_label`, `linkback_records` | `mode_quiet_hours_user`, `mode_do_not_disturb_user`, `mode_focus_mode_user`, `mode_presentation`, `mode_screen_share`, `mode_power_saver_runtime`, `mode_reduced_attention_policy`, `mode_admin_suppression` |
| `degraded_or_reconnecting_state`                    | `status_item` + `contextual_banner`          | `attention_item` when prolonged, repeated, or partially blocking   | `toast` (toast-only treatment is prohibited)         | `failed_runs`, `security_notices`            | `subsystem_plus_object_plus_phase`               | `canonical_event_id`, `last_known_good_state_label`, `last_failure_reason_label`, `linkback_records`, `cancel_or_open_details_action_label` | `mode_presentation`, `mode_screen_share`, `mode_reduced_attention_policy`, `mode_power_saver_runtime` |
| `trust_policy_auth_boundary_change`                 | `contextual_banner` or review sheet          | `attention_item`, modal on `tier_blocking_trust`, `os_notification` (`lock_screen_safe_generic` only) | `toast` (blocking state is never toast-only), `in_product_only` on lock-screen | `security_notices`, `mentions`               | `canonical_object_target_plus_event_class`       | `canonical_event_id`, `affected_scope_label`, `authority_source_label`, `reauth_or_expiry_label`, `what_still_works_label`, `linkback_records`, `interaction_safety_packet_id_ref` | `mode_admin_suppression` MAY narrow; never blocks `tier_critical_safety`           |
| `collaboration_or_human_request`                    | `attention_item` or inline collaboration     | `os_notification` for direct mentions, reviews, invited joins; `companion_push` | `lock_screen_summary` beyond `lock_screen_safe_scoped` | `mentions`, `session_requests`, `needs_review` | `cross_client_canonical_event_id`                | `canonical_event_id`, `actor_identity_ref`, `workspace_or_session_id_ref`, `reply_or_open_path_label`, `linkback_records` | `mode_quiet_hours_user`, `mode_do_not_disturb_user`, `mode_focus_mode_user`, `mode_presentation`, `mode_privacy_mode`, `mode_reduced_attention_policy`, `mode_admin_suppression` |
| `repeated_low_severity_burst`                       | `digest_group_row`                           | None unless severity increases (escalate to `attention_item` with `severity_increased` reason) | `toast` per member (stacked toasts forbidden)        | `held_or_suppressed_count`, `completion_unread` | `grouped_burst_id`                               | `canonical_event_id`, `grouped_burst_id_ref`, `member_count`, `latest_event_time`, `open_group_action_label`, `linkback_records` | Any mode that would have suppressed the members; digest releases on mode exit     |
| `long_running_job_progress`                         | `durable_job_row` + `os_badge_app_icon`      | `attention_item` on failure or mixed outcome                       | `lock_screen_summary` (durable state lives in product) | `durable_running_count`, `failed_runs`       | `canonical_event_id`                             | `canonical_event_id`, `durable_job_id_ref`, `lifecycle_state`, `phase_label`, `cancel_or_open_details_action_label`, `linkback_records` | `mode_quiet_hours_user`, `mode_do_not_disturb_user`, `mode_focus_mode_user`, `mode_power_saver_runtime`, `mode_admin_suppression` |
| `high_risk_shortcut_no_bypass`                      | `durable_job_row` + review sheet             | `tier_blocking_trust` or `tier_critical_safety` banner / modal; `os_notification` (`lock_screen_safe_generic` only) | `toast` alone (toast-only is non-conforming), `os_badge_app_icon` alone | `security_notices`, `failed_runs`            | `canonical_event_id`                             | `canonical_event_id`, `canonical_object_target_ref`, `interaction_safety_packet_id_ref`, `authority_source_label`, `next_action_label`, `linkback_records` | None (critical-safety always renders; admin suppression MAY narrow OS surfaces but MAY NOT silently block) |

Rules (frozen):

1. **No surface may re-mint a row for a frozen event class.** A
   surface that invents a private default surface for one of the
   seven row classes is non-conforming.
2. **Escalation requires a typed reason.** A lineage that moves
   from one tier to a higher tier names a non-`none`
   `escalation_reason` (`severity_increased`,
   `duration_threshold_crossed`, `required_authority_changed`,
   `consequence_class_escalated`, `basis_snapshot_drifted`,
   `policy_epoch_changed`, `trust_state_changed`,
   `provider_grant_narrowed`,
   `recovery_class_downgraded_to_no_recovery`,
   `repeated_dedupe_threshold_crossed`). Decorative escalation is
   non-conforming (`denial_reason =
   escalation_reason_missing_on_tier_change`).
3. **Badge counts stay per-class.** Badge classes in the routing
   row are the only classes the event class MAY increment;
   combining classes into one overloaded count is non-conforming
   (see `badge_count_inflation_mixed_classes` on the
   activity_event_envelope audit stream).
4. **Dedupe collapses repeats.** Ten retries against one pipeline
   become one evolving lineage under `grouped_burst_id` or
   `subsystem_plus_object_plus_phase`, not ten independent
   notifications.

## 5. Redaction and delivery rules for protected surfaces

The redaction posture is re-exported from
`docs/ux/attention_activity_taxonomy.md` §Audit, redaction, and
boundary posture and from
`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`.
This contract names the **delivery rules** that MUST apply on top
of the redaction pass.

### 5.1 OS notification payloads

- Every OS notification payload carries `envelope_id`,
  `canonical_event_id`, the short privacy-safe `summary_label`,
  `next_action_label` when present, and a `reopen_target_ref`.
  Raw bodies, raw paths, raw URLs, raw secret material, raw
  customer-owned identifiers, and raw AI prompt / completion
  text are never embedded
  (`denial_reason = raw_body_forbidden_on_os_notification`,
  `raw_prompt_text_forbidden_on_os_notification`,
  `raw_secret_forbidden_on_any_surface`).
- The payload's `privacy_payload_class` MUST match the lineage's
  declared class; widening across clients is non-conforming
  (`denial_reason = privacy_payload_class_widened_across_clients`).

### 5.2 Companion push payloads

- Companion push payloads share the OS-notification payload
  shape. Cross-client duplicate deliveries collapse via
  `cross_client_canonical_event_id`. A companion surface that
  renews per-client events rather than collapsing is
  non-conforming (`denial_reason = cross_client_divergence`).

### 5.3 Lock-screen summaries

- Default `privacy_payload_class` is `lock_screen_safe_generic`
  — category-only ("New review item").
- A workspace- or session-scoped preview MAY upgrade to
  `lock_screen_safe_scoped` — bounded category plus workspace /
  session label, never object identity.
- `in_product_only` payloads MUST NOT deliver on
  `lock_screen_summary`
  (`denial_reason = privacy_payload_class_missing_on_lock_screen`).
- `policy_forbidden_on_lock_screen` denies OS-surface delivery
  entirely; the in-product durable row stays intact and the
  lineage records a `linkback_records` entry
  (`activity_center_item` or `audit_trail_only`) so chronology
  is preserved.
- Under `mode_privacy_mode`, lock-screen deliveries are denied
  with `policy_forbidden_on_lock_screen`; the lineage records
  `delivery_surface_class = not_delivered_held` and a
  `suppression_reasons` entry.

### 5.4 Digest release

- Held envelopes release on mode exit as one grouped digest
  grouped by source and severity — not replayed as a toast
  backlog. The release emits a `release_step_record` with
  `kind = mode_exit_grouped_digest`, a `grouped_burst_id_ref`,
  and a `member_count`.
- Releases outside mode exit (`user_explicit_show_held`,
  `escalation_to_critical_safety`, `cross_client_collapse`,
  `reopen_revalidation_completed`) MUST still be recorded on the
  lineage; silent release is non-conforming.

### 5.5 Quiet hours, focus, presentation, privacy, admin suppression

Per-mode rules are frozen in
`artifacts/ux/quiet_hours_policy_matrix.yaml`. This contract
preserves those rules unchanged; the routing table above names
which event classes MAY be suppressed under which modes.

### 5.6 Grouped delivery

- A grouped burst preserves lineage: `member_count`,
  `first_event_id_ref`, and `latest_event_id_ref` bracket the
  collapsed envelopes. Splitting a lineage into two bursts
  without a new `dedupe_key_scheme` is non-conforming
  (`denial_reason = grouped_burst_split_without_new_lineage`).

### 5.7 Duplicate-collapse across desktop / browser / mobile

- `cross_client_canonical_event_id` collapses the same event
  across desktop, companion, remote-agent, and managed-admin
  surfaces. `cross_client_lineage_refs` records sibling lineages;
  a lineage that renews per-client events is non-conforming.

## 6. Durable linkback rules

Every delivery — transient or held — resolves through one or more
`linkback_record` entries on the lineage. Linkback kinds are a
closed set:

- `durable_job_row_exact` — long-running work row.
- `attention_item_exact` — triage inbox item.
- `activity_center_item` — activity-center row with the same
  `canonical_event_id`.
- `history_lane_row` — cleared-history row preserved for audit
  review.
- `evidence_packet_row` — release-relevant evidence packet.
- `review_sheet` / `diff_view` — interaction-safety surface that
  owns the consequence-bearing interaction.
- `canonical_object_target_exact` — the canonical object (review
  item, build, branch, artifact, session, provider grant).
- `export_preview_row` — the row on an in-product export preview
  (support bundle, claim manifest, mutation journal).
- `audit_trail_only` — reserved for deliveries whose
  user-facing delivery was denied under policy; the linkback is
  preserved only as an audit entry.

Rules (frozen):

1. **Every transient delivery MUST carry a non-empty
   `linkback_records`.** A toast / OS notification / companion
   push / digest card / lock-screen summary without a durable
   linkback is non-conforming
   (`denial_reason = linkback_target_missing` or
   `linkback_silently_downgraded_to_home_screen`).
2. **Delayed or suppressed deliveries preserve the same
   linkback.** A held envelope records the linkback the delivery
   would have carried if unsuppressed; on release, the linkback
   is unchanged.
3. **`audit_trail_only` is an explicit class, not a fallback.**
   Deliveries blocked by `policy_forbidden_on_lock_screen` or by
   admin policy record an `audit_trail_only` linkback with
   `is_durable = false`; surfaces MUST NOT silently fall back to
   nothing.
4. **Reopen resolves a linkback.** Invoking a delivery lands on
   the narrowest truthful destination; the lineage records a
   `reopen_step_record` with a `reopen_target_kind` that aligns
   with the linkback kind. Reopen to a generic home screen is
   non-conforming (`denial_reason =
   linkback_silently_downgraded_to_home_screen`).

## 7. Reopen semantics and revalidation

Invoking a notification records a `reopen_step_record`:

1. **`canonical_object_target_exact`** — lands on the canonical
   object row or detail panel.
2. **`review_sheet` / `diff_view` / `evidence_packet_row`** —
   lands on the interaction-safety surface that owns the
   consequence-bearing interaction.
3. **`activity_center_item` / `history_lane_row` /
   `attention_item_exact` / `durable_job_row_exact`** — lands on
   the durable row.
4. **`placeholder_announced`** — target missing / moved / policy-
   blocked / extension unavailable / display topology lost; the
   step carries a `placeholder_announcement_label` explaining
   why.
5. **`reopen_denied_requires_revalidation`** — target requires
   fresh user intent (wake-from-sleep, display reconnect,
   policy-epoch change, provider-grant narrowing); the step
   carries a `revalidation_required_reason_label`. After the
   user satisfies the revalidation trigger, a
   `release_step_record` with
   `kind = reopen_revalidation_completed` records the transition.

A delivery that reopens to a generic home screen is
non-conforming.

## 8. Action taxonomy

`acknowledge`, `resolve`, `dismiss`, `snooze`, `mute`, and
`suppress` are different states and MUST NEVER be used as
interchangeable labels
(`denial_reason = dismissal_verb_used_as_alias`). Every applied
verb records a `dismissal_step_record` with
`mutates_source_object` and `preserves_durable_history`:

| Verb          | Removes badge | Mutates source object                             | Preserves durable history | Required fields                                        |
|---------------|---------------|---------------------------------------------------|---------------------------|--------------------------------------------------------|
| `acknowledge` | yes           | no                                                | yes                       | —                                                      |
| `resolve`     | yes           | yes (underlying change or explicit mark-done)     | yes                       | — (`mutates_source_object = true` when applied)        |
| `dismiss`     | yes           | no (closes transient delivery; durable row stays) | yes                       | —                                                      |
| `snooze`      | yes           | no                                                | yes                       | `snooze_resume_condition_label`                        |
| `mute`        | yes           | no                                                | yes                       | `muted_class_ref`                                      |
| `suppress`    | yes           | no (system-side hold under admin / quiet-hours)   | yes                       | At least one `suppression_reason` on the lineage       |

Rules (frozen):

1. **Closing a toast does not mutate the source object.**
   `dismiss` with `mutates_source_object = true` is non-conforming
   (`denial_reason = dismissal_silently_mutated_source_object`).
2. **Closing a toast does not erase the durable row.**
   `preserves_durable_history = false` is non-conforming on every
   dismissal verb
   (`denial_reason = dismissal_erased_durable_history`).
3. **`snooze` requires a resume condition.** A snooze without
   `snooze_resume_condition_label` is non-conforming
   (`denial_reason = snooze_without_resume_condition`).
4. **`mute` names the class.** A mute that silences more than
   one badge class is non-conforming; mute one class at a time.
5. **`resolve` requires a real change.** `resolve` with
   `mutates_source_object = false` is non-conforming; a surface
   that treats `acknowledge` as `resolve` is non-conforming
   (`denial_reason = resolve_silently_mutated_source_object` on
   the activity_event_envelope audit stream).
6. **`suppress` is a system verb.** End users do not invoke
   `suppress` directly; it records admin-policy, quiet-hours, or
   presentation-mode holds. `mode_admin_suppression` is not
   user-overridable.

## 9. No bypass on high-risk shortcuts

Shortcuts that invoke irreversible or high-blast-radius actions
(irreversible_high_blast consequence class, destructive bulk
mutation, kill-switch trip, secret-compromise recovery, protected-
path apply) route as `event_class = high_risk_shortcut_no_bypass`:

- MUST NOT deliver as a toast alone
  (`denial_reason = high_risk_shortcut_toast_only_forbidden`).
- MUST route through the interaction-safety review sheet or
  modal path; bypassing the review sheet is non-conforming
  (`denial_reason = high_risk_shortcut_bypassed_review_sheet`).
- MUST land on `tier_durable`, `tier_actionable`,
  `tier_blocking_trust`, or `tier_critical_safety` — never
  `tier_ambient` or `tier_transient`.
- MAY emit an OS notification or companion push only with
  `privacy_payload_class = lock_screen_safe_generic` so lock-
  screen surfaces do not leak the destructive intent.
- MUST carry `interaction_safety_packet_id_ref` so authority,
  consequence, preview / apply / revert, and focus-return
  posture are inherited from the interaction-safety contract
  without re-minting.

## 10. Audit, redaction, and boundary posture

Process-boundary constraints (frozen):

1. `event_lineage_record` and `notification_route_rule_record`
   cross the RPC boundary as typed payloads. Raw bodies, raw
   paths, raw URLs, raw prompt text, and raw credential material
   never cross.
2. OS notification payloads, companion push payloads, and lock-
   screen summaries go through the broker-owned redaction pass
   (ADR-0007) before bytes reach the sink.
3. Mutation-journal entries, support bundles, and evidence
   packets name `lineage_id`, `canonical_event_id`, and
   `linkback_records[*].target_identity_ref` only.
4. Crash dumps and core files MUST NOT inherit unresolved
   lineages; a crash that lands mid-delivery discards the
   lineage rather than persisting a partial axis set.

Audit-stream events on the event-lineage stream:

| Audit-event id                       | Fires when                                                                                                               |
|--------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| `lineage_opened`                     | A new `canonical_event_id` mints its first envelope.                                                                      |
| `lineage_delivery_recorded`          | A delivery step reaches a user surface.                                                                                   |
| `lineage_dedupe_recorded`            | A repeat envelope collapses under one of the dedupe schemes.                                                              |
| `lineage_escalation_recorded`        | A delivery step crosses into a higher interruptibility tier with a typed reason.                                          |
| `lineage_hold_recorded`              | A delivery is held under one of the frozen quiet-hours modes.                                                             |
| `lineage_release_recorded`           | A previously held delivery is released (grouped digest, user-explicit show, escalation, cross-client collapse).           |
| `lineage_digest_released`            | A grouped burst is released as one digest on mode exit.                                                                   |
| `lineage_dismissal_recorded`         | A dismissal verb is applied.                                                                                              |
| `lineage_reopen_recorded`            | A reopen is invoked and resolves to a concrete target.                                                                    |
| `lineage_reopen_denied_recorded`     | A reopen denies with `reopen_denied_requires_revalidation`.                                                               |
| `lineage_linkback_resolved`          | A transient delivery resolves to a durable linkback.                                                                      |
| `lineage_linkback_lost`              | A durable linkback becomes invalid (canonical target deleted, policy-blocked); the lineage records a placeholder or audit_trail_only. |
| `lineage_closed`                     | The lineage closes (canonical target resolved, permanent dismissal, policy-governed expiry).                              |
| `lineage_denial_emitted`             | Any of the frozen denial reasons fires.                                                                                   |

## 11. Schema-of-record posture

The eventual shell / attention-routing crate's Rust types are the
source of truth. The JSON Schema export at
`schemas/ux/event_lineage.schema.json` is the cross-tool boundary
every non-owning surface reads. Adding a new event class,
escalation reason, linkback-target kind, delivery-step stage,
release-step kind, audit-event id, or denial reason is
additive-minor and bumps `event_lineage_schema_version`;
repurposing any existing value is breaking and requires a new
decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0014.

## 12. Reuse guarantee

This contract is reusable by every owning subsystem without
redefining core delivery semantics. A subsystem that mints an
event MUST:

1. Open one `event_lineage_record` per `canonical_event_id`;
   never split canonical object identity across two lineages
   that share the id.
2. Resolve the event to one of the frozen `event_class` rows and
   honour the default surface, allowed escalations, forbidden
   surfaces, badge classes, dedupe scheme, and required
   carry-over fields for that row.
3. Emit one `activity_event_envelope_record` per
   (`canonical_event_id`, `delivery_surface_class`,
   `client_scope`) triple and link it on `delivery_steps`.
4. Record every dismissal verb as a `dismissal_step_record`;
   never use `acknowledge`, `resolve`, `dismiss`, `snooze`,
   `mute`, or `suppress` as interchangeable labels.
5. Record every reopen as a `reopen_step_record`; never reopen
   to a generic home screen.
6. Carry at least one `linkback_record` so transient deliveries
   resolve to a durable row or audit-trail entry.
7. Preserve durable history under every `quiet_hours_mode`;
   held deliveries emit `not_delivered_held` envelopes and
   release on mode exit as one grouped digest grouped by source
   and severity.
8. Honour the high-risk-shortcut-no-bypass rules for irreversible
   or high-blast-radius actions; never deliver a high-risk
   shortcut as a toast alone.
