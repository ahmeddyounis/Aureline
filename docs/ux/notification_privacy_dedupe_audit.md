# Notification privacy and dedupe audit

This document is the **review-side audit checklist** for the notification,
notification-delivery, and OS-notification / quiet-hours contracts. It exists
so reviewers can mechanically verify — before the notification router and
desktop / companion adapters land — that grouped-burst dedupe, repeated-
failure coalescing, lock-screen-safe summaries, companion fanout, and
forbidden shortcut actions are inspectable on every emitting subsystem
rather than restated in private words per surface.

The audit is normative. Where this document disagrees with the UI / UX
Spec or with the upstream attention-routing, notification, notification-
delivery, durable-work, durable-job-envelope, or OS-notification /
quiet-hours contracts, the source spec wins and this document, the
companion artifacts, and the fixtures must change in the same patch.
Where a downstream surface invents private dedupe vocabulary, a private
privacy posture, a private shortcut path, or a private companion fanout
rule, this audit wins and the surface is non-conforming.

## Companion artifacts

- [`/artifacts/ux/badge_class_review.yaml`](../../artifacts/ux/badge_class_review.yaml)
  binds every closed `badge_class` value to one source-subsystem set,
  one canonical-object kind, one count-basis rule, one held-count
  posture, and one privileged-detail review surface.
- [`/artifacts/ux/quiet_hours_override_matrix.yaml`](../../artifacts/ux/quiet_hours_override_matrix.yaml)
  names the narrow override cases — security advisories, trust
  downgrades, approval expiries, route warnings — that may break a
  quiet-hours hold and the evidence they must leave behind.
- [`/fixtures/ux/notification_privacy_cases/`](../../fixtures/ux/notification_privacy_cases/)
  contains worked YAML fixtures one per audit row binding the
  notification event, the suppression record, the badge-class review
  row, and (where applicable) the quiet-hours override row.

## Upstream contracts

This audit composes with existing owners and does not replace them:

- [`docs/ux/attention_activity_taxonomy.md`](./attention_activity_taxonomy.md)
  owns delivery-surface class, attention class, interruptibility tier,
  quiet-hours mode, suppression reason, privacy payload class, dedupe
  scheme, dismissal verb, badge class, durable-job lifecycle state,
  reopen-target kind, and source-subsystem vocabularies. This audit
  re-uses every axis verbatim and never re-mints them.
- [`docs/ux/notification_contract.md`](./notification_contract.md) and
  [`/schemas/ux/notification_event.schema.json`](../../schemas/ux/notification_event.schema.json)
  own the surface-class, durability, badge-count, and exact-reopen
  contract every notification event records.
- [`docs/ux/notification_delivery_contract.md`](./notification_delivery_contract.md)
  and [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json)
  own canonical event lineage, routing, dedupe collapse, redaction,
  durable linkbacks, the action taxonomy, and the high-risk-shortcut-
  no-bypass row.
- [`docs/ux/os_notification_and_quiet_hours_contract.md`](./os_notification_and_quiet_hours_contract.md)
  and [`/schemas/ux/notification_suppression_record.schema.json`](../../schemas/ux/notification_suppression_record.schema.json)
  own the suppression record, the privacy-safe payload rule record,
  the desktop-summary affordance record, the closed forbidden-shortcut
  action classes, the closed allowed-label kinds, and the closed
  redaction template tokens.
- [`/artifacts/ux/quiet_hours_policy_matrix.yaml`](../../artifacts/ux/quiet_hours_policy_matrix.yaml)
  owns per-`quiet_hours_mode` suppression / preservation rules. This
  audit cites those rows by id and never re-declares them.
- [`docs/ux/durable_work_contract.md`](./durable_work_contract.md) and
  [`/schemas/ux/job_row.schema.json`](../../schemas/ux/job_row.schema.json)
  own the durable row a badge count or held-event record resolves back
  to. Counts that point at surface-local counters rather than durable
  rows are non-conforming.

## Who reads this document

- **Shell, notification, activity-center, badge, OS-shim, and companion
  authors** verifying their delivery routes against the same five
  audit checks reviewers run. The audit is not a release gate by
  itself; it is the structured worksheet release gates read.
- **Owning subsystems** minting events that reach users, before they
  ship a new event class. Each subsystem confirms its events resolve
  the five audit rows mechanically — never by free-text rationale.
- **Support, parity-audit, and review tooling** mechanically joining
  `canonical_event_id`, `event_lineage_id_ref`,
  `canonical_object_target_ref`, `suppression_record_id`,
  `desktop_summary_affordance_id`, and badge-review-row ids across
  the contracts above and this audit.
