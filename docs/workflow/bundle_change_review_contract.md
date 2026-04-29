# Bundle install/update review, change preview, and rollback-checkpoint contract

This document freezes the cross-surface review model every
**workflow-bundle install, update, downgrade, re-apply,
partial-apply, channel-promotion, channel-demotion, or
set-up-later** flow renders before any durable side effect lands.
The goal is that bundle-driven setup remains **previewable,
attributable, and reversible** — never an opaque one-click apply —
so M1 / M2 surfaces and CLI / headless paths cannot quietly install,
import, enable, mutate, or stage state that a reviewer has not seen.

The companion machine-readable schema lives at:

- [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json)

The companion fixtures live under:

- [`/fixtures/workflow/bundle_review_cases/`](../../fixtures/workflow/bundle_review_cases/)

This contract is normative for the change-preview record, the
review-sheet action set, and the rollback-checkpoint linkage. Where
it disagrees with the PRD, TAD, TDD, UI/UX spec, design-system
style guide, or milestone document anchors quoted in §13, those
sources win and this document plus its companion schema and
fixtures update in the same change. Where a downstream Start
Center, gallery, browse-bundles, update-detail, project-doctor,
support-export, CLI / headless, or import-ingress surface mints a
parallel review verb, change axis, or rollback handle, this
contract wins and the surface is non-conforming.

