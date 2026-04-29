# Template registry, scaffold-hook policy, and generated-project update semantics contract

This document freezes the boundary contract every **template registry
row**, **scaffold-hook policy**, and **generated-project update
semantics record** inherits before public generators, organization
mirrors, repo-local templates, and post-create update flows can drift
into incompatible lifecycle rules.

The companion schemas live at:

- [`/schemas/templates/template_registry_entry.schema.json`](../../schemas/templates/template_registry_entry.schema.json)
- [`/schemas/templates/scaffold_hook_policy.schema.json`](../../schemas/templates/scaffold_hook_policy.schema.json)
- [`/schemas/templates/generated_project_update_semantics.schema.json`](../../schemas/templates/generated_project_update_semantics.schema.json)

The companion fixtures live under:

- [`/fixtures/templates/template_registry_cases/`](../../fixtures/templates/template_registry_cases/)

This contract composes with, and does not replace:

- the gallery, prebuild, starter summary, health, and bypass-path
  disclosure contract in
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md);
- the template manifest, scaffold-run, and generated-project lineage
  contract in
  [`/docs/scaffolding/template_and_scaffold_contract.md`](../scaffolding/template_and_scaffold_contract.md);
- the source acquisition, trust-stage, mirror-freshness, and bootstrap
  queue vocabulary in
  [`/docs/workspace/source_acquisition_and_bootstrap_seed.md`](../workspace/source_acquisition_and_bootstrap_seed.md);
- the generated artifact safe-edit policy in
  [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md).

Where a downstream registry, gallery, scaffold runner, post-create
handoff, support export, or repo-local generator mints private trust,
hook, update, freshness, validation, or overwrite vocabulary, this
contract wins and the surface is non-conforming.

## Why freeze this now

Aureline already has a template manifest and scaffold-run contract.
That contract answers what a template revision declares and what a run
did. It does not answer the registry-level questions users and policy
surfaces need before selecting a template:

- which signing root or trust source vouches for this row;
- whether a mirror is preserving upstream identity or replacing it;
- whether a repo-local generator is local-only, supported, or merely
  present in the workspace;
- which hook classes may run and which network / credential rules gate
  them;
- whether a generated project can be reapplied, updated, rebased, or
  left alone when local divergence exists.

This contract freezes those answers without implementing a registry,
generator runtime, gallery, or update engine.

## Scope

Frozen at this revision:

1. `template_registry_entry_record` — one inspectable registry row per
   template revision, mirror row, or repo-local generator row. The row
   names template identity, signing root or trust source, origin /
   mirror class, certification and support class, compatible runtime
   and schema ranges, health cadence, known-issue disclosure, and the
   open-without-starter continuity posture.
2. `scaffold_hook_policy_record` — one policy packet per template
   revision or registry row. The packet names allowed hook classes,
   preview requirements, network rules, credential rules, generated
   artifact lineage requirements, and hidden imperative setup denial.
3. `generated_project_update_semantics_record` — one update / reapply /
   rebase evaluation per generated project and target template
   revision. The record names drift detection, local divergence,
   migration-note requirements, no-silent-overwrite rules, validation
   status, recovery choices, and bypass continuity.

Out of scope:

- building template registries, mirror services, hosted generator
  runtimes, or updater engines;
- defining final UI copy;
- executing hooks or package managers;
- changing the template manifest or scaffold-run schemas except by
  reference.

## 1. Re-exported vocabulary

This contract re-exports these existing closed sets without changing
their meaning:

- `template_source_class`, `support_class`,
  `runtime_and_toolchain_scope`, `template_lifecycle_class`,
  `declared_freshness_class`, `starter_setup_cost_class`,
  `template_health_signal_class`, `template_health_check_class`, and
  `bypass_path_id` from the template/prebuild disclosure contract.
- `template_archetype_class`, `supported_ecosystem_class`,
  `supported_platform_class`, `template_trust_posture_class`,
  `template_egress_posture_class`, `template_known_issue_class`,
  `template_reopen_preflight_class`,
  `generated_project_lineage_state_class`, and
  `generated_project_update_or_rebase_state_class` from the
  scaffolding contract.