- **Reviewers** confirming, before notification behavior hardens, that
  privacy-safe summaries are distinguishable from privileged details
  and that every count is traceable to a canonical durable row or an
  authoritative object rather than a per-surface counter.

## Scope

This revision freezes the **review checklist** for five audit rows and
the **privacy / privileged-detail boundary** for every OS-bound or
companion-visible payload. It does not invent new vocabulary; every
axis is a re-export of the upstream contracts above.

In scope:

1. Grouped-burst dedupe — confirming repeated deliveries collapse
   under one of the five frozen `dedupe_key_scheme` values and
   preserve `grouped_burst_id` lineage rather than minting per-burst
   ids per surface.
2. Repeated-failure coalescing — confirming that retry bursts on one
   pipeline collapse under `subsystem_plus_object_plus_phase`, that
   the resulting digest names `failed_runs` rather than
   `completion_unread`, and that escalation through
   `severity_increased` /
   `repeated_dedupe_threshold_crossed` is recorded with a typed
   reason rather than decorative copy.
3. Lock-screen-safe summaries — confirming default
   `lock_screen_safe_generic` posture, the bounded
   `lock_screen_safe_scoped` upgrade rule, the
   `policy_forbidden_on_lock_screen` denial path, and that no raw
   path / URL / secret material / prompt text / customer identifier
   reaches the OS sink.
4. Companion fanout — confirming that desktop, companion, remote-
   agent, and managed-admin deliveries collapse under
   `cross_client_canonical_event_id`, that companion payload class
   never widens beyond the lineage's declared class, and that
   companion shortcut actions never complete a forbidden mutation.
5. Forbidden shortcut actions — confirming the thirteen closed
   forbidden-class set is enumerated on every payload that could
   otherwise expose a destructive / mutating / policy-overriding
   shortcut.

Out of scope (frozen):

- Platform notification adapter code (macOS User Notifications, Linux
  XDG `org.freedesktop.Notifications`, Windows Toast and Action
  Center, browser Push API, mobile companion push providers, dock /
  taskbar / system-tray implementation specifics).
- Email, SMS, push transport, and mobile notification implementation.
- Final copy, iconography, animation, layout, and accessible-name
  authoring per platform.
- The eventual notification-router / desktop-summary crate's Rust
  types — the JSON Schemas above reserve the boundary shape until the
  crate lands.

## Privacy-safe summaries vs. privileged details

The audit distinguishes two payload tiers per surface. Reviewers MUST
confirm both posture and overflow path before any new event class
ships:

| Tier | What may render on OS / companion / lock-screen | Where the privileged detail lives |
| --- | --- | --- |
| Privacy-safe summary | `lock_screen_safe_generic` (category + severity + lifecycle + member-count + audit-only marker) and `lock_screen_safe_scoped` (adds workspace label, session label, next-action label) only. | The in-product canonical-object surface, review sheet, diff view, attention item, durable job row, or evidence packet. |
| Privileged detail | Never on `lock_screen_summary`, `os_notification`, `companion_push`, `os_badge_app_icon`, `dock_taskbar_progress`, `dock_taskbar_badge`, `system_tray_summary`, `desktop_widget_summary`, or `lock_screen_quick_action`. | Always on the in-product review surface; reopen routes through the canonical object identity. |

Forbidden label kinds — `raw_path`, `raw_url`, `raw_email`,
`raw_provider_payload`, `raw_secret_material`, `raw_prompt_text`,
`raw_completion_text`, `raw_command_body`, `customer_owned_identifier`,
`destructive_intent_phrase`, `object_identity_on_lock_screen`,
`actor_real_name_on_lock_screen`, `review_diff_excerpt_on_lock_screen` —
are stripped before bytes reach the OS sink and recorded on
`privacy_redaction.stripped_label_kinds` so the audit row can show
exactly what was redacted (re-exported from
`docs/ux/os_notification_and_quiet_hours_contract.md` §Privacy-safe
payload classes).

A surface that surfaces any item from the privileged tier on a non-
in-product surface is non-conforming
(`denial_reason = privacy_payload_class_widened_across_clients`,
`raw_body_forbidden_on_os_notification`,
`raw_secret_forbidden_on_any_surface`,
`raw_prompt_text_forbidden_on_os_notification`, or
`privacy_payload_class_missing_on_lock_screen` per the upstream
schemas).

## Audit rows

Each audit row names the **scenario**, the **upstream contract
section** the reviewer reads first, the **mechanical proof** the
reviewer checks (one or more refs from the upstream schemas), the
**failure mode** that fires when the proof is missing, and the
**fixture** under `/fixtures/ux/notification_privacy_cases/` the
reviewer cites when signing off.

