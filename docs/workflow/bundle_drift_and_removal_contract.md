# Bundle drift banner, local-override merge, and remove-bundle safety contract

This document freezes the cross-surface drift, merge, and
remove-bundle review model every **workflow-bundle banner,
inspector, project-doctor recommendation, support-export
disclosure, CLI / headless drift listing, and remove / disable
flow** renders after a `workflow_bundle_manifest_record` has been
applied. The goal is that bundle adoption never degenerates into
silent divergence or destructive reset-to-bundle behavior — local
changes, certification drift, and safe removal remain
**reviewable, attributable, and recoverable**, not a hidden
trade between the user and the bundle author.

The companion machine-readable schema lives at:

- [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json)

The companion fixtures live under:

- [`/fixtures/workflow/bundle_drift_cases/`](../../fixtures/workflow/bundle_drift_cases/)

This contract is normative for the drift-row record, the
merge / resolve action set, the claim-narrowing block, and the
remove-bundle review block. Where it disagrees with the PRD,
TAD, TDD, UI / UX spec, design-system style guide, or milestone
document anchors quoted in §13, those sources win and this
document plus its companion schema and fixtures update in the
same change. Where a downstream Start Center, project-doctor,
support-export, CLI / headless, browse-bundles, bundle detail
panel, or removal / rollback surface mints a parallel drift-state,
merge-action, ownership, safe-to-remove, claim-narrowing, or
remove-review verb, this contract wins and the surface is
non-conforming.

This contract mints **no** new bundle, identity, source, status,
packaging, signer, channel-relation, dependency-marker,
component-kind, evidence-link, lifecycle, successor, removal-
surface, or rollback-surface vocabulary. It re-exports — by
reference — the closed sets frozen in
[`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
(§3.1–§3.15, §4–§9), the change-preview vocabulary frozen in
[`/docs/workflow/bundle_change_review_contract.md`](./bundle_change_review_contract.md)
(§3.1–§3.11), the appearance-checkpoint vocabulary frozen in
[`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md),
the Start Center bundle-card / detail-page vocabulary frozen in
[`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md),
and the template-and-prebuild availability-narrowing vocabulary
in
[`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).

## Who reads this contract

- **Bundle drift banner authors** rendering the post-install
  drift indicator on the bundle detail panel, the Start Center
  bundle card, the project-doctor row, the support-export
  surface, and the CLI / headless `aureline bundle drift`
  surface. The banner reads one or more
  `bundle_drift_row_record`s and projects them onto each surface
  family without minting parallel vocabulary.
- **Reviewers** comparing what diverged between the applied
  bundle revision and the workspace's current state. They expect
  to read every drift state (version drift, missing artifact,
  local override, unmanaged addition, mirror mismatch, evidence
  stale, signer continuity break, policy-pinned revision lag),
  every typed merge / resolve action (`resolve.keep_local`,
  `resolve.adopt_bundle`, `resolve.compare`,
  `resolve.rebase_to_bundle`, `resolve.ignore_this_drift`), and
  every claim-narrowing implication in the same record.
- **Remove-bundle and rollback-review authors** distinguishing
  bundle-owned versus user-owned assets, naming safe-to-remove
  classes, retaining local overrides, and citing the recovery /
  rollback handle that pairs with the prior change-preview
  record's `rollback_checkpoint_linkage`.
- **Project-doctor, support-export, and certification authors**
  flagging that bundle drift may narrow a previously-asserted
  `support_class`, suspend a certification target, or recommend
  a successor bundle.
- **CLI / headless / API authors** later in M1 / M2 who must
  read the same drift truth a UI surface renders. The boundary
  is one row record kind across surfaces.

## 1. Scope

- Freeze one `bundle_drift_row_record` per
  `(target_bundle_identity, drift_axis, drift_subject_id)`
  triple where the workspace's current state diverges from the
  bundle's applied revision (or the bundle's canonical revision
  on its declared channel). Each row is the reviewable unit
  every drift surface renders against.
- Freeze the **drift-state taxonomy** (§3.1) covering
  `bundle_version_drift`, `missing_artifact`, `local_override`,
  `unmanaged_addition`, `mirror_mismatch`, `evidence_stale`,
  `signer_continuity_break`, `policy_pinned_revision_lag`, and
  `drift_state_unknown` so reviewers see a closed vocabulary
  rather than ad-hoc "out of sync" copy.