- `signer_continuity_class`, `mirror_freshness`, `trust_stage`,
  `browse_safe_action`, `blocked_execution_path`, and
  `bootstrap_item_class` from the source-acquisition seed.
- `artifact_origin_class`, `provenance_state`,
  `default_edit_posture`, and `rebuild_intent.intent_class` from the
  generated-artifact safe-edit policy.

## 2. New closed sets

### 2.1 `template_registry_origin_class`

The source of the registry row. Closed:

- `official_origin` — the row is published by the Aureline public
  template registry and anchored in the core signing root.
- `official_mirror` — the row is an official template delivered through
  a mirror or offline bundle while preserving upstream identity.
- `org_mirror` — the row is promoted or approved by an organization
  mirror, policy bundle, or private registry.
- `community_origin` — the row is published through a governed
  community channel.
- `extension_provided` — the row is provided by an installed extension
  and inherits extension signer / permission review.
- `repo_local_generator` — the row is discovered in the opened
  repository or workspace and has no authority outside that repo unless
  explicitly exported.
- `ad_hoc_local_template` — the row was added by the local user and is
  local-only until published through a reviewed path.
- `offline_bundle` — the row is imported from a signed offline bundle.

Rules:

1. Every registry entry names exactly one
   `template_registry_origin_class`.
2. Mirrors MUST preserve both `origin_class` and
   `mirrored_from_origin_class`; a mirror row may not erase upstream
   identity.
3. `repo_local_generator` and `ad_hoc_local_template` MUST declare
   explicit trust and support classes. Location alone never implies
   trust or support.

### 2.2 `template_trust_source_class`

The trust source used to verify or bound the row. Closed:

- `core_signing_root`
- `core_signing_root_via_mirror`
- `org_policy_signing_root`
- `org_mirror_signing_root`
- `extension_publisher_signature`
- `community_channel_signature`
- `signed_offline_bundle_root`
- `repo_local_workspace_trust`
- `local_user_trust_only`
- `unsigned_user_review_required`
- `trust_source_unknown_review_required`

Rules:

1. Official rows MUST use `core_signing_root` or
   `core_signing_root_via_mirror`.
2. Org mirror rows MUST use `org_policy_signing_root` or
   `org_mirror_signing_root`.
3. Repo-local and ad hoc rows MUST use `repo_local_workspace_trust`,
   `local_user_trust_only`, or `unsigned_user_review_required`.
4. Unknown trust sources may remain visible for inspection, but they
   cannot run hooks or apply files without user review.

### 2.3 `template_certification_class`

The claim class attached to a row. Closed:

- `core_certified`
- `official_supported`
- `org_certified`
- `org_approved`
- `community_reviewed`
- `community_unreviewed`
- `extension_publisher_claimed`
- `repo_local_unreviewed`
- `local_only_unreviewed`
- `deprecated_or_archived`
- `certification_unknown_review_required`

Rules:

1. Certification and support are separate. A row may be
   `org_approved` while its `support_class` is
   `community_supported`, but both facts must remain visible.
2. `core_certified`, `official_supported`, and `org_certified` require
   a non-empty validation bundle ref and health cadence.
3. `repo_local_unreviewed`, `local_only_unreviewed`, and
   `certification_unknown_review_required` cannot be promoted to
   certified by placement inside a trusted repository or mirror.

### 2.4 `template_health_cadence_class`

How often the row's registry health evidence must be refreshed.
Closed:

- `on_every_registry_refresh`
- `daily`
- `weekly`
- `per_release_train`
- `on_template_revision_change`
- `manual_only`
- `not_scheduled_review_required`

Rules:

1. Every registry row names one cadence and one last health state.
2. Certified and official rows cannot use
   `not_scheduled_review_required`.
3. A stale, warning, or issue-flagged health state MUST NOT remove the
   open-without-starter bypass path.

### 2.5 `template_registry_health_state_class`

The current registry-level health result. Closed:

- `healthy_current`
- `healthy_cached`
- `stale_but_inspectable`
- `known_issue_non_blocking`
- `known_issue_blocks_starter`
- `validation_failed_blocks_starter`
- `signature_or_trust_failed_blocks_starter`
- `health_unknown_review_required`