### 1. Grouped-burst dedupe

| Field | Value |
| --- | --- |
| Scenario | Repeated deliveries of one event collapse under a single lineage rather than stacking toasts, app-icon badges, or OS notifications. |
| Upstream | `attention_activity_taxonomy.md` §Dedupe-key scheme; `notification_delivery_contract.md` §3 / §4 / §5.6; `notification_contract.md` §Dedupe, Grouping, and Escalation; `os_notification_and_quiet_hours_contract.md` §Suppression class table (`collapsed_dedupe_same_grouped_burst`). |
| Proof | One `grouped_burst_record` with one `grouped_burst_id`, `member_count` evolving across deliveries, `first_event_id_ref` and `latest_event_id_ref` bracketing the lineage, and one `dedupe_key_scheme` from the closed five. Per-surface envelopes share the same `canonical_event_id`. |
| Failure | `dedupe_violated_stacked_toasts`, `grouped_burst_split_without_new_lineage`, `companion_cross_client_divergence`. |
| Fixture | `fixtures/ux/notification_privacy_cases/grouped_burst_dedupe_audit.yaml`. |

Reviewer checklist:

- [ ] Every emitted `notification_event_record` for the burst names
  the same `canonical_event_id`.
- [ ] The `dedupe_policy.dedupe_key_scheme` is one of
  `canonical_event_id`,
  `canonical_object_target_plus_event_class`,
  `grouped_burst_id`, `subsystem_plus_object_plus_phase`,
  `cross_client_canonical_event_id`.
- [ ] `grouped_burst_id_ref` is consistent across every surface
  instance and across cross-client siblings.
- [ ] `member_count` evolves rather than minting new digest cards.
- [ ] No surface stacks repeat toasts (`shows_individual_member_toasts =
  false` on the digest envelope).
- [ ] The release path on mode exit emits one
  `release_step_record.kind = mode_exit_grouped_digest`.

### 2. Repeated-failure coalescing

| Field | Value |
| --- | --- |
| Scenario | Ten retries against one pipeline (build, test run, indexer pass, debug session, save_or_sync, AI apply, provider handoff) collapse into one evolving lineage with the right badge class. |
| Upstream | `attention_activity_taxonomy.md` §Badge class / §Repeated-event dedupe; `notification_delivery_contract.md` §4 (`degraded_or_reconnecting_state`, `repeated_low_severity_burst`, `long_running_job_progress`); `durable_work_contract.md` activity-center partitions. |
| Proof | One `grouped_burst_record` whose `dedupe_key_scheme = subsystem_plus_object_plus_phase`; one `badge_record` whose `badge_class = failed_runs` (not `completion_unread`); a typed `escalation_reason` (`severity_increased`, `duration_threshold_crossed`, `repeated_dedupe_threshold_crossed`) when the lineage tier rises. |
| Failure | `badge_count_inflation_mixed_classes`, `escalation_reason_missing_on_tier_change`, `dedupe_violated_stacked_toasts`. |
| Fixture | `fixtures/ux/notification_privacy_cases/repeated_failure_coalescing_audit.yaml`. |

Reviewer checklist:

- [ ] The retry burst increments `failed_runs` only — no `mentions`,
  `needs_review`, or `completion_unread` from the same envelopes.
- [ ] `badge_count_policy.count_basis` is `deduped_durable_item` or
  `group_member_count`, never a per-toast tally.
- [ ] The escalation step on the lineage carries a non-`none`
  `escalation_reason` when the tier increases.
- [ ] The durable-job row stays the source of truth; the toast and OS
  notification mirror via `linkback_records` rather than holding
  their own progress / failure copy.
- [ ] The grouped-burst is re-evaluated on every retry rather than
  re-issued under a new `grouped_burst_id`.

### 3. Lock-screen-safe summaries

| Field | Value |
| --- | --- |
| Scenario | An OS notification, lock-screen summary, or companion push payload is rendered on a locked or shared surface; the payload must carry only category-level (or scoped) labels and never a privileged detail. |
| Upstream | `attention_activity_taxonomy.md` §Lock-screen-safe summaries; `notification_delivery_contract.md` §5.1 / §5.3; `os_notification_and_quiet_hours_contract.md` §Privacy-safe payload classes; `notification_contract.md` §Required event anatomy. |
| Proof | `privacy_payload_class` is `lock_screen_safe_generic` (default) or `lock_screen_safe_scoped`, never `in_product_only`; `privacy_redaction.applied_payload_class` matches the surface's payload class; `privacy_redaction.stripped_label_kinds` lists every forbidden label kind removed; reopen resolves to the in-product canonical object. |
| Failure | `privacy_payload_class_missing_on_lock_screen`, `privacy_payload_class_widened_across_clients`, `raw_body_forbidden_on_os_notification`, `raw_prompt_text_forbidden_on_os_notification`, `raw_secret_forbidden_on_any_surface`. |
| Fixture | `fixtures/ux/notification_privacy_cases/lock_screen_safe_summary_audit.yaml`. |