- Freeze the **drift-axis taxonomy** (§3.2) so each row resolves
  through the thirteen `bundle_component_kind` axes plus the
  four review-local axes (`settings_or_token`,
  `trust_or_permission`, `compatibility_or_runtime`,
  `side_effect`) re-exported from
  [`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  §3.3. The drift surface and the change-preview surface MUST
  speak the same axis names.
- Freeze the **resolve-action set** (§3.5, §5) — exactly five
  user-visible verbs (`resolve.keep_local`,
  `resolve.adopt_bundle`, `resolve.compare`,
  `resolve.rebase_to_bundle`, `resolve.ignore_this_drift`) — so
  the merge surface never mints `Reset`, `Force-apply`, or
  `Wipe local` parallel verbs. Adopt-bundle and rebase-to-bundle
  always route through a new `bundle_change_preview_record`;
  drift never authorises durable apply on its own.
- Freeze the **resolve-action blocker classes** (§3.7) so a
  `resolve.adopt_bundle` blocked by policy, by compatibility, by
  signer continuity, or by retained-local-override safety
  renders a typed `resolve_blocker_class` rather than free-form
  copy.
- Freeze the **asset-ownership taxonomy** (§3.9) for the
  remove-bundle review so `bundle_owned`, `user_owned`,
  `shared_user_overlay_on_bundle`, and
  `mixed_unknown_provenance` are stable across surfaces.
- Freeze the **safe-to-remove taxonomy** (§3.10) covering
  `safe_to_remove_no_user_data`,
  `safe_to_remove_user_overlay_preserved`,
  `review_required_user_data_co_resident`,
  `not_safe_to_remove_user_owned`, and
  `not_safe_to_remove_policy_locked`. Removing a bundle MUST
  emit one of these classes per asset entry.
- Freeze the **retained-local-override taxonomy** (§3.11)
  covering `override_retained_in_user_scope`,
  `override_retained_in_profile_scope`,
  `override_retained_in_workspace_scope`,
  `override_inlined_to_user_authored_record`, and
  `override_dropped_with_user_consent`.
- Freeze the **remove-bundle review state lifecycle** (§3.8) so
  `remove_review_minted_pending_classification`,
  `remove_review_classified_ready_to_confirm`,
  `remove_review_blocked_user_data_co_resident`,
  `remove_review_blocked_policy_locked`,
  `remove_review_cancelled`, `remove_review_confirmed`,
  `remove_review_completed_succeeded`, and
  `remove_review_completed_partial_user_data_retained` are the
  only states a remove-bundle review record carries.
- Freeze the **claim-narrowing taxonomy** (§3.12) so drift can
  narrow `support_class`, suspend a certification target, or
  recommend a successor bundle — but never re-promote a row's
  authority on its own.
- Freeze the **provenance and bundle-history record classes**
  (§3.14) so every drift row, every resolve action, and every
  remove-bundle review is later auditable as a typed lifecycle
  event rather than only as a transient diff list.
- Freeze the cross-cutting **drift-surface invariants** (§9)
  every surface family (Start Center bundle card, bundle detail
  panel, project-doctor row, support-export, CLI / headless,
  browse-bundles, removal / rollback surface) MUST satisfy.

## 2. Out of scope

- The bundle **manifest** itself.
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  owns identity, component inventory, source / status / class
  linkage, and lifecycle. This contract reads that record by
  reference.
- The bundle **change-preview** record.
  [`/docs/workflow/bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  owns the install / update / downgrade / re-apply review sheet,
  the six review-sheet action ids, and the
  `rollback_checkpoint_linkage` block. This contract pins the
  drift indicator and the merge / remove review only — every
  durable apply still rides through a new change-preview record.
- The **execution engine** that performs install, activation,
  scaffold expansion, settings writes, extension uninstall, or
  filesystem deletion. This contract pins only the disclosure
  shape, the resolve action's typed verb, the asset ownership
  classification, and the recovery / rollback handle reference;
  the engine itself remains owned by the environment-starter
  summary contract and the recovery / local-history contracts.
- The **rollback engine** that materialises a checkpoint or
  restores from one. This contract reserves only the ref to a
  prior `rollback_checkpoint_linkage` (or to the appearance
  checkpoint contract); storage, scope, and atomicity mechanics
  live with the recovery / local-history and the
  appearance-checkpoint contracts.
- **Package uninstall or cleanup implementation.** This contract
  pins the disclosure model and the safe-to-remove classification
  every removal flow renders before any durable side effect; it
  does not specify how the underlying extension manager,
  filesystem, or settings store evicts records.
- Final user-facing **copy / microcopy**. The shell-interaction-
  safety contract and the UX style guide own the exact strings;
  this contract pins the closed sets the copy resolves against.
- **Telemetry wire format**. Drift, merge, and remove-bundle
  measurement is owned by the support-export and onboarding-
  measurement plans; this contract only tags records with the
  drift-row id and the bundle id + revision those plans cite.

## 3. Frozen vocabulary (re-exported) and new closed sets

### 3.0 Re-exported vocabulary

This contract re-exports — by reference, never by redefinition —
the following closed sets:

- `bundle_class`, `bundle_source_class`, `bundle_status_class`,
  `bundle_signer_source_class`, `signer_continuity_class`,
  `signature_class`, `support_class`,
  `bundle_channel_relation_class`,
  `mirror_or_offline_packaging_posture`,
  `bundle_dependency_marker_kind`, `bundle_component_kind`,
  `evidence_link_kind`, `lifecycle_state_class`,
  `successor_recommendation_class`, `evidence_age_class`,
  `retest_needed_posture`, `removal_surface_kind`,
  `rollback_surface_kind`, `bundle_id`, `bundle_revision` —
  [`workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  §3.1–§3.15.
- `change_kind`, `change_axis`, `settings_or_token_axis`,
  `settings_value_class`, `trust_or_permission_axis`,
  `trust_or_permission_state_class`, `compatibility_axis`,
  `compatibility_state_class`, `side_effect_class`,
  `side_effect_scope_class`,
  `rollback_checkpoint_linkage_class`, `rollback_path_class`,
  `non_reversibility_justification_class`,
  `rollback_restorable_axis`, `disabled_reason_code`,
  `action_rendered_state` —
  [`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  §3.2–§3.11 and
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).
- `appearance_axis`, `checkpoint_class`,
  `checkpoint_scope_class`, `appearance_apply_state` —
  [`appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md).
- `bundle_card_surface_family`, `bundle_evidence_badge_id`,
  `bundle_detail_page_section_id`, `bundle_detail_page_zone` —
  [`start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md).
- `policy_notice_class`, `availability_narrowing_class`,
  `bypass_path_id` —
  [`template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).

This contract introduces **fifteen** small vocabularies scoped
to the drift-row record, the resolve-action set, the
remove-bundle review block, and the claim-narrowing block. Each
is closed; adding a value is additive-minor and bumps
`bundle_drift_row_schema_version`, repurposing is breaking and
opens a decision row.

### 3.1 `drift_state_class`

What kind of drift the row records against the applied bundle's
declared state. The set is closed:

- `bundle_version_drift` — the workspace is pinned to a bundle
  revision that no longer matches the canonical revision on the
  bundle's declared channel (typically because the channel
  advanced, was demoted, or was rotated). The applied revision
  itself is intact; only the bundle's freshness diverges.
- `missing_artifact` — a component the applied bundle revision
  declared (extension, profile preset, surface preset, task /
  launch / debug recipe, template / scaffold ref, docs / tour /
  glossary pack, migration mapping, certification target,
  evidence link) is no longer present locally. The artifact
  was removed, never installed, lost on disk, evicted by mirror
  policy, or demoted by upstream.
- `local_override` — a setting / token, profile preset, surface
  preset, task / launch / debug recipe, or other component the
  bundle would otherwise own has been changed locally so that
  the workspace's effective value diverges from the bundle's
  declared value. The override is the user's; the bundle did
  not author it.
- `unmanaged_addition` — the workspace contains an item the
  bundle does not own (an extra extension, a user-authored
  task, a user-added scaffold file, a setting outside the
  bundle's inventory). The addition is informational — it is
  not a divergence in the bundle's sense — but the row exists
  so the user can see it and so remove-bundle reviews can
  classify it as `user_owned` (§3.9).
- `mirror_mismatch` — the workspace's mirror copy of a bundle
  artifact differs from the canonical mirror or origin
  artifact at the bound revision. Typical causes are an
  air-gapped mirror that has not refreshed, an admin override
  that pinned an older artifact, or a corrupted local cache.
- `evidence_stale` — the evidence backing the bundle's
  certification, compatibility, benchmark, or migration claims
  has aged past its declared freshness window
  (`evidence_age_class = aging_within_window` or
  `stale_past_window`), independent of whether the bundle
  itself or its components have changed.
- `signer_continuity_break` — the signer continuity class of
  the bundle revision has been invalidated since apply
  (signer rotated unexpectedly, signature review window
  expired, key revocation observed). The bundle remains
  installed but its signature posture diverges from the
  state at apply.
- `policy_pinned_revision_lag` — an organisation or fleet
  policy is pinning the workspace to an older bundle revision
  than the bundle's canonical channel currently advertises;
  drift is policy-intentional and resolve actions narrow
  accordingly.
- `drift_state_unknown` — drift was detected but the kind
  could not be classified (offline, expired registry, lost
  signer continuity, mirror unreachable). The row renders
  with a typed `disabled_reason_code` so the reviewer sees
  why classification was deferred.

Rules (frozen):

1. Every drift row names exactly one `drift_state_class`. A
   surface that emits a tenth class (`untrusted_drift`,
   `auto_repair_pending`, etc.) is non-conforming.
2. `bundle_version_drift` MUST cite both the
   `applied_revision` and the `canonical_revision` so the
   delta is reviewable.
3. `local_override` MUST cite a
   `preserved_local_override_summary_ref` and an
   `asset_ownership_class` of either
   `shared_user_overlay_on_bundle` or `user_owned` (§3.9).
4. `unmanaged_addition` MUST set
   `asset_ownership_class = user_owned` (§3.9). A bundle that
   tries to claim an `unmanaged_addition` as `bundle_owned`
   is non-conforming.
5. `mirror_mismatch` MUST cite a
   `mirror_or_offline_packaging_posture` (`live_or_mirror`,
   `mirror_only`, `signed_offline_bundle`); the row cannot
   surface against `live_origin_only` posture.
6. `evidence_stale` MUST cite an `evidence_age_class` of
   `aging_within_window`, `stale_past_window`, or
   `age_unknown`, and MUST resolve a typed
   `claim_narrowing_class` (§3.12).
7. `signer_continuity_break` MUST cite the new
   `signer_continuity_class` and a typed
   `disabled_reason_code` of `signature_review_required` on
   the matching `resolve.adopt_bundle` action.
8. `policy_pinned_revision_lag` MUST cite a
   `policy_pinned_setting`-class source ref and MUST disable
   `resolve.adopt_bundle` and `resolve.rebase_to_bundle`
   with `disabled_reason_code = policy_narrowed_admin` or
   `policy_narrowed_fleet`.
9. `drift_state_unknown` MUST cite a typed
   `disabled_reason_code` (`network_unreachable`,
   `signature_review_required`, `mirror_only_cached_subset`,
   `policy_narrowed_review_pending`,
   `evidence_aged_review_required`).

### 3.2 `drift_axis`

Which axis the drift is observed on. The set is closed and
mirrors the `change_axis` set re-exported from
[`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
§3.3:

- The thirteen component axes from
  `bundle_component_kind` (§3.8 of the manifest contract):
  `extension_set`, `profile_preset`, `surface_preset`,
  `task_recipe`, `launch_recipe`, `debug_recipe`,
  `template_or_scaffold_ref`, `docs_pack`, `tour_pack`,
  `glossary_pack`, `migration_mapping`,
  `certification_target`, `evidence_link`.
- The four review-local axes:
  - `settings_or_token` — token / setting drift the bundle
    would otherwise have owned at the user, profile,
    workspace, or policy scope.
  - `trust_or_permission` — workspace-trust state, extension
    capability, network egress, filesystem scope, subprocess
    scope, task / run / test / debug scope, external
    provider grant, managed-cloud identity grant, signer
    continuity carryover, signature review carryover,
    policy lock state.
  - `compatibility_or_runtime` — Aureline runtime range,
    release-channel compatibility, extension-set
    compatibility, template compatibility, archetype
    compatibility, toolchain / runtime class, mirror /
    offline packaging, evidence freshness window.
  - `side_effect` — durable effects (shell command
    invocation, external provider request, managed-cloud
    identity use, telemetry emission, scaffold write) that
    diverged from what the apply-time side-effect envelope
    enumerated.

Rules (frozen):

1. Every drift row names exactly one `drift_axis`.
2. The `drift_axis` MUST resolve through the same closed set
   the change-preview record uses; a surface that mints a
   parallel axis (`runtime_environment_drift`,
   `appearance_state_drift`) is non-conforming.
3. `appearance_token_overlay` and `appearance_theme_package`
   drift on `settings_or_token` MUST cite an
   `appearance_checkpoint_ref` so the appearance-session
   contract owns visual rollback. The drift row never owns
   the visual rollback handle directly.

### 3.3 `drift_subject_kind`

What the row is the drift of. The set is closed:

- `bundle_component_record` — the row tracks drift of one
  component the manifest declared.
- `settings_or_token_path` — the row tracks drift of a
  settings / token path identified by an opaque path id.
- `trust_or_permission_state` — the row tracks drift of a
  trust / permission state the bundle would otherwise own.
- `compatibility_state` — the row tracks drift of a
  compatibility / runtime axis.
- `side_effect_envelope_entry` — the row tracks drift of a
  durable effect declared at apply.
- `bundle_revision_pin` — the row tracks drift of the bundle
  revision pin itself (`bundle_version_drift`,
  `policy_pinned_revision_lag`,
  `signer_continuity_break`,
  `mirror_mismatch` at the manifest level).
- `evidence_link_record` — the row tracks drift of an
  evidence link's freshness or availability.

Rules (frozen):

1. Every row names exactly one `drift_subject_kind`.
2. `drift_subject_kind` MUST be consistent with `drift_axis`:
   `bundle_component_record` resolves through one of the
   thirteen component axes; `settings_or_token_path`
   resolves through `settings_or_token`;
   `trust_or_permission_state` through
   `trust_or_permission`; `compatibility_state` through
   `compatibility_or_runtime`; `side_effect_envelope_entry`
   through `side_effect`; `evidence_link_record` through
   `evidence_link`; `bundle_revision_pin` through any axis
   when the drift is at the revision-pin level
   (typically reported on at least one summary row alongside
   the per-component rows).

### 3.4 `drift_severity_class`

How load-bearing the drift is for review. The set is closed:

- `informational_no_narrowing` — drift exists but does not
  narrow `support_class`, suspend a certification target,
  recommend a successor, or block any user action. Typical
  for `unmanaged_addition` and benign `local_override`.
- `narrowing_review_recommended` — drift narrows the
  bundle's projected `support_class` by at least one rung,
  suspends a certification target, or queues a successor
  recommendation. Resolve actions remain enabled but the
  row carries a typed `claim_narrowing_class` (§3.12).
- `narrowing_review_required` — drift cannot be left
  unreviewed: the bundle's authority is materially reduced
  (signer continuity break, policy-pinned lag,
  evidence-stale on a certified row) and the surface MUST
  render the row above the fold on the bundle detail panel
  and project-doctor.
- `severity_unknown_review_required` — severity could not
  be classified (offline, expired registry, mirror
  unreachable). The row carries a typed
  `disabled_reason_code` so the reviewer sees why.

Rules (frozen):

1. Every row names exactly one `drift_severity_class`.
2. `narrowing_review_recommended` and
   `narrowing_review_required` MUST cite a
   `claim_narrowing_class` other than
   `no_narrowing_informational` (§3.12).
3. `severity_unknown_review_required` MUST cite a typed
   `disabled_reason_code`.
4. A `local_override` whose effective value belongs to the
   user (the bundle owns the path but the user has chosen
   to author it) is `informational_no_narrowing` unless the
   override widens trust / permission, in which case it is
   `narrowing_review_required` and the row MUST cite the
   `trust_or_permission_change_entry` it implies.

### 3.5 `resolve_action_id`

Closed set of merge / resolve actions. The set is exactly:

- `resolve.keep_local` — leave the workspace's current
  state alone; record that the user explicitly chose the
  local value. The drift row transitions to
  `drift_row_resolved_keep_local` (§3.15).
- `resolve.adopt_bundle` — accept the bundle's value for
  this row. Adopt-bundle MUST mint a new
  `bundle_change_preview_record` (re-apply or partial-apply
  intent) and MUST NOT durably write any state on its own.
  The drift row transitions to
  `drift_row_resolved_adopt_bundle` only after the new
  preview's `preview_apply_succeeded` outcome.
- `resolve.compare` — open a side-by-side view of the
  workspace's current value against the bundle's declared
  value. The action's `destination_ref` resolves to a
  compare-report ref shared with the change-preview
  contract's `review.compare`.
- `resolve.rebase_to_bundle` — re-apply the bundle's
  declared inventory in scope of the current drift,
  preserving local overrides routed through the
  `retained_local_override_class` set (§3.11) and routing
  any visual axis through the appearance-checkpoint
  contract. Like `resolve.adopt_bundle`,
  rebase-to-bundle MUST mint a new
  `bundle_change_preview_record` (re-apply or
  partial-apply intent) and never bypasses the review
  sheet.
- `resolve.ignore_this_drift` — record that the user
  explicitly chose to ignore this row. The row transitions
  to `drift_row_resolved_ignore` and is suppressed from
  banner surfaces but remains visible on the bundle detail
  panel and the support-export so the audit trail is
  preserved.

Rules (frozen):

1. Every drift row emits all five actions in
   `resolve_actions`, each with a typed
   `action_rendered_state` (§3.6 below) and a
   `keyboard_reachable = true`. Omitting an action is
   non-conforming; a surface that needs the action hidden
   uses `rendered_state = hidden_not_applicable` (typically
   only `resolve.adopt_bundle` and
   `resolve.rebase_to_bundle` on an `unmanaged_addition`
   row, since adopting / rebasing a user-authored addition
   is meaningless).
2. `resolve.adopt_bundle` and `resolve.rebase_to_bundle`
   MUST resolve their `destination_ref` to a new
   `bundle_change_preview_record` so the durable write
   rides through the change-preview contract. A surface
   that durably writes from a drift row directly is
   non-conforming.
3. `visible_disabled` actions MUST cite a typed
   `resolve_blocker_class` (§3.7) and a
   `disabled_reason_code` re-exported from the
   change-preview schema's `disabled_reason_code` set.

### 3.6 `resolve_action_rendered_state`

Re-exported from
[`bundle_change_review_contract.md`](./bundle_change_review_contract.md)
§3.8 (`action_rendered_state`):

- `enabled`
- `visible_disabled`
- `hidden_not_applicable`
- `preflight_pending`

Rules (frozen):

1. The set re-exported here is identical; this contract
   does not extend it.
2. `visible_disabled` MUST cite a typed
   `resolve_blocker_class` (§3.7) and a
   `disabled_reason_code`.

### 3.7 `resolve_blocker_class`

Closed set of reasons a resolve action is blocked. Each
value pairs with a `disabled_reason_code` on the action
record:

- `policy_pinned_blocks_adopt` — paired
  `disabled_reason_code = policy_narrowed_admin` or
  `policy_narrowed_fleet`.
- `signer_continuity_break_blocks_adopt` — paired
  `disabled_reason_code = signature_review_required`.
- `compatibility_blocks_rebase` — paired
  `disabled_reason_code = target_runtime_unavailable`,
  `rebuild_required`, or
  `mirror_only_cached_subset`.
- `retained_override_safety_blocks_rebase` — paired
  `disabled_reason_code =
  permission_grant_review_required` or
  `non_reversible_review_required`. Used when rebasing
  would silently overwrite a `user_owned` or
  `shared_user_overlay_on_bundle` asset without an
  explicit retain decision.
- `network_or_mirror_unreachable_blocks_compare` — paired
  `disabled_reason_code = network_unreachable` or
  `mirror_only_cached_subset`. Used when the canonical
  bundle revision cannot be fetched for a side-by-side
  view.
- `evidence_stale_blocks_adopt` — paired
  `disabled_reason_code =
  evidence_aged_review_required`. Used when adoption
  would cement a stale-evidence row as the active
  baseline.
- `non_reversible_review_required_blocks_adopt` — paired
  `disabled_reason_code =
  non_reversible_review_required`. Used when the
  underlying change-preview record would be
  non-reversible-with-justification and a fresh decision
  row is needed.

Rules (frozen):

1. Every `visible_disabled` action names exactly one
   `resolve_blocker_class`. A surface that emits a free-text
   blocker reason is non-conforming.
2. Adding a new blocker class requires a paired
   `disabled_reason_code` from the change-preview schema's
   `disabled_reason_code` enum; a blocker class without a
   paired code is non-conforming.

### 3.8 `remove_bundle_review_state_class`

Closed lifecycle of the remove-bundle review record:

- `remove_review_minted_pending_classification`
- `remove_review_classified_ready_to_confirm`
- `remove_review_blocked_user_data_co_resident`
- `remove_review_blocked_policy_locked`
- `remove_review_cancelled`
- `remove_review_confirmed`
- `remove_review_completed_succeeded`
- `remove_review_completed_partial_user_data_retained`

Rules (frozen):

1. Every remove-bundle review record names exactly one
   `remove_bundle_review_state_class`.
2. `remove_review_blocked_user_data_co_resident` MUST cite
   at least one `removable_asset_record` whose
   `safe_to_remove_class` is
   `review_required_user_data_co_resident` or
   `not_safe_to_remove_user_owned` (§3.10).
3. `remove_review_blocked_policy_locked` MUST cite at least
   one removable asset whose `safe_to_remove_class` is
   `not_safe_to_remove_policy_locked`.
4. `remove_review_completed_partial_user_data_retained`
   MUST cite at least one
   `retained_local_override_record` (§3.11).

### 3.9 `asset_ownership_class`

Closed set of asset-ownership classifications used by both
drift rows and remove-bundle reviews:

- `bundle_owned` — the asset was authored or installed by
  the bundle and has not been modified locally.
- `user_owned` — the asset was authored or modified by the
  user, even if it lives at a path the bundle's inventory
  recognises. `unmanaged_addition` rows always resolve
  here.
- `shared_user_overlay_on_bundle` — the bundle owns the
  artifact's identity but the user has overlaid one or more
  fields. Typical for `local_override` rows and for
  appearance / token overlays.
- `mixed_unknown_provenance` — provenance cannot be
  determined (no manifest record, no user-authored record,
  no migration mapping). The row MUST be reviewed before
  any removal action can confirm.

Rules (frozen):

1. Every drift row and every removable asset record names
   exactly one `asset_ownership_class`.
2. `mixed_unknown_provenance` MUST cite a typed
   `disabled_reason_code` of
   `policy_narrowed_review_pending` or
   `signature_review_required`, and MUST disable
   `resolve.rebase_to_bundle` with
   `resolve_blocker_class =
   retained_override_safety_blocks_rebase`.
3. `unmanaged_addition` always resolves to `user_owned`;
   `bundle_version_drift` and
   `policy_pinned_revision_lag` always resolve to
   `bundle_owned` (the drift is on the revision pin, not
   on user content).
4. A surface that displays an asset's ownership without the
   class (or that paraphrases the class as "yours" /
   "ours" copy without the closed value) is
   non-conforming.

### 3.10 `safe_to_remove_class`

Closed set of safe-to-remove classifications used by the
remove-bundle review:

- `safe_to_remove_no_user_data` — the asset is
  `bundle_owned` with no user overlay; removing it
  restores the pre-apply state covered by the apply-time
  rollback checkpoint.
- `safe_to_remove_user_overlay_preserved` — the asset is
  `shared_user_overlay_on_bundle`; removing the bundle
  removes the bundle's contribution but the user overlay
  is preserved per the matching
  `retained_local_override_class` (§3.11).
- `review_required_user_data_co_resident` — the asset is
  `bundle_owned` but user data lives alongside it (for
  example, scaffold output where the user has authored
  files). Removal MUST surface the co-resident user data
  in the typed retained-override list before it can
  confirm.
- `not_safe_to_remove_user_owned` — the asset is
  `user_owned`; removal MUST NOT touch it. The remove-
  bundle review record MUST omit the asset from the
  durable removal set and surface it on the typed
  retained-override list.
- `not_safe_to_remove_policy_locked` — an organisation or
  fleet policy locks the asset against removal. Remove-
  bundle review transitions to
  `remove_review_blocked_policy_locked` (§3.8).

Rules (frozen):

1. Every removable-asset entry on a remove-bundle review
   record names exactly one `safe_to_remove_class`.
2. `not_safe_to_remove_user_owned` MUST cite a
   `retained_local_override_class` (§3.11).
3. `safe_to_remove_user_overlay_preserved` MUST cite the
   matching `retained_local_override_class`.
4. A remove-bundle review whose every entry is
   `safe_to_remove_no_user_data` MAY transition directly
   to `remove_review_classified_ready_to_confirm` without
   a co-resident block.

### 3.11 `retained_local_override_class`

Closed set describing how a local override survives a
bundle removal or rebase:

- `override_retained_in_user_scope` — the override is
  preserved at the user-scope settings store, surviving
  bundle removal entirely.
- `override_retained_in_profile_scope` — preserved at the
  profile-scope settings store.
- `override_retained_in_workspace_scope` — preserved at
  the workspace-scope settings store.
- `override_inlined_to_user_authored_record` — the
  override is materialised into a user-authored bundle or
  a user-authored settings file so it survives without a
  bundle owner.
- `override_dropped_with_user_consent` — the user
  explicitly consented to drop the override on remove /
  rebase. The remove-bundle review record cites a
  `decision_row_ref` for the consent.

Rules (frozen):

1. Every retained-override record names exactly one
   `retained_local_override_class`.
2. `override_dropped_with_user_consent` MUST cite a
   `decision_row_ref` in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
3. `override_inlined_to_user_authored_record` MUST cite
   the target user-authored record (typically a
   `local_draft_bundle` manifest or a user-scope
   settings file) by opaque ref.

### 3.12 `claim_narrowing_class`

Closed set describing how the drift narrows the bundle's
projected claims:

- `no_narrowing_informational` — drift is informational;
  `support_class`, certification targets, and successor
  recommendations are unchanged.
- `narrows_support_class_one_rung` — projection narrows
  by one rung (for example
  `officially_supported → community_supported`). MUST
  cite the before / after `support_class` pair.
- `narrows_certification_target_pending_retest` —
  drift suspends a certification target pending retest;
  the bundle's `bundle_status_class` MUST narrow to
  `certified_retest_pending` or `retest_needed`.
- `breaks_certification_target_recall` — drift breaks a
  certification target so the row is recalled until a new
  manifest revision lands.
- `narrows_to_imported_pending_review` — drift on an
  imported-user bundle narrows the row back to
  `imported_pending_review`; ingress review must run
  again.
- `pending_review_narrowing_unknown` — narrowing could
  not be determined (offline, expired registry); MUST
  cite a typed `disabled_reason_code`.

Rules (frozen):

1. Every drift row names exactly one
   `claim_narrowing_class`.
2. Drift never re-promotes a row's authority on its own;
   adoption / rebase rides through a new
   `bundle_change_preview_record`.
3. `breaks_certification_target_recall` and
   `narrows_to_imported_pending_review` MUST cite a
   typed `successor_bundle_suggestion_class` (§3.13)
   other than `no_successor_drift_review`.

### 3.13 `successor_bundle_suggestion_class`

Closed set describing how a successor bundle is suggested
in response to drift:

- `inherits_manifest_successor_recommendation` — the
  drift row inherits the manifest's
  `successor_recommendation` block (§3.11 of the manifest
  contract) verbatim.
- `successor_recommended_via_drift_review` — drift
  surfaces a successor that the manifest did not name;
  the suggestion lives on the drift row only and MUST
  cite a `decision_row_ref` so it never silently
  promotes a community successor to certified language.
- `no_successor_drift_review` — drift does not recommend
  a successor (either because none exists or because the
  drift is informational).

Rules (frozen):

1. Every drift row names exactly one
   `successor_bundle_suggestion_class`.
2. `successor_recommended_via_drift_review` MUST cite a
   stable successor `bundle_id` and `bundle_revision`,
   and MUST cite a `decision_row_ref`.
3. The drift row never widens the
   `successor_recommendation_class` re-exported from the
   manifest; if the manifest says
   `no_successor`, the drift row may only emit
   `no_successor_drift_review` or
   `successor_recommended_via_drift_review` (with the
   decision row).

### 3.14 `provenance_record_class`

Closed set describing how the drift row, the resolve
action, and the remove-bundle review are preserved as
audit lifecycle events rather than only as a transient
diff list:

- `bundle_history_event_recorded` — the row, action, or
  remove review is appended to the bundle's
  per-bundle history log under
  `artifacts/qe/workflow_bundle_ids.yaml`'s
  bundle-history register (referenced by opaque ref
  here).
- `support_export_audit_link` — the row, action, or
  remove review is referenced from a support-export
  packet so a later support engagement can read the
  same id.
- `decision_row_link` — the row, action, or remove
  review is linked to a decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  (typically required for
  `successor_recommended_via_drift_review`,
  `override_dropped_with_user_consent`, and any
  `non_reversible_review_required_blocks_adopt`
  resolution).
- `provenance_unknown_review_required` — provenance
  could not be recorded (offline, expired registry,
  mirror unreachable); MUST cite a typed
  `disabled_reason_code`.

Rules (frozen):

1. Every drift row carries at least one
   `provenance_record_class` entry.
2. `decision_row_link` MUST cite a stable
   `decision_row_ref`.
3. `provenance_unknown_review_required` MUST NOT be the
   only entry once the row transitions out of
   `drift_row_open_pending_review`; resolution always
   pairs with a `bundle_history_event_recorded` or a
   `support_export_audit_link`.

### 3.15 `drift_row_state_class`

Closed lifecycle of the drift row record:

- `drift_row_open_pending_review`
- `drift_row_resolved_keep_local`
- `drift_row_resolved_adopt_bundle`
- `drift_row_resolved_rebase_to_bundle`
- `drift_row_resolved_ignore`
- `drift_row_superseded_by_newer_drift`
- `drift_row_superseded_by_remove_bundle_review`
- `drift_row_blocked_review_required`

Rules (frozen):

1. Every row names exactly one `drift_row_state_class`.
2. `drift_row_resolved_adopt_bundle` and
   `drift_row_resolved_rebase_to_bundle` MUST cite the
   `bundle_change_preview_record` id whose
   `preview_apply_succeeded` outcome resolved the row.
3. `drift_row_superseded_by_remove_bundle_review` MUST
   cite the remove-bundle review record id that
   superseded the row.
4. `drift_row_superseded_by_newer_drift` MUST cite the
   newer row's id; the original row is preserved for
   audit but is no longer surfaced on banner surfaces.

## 4. Drift-row record shape

Every drift row emits one `bundle_drift_row_record` with the
following required slots (see
[schema](../../schemas/workflow/bundle_drift_row.schema.json)
for exact shapes):

- `record_kind = bundle_drift_row_record`.
- `bundle_drift_row_schema_version`.
- `drift_row_id` — opaque, stable across resolve transitions.
- `minted_at` — monotonic timestamp.
- `target_bundle` — `bundle_identity_ref` re-exported from the
  change-preview schema.
- `applied_revision` — opaque ref to the bundle revision
  currently applied in the workspace.
- `canonical_revision_ref` — opaque ref to the bundle's
  canonical revision on its declared channel; null only when
  the registry is unreachable and `drift_state_class =
  drift_state_unknown`.
- `applied_change_preview_ref` — opaque ref to the
  `bundle_change_preview_record` that originally applied the
  bundle (so the drift row can read the apply-time rollback
  linkage by reference).
- `drift_state_class` (§3.1).
- `drift_axis` (§3.2).
- `drift_subject_kind` (§3.3).
- `drift_subject_id` — opaque id of the subject (component id,
  settings path id, evidence link id, revision pin id, etc.).
- `before_value_class` and `after_value_class` — re-exported
  `settings_value_class` for `settings_or_token` axis;
  re-exported `trust_or_permission_state_class` for
  `trust_or_permission`; re-exported `compatibility_state_class`
  for `compatibility_or_runtime`; otherwise opaque
  `before_revision_ref` / `after_revision_ref`. The reviewer
  sees who owns the value, never the literal payload.
- `drift_severity_class` (§3.4).
- `asset_ownership_class` (§3.9).
- `claim_narrowing_class` (§3.12).
- `successor_bundle_suggestion` block — citing
  `successor_bundle_suggestion_class` (§3.13), the successor
  identity ref when applicable, and a `decision_row_ref` when
  required by §3.13 rule 2.
- `provenance_records[]` — non-empty list of
  `provenance_record_class` entries (§3.14).
- `resolve_actions[]` — exactly five
  `resolve_action_record`s (§3.5) with typed
  `action_rendered_state` (§3.6) and, when disabled, typed
  `resolve_blocker_class` (§3.7) plus a `disabled_reason_code`.
- `change_source_refs[]` — at least one
  `change_source_ref` re-exported from the change-preview
  schema so the user can drill from the drift row back to a
  manifest field, component record, evidence link,
  dependency marker, lifecycle block, appearance / theme-
  import report, imported-environment record, policy-pinned
  record, or previous-revision manifest.
- `drift_summary_ref` — opaque ref to a reviewer-facing
  summary (≤ 256 graphemes, redaction-aware).
- `preserved_local_override_summary_ref` — required when
  `drift_state_class = local_override` or `mirror_mismatch`
  with a user overlay.
- `appearance_checkpoint_ref` — required when `drift_axis =
  settings_or_token` and the
  `settings_or_token_axis` is `appearance_token_overlay` or
  `appearance_theme_package`.
- `policy_context` — re-exported policy-context block.
- `redaction_class` — re-exported redaction class.
- `drift_row_state_class` (§3.15).
- `superseded_by_drift_row_id` — required when
  `drift_row_state_class = drift_row_superseded_by_newer_drift`.
- `superseded_by_remove_bundle_review_id` — required when
  `drift_row_state_class =
  drift_row_superseded_by_remove_bundle_review`.
- `resolved_via_change_preview_ref` — required when
  `drift_row_state_class` is
  `drift_row_resolved_adopt_bundle` or
  `drift_row_resolved_rebase_to_bundle`.
- `remove_bundle_review` — optional embedded block (§6) when
  the row is part of a remove-bundle review packet.

Cross-cutting row rules (frozen):

1. Every row carries `keyboard_reachable = true` on every
   action and on the row record itself; non-conforming if
   false.
2. Drift rows are append-only across the same
   `drift_row_id`: a re-evaluation that produces the same
   subject MUST emit the same id with an updated
   `drift_row_state_class`. Removing a row across re-mint is
   breaking.
3. `drift_state_class`, `drift_axis`, and
   `drift_subject_kind` are stable on a given
   `drift_row_id`. A surface that mutates any of these
   in-place is non-conforming; a state change of that
   shape mints a new row and supersedes the old one
   through `drift_row_superseded_by_newer_drift`.

## 5. Resolve-action set

Every drift row emits exactly five resolve actions:

- `resolve.keep_local` — leave the workspace's value
  alone; transitions the row to
  `drift_row_resolved_keep_local`. The action's
  `destination_ref` resolves to the user's typed
  acknowledgement record.
- `resolve.adopt_bundle` — opens a
  `bundle_change_preview_record` (re-apply or
  partial-apply intent) targeting only this row's
  subject. The drift row remains
  `drift_row_open_pending_review` until the new preview's
  `preview_apply_succeeded` outcome.
- `resolve.compare` — opens a side-by-side view
  resolving the workspace's current value against the
  bundle's declared value. The action's
  `destination_ref` resolves to a compare-report ref
  shared with the change-preview contract's
  `review.compare`.
- `resolve.rebase_to_bundle` — opens a
  `bundle_change_preview_record` (re-apply or
  partial-apply intent) targeting the row's subject and
  any sibling rows the rebase scope covers. Local
  overrides preserved by `retained_local_override_class`
  (§3.11) are routed through the change-preview's
  `preserved_local` change kind. The drift row remains
  `drift_row_open_pending_review` until the new preview's
  outcome.
- `resolve.ignore_this_drift` — transitions the row to
  `drift_row_resolved_ignore`. The row is suppressed from
  banner surfaces but remains visible on the bundle
  detail panel and the support-export.

Rules (frozen):

1. Every action's `keyboard_reachable` is `const true`.
2. Action ids are stable: surface families that need
   richer verbs (Reset, Force-apply, Wipe local, Restore,
   Revert) project onto these five action ids rather
   than minting parallel verbs.
3. Banner surfaces (Start Center bundle card,
   project-doctor) MAY collapse the action set behind a
   single "Review drift" affordance, but the underlying
   record MUST emit all five actions with their typed
   states so the bundle detail panel and CLI / headless
   surfaces speak the same vocabulary.
4. `resolve.adopt_bundle` and `resolve.rebase_to_bundle`
   MUST resolve `destination_ref` to a new
   `bundle_change_preview_record`. Rebase and adopt are
   the only paths that mint durable writes, and both ride
   through the change-preview review sheet.

## 6. Remove-bundle review block

Every remove-bundle flow emits one
`remove_bundle_review_record` (embedded on the
`bundle_drift_row_record` when the drift surface routes the
removal review, or stand-alone when the bundle detail panel
mints removal directly from a non-drifted bundle row).

### 6.1 Required fields

- `remove_bundle_review_id` — opaque, stable across review
  transitions.
- `target_bundle` — `bundle_identity_ref`.
- `removable_assets[]` — non-empty list of
  `removable_asset_record`s (§6.2).
- `retained_local_overrides[]` — list of
  `retained_local_override_record`s (§6.3).
- `recovery_link` — typed reference to the apply-time
  `rollback_checkpoint_linkage` from the bundle's last
  `bundle_change_preview_record`. The remove-bundle
  review never mints its own rollback handle; recovery
  rides through the change-preview's linkage.
- `bundle_history_provenance_ref` — typed reference to
  the bundle-history event that records the removal so
  the row is later auditable as a lifecycle event rather
  than a vanished diff.
- `remove_bundle_review_state_class` (§3.8).
- `policy_context`.
- `redaction_class`.
- `keyboard_reachable = true`.

### 6.2 `removable_asset_record`

- `asset_id` — opaque id.
- `asset_subject_kind` (§3.3).
- `drift_axis` (§3.2).
- `asset_ownership_class` (§3.9).
- `safe_to_remove_class` (§3.10).
- `asset_summary_ref` — opaque ref (≤ 256 graphemes).
- `change_source_refs[]` — at least one re-exported
  `change_source_ref`.
- `co_resident_user_data_summary_ref` — required when
  `safe_to_remove_class =
  review_required_user_data_co_resident`.
- `policy_lock_summary_ref` — required when
  `safe_to_remove_class = not_safe_to_remove_policy_locked`.
- `disclosure_required = true` — non-conforming if false.

### 6.3 `retained_local_override_record`

- `override_id` — opaque id.
- `retained_local_override_class` (§3.11).
- `target_subject_kind` (§3.3).
- `drift_axis` (§3.2).
- `target_user_authored_record_ref` — required when
  `retained_local_override_class =
  override_inlined_to_user_authored_record`.
- `decision_row_ref` — required when
  `retained_local_override_class =
  override_dropped_with_user_consent`.
- `override_summary_ref` — opaque ref (≤ 256 graphemes).

### 6.4 Remove-bundle rules (frozen)

1. **Removing a bundle never deletes user-owned assets.**
   `not_safe_to_remove_user_owned` entries MUST be excluded
   from the durable removal set and surfaced on the
   retained-override list. A surface that "cleans up" a
   user-owned asset because the bundle is being removed is
   non-conforming.
2. **Removing a bundle never widens trust authority.**
   Removing a bundle MUST NOT silently re-grant or revoke
   workspace trust, extension activation capability,
   external provider grants, or managed-cloud identity
   grants without a typed `trust_or_permission_change_entry`
   on the matching change-preview record.
3. **Recovery rides through the apply-time checkpoint.**
   The remove-bundle review's `recovery_link` MUST resolve
   to a prior `rollback_checkpoint_linkage` on the bundle's
   last applied change-preview. The review does not mint a
   new rollback handle; if no prior reversible linkage
   exists, removal MUST cite a
   `non_reversibility_justification_class` re-exported from
   the change-preview schema (§3.10 of the change-preview
   contract) and a `decision_row_ref`.
4. **Co-resident user data blocks removal until reviewed.**
   Any `removable_asset_record` whose `safe_to_remove_class`
   is `review_required_user_data_co_resident` transitions
   the review to
   `remove_review_blocked_user_data_co_resident` until the
   user explicitly classifies the co-resident asset.
5. **Policy-locked assets block removal until policy
   resolves.** `not_safe_to_remove_policy_locked`
   transitions the review to
   `remove_review_blocked_policy_locked`.
6. **Drift narrows certification claims; removal does not
   resurrect them.** Removing a bundle never widens any
   claim authority. A successor bundle is reached by
   navigating to the successor explicitly through the
   manifest's `successor_recommendation` or the drift
   row's `successor_bundle_suggestion`.

## 7. Claim narrowing and successor recommendation

Every drift row emits exactly one `claim_narrowing_class`
(§3.12) and exactly one `successor_bundle_suggestion` block
(§3.13).

Rules (frozen):

1. **Drift never widens authority.** A
   `claim_narrowing_class` value that widens
   `support_class`, restores a certification target, or
   re-promotes a community / imported row to certified
   language is non-conforming. Re-promotion rides through a
   new manifest revision and a new
   `bundle_change_preview_record`.
2. **`narrows_support_class_one_rung` is the only rung-
   level narrowing.** Multi-rung narrowing requires either
   `narrows_certification_target_pending_retest` or
   `breaks_certification_target_recall`.
3. **Successor suggestions do not overwrite manifest
   recommendations.** The drift row's
   `successor_bundle_suggestion_class =
   inherits_manifest_successor_recommendation` re-uses the
   manifest's block verbatim; a drift-mint of a different
   successor MUST cite a `decision_row_ref` and resolve as
   `successor_recommended_via_drift_review`.
4. **Claim narrowing flows through `support_class` and
   `bundle_status_class` projections.** The drift row's
   narrowing MUST mirror the same projection the manifest
   contract's §6 projects on Start Center, diagnostics, and
   claim surfaces. A surface that renders a different
   narrowing on a different family is non-conforming.

## 8. Provenance and audit lifecycle

Every drift row, every resolve action, and every
remove-bundle review record names at least one
`provenance_record_class` entry (§3.14) so the row is later
auditable as a typed lifecycle event rather than only as a
local diff list.

Rules (frozen):

1. The drift row's `drift_row_id` is **stable**: re-
   evaluation that observes the same subject MUST emit the
   same id with an updated `drift_row_state_class`. A
   surface that mints a new id on each scan is
   non-conforming.
2. Resolution is recorded by transitioning
   `drift_row_state_class` (§3.15); the prior state is
   preserved through the bundle-history event referenced
   by `bundle_history_event_recorded`. The drift surface
   MAY collapse resolved rows under a "resolved" disclosure
   but MUST NOT delete them from the support-export view.
3. Remove-bundle review records are append-only against
   `remove_bundle_review_id`: a re-mint of the same review
   on the same bundle MUST cite the prior id (typically
   for `remove_review_completed_partial_user_data_retained`
   followed by a re-confirmation on the retained set).

## 9. Surface invariants (cross-cutting)

1. **One drift row, many surfaces.** Start Center bundle
   card, bundle detail panel, project-doctor row, support-
   export, CLI / headless `aureline bundle drift`,
   browse-bundles, and any later API all read the same
   `drift_row_id`. A surface that mints an alternate id is
   non-conforming.
2. **Five resolve action ids, every surface.** The five
   resolve action ids are stable across families. A
   surface that uses `Reset`, `Force-apply`, `Wipe local`,
   `Restore`, or `Revert` as parallel verbs is
   non-conforming.
3. **Drift never durably writes on its own.** Adopt-bundle
   and rebase-to-bundle MUST mint a new
   `bundle_change_preview_record` and route through the
   change-preview contract's review sheet. A surface that
   adopts or rebases without a preview is non-conforming.
4. **Drift never silently resets user state.** Resolving a
   `local_override` row MUST preserve the override unless
   the user explicitly chooses
   `resolve.adopt_bundle` / `resolve.rebase_to_bundle` and
   confirms on the resulting preview. Surfaces that
   collapse a "Reset to bundle" affordance into a single
   one-click verb are non-conforming.
5. **Remove-bundle never deletes user-created artifacts
   without an explicit reviewed list.** Every removable
   asset whose ownership resolves to `user_owned` or
   `mixed_unknown_provenance` MUST appear on the
   retained-override list before the review can transition
   to `remove_review_completed_succeeded`.
6. **Drift narrows; it does not promote.** Drift can
   narrow `support_class`, suspend a certification target,
   or recommend a successor; it MUST NOT widen authority,
   re-promote a community / imported row to certified
   language, or restore a recalled certification target.
7. **Drift, merge, and removal preserve provenance.**
   Every row, every resolve, and every remove-bundle
   review names at least one typed
   `provenance_record_class`. A surface that drops the
   audit linkage to make a row "less noisy" is
   non-conforming.
8. **Lifecycle visibility post-resolve.** After a row
   transitions to a resolved state, the bundle detail
   panel, project-doctor row, support-export, and
   removal / rollback surfaces continue to project the
   row. Hiding a resolved row from support-export because
   the banner cleared is non-conforming.

## 10. Worked examples

Each example has a companion fixture under
[`/fixtures/workflow/bundle_drift_cases/`](../../fixtures/workflow/bundle_drift_cases/).
Every fixture is YAML and validates against
[`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json).

### 10.1 Bundle version drift on a certified launch bundle

`drift_state_class = bundle_version_drift`,
`drift_axis = extension_set`,
`asset_ownership_class = bundle_owned`,
`drift_severity_class = narrowing_review_recommended`,
`claim_narrowing_class = narrows_support_class_one_rung`,
`successor_bundle_suggestion_class =
inherits_manifest_successor_recommendation`. Target bundle
is the certified TypeScript web-app launch bundle from
[`fixtures/workflow/bundles/launch_bundle_typescript_web_app.yaml`](../../fixtures/workflow/bundles/launch_bundle_typescript_web_app.yaml);
the workspace is pinned to revision 1 while the channel
advanced to revision 2. Resolve actions: `compare`,
`adopt_bundle`, and `rebase_to_bundle` enabled (each routes
through a new change-preview record); `keep_local` and
`ignore_this_drift` enabled. See
[`bundle_version_drift_certified_launch.yaml`](../../fixtures/workflow/bundle_drift_cases/bundle_version_drift_certified_launch.yaml).

### 10.2 Missing artifact (extension uninstalled by user)

`drift_state_class = missing_artifact`,
`drift_axis = extension_set`,
`asset_ownership_class = bundle_owned`,
`drift_severity_class = narrowing_review_recommended`,
`claim_narrowing_class =
narrows_certification_target_pending_retest`. The
TypeScript language server extension was uninstalled
locally; the certification target binding requires it to
project replacement-grade language. See
[`missing_artifact_extension_uninstalled.yaml`](../../fixtures/workflow/bundle_drift_cases/missing_artifact_extension_uninstalled.yaml).

### 10.3 Local override on an editor setting

`drift_state_class = local_override`,
`drift_axis = settings_or_token`,
`asset_ownership_class = shared_user_overlay_on_bundle`,
`drift_severity_class = informational_no_narrowing`,
`claim_narrowing_class = no_narrowing_informational`. The
user changed `editor.format_on_save` from the bundle's
`true` to `false`; the bundle owns the path but the user's
overlay is preserved through
`override_retained_in_workspace_scope`. Resolve actions:
all five enabled. See
[`local_override_editor_setting.yaml`](../../fixtures/workflow/bundle_drift_cases/local_override_editor_setting.yaml).

### 10.4 Unmanaged addition (user-installed extra extension)

`drift_state_class = unmanaged_addition`,
`drift_axis = extension_set`,
`asset_ownership_class = user_owned`,
`drift_severity_class = informational_no_narrowing`,
`claim_narrowing_class = no_narrowing_informational`.
`resolve.adopt_bundle` and `resolve.rebase_to_bundle` are
`hidden_not_applicable` (adopting or rebasing a user-
authored addition is meaningless); the row exists so the
remove-bundle review can later classify the extension as
`user_owned`. See
[`unmanaged_addition_user_extension.yaml`](../../fixtures/workflow/bundle_drift_cases/unmanaged_addition_user_extension.yaml).

### 10.5 Mirror mismatch on signed-offline-bundle posture

`drift_state_class = mirror_mismatch`,
`drift_axis = compatibility_or_runtime`,
`asset_ownership_class = bundle_owned`,
`drift_severity_class = narrowing_review_required`,
`claim_narrowing_class = narrows_support_class_one_rung`,
`mirror_or_offline_packaging_posture =
signed_offline_bundle`. The local mirror copy of the
extension manifest disagrees with the canonical mirror at
the bound revision; `resolve.compare` is
`visible_disabled` with
`resolve_blocker_class =
network_or_mirror_unreachable_blocks_compare`. See
[`mirror_mismatch_signed_offline.yaml`](../../fixtures/workflow/bundle_drift_cases/mirror_mismatch_signed_offline.yaml).

### 10.6 Evidence stale narrowing certified-current to retest-pending

`drift_state_class = evidence_stale`,
`drift_axis = evidence_link`,
`asset_ownership_class = bundle_owned`,
`drift_severity_class = narrowing_review_required`,
`claim_narrowing_class =
narrows_certification_target_pending_retest`. The
benchmark evidence backing the certification target has
aged past its declared freshness window; the bundle's
`bundle_status_class` narrows from `certified_current` to
`certified_retest_pending` and the drift row binds the
retest. See
[`evidence_stale_certification_retest_pending.yaml`](../../fixtures/workflow/bundle_drift_cases/evidence_stale_certification_retest_pending.yaml).

### 10.7 Remove-bundle review with retained user overlays

`drift_state_class = bundle_version_drift` (precipitating
review), with an embedded `remove_bundle_review` block
whose state is
`remove_review_completed_partial_user_data_retained`.
Removable assets enumerate `safe_to_remove_no_user_data`
extensions and `safe_to_remove_user_overlay_preserved`
profile presets, plus a
`review_required_user_data_co_resident` scaffold output
that the user resolved by selecting
`override_inlined_to_user_authored_record`. Recovery rides
through the apply-time
`single_attributable_workspace_checkpoint`. See
[`remove_bundle_review_certified_with_overlays.yaml`](../../fixtures/workflow/bundle_drift_cases/remove_bundle_review_certified_with_overlays.yaml).

### 10.8 Remove-bundle review on a local-draft bundle

Stand-alone `remove_bundle_review` record with no
precipitating drift row (the user is removing a
local-draft bundle they authored). Removable assets are
all `safe_to_remove_user_overlay_preserved` or
`not_safe_to_remove_user_owned`. The review preserves the
user-authored scaffolds through
`override_retained_in_workspace_scope` and cites the
non-reversibility justification
`local_draft_no_prior_state` on the recovery link. See
[`remove_bundle_review_local_draft.yaml`](../../fixtures/workflow/bundle_drift_cases/remove_bundle_review_local_draft.yaml).

A `manifest.yaml` index lives alongside the fixtures and maps
every fixture file to its `drift_state_class`, the closed
sets it exercises, and the rules it validates.

## 11. Acceptance mapping

- **Bundle drift never forces an implicit reset of user-
  owned settings, files, or layout state.** §3.5 (resolve
  actions), §3.9 (asset ownership), §3.11 (retained local
  overrides), §5 (resolve-action set rules), and §9.4 (no
  silent reset) together freeze the no-implicit-reset
  invariant. Fixtures §10.3 (local override),
  §10.4 (unmanaged addition), and §10.7 (remove with
  retained overlays) exercise the invariant across drift
  and removal flows.
- **Users can inspect which differences are informational
  versus which narrow support or certification claims.**
  §3.4 (drift severity), §3.12 (claim narrowing), and §7
  (claim-narrowing rules) freeze the
  informational-vs-narrowing distinction. Fixtures §10.1
  (version drift narrowing one rung), §10.2 (missing
  artifact suspending certification), and §10.4
  (informational unmanaged addition) exercise the
  distinction.
- **Removing a bundle cannot imply deletion of user-
  created artifacts without an explicit reviewed list.**
  §3.9 (asset ownership), §3.10 (safe-to-remove classes),
  §3.11 (retained local overrides), §6 (remove-bundle
  review block), and §9.5 (remove-bundle never deletes
  user artifacts silently) together freeze the explicit-
  reviewed-list invariant. Fixtures §10.7 and §10.8
  exercise the invariant on certified and local-draft
  bundles.
- **Bundle drift can later be audited as a lifecycle event
  rather than only as a local diff list.** §3.14
  (provenance record classes), §3.15 (drift-row state
  lifecycle), §8 (provenance and audit lifecycle), and
  §9.7 (drift / merge / removal preserve provenance)
  freeze the audit-event invariant. Every fixture cites
  at least one `provenance_record_class` entry.

## 12. Changing this contract

- **Additive-minor** changes (new `drift_state_class`,
  new `drift_axis` value re-exported from upstream, new
  `drift_subject_kind`, new `drift_severity_class`, new
  `resolve_action_id`, new `resolve_blocker_class`, new
  `remove_bundle_review_state_class`, new
  `asset_ownership_class`, new `safe_to_remove_class`,
  new `retained_local_override_class`, new
  `claim_narrowing_class`, new
  `successor_bundle_suggestion_class`, new
  `provenance_record_class`, new `drift_row_state_class`)
  land here, in
  [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json),
  and in at least one fixture under
  [`/fixtures/workflow/bundle_drift_cases/`](../../fixtures/workflow/bundle_drift_cases/)
  in the same change. Adding a value bumps
  `bundle_drift_row_schema_version`. Each new value
  cites the motivating drift state, fixture, or surface
  family.
- **Repurposing** an existing vocabulary value, an
  action id, or a linkage class is breaking and requires
  a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (workflow-bundle object
  model, change-preview contract, appearance-checkpoint
  contract, Start Center bundle surfaces,
  template-and-prebuild contract) happen at source and
  this contract re-exports by reference; it MUST NOT
  shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any
  disagreement with the quotations in §13; this contract
  and its schema plus fixtures update in the same change.

## 13. Source anchors

- `.t2/docs/Aureline_PRD.md:254` — devcontainer
  compatibility, workspace templates, and optional
  prebuild snapshots are part of the remote story from
  day one (drift surfaces inherit the same disclosure
  posture).
- `.t2/docs/Aureline_PRD.md:1259` — remote workspaces
  should accept repo-defined devcontainer metadata and
  optional prebuild snapshots so environment setup is
  reproducible and accelerable (drift detection runs on
  the same metadata).
- `.t2/docs/Aureline_PRD.md:2328` — intelligent project
  scaffolding and generation: starter templates and
  agentic setup for new services / apps / modules using
  team standards (drift narrows team-standard claims when
  evidence ages or local overrides land).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` — §6.9
  templates, starters, and prebuilds: source class,
  support class, runtime / toolchain, freshness, setup
  actions, always-available bypass path, side-effect
  envelope (drift narrows source / support claim
  language).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346` — §17.7
  scaffolding, generation, and template health: signal
  classes (`live`, `cached`, `policy_evaluated`,
  `not_checked`) — drift severity mirrors the same
  evidence-freshness signals.
- `.t2/docs/Aureline_Milestones_Document.md:3787` —
  environment-capsule schema draft, workspace-template
  seed, and prebuild-metadata baseline (drift records
  reference the same capsule identity).

## 14. Linked artifacts

- Workflow-bundle manifest, component inventory, and
  source-class contract (source of truth for bundle
  identity, component inventory, source / status / class
  linkage, lifecycle, and the
  `removal_surface_kind` /
  `rollback_surface_kind` projections drift rows
  re-export):
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  and
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- Bundle install / update review, change preview, and
  rollback-checkpoint contract (source of truth for
  `change_kind`, `change_axis`, the six review-sheet
  action ids, the `rollback_checkpoint_linkage` block,
  and the `disabled_reason_code` set drift rows
  re-export):
  [`/docs/workflow/bundle_change_review_contract.md`](./bundle_change_review_contract.md)
  and
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).
- Workflow-bundle fixtures (the manifests drift rows
  compare against):
  [`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/).
- Change-preview fixtures (the apply-time records drift
  rows reference):
  [`/fixtures/workflow/bundle_review_cases/`](../../fixtures/workflow/bundle_review_cases/).
- Appearance import, token-overlay, and checkpoint
  contract (source of truth for the appearance
  checkpoint refs drift rows on
  `appearance_token_overlay` /
  `appearance_theme_package` cite):
  [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md)
  and
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json).
- Start Center bundle card, bundle detail page, and
  evidence-badge contract (source of truth for the
  drift banner's surface families and detail-page
  section ids):
  [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
  and
  [`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json).
- Template gallery / prebuild / resume-live disclosure
  contract (source of truth for `support_class`,
  `availability_narrowing_class`, and
  `policy_notice_class` drift narrowing routes through):
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Bundle drift-row schema (machine-readable companion to
  this contract):
  [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json).
- Worked-example fixtures:
  [`/fixtures/workflow/bundle_drift_cases/`](../../fixtures/workflow/bundle_drift_cases/).