Rules:

1. `stale_but_inspectable` and `known_issue_non_blocking` leave
   `open_without_starter_continuity_class =
   open_without_starter_available`.
2. Blocking states disable starter apply or reapply, but the row remains
   inspectable with health and known-issue disclosure.
3. `signature_or_trust_failed_blocks_starter` denies all mutating hooks
   even if the manifest declares them.

### 2.6 `open_without_starter_continuity_class`

Whether the non-starter path remains available when template trust,
freshness, or health narrows. Closed:

- `open_without_starter_available`
- `open_without_starter_available_read_only`
- `open_without_starter_available_after_review`
- `open_without_starter_policy_blocked_with_reason`

Rules:

1. Stale, cached, warning, and non-blocking known-issue states MUST use
   one of the first three values.
2. `open_without_starter_policy_blocked_with_reason` requires a policy
   notice ref; it may not be inferred from template health alone.

### 2.7 `scaffold_hook_policy_resolution_class`

Top-level outcome of a hook-policy evaluation. Closed:

- `allowed_after_preview`
- `allowed_without_mutation`
- `allowed_after_user_review`
- `blocked_by_policy`
- `blocked_by_trust`
- `blocked_hidden_imperative_setup`
- `blocked_network_or_credential_rule`
- `blocked_lineage_requirement_missing`

### 2.8 `scaffold_preview_requirement_class`

Preview bar before hooks may run. Closed:

- `preview_required_before_any_write`
- `preview_required_before_network_or_credential_use`
- `preview_required_before_post_create_action`
- `preview_not_required_read_only_only`
- `preview_unavailable_blocks_apply`

### 2.9 `hook_network_rule_class`

Network behavior permitted to a hook. Closed:

- `network_denied`
- `network_allowed_first_party_origin_only`
- `network_allowed_org_mirror_only`
- `network_allowed_declared_hosts_after_review`
- `network_allowed_managed_workspace_envelope_only`
- `network_unknown_blocks_hook`

### 2.10 `hook_credential_rule_class`

Credential behavior permitted to a hook. Closed:

- `no_credentials_available`
- `brokered_handle_only`
- `delegated_handle_after_review`
- `managed_workspace_handle_only`
- `credential_use_denied`
- `raw_secret_material_forbidden`

Rules:

1. Raw credential material is always forbidden in hook policy records.
2. Hooks requiring credentials must use a brokered or delegated handle
   and disclose whether the run can continue without it.

### 2.11 `generated_artifact_lineage_requirement_class`

Lineage a hook must preserve when it writes or rewrites generated
artifacts. Closed:

- `lineage_not_applicable_read_only`
- `lineage_metadata_required_before_write`
- `lineage_metadata_required_before_post_create`
- `lineage_update_required_before_reapply`
- `lineage_missing_blocks_hook`

### 2.12 `generated_project_update_operation_class`

The operation being evaluated. Closed:

- `inspect_only`
- `template_reapply`
- `template_update`
- `template_rebase`
- `relink_to_template_revision`
- `unlink_from_template`
- `open_without_starter`

### 2.13 `template_drift_detection_class`

Drift result for an update / reapply evaluation. Closed:

- `in_sync_no_drift`
- `template_revision_advanced`
- `template_revision_archived`
- `local_divergence_detected`
- `local_divergence_and_template_advanced`
- `lineage_unknown`
- `drift_detection_unavailable`

### 2.14 `local_divergence_class`

Scope of local divergence. Closed:

- `no_local_divergence`
- `comments_or_formatting_only`
- `safe_user_owned_files_only`
- `generated_files_modified`
- `conflicting_generated_and_user_files`
- `untracked_local_files_in_generated_scope`
- `divergence_unknown_review_required`

### 2.15 `overwrite_guard_class`

How overwrites are controlled. Closed:

- `no_overwrite_needed`
- `overwrite_forbidden_without_preview`
- `overwrite_requires_three_way_review`
- `overwrite_requires_user_selected_files`
- `overwrite_blocked_policy_or_trust`
- `overwrite_blocked_lineage_unknown`

Rules:

1. A generated project update or reapply record MUST NOT silently
   overwrite local divergence.