Reviewer checklist:

- [ ] Default lock-screen payload is `lock_screen_safe_generic`. A
  scoped upgrade names a workspace label or session label only —
  never object identity, actor real name, or diff excerpt.
- [ ] `policy_forbidden_on_lock_screen` denies the lock-screen surface
  outright; the in-product durable row remains and the lineage
  records `linkback_records[*].kind = audit_trail_only` when
  in-product render is also forbidden.
- [ ] `redaction_template_tokens` is a subset of the closed eight:
  `{category}`, `{workspace_label}`, `{session_label}`,
  `{member_count}`, `{actor_role}`, `{severity_class}`,
  `{lifecycle_phase}`, `{audit_only_marker}`.
- [ ] `stripped_label_kinds` is non-empty whenever a forbidden label
  was removed; silent strip is non-conforming.
- [ ] The reopen path resolves through the in-product surface
  (`must_resolve_through_in_product_surface = true`) rather than a
  privileged OS-owned panel.

### 4. Companion fanout

| Field | Value |
| --- | --- |
| Scenario | The same canonical event is fanned out to desktop, companion, remote-agent, and managed-admin clients; the audit confirms one canonical event id, one privacy class per delivery, and no per-client duplicate renewal. |
| Upstream | `attention_activity_taxonomy.md` §Dedupe-key scheme (`cross_client_canonical_event_id`); `notification_delivery_contract.md` §3 / §5.2 / §5.7; `notification_contract.md` §Cross-Window and Cross-Client Rules; `os_notification_and_quiet_hours_contract.md` §Suppression class table (`held_*`) + §Desktop summary affordances. |
| Proof | One `event_lineage_record` with `cross_client_lineage_refs` enumerating sibling client scopes; every sibling envelope carries the same `canonical_event_id`, the same `canonical_object_target_ref`, and a `privacy_payload_class` that does not widen beyond the lineage's declared class; companion-side `desktop_summary_affordance_record` (companion_push_action) declares `effect_class` from the read-only set and `forbidden_shortcut_action_classes` enumerates every blocked mutation. |
| Failure | `cross_client_divergence`, `companion_cross_client_divergence`, `privacy_payload_class_widened_across_clients`, `desktop_summary_affordance_drift_denied`. |
| Fixture | `fixtures/ux/notification_privacy_cases/companion_fanout_audit.yaml`. |

Reviewer checklist:

- [ ] Each client_scope appears once on `cross_client_lineage_refs`.
- [ ] No sibling carries a payload class strictly more permissive than
  the lineage's `intended_privacy_payload_class`.
- [ ] Companion push payloads share the OS-notification payload shape
  and never embed raw bodies or raw URLs.
