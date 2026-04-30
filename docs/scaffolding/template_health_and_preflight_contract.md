# Scaffold template-card, generation-preflight, and template-health contract

This document freezes the cross-tool contract every **scaffold
template card**, **generation-preflight sheet**, and **template-
health state** inherits before starter galleries, organisation
mirrors, and generated-project flows harden into the Aureline
shell. The goal is to make scaffolding reviewable and truth-
bearing: a template card discloses where the starter comes from,
who supports it, what runtime / language / platform it targets,
where its host boundary lives, what badge it carries, and what
side effects it would induce; a generation preflight enumerates
every parameter / environment use, file write, dependency
impact, execution step, immediate-vs-deferred action, checkpoint,
and delete-generated-output recovery path before any file is
written; and the template-health state vocabulary names a closed
set of conditions (fresh, validation-stale, known-issue,
deprecated, unsupported, AI-proposed-pending-review) that never
remove the user's open-without-starter route.

The companion schemas live at:

- [`/schemas/scaffolding/template_card.schema.json`](../../schemas/scaffolding/template_card.schema.json)
- [`/schemas/scaffolding/generation_preflight.schema.json`](../../schemas/scaffolding/generation_preflight.schema.json)

The companion artifact lives at:

- [`/artifacts/scaffolding/template_health_states.yaml`](../../artifacts/scaffolding/template_health_states.yaml)

The companion fixture corpus lives under:

- [`/fixtures/scaffolding/template_preflight_cases/`](../../fixtures/scaffolding/template_preflight_cases/)

This contract is normative for the card / preflight / health-
state shapes. Where it disagrees with the PRD, TAD, TDD, UI/UX
spec, or milestone document, those sources win and this document
plus its companion schemas, artifact, and fixtures update in the
same change. Where a downstream surface (template gallery,
preflight runner, post-create handoff, support / export reader)
mints a parallel card / preflight / health-state vocabulary, this
contract wins and the surface is non-conforming.

This contract mints **no** new `template_source_class`,
`support_class`, `runtime_and_toolchain_scope`,
`template_lifecycle_class`, `declared_freshness_class`,
`starter_setup_cost_class`, `template_availability_narrowing_class`,
`bypass_path_id`, `template_health_signal_class`,
`template_health_check_class`, `generation_preflight_axis`,
`post_create_handoff_axis`, `policy_notice_class`,
`template_archetype_class`, `supported_ecosystem_class`,
`supported_platform_class`, `required_parameter_kind_class`,
`generated_file_class`, `declared_hook_class`,
`declared_setup_task_class`, `template_trust_posture_class`,
`template_egress_posture_class`, `template_known_issue_class`,
`template_reopen_preflight_class`,
`scaffold_run_outcome_class`, `scaffold_dry_run_posture_class`,
`generated_project_lineage_state_class`,
`generated_project_update_or_rebase_state_class`, or
`delete_generated_output_recovery_route_class` values. Every row
here re-exports — by reference — the vocabularies frozen in:

- [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  (§3.2–§3.14) — source / support / runtime / lifecycle /
  freshness / setup-cost / availability-narrowing / bypass /
  health-signal / health-check / preflight-axis / post-create-
  handoff-axis / policy-notice classes.
- [`/docs/scaffolding/template_and_scaffold_contract.md`](./template_and_scaffold_contract.md)
  (§3.1–§3.16) — manifest / run / lineage closed sets the card,
  preflight, and health state compose with.
- [`/docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md)
  — automation-capability, trust-gate, policy-gate, approval-
  posture, dry-run-posture, signature-class, redaction-class.
- [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md)
  — signer-continuity-class, mirror-freshness-ref.

This contract introduces **eight** new closed sets scoped to the
template-card, generation-preflight, and template-health-state
surfaces.

## Who reads this contract

- **Template-card authors** publishing the per-template gallery
  projection a user inspects before a generation preflight is
  opened.
- **Generation-preflight authors** wiring the Generate sheet
  every scaffold dispatch passes through; the sheet enumerates
  parameter use, environment use, file writes, dependency
  impact, ordered execution steps, immediate-vs-deferred action
  partition, checkpoints, and the delete-generated-output
  recovery route.
- **Template-health surface authors** (gallery health row,
  preflight banner, post-create handoff) resolving fresh,
  validation-stale, known-issue, deprecated, unsupported, and
  AI-proposed-pending-review states without removing the
  always-available open-without-starter route.
- **Docs, support, and measurement authors** attributing card,
  preflight, and health evidence to the same record kinds the
  shell renders.

## 1. Scope

- Freeze one `template_card_record` per template revision the
  scaffold gallery exposes. The card names the source class,
  support class, language / runtime / platform target set, host
  boundary, signing-or-trust badge, and the enumerated side-
  effect summary (network egress, extension install, remote
  provisioning, managed service, credential provisioning).
- Freeze one `generation_preflight_record` per preflight sheet a
  user opens before pressing Generate. The preflight enumerates
  parameter use, environment use, file writes, dependency
  impact, ordered execution steps, immediate-vs-deferred action
  partition, checkpoints, and the delete-generated-output
  recovery route — never collapsed into a generic `Create`
  action.
- Freeze the closed `template_health_state_class` vocabulary
  (§3.1) and its surface-disposition mapping. Every state
  preserves at least one `bypass_path_id` at equal weight; a
  surface that hides every bypass under any state is non-
  conforming.
- Out of scope: building a template registry, mirror service,
  scaffold generator, post-create handoff renderer, or
  delete-generated-output recovery flow. The contract pins the
  closed sets and record shapes those surfaces resolve against.

## 2. Out of scope

- Implementing per-ecosystem package adapters (Cargo, npm /
  pnpm / yarn, pip / uv / poetry, Go modules, Maven / Gradle,
  RubyGems / Bundler, NuGet, system-package adapters).
- Implementing the prebuild service, warm-start backend, or
  managed-cloud provisioning path.
- Final user-facing copy / microcopy. The UX style guide and
  shell interaction-safety contract own the exact strings.
- Telemetry wire format. The telemetry / support schema
  registry pins the event names and retention.

## 3. Frozen vocabulary (re-exported) and new closed sets

This contract re-exports without modification:

- `template_source_class`, `support_class`,
  `runtime_and_toolchain_scope`, `template_lifecycle_class`,
  `declared_freshness_class`, `starter_setup_cost_class`,
  `bypass_path_id`, `template_health_signal_class`,
  `template_health_check_class` — UX template-and-prebuild
  contract §3.2–§3.11.
- `template_archetype_class`, `supported_ecosystem_class`,
  `supported_platform_class`, `required_parameter_kind_class`,
  `generated_file_class`, `declared_hook_class`,
  `declared_setup_task_class`, `template_trust_posture_class`,
  `template_egress_posture_class`, `template_known_issue_class`,
  `template_reopen_preflight_class`,
  `scaffold_run_outcome_class`, `scaffold_dry_run_posture_class`,
  `delete_generated_output_recovery_route_class` — scaffolding
  template-and-scaffold contract §3.1–§3.16.
- `signature_class`, `redaction_class` — recipe / macro
  contract.
- `signer_continuity_class` — source-acquisition / bootstrap
  seed §1.5.

This contract introduces eight new closed sets:

### 3.1 `template_health_state_class`

Closed roll-up of the per-card health state. Authoritative source
for the chip the gallery card and the preflight sheet render:

- `fresh`
- `validation_pending`
- `validation_stale`
- `validation_failed`
- `known_issue_active_workaround_documented`
- `known_issue_active_blocking_user_review_required`
- `deprecated`
- `unsupported`
- `preview_or_experimental`
- `ai_tool_proposed_pending_review`
- `health_state_unknown_requires_review`

Rules:

1. Every `template_card_record` and every
   `generation_preflight_record` names exactly one
   `template_health_state_class`. A row that elides the state is
   non-conforming.
2. Every state preserves at least one
   `create_without_starter_route_id` at equal weight on the
   surface that renders the row. A surface that hides the bypass
   under any state denies with
   `create_without_starter_route_must_remain_at_equal_weight`.
3. `validation_failed`, `unsupported`,
   `known_issue_active_blocking_user_review_required`, and
   `ai_tool_proposed_pending_review` block generation but never
   block bypass; the card disposition resolves to a typed
   visible-disabled or pending class and the preflight resolves
   to the matching `preflight_blocked_*` /
   `preflight_pending_ai_admission` disposition.
4. `deprecated` does not block generation but MUST render the
   deprecation notice on the card chrome (UX
   template-and-prebuild contract §3.3 rule 3); the preflight
   admits apply only after explicit deprecation acknowledgement.
5. `ai_tool_proposed_pending_review` MUST resolve
   `template_lifecycle_class = ai_tool_proposed_pending_review`,
   `template_trust_posture_class =
   ai_tool_proposed_template_pending_review`, and a non-null
   `ai_tool_proposed_review_ticket_ref` on both the card and the
   preflight.

### 3.2 `host_boundary_class`

Where the resulting workspace's host boundary actually lives.
Composed with `runtime_and_toolchain_scope` but not redundant:
the scope names which scope applies; the boundary class names
whether the user's process / filesystem is on-device, attached
to a container, or on a managed workspace. Closed:

- `host_local_device_only`
- `host_local_with_devcontainer_attached`
- `host_local_with_container_attached`
- `host_remote_image_required`
- `host_managed_workspace_required`
- `host_mixed_local_and_remote`
- `host_boundary_unknown_requires_review`

Rules:

1. Every `template_card_record` names exactly one
   `host_boundary_class`. The schema's allOf gate forces the
   class to agree with `runtime_and_toolchain_scope` (e.g.
   `local_only` ↔ `host_local_device_only`,
   `managed_cloud_required` ↔ `host_managed_workspace_required`).
2. `host_boundary_unknown_requires_review` is admissible only
   when the underlying `runtime_and_toolchain_scope` is
   `not_declared`.

### 3.3 `signing_or_trust_badge_class`

Closed badge the card renders next to the title. Composes
`signature_class` with `template_trust_posture_class` so the
gallery never has to compute the badge separately:

- `first_party_signed_badge`
- `organization_signed_badge`
- `managed_only_channel_signed_badge`
- `author_signed_badge`
- `community_signer_continuity_review_required_badge`
- `uncertified_no_signature_badge`
- `ai_tool_proposed_pending_review_badge`
- `quarantined_no_signature_badge`
- `trust_badge_unknown_requires_review`

Rules:

1. Every `template_card_record` names exactly one
   `signing_or_trust_badge_class`.
2. `community_signer_continuity_review_required_badge` MUST
   pair with `signer_continuity_class ∈
   {signer_changed_review_required,
   signer_unknown_user_review_required}`.
3. `ai_tool_proposed_pending_review_badge` MUST pair with
   `template_lifecycle_class =
   ai_tool_proposed_pending_review` and
   `template_trust_posture_class =
   ai_tool_proposed_template_pending_review`.

### 3.4 Side-effect-summary classes

Each card's `side_effect_summary` is a tuple of five closed
classes the gallery and the preflight chip strip read directly:

- `required_network_egress_class` — verbatim re-export of
  `template_egress_posture_class`.
- `required_extension_install_class` — closed:
  `no_extension_install_required`,
  `first_party_extension_install_required`,
  `organization_curated_extension_install_required`,
  `marketplace_extension_install_user_review_required`,
  `managed_only_channel_extension_install_required`,
  `extension_install_review_required_signature_unverified`,
  `extension_install_class_unknown_requires_review`.
- `required_remote_provisioning_class` — closed:
  `no_remote_provisioning_required`,
  `devcontainer_attach_required`, `container_attach_required`,
  `remote_image_required`, `managed_workspace_required`,
  `mixed_local_and_remote_provisioning_required`,
  `remote_provisioning_unknown_requires_review`.
- `required_managed_service_class` — closed:
  `no_managed_service_required`,
  `managed_workspace_envelope_required`,
  `managed_only_channel_invocation_required`,
  `third_party_connected_provider_required`,
  `first_party_managed_service_required`,
  `managed_service_class_unknown_requires_review`.
- `required_credential_provisioning_class` — closed:
  `no_credential_provisioning_required`,
  `secret_broker_handle_required`,
  `credential_provisioning_step_required`,
  `remote_attach_handshake_required`,
  `credential_provisioning_class_unknown_requires_review`.

Rules:

1. Every `template_card_record` carries a non-null
   `side_effect_summary` with all five classes set. A card that
   elides any of them is non-conforming.
2. `no_network_egress_required` MUST pair with
   `required_remote_provisioning_class ∈
   {no_remote_provisioning_required,
   remote_provisioning_unknown_requires_review}` and
   `required_credential_provisioning_class ∈
   {no_credential_provisioning_required,
   credential_provisioning_class_unknown_requires_review}` (no
   silent network-bound side effects under "no egress").
3. `runtime_and_toolchain_scope = managed_cloud_required` forces
   `required_remote_provisioning_class =
   managed_workspace_required` and
   `required_managed_service_class ∈
   {managed_workspace_envelope_required,
   managed_only_channel_invocation_required,
   first_party_managed_service_required}`.

### 3.5 `card_disposition_class`

Closed disposition for the card surface:

- `card_admissible_for_generation`
- `card_visible_disabled_signature_review_required`
- `card_visible_disabled_policy_narrowed`
- `card_visible_disabled_target_runtime_unavailable`
- `card_visible_disabled_template_revision_archived`
- `card_visible_disabled_known_issue_blocking`
- `card_visible_disabled_unsupported`
- `card_visible_disabled_validation_failed`
- `card_hidden_review_required`
- `ai_tool_proposed_pending_admission`
- `card_disposition_class_unknown_requires_review`

Rules:

1. Every card names exactly one `card_disposition_class`.
2. The bypass routes (`create_without_starter_route_ids`) MUST
   remain at equal weight in every disposition.
3. `card_admissible_for_generation` is admissible only when
   `template_health_state_class ∈ {fresh, validation_pending,
   validation_stale, known_issue_active_workaround_documented,
   deprecated, preview_or_experimental}` (the non-blocking
   states).
4. `ai_tool_proposed_pending_admission` MUST cite a non-null
   `ai_tool_proposed_review_ticket_ref`.

### 3.6 `preflight_axis_class`

Closed set of axes the preflight sheet renders:

- `parameter_use_preview`
- `environment_use_preview`
- `file_write_preview`
- `dependency_impact_preview`
- `execution_step_preview`
- `immediate_vs_deferred_partition`
- `checkpoint_preview`
- `delete_generated_recovery_route_preview`
- `side_effect_summary_chip_set`
- `bypass_path_preview`

Rules:

1. Every `generation_preflight_record` lists at minimum the
   following axes in `preflight_axis_set_rendered`:
   `file_write_preview`, `dependency_impact_preview`,
   `execution_step_preview`,
   `immediate_vs_deferred_partition`, `checkpoint_preview`,
   `delete_generated_recovery_route_preview`,
   `bypass_path_preview`. A preflight that hides any of these
   collapses execution into a generic `Create` action and is
   non-conforming.
2. `parameter_use_preview` is required when the bound manifest's
   `required_parameters` is non-empty; an empty parameter list
   admits omitting the axis.
3. `environment_use_preview` is required when any execution
   step or setup task reads a workspace setting, profile
   setting, secret-broker handle, managed-workspace envelope, or
   organisation provenance handle. Pure-files manifests with
   no environment use admit omitting the axis.

### 3.7 Per-axis descriptor classes

The preflight enumerates each axis through typed descriptors,
each of which composes a closed class:

- `file_write_disposition_class` — `write_new_file_no_existing_target`,
  `overwrite_existing_file_admitted`,
  `skip_if_exists_preserve_existing`,
  `refuse_overwrite_review_required`,
  `merge_with_existing_round_trip`,
  `rename_existing_then_write_new`,
  `write_disposition_class_unknown_requires_review`.
- `dependency_impact_class` — `no_dependency_impact`,
  `install_full_dependency_set`,
  `install_added_dependencies_only`,
  `restore_from_lockfile_only_no_resolution`,
  `skip_install_local_lockfile_present`,
  `network_unavailable_review_required`,
  `managed_only_channel_install_required`,
  `dependency_impact_class_unknown_requires_review`.
- `lockfile_mutation_class` — `no_lockfile_mutation`,
  `lockfile_created_new`,
  `lockfile_updated_paired_text_round_trip`,
  `lockfile_unchanged_restore_only`,
  `lockfile_mutation_class_unknown_requires_review`.
- `execution_phase_class` — `pre_fetch_phase`,
  `pre_write_phase`, `post_write_phase`, `post_create_phase`,
  `post_delete_phase`.
- `immediate_or_deferred_action_class` —
  `immediate_action_runs_on_apply`,
  `immediate_action_blocks_apply_until_done`,
  `deferred_action_runs_after_open`,
  `deferred_action_user_must_invoke`,
  `deferred_action_set_up_later_path`,
  `skipped_under_bypass_path`,
  `immediate_or_deferred_class_unknown_requires_review`.
- `checkpoint_kind_class` — `no_checkpoint_pure_files`,
  `lockfile_checkpoint_planted`,
  `workspace_snapshot_checkpoint_planted`,
  `vcs_init_checkpoint_planted`,
  `local_history_checkpoint_only`,
  `managed_workspace_checkpoint_envelope`,
  `checkpoint_kind_unknown_requires_review`.
- `environment_use_kind_class` — `no_environment_use`,
  `read_environment_variable_local_only`,
  `read_workspace_setting_local_only`,
  `read_profile_setting_local_only`,
  `read_secret_broker_handle_only`,
  `read_managed_workspace_envelope_only`,
  `read_organization_provenance_handle_only`,
  `environment_use_kind_unknown_requires_review`.

Rules:

1. Every `execution_step_preview` entry's `step_kind ∈
   {declared_hook_invocation, declared_setup_task_invocation}`
   MUST cite a non-null `bound_declared_hook_id_ref` or
   `bound_declared_setup_task_id_ref` resolving to an entry in
   the bound `template_manifest_record`'s `declared_hooks` /
   `declared_setup_tasks` arrays. An execution step that
   resolves to an undeclared hook or task is non-conforming and
   denies with
   `undeclared_hook_or_setup_task_must_not_run`.
2. `checkpoint_plant` step kinds MUST cite a non-null
   `bound_checkpoint_kind_class`.
3. `lockfile_mutation_class ∈ {lockfile_created_new,
   lockfile_updated_paired_text_round_trip}` MUST pair with a
   non-null `bound_supported_ecosystem_class` on the same
   dependency-impact entry.

### 3.8 `preflight_disposition_class` and `denial_reason_class`

`preflight_disposition_class` is closed:

- `preflight_admissible_apply_admitted`
- `preflight_dry_run_only_admitted`
- `preflight_blocked_workspace_trust_required`
- `preflight_blocked_policy_narrowed`
- `preflight_blocked_signature_review_required`
- `preflight_blocked_known_issue_blocking`
- `preflight_blocked_unsupported`
- `preflight_blocked_validation_failed`
- `preflight_blocked_target_runtime_unavailable`
- `preflight_blocked_template_revision_archived`
- `preflight_pending_ai_admission`
- `preflight_disposition_class_unknown_requires_review`

`denial_reason_class` re-exports the scaffold-run contract's
denial vocabulary (§6) and adds one new value bound to the
bypass invariant:

- `no_denial_preflight_admissible`
- `undeclared_hook_or_setup_task_must_not_run`
- `dry_run_required_before_apply_was_skipped`
- `raw_secret_in_template_manifest_forbidden`
- `raw_absolute_path_in_template_manifest_forbidden`
- `parameter_kind_must_resolve_to_typed_class`
- `template_egress_posture_must_resolve_to_typed_class`
- `template_trust_posture_must_match_declared_hooks_and_tasks`
- `workspace_trust_unset_or_restricted`
- `policy_epoch_expired_re_evaluation_required`
- `signature_review_required_user_review_required`
- `community_or_uncertified_template_pending_user_admit`
- `ai_tool_proposed_template_must_not_apply_pending_review`
- `ai_tool_proposed_run_must_not_apply_pending_admission`
- `manifest_content_address_unverifiable_user_review_required`
- `template_revision_archived_no_apply_path`
- `lineage_metadata_must_be_plain_reviewable_file`
- `create_without_starter_route_must_remain_at_equal_weight`
- `denial_reason_class_unknown_requires_review`

Rules:

1. `preflight_admissible_apply_admitted` and
   `preflight_dry_run_only_admitted` require
   `denial_reason_class = no_denial_preflight_admissible`.
2. Every `preflight_blocked_*` disposition MUST cite a denial
   reason other than `no_denial_preflight_admissible`.
3. `preflight_pending_ai_admission` MUST cite
   `ai_tool_proposed_run_must_not_apply_pending_admission`,
   `template_health_state_class_carried =
   ai_tool_proposed_pending_review`, and a non-null
   `ai_tool_proposed_review_ticket_ref`.
4. `delete_generated_recovery_route_class =
   delete_unrecoverable_user_review_required` forces the
   disposition out of `preflight_admissible_apply_admitted`.

## 4. `template_card_record`

Every template revision the scaffold gallery exposes publishes
exactly one `template_card_record`.

### 4.1 Required fields

- `record_kind = template_card_record`.
- `template_card_schema_version` (integer, const 1).
- `template_card_id` (opaque).
- `bound_template_manifest_record_ref` (opaque) — resolves
  through
  [`/schemas/scaffolding/template_manifest.schema.json`](../../schemas/scaffolding/template_manifest.schema.json);
  the manifest is the source of truth for declared hooks /
  setup tasks / parameters / generated files.
- `bound_template_id_ref`, `bound_template_revision_ref`,
  `bound_template_revision_semver` (optional).
- `title` (non-empty string).
- `summary` (reviewable sentence).
- `template_archetype_class`, `template_source_class`,
  `support_class`, `template_lifecycle_class`,
  `runtime_and_toolchain_scope`,
  `supported_ecosystem_class_set`,
  `supported_platform_class_set`.
- `host_boundary_class` (§3.2).
- `signing_or_trust_badge_class` (§3.3),
  `template_trust_posture_class`, `signature_class`,
  `signer_continuity_class` (required when
  `template_source_class ∈ {community, uncertified}`).
- `declared_freshness_class`, `starter_setup_cost_class`.
- `side_effect_summary` (§3.4) — required, with all five
  classes set.
- `template_health_state_class` (§3.1).
- `known_issue_disclosure_class` — re-exports
  `template_known_issue_class`.
- `card_disposition_class` (§3.5).
- `create_without_starter_route_ids[]` — non-empty
  `bypass_path_id` set; MUST include
  `bypass.open_folder_without_starter` on first-party / team-
  managed / community / mirror-cached cards.
- `ai_tool_proposed_review_ticket_ref` — required when
  `card_disposition_class =
  ai_tool_proposed_pending_admission` or
  `template_lifecycle_class =
  ai_tool_proposed_pending_review`.
- `notes_ref` (optional opaque ref to a notes registry).
- `minted_at` — monotonic timestamp.

### 4.2 Card invariants (allOf gates)

1. **Manifest is source of truth.** Every card cites a non-null
   `bound_template_manifest_record_ref`. The card never mints
   parallel hook / setup-task / parameter / generated-file /
   trust-posture vocabulary.
2. **Host boundary agrees with runtime scope.** §3.2 rule 1.
3. **Side-effect summary honest under no-egress.** §3.4 rule 2.
4. **Managed-cloud forces remote-provisioning.** §3.4 rule 3.
5. **Health-state forces disposition.** §3.5 rule 3 and §3.1
   rule 3.
6. **Bypass non-empty.** Every card carries at least one
   `create_without_starter_route_id` and the schema's allOf gate
   keeps the array `minItems: 1`.
7. **Community / uncertified cite signer continuity.** §3.3
   rule 2.
8. **AI-proposed pending review cites ticket.** §3.1 rule 5,
   §3.5 rule 4.

## 5. `generation_preflight_record`

Every preflight sheet a user opens before pressing Generate
publishes exactly one `generation_preflight_record`.

### 5.1 Required fields

- `record_kind = generation_preflight_record`.
- `generation_preflight_schema_version` (integer, const 1).
- `preflight_id` (opaque).
- `bound_template_card_record_ref` (opaque),
  `bound_template_manifest_record_ref` (opaque),
  `bound_template_id_ref`, `bound_template_revision_ref`.
- `preflight_axis_set_rendered[]` (§3.6) — non-empty; MUST
  contain `file_write_preview`, `dependency_impact_preview`,
  `execution_step_preview`,
  `immediate_vs_deferred_partition`, `checkpoint_preview`,
  `delete_generated_recovery_route_preview`, and
  `bypass_path_preview`.
- `parameter_use_preview[]` (array of
  `parameter_use_preview_descriptor`) — one entry per
  manifest-declared parameter; `typed_secret_broker_handle_only`
  parameters MUST cite a non-null `value_handle_ref`.
- `environment_use_preview[]` (array of
  `environment_use_preview_descriptor`).
- `file_write_preview[]` (array of
  `file_write_preview_descriptor`) — one entry per generated
  file the preflight previews; the descriptor names the
  `generated_file_id_ref`, the `generated_file_class`, and the
  `file_write_disposition_class`.
- `dependency_impact_preview[]` — one entry per ecosystem the
  manifest's `supported_ecosystem_class_set` declares; the
  descriptor names `bound_supported_ecosystem_class`,
  `dependency_impact_class`, `lockfile_mutation_class`, and
  `added_dependency_count_bucket`.
- `execution_step_preview[]` — ordered (by `step_index`) list of
  steps; each step names `execution_phase_class`,
  `immediate_or_deferred_action_class`, and `step_kind`.
- `checkpoint_preview[]` — one entry per checkpoint kind the
  preflight will plant.
- `delete_generated_recovery_route_class` (re-export);
  `delete_generated_recovery_route_ref` (optional).
- `scaffold_dry_run_posture_class` (re-export).
- `preflight_disposition_class` (§3.8).
- `denial_reason_class` (§3.8).
- `template_health_state_class_carried` (§3.1).
- `create_without_starter_route_ids[]` — non-empty.
- `ai_tool_proposed_review_ticket_ref` — required when the
  disposition is `preflight_pending_ai_admission`.
- `notes_ref` (optional).
- `minted_at` — monotonic timestamp.

### 5.2 Preflight invariants (allOf gates)

1. **No undeclared hook or setup task.** §3.7 rule 1.
2. **No collapsed `Create` action.** §3.6 rule 1 — the preflight
   MUST render the seven mechanical axes; collapsing them denies
   with `undeclared_hook_or_setup_task_must_not_run` (when an
   execution step appears that is not declared) or with
   `dry_run_required_before_apply_was_skipped` (when the
   manifest declared `dry_run_required_before_apply` but the
   preflight elided `file_write_preview`).
3. **Dry-run before apply when required.** When the bound
   manifest pins `scaffold_dry_run_posture_class =
   dry_run_required_before_apply` and
   `preflight_disposition_class =
   preflight_admissible_apply_admitted`, the
   `file_write_preview` MUST be non-empty.
4. **Unrecoverable delete blocks admit.** §3.8 rule 4.
5. **Health-blocking states block apply, never bypass.** §3.1
   rules 2 and 3.
6. **AI-proposed admission carries ticket.** §3.8 rule 3.
7. **Bypass non-empty.** Every preflight carries at least one
   `create_without_starter_route_id`.
8. **Card and preflight agree on health.**
   `template_health_state_class_carried` MUST equal the bound
   card's `template_health_state_class`; a preflight whose
   carried state disagrees with the card it points at is non-
   conforming.

## 6. Template-health surface rules

The following rules pin the "health warnings never remove the
plain open-without-starter alternative" property:

1. **Bypass at equal weight on every state.** Every
   `template_health_state_class` row in
   [`/artifacts/scaffolding/template_health_states.yaml`](../../artifacts/scaffolding/template_health_states.yaml)
   names at least one `bypass_path_id` and the schema's allOf
   gate keeps the bypass array `minItems: 1`. A surface that
   hides every bypass under any state denies with
   `create_without_starter_route_must_remain_at_equal_weight`.
2. **Card disposition follows health.** `validation_failed`,
   `unsupported`, and
   `known_issue_active_blocking_user_review_required` resolve to
   the matching `card_visible_disabled_*` disposition; the card
   never silently flips to `card_admissible_for_generation`.
3. **Preflight blocks but never hides bypass.** A preflight
   whose `template_health_state_class_carried ∈
   {known_issue_active_blocking_user_review_required,
   unsupported, validation_failed,
   ai_tool_proposed_pending_review}` resolves
   `preflight_disposition_class` to one of the
   `preflight_blocked_*` / `preflight_pending_ai_admission`
   classes, but `create_without_starter_route_ids` remains
   non-empty.
4. **Deprecated preserves apply.** `deprecated` does not block
   generation; the card MUST render the deprecation notice on
   the card chrome and the preflight admits apply only after
   explicit acknowledgement (the gallery and post-create
   handoff resolve the acknowledgement record outside this
   contract).
5. **Stale and pending preserve apply.** `validation_pending`
   and `validation_stale` do not block generation; the
   preflight resolves to `preflight_dry_run_only_admitted` when
   the bound manifest pins `dry_run_required_before_apply` and
   the user has not yet completed the preview.

## 7. Acceptance mapping

- **Users can inspect source/support class, trust, side
  effects, and template health before generation.** §4.1
  reserves `template_source_class`, `support_class`,
  `signing_or_trust_badge_class`, `host_boundary_class`,
  `side_effect_summary`, and `template_health_state_class` on
  every card; the schema's allOf gates force them to be
  non-null. Fixtures
  [`template_card_first_party_fresh.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_first_party_fresh.yaml)
  and
  [`template_card_known_issue_blocking.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_known_issue_blocking.yaml)
  exercise the disclosure floor.
- **Preflights enumerate write/dependency/execution impact
  instead of collapsing them into a generic `Create` action.**
  §5.1 reserves the seven required preflight axes and the
  schema's allOf gate forces the array to contain them.
  Execution steps cite manifest-declared hook / setup-task ids
  by ref so undeclared invocations deny with
  `undeclared_hook_or_setup_task_must_not_run`. Fixtures
  [`preflight_first_party_local_apply_admitted.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_first_party_local_apply_admitted.yaml)
  and
  [`preflight_known_issue_workaround_acknowledged.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_known_issue_workaround_acknowledged.yaml)
  exercise the enumeration.
- **Template-health warnings never remove the plain
  open-without-starter alternative.** §6 rules 1–3 pin the
  bypass invariant; the schema keeps
  `create_without_starter_route_ids` `minItems: 1` on every
  card and preflight, regardless of disposition. Fixture
  [`preflight_unsupported_template_blocked_bypass_intact.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_unsupported_template_blocked_bypass_intact.yaml)
  exercises the bypass-intact-under-blocked case;
  [`template_card_validation_stale_bypass_intact.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_validation_stale_bypass_intact.yaml)
  exercises the bypass-intact-under-stale case.

## 8. Worked examples

Each example has a companion fixture under
[`/fixtures/scaffolding/template_preflight_cases/`](../../fixtures/scaffolding/template_preflight_cases/).

### 8.1 Fresh first-party local Rust CLI card

A card for the first-party Rust CLI starter
(`template_source_class = first_party`, `support_class =
officially_supported`, `runtime_and_toolchain_scope =
local_only`, `host_boundary_class = host_local_device_only`,
`signing_or_trust_badge_class = first_party_signed_badge`,
`template_health_state_class = fresh`,
`card_disposition_class = card_admissible_for_generation`). The
side-effect summary names `no_network_egress_required`,
`no_extension_install_required`,
`no_remote_provisioning_required`,
`no_managed_service_required`, and
`no_credential_provisioning_required`. Bypass routes:
`bypass.open_folder_without_starter`,
`bypass.create_empty_workspace`. See
[`template_card_first_party_fresh.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_first_party_fresh.yaml).

### 8.2 Validation-stale card with bypass intact

The same first-party card after the freshness check returned
`stale_or_invalid` against the canonical source.
`template_health_state_class = validation_stale`;
`card_disposition_class = card_admissible_for_generation`;
bypass remains at equal weight; the card surfaces a typed repair
hook (`refresh_mirror_then_retry`). See
[`template_card_validation_stale_bypass_intact.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_validation_stale_bypass_intact.yaml).

### 8.3 Known-issue blocking card with bypass intact

A team-managed Go backend-service card whose
`known_issue_disclosure_class =
known_issue_active_blocking_user_review_required`; the schema
forces `template_health_state_class =
known_issue_active_blocking_user_review_required` and
`card_disposition_class =
card_visible_disabled_known_issue_blocking`. Bypass remains at
equal weight. See
[`template_card_known_issue_blocking.yaml`](../../fixtures/scaffolding/template_preflight_cases/template_card_known_issue_blocking.yaml).

### 8.4 Generation preflight: first-party local apply admitted

A `generation_preflight_record` against the §8.1 card. The
preflight enumerates `parameter_use_preview` (project name,
author handle, license choice), `file_write_preview` (Cargo
manifests, sources, license, README, JSONC settings overlay,
plain-JSON lineage metadata), `dependency_impact_preview`
(`rust_cargo` ecosystem, `restore_from_lockfile_only_no_resolution`
because the manifest has no scaffold-time package restore),
`execution_step_preview` (pre-write parameter validation,
post-write format, post-write apply lineage metadata,
post-create print handoff — all manifest-declared),
`checkpoint_preview` (lockfile checkpoint planted), and
`delete_generated_recovery_route_preview`
(`delete_with_lockfile_checkpoint_recovery`).
`preflight_disposition_class =
preflight_admissible_apply_admitted`. See
[`preflight_first_party_local_apply_admitted.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_first_party_local_apply_admitted.yaml).

### 8.5 Preflight: known-issue with workaround acknowledged

A preflight against a card whose health state is
`known_issue_active_workaround_documented`. The preflight
admits apply (`preflight_admissible_apply_admitted`) only after
the user has acknowledged the workaround through the
`acknowledge_known_issue_workaround` repair hook. The
`execution_step_preview` includes a `pre_write_phase` step that
records the acknowledgement; the
`delete_generated_recovery_route_class` is
`delete_with_lockfile_checkpoint_recovery`. See
[`preflight_known_issue_workaround_acknowledged.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_known_issue_workaround_acknowledged.yaml).

### 8.6 Preflight: unsupported template blocked, bypass intact

A preflight against a card whose
`template_health_state_class = unsupported` and
`card_disposition_class = card_visible_disabled_unsupported`.
The preflight resolves to
`preflight_disposition_class = preflight_blocked_unsupported`
with `denial_reason_class =
template_revision_archived_no_apply_path`; the
`create_without_starter_route_ids` array still names the
folder, workspace, clone, and create-empty bypass routes. See
[`preflight_unsupported_template_blocked_bypass_intact.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_unsupported_template_blocked_bypass_intact.yaml).

### 8.7 Preflight: AI-proposed template pending admission

A preflight whose bound card has
`template_lifecycle_class = ai_tool_proposed_pending_review`
and `card_disposition_class =
ai_tool_proposed_pending_admission`. The preflight resolves to
`preflight_disposition_class = preflight_pending_ai_admission`
with `denial_reason_class =
ai_tool_proposed_run_must_not_apply_pending_admission`; the
`ai_tool_proposed_review_ticket_ref` is non-null on both the
card and the preflight; bypass remains at equal weight. See
[`preflight_ai_proposed_pending_admission.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_ai_proposed_pending_admission.yaml).

## 9. Changing this contract

- **Additive-minor** changes (new `host_boundary_class`, new
  `signing_or_trust_badge_class`, new
  `required_extension_install_class`, new
  `required_remote_provisioning_class`, new
  `required_managed_service_class`, new
  `required_credential_provisioning_class`, new
  `card_disposition_class`, new `template_health_state_class`,
  new `preflight_axis_class`, new
  `file_write_disposition_class`, new
  `dependency_impact_class`, new `lockfile_mutation_class`, new
  `execution_phase_class`, new
  `immediate_or_deferred_action_class`, new
  `checkpoint_kind_class`, new `environment_use_kind_class`,
  new `preflight_disposition_class`, new
  `denial_reason_class` value) land here, in the companion
  schemas, in the artifact, and in at least one fixture in the
  same change. Adding a value bumps the relevant
  `*_schema_version` const.
- **Repurposing** an existing vocabulary value is breaking and
  opens a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (UX template-and-prebuild
  contract, scaffolding template-and-scaffold contract, recipe
  / macro contract, source-acquisition / bootstrap seed) happen
  at source and this contract re-exports by reference; it MUST
  NOT shadow the change.

## 10. Linked artifacts

- UX template-and-prebuild gallery / picker / preflight /
  post-create / health / policy contract:
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Scaffolding template-manifest, scaffold-run, and lineage
  contract:
  [`/docs/scaffolding/template_and_scaffold_contract.md`](./template_and_scaffold_contract.md).
- Template-card schema:
  [`/schemas/scaffolding/template_card.schema.json`](../../schemas/scaffolding/template_card.schema.json).
- Generation-preflight schema:
  [`/schemas/scaffolding/generation_preflight.schema.json`](../../schemas/scaffolding/generation_preflight.schema.json).
- Template-health-state matrix:
  [`/artifacts/scaffolding/template_health_states.yaml`](../../artifacts/scaffolding/template_health_states.yaml).
- Worked-example fixtures:
  [`/fixtures/scaffolding/template_preflight_cases/`](../../fixtures/scaffolding/template_preflight_cases/).