This contract mints **no** new bundle, identity, source, status,
packaging, signer, channel-relation, dependency-marker,
component-kind, evidence-link, lifecycle, successor, removal-
surface, or rollback-surface vocabulary. It re-exports — by
reference — the closed sets frozen in
[`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
(§3.1–§3.15, §4–§9), the appearance-session vocabulary frozen in
[`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md),
and the Start Center bundle-card / detail-page vocabulary frozen
in
[`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md).

## Who reads this contract

- **Bundle install / update flow authors** rendering the review
  sheet that gates `Confirm`. The sheet reads one
  `bundle_change_preview_record` and projects it onto the
  Start Center, gallery, update-detail, project-doctor, support-
  export, and CLI / headless families without minting parallel
  vocabulary.
- **Reviewers** comparing what a bundle would change against the
  workspace's current state. They expect to read every change
  axis (extension, settings/token, trust/permission, toolchain,
  side effect) and every action (Compare, Confirm, Cancel, Set
  up later, Inspect change source, Create rollback checkpoint)
  in the same record.
- **Project-doctor, support-export, and removal / rollback
  authors** linking a previously-applied bundle revision back to
  its change-preview record so the rollback checkpoint, decision
  rows, and non-reversible justifications stay attributable.
- **CLI / headless / API authors** later in M1 / M2 who must read
  the same review truth a UI surface renders. The boundary is
  one record kind across surfaces.

## 1. Scope

- Freeze one `bundle_change_preview_record` per install / update /
  downgrade / re-apply / partial-apply / channel-promotion /
  channel-demotion / set-up-later flow that touches one
  `workflow_bundle_manifest_record`. The preview is the
  reviewable artifact every surface renders before durable
  apply.
- Freeze the **change-axis taxonomy** (§3.3) every change entry
  resolves through: the thirteen bundle-component axes mirror
  [`workflow_bundle_object_model.md` §3.8](./workflow_bundle_object_model.md);
  the four review-local axes (`settings_or_token`,
  `trust_or_permission`, `compatibility_or_runtime`,
  `side_effect`) cover deltas the manifest does not enumerate as
  components but that a reviewer must still see.
- Freeze the **change-kind taxonomy** (§3.2) so `added`,
  `removed`, `changed`, `revision_bumped`, `unchanged_visible`,
  `preserved_local`, `blocked_pending_review`, and
  `skipped_no_op` carry stable semantics across surfaces.
- Freeze the **review-sheet action set** (§6) — `review.compare`,
  `review.confirm`, `review.cancel`, `review.set_up_later`,
  `review.inspect_change_source`, and
  `review.create_rollback_checkpoint` — so install authority,
  defer authority, drill-in authority, and rollback authority
  are typed verbs, not free-form copy.
- Freeze the **rollback-checkpoint linkage** (§7) so every
  bundle apply pairs with **exactly one** attributable rollback
  handle (single workspace / profile / user checkpoint, single
  appearance checkpoint for visual-only changes, or paired
  workspace + appearance checkpoint), or is explicitly marked
  non-reversible with a closed `non_reversibility_justification`
  class and a decision row ref.
- Freeze the **side-effect envelope** (§5) so every durable
  effect (extension install, settings write, scaffold write,
  shell invocation, provider request, telemetry emission,
  checkpoint creation) is enumerated against its scope and
  reversibility before `Confirm` runs.
- Freeze the cross-cutting **review invariants** (§9) every
  surface family (Start Center, gallery, update detail,
  project-doctor, support-export, CLI / headless, ingress
  review) MUST satisfy.

## 2. Out of scope

- The bundle **manifest** itself.
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  owns identity, component inventory, source / status / class
  linkage, lifecycle, and projection truth. This contract reads
  that record by reference.
- The **execution engine** that performs install, activation,
  scaffold expansion, settings writes, or extension network
  calls. This contract pins only the disclosure shape and the
  rollback handle; commit rides through the environment-starter
  summary contract in
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  §6.
- The **rollback engine** that materialises a checkpoint or
  restores from one. This contract reserves only the linkage
  slot; storage, scope, and atomicity mechanics live with the
  recovery / local-history and the appearance-checkpoint
  contracts.
- Final user-facing **copy / microcopy**. The shell-interaction-
  safety contract and the UX style guide own the exact strings;
  this contract pins the closed sets the copy resolves against.
- **Telemetry wire format**. Onboarding, install, and rollback
  measurement is owned by the support-export and onboarding-
  measurement plans; this contract only tags records with the
  preview id and the bundle id + revision those plans cite.
- Bundle **recommendation ranking**, **search**, or **gallery
  ordering** — owned by Start Center surfaces.

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
  `rollback_surface_kind`, `pilot_expiry_or_review_class`,
  `bundle_id`, `bundle_revision`,
  `runtime_compatibility_range` —
  [`workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  §3.1–§3.15, §5.
- `appearance_axis`, `checkpoint_class`,
  `checkpoint_scope_class`, `rollback_path_class`,
  `atomicity_class`, `appearance_apply_state` —
  [`appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md)
  and
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json).
- `bundle_card_surface_family`, `bundle_evidence_badge_id`,
  `bundle_detail_page_section_id`, `bundle_detail_page_zone` —
  [`start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md).
- `policy_notice_class`, `availability_narrowing_class`,
  `bypass_path_id` —
  [`template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).

This contract introduces **eleven** small vocabularies scoped to
the change-preview record, the review-sheet action set, and the
rollback-checkpoint linkage. Each is closed; adding a value is
additive-minor and bumps `bundle_change_preview_schema_version`,
repurposing is breaking and opens a decision row.

### 3.1 `preview_intent_class`

Why the preview was minted. The set is closed:

- `fresh_install_preview` — bundle has no prior accepted
  revision in the current workspace / profile / user scope.
- `update_to_newer_revision_preview` — current bundle is being
  updated to a higher accepted revision on the same channel.
- `downgrade_to_older_revision_preview` — current bundle is
  being downgraded to a lower accepted revision (typically a
  pinned-revision recall or a rollback to a known-good).
- `channel_promotion_preview` — bundle revision is moving from
  a less-stable to a more-stable channel (mirrors
  `bundle_channel_relation_class = cross_channel_promotion`).
- `channel_demotion_preview` — bundle revision is moving from
  a more-stable to a less-stable channel (mirrors
  `cross_channel_demotion`).
- `reapply_same_revision_preview` — bundle is being re-applied
  at the same revision (for example, after a partial failure).
- `partial_apply_after_failure_preview` — bundle apply
  previously failed mid-flight and the preview enumerates the
  un-applied delta plus the rollback handle for what was
  applied.
- `set_up_later_deferred_preview` — preview was minted but the
  reviewer chose `Set up later`; the record is preserved so
  the deferred review surface can resume from the same id.

Rules (frozen):

1. Every preview names exactly one `preview_intent_class`. A
   surface that emits a ninth class (`silent_apply`,
   `auto_repair`, etc.) is non-conforming.
2. Every intent except `fresh_install_preview` and
   `set_up_later_deferred_preview` from a fresh install MUST
   cite a `previous_bundle` identity ref. A
   `fresh_install_preview` MAY cite a previous bundle when the
   preview was minted from an imported environment so the
   reviewer sees the import source.
3. `partial_apply_after_failure_preview` MUST carry at least
   one `change_kind = blocked_pending_review` or
   `skipped_no_op` entry whose summary names the failure.

### 3.2 `change_kind`

Per-entry change posture against the workspace's prior state.
The set is closed:

- `added` — the bundle introduces this entry; before-state was
  absent. `before_revision_ref` is null.
- `removed` — the bundle removes this entry; after-state is
  absent. `after_revision_ref` is null.
- `changed` — the entry exists on both sides and one or more
  fields would change.
- `revision_bumped` — the entry's component id is unchanged
  and only the `component_revision` advances; equivalent to
  `changed` but explicitly typed so reviewers see a clean
  bump.
- `unchanged_visible` — the entry is unchanged. Listed
  explicitly so a reviewer sees that an axis was inspected
  and not silently dropped.
- `preserved_local` — the bundle would have overwritten a
  local override and the apply preserves the local override
  instead. Requires a `preserved_local_override_summary_ref`.
- `blocked_pending_review` — the entry cannot apply yet
  (signature review pending, policy narrowing, network
  unreachable, evidence aged); the preview holds the slot
  open.
- `skipped_no_op` — the entry was evaluated and the apply
  decided no change is needed in the current scope (for
  example, a docs pack already pinned to the same revision
  via another bundle).

Rules (frozen):

1. Every change entry names exactly one `change_kind`.
2. `unchanged_visible` is **never** silently dropped from the
   preview record on a `Compare` action — the surface MAY
   collapse it under a "no change" disclosure but the entry
   MUST remain reachable.
3. `blocked_pending_review` entries MUST cite a typed
   `disabled_reason_code` on the matching review-sheet action
   (typically `review.confirm`).

### 3.3 `change_axis`

Top-level axis a change entry belongs to. The set is closed
and merges the thirteen `bundle_component_kind` axes from the
manifest with the four review-local axes that cover deltas the
manifest does not enumerate as components:

- `extension_set`, `profile_preset`, `surface_preset`,
  `task_recipe`, `launch_recipe`, `debug_recipe`,
  `template_or_scaffold_ref`, `docs_pack`, `tour_pack`,
  `glossary_pack`, `migration_mapping`, `certification_target`,
  `evidence_link` — re-exported from manifest §3.8.
- `settings_or_token` — token / setting deltas the bundle
  would write at the user, profile, workspace, or policy
  scope.
- `trust_or_permission` — workspace-trust state, extension
  capability, network egress, filesystem scope, subprocess
  scope, task / run / test / debug scope, external provider
  grant, managed-cloud identity grant, signer continuity
  carryover, signature review carryover, policy lock state.
- `compatibility_or_runtime` — Aureline runtime range,
  release-channel compatibility, extension-set compatibility,
  template compatibility, archetype compatibility, toolchain /
  runtime class, mirror / offline packaging, evidence
  freshness window.
- `side_effect` — durable effects the apply would produce
  outside the manifest's component inventory (shell command
  invocation, external provider request, managed-cloud
  identity use, telemetry emission, scaffold write).

Rules (frozen):

1. Every change entry names exactly one `change_axis`.
2. The thirteen component axes use the
   `component_change_entry` shape; `settings_or_token`,
   `trust_or_permission`, `compatibility_or_runtime`, and
   `side_effect` use their own typed entry shapes (§4).
3. The preview record MUST emit every change-axis array as a
   key — empty arrays are allowed but missing keys are non-
   conforming, so reviewers see the absence.

### 3.4 `settings_or_token_axis`

Closed set of setting / token domains a bundle delta may
touch:

- `appearance_token_overlay` — token overrides routed through
  the appearance-checkpoint contract.
- `appearance_theme_package` — theme-package selection routed
  through the appearance-session contract.
- `editor_setting`, `language_setting`, `shell_setting`.
- `workspace_setting`, `profile_setting`,
  `policy_pinned_setting`.
- `keymap_binding`, `command_alias`.
- `telemetry_setting`.

Rules (frozen):

1. Every `settings_or_token_change_entry` names exactly one
   `settings_or_token_axis`.
2. `appearance_token_overlay` and `appearance_theme_package`
   entries MUST cite an
   [`appearance_checkpoint`](../ux/appearance_import_and_checkpoint_contract.md)
   ref so the appearance-session contract owns checkpoint
   atomicity for visual changes. The bundle change preview
   never owns the visual rollback handle directly.
3. `policy_pinned_setting` entries are read-only on apply:
   `change_kind` MUST be `unchanged_visible` or
   `blocked_pending_review`.

### 3.5 `trust_or_permission_axis`

Closed set of trust / permission axes a bundle apply may
touch (see schema for the full enum). Every entry MUST cite a
`before_state_class` and an `after_state_class` from
`trust_or_permission_state_class`, and MUST set
`requires_explicit_user_grant = true` whenever the after-state
is more permissive than the before-state. Hiding an
escalation behind bundle apply is non-conforming.

Rules (frozen):

1. `disclosure_required` is `const true` on every
   `trust_or_permission_change_entry`. A surface that hides
   the entry to make a row "less noisy" is non-conforming.
2. Any `change_kind = added` or `changed` entry where the
   after-state widens trust or permission MUST appear on
   `Compare` and MUST trigger
   `review.confirm.rendered_state = visible_disabled`
   (`disabled_reason_code = trust_review_required` or
   `permission_grant_review_required`) until the typed
   trust / grant prompt routed through the entry-restore
   object model has resolved.
3. `policy_pinned` trust or permission state cannot be
   widened by a bundle apply: those entries MUST be
   `unchanged_visible` or `blocked_pending_review`.

### 3.6 `compatibility_axis`

Closed set of compatibility / runtime axes. Every
`compatibility_change_entry` cites one axis and a before /
after `compatibility_state_class` from `compatible_no_change`,
`compatible_widened`, `compatible_narrowed`,
`incompatible_blocking`, `incompatible_review_required`, and
`compatibility_unknown`.

Rules (frozen):

1. `incompatible_blocking` and
   `incompatible_review_required` entries MUST cite a
   `compatibility_note_ref`.
2. `incompatible_blocking` MUST narrow `review.confirm` to
   `visible_disabled` with `disabled_reason_code` resolved
   from the manifest's `disabled_reason_code` set (typically
   `target_runtime_unavailable`, `rebuild_required`, or
   `mirror_only_cached_subset`).
3. `mirror_or_offline_packaging` axis entries MUST mirror the
   manifest's `mirror_or_offline_packaging_posture` and MUST
   NOT silently widen from `mirror_only` to `live_or_mirror`.

### 3.7 `side_effect_class` and `side_effect_scope_class`

Closed set of durable effects (`extension_install_or_activate`,
`settings_write_*`, `filesystem_scaffold_write`,
`filesystem_scaffold_overwrite_existing`,
`shell_command_invocation`, `external_provider_request`,
`managed_cloud_identity_use`, `telemetry_event_emission`,
`rollback_checkpoint_creation`, and the always-listed
`no_durable_side_effect` and
`rollback_checkpoint_omitted_non_reversible`) and the
matching scope set (`user_scope`, `profile_scope`,
`workspace_scope`, `fleet_managed_scope`,
`external_provider_scope`, `device_scope`).

Rules (frozen):

1. Every preview emits a `side_effect_envelope` array with
   `minItems = 1`. A bundle that produces no durable effect
   still emits one entry whose
   `side_effect_class = no_durable_side_effect` so reviewers
   see the absence verbatim.
2. Every entry whose `reversible_in_rollback = false` MUST
   cite a `non_reversibility_justification_class` and the
   `rollback_checkpoint_linkage` MUST resolve to
   `non_reversible_with_justification` or
   `non_reversible_pending_review`.

### 3.8 `review_action_id`

Closed set of review-sheet actions. The set is exactly:

- `review.compare`
- `review.confirm`
- `review.cancel`
- `review.set_up_later`
- `review.inspect_change_source`
- `review.create_rollback_checkpoint`

Rules (frozen):

1. Every preview emits all six actions in `review_actions`,
   each with a typed `action_rendered_state` and a
   `keyboard_reachable = true`. Omitting an action is non-
   conforming; a surface that needs the action hidden uses
   `rendered_state = hidden_not_applicable` (typically only
   `review.create_rollback_checkpoint` on a true fresh
   install with no prior state, or `review.compare` on a
   `set_up_later_deferred_preview` with no comparable prior
   bundle).
2. `visible_disabled` actions MUST cite a typed
   `disabled_reason_code`.
3. `review.confirm` MUST resolve to `visible_disabled` (with
   `disabled_reason_code = rollback_checkpoint_unavailable`
   or `non_reversible_review_required`) until the
   `rollback_checkpoint_linkage` is one of the reversible
   classes or a non-reversible class with a decision row
   ref.

### 3.9 `rollback_checkpoint_linkage_class`

Closed set of linkage shapes:

- `single_attributable_workspace_checkpoint`
- `single_attributable_profile_checkpoint`
- `single_attributable_user_checkpoint`
- `single_appearance_checkpoint_only_for_visual`
- `paired_workspace_and_appearance_checkpoint`
- `non_reversible_with_justification`
- `non_reversible_pending_review`

Rules (frozen):

1. Every preview emits exactly one
   `rollback_checkpoint_linkage`. A preview that names two
   reversible linkages or two parallel rollback handles is
   non-conforming.
2. The reversible classes MUST resolve `rollback_path_class`
   to one of `single_checkpoint_revert`,
   `surface_reload_then_revert`, or
   `full_restart_then_revert`.
3. The non-reversible classes MUST resolve
   `rollback_path_class` to `not_reversible` or
   `manual_recovery_only`, MUST cite a
   `non_reversibility_justification_class`, MUST cite a
   `non_reversibility_justification_summary_ref`, and MUST
   cite a `decision_row_ref` in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
4. `single_appearance_checkpoint_only_for_visual` is the only
   linkage class that resolves a visual change without a
   workspace checkpoint; the appearance checkpoint MUST
   resolve through
   [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json).
   When a bundle apply touches both visual and non-visual
   axes, the linkage MUST be
   `paired_workspace_and_appearance_checkpoint`.

### 3.10 `non_reversibility_justification_class`

Closed justification set for non-reversible apply paths. See
the schema for the full enum
(`external_state_mutation_committed`,
`provider_grant_unrevocable_in_session`,
`filesystem_destructive_replace_acknowledged`,
`mirror_promotion_immutable_revision`,
`local_draft_no_prior_state`,
`imported_environment_no_prior_state`,
`policy_pinned_revision_no_rollback`,
`telemetry_event_already_emitted`).

Rules (frozen):

1. Every non-reversible class names exactly one
   `non_reversibility_justification_class`.
2. The justification summary MUST name the side-effect class
   that cannot be unwound (for example, "external provider
   grant emitted") rather than free-form copy.
3. `local_draft_no_prior_state` and
   `imported_environment_no_prior_state` are the only
   classes valid on a `fresh_install_preview` with no prior
   bundle.

### 3.11 `preview_state_class`

Closed lifecycle state of the preview record itself. See the
schema for the full enum (`preview_minted_pending_validation`,
`preview_validated_ready_to_confirm`,
`preview_blocked_review_required`, `preview_cancelled`,
`preview_set_up_later_deferred`,
`preview_confirmed_apply_started`,
`preview_apply_succeeded`, `preview_apply_failed_rolled_back`,
`preview_apply_failed_partial`,
`preview_superseded_by_newer_revision`).

Rules (frozen):

1. `preview_confirmed_apply_started` and
   `preview_apply_*` states MUST preserve the original
   change-entry arrays verbatim — the apply path MAY append
   per-entry outcomes through downstream artifacts but MUST
   NOT mutate the preview record's enumerated deltas.
2. `preview_apply_failed_rolled_back` MUST cite the same
   `rollback_checkpoint_linkage` the preview was confirmed
   against.
3. `preview_superseded_by_newer_revision` MUST cite the
   newer preview's id through the support-export reference
   (this contract reserves the slot via support-export
   tagging; the wire format is owned by the support-export
   contract).

## 4. Change-entry shapes

The preview record emits five typed entry arrays (see
[schema](../../schemas/workflow/bundle_change_preview.schema.json)
for exact shapes):

- `component_change_entries[]` — one per change to a manifest
  component (`bundle_component_kind`); cites
  `before_revision_ref`, `after_revision_ref`,
  `change_summary_ref`, and at least one `change_source_ref`.
  `change_source_ref.source_class` resolves through
  `bundle_manifest_field`, `bundle_component_record`,
  `bundle_evidence_link`, `bundle_dependency_marker`,
  `bundle_lifecycle_block`, or `previous_revision_manifest`.
- `settings_or_token_change_entries[]` — one per
  setting / token delta; cites
  `settings_or_token_path_id` (opaque digest), a before /
  after `settings_value_class` (never the raw value), and the
  appearance-checkpoint / theme-import / token-overlay refs
  when the axis is `appearance_*`.
- `trust_or_permission_change_entries[]` — one per
  trust / permission delta; cites a before / after
  `trust_or_permission_state_class` and
  `requires_explicit_user_grant` (must be `true` when the
  after-state widens the before-state).
- `compatibility_change_entries[]` — one per compatibility /
  runtime delta; cites a `compatibility_state_class` pair and
  a `compatibility_note_ref` for any incompatible state.
- `side_effect_envelope[]` — one per durable effect (and at
  least one `no_durable_side_effect` entry when nothing
  durable changes); cites `side_effect_scope_class` and
  `reversible_in_rollback`.

Cross-cutting entry rules (frozen):

1. Every entry carries `keyboard_reachable = true` and
   `disclosure_required` (boolean). `disclosure_required` is
   `const true` on every `trust_or_permission_change_entry`.
2. Entries whose source class differs from the manifest's
   overall source class MUST cite a `change_source_ref`
   pointing at the originating component or evidence link.
3. Entries are append-only across preview revisions: a
   re-mint of the preview at the same `preview_id` MUST emit
   the same entries plus optional new ones. Removing an
   entry across re-mint is breaking.

## 5. Side-effect envelope

Every preview emits at least one `side_effect_envelope_entry`.
The envelope is the **commit-ready summary** the
environment-starter summary contract reads from before
applying the bundle. Side-effect entries enumerate every
durable effect across `user_scope`, `profile_scope`,
`workspace_scope`, `fleet_managed_scope`,
`external_provider_scope`, and `device_scope`.

Rules (frozen):

1. A bundle whose apply produces no durable effect emits one
   entry with
   `side_effect_class = no_durable_side_effect` so reviewers
   see the absence.
2. `rollback_checkpoint_creation` MUST appear on every
   reversible apply; `rollback_checkpoint_omitted_non_reversible`
   MUST appear on every non-reversible apply.
3. `extension_install_or_activate`,
   `extension_uninstall_or_deactivate`,
   `filesystem_scaffold_write`,
   `filesystem_scaffold_overwrite_existing`,
   `shell_command_invocation`,
   `external_provider_request`,
   `managed_cloud_identity_use`, and
   `telemetry_event_emission` entries each cite at least one
   `change_source_ref` so the
   `review.inspect_change_source` action can drill in.
4. `filesystem_scaffold_overwrite_existing` MUST trigger a
   matching `change_kind = preserved_local` entry on the
   relevant component or `non_reversibility_justification_class
   = filesystem_destructive_replace_acknowledged`.

## 6. Review-sheet actions

The review sheet emits exactly six action records:

- `review.compare` — opens a side-by-side view of the
  bundle's after-state against the workspace's before-state
  using the same change-entry arrays. The action's
  `destination_ref` resolves to a compare-report ref.
- `review.confirm` — triggers durable apply. `rendered_state`
  is `enabled` only when the rollback linkage is satisfied
  and no `blocked_pending_review` entry is present.
- `review.cancel` — discards the preview. The cancel path
  MUST NOT silently write any cached state from the preview
  back to the workspace.
- `review.set_up_later` — defers the preview by setting
  `preview_state_class = preview_set_up_later_deferred`. The
  same `preview_id` MUST be resumable from a deferred queue
  surface.
- `review.inspect_change_source` — drills into a change
  entry's `change_source_refs[]`. The destination ref
  resolves to a manifest field, component record, evidence
  link, dependency marker, lifecycle block, appearance /
  theme-import report, imported-environment record, policy-
  pinned record, or previous-revision manifest.
- `review.create_rollback_checkpoint` — creates the
  attributable rollback checkpoint that the
  `rollback_checkpoint_linkage` will reference. The action's
  `destination_ref` resolves to the checkpoint ref the
  linkage cites. On linkages that are
  `non_reversible_with_justification` or
  `non_reversible_pending_review`, this action's
  `rendered_state` is `visible_disabled` with
  `disabled_reason_code = non_reversible_review_required`.

Rules (frozen):

1. Every action's `keyboard_reachable` is `const true`.
2. Action ids are stable: surface families that need richer
   verbs (Diff, Approve, Discard, Schedule, Trace, Snapshot)
   project onto these six action ids rather than minting
   parallel verbs.
3. The review sheet renders these six action ids in the same
   order across Start Center, browse-bundles, update-detail,
   project-doctor, support-export, CLI / headless, and
   ingress-review surface families.

## 7. Rollback-checkpoint linkage

Every preview emits exactly one `rollback_checkpoint_linkage`
block. The linkage is the load-bearing trust contract for
reversibility.

### 7.1 Required fields

- `linkage_class` (§3.9).
- `rollback_path_class` (§3.9 rule 2/3).
- One of `workspace_checkpoint_ref`,
  `profile_checkpoint_ref`, `user_checkpoint_ref`, or
  `appearance_checkpoint_ref` — required by the matching
  linkage class. `paired_workspace_and_appearance_checkpoint`
  cites both `workspace_checkpoint_ref` and
  `appearance_checkpoint_ref`.
- `rollback_user_visible_action_id` — opaque ref to the
  rollback verb on the bundle detail panel, project-doctor,
  support-export, CLI / headless, or org-policy pinned-
  revision surface.
- `restores_axes[]` — closed set
  (`extensions_installed_or_activated`,
  `settings_or_token_overlays`,
  `appearance_session_state`,
  `profile_or_surface_presets`,
  `task_run_test_debug_recipes`,
  `filesystem_scaffolded_files`,
  `trust_or_permission_state`,
  `compatibility_or_runtime_pinning`,
  `managed_provider_grants_session_only`,
  `policy_pinned_revision`).
- `non_reversibility_justification_class`,
  `non_reversibility_justification_summary_ref`, and
  `decision_row_ref` — required on non-reversible classes.
- `keyboard_reachable = true`.

### 7.2 Linkage rules (frozen)

1. **One attributable handle.** A bundle apply pairs with
   exactly one rollback handle (or one paired
   workspace + appearance checkpoint). A surface that mints
   parallel handles is non-conforming.
2. **Visual changes route through the appearance contract.**
   When the preview's `change_axis` is exclusively
   `settings_or_token` with
   `settings_or_token_axis ∈ {appearance_token_overlay,
   appearance_theme_package}`, the linkage class MUST be
   `single_appearance_checkpoint_only_for_visual` and the
   appearance checkpoint MUST be a record validated by
   [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json).
3. **Mixed axes pair the checkpoint.** When the preview
   touches both visual and non-visual axes, the linkage
   class MUST be
   `paired_workspace_and_appearance_checkpoint` and both
   checkpoint refs MUST be present.
4. **Non-reversible apply requires a decision row.** The
   non-reversible classes only resolve when a decision row
   in
   [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
   names the justification, the side-effect class that
   cannot be unwound, and the user-visible disclosure on
   the review sheet.
5. **Rollback is single-action.** The rollback path MUST be
   reachable from a single user-visible action (resolved
   through `rollback_user_visible_action_id`). A rollback
   that requires multiple uncoordinated affordances is
   non-conforming.
6. **Manifest-level rollback surface stays consistent.** The
   manifest's
   [`rollback_surface_kind`](./workflow_bundle_object_model.md#3-15-rollback_surface_kind)
   MUST be compatible with the linkage's
   `rollback_user_visible_action_id` destination. A
   `not_applicable_no_prior_revision` manifest cannot
   carry a reversible linkage; it carries
   `local_draft_no_prior_state` or
   `imported_environment_no_prior_state` as justification
   instead.

## 8. Preview lifecycle and reattribution

Every preview emits exactly one `preview_state_class` (§3.11).
The lifecycle is:

```
preview_minted_pending_validation
        │
        ├──► preview_validated_ready_to_confirm ──► preview_confirmed_apply_started ──► preview_apply_succeeded
        │                                                                            └─► preview_apply_failed_rolled_back
        │                                                                            └─► preview_apply_failed_partial
        ├──► preview_blocked_review_required
        ├──► preview_cancelled
        ├──► preview_set_up_later_deferred ──► (resume) ──► preview_minted_pending_validation
        └──► preview_superseded_by_newer_revision
```

Rules (frozen):

1. The `preview_id` is **stable** across resume from
   `preview_set_up_later_deferred`. A surface that mints a
   new id on resume is non-conforming.
2. `preview_confirmed_apply_started` is irreversible only as
   the start signal: the apply path itself flows through
   the rollback handle. Failure transitions to
   `preview_apply_failed_rolled_back` (rollback succeeded)
   or `preview_apply_failed_partial` (rollback returned the
   workspace to a known intermediate state). Both states
   preserve the linkage ref so project-doctor and support-
   export read the same handle.
3. `preview_superseded_by_newer_revision` is the only
   transition that retires a preview without applying it
   while preserving its identity for evidence.

## 9. Surface invariants (cross-cutting)

1. **One preview, many surfaces.** Start Center, gallery,
   browse-bundles, update-detail, project-doctor, support-
   export, CLI / headless, ingress-review, and any later
   API all read the same `preview_id`. A surface that mints
   an alternate id is non-conforming.
2. **Six action ids, every surface.** The six review-sheet
   action ids are stable across families. A surface that
   uses `Apply`, `Skip`, `Snapshot`, or `Undo` as parallel
   verbs is non-conforming.
3. **No silent install authority.** Adding a bundle to a
   manifest, mirroring it, or moving it across channels
   MUST NOT install or activate any of its inventory
   without a confirmed preview record. The preview is the
   only authority `review.confirm` reads.
4. **No silent trust escalation.** Any
   `trust_or_permission_change_entry` whose after-state
   widens the before-state narrows
   `review.confirm.rendered_state` to `visible_disabled`
   until the typed grant prompt resolves.
5. **No silent setting writes.** Every
   `settings_or_token_change_entry` cites a setting / token
   axis and a value class — never the raw value. A surface
   that renders the raw value (or hides the entry to make a
   row "less noisy") is non-conforming.
6. **One rollback handle.** Every reversible apply pairs
   with exactly one attributable rollback handle; non-
   reversible apply cites exactly one decision row and one
   justification class.
7. **Inspect drills back to source.** The
   `review.inspect_change_source` action resolves through
   the closed `change_source_ref.source_class` set so a
   reviewer can trace any delta back to a manifest field,
   component record, evidence link, dependency marker,
   lifecycle block, appearance / theme-import report, or
   imported-environment record.
8. **Surface families share the same record.** The
   `preview_origin_class` set names every family that
   mints a preview; the record itself is identical across
   them.
9. **Lifecycle visibility post-apply.** After
   `preview_apply_succeeded`, the bundle detail panel,
   project-doctor row, support-export, and removal /
   rollback surfaces continue to project the preview's
   `preview_id`, `rollback_checkpoint_linkage`, and
   side-effect envelope. Hiding the preview because the
   bundle is installed is non-conforming.

## 10. Worked examples

Each example has a companion fixture under
[`/fixtures/workflow/bundle_review_cases/`](../../fixtures/workflow/bundle_review_cases/).
Every fixture is YAML and validates against
[`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).

### 10.1 Fresh install, certified launch bundle, single workspace checkpoint

`preview_intent_class = fresh_install_preview`,
`preview_origin_class = start_center_review_bundle_action`,
`preview_state_class = preview_validated_ready_to_confirm`.
Target bundle is the certified TypeScript web-app launch
bundle from
[`fixtures/workflow/bundles/launch_bundle_typescript_web_app.yaml`](../../fixtures/workflow/bundles/launch_bundle_typescript_web_app.yaml).
Component change entries cover added extension set, profile
preset, surface preset, task / launch / debug recipes,
template / scaffold ref, docs / tour / glossary packs,
migration mapping, and certification target. A
`trust_or_permission_change_entry` declares that workspace
trust will be granted under user review; a
`compatibility_change_entry` declares the runtime range is
compatible. Side-effect envelope enumerates extension
install, settings writes, scaffold write, rollback
checkpoint creation. Linkage:
`single_attributable_workspace_checkpoint`. See
[`fresh_install_certified_launch_bundle.yaml`](../../fixtures/workflow/bundle_review_cases/fresh_install_certified_launch_bundle.yaml).

### 10.2 Update to newer revision, paired workspace + appearance checkpoint

`preview_intent_class = update_to_newer_revision_preview`.
Update bumps the bundle to a newer revision that ships a
new theme-package preset alongside an updated extension
set. Component entries: `revision_bumped` for the extension
set; `changed` for the surface preset; `unchanged_visible`
for everything else. A `settings_or_token_change_entry`
cites `appearance_theme_package` with an
`appearance_checkpoint_ref`. Linkage:
`paired_workspace_and_appearance_checkpoint`. See
[`update_paired_appearance_and_workspace.yaml`](../../fixtures/workflow/bundle_review_cases/update_paired_appearance_and_workspace.yaml).

### 10.3 Imported-user bundle, ingress review, blocked pending review

`preview_intent_class = fresh_install_preview`,
`preview_origin_class = imported_bundle_ingress_review`,
`preview_state_class = preview_blocked_review_required`.
Target is the imported-user bundle from
[`fixtures/workflow/bundles/imported_user_bundle_pending_review.yaml`](../../fixtures/workflow/bundles/imported_user_bundle_pending_review.yaml);
a `trust_or_permission_change_entry` widens workspace trust
and the matching `review.confirm` action is
`visible_disabled` with
`disabled_reason_code = trust_review_required`. Linkage:
`single_attributable_workspace_checkpoint` so the eventual
apply remains reversible. See
[`imported_user_blocked_pending_review.yaml`](../../fixtures/workflow/bundle_review_cases/imported_user_blocked_pending_review.yaml).

### 10.4 Local-draft bundle, non-reversible with justification

`preview_intent_class = fresh_install_preview`,
`preview_origin_class = cli_or_headless_review`. Local-draft
bundle has no prior accepted state, scaffolds a directory
with `filesystem_scaffold_overwrite_existing` flagged. The
`rollback_checkpoint_linkage` resolves to
`non_reversible_with_justification` with
`non_reversibility_justification_class
= local_draft_no_prior_state` and a decision row ref. A
matching side-effect envelope entry has
`reversible_in_rollback = false`. See
[`local_draft_non_reversible.yaml`](../../fixtures/workflow/bundle_review_cases/local_draft_non_reversible.yaml).

### 10.5 Set-up-later deferred preview

`preview_intent_class = set_up_later_deferred_preview`,
`preview_state_class = preview_set_up_later_deferred`.
`review.set_up_later.rendered_state = enabled` was the
prior selected action; on resume the same `preview_id`
re-mints `preview_minted_pending_validation`. See
[`set_up_later_deferred.yaml`](../../fixtures/workflow/bundle_review_cases/set_up_later_deferred.yaml).

### 10.6 Appearance-only change, single appearance checkpoint

`preview_intent_class = update_to_newer_revision_preview`.
Update only ships a new theme-package preset and token
overlays. Component entries are all `unchanged_visible`
except the `profile_preset` (theme reference). A
`settings_or_token_change_entry` cites
`appearance_theme_package` with an
`appearance_checkpoint_ref`. Linkage:
`single_appearance_checkpoint_only_for_visual`. See
[`appearance_only_visual_update.yaml`](../../fixtures/workflow/bundle_review_cases/appearance_only_visual_update.yaml).

### 10.7 Apply-failed-rolled-back

`preview_state_class = preview_apply_failed_rolled_back`.
The same `preview_id` and the same
`rollback_checkpoint_linkage` from a prior reversible
preview now project the failure outcome onto project-
doctor and support-export. See
[`apply_failed_rolled_back.yaml`](../../fixtures/workflow/bundle_review_cases/apply_failed_rolled_back.yaml).

A `manifest.yaml` index lives alongside the fixtures and maps
every fixture file to its `preview_intent_class`, the closed
sets it exercises, and the rules it validates.

## 11. Acceptance mapping

- **A reviewer can see precisely what the bundle will install,
  import, enable, or stage before applying it.** §3.2 (change
  kinds), §3.3 (change axes), §4 (entry shapes), §5 (side-
  effect envelope), and §9 (surface invariants) together
  freeze the diffable axes a preview carries. Fixtures §10.1,
  §10.2, and §10.6 exercise the diff posture across fresh
  install, update, and appearance-only update.
- **Trust, permission, toolchain, and side-effect deltas are
  visible in the same review model as ordinary content
  changes.** §3.5
  (`trust_or_permission_axis`), §3.6 (`compatibility_axis`),
  §3.7 (`side_effect_class`), and §4
  (entry shapes) make trust, permission, compatibility, and
  side-effect deltas first-class change entries alongside the
  manifest's component-axis entries. Fixture §10.3 exercises
  the trust-escalation gate on `review.confirm`.
- **Bundle application can be paired with one attributable
  rollback checkpoint or explicitly marked as non-reversible
  where justified.** §3.9 (linkage classes), §3.10
  (justification classes), §7 (linkage rules), and §9.6 (one
  rollback handle) freeze the single-handle invariant.
  Fixtures §10.1, §10.2, §10.6 exercise reversible linkages;
  fixture §10.4 exercises non-reversible-with-justification.

## 12. Changing this contract

- **Additive-minor** changes (new `preview_intent_class`, new
  `preview_origin_class`, new `change_kind`, new
  `change_axis`, new `settings_or_token_axis`, new
  `trust_or_permission_axis`, new
  `trust_or_permission_state_class`, new `compatibility_axis`,
  new `compatibility_state_class`, new `side_effect_class`,
  new `side_effect_scope_class`, new `review_action_id`, new
  `action_rendered_state`, new `disabled_reason_code`, new
  `rollback_checkpoint_linkage_class`, new
  `rollback_path_class`, new
  `non_reversibility_justification_class`, new
  `rollback_restorable_axis`, new `preview_state_class`)
  land here, in
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json),
  and in at least one fixture under
  [`/fixtures/workflow/bundle_review_cases/`](../../fixtures/workflow/bundle_review_cases/)
  in the same change. Adding a value bumps
  `bundle_change_preview_schema_version`. Each new value
  cites the motivating preview intent, fixture, or surface
  family.