- [ ] The companion action's `desktop_summary_affordance_record.
  effect_class` is one of `read_only_reopen_attention_item`,
  `read_only_progress_mirror`, `read_only_badge_mirror`,
  `read_only_summary_mirror`, or `mutation_via_review_path_only`
  (never a direct mutation class).
- [ ] Cross-client dismissal collapses via
  `release_step.release_trigger_class = cross_client_collapse` so a
  user dismissing on one client does not strand the row on another.

### 5. Forbidden shortcut actions

| Field | Value |
| --- | --- |
| Scenario | A high-risk action — destructive publish / apply, secret reveal, irreversible high-blast operation, policy override, trust-state change, provider-grant change, cross-workspace mutation — could otherwise be completed from an OS-level shortcut, lock-screen quick action, companion push action, dock or taskbar shortcut, or system-tray summary. The audit confirms the shortcut path refuses and routes through the in-product review sheet, approval workflow, or revalidation step. |
| Upstream | `notification_delivery_contract.md` §9 (no bypass on high-risk shortcuts) and §4 row `high_risk_shortcut_no_bypass`; `os_notification_and_quiet_hours_contract.md` §No-bypass on high-risk shortcuts; `shell_interaction_safety_contract.md` interaction-safety packet. |
| Proof | `forbidden_shortcut_action_classes` enumerates every blocked class from the closed thirteen; `bypass_protection.must_route_through_review_sheet`, `must_route_through_approval_workflow`, or `requires_revalidation` is `true` with a non-null ref; `desktop_summary_affordance_record.effect_class` is `mutation_via_review_path_only` or `forbidden_mutation_shortcut`; `interaction_safety_packet_id_ref` is non-null and quotes the consequence-bearing interaction by ref. |
| Failure | `high_risk_shortcut_toast_only_forbidden`, `high_risk_shortcut_bypassed_review_sheet`, `desktop_summary_affordance_drift_denied`, `dismissal_silently_mutated_source_object`, `resolve_silently_mutated_source_object`. |
| Fixture | `fixtures/ux/notification_privacy_cases/forbidden_shortcut_audit.yaml`. |

Reviewer checklist:

- [ ] The thirteen forbidden classes are explicitly listed:
  `destructive_publish_or_apply`, `secret_or_credential_reveal`,
  `irreversible_high_blast`, `bypass_review_sheet`,
  `bypass_approval_workflow`, `cross_workspace_mutation`,
  `direct_mutation_from_lock_screen`,
  `direct_mutation_from_companion_push`,
  `direct_mutation_from_dock_or_taskbar`,
  `direct_mutation_from_system_tray`,
  `policy_override_from_os_shortcut`,
  `trust_state_change_from_os_shortcut`,
  `provider_grant_change_from_os_shortcut`.
- [ ] Lock-screen, companion, dock / taskbar, system-tray, and
  desktop widget affordances are read-only or route through a
  review path; mutation classes are non-conforming on those
  surfaces.
- [ ] An OS notification action that completes a high-risk action
  carries `effect_class = mutation_via_review_path_only` and a
  non-null `review_sheet_ref`.
- [ ] `announce_actions_to_assistive_tech` is `true` for every OS
  shortcut button and lock-screen quick action so screen reader and
  switch control users see the same actions a sighted user does.
- [ ] Refusal records `suppression_record_bypass_blocked` on the
  audit stream so the no-bypass rule is mechanically reviewable.

## Audit-stream cross-reference

The audit rows above route to existing audit-event ids on the upstream
streams; this audit does not mint new audit events. Reviewers cite the
existing ids when they sign off:

| Audit row | Audit-event ids fired |
| --- | --- |
| Grouped-burst dedupe | `lineage_dedupe_recorded`, `lineage_digest_released`, `desktop_summary_affordance_synced`. |
| Repeated-failure coalescing | `lineage_dedupe_recorded`, `lineage_escalation_recorded`, `badge_record_updated` (re-exported from the badge contract), `lineage_release_recorded`. |
| Lock-screen-safe summaries | `suppression_record_redaction_recorded`, `suppression_record_audit_trail_only_emitted`, `lineage_denial_emitted` (when a privacy-class or raw-body denial fires). |
| Companion fanout | `lineage_delivery_recorded`, `desktop_summary_affordance_drift_denied`, `lineage_denial_emitted` (cross-client divergence). |
| Forbidden shortcut actions | `suppression_record_bypass_blocked`, `lineage_denial_emitted`, `lineage_reopen_denied_recorded`. |

## Schema-of-record posture

This audit is a review-side checklist; it carries no schema of its own.
Every cited record (`notification_event_record`, `event_lineage_record`,
`activity_event_envelope_record`, `notification_suppression_record`,
`privacy_safe_payload_rule_record`, `desktop_summary_affordance_record`,
`badge_record`, `grouped_burst_record`,
`quiet_hours_policy_row_record`, `quiet_hours_override_row_record`)
is owned by an upstream schema. Adding a new audit row is additive and
attaches to the existing schemas; it does not bump a schema version.

## Reuse guarantee

This audit is reusable by every owning subsystem and every reviewer.
A subsystem that mints a notification-bearing event MUST be able to
prove, mechanically:

1. Every repeated delivery for the event collapses under one of the
   five frozen `dedupe_key_scheme` values; per-surface envelopes share
   the same `canonical_event_id` and, where applicable, the same
   `grouped_burst_id`.
2. Retry bursts on one pipeline coalesce into the right badge class
   (`failed_runs` for retryable failure, `durable_running_count` for
   running work, `completion_unread` for finished but unread work,
   `held_or_suppressed_count` for held bursts) and never combine
   classes into one overloaded count.
3. Lock-screen, companion, and OS-notification payloads carry only
   privacy-safe summaries; privileged details are reachable only by
   reopening through the in-product surface that owns the canonical
   object.
4. Companion fanout collapses under `cross_client_canonical_event_id`;
   no per-client renewal, no payload-class widening, no
   cross-client dismissal drift.
5. Every shortcut path that could otherwise complete a forbidden
   class refuses and routes the user back into the in-product review
   sheet, approval workflow, or revalidation step before the action
   can complete.