2. Any non-`no_local_divergence` local divergence requires a preview
   diff ref and one of the review-oriented overwrite guards.

### 2.16 `migration_note_class`

Migration-note requirement for an update target. Closed:

- `migration_note_not_required`
- `migration_note_available_required_for_review`
- `migration_note_missing_blocks_breaking_update`
- `migration_note_not_applicable_unlinked`

### 2.17 `update_recovery_choice_class`

Choices the gallery, generated-project detail page, and post-create
handoff can present without inventing local jargon. Closed:

- `keep_local_project`
- `open_without_starter`
- `preview_reapply`
- `preview_three_way_update`
- `export_local_patch`
- `unlink_from_template`
- `rollback_to_last_scaffold_checkpoint`
- `open_template_migration_note`
- `report_template_issue`

## 3. `template_registry_entry_record`

Every row a registry, mirror, gallery, organization policy bundle, or
repo-local discovery source exposes for selection emits one
`template_registry_entry_record`.

Required fields:

- `record_kind = template_registry_entry_record`
- `template_registry_entry_schema_version` (integer, const 1)
- `registry_entry_id` (opaque)
- `template_id` (opaque)
- `template_revision_ref` (opaque)
- `template_revision_semver` (string)
- `template_manifest_ref` (opaque)
- `template_manifest_schema_version` (integer)
- `template_registry_origin_class`
- `mirrored_from_origin_class` (nullable; required for mirror rows)
- `template_source_class`
- `template_trust_source_class`
- `trust_root_ref` (nullable; required except for local-only rows)
- `signer_continuity_class`
- `signature_class`
- `template_certification_class`
- `support_class`
- `runtime_and_toolchain_scope`
- `template_archetype_class`
- `supported_ecosystem_class_set[]`
- `supported_platform_class_set[]`
- `compatible_runtime_range`
- `compatible_schema_range`
- `declared_freshness_class`
- `mirror_freshness_ref` (nullable; required for mirror rows)
- `template_health_cadence_class`
- `template_registry_health_state_class`
- `health_check_refs[]`
- `known_issue_refs[]`
- `known_issue_disclosure_summary_ref` (nullable)
- `scaffold_hook_policy_ref`
- `generated_project_update_semantics_policy_ref`
- `open_without_starter_continuity_class`
- `bypass_path_ids[]`
- `admitted_for_generation` (boolean)
- `minted_at`

Registry invariants:

1. **Trust source visible before generation.** A row cannot rely on
   location to imply trust. The row names `template_trust_source_class`,
   `trust_root_ref`, `signer_continuity_class`, and
   `signature_class`.
2. **Mirror identity preserved.** Mirror rows retain upstream origin,
   mirror freshness, trust source, support class, and certification
   class. Mirror rows that erase upstream identity are
   non-conforming.
3. **Repo-local rows stay explicit.** Repo-local generator rows carry
   local trust and support posture explicitly, and default to user review
   before any mutating hook runs.
4. **Health does not remove the bypass.** Stale or issue-flagged rows
   may block starter apply, but they do not remove the open-without-
   starter path unless a separate policy notice blocks it.

## 4. `scaffold_hook_policy_record`

Every registry entry resolves to one `scaffold_hook_policy_record`
before any hook, setup task, package restore, credential request, or
post-create action can run.

Required fields:

- `record_kind = scaffold_hook_policy_record`
- `scaffold_hook_policy_schema_version` (integer, const 1)
- `hook_policy_id` (opaque)
- `registry_entry_ref`
- `template_id_ref`
- `template_revision_ref`
- `template_trust_source_class`
- `template_registry_health_state_class`
- `policy_resolution_class`
- `preview_requirement_class`
- `allowed_hook_rows[]`
- `blocked_hook_rows[]`
- `network_rule_class`
- `credential_rule_class`
- `generated_artifact_lineage_requirement_class`
- `hidden_imperative_setup_policy = hidden_imperative_setup_forbidden`
- `attempted_hidden_imperative_setup_detected` (boolean)
- `generated_artifact_lineage_ref` (nullable; required before writes
  when the lineage requirement is not read-only)