- **Repurposing** an existing vocabulary value, an action id,
  or a linkage class is breaking and requires a new decision
  row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (workflow-bundle object
  model, appearance-checkpoint contract, Start Center bundle
  surfaces, template-and-prebuild contract) happen at source
  and this contract re-exports by reference; it MUST NOT
  shadow the change.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement
  with the quotations in §13; this contract and its schema
  plus fixtures update in the same change.

## 13. Source anchors

- `.t2/docs/Aureline_PRD.md:254` — devcontainer
  compatibility, workspace templates, and optional prebuild
  snapshots are part of the remote story from day one.
- `.t2/docs/Aureline_PRD.md:1259` — remote workspaces should
  accept repo-defined devcontainer metadata and optional
  prebuild snapshots so environment setup is reproducible
  and accelerable.
- `.t2/docs/Aureline_PRD.md:2328` — intelligent project
  scaffolding and generation: starter templates and agentic
  setup for new services / apps / modules using team
  standards.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:802` — §6.9
  templates, starters, and prebuilds: source class, support
  class, runtime / toolchain, freshness, setup actions,
  always-available bypass path, side-effect envelope.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:6346` — §17.7
  scaffolding, generation, and template health: signal
  classes (`live`, `cached`, `policy_evaluated`,
  `not_checked`).
