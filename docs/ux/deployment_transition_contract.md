# Mode-change review, disconnect-review, and deployment-boundary continuity contract

This document is the **cross-surface continuity contract** for every
transition Aureline performs between local-only, managed, mirrored,
offline, and degraded operating states. It freezes one
`mode_change_review_record` and one paired `disconnect_review_record`
so the **transition class, source posture, target posture, preserved
local state, invalidated cached authority, reauth or retry
requirements, export-before-change actions, and rollback or reopen
path** of any deployment-boundary change are inspectable as one
record family rather than scattered banner copy.

The contract exists so a sign-in sheet, a sign-out confirmation, an
org-switch sheet, a mirror-fallback notice, an offline-transition
review, a service-degradation banner, a reconnect-required prompt,
and a deployment-profile-narrow review all project the same
transition record without recomputing field names; so a mode-change
surface cannot silently invalidate or widen authority; and so a
disconnect cannot silently collapse local-safe work into a generic
"service unavailable" message when local continuity remains.

The contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX Spec, or design-system style guide, those sources win and this
document plus its schema and fixtures update in the same change.
Where an auth sheet, account chip, status-bar deployment cell,
activity center, support packet, admin-audit export, or
release-evidence excerpt mints a parallel mode-change object, a
parallel disconnect record, or a parallel deployment-boundary
continuity narrative, this contract wins and the surface is
non-conforming.

## Companion artifacts

- [`/schemas/deployment/mode_change_review.schema.json`](../../schemas/deployment/mode_change_review.schema.json)
  — boundary schema for `mode_change_review_record` and
  `disconnect_review_record`.
- [`/fixtures/deployment/mode_change_cases/`](../../fixtures/deployment/mode_change_cases/)
  — worked YAML cases covering managed sign-in, sign-out to
  local-only, org switch with cached-authority invalidation, mirror
  fallback with paired disconnect review, offline transition with
  dirty buffers, control-plane service degradation, reconnect-
  required after grace, and deployment-profile narrow from
  managed-cloud to local-only.

## Upstream contracts this contract rides on

This contract does **not** re-mint vocabulary that is already frozen
upstream; it consumes the frozen sets by name and by value:

- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  and [`/docs/governance/deployment_profile_truth.md`](../governance/deployment_profile_truth.md)
  — `deployment_profile_id` vocabulary
  (`individual_local`, `self_hosted`, `enterprise_online`,
  `air_gapped`, `managed_cloud`),
  `product_facing_label_vocabulary`,
  `tenant_org_scope_class_vocabulary`,
  `region_scope_class_vocabulary`,
  `key_mode_class_vocabulary`. Mode-change review records re-export
  those field names byte-for-byte on the `from_posture` and
  `to_posture` summaries.
- [`/docs/ux/deployment_summary_contract.md`](./deployment_summary_contract.md)
  and [`/schemas/deployment/deployment_summary_card.schema.json`](../../schemas/deployment/deployment_summary_card.schema.json)
  — the deployment-posture summary fields the review cites by name
  (`mirror_offline_state_class`, `control_plane_state_summary`,
  `data_plane_state_summary`,
  `prohibited_implied_claim_class`,
  `consumer_surface_class`). A mode-change review never paraphrases
  those fields and never widens the resulting card's prohibited-
  claim guardrails.
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md),
  [`/schemas/auth/managed_session_state.schema.json`](../../schemas/auth/managed_session_state.schema.json),
  and [`/schemas/auth/reauth_requirement.schema.json`](../../schemas/auth/reauth_requirement.schema.json)
  — the managed-session state classes, flow classes, no-silent-
  signout guarantee, and `reauth_requirement_record` shape the
  review cites when a transition pauses managed capabilities behind
  fresh auth. The mode-change review never mints a parallel reauth
  shape; it links existing reauth requirement refs.