- `dry_run_preview_ref` (nullable; required by preview policy before
  mutation)
- `denial_reason_refs[]`
- `bypass_path_ids[]`
- `minted_at`

Hook policy invariants:

1. **No hidden hooks.** A hook or setup task not declared by the
   template manifest and admitted by the hook policy cannot run.
2. **Preview first.** Mutating hooks require a dry-run or preflight
   preview according to `preview_requirement_class`.
3. **Network and credentials are separate axes.** A hook that passes
   trust review may still be blocked by network or credential posture.
4. **Lineage before generated writes.** A hook that writes generated
   artifacts must have a lineage record before it writes or updates the
   lineage record in the same reviewed mutation group.
5. **Bypass remains visible.** Blocked hooks do not remove the
   open-without-starter path.

## 5. `generated_project_update_semantics_record`

Every generated project update, reapply, rebase, relink, unlink, or
inspect flow emits one `generated_project_update_semantics_record`
before writing.

Required fields:

- `record_kind = generated_project_update_semantics_record`
- `generated_project_update_semantics_schema_version` (integer, const 1)
- `update_semantics_id` (opaque)
- `generated_project_lineage_ref`
- `generated_project_lineage_state_class`
- `current_template_revision_ref` (nullable)
- `target_template_revision_ref` (nullable)
- `operation_class`
- `template_drift_detection_class`
- `local_divergence_class`
- `generated_project_update_or_rebase_state_class`
- `template_reopen_preflight_class`
- `overwrite_guard_class`
- `migration_note_class`
- `migration_note_ref` (nullable)
- `preview_diff_ref` (nullable; required before mutating reapply or
  update when drift or divergence exists)
- `validation_status_class`
- `health_check_refs[]`
- `known_issue_refs[]`
- `freshness_explanation_ref`
- `no_silent_overwrite = true`
- `recovery_choice_classes[]`
- `post_create_handoff_ref` (nullable)
- `bypass_path_ids[]`
- `admitted_for_apply` (boolean)
- `minted_at`

Update semantics invariants:

1. **No silent regeneration.** Reapply, update, and rebase may not
   overwrite generated or user-owned files without a preview diff and
   an overwrite guard.
2. **Local divergence is first-class.** Local edits, untracked files,
   generated-file edits, and unknown divergence are distinct states.
3. **Migration notes gate breaking movement.** A breaking update cannot
   be admitted without a migration note or an explicit blocking state.
4. **Gallery and post-create flows reuse this vocabulary.** Freshness,
   validation, known issues, recovery choices, and bypass paths are
   carried on the record so downstream surfaces do not invent template-
   local jargon.
5. **Open without starter remains a recovery choice.** Update failure,
   stale registry state, or local divergence cannot hide the
   open-without-starter route unless a policy block with a reason says
   so.

## 6. Acceptance mapping

The fixtures under
[`/fixtures/templates/template_registry_cases/`](../../fixtures/templates/template_registry_cases/)
exercise the required cases:

- `official_signed_template.yaml` — official signed registry row with
  core trust root, live health cadence, no known issues, and bypass
  continuity.
- `org_mirror_template.yaml` — organization mirror row that preserves
  upstream identity while naming org trust, support, mirror freshness,
  and health cadence explicitly.
- `repo_local_generator.yaml` — repo-local generator row with local trust
  and support posture explicit rather than inferred from repository
  location.
- `hook_blocked_template.yaml` — hook policy denying hidden imperative
  setup while preserving preview and open-without-starter continuity.
- `reapply_with_local_divergence.yaml` — generated-project update
  semantics record showing local divergence, preview-required
  overwrite guard, migration note, validation status, and recovery
  choices.

## 7. Changing this contract

- Adding a new closed-set value is additive-minor and requires updating
  the relevant schema and at least one fixture in the same change.
- Repurposing an existing value is breaking and requires a new decision
  row before any downstream surface consumes the new meaning.
- A registry, hook policy, or update semantics surface that needs a new
  field must first decide whether the field belongs here, in the
  template manifest / scaffold-run contract, or in the gallery /
  preflight disclosure contract. Do not duplicate fields across layers
  merely to simplify one renderer.
