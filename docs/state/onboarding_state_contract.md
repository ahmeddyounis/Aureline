# Portable onboarding progress, dismissals, and imported-profile history state contract

This document freezes one cross-surface state contract that every
surface which **writes**, **reads**, **exports**, **deletes**, or
**holds** a row of onboarding progress resolves through.

It pins **where each row physically lives**, **which authority owns
it**, **whether it crosses a machine boundary**, and **how a delete,
export, legal-hold, or supportability flow classifies it from the
schema alone** — so onboarding progress, dismissals, first-useful-work
milestones, imported-profile history, rollback reminders, and
remembered compatibility notices can move with the profile where
promised, stay on the device where promised, and never masquerade as
workspace source truth.

The machine-readable schema lives at:

- [`/schemas/state/onboarding_progress.schema.json`](../../schemas/state/onboarding_progress.schema.json)

Companion fixtures live under:

- [`/fixtures/state/onboarding_progress_cases/`](../../fixtures/state/onboarding_progress_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or Design-System style guide, those documents win and
this document plus its companion schema and fixtures update in the
same change. Where a downstream Start Center, first-run tour,
migration-center, rollback-reminder banner, remembered-compatibility
notice card, support-export, legal-hold review, or remembered-state
inspector mints a parallel vocabulary, this document wins and the
surface is non-conforming.

## Why freeze this now

Onboarding state is one of the easiest state classes to leak or lose:

- Tour progress that should move with the profile silently stays on
  the first device instead.
- A dismissal meant as "I've acknowledged this" vanishes when the
  user switches machines.
- First-useful-work milestones get attached to the workspace folder
  instead of the profile, so opening the same folder on a different
  machine makes the new user look like they already shipped.
- Imported-profile history becomes a silent portable payload and
  carries one account's import ledger onto a completely different
  account.
- A rollback reminder that was supposed to nag the user until they
  decided keeps firing forever because its expiry was never recorded.
- A remembered compatibility notice ("you've been shown this
  translation hint before, don't show again") gets classified as
  workspace truth and refuses to clear when the user resets
  onboarding state.

Each of those is the same underlying failure — unspecified storage
lane and unspecified delete / export / hold posture — and each one
surfaces later as a support ticket, a privacy escalation, or a
support-bundle redaction bug. This contract makes the classification
mechanical.

## Companion contracts this contract rides on

This contract does **not** re-mint vocabulary already frozen
upstream; it consumes it by reference.

- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
  and
  [`/schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json)
  — `state_authority_class` (user-authored durable truth, user-owned
  recovery state, admin or control artifact, disposable derived
  cache), `portability_class`, `location_root_id`, `state_class_id`,
  `exclusion_reason_id`, `fidelity_label`, `redaction_class`.
- [`/docs/ux/no_account_local_entry_contract.md`](../ux/no_account_local_entry_contract.md)
  and
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../schemas/ux/onboarding_portability_state.schema.json)
  — `state_portability_class`, `reset_class`, `export_class`,
  `profile_scope_class`, `recovery_binding_class`, the canonical
  state-item id namespace, and the portability manifest's twelve
  const-true invariants.
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
  — state-plane vocabulary, downgrade reasons, preserved-prior-
  artifact rules, rollback checkpoint refs, equivalence-map refs.
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
  and
  [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
  — persisted-object inventory, authority-owner vocabulary,
  schema-evolution posture, backup-before-migrate rule.
- [`/docs/governance/record_class_governance.md`](../governance/record_class_governance.md)
  and
  [`/artifacts/governance/record_class_registry.yaml`](../../artifacts/governance/record_class_registry.yaml)
  — record-class scope / retention / hold / delete / export /
  offboarding postures this contract projects onto each row.
- [`/docs/settings/schema_registry_seed.md`](../settings/schema_registry_seed.md)
  — scope, winning-source, and write-intent vocabulary reset /
  export / clear actions bind to when the item is also a setting.
- [`/docs/settings/sync_and_device_registry_seed.md`](../settings/sync_and_device_registry_seed.md)
  — sync scope bundle, device registry, and local-authoritative
  degrade contract an onboarding state item crosses when the user
  opts into sync.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  — migration-center object model rollback reminders and imported-
  profile history entries are projections of.

## Who reads this contract

- **First-run tour, Start Center, welcome-banner, onboarding-prompt,
  restore-prompt, migration-center, rollback-reminder banner, and
  remembered-compatibility notice authors** writing any code that
  mutates an onboarding state row. Every write resolves through one
  record kind here.
- **Support export, admin export, and recovery-ladder packet
  authors** so each row is classified correctly before it crosses a
  support / admin / recovery boundary.
- **Legal-hold and privacy-reduction reviewers** so the rows a
  hold freezes and the rows a privacy-reduction clear wipes are
  mechanical, not implementation-specific.
- **Remembered-state inspector, clear-data review, Project-Doctor,
  and Migration-rollback review surfaces** so every action a user
  can take on an onboarding row is classified from the schema.

## 1. Scope

- Freeze seven record kinds covering the onboarding-state rows named
  in the spec:
  1. `onboarding_progress_entry_record` — one tour step, welcome-
     tour page, guided-walkthrough step, or first-run checklist
     task progress row (started, advanced, completed, skipped,
     restarted, expired).
  2. `onboarding_dismissal_record` — one dismissal of a banner,
     card, tip, chip, notification, nudge, or onboarding prompt.
  3. `first_useful_work_milestone_record` — one observation that a
     first-useful-work milestone (first useful edit, first useful
     navigation, first save, first successful clone, first successful
     restore, first completed migration review) was reached.
  4. `imported_profile_history_entry_record` — one row in the
     imported-profile history ledger, binding one
     competitor-config-root import, one portable-profile import,
     one handoff-packet apply, or one template-or-prebuild snapshot
     apply to its outcome fidelity, rollback checkpoint, and
     equivalence-map ref.
  5. `rollback_reminder_record` — one reminder attached to a
     migration, import, or restore outcome whose rollback window
     has not yet closed.
  6. `remembered_compatibility_notice_record` — one remembered
     cross-release / cross-channel / cross-equivalence-map
     compatibility notice the user has already acknowledged (or
     explicitly declined) so it is not re-shown on every launch.
  7. `onboarding_state_bundle_record` — top-level aggregator that
     binds every row of the kinds above and declares the const-true
     invariants that make leakage, dual-homing, or hidden workspace
     writes mechanical violations.
- Freeze one closed `storage_lane` vocabulary naming where the row
  physically lives and one closed `delete_posture`, `hold_posture`,
  `support_export_posture` vocabulary for each row so delete / export /
  legal-hold / supportability flows classify it from the schema.
- Freeze separation rules between portable profile state, workspace
  state, machine-local diagnostics, and transient session state,
  including the delete / export and reset semantics the shell and
  support-export tooling MUST honour.
- Freeze the seed canonical row table (§7) binding each named
  onboarding state class to exactly one `storage_lane`, one
  `state_authority_class`, one `state_portability_class`, one
  `reset_class`, one `export_class`, one `delete_posture`, and one
  `support_export_posture`.

## 2. Out of scope

- **UI implementation of onboarding history inspection.** This
  contract freezes the row shapes and the storage lane rules; it does
  not specify the exact surfaces, copy, icons, keyboard shortcuts, or
  animations the remembered-state inspector, migration-center, or
  rollback-reminder banner render. Those live in the shell contract.
- **Transport or wire format for the portable profile package.**
  The profile-artifact and managed-sync contracts own that; this
  contract only names which record kinds ride that lane.
- **Exact retention clocks.** The record-class registry owns the
  retention owner and default trigger; this contract names which
  record-class row each kind projects onto and leaves clocks to the
  registry row.
- **Concrete on-disk encoding** for any of the rows. The eventual
  state crates' Rust types are the schema of record once they land;
  this file is the cross-tool export.

## 3. Frozen vocabulary

This contract re-exports — by reference — the vocabularies listed in
§companion-contracts and mints nothing new in those sets.

It introduces four small closed vocabularies scoped to where a row
physically lives and how it is classified from the schema.

### 3.1 `storage_lane`

Where the row physically lives on disk or in memory. Closed set:

- `portable_profile_body` — the row rides the text-based
  `*.aureprofile.json` body. It moves with the profile across
  machines and org boundaries. Examples: tour progress, welcome-
  banner dismissal, first-run-seen flag, imported-profile history
  ledger (redacted body), remembered compatibility notice for a
  profile-level translation hint.
- `portable_profile_machine_addendum` — the row rides the local-only
  addendum a portable profile exports alongside its body but never
  syncs by default. Use only when the row is a
  `portable_with_machine_addendum` state class.
- `workspace_tree` — the row rides inside the workspace tree
  (`.aureline/*`). It travels with the workspace export, not the
  profile. Examples: workspace-level first-run checklist a team
  opted into, workspace-specific dismissal of a repo-health banner.
- `machine_local_state` — the row lives under the device's
  `AURELINE_STATE` root and never crosses a machine boundary via
  a portable profile or default sync. Examples: recent-work
  metadata, restore-prompt skip registry, first-useful-work
  milestone observations (per device), decode-recovery ring
  entries, rollback reminder alarms.
- `ephemeral_session_memory` — the row lives only for the current
  session and is dropped on window close. Examples: current tour
  step pointer, in-memory restore-card dismissal, transient chip
  "remember this for today only".
- `policy_bundle_cache` — the row rides the active policy bundle
  cache and is owned by signed-bundle authority. Examples: consent
  acknowledgement log entries the policy bundle owns, forced-
  onboarding acknowledgement.
- `credential_store_metadata_only` — the row lives as metadata
  beside an OS credential-store handle. The handle itself never
  appears here; only a redacted class label. Examples: "the user
  has approved connected-provider X" markers read-only from
  onboarding.
- `support_export_scratch` — the row is never persisted; it is
  minted only inside a support / admin / recovery-ladder export and
  destroyed with the packet.

Rules (frozen):

1. **One row, one lane.** A record MUST declare exactly one
   `storage_lane`. Dual-homing ("rides both the profile body and
   the machine-local state") is non-conforming.
2. **Lane matches authority.** `portable_profile_body` and
   `portable_profile_machine_addendum` rows MUST declare
   `state_authority_class = user_authored_durable_truth`.
   `machine_local_state` rows MUST declare
   `state_authority_class` in `{user_authored_durable_truth,
   user_owned_recovery_state, disposable_derived_cache}`.
   `workspace_tree` rows MUST declare
   `state_authority_class = user_authored_durable_truth` or
   `state_authority_class = user_owned_recovery_state`.
   `policy_bundle_cache` rows MUST declare
   `state_authority_class = admin_or_control_artifact`.
   `credential_store_metadata_only` rows MUST declare
   `state_authority_class = user_owned_recovery_state`.
   `ephemeral_session_memory` rows MUST declare
   `state_authority_class = disposable_derived_cache`.
   `support_export_scratch` rows MUST declare
   `state_authority_class = user_owned_recovery_state` or
   `state_authority_class = disposable_derived_cache`.
3. **Lane matches portability class.** `portable_profile_body` ↔
   `portable_profile_state`; `machine_local_state` ↔
   `machine_local_diagnostic` or `device_scoped`;
   `policy_bundle_cache` ↔ `policy_scoped`;
   `credential_store_metadata_only` ↔ `account_scoped`;
   `ephemeral_session_memory` ↔ `ephemeral_session`. A row that
   pairs classes outside the table is a schema violation.

### 3.2 `delete_posture`

How a clear / reset / delete request on this row is classified.
Closed set:

- `clear_allowed` — a Clear / Reset action is permitted without a
  rollback checkpoint. Used for disposable-derived-cache rows and
  ephemeral-session rows.
- `clear_requires_preview` — a Clear action requires a preview of
  what will be cleared; no rollback checkpoint is mandated.
- `clear_requires_preview_and_rollback` — a Clear action requires a
  preview and a rollback checkpoint. Used for
  `user_authored_durable_truth` rows whose loss cannot be
  reconstructed.
- `clear_denied_admin_owned` — the shell denies the clear; routing
  is through the signed-bundle distribution path.
- `clear_denied_live_authority` — the shell denies the clear; the
  row refers to a live authority (credential store, approval
  ticket) whose clear goes through that authority's path.
- `clear_denied_record_class_immutable` — the row cannot be
  cleared locally (audit-only ledger); routing goes through the
  record-class registry's retention path.

Rules (frozen):

1. A `state_authority_class = admin_or_control_artifact` row MUST
   declare `delete_posture = clear_denied_admin_owned`.
2. A `state_authority_class = user_authored_durable_truth` row
   whose loss is not reconstructible MUST declare
   `delete_posture in {clear_requires_preview,
   clear_requires_preview_and_rollback}`.
3. A `state_authority_class = disposable_derived_cache` row MAY
   declare `delete_posture = clear_allowed`.
4. `credential_store_metadata_only` rows MUST declare
   `delete_posture = clear_denied_live_authority`.

### 3.3 `hold_posture`

Whether the row participates in legal-hold / retention-hold flows.
Closed set:

- `not_hold_eligible` — the row is not subject to any hold; a hold
  request that names this row is rejected by the record-class
  registry.
- `hold_eligible_per_profile` — the row can be frozen per profile;
  holds are released when the profile owner's hold lifts.
- `hold_eligible_per_account` — the row can be frozen per account;
  holds travel with the account, not the profile.
- `hold_eligible_per_policy_bundle` — the row can be frozen per
  active policy bundle; holds release when the bundle's hold
  lifts.
- `hold_handled_by_authority` — the hold is owned by a different
  authority (credential store, signed-bundle distribution, live
  provider). The onboarding-state row itself carries only a typed
  marker.

Rules (frozen):

1. An `ephemeral_session_memory` row MUST declare
   `hold_posture = not_hold_eligible`. Session-only rows cannot
   carry holds.
2. A `policy_bundle_cache` row MUST declare
   `hold_posture in {hold_eligible_per_policy_bundle,
   hold_handled_by_authority}`.
3. A `credential_store_metadata_only` row MUST declare
   `hold_posture = hold_handled_by_authority`.
4. A `portable_profile_body` row MUST declare
   `hold_posture in {not_hold_eligible, hold_eligible_per_profile,
   hold_eligible_per_account}`. Policy-scoped holds on portable
   rows ride the policy bundle, not the profile body.

### 3.4 `support_export_posture`

How the row appears in a support bundle or admin export. Closed
set, aligned with `/docs/state/profile_and_state_map.md` §6 so a
support-export surface can project directly:

- `included_by_default` — the row ships in every support / admin
  export.
- `included_metadata_only` — only the row's class label, counts,
  and timestamps ship; the row body is redacted.
- `opt_in_only` — the row ships only if the user or admin opts in
  at export time.
- `excluded_by_default` — the row is excluded by default; opt-in
  adds it but still routes through the redacted projection.
- `excluded_always` — the row never ships in a support or admin
  export.

Rules (frozen):

1. A `portable_profile_body` row MAY declare
   `support_export_posture in {included_by_default,
   included_metadata_only, opt_in_only}` but MUST NOT declare
   `excluded_always`; portable rows are inspectable on export.
2. A `credential_store_metadata_only` row MUST declare
   `support_export_posture in {excluded_always,
   included_metadata_only}`. Raw credential handles never ship.
3. A `policy_bundle_cache` row MUST declare
   `support_export_posture = included_metadata_only`.
4. An `ephemeral_session_memory` row MUST declare
   `support_export_posture in {excluded_always,
   included_metadata_only}`.

## 4. Per-record-kind field rules

### 4.1 `onboarding_progress_entry_record`

- `progress_kind` — one of `tour_step`, `welcome_tour_page`,
  `guided_walkthrough_step`, `first_run_checklist_task`,
  `migration_welcome_step`.
- `progress_state` — closed set: `not_started`, `in_progress`,
  `completed`, `skipped`, `restarted`, `expired_by_policy`.
- `attempts` — non-negative integer; number of times the step has
  been entered.
- `state_item_id_ref` — pointer into the canonical onboarding
  state-item namespace (§7).
- `first_seen_monotonic`, `last_advanced_monotonic`,
  `resolved_monotonic` (nullable) — monotonic timestamps only.
- `remembered_across_restart` — boolean; true when the row
  survives a restart. `ephemeral_session_memory` rows MUST set
  this to false; every other lane MUST set it to true.

### 4.2 `onboarding_dismissal_record`

- `dismissed_surface_family` — one of `welcome_banner`,
  `onboarding_prompt`, `tour_step`, `restore_card`,
  `import_banner`, `migration_center_card`,
  `notification_inbox_entry`, `contextual_tip`.
- `dismissal_intent` — closed set: `acknowledged_once`
  (user pressed "got it"), `dismiss_for_this_session` (closes for
  current session only), `dismiss_until_reset` (only a profile
  reset re-shows it), `dismiss_never_show_again` (user opted out
  permanently on this profile), `snoozed_until_next_launch`,
  `snoozed_until_rollback_window_closes`.
- `scope_class` — one of `per_profile`, `per_device`,
  `per_account`, `per_policy_bundle`, `per_session`.
- `dismissed_at_monotonic` — monotonic timestamp only.
- `reappears_rule` — closed set describing when the dismissed
  surface re-appears: `never`, `on_profile_reset`,
  `on_restart`, `on_policy_change`,
  `on_rollback_window_close`, `on_boundary_crossing`.

Rule: `dismissal_intent = dismiss_for_this_session` MUST pair
with `scope_class = per_session`, `reappears_rule = on_restart`,
and `storage_lane = ephemeral_session_memory`. A row that
declares "dismiss for this session" on a non-session lane is a
schema violation.

### 4.3 `first_useful_work_milestone_record`

- `milestone_kind` — closed set: `surface_first_useful_edit`,
  `surface_first_useful_navigation`, `surface_first_save`,
  `surface_first_clone`, `surface_first_restore`,
  `surface_first_migration_review`. The vocabulary is a subset of
  the measurement-plan surface vocabulary; no other values admitted.
- `measurement_surface_ref` — optional pointer to the measurement-
  plan surface row, for lanes that cross-link.
- `first_observed_monotonic` — monotonic timestamp, immutable
  once set.
- `observed_on_device_id_ref` — opaque device id handle, never a
  raw serial. Optional but required when the row rides
  `machine_local_state`.
- `recurrence_counter` — non-negative integer. Re-observations
  increment; the `first_observed_monotonic` does NOT reset.

Rule: A `first_useful_work_milestone_record` MUST ride
`storage_lane = machine_local_state` or
`storage_lane = portable_profile_body`. A milestone observation
written into `workspace_tree` is non-conforming — milestones are
user state, not workspace truth. When the row rides
`portable_profile_body`, `observed_on_device_id_ref` MAY be
present as redacted class label only (§3.4 rule 1).

### 4.4 `imported_profile_history_entry_record`

- `import_kind` — closed set: `portable_profile_import`,
  `handoff_packet_apply`, `competitor_config_root_import`,
  `template_or_prebuild_snapshot_apply`.
- `source_ref` — opaque ref to the source artifact (never a raw
  path or URL).
- `target_profile_scope_ref` — opaque profile handle on the
  destination.
- `fidelity_label_ref` — one of `exact`, `compatible`,
  `layout_only`, `manual_review` (re-exported from
  `/docs/state/migration_and_restore_playbook.md`).
- `rollback_checkpoint_ref` — nullable opaque ref. MUST be
  non-null when `fidelity_label_ref` is `compatible` or
  `manual_review`.
- `equivalence_map_ref` — nullable opaque ref; MUST be non-null
  when `fidelity_label_ref` is `compatible` or `manual_review`.
- `imported_at_monotonic` — monotonic timestamp.
- `resolved_state` — closed set: `kept`, `rolled_back`,
  `partial_apply_pending_review`, `rollback_window_open`,
  `rollback_window_closed`.
- `rollback_reminder_ref` — optional opaque ref to a paired
  `rollback_reminder_record` when the history row still has an
  open rollback window.

Rule: An `imported_profile_history_entry_record` MUST ride
`storage_lane = portable_profile_body` (the history ledger rides
with the profile, redacted) OR `storage_lane = machine_local_state`
(the local copy of a history entry waiting for the profile export
lane). Writing an imported-profile history row into
`workspace_tree` is non-conforming.

### 4.5 `rollback_reminder_record`

- `reminder_kind` — closed set: `migration_rollback_window`,
  `import_rollback_window`, `profile_restore_rollback_window`,
  `remembered_compatibility_equivalence_map_review`.
- `bound_history_ref` — opaque ref to the
  `imported_profile_history_entry_record` (or migration-center
  entry) this reminder tracks. Required.
- `rollback_window_opens_monotonic` — monotonic timestamp.
- `rollback_window_closes_monotonic` — monotonic timestamp. The
  reminder MUST have a closing time; a reminder without a close
  is non-conforming (invariant 12).
- `reminder_state` — closed set: `armed`, `snoozed_once`,
  `resolved_kept`, `resolved_rolled_back`, `window_closed_expired`.
- `paired_checkpoint_ref` — nullable opaque ref to the rollback
  checkpoint the reminder will invoke on confirm.

### 4.6 `remembered_compatibility_notice_record`

- `notice_kind` — closed set: `schema_upgrade_translation_hint`,
  `cross_channel_restore_downgrade_notice`,
  `extension_equivalence_map_review`,
  `layout_restore_layout_only_notice`,
  `workspace_trust_prompt_memo`,
  `template_or_prebuild_compatibility_memo`.
- `acknowledged_by` — closed set: `user_acknowledged`,
  `user_declined`, `user_snoozed_until_reset`,
  `policy_suppressed`.
- `equivalence_map_ref` — nullable opaque ref; required for
  `schema_upgrade_translation_hint` and
  `extension_equivalence_map_review`.
- `applies_to_source_schema_version` — optional redaction-aware
  opaque token (e.g. `portable-profile-schema-v1`).
- `first_shown_monotonic`, `last_shown_monotonic`,
  `acknowledged_monotonic` — monotonic timestamps.
- `reappears_rule` — same closed set as §4.2.

Rule: a `remembered_compatibility_notice_record` MUST NOT carry a
pointer to any live-authority handle (credential handle, approval
ticket, live provider session). The notice refers to translation
or compatibility decisions only; live authority stays behind its
authority-specific lane (invariant 5).

### 4.7 `onboarding_state_bundle_record`

- `bundle_id` — opaque handle.
- `records[]` — list of refs, at least one. Each ref points at a
  record of one of the six kinds above.
- `invariants[]` — array of twelve const-true invariant assertions
  (see §6.2).
- `minted_at` — monotonic timestamp.

## 5. Separation rules

### 5.1 Portable profile state vs. workspace state

1. **Onboarding progress is never workspace truth.** No record
   kind defined here may ride `workspace_tree` except
   `onboarding_dismissal_record` rows whose
   `dismissed_surface_family` is scoped to workspace content (e.g.
   `notification_inbox_entry` for a repo-health notice) and which
   declare `scope_class = per_profile` with an explicit narrowing
   reason in `notes`. A first-useful-work milestone in
   `workspace_tree` is always non-conforming.
2. **Workspace-hosted dismissals are rare.** The default lane for
   a dismissal is `portable_profile_body` (acknowledged once,
   moves with user) or `machine_local_state` (per-device). A
   workspace-tree lane requires an explicit narrowing reason and
   MUST still project into the workspace export contract.

### 5.2 Portable profile state vs. machine-local diagnostics

1. **Recent-work metadata stays device-local.** Every row whose
   canonical state-item id is `state_item.recent_work_metadata` or
   `state_item.restore_prompt_skipped_registry` MUST ride
   `machine_local_state`. The portable profile package never
   carries them (invariant 4).
2. **Diagnostic rings ride the redacted support bundle only.** A
   decode-recovery ring, rollback-reminder alarm log, or
   machine-local onboarding diagnostic ring rides
   `machine_local_state` with `support_export_posture in
   {included_metadata_only, excluded_by_default}`.
3. **First-useful-work milestones are device-scoped by default.**
   A first-useful-work milestone declared `portable_profile_body`
   MUST document why in the canonical row's `portability_reason`
   (§7); the default lane is `machine_local_state`.

### 5.3 Transient session state vs. durable state

1. **Session-only rows never persist.** Every
   `ephemeral_session_memory` row is dropped on window close.
   `remembered_across_restart` MUST be false.
2. **Session-only rows never export.** Every
   `ephemeral_session_memory` row declares
   `support_export_posture in {excluded_always,
   included_metadata_only}`. The support bundle never carries
   ephemeral state bodies.

### 5.4 Delete / export / reset semantics

1. **Clear data (shell).** The shell's Clear-onboarding control
   routes through every row's `delete_posture`. A Clear request
   against `clear_denied_admin_owned` rows is denied at the
   surface and routed through signed-bundle distribution; a
   request against `clear_denied_live_authority` rows is routed
   through the credential store or the provider's approval path.
2. **Reset onboarding state (profile).** Resetting onboarding
   state clears every `reset_class = resettable_per_profile` row
   whose delete posture is not denied. Rows with
   `clear_requires_preview_and_rollback` render a preview that
   names the rollback checkpoint that will be kept.
3. **Export profile.** A portable profile export includes every
   `portable_profile_body` row, with redaction applied per the
   row's `redaction_class`. `imported_profile_history_entry_record`
   rows are always redacted to class labels on export.
4. **Support bundle.** A support export includes every row per
   its `support_export_posture`. Rows with
   `support_export_posture = excluded_always` never ship, even
   with opt-in.

## 6. Top-level bundle record

Every installation or repository that ships onboarding state MUST
emit exactly one `onboarding_state_bundle_record` tying the rows
above together and declaring the invariants. The bundle is the
record that governance tooling, migration-center, rollback-reminder,
privacy-reduction review, remembered-state inspector, and
support-export surfaces read.

### 6.1 Required fields

- `record_kind = onboarding_state_bundle_record`.
- `bundle_id` — opaque.
- `records[]` — at least one; each entry is an opaque ref to one of
  the six concrete record kinds.
- `invariants[]` — twelve const-true invariant assertions (§6.2).
- `minted_at` — monotonic timestamp.

### 6.2 Required invariants (const true)

Every bundle record MUST declare every invariant below as const
`true`:

1. `one_record_one_storage_lane` — every record declares exactly
   one `storage_lane`; dual-homing is non-conforming.
2. `lane_matches_state_authority_class` — `portable_profile_body`
   rows declare `state_authority_class =
   user_authored_durable_truth`; `policy_bundle_cache` rows declare
   `admin_or_control_artifact`; `ephemeral_session_memory` rows
   declare `disposable_derived_cache`; every row matches the table
   in §3.1 rule 2.
3. `portability_class_matches_storage_lane` — the
   `storage_lane` ↔ `state_portability_class` pairing matches the
   table in §3.1 rule 3.
4. `recent_work_and_milestones_are_device_scoped_by_default` —
   every `first_useful_work_milestone_record` rides
   `machine_local_state` unless the canonical row (§7) declares a
   `portable_profile_body` exception; no milestone ever rides
   `workspace_tree`.
5. `remembered_compatibility_notice_never_binds_live_authority` —
   no `remembered_compatibility_notice_record` carries a pointer
   to a credential handle, approval ticket, or live provider
   session; only translation / compatibility memos are admitted.
6. `imported_profile_history_rides_portable_or_is_machine_scratch` —
   every `imported_profile_history_entry_record` rides
   `portable_profile_body` or `machine_local_state`; a row on any
   other lane is non-conforming.
7. `ephemeral_session_never_exports_bodies` — every
   `ephemeral_session_memory` row declares
   `support_export_posture in {excluded_always,
   included_metadata_only}` and
   `remembered_across_restart = false`.
8. `admin_owned_rows_refuse_local_clear` — every
   `state_authority_class = admin_or_control_artifact` row
   declares `delete_posture = clear_denied_admin_owned`.
9. `credential_metadata_never_exports_raw_handle` — every
   `credential_store_metadata_only` row declares
   `support_export_posture in {excluded_always,
   included_metadata_only}` and
   `delete_posture = clear_denied_live_authority`.
10. `every_row_classifies_delete_export_and_hold` — every record
    carries a non-null `delete_posture`, `support_export_posture`,
    and `hold_posture`; no row relies on "implementation default"
    for any of the three.
11. `dismissal_scope_matches_lane` — every
    `onboarding_dismissal_record` satisfies the scope / lane
    pairings in §4.2 rule; a "dismiss for this session" row on a
    non-session lane is non-conforming.
12. `every_rollback_reminder_has_close_or_resolution` — every
    `rollback_reminder_record` declares a non-null
    `rollback_window_closes_monotonic` and a `reminder_state`
    that either still has the window open or carries a resolution
    (`resolved_kept`, `resolved_rolled_back`,
    `window_closed_expired`).

A bundle that declares any invariant as `false` is non-conforming
by construction.

## 7. Canonical row table

The table below names the canonical onboarding state items this
contract governs and pins each one to one `storage_lane`, one
`state_authority_class`, one `state_portability_class`, one
`reset_class`, one `export_class`, one `delete_posture`, one
`hold_posture`, and one `support_export_posture`. Downstream
features that add new state items seat a new row here in the same
change.

| `state_item_id` | `storage_lane` | `state_authority_class` | `state_portability_class` | `reset_class` | `export_class` | `delete_posture` | `hold_posture` | `support_export_posture` |
|---|---|---|---|---|---|---|---|---|
| `state_item.tour_progress` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_requires_preview` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.welcome_tour_page_progress` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_requires_preview` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.first_run_checklist_task_progress` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_requires_preview` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.dismissal.welcome_banner` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_allowed` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.dismissal.onboarding_prompt` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_allowed` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.dismissal.restore_card_session_only` | `ephemeral_session_memory` | `disposable_derived_cache` | `ephemeral_session` | `resettable_per_device` | `not_exported_machine_local` | `clear_allowed` | `not_hold_eligible` | `excluded_always` |
| `state_item.first_useful_work_milestone` | `machine_local_state` | `user_owned_recovery_state` | `machine_local_diagnostic` | `resettable_per_device` | `in_support_bundle_redacted` | `clear_requires_preview` | `not_hold_eligible` | `included_metadata_only` |
| `state_item.imported_profile_history` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package_redacted` | `clear_requires_preview_and_rollback` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.rollback_reminder` | `machine_local_state` | `user_owned_recovery_state` | `device_scoped` | `resettable_per_device` | `not_exported_machine_local` | `clear_requires_preview` | `not_hold_eligible` | `included_metadata_only` |
| `state_item.remembered_compatibility_notice` | `portable_profile_body` | `user_authored_durable_truth` | `portable_profile_state` | `resettable_per_profile` | `in_portable_profile_package` | `clear_allowed` | `hold_eligible_per_profile` | `included_metadata_only` |
| `state_item.consent_acknowledgement_onboarding` | `policy_bundle_cache` | `admin_or_control_artifact` | `policy_scoped` | `resettable_by_policy` | `in_support_bundle_redacted` | `clear_denied_admin_owned` | `hold_eligible_per_policy_bundle` | `included_metadata_only` |
| `state_item.connected_provider_approval_ticket` | `credential_store_metadata_only` | `user_owned_recovery_state` | `account_scoped` | `resettable_per_account` | `not_exported_machine_local` | `clear_denied_live_authority` | `hold_handled_by_authority` | `excluded_always` |

Rules:

1. A surface that reads or writes a canonical row MUST honour every
   column verbatim. A "but in this surface it's different"
   treatment is non-conforming.
2. A new state item is additive-minor: seat a new row, bump
   `onboarding_progress_schema_version`. Repurposing an existing
   row is breaking and requires a new decision row.
3. A row MUST declare a pairing that also satisfies the M00-211
   `state_portability_class` ↔ `profile_scope_class` ↔
   `export_class` consistency matrix and the portable-profile map
   `state_authority_class` ↔ `portability_class` matrix.

## 8. Projection onto delete / export / hold / support flows

The per-row `delete_posture`, `support_export_posture`, and
`hold_posture` are the only fields those flows read. Every clear-
data, export, legal-hold, or supportability flow MUST classify a
row from the schema alone; no flow may fall back on file-path
heuristics or hard-coded "I know this one is special" branches.

- **Clear-data review** projects `delete_posture` onto the clear
  preview and either renders the preview, renders the rollback
  checkpoint, or refuses the action.
- **Support export** projects `support_export_posture` onto the
  bundle contents. `excluded_always` rows never appear even on
  opt-in; `included_metadata_only` rows appear only as class
  labels / counts / timestamps.
- **Admin export** projects the same fields; `policy_bundle_cache`
  rows carry their admin narrative (who acknowledged what when).
- **Legal-hold / retention-hold** projects `hold_posture` onto
  the hold registrar. A `not_hold_eligible` row rejects the hold;
  a `hold_handled_by_authority` row forwards the hold to the named
  authority.
- **Remembered-state inspector** renders the row inline using the
  `state_class_id`, the `storage_lane`, and the three postures;
  no parallel pane-id or inspector-action surface may be minted
  (the profile-and-state-map §5 reservations own that).

## 9. Cross-cutting rules

1. **Imported-profile history and onboarding progress move with
   the profile where promised.** Rows whose canonical row (§7)
   declares `storage_lane = portable_profile_body` ride the
   portable profile package body. Redaction applies per the row's
   `redaction_class`. A row that claims to be portable but lives
   on `machine_local_state` is a schema violation.
2. **Machine-local exclusions are named and visible.** Rows that
   stay on the device declare `storage_lane in {machine_local_state,
   ephemeral_session_memory, policy_bundle_cache,
   credential_store_metadata_only, support_export_scratch}` and
   carry a `portability_reason` (reused from M00-211) explaining
   why. A "silently not exported" row is non-conforming.
3. **Delete / export / legal-hold / supportability are classifiable
   from the schema.** A clear-data review, export flow, legal-hold
   registrar, or support-export tool that has to read the row's
   content to decide how to route the action is non-conforming.
   The three posture fields plus the authority class are sufficient.
4. **No hidden workspace writes.** No onboarding progress,
   dismissal, first-useful-work milestone, imported-profile history
   row, rollback reminder, or remembered compatibility notice may
   be silently stored under the workspace tree. The workspace is
   source truth; onboarding state lives on the profile, the device,
   the session, the policy bundle, or beside the credential store.

## 10. Change-discipline rules

Additions are additive-minor and bump
`onboarding_progress_schema_version`:

- A new `progress_kind`, `progress_state`, `dismissed_surface_family`,
  `dismissal_intent`, `scope_class`, `reappears_rule`,
  `milestone_kind`, `import_kind`, `reminder_kind`,
  `reminder_state`, `notice_kind`, or `acknowledged_by` value.
- A new `storage_lane`, `delete_posture`, `hold_posture`, or
  `support_export_posture` value, when paired with a decision-row
  update naming the authority-class rule it obeys.
- A new canonical row in §7.

Breaking changes require a new decision row:

- Reclassifying a canonical row's `storage_lane`,
  `state_authority_class`, `state_portability_class`,
  `delete_posture`, `hold_posture`, or `support_export_posture`.
- Removing a record kind.
- Adding a new record kind (requires a new design review before
  the schema accepts it).

## 11. Acceptance-criteria mapping

- **Imported-profile history and onboarding progress can move with
  the profile where promised.** §7 canonical table rows for
  `state_item.tour_progress`,
  `state_item.welcome_tour_page_progress`,
  `state_item.first_run_checklist_task_progress`,
  `state_item.imported_profile_history`, and
  `state_item.remembered_compatibility_notice` ride
  `portable_profile_body` with the correct
  `state_portability_class` / `export_class` pairing; fixtures
  under `/fixtures/state/onboarding_progress_cases/` exercise
  both the plain and redacted export paths.
- **Machine-local exclusions are named and visible rather than
  silently divergent.** §5.1–§5.3 separation rules, §3.1 rule 3
  portability-class ↔ storage-lane table, and the §7 rows for
  `state_item.first_useful_work_milestone`,
  `state_item.rollback_reminder`, and
  `state_item.dismissal.restore_card_session_only` name each
  machine-local / device-scoped / session-only row explicitly, each
  row carries a `portability_reason`, and the
  remembered-state-inspector reservation in
  `profile_and_state_map.md` §5 reads the same row shape.
- **Delete / export / legal-hold / supportability flows can
  classify this state correctly from the schema.** §8 spells out
  which posture field each flow reads; §3.2–§3.4 freezes the
  closed sets; §6.2 invariants 8–10 make the classification
  mechanical; fixtures for every record kind exercise the
  postures a reviewer and a flow read.

## 12. Reference rows

- PRD §12.4 and §12.4.1 — portable profile artifact rules.
- PRD §22.6.1 — signed policy bundle, offline entitlement, and
  admin-distribution lifecycle.
- TAD §21.10 — profile sync, snapshot, backup, and restore
  architecture.
- TAD Appendix F — configuration and state map.
- ADR-0001 — identity modes and workspace trust.
- ADR-0007 — secret broker, credential handle, trust store, and
  redaction.
- ADR-0008 — settings definition and effective-configuration
  resolver.
- `/docs/state/profile_and_state_map.md` — portable profile and
  state-map seed.
- `/docs/state/migration_and_restore_playbook.md` — fidelity labels,
  downgrade reasons, preserved-artifact rules.
- `/docs/state/state_object_inventory.md` — persisted-object
  inventory, authority owner, schema-evolution posture, corruption
  routing.
- `/docs/ux/no_account_local_entry_contract.md` — onboarding-
  portability state contract this document projects onto storage
  lanes.
- `/artifacts/governance/record_class_registry.yaml` — record-class
  rows each posture projects onto.