- [`/schemas/deployment/local_core_continuity_packet.schema.json`](../../schemas/deployment/local_core_continuity_packet.schema.json)
  and [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  — `control_plane_service_class`,
  `control_plane_service_state_class`,
  `data_plane_capability_class`,
  `data_plane_capability_state_class`, `mirror_freshness_class`,
  `restore_class`. The review and the disconnect record re-export
  those names verbatim and link the underlying continuity packet
  that resolved against the same posture.
- [`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md)
  and [`/schemas/ux/transport_posture.schema.json`](../../schemas/ux/transport_posture.schema.json)
  — `offline_or_deny_all_state` vocabulary the review's
  `mirror_offline_state_class` field re-exports verbatim plus the
  modal-prohibition rule for inspect-only export and rollback
  actions.
- [`/docs/ux/control_data_plane_status_contract.md`](./control_data_plane_status_contract.md)
  and [`/schemas/ops/outage_notice.schema.json`](../../schemas/ops/outage_notice.schema.json)
  — plane-separation rule and the local-continuity vocabulary the
  disconnect record cites when a control-plane outage is in flight.

## Who reads this contract

- **Auth sheet, account chip, sign-out confirmation, org-switch
  sheet, mirror-fallback notice, offline-transition review,
  service-degradation banner, reconnect-required prompt, deployment-
  profile narrow review, and CLI `--explain-transition` text
  formatter** — to read **one** transition record family instead of
  recomputing fields per surface.
- **Status-bar deployment cell, activity center, project doctor /
  attention inbox, support-packet builder, admin-audit-export
  emitter, release-evidence excerpt builder** — to render the same
  record bytes for every consumer.
- **Reviewers (release, security, accessibility, claim-manifest)** —
  to verify that no transition silently widens authority, that
  cached authority invalidations remain explicit, that the local-
  continuity statement on every disconnect cannot collapse into a
  generic outage, and that export-before-change and rollback paths
  remain inspect-only and reviewable.

## Two questions the contract answers

Any Aureline surface running a deployment-boundary transition MUST
answer both questions mechanically, without per-surface copy:

1. **What is changing right now?** Which `transition_class`?
   What is the source `from_posture` (deployment profile,
   tenant/org, region, key mode, mirror/offline state, control-
   plane state, data-plane state)? What is the target `to_posture`?
   What stays local? What managed authority is invalidated, and what
   recheck applies? Which reauth or retry is required, and which
   local capabilities continue while it is required?
2. **What if the user does not want to commit, or service surfaces
   pause, degrade, or go stale during the change?** Which
   `export_before_change_action` does the surface offer for the
   user-owned local state that may be affected? Which
   `rollback_or_reopen_path` lets the user step back? When the
   transition is itself a disconnect (mirror fallback, offline
   transition, service degradation, reconnect-required), what does
   the paired `disconnect_review_record` say remains available
   locally, what is paused, and how does the user reconnect?

Generic prose like "signing in", "switched", "now offline",
"reconnecting", or "service unavailable" is forbidden when a more
precise transition class, posture pair, and continuity statement are
knowable. The schema enforces typed vocabulary and typed sentences;
surfaces render those values.

## 1. Scope

This contract freezes:

- One **mode-change review record** (§3) emitted whenever a
  deployment-boundary transition is proposed, in-flight, committed,
  or rolled back. The record carries the
  `transition_class` (§3.3), the `from_posture` and `to_posture`
  deployment summaries (§3.4), the `preserved_local_state` (§3.5),
  the `invalidated_cached_authority_rows[]` (§3.6), the
  `reauth_or_retry_requirements` (§3.7), the
  `export_before_change_actions[]` (§3.8), the
  `rollback_or_reopen_path` (§3.9), the `authority_change_guard`
  (§3.10), the `consumer_surfaces[]` (§3.11), and the optional
  paired `disconnect_review_ref` (§3.12) when the transition is
  itself a disconnect, degrade, or stale event.
- One **disconnect review record** (§4) emitted whenever a service
  surface pauses, degrades, or goes stale for the deployment.
  The record carries the `disconnect_event_class` (§4.3), the
  `affected_service_classes[]` and
  `affected_capability_classes[]` (§4.4), the
  `local_continuity_statement` (§4.5), the
  `paused_capabilities[]` and `stale_artifacts[]` (§4.6), the
  `reconnect_path_class` and `reconnect_label` (§4.7), the
  `freshness_label` (§4.8), and the optional back-pointer
  `mode_change_review_ref` to the transition that produced the
  disconnect when one applies.
- The **cross-record invariants** (§5) so review sheets cannot
  silently invalidate or widen authority, cannot lose preserved
  local state, cannot omit a rollback or reopen path, and disconnect
  reviews cannot flatten local continuity into a generic outage.

## 2. Out of scope

- The actual transition handler, session migration logic, mirror
  failover engine, offline-bundle loader, network-disabled
  enforcement code, deny-all enforcement code, and managed sign-in
  / sign-out / org-switch backends. This contract pins the
  inspectable record family; it does not implement any runtime
  transition.
- Final user-facing copy and microcopy. This contract pins the
  closed sets the copy resolves against; the design-system style
  guide and shell-interaction-safety contract own the strings.
- Per-platform widget toolkits, sheet animations, focus management,
  and accessibility wiring. The component contracts and the
  shell-interaction-safety contract own those.
- Telemetry wire format and the diagnostics-bundle envelope. The
  schema registry rows for the auth-sheet, status-bar, support,
  admin-audit, and release-evidence packet families consume this
  record family separately.

## 3. The mode-change review record

A mode-change review is one structured projection of the current
proposed, in-flight, committed, or rolled-back deployment-boundary
transition. The auth sheet, account chip, sign-out confirmation,
org-switch sheet, mirror-fallback notice, offline-transition review,
service-degradation banner, reconnect-required prompt, deployment-
profile narrow review, and CLI text formatter each render the same
record without changing field names.

A surface that runs a deployment-boundary transition without first
emitting one `mode_change_review_record` is non-conforming. A surface
that emits the record but commits before the user reviews the
preserved-local-state, cached-authority-invalidation, and reauth-or-
retry rows is non-conforming. A surface that paraphrases or omits
the rollback-or-reopen path is non-conforming.

### 3.1 Required fields

- `record_kind = mode_change_review_record`.
- `mode_change_review_schema_version = 1`.
- `review_id` — opaque, stable, safe to log and export.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `transition_class` — closed set (§3.3).
- `transition_phase` — closed set: `proposed`, `in_progress`,
  `committed`, `rolled_back`. The schema enforces phase ordering on
  rollback paths; see §5.6.
- `triggered_by_user_action` — boolean. Required `false` for
  involuntary transitions (`mirror_fallback`, `offline_transition`,
  `service_degradation`, `reconnect_required`); required `true` for
  voluntary transitions (`sign_in`, `sign_out`, `org_switch`,
  `deployment_profile_narrow`).
- `from_posture` — see §3.4.
- `to_posture` — see §3.4.
- `preserved_local_state` — see §3.5.
- `invalidated_cached_authority_rows` — array of structured rows
  (§3.6). Empty only when the transition does not affect any cached
  authority (for example, a sign-in starting from `local_only`
  posture with no prior managed binding).
- `reauth_or_retry_requirements` — see §3.7.
- `export_before_change_actions` — array of inspect-only export
  actions (§3.8). Required non-empty when the transition narrows
  capability or invalidates cached authority and the user has any
  unsaved or managed-only state that could be affected.
- `rollback_or_reopen_path` — see §3.9. Every record carries
  exactly one rollback-or-reopen path; if no rollback is possible,
  the record declares `no_rollback_after_commit` with a reviewable
  reason (§3.9).
- `authority_change_guard` — see §3.10.
- `consumer_surfaces` — closed set of surfaces consuming the record
  (§3.11). Non-empty.
- `redaction_class` — one of `metadata_safe_default`,
  `operator_only_restricted`, `internal_support_restricted`,
  `signing_evidence_only`.
- `export_safe` — boolean. Required `true` when the record is
  exported through support, admin-audit, or release-evidence
  surfaces. The schema enforces that exported records never widen
  redaction relative to the underlying continuity packet.

Optional fields: `disconnect_review_ref` (required by §3.12 for
disconnect-class transitions), `linked_continuity_packet_ref`,
`linked_managed_session_state_ref`, `linked_deployment_summary_card_ref`,
`linked_reauth_requirement_refs[]`, `linked_outage_notice_refs[]`,
`linked_transport_posture_ref`, `notes`.

### 3.2 No-silent-change invariants

A mode-change review never:

- silently invalidates a managed session (every revoked session is
  surfaced as an `invalidated_cached_authority_row` with a typed
  `invalidation_reason_class`);
- silently widens authority (every widening transition carries an
  `authority_change_guard.authority_change_class =
  widens_with_explicit_consent`, an
  `explicit_consent_recorded = true`, and a non-null
  `consent_evidence_ref`);
- silently loses preserved local state (every record carries
  `preserved_local_state.no_silent_loss_guarantee = true` and
  enumerated retained capabilities);
- skips the rollback-or-reopen path (every record carries
  exactly one path, even if it resolves to
  `no_rollback_after_commit` with a typed reason).

These are schema-enforced.

### 3.3 `transition_class` (closed)

- `sign_in` — managed identity becomes bound for its declared scope.
  Voluntary; widens authority only with explicit consent.
- `sign_out` — managed identity is unbound. Voluntary; narrows
  authority. Local artifacts and user-owned exports remain.
- `org_switch` — active org / tenant scope changes. Voluntary;
  preserves local artifacts. Cached authority bound to the previous
  org is invalidated.
- `mirror_fallback` — live origin egress is no longer reachable; the
  deployment falls back to its mirror. Involuntary; narrows to
  mirror-backed read-only or boundary-recheck-required state.
- `offline_transition` — public-internet egress becomes unavailable
  (network disabled by user, network unreachable, or deny-all
  enforced). Involuntary; narrows to local-safe or offline-bundle
  state. Local edit/save/undo/export remain.
- `service_degradation` — one or more control-plane service classes
  enter a non-healthy state (stale_cache, unavailable, mirror_only,
  boundary_recheck_required). Involuntary; the data-plane summary
  retains every local-safe capability the underlying packet still
  reports.
- `reconnect_required` — the deployment grace window closed and the
  next attempted managed action requires a fresh reconnect.
  Involuntary; pauses managed capabilities until the user
  reconnects. Local work continues.
- `deployment_profile_narrow` — the deployment profile narrows
  (e.g. `managed_cloud` to `individual_local`,
  `enterprise_online` to `self_hosted`, `self_hosted` to
  `air_gapped`). Voluntary; the to-posture's
  `prohibited_implied_claim_classes[]` becomes a superset of the
  from-posture's so claims cannot widen across the narrow.

### 3.4 `from_posture` and `to_posture`

Both summaries share the same `deployment_posture_summary` shape and
re-export the deployment-summary card vocabulary verbatim. Required
fields:

- `deployment_profile` — re-export of `deployment_profile_id`.
- `product_facing_label_class` — re-export.
- `tenant_org_scope_class` — re-export.
- `region_scope_class` — re-export.
- `key_mode_class` — re-export.
- `mirror_offline_state_class` — re-export.
- `control_plane_worst_state_class` — re-export of
  `control_plane_service_state_class`. The worst state across all
  in-scope service classes at the captured moment.
- `data_plane_worst_state_class` — re-export of
  `data_plane_capability_state_class`.
- `prohibited_implied_claim_classes[]` — re-export of
  `prohibited_implied_claim_class` values the corresponding card
  has denied. The schema enforces that
  `to_posture.prohibited_implied_claim_classes[]` is a superset of
  `from_posture.prohibited_implied_claim_classes[]` whenever the
  `transition_class` is `deployment_profile_narrow`,
  `mirror_fallback`, `offline_transition`, or
  `service_degradation` — narrowing posture cannot drop a
  prohibited-claim guardrail.
- `posture_label` — short reviewable sentence in product terms.

Optional: `linked_summary_card_ref`,
`linked_continuity_packet_ref`,
`linked_transport_posture_ref`.

### 3.5 `preserved_local_state`

Required fields:

- `local_artifacts_safe` — boolean, const `true`. The transition
  cannot imply local files are unsafe.
- `edit_save_undo_export_available` — boolean, const `true`. The
  transition cannot imply local edit, save, undo, or user-owned
  export is unavailable.
- `no_silent_loss_guarantee` — boolean, const `true`.
- `retained_local_capabilities[]` — non-empty array of typed
  reviewable labels naming what continues locally.
- `retained_local_artifact_class[]` — closed-set re-export of
  `data_plane_capability_class` values the local-safe summary
  retains. Non-empty.
- `retained_credential_classes[]` — closed set:
  `os_store`, `customer_managed_keystore`,
  `offline_trust_root`, `none_required`. Empty only when the
  transition does not retain any credential surface (for example,
  sign-out from local-only with no managed binding).
- `unsaved_buffer_count` — integer ≥ 0. When non-zero, the schema
  enforces that `export_before_change_actions[]` is non-empty for
  voluntary narrowing transitions.
- `safety_label` — short reviewable sentence in product terms.

### 3.6 `invalidated_cached_authority_rows[]`

Each row is a structured projection of one cached authority that the
transition invalidates or requires recheck of. Required fields:

- `row_id` — opaque, stable.
- `authority_class` — closed set:
  `managed_session`, `signed_policy_bundle`, `signed_docs_pack`,
  `mirror_trust_root`, `ai_provider_token`,
  `browser_handoff_capability`, `companion_notification_channel`,
  `hosted_control_plane_session`, `customer_signed_offline_bundle`.
- `invalidation_reason_class` — closed set:
  `session_revoked`, `org_changed`, `tenant_changed`,
  `policy_epoch_changed`, `key_mode_changed`,
  `profile_changed`, `mirror_changed`, `offline_boundary_entered`,
  `service_unavailable`, `boundary_recheck_required`,
  `seat_removed`, `account_deprovisioned`.
- `recheck_required` — boolean.
- `scope_label` — short reviewable sentence in product terms naming
  the actor scope, org / tenant, or capability scope the authority
  applied to.
- `evidence_links[]` — opaque refs into the records that document
  the authority and its invalidation (for example, the
  `managed_session_state_record`, the
  `signed_policy_bundle_record`, the prior continuity packet, or
  the outage notice).

The schema enforces that no row carries
`authority_class = managed_session` paired with
`invalidation_reason_class` outside the closed set
`{ session_revoked, org_changed, tenant_changed,
seat_removed, account_deprovisioned, profile_changed,
service_unavailable, offline_boundary_entered }`. Sign-in is **not**
an invalidation; surfaces that mint a managed-session row on a
sign-in transition are non-conforming.

### 3.7 `reauth_or_retry_requirements`

Required fields:

- `reauth_class` — closed set: `required`, `optional`,
  `not_required`.
- `retry_class` — closed set: `required_after_change`, `optional`,
  `not_required`, `blocked_until_reconnect`,
  `manual_reconcile_after_boundary_change`.
- `reauth_requirement_refs[]` — opaque refs into existing
  `reauth_requirement_record` rows. Required non-empty whenever
  `reauth_class = required`.
- `blocked_managed_actions[]` — array of typed reviewable labels
  naming managed actions that pause until reauth or retry succeeds.
  Empty only when `reauth_class = not_required` and `retry_class ∈
  { not_required, optional }`.
- `local_continuity_label` — short reviewable sentence stating
  which local capabilities continue while reauth or retry is
  required.

### 3.8 `export_before_change_actions[]`

Each export action is inspect-only and follows the
repair-action-card shape from
[`/docs/ux/transport_and_environment_status_contract.md`](./transport_and_environment_status_contract.md).
Each carries:

- `action_id` — opaque.
- `label` — short reviewable label (e.g. "Export local archive
  before sign-out").
- `target_route_ref` — opaque ref to the export route.
- `scope_class` — `scope_local_only`. Export-before-change actions
  cannot reach beyond the local device.
- `authority_class` — `user_local_authority`.
- `consent_class` — `no_consent_required_safe_default`.
- `side_effects` — exactly `["no_side_effect_inspect_only"]`.
- `preserves_evidence_context` — `true`.
- `revalidation_on_open` — one of `none_already_fresh`,
  `snapshot_open_read_only`. Generic refresh-on-open is non-
  conforming; export-before-change resolves against the captured
  posture only.
- `modal_prohibited` — `true`.

### 3.9 `rollback_or_reopen_path`

Every record carries exactly one rollback-or-reopen path. Required
fields:

- `rollback_class` — closed set:
  `full_rollback_available`,
  `partial_rollback_available`,
  `reopen_with_warning`,
  `no_rollback_after_commit`,
  `no_rollback_required_inspect_only`.
- `rollback_label` — short reviewable label.
- `rollback_action` — see action shape below.
- `rollback_safe` — boolean. Required `true` for
  `full_rollback_available`,
  `partial_rollback_available`,
  `reopen_with_warning`, and
  `no_rollback_required_inspect_only`. Required `false` for
  `no_rollback_after_commit`.
- `no_rollback_reason_label` — short reviewable sentence. Required
  non-null whenever `rollback_class = no_rollback_after_commit`;
  `null` otherwise.

The `rollback_action` follows the same inspect-only shape as the
export-before-change action above, with one widening: when
`rollback_class = full_rollback_available` or
`partial_rollback_available`, `revalidation_on_open` MAY include
`requery_before_batch` so the rollback step re-resolves the
captured posture before performing the local-scope revert. The
`rollback_action.scope_class` is `scope_local_only` for
voluntary-narrow transitions and for involuntary transitions; a
rollback action that crosses the local boundary is non-conforming.

### 3.10 `authority_change_guard`

Required fields:

- `authority_change_class` — closed set:
  `narrows_only`,
  `maintains_orthogonal`,
  `widens_with_explicit_consent`.
- `silent_widening_blocked` — boolean, const `true`.
- `explicit_consent_recorded` — boolean. Required `true` whenever
  `authority_change_class = widens_with_explicit_consent`; required
  `false` otherwise.
- `consent_evidence_ref` — opaque ref into the consent ledger row
  that records the user's explicit consent. Required non-null
  whenever `explicit_consent_recorded = true`; `null` otherwise.
- `widening_summary_label` — short reviewable sentence in product
  terms describing the new authority scope. Required non-null
  whenever `authority_change_class = widens_with_explicit_consent`;
  `null` otherwise.

### 3.11 `consumer_surface_class` (closed)

- `auth_sheet` — managed sign-in / sign-out / step-up sheet.
- `account_chip` — status-bar account / org chip.
- `org_switch_sheet` — org / tenant switch sheet.
- `mode_change_sheet` — generic mode-change sheet for offline,
  mirror-fallback, service-degradation, reconnect-required, and
  deployment-profile narrow transitions.
- `status_bar_cell` — status-bar deployment cell projection.
- `activity_center_row` — durable activity-center row for the
  transition.
- `support_packet_export` — support-bundle export.
- `admin_audit_export` — admin-audit export.
- `release_evidence_excerpt` — release-evidence packet section.
- `cli_text_formatter` — CLI text formatter render.
- `companion_surface` — browser-companion summary projection.
  Schema-denied for any record whose `to_posture.deployment_profile`
  is `air_gapped` (the air-gapped profile's
  `companion_surface_posture_class` is
  `companion_handoff_explicitly_disallowed`).

### 3.12 `disconnect_review_ref`

The `disconnect_review_ref` is required (non-null) whenever
`transition_class` is `mirror_fallback`, `offline_transition`,
`service_degradation`, or `reconnect_required`. Disconnect-class
transitions without a paired disconnect review are non-conforming.

The `disconnect_review_ref` is `null` for `sign_in`, `sign_out`,
`org_switch`, and `deployment_profile_narrow` records unless the
profile narrow is itself motivated by a disconnect (e.g. an admin
forcing a profile narrow on offline-bundle import); in that case the
narrow record links the same disconnect review the underlying
service-degradation or offline-transition record links.

## 4. The disconnect review record

A disconnect review is one structured projection of a service
surface pausing, degrading, or going stale for the deployment. The
auth sheet, mode-change sheet, status-bar deployment cell, activity
center, support packet, admin-audit packet, and release-evidence
excerpt each render the same record without changing field names.

A surface that pauses, degrades, or staleens a service surface
without first emitting one `disconnect_review_record` is non-
conforming. A surface that emits the record but lets the
`local_continuity_statement` collapse into a generic "service
unavailable" sentence is non-conforming.

### 4.1 Required fields

- `record_kind = disconnect_review_record`.
- `disconnect_review_schema_version = 1`.
- `review_id` — opaque, stable.
- `emitted_at` — RFC 3339 UTC timestamp from a monotonic clock.
- `disconnect_event_class` — closed set (§4.3).
- `affected_service_classes[]` — non-empty array of
  `control_plane_service_class` values whose state is non-
  `healthy`. Re-export verbatim.
- `affected_capability_classes[]` — array of
  `data_plane_capability_class` values whose state is non-
  `available_local_safe`.
- `local_continuity_statement` — see §4.5.
- `paused_capabilities[]` — array of typed reviewable labels in
  product terms naming what is currently paused. Empty only when
  every affected capability still resolves to `available_local_safe`
  on the underlying packet (for example, a service-degradation that
  affects only telemetry sink does not pause user-facing
  capabilities).
- `stale_artifacts[]` — array of stale-artifact rows (§4.6).
- `reconnect_path_class` — closed set (§4.7).
- `reconnect_label` — short reviewable sentence.
- `requires_user_action` — boolean.
- `freshness_label` — short reviewable sentence in product terms.
  Required non-null when any `stale_artifacts[]` row carries a
  non-`mirror_fresh_within_window` `mirror_freshness_class`.
- `redaction_class` — one of the four standard classes.
- `export_safe` — boolean.

Optional fields: `mode_change_review_ref` (back-pointer to the
parent transition record),
`linked_continuity_packet_ref`, `linked_outage_notice_refs[]`,
`linked_transport_posture_ref`, `notes`.

### 4.2 No-silent-collapse invariants

A disconnect review never:

- collapses local-safe data-plane capability into the affected
  service list (control-plane impairment cannot zero out the
  data-plane summary; see §5.2);
- omits the local-continuity statement;
- paraphrases the reconnect path with generic copy ("try again",
  "reconnect" alone);
- silently widens authority. A disconnect cannot grant new
  capability; the schema rejects any disconnect record that pairs
  with a mode-change review whose `authority_change_guard`
  resolves to `widens_with_explicit_consent`.

### 4.3 `disconnect_event_class` (closed)

- `service_pause` — a control-plane service surface paused
  (operator pause, planned maintenance, voluntary pause).
- `service_degrade` — a control-plane service surface entered a
  reduced state (stale_cache, unavailable, mirror_only).
- `service_stale` — a service surface kept its last-known-good
  state, but the resolved freshness window has passed.
- `mirror_loss` — the mirror surface itself is unreachable.
- `offline_transition_started` — the deployment crossed into an
  offline state (`offline_grace_preserved`,
  `offline_air_gapped`, `network_disabled_by_user`,
  `deny_all_enforced`, `network_degraded_heuristic`).
- `reconnect_required` — the grace window closed and the next
  managed action requires a fresh reconnect.
- `boundary_recheck_required` — a boundary recheck (org, tenant,
  region, key mode, profile) is required before managed
  capabilities resume.

### 4.4 `affected_service_classes[]` and
`affected_capability_classes[]`

Both arrays re-export their respective vocabularies from the
deployment-summary contract (`control_plane_service_class`,
`data_plane_capability_class`). The schema enforces that
`affected_service_classes[]` is non-empty for every disconnect
event that is not `offline_transition_started` with
`mirror_offline_state_class = network_disabled_by_user` (in which
case the disconnect originates with the user, not a service).

### 4.5 `local_continuity_statement`

Required fields:

- `available_local_safe_capability_classes[]` — non-empty array of
  `data_plane_capability_class` values reporting
  `available_local_safe`. The schema enforces this list is
  non-empty whenever the underlying continuity packet retains any
  local-safe capability — a control-plane disconnect cannot zero
  out the local data-plane summary.
- `local_safe_label` — short reviewable sentence in product terms
  naming what continues on device.
- `no_silent_loss_guarantee` — boolean, const `true`.
- `unsaved_local_artifacts_safe` — boolean, const `true`.

### 4.6 `paused_capabilities[]` and `stale_artifacts[]`

`paused_capabilities[]` carries typed reviewable labels naming
managed capabilities that pause for the duration of the disconnect.

`stale_artifacts[]` rows project the trust state of mirrored or
offline-cached artifacts the disconnect leaves stale. Each row
carries:

- `artifact_class` — re-export of `artifact_class` from the
  deployment-summary contract: `updates`, `extensions`,
  `docs_pack`, `policy_bundle`, `models`.
- `mirror_freshness_class` — re-export.
- `last_refresh_at` — nullable RFC 3339 UTC timestamp.
- `freshness_label` — short reviewable sentence in product terms.
- `evidence_links[]` — opaque refs into the underlying
  mirror-snapshot, offline-bundle, or signed-manifest record that
  backs the row.

### 4.7 `reconnect_path_class` (closed)

- `wait_for_reconnect` — the deployment will retry automatically
  when the surface returns; no user action is required.
- `manual_reconnect` — the user must take an action to reconnect
  (re-open auth sheet, re-enable network).
- `mirror_refresh` — the deployment must refresh from the mirror.
- `offline_bundle_import` — the deployment must consume a signed
  offline bundle.
- `boundary_recheck_required` — an org / tenant / region / key /
  profile boundary recheck is required before reconnect can
  proceed.
- `no_action_local_continue` — the user MAY continue local-safe
  work indefinitely; reconnect is unnecessary for the current
  task.
- `escalate_to_admin` — only an org administrator can resolve the
  disconnect; the user is shown the admin path.

### 4.8 `freshness_label`

Short reviewable sentence in product terms. Required non-null
whenever any `stale_artifacts[]` row carries a non-
`mirror_fresh_within_window` `mirror_freshness_class`. The schema
enforces this so a stale artifact cannot be surfaced without a
freshness sentence.

## 5. Cross-record invariants

The schema enforces these invariants mechanically. A surface that
violates any of them is non-conforming.

1. **Local continuity is preserved.** Every
   `mode_change_review_record` carries
   `preserved_local_state.local_artifacts_safe = true`,
   `edit_save_undo_export_available = true`,
   `no_silent_loss_guarantee = true`, and
   `retained_local_capabilities[]` non-empty. Every
   `disconnect_review_record` carries
   `local_continuity_statement.no_silent_loss_guarantee = true`,
   `unsaved_local_artifacts_safe = true`, and
   `available_local_safe_capability_classes[]` non-empty whenever
   the underlying continuity packet retains any local-safe
   capability.
2. **Plane separation never collapses on disconnect.** Any
   `disconnect_review_record` with
   `disconnect_event_class ∈ { service_pause, service_degrade,
   service_stale, mirror_loss, boundary_recheck_required }` MUST
   keep `local_continuity_statement.available_local_safe_capability_classes[]`
   non-empty whenever the underlying continuity packet retains any
   local-safe capability. A control-plane impairment cannot zero
   out the local data-plane summary.
3. **Authority widening requires explicit consent.** Any
   `mode_change_review_record` whose
   `authority_change_guard.authority_change_class =
   widens_with_explicit_consent` MUST set
   `explicit_consent_recorded = true` and a non-null
   `consent_evidence_ref`. Any record whose `transition_class` is
   `sign_out`, `mirror_fallback`, `offline_transition`,
   `service_degradation`, `reconnect_required`, or
   `deployment_profile_narrow` MUST resolve
   `authority_change_class` to `narrows_only` or
   `maintains_orthogonal`.
4. **Cached authority is invalidated, not silently reused.** Any
   record whose `transition_class` is `sign_out`, `org_switch`,
   `service_degradation`, or `deployment_profile_narrow` and whose
   `from_posture.deployment_profile` carried any managed binding
   MUST list at least one `invalidated_cached_authority_row` whose
   `authority_class` matches the bound surface (managed_session,
   signed_policy_bundle, etc.) with a typed
   `invalidation_reason_class`.
5. **Reauth or retry pairs with the auth contract.** Any record
   whose `reauth_or_retry_requirements.reauth_class = required`
   MUST list at least one
   `reauth_requirement_refs[]` opaque ref into a
   `reauth_requirement_record`; the schema does not re-mint the
   reauth shape.
6. **Rollback path is always present.** Every record carries
   exactly one `rollback_or_reopen_path`. A record whose
   `rollback_class = no_rollback_after_commit` MUST set
   `rollback_safe = false` and carry a non-null
   `no_rollback_reason_label`. A record whose `rollback_class` is
   one of the rollback-available classes MUST set
   `rollback_safe = true` and `no_rollback_reason_label = null`.
7. **Disconnect-class transitions pair with a disconnect review.**
   Any record whose `transition_class` is `mirror_fallback`,
   `offline_transition`, `service_degradation`, or
   `reconnect_required` MUST set `disconnect_review_ref` to a
   non-null opaque ref. Any record that does NOT carry one of
   these transition classes MAY omit the disconnect_review_ref or
   set it to null; if non-null, the disconnect review's
   `mode_change_review_ref` MUST point back to this record.
8. **Air-gapped profiles forbid companion surfaces and online
   reconnect paths.** A record whose
   `to_posture.deployment_profile = air_gapped` MUST NOT list
   `companion_surface` in `consumer_surfaces[]`, and any paired
   disconnect review MUST NOT resolve `reconnect_path_class` to
   `wait_for_reconnect` or `manual_reconnect` — air-gapped
   reconnect resolves through `mirror_refresh`,
   `offline_bundle_import`, `boundary_recheck_required`,
   `no_action_local_continue`, or `escalate_to_admin`.
9. **Narrowing transitions cannot drop a prohibited-claim
   guardrail.** When `transition_class` is
   `deployment_profile_narrow`, `mirror_fallback`,
   `offline_transition`, or `service_degradation`, the
   `to_posture.prohibited_implied_claim_classes[]` MUST be a
   superset of the
   `from_posture.prohibited_implied_claim_classes[]`. Surfaces
   that drop a guardrail across a narrowing transition are non-
   conforming.
10. **Inspect-only actions stay inspect-only.** Every
    `export_before_change_action` and the
    `rollback_or_reopen_path.rollback_action` MUST declare
    `side_effects = ["no_side_effect_inspect_only"]` and
    `scope_class = scope_local_only`. Stacking another effect or
    widening the scope on any of those actions is non-conforming.
11. **Export discipline preserves redaction.** A record exported
    under `export_safe = true` MUST keep its `redaction_class` ≤
    the underlying continuity packet's redaction class. Exports
    that widen redaction are non-conforming.

## 6. Voluntary versus involuntary transitions

The acceptance criterion *users can tell what remains local, what
pauses, and what must be reauthorized when a deployment boundary
changes* resolves through the
`triggered_by_user_action` field plus the per-transition pairing
rules:

- Voluntary (`sign_in`, `sign_out`, `org_switch`,
  `deployment_profile_narrow`) records set
  `triggered_by_user_action = true`. Authority changes MUST be
  recorded explicitly through `authority_change_guard`. Voluntary
  narrowing transitions MUST surface at least one
  `export_before_change_action` whenever
  `preserved_local_state.unsaved_buffer_count > 0`.
- Involuntary (`mirror_fallback`, `offline_transition`,
  `service_degradation`, `reconnect_required`) records set
  `triggered_by_user_action = false`. Authority MUST resolve to
  `narrows_only` or `maintains_orthogonal`; an involuntary
  transition cannot widen authority. The disconnect review carries
  the `reconnect_path_class` so the user can return to the prior
  posture without losing local work.

## 7. Support and audit reuse posture

The acceptance criterion *review sheets are explicit enough that
support/export flows can reconstruct what changed and why* resolves
through:

- The `consumer_surfaces[]` field on the record naming exactly the
  surfaces consuming the record.
- The `from_posture` and `to_posture` summaries together with
  `invalidated_cached_authority_rows[]`,
  `reauth_or_retry_requirements`,
  `export_before_change_actions[]`, and
  `rollback_or_reopen_path` so a support packet, admin-audit
  export, or release-evidence excerpt can replay the transition
  inputs without re-deriving them.
- The disconnect review's `affected_service_classes[]`,
  `affected_capability_classes[]`,
  `paused_capabilities[]`, `stale_artifacts[]`, and
  `reconnect_path_class` so the same packet can replay the
  disconnect inputs.
- A record exported under `export_safe = true` preserves field
  names exactly. Renaming, paraphrasing, or splitting fields on
  export is non-conforming.

## 8. Adding or changing vocabulary

Adding a value to any vocabulary in this contract is **additive-
minor** and requires:

1. Updating the schema enum in
   `schemas/deployment/mode_change_review.schema.json`.
2. Updating this document.
3. Adding or updating a fixture under
   `fixtures/deployment/mode_change_cases/` exercising the new
   value.
4. Bumping the corresponding `*_schema_version` integer.

Repurposing an existing value is **breaking** and requires:

1. A new decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecation of the old value, addition of the new value through
   an additive-minor landing, and a translation pass on auth,
   mode-change, status-bar, support, admin-audit, and release-
   evidence consumers across the deprecation window.

Vocabularies that re-export from upstream seeds (deployment
profile, locality posture, residual-dependency ledger, control-
plane / data-plane state, mirror freshness, transport posture,
managed-session state, reauth reason class) follow the upstream
change rules; this contract follows in the same change.

## 9. Out of scope at this revision

- The transition handler runtime. Session migration, mirror
  failover, offline-bundle loader, network-disabled enforcement,
  deny-all enforcement, managed sign-in / sign-out / org-switch
  backends, and the actual reauth flow are owned by their
  upstream contracts.
- Final auth-sheet, account-chip, status-bar, activity-center,
  support, admin-audit, and release-evidence layout, animation,
  and accessibility wiring. The contract pins the record family;
  the rendering surfaces own their own component contracts.
- Localization-ready string tables; the contract carries
  reviewable English sentences and the localization layer is
  consumed separately.
- Pixel-perfect mode-change sheet layout and the cross-platform
  widget toolkit.