- `.t2/docs/Aureline_Milestones_Document.md:3787` —
  environment-capsule schema draft, workspace-template seed,
  and prebuild-metadata baseline.

## 14. Linked artifacts

- Workflow-bundle manifest, component inventory, and
  source-class contract (source of truth for bundle
  identity, component inventory, source / status / class
  linkage, and lifecycle):
  [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  and
  [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json).
- Workflow-bundle fixtures (the manifests previews compare
  against):
  [`/fixtures/workflow/bundles/`](../../fixtures/workflow/bundles/).
- Appearance import, token-overlay, and checkpoint contract
  (source of truth for `appearance_checkpoint_record`,
  `appearance_axis`, `rollback_path_class`, and visual
  rollback atomicity):
  [`/docs/ux/appearance_import_and_checkpoint_contract.md`](../ux/appearance_import_and_checkpoint_contract.md)
  and
  [`/schemas/ux/appearance_checkpoint.schema.json`](../../schemas/ux/appearance_checkpoint.schema.json).
- Start Center bundle card, bundle detail page, and
  evidence-badge contract (source of truth for the
  `Review bundle` action's surface families and
  detail-page section ids):
  [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
  and
  [`/schemas/ux/bundle_detail_page.schema.json`](../../schemas/ux/bundle_detail_page.schema.json).
- Template gallery / prebuild / resume-live disclosure
  contract (source of truth for `support_class`,
  `availability_narrowing_class`, and `policy_notice_class`):
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Bundle change preview schema (machine-readable companion
  to this contract):
  [`/schemas/workflow/bundle_change_preview.schema.json`](../../schemas/workflow/bundle_change_preview.schema.json).
- Bundle drift banner, local-override merge, and
  remove-bundle safety contract (downstream consumer of
  this contract's `change_kind`, `change_axis`,
  `disabled_reason_code`, and
  `rollback_checkpoint_linkage` re-exports; mints the
  drift-row record that surfaces post-install drift and
  the remove-bundle review block whose `recovery_link`
  resolves to a `bundle_change_preview_record`'s
  rollback linkage):
  [`/docs/workflow/bundle_drift_and_removal_contract.md`](./bundle_drift_and_removal_contract.md)
  and
  [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json).
- Worked-example fixtures:
  [`/fixtures/workflow/bundle_review_cases/`](../../fixtures/workflow/bundle_review_cases/).
