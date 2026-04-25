# Template-manifest, scaffold-run, and generated-project lineage contract

This document freezes the cross-tool contract every **template
manifest**, **scaffold-run record**, and **generated-project
lineage record** inherits before any template registry, scaffold
generator, or post-create handoff surface is implemented. The
goal is to keep project scaffolding an honest, reviewable
operation: a template is a declared bundle of files plus a
declared, exhaustively enumerated list of hooks and setup tasks;
a scaffold run is a previewable, rollback-aware execution of that
bundle; and the generated project is plain, reviewable files
under version control plus documented lineage metadata, never a
hidden Aureline-only project database.

The companion schemas live at:

- [`/schemas/scaffolding/template_manifest.schema.json`](../../schemas/scaffolding/template_manifest.schema.json)
- [`/schemas/scaffolding/scaffold_run.schema.json`](../../schemas/scaffolding/scaffold_run.schema.json)

The companion fixtures live under:

- [`/fixtures/scaffolding/template_cases/`](../../fixtures/scaffolding/template_cases/)

This contract is normative for the manifest, run, and lineage
shapes. Where it disagrees with the PRD, TAD, TDD, UI/UX spec, or
milestone document, those sources win and this document plus its
companion schemas and fixtures update in the same change. Where a
downstream surface (template gallery, scaffold generator, post-
create handoff, delete-generated-output recovery flow, lineage
reader, support / export reader) mints a parallel vocabulary,
this contract wins and the surface is non-conforming.

This contract mints **no** new `template_source_class`,
`support_class`, `runtime_and_toolchain_scope`,
`template_lifecycle_class`, `declared_freshness_class`,
`starter_setup_cost_class`, `template_availability_narrowing_class`,
`bypass_path_id`, `template_health_signal_class`,
`template_health_check_class`, `generation_preflight_axis`,
`post_create_handoff_axis`, or `policy_notice_class` values.
Every row here re-exports — by reference — the vocabulary frozen
in
[`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
(§3.2–§3.14) and composes with the closed sets reserved by
[`/docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md)
(automation-capability, trust-gate, policy-gate, approval-
posture, dry-run-posture, deferred-intent, queueable-action,
reconciliation-intent, signature-class, redaction-class), the
artifact-format / comment-preservation / schema-evolution policy
in
[`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md)
(generated-file-class lineage, migration-record shape), and the
structured-artifact diff / merge / review seed in
[`/docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md)
(generated-source back-link, regenerate-or-review posture).

## Who reads this contract

- **Template-manifest authors** publishing first-party, team-
  managed, community, or local-only templates and the manifest
  rows the gallery / picker / preflight / post-create surfaces
  read.
- **Scaffold-runner authors** wiring the dry-run, apply, and
  rollback paths; every executed hook, every dependency install,
  and every setup task on the generated project resolves through
  a row defined here.
- **Generated-project lineage readers** (workspace switcher,
  doctor, support / export, delete-generated-output recovery,
  rebase / update review) attributing a project to its template
  revision and surfacing drift, rebase, and delete-recovery
  truth.
- **Docs, support, compatibility, and measurement authors**
  attributing template-health, known-issue, freshness, and
  reopen-preflight evidence to the same record kinds the shell
  renders.

## 1. Scope

- Freeze one `template_manifest_record` per template revision.
  The manifest names every file class it generates, every hook
  it declares, every setup task it declares, every required
  parameter, every supported ecosystem, every supported
  platform, every trust / egress posture, and the support class
  the template ships under. **Nothing runs that is not declared
  on the manifest.**
- Freeze one `scaffold_run_record` per attempted scaffold-run
  dispatch (whether the run completes, is dry-run only, is
  denied at a gate, is aborted by the user, is rolled back
  after partial application, or is queued pending an AI-
  proposed-run admission).
- Freeze one `generated_project_lineage_record` that the created
  workspace carries forward — and that survives portable-profile
  export, support-bundle export, and re-import — explaining
  whether the workspace is still linked to its template
  revision, what update / rebase state it is in, and what
  delete-recovery routes remain available.
- Freeze the closed vocabularies (§3) bounding template
  archetype, supported ecosystem, supported platform, parameter
  kind, generated-file class, declared-hook class, declared-
  setup-task class, trust posture, egress posture, template
  lifecycle state, known-issue posture, scaffold-run outcome,
  dry-run posture, lineage state, update / rebase / drift state,
  rollback posture, and delete-recovery route. Every value here
  composes with — never replaces — the vocabularies frozen in
  the four upstream contracts.

## 2. Out of scope

- Implementing a template gallery, a template registry, a
  scaffold generator, a post-create handoff renderer, or a
  delete-generated-output recovery flow. The contract pins the
  closed sets and record shapes those surfaces resolve against.
- Implementing per-ecosystem package adapters (Cargo, npm /
  pnpm / yarn, pip / uv / poetry, Go modules, Maven / Gradle,
  RubyGems / Bundler, NuGet, system-package adapters). The
  ecosystem class is named here; the adapter implementation is
  scoped to the package family contract.
- The prebuild service, warm-start backend, or managed-cloud
  provisioning path. UI/UX spec §16.11 owns the managed-
  workspace state taxonomy; this contract re-exports the
  relevant slots only.
- Final user-facing copy / microcopy. The UX style guide and
  shell interaction-safety contract own the exact strings; this
  contract pins the closed sets the copy resolves against.
- Telemetry wire format. The telemetry / support schema
  registry pins the event names and retention; this contract
  only references those record-class ids.

## 3. Frozen vocabulary (re-exported) and new closed sets

This contract re-exports without modification:

- `template_source_class`, `support_class`,
  `runtime_and_toolchain_scope`, `template_lifecycle_class`,
  `declared_freshness_class`, `starter_setup_cost_class`,
  `template_availability_narrowing_class`, `bypass_path_id`,
  `template_health_signal_class`, `template_health_check_class`,
  `generation_preflight_axis`, `post_create_handoff_axis`,
  `policy_notice_class` — UX template-and-prebuild contract
  §3.2–§3.14.
- `automation_capability_class`, `trust_gate_class`,
  `policy_gate_class`, `approval_posture_class`,
  `dry_run_posture_class`, `preview_posture_class`,
  `deferred_intent_class`, `queueable_action_class`,
  `reconciliation_intent_class`, `signature_class`,
  `redaction_class` — recipe / macro contract.
- `generated_file_origin_class` (`generated_only`,
  `paired_text_round_trip`, `paired_text_one_way`,
  `human_authored_no_round_trip`) and the migration-record
  shape — artifact-format / migration policy.
- `structured_artifact_class` (notebook / structured config /
  lockfile-or-manifest / coverage / image-or-design /
  evidence-packet / generated-artifact rows) — structured-
  artifact diff / merge / review seed.
- `workspace_trust_state_class` — ADR-0001 / identity family.
- `entry_verb`, `target_kind`, `resulting_mode`,
  `next_step_decision_hook`, `recovery_class` — entry-restore
  object model.

This contract introduces fifteen new closed sets scoped to
template-manifest, scaffold-run, and generated-project lineage
surfaces.

### 3.1 `template_archetype_class`

Project archetypes a template materialises. Closed:

- `web_application`
- `web_frontend_library`
- `backend_service`
- `cli_tool`
- `library_or_sdk`
- `data_or_ml_workbench`
- `mobile_application`
- `embedded_or_firmware`
- `monorepo_root_with_workspaces`
- `monorepo_member_workspace`
- `documentation_site`
- `infrastructure_or_pipeline`
- `extension_or_plugin`
- `archetype_class_unknown_requires_review`

Rules:

1. Every `template_manifest_record` names exactly one
   `template_archetype_class`. A manifest without an archetype
   is non-conforming.
2. `archetype_class_unknown_requires_review` is admissible only
   on `template_source_class` ∈ {`community`, `local_only`,
   `uncertified`}; first-party / team-managed templates MUST
   resolve to a typed archetype.

### 3.2 `supported_ecosystem_class`

Ecosystem the template generates code for. Closed:

- `rust_cargo`
- `node_npm`
- `node_pnpm`
- `node_yarn`
- `node_bun`
- `python_pip`
- `python_poetry`
- `python_uv`
- `go_modules`
- `java_maven`
- `java_gradle`
- `dotnet_nuget`
- `ruby_bundler`
- `swift_package_manager`
- `multi_ecosystem_polyglot`
- `no_ecosystem_pure_files`
- `ecosystem_class_unknown_requires_review`

Rules:

1. Every `template_manifest_record` names a non-empty
   `supported_ecosystem_class_set`. `no_ecosystem_pure_files`
   is admissible only as a singleton (a manifest cannot
   simultaneously be "no ecosystem" and a real ecosystem).
2. A manifest whose `supported_ecosystem_class_set` includes a
   real ecosystem MUST declare at least one
   `declared_setup_task_class.package_restore_for_ecosystem`
   row keyed by that ecosystem. A manifest that quietly relies
   on a package restore not declared on the manifest is non-
   conforming.

### 3.3 `supported_platform_class`

Platforms the resulting project is expected to build / run on.
Closed:

- `linux_x86_64`
- `linux_arm64`
- `macos_x86_64`
- `macos_arm64`
- `windows_x86_64`
- `windows_arm64`
- `web_browser_runtime`
- `container_runtime_target`
- `devcontainer_runtime_target`
- `managed_workspace_remote_target`
- `mobile_ios_target`
- `mobile_android_target`
- `embedded_target`
- `platform_class_unknown_requires_review`

Rules:

1. Every `template_manifest_record` names a non-empty
   `supported_platform_class_set`.
2. A manifest whose `runtime_and_toolchain_scope` is
   `managed_cloud_required` MUST include
   `managed_workspace_remote_target` in
   `supported_platform_class_set`.

### 3.4 `required_parameter_kind_class`

Closed kind for every required parameter the template prompts
for at scaffold time:

- `typed_string_label`
- `typed_enum_choice_from_closed_set`
- `typed_boolean_choice`
- `typed_integer_value_with_bounds`
- `typed_directory_label_no_raw_path`
- `typed_identity_label_no_raw_email`
- `typed_secret_broker_handle_only`
- `typed_organization_provenance_handle_only`
- `parameter_kind_unknown_requires_review`

Rules:

1. Every entry in `required_parameters` names exactly one
   `required_parameter_kind_class`. Free-form parameter kinds
   are non-conforming.
2. `typed_secret_broker_handle_only` MUST resolve to a non-null
   secret-broker handle ref at scaffold time; a manifest that
   inlines a literal secret value as a default is non-
   conforming and denies with
   `raw_secret_in_template_manifest_forbidden`.
3. `typed_directory_label_no_raw_path` and
   `typed_identity_label_no_raw_email` carry redaction-aware
   labels only; raw absolute paths and raw author email
   addresses never cross the manifest boundary.

### 3.5 `generated_file_class`

Closed class for every file the template emits:

- `source_file_text_human_authored`
- `configuration_json_human_edited`
- `configuration_jsonc_with_comments_preserved`
- `configuration_yaml_human_edited`
- `lockfile_or_manifest_paired_text_round_trip`
- `license_or_legal_text_immutable`
- `documentation_markdown_or_text`
- `static_asset_image_or_binary`
- `test_fixture_or_golden_artifact`
- `script_or_executable_file_no_post_create_invocation`
- `script_or_executable_file_invoked_under_declared_hook`
- `generated_only_artifact_back_linked_to_canonical_source`
- `lineage_metadata_file_for_generated_project`
- `generated_file_class_unknown_requires_review`

Rules:

1. Every entry in `generated_files` names exactly one
   `generated_file_class`. A file row without a class is non-
   conforming.
2. `script_or_executable_file_invoked_under_declared_hook`
   MUST cite the matching `declared_hook_id` from
   `declared_hooks`; a script that the manifest emits but does
   not declare a hook for cannot be invoked by the scaffold
   run.
3. `generated_only_artifact_back_linked_to_canonical_source`
   MUST cite a `back_link_to_canonical_source_ref` so the
   structured-artifact review seed can resolve the
   regenerate-or-review posture without inventing a parallel
   back-link convention.
4. `configuration_jsonc_with_comments_preserved` inherits the
   comment-preservation policy in
   [`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md);
   the scaffold runner MUST NOT canonicalise the JSONC body on
   apply.
5. `lineage_metadata_file_for_generated_project` is the
   on-disk projection of the
   `generated_project_lineage_record` (§7); the scaffold
   runner MUST emit at least one such file when the manifest
   produces a workspace, and the file MUST be a plain,
   reviewable text artifact (JSON or JSONC), never a binary
   blob.

### 3.6 `declared_hook_class`

Closed class for every hook the manifest declares the scaffold
runner MAY invoke. **A hook that is not declared on the
manifest cannot run.** That invariant is enforced by §6 rule 1.

- `pre_fetch_inspect_only_no_mutation`
- `pre_write_validate_parameters_no_mutation`
- `post_write_format_generated_files_local_only`
- `post_write_apply_lineage_metadata_local_only`
- `post_create_invoke_declarative_recipe`
- `post_create_invoke_recorded_macro_ui_state_replay_only`
- `post_create_open_workspace_in_editor_no_mutation`
- `post_create_print_handoff_summary_no_mutation`
- `post_delete_emit_recovery_breadcrumb_local_only`
- `hook_class_unknown_requires_review`

Rules:

1. Every entry in `declared_hooks` names exactly one
   `declared_hook_class` and exactly one `automation_capability_class`
   from the recipe / macro contract; a hook that mints a parallel
   capability ('shell_out', 'process_spawn',
   'raw_filesystem_write') is non-conforming.
2. `post_create_invoke_declarative_recipe` MUST cite a non-null
   `recipe_manifest_ref` resolvable against
   `schemas/automation/recipe_manifest.schema.json`. The
   scaffold runner MUST NOT inline raw shell fragments or raw
   command lines into a hook row.
3. `post_create_invoke_recorded_macro_ui_state_replay_only`
   MUST cite a non-null `recorded_macro_manifest_ref`. The
   recorded-macro contract's mechanical constraint forbids
   ambient network / process / secret access in this hook.
4. `pre_fetch_inspect_only_no_mutation` and
   `pre_write_validate_parameters_no_mutation` MUST resolve to
   `automation_capability_class = read_only_workspace_inspection`
   only.

### 3.7 `declared_setup_task_class`

Closed class for every setup task the manifest declares the
scaffold runner MAY enqueue. **A setup task not declared on the
manifest cannot run.** That invariant is enforced by §6 rule 1.

- `no_setup_task_pure_files`
- `package_restore_for_ecosystem`
- `toolchain_detect_local_only`
- `settings_materialize_local_only`
- `profile_materialize_local_only`
- `devcontainer_attach_under_declared_image`
- `container_attach_under_declared_image`
- `index_warm_up_local_only`
- `docs_import_local_only`
- `credential_provisioning_via_secret_broker_handle_only`
- `remote_attach_handshake_for_managed_workspace`
- `setup_task_class_unknown_requires_review`

Rules:

1. Every entry in `declared_setup_tasks` names exactly one
   `declared_setup_task_class` and one
   `starter_setup_cost_class` (re-exported from the UX
   template-and-prebuild contract §3.7) so the gallery card,
   the preflight summary, and the scaffold-run record agree on
   the cost class.
2. `package_restore_for_ecosystem` MUST cite the matching
   `supported_ecosystem_class` value; a restore for an
   ecosystem the manifest did not declare in
   `supported_ecosystem_class_set` is non-conforming.
3. `devcontainer_attach_under_declared_image` and
   `container_attach_under_declared_image` MUST cite a non-null
   `container_image_handle_ref` (opaque) so the gallery and
   preflight surfaces can resolve image identity without
   minting parallel image-name vocabularies.
4. `credential_provisioning_via_secret_broker_handle_only`
   MUST cite a non-null secret-broker handle ref;
   `remote_attach_handshake_for_managed_workspace` MUST cite a
   non-null managed-workspace envelope ref.

### 3.8 `template_trust_posture_class`

How workspace trust is required to run the manifest. Closed:

- `workspace_trust_not_required_pure_local_files_only`
- `workspace_trust_required_before_post_create_actions`
- `workspace_trust_required_before_any_apply`
- `workspace_trust_revalidated_each_step`
- `restricted_mode_tolerated_read_only_only`
- `ai_tool_proposed_template_pending_review`

Rules:

1. `workspace_trust_not_required_pure_local_files_only` is
   admissible only when `declared_setup_tasks` is empty or
   contains only `no_setup_task_pure_files` and
   `declared_hooks` is empty or contains only the read-only /
   no-mutation classes.
2. `restricted_mode_tolerated_read_only_only` requires every
   declared hook's `automation_capability_class` to be
   `read_only_workspace_inspection`.
3. `ai_tool_proposed_template_pending_review` MUST leave
   `admitted_at` null; an AI-proposed template cannot be
   committed pending user admission.

### 3.9 `template_egress_posture_class`

What network egress the manifest performs at scaffold time.
Closed:

- `no_network_egress_required`
- `egress_to_first_party_origin_only`
- `egress_to_team_managed_mirror_only`
- `egress_to_community_origin_user_review_required`
- `egress_to_managed_workspace_envelope_only`
- `egress_envelope_unknown_requires_review`

Rules:

1. Every `template_manifest_record` names exactly one
   `template_egress_posture_class`.
2. `egress_to_community_origin_user_review_required` and
   `egress_envelope_unknown_requires_review` MUST pair with a
   `template_availability_narrowing_class` from the UX
   template-and-prebuild contract §3.8 (typically
   `signature_review_required`,
   `connected_provider_policy`, or
   `uncertified_excluded`); the gallery surface that lists the
   row resolves the matching `starter_policy_notice_record`.
3. `no_network_egress_required` MUST pair with
   `declared_setup_tasks` containing no
   `package_restore_for_ecosystem`,
   `docs_import_local_only` (allowed if backed by an offline
   bundle ref), `remote_attach_handshake_for_managed_workspace`,
   or `credential_provisioning_via_secret_broker_handle_only`
   row.

### 3.10 `template_known_issue_class`

Per-issue posture for known issues the manifest discloses.
Closed:

- `no_known_issue`
- `known_issue_active_workaround_documented`
- `known_issue_active_blocking_user_review_required`
- `known_issue_mitigated_by_workaround_in_template`
- `known_issue_resolved_in_revision`
- `known_issue_class_unknown_requires_review`

Rules:

1. Every `known_issue_row` (§4.1) names exactly one
   `template_known_issue_class`.
2. `known_issue_active_blocking_user_review_required` MUST
   resolve `template_lifecycle_class` to
   `legacy_deprecated`, `unsupported`, or
   `experimental`; an officially-supported template that
   carries a blocking known issue without a deprecation /
   experimental notice is non-conforming.

### 3.11 `template_reopen_preflight_class`

Per-template result of the reopen preflight (the
"do we know enough to reopen this generated project against
its template revision?" probe). Closed:

- `reopen_preflight_in_sync_admissible`
- `reopen_preflight_template_revision_unchanged_admissible`
- `reopen_preflight_template_revision_advanced_update_available`
- `reopen_preflight_template_revision_advanced_breaking_rebase_required`
- `reopen_preflight_template_revision_archived_no_update_path`
- `reopen_preflight_template_revision_unverifiable_user_review_required`
- `reopen_preflight_lineage_unknown_imported_without_lineage_packet`

Rules:

1. Every `generated_project_lineage_record` (§7) names exactly
   one `template_reopen_preflight_class`.
2. `reopen_preflight_lineage_unknown_imported_without_lineage_packet`
   MUST resolve `generated_project_lineage_state_class` to
   `lineage_unknown_imported_without_lineage_packet` and MUST
   leave the `bound_template_revision_ref` null.

### 3.12 `scaffold_run_outcome_class`

Closed terminal outcome for a scaffold-run dispatch:

- `dry_run_preview_only_no_files_written`
- `applied_all_files_no_post_create_action`
- `applied_all_files_post_create_succeeded`
- `applied_files_post_create_partial_success`
- `applied_files_post_create_failed`
- `denied_at_trust_gate`
- `denied_at_policy_gate`
- `denied_at_approval_gate`
- `denied_pre_apply_undeclared_hook_or_task`
- `aborted_by_user_no_files_written`
- `aborted_by_user_after_partial_apply_rolled_back`
- `rolled_back_after_partial_apply`
- `rolled_back_after_failure`
- `ai_tool_proposed_run_pending_admission`
- `run_outcome_class_unknown_requires_review`

Rules:

1. Every `scaffold_run_record` names exactly one
   `scaffold_run_outcome_class`.
2. `denied_pre_apply_undeclared_hook_or_task` MUST cite a
   non-null `denial_reason_class =
   undeclared_hook_or_setup_task_must_not_run`. This is the
   schema-level enforcement of the manifest-declared-only
   invariant from §6 rule 1.
3. `ai_tool_proposed_run_pending_admission` MUST leave
   `applied_at` null and MUST cite an
   `ai_tool_proposed_run_review_ticket_ref`.

### 3.13 `scaffold_dry_run_posture_class`

Closed dry-run posture for the manifest:

- `dry_run_required_before_apply`
- `dry_run_offered_before_apply_user_may_skip`
- `dry_run_not_applicable_pure_files_no_side_effects`
- `dry_run_unavailable_irreversible_under_approval_only`

Rules:

1. A manifest whose `declared_hooks` includes any
   `post_create_invoke_declarative_recipe`,
   `post_create_invoke_recorded_macro_*`,
   `devcontainer_attach_*`, `container_attach_*`,
   `package_restore_for_ecosystem`,
   `credential_provisioning_*`, or
   `remote_attach_handshake_*` MUST set
   `scaffold_dry_run_posture_class =
   dry_run_required_before_apply` or
   `dry_run_unavailable_irreversible_under_approval_only`.
2. A manifest pinning
   `dry_run_unavailable_irreversible_under_approval_only` MUST
   pair with an `approval_posture_class` other than
   `no_approval_required`.

### 3.14 `generated_project_lineage_state_class`

Closed lineage state for a generated project. The lineage
record is the on-disk truth that survives portable export /
import and answers "is this project still linked to its
template?":

- `linked_to_template_revision`
- `linked_to_template_revision_with_local_overrides`
- `unlinked_user_disclaimed_link`
- `unlinked_template_revision_archived`
- `drifted_from_template_revision_user_review_required`
- `rebased_to_newer_template_revision`
- `forked_to_local_template_revision`
- `lineage_unknown_imported_without_lineage_packet`
- `ai_tool_proposed_relink_pending_review`
- `lineage_state_class_unknown_requires_review`

Rules:

1. Every `generated_project_lineage_record` names exactly one
   `generated_project_lineage_state_class`.
2. `linked_to_template_revision`,
   `linked_to_template_revision_with_local_overrides`,
   `drifted_from_template_revision_user_review_required`, and
   `rebased_to_newer_template_revision` MUST cite a non-null
   `bound_template_revision_ref`; the others MUST leave it
   null.
3. `lineage_unknown_imported_without_lineage_packet` MUST
   resolve `template_reopen_preflight_class` (§3.11) to
   `reopen_preflight_lineage_unknown_imported_without_lineage_packet`.
4. `ai_tool_proposed_relink_pending_review` MUST leave the
   `relinked_at` field null.

### 3.15 `generated_project_update_or_rebase_state_class`

Closed update / rebase / drift state. Closed:

- `in_sync_with_bound_template_revision`
- `update_available_minor_no_breaking_change`
- `update_available_major_breaking_change_review_required`
- `rebase_required_breaking_change_pending_user_review`
- `rebase_blocked_pending_user_admit`
- `rebase_blocked_pending_policy_review`
- `rebase_blocked_pending_workspace_trust`
- `no_update_path_template_revision_archived`
- `no_update_path_unlinked_or_lineage_unknown`
- `update_state_class_unknown_requires_review`

Rules:

1. Every `generated_project_lineage_record` names exactly one
   `generated_project_update_or_rebase_state_class`.
2. `in_sync_with_bound_template_revision`,
   `update_available_minor_no_breaking_change`, and
   `update_available_major_breaking_change_review_required`
   MUST pair with `generated_project_lineage_state_class ∈
   {linked_to_template_revision,
   linked_to_template_revision_with_local_overrides,
   rebased_to_newer_template_revision}`.
3. `no_update_path_unlinked_or_lineage_unknown` MUST pair with
   `generated_project_lineage_state_class ∈
   {unlinked_user_disclaimed_link,
   unlinked_template_revision_archived,
   lineage_unknown_imported_without_lineage_packet,
   forked_to_local_template_revision}`.

### 3.16 `delete_generated_output_recovery_route_class`

Closed recovery route for deleting the generated project's
files. The route is announced before delete and named on the
post-delete handoff so users always have a path back. Closed:

- `delete_with_lockfile_checkpoint_recovery`
- `delete_with_workspace_snapshot_recovery`
- `delete_with_local_history_recovery_only`
- `delete_with_regenerate_from_template_recovery`
- `delete_with_no_lineage_packet_local_history_only_review_required`
- `delete_unrecoverable_user_review_required`

Rules:

1. Every `scaffold_run_record` whose `scaffold_run_outcome_class`
   is `applied_all_files_*`,
   `applied_files_post_create_*`, or
   `rolled_back_after_*` MUST cite a non-null
   `delete_generated_output_recovery_route_ref` resolving to
   one of these classes; a record that elides the route is
   non-conforming.
2. `delete_unrecoverable_user_review_required` is admissible
   only when no checkpoint, no workspace snapshot, no local-
   history mirror, and no template revision reference are
   available — and MUST disclose that explicitly on the
   post-create handoff. A manifest that quietly produces a
   workspace with no recovery route is non-conforming.

## 4. `template_manifest_record`

Every template revision publishes exactly one
`template_manifest_record`.

### 4.1 Required fields

- `record_kind = template_manifest_record`.
- `template_manifest_schema_version` (integer, const 1).
- `template_id` (opaque).
- `template_revision_ref` (opaque).
- `template_revision_semver` (string).
- `title` (non-empty string).
- `summary` (reviewable sentence).
- `template_archetype_class` (§3.1).
- `template_source_class` — re-exported from UX
  template-and-prebuild contract §3.2.
- `support_class` — re-exported from UX template-and-prebuild
  contract §3.3.
- `template_lifecycle_class` — re-exported from UX template-
  and-prebuild contract §3.5.
- `signer_continuity_class` — re-exported from the source-
  acquisition seed §1.5; required when
  `template_source_class ∈ {community, uncertified}`.
- `publisher_identity_ref` (opaque).
- `signature_class` — re-exported from the recipe / macro
  contract.
- `manifest_content_address` — content-address pair (digest
  algorithm, digest hex).
- `supported_ecosystem_class_set` (§3.2; non-empty).
- `supported_platform_class_set` (§3.3; non-empty).
- `runtime_and_toolchain_scope` — re-exported from UX
  template-and-prebuild contract §3.4.
- `required_parameters` (array of
  `required_parameter_descriptor`; may be empty).
- `generated_files` (array of `generated_file_descriptor`;
  non-empty).
- `declared_hooks` (array of `declared_hook_descriptor`; may
  be empty).
- `declared_setup_tasks` (array of
  `declared_setup_task_descriptor`; may be empty).
- `template_trust_posture_class` (§3.8).
- `template_egress_posture_class` (§3.9).
- `starter_setup_cost_class` — re-exported from UX template-
  and-prebuild contract §3.7.
- `declared_freshness_class` — re-exported from UX template-
  and-prebuild contract §3.6.
- `template_health_check_descriptors[]` — at least one
  health check; each names a
  `template_health_check_class` (§3.11 of UX contract) and a
  default `template_health_signal_class`.
- `known_issue_rows[]` — array of
  `template_known_issue_descriptor`; may be empty. Each row
  names a `template_known_issue_class` (§3.10) and a
  reviewable issue summary.
- `reopen_preflight_default_class` (§3.11).
- `create_without_starter_route_ids[]` — non-empty list drawn
  from the `bypass_path_id` set re-exported from UX template-
  and-prebuild contract §3.9.
- `post_create_handoff_axes_reserved_set[]` — non-empty
  subset of `post_create_handoff_axis` re-exported from UX
  template-and-prebuild contract §3.13.
- `delete_generated_output_recovery_route_default_class`
  (§3.16).
- `linked_artifact_format_policy_ref` — opaque ref into the
  artifact-format / migration policy registry; required when
  `generated_files` includes any
  `configuration_jsonc_with_comments_preserved` or
  `lockfile_or_manifest_paired_text_round_trip` row.
- `linked_structured_artifact_review_ref` — opaque ref into
  the structured-artifact diff / merge / review seed;
  required when `generated_files` includes any
  `generated_only_artifact_back_linked_to_canonical_source`
  row.
- `redaction_class` — re-exported from the recipe / macro
  contract.
- `notes_ref` (optional opaque ref to a notes registry).
- `minted_at` — monotonic timestamp.

### 4.2 `required_parameter_descriptor`

- `parameter_id` (opaque).
- `parameter_label_ref` (opaque, redaction-aware).
- `required_parameter_kind_class` (§3.4).
- `enum_choice_set_ref` (opaque) — required when the kind is
  `typed_enum_choice_from_closed_set`.
- `default_value_ref` (opaque, optional).

### 4.3 `generated_file_descriptor`

- `generated_file_id` (opaque).
- `generated_file_label_ref` (opaque, redaction-aware; never
  a raw absolute path).
- `generated_file_class` (§3.5).
- `generated_file_origin_class` — re-exported from
  artifact-format / migration policy.
- `back_link_to_canonical_source_ref` (opaque) — required
  when the class is
  `generated_only_artifact_back_linked_to_canonical_source`.
- `invoked_under_declared_hook_id_ref` (opaque) — required
  when the class is
  `script_or_executable_file_invoked_under_declared_hook`.

### 4.4 `declared_hook_descriptor`

- `declared_hook_id` (opaque).
- `declared_hook_class` (§3.6).
- `automation_capability_class` (re-exported).
- `recipe_manifest_ref` (opaque, optional) — required when
  the hook class is
  `post_create_invoke_declarative_recipe`.
- `recorded_macro_manifest_ref` (opaque, optional) —
  required when the hook class is
  `post_create_invoke_recorded_macro_ui_state_replay_only`.
- `trust_gate_class` (re-exported).
- `policy_gate_class` (re-exported).
- `approval_posture_class` (re-exported).

### 4.5 `declared_setup_task_descriptor`

- `declared_setup_task_id` (opaque).
- `declared_setup_task_class` (§3.7).
- `bound_supported_ecosystem_class` (optional) — required
  when the class is `package_restore_for_ecosystem`.
- `container_image_handle_ref` (optional) — required when
  the class is
  `devcontainer_attach_under_declared_image` or
  `container_attach_under_declared_image`.
- `secret_broker_handle_ref` (optional) — required when the
  class is
  `credential_provisioning_via_secret_broker_handle_only`.
- `managed_workspace_envelope_ref` (optional) — required
  when the class is
  `remote_attach_handshake_for_managed_workspace`.
- `starter_setup_cost_class` (re-exported).
- `dry_run_posture_class` (re-exported).

### 4.6 `template_known_issue_descriptor`

- `known_issue_id` (opaque).
- `template_known_issue_class` (§3.10).
- `issue_summary` (reviewable sentence).
- `affected_template_revision_ref_set[]` (opaque ids).
- `workaround_summary_ref` (opaque, optional).
- `resolution_revision_ref` (opaque, optional) — required
  when the class is `known_issue_resolved_in_revision`.

### 4.7 Manifest invariants

1. **No undeclared hook or task may run.** Every hook and
   every setup task the scaffold runner enqueues MUST resolve
   through a `declared_hook_id` / `declared_setup_task_id`
   that exists on the manifest. The scaffold-run record (§5)
   denies with
   `undeclared_hook_or_setup_task_must_not_run` when this
   invariant is violated.
2. **No raw secret in manifest.** A manifest that inlines a
   raw bearer token, raw API key, raw password, raw refresh
   token, raw signing key, or raw certificate / key material
   in any default-value or label field is non-conforming and
   denies with `raw_secret_in_template_manifest_forbidden`.
3. **No raw absolute path.** Manifest fields carry redaction-
   aware labels and opaque ids only; raw absolute filesystem
   paths, raw repository URLs, and raw author email addresses
   never cross the manifest boundary.
4. **AI-proposed templates pending review.** A manifest whose
   `template_lifecycle_class` is
   `ai_tool_proposed_pending_review` MUST set
   `template_trust_posture_class =
   ai_tool_proposed_template_pending_review` and MUST leave
   `admitted_at` null.
5. **Health, freshness, known-issue, reopen-preflight all
   first-class.** A manifest that elides
   `template_health_check_descriptors`,
   `declared_freshness_class`, `known_issue_rows`, or
   `reopen_preflight_default_class` is non-conforming.
   Gallery, post-create, and lineage surfaces resolve those
   slots without minting parallel metadata.
6. **Create-without-starter route reserved.** Every manifest
   names at least one `create_without_starter_route_id` so the
   gallery / preflight / post-create surfaces can keep an
   equal-weight bypass path on the same surface (UX template-
   and-prebuild §3.9 / §11.5).

## 5. `scaffold_run_record`

Every attempted scaffold-run dispatch publishes exactly one
`scaffold_run_record`.

### 5.1 Required fields

- `record_kind = scaffold_run_record`.
- `scaffold_run_schema_version` (integer, const 1).
- `scaffold_run_id` (opaque).
- `bound_template_id_ref` (opaque).
- `bound_template_revision_ref` (opaque).
- `dispatched_by_actor_class` — closed (`user_authored_local`,
  `workspace_shared_committed`, `org_curated_admin_published`,
  `managed_workspace_provisioned`,
  `imported_from_export_bundle`, `ad_hoc_session`,
  `ai_tool_proposed_pending_review`).
- `scaffold_run_outcome_class` (§3.12).
- `scaffold_dry_run_posture_class` (§3.13).
- `dry_run_preview_ref` (opaque) — required when
  `scaffold_run_outcome_class ∈
  {dry_run_preview_only_no_files_written,
  applied_all_files_no_post_create_action,
  applied_all_files_post_create_succeeded,
  applied_files_post_create_partial_success,
  applied_files_post_create_failed}` and
  `scaffold_dry_run_posture_class =
  dry_run_required_before_apply` (the dry-run preview MUST
  exist before apply when required).
- `parameter_value_refs[]` — opaque refs to a per-parameter
  value registry; one entry per `required_parameters` row.
- `applied_at` — monotonic timestamp; null when outcome is
  `dry_run_preview_only_no_files_written`,
  `denied_at_*`, `aborted_by_user_no_files_written`, or
  `ai_tool_proposed_run_pending_admission`.
- `applied_generated_file_ids[]` — every
  `generated_file_id` actually written; subset of the
  manifest's `generated_files`.
- `invoked_declared_hook_ids[]` — every
  `declared_hook_id` actually invoked; subset of the
  manifest's `declared_hooks`.
- `invoked_declared_setup_task_ids[]` — every
  `declared_setup_task_id` actually invoked; subset of the
  manifest's `declared_setup_tasks`.
- `denial_reason_class` — closed (see §6.2); required when
  outcome is any `denied_*` value.
- `rollback_checkpoint_ref` (opaque) — required when outcome
  is `rolled_back_after_*` or
  `aborted_by_user_after_partial_apply_rolled_back`.
- `delete_generated_output_recovery_route_ref` — opaque ref
  into a per-route registry resolving to a
  `delete_generated_output_recovery_route_class` (§3.16);
  required when the run produced files (i.e., outcome is
  `applied_all_files_*`, `applied_files_post_create_*`, or
  `rolled_back_after_*`).
- `post_create_handoff_summary_ref` (opaque) — opaque ref to
  a `post_create_handoff_summary_record` (UX template-and-
  prebuild contract §8); required when outcome is
  `applied_all_files_post_create_*` or
  `applied_files_post_create_*`.
- `created_workspace_lineage_record_ref` (opaque) —
  required when the run produced a workspace; cites the
  matching `generated_project_lineage_record` (§7).
- `ai_tool_proposed_run_review_ticket_ref` (opaque) —
  required when outcome is
  `ai_tool_proposed_run_pending_admission`.
- `dispatched_at` — monotonic timestamp.

### 5.2 Run invariants (allOf gates)

1. **No undeclared hook or task.** If the run lists an
   `invoked_declared_hook_id` not present in the manifest's
   `declared_hooks`, or an `invoked_declared_setup_task_id`
   not present in the manifest's `declared_setup_tasks`, the
   record MUST resolve `scaffold_run_outcome_class =
   denied_pre_apply_undeclared_hook_or_task` and
   `denial_reason_class =
   undeclared_hook_or_setup_task_must_not_run`.
2. **Dry-run before apply when required.** When
   `scaffold_dry_run_posture_class =
   dry_run_required_before_apply` and
   `scaffold_run_outcome_class ∈
   {applied_all_files_*, applied_files_post_create_*}`, the
   `dry_run_preview_ref` MUST be non-null. A run that
   short-circuits the dry-run preview is non-conforming.
3. **Rollback checkpoint when rolled back.** Outcomes
   `rolled_back_after_partial_apply`,
   `rolled_back_after_failure`, and
   `aborted_by_user_after_partial_apply_rolled_back` MUST
   cite a non-null `rollback_checkpoint_ref`.
4. **Recovery route when files were written.** When the run
   produced files (i.e., outcome is
   `applied_all_files_*`, `applied_files_post_create_*`, or
   `rolled_back_after_*`), the
   `delete_generated_output_recovery_route_ref` MUST be
   non-null.
5. **Lineage record when files were written.** When the run
   produced a workspace, the
   `created_workspace_lineage_record_ref` MUST be non-null;
   the lineage record carries the on-disk projection that
   survives portable export / import.
6. **AI-proposed run pending admission.** Outcome
   `ai_tool_proposed_run_pending_admission` MUST leave
   `applied_at` null and MUST cite a non-null
   `ai_tool_proposed_run_review_ticket_ref`.

## 6. Closed denial-reason vocabulary

`scaffold_run_record.denial_reason_class` resolves only to
values in this closed set:

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
- `denial_reason_class_unknown_requires_review`

Adding a new value is additive-minor; repurposing one is
breaking and opens a new decision row in
`artifacts/governance/decision_index.yaml`.

## 7. `generated_project_lineage_record`

The lineage record is the on-disk truth that survives portable
export / import. It is the projection of the
`lineage_metadata_file_for_generated_project` generated-file
class onto a structured record, and answers — without external
context — whether the generated project is still linked to its
template.

### 7.1 Required fields

- `record_kind = generated_project_lineage_record`.
- `generated_project_lineage_schema_version` (integer, const 1).
- `lineage_id` (opaque).
- `created_workspace_label_ref` (opaque, redaction-aware; never
  a raw absolute path).
- `bound_template_id_ref` (opaque, may be null).
- `bound_template_revision_ref` (opaque, may be null).
- `bound_template_revision_semver` (string, may be null).
- `generated_project_lineage_state_class` (§3.14).
- `generated_project_update_or_rebase_state_class` (§3.15).
- `template_reopen_preflight_class` (§3.11).
- `dispatched_scaffold_run_id_ref` (opaque, may be null when
  the project was imported without lineage).
- `relinked_at` — monotonic timestamp; non-null only when
  state is `rebased_to_newer_template_revision` or
  `linked_to_template_revision_with_local_overrides`.
- `delete_generated_output_recovery_route_default_class`
  (§3.16).
- `lineage_packet_export_posture_class` — closed
  (`exportable_via_portable_profile`,
  `exportable_via_support_bundle_redacted`,
  `exportable_via_organization_share_managed_only`,
  `not_exportable_local_only`,
  `export_denied_by_policy`); reuses the `export_posture_class`
  vocabulary from the recipe / macro contract.
- `lineage_packet_redaction_class` — re-exported from the
  recipe / macro contract.
- `notes_ref` (opaque, optional).
- `minted_at` — monotonic timestamp.

### 7.2 Lineage invariants (allOf gates)

1. **Bound revision iff linked.** States in §3.14 rule 2 MUST
   cite a non-null `bound_template_revision_ref`; the others
   MUST leave it null.
2. **Reopen-preflight matches lineage.**
   `lineage_unknown_imported_without_lineage_packet` MUST
   resolve `template_reopen_preflight_class` to
   `reopen_preflight_lineage_unknown_imported_without_lineage_packet`.
3. **Update-state matches lineage.** §3.15 rules 2 and 3
   gate the update / rebase / drift state to the lineage
   state.
4. **Survives export / import.** The lineage record is the
   portable truth: a portable-profile export, a support-
   bundle export, and a re-import round-trip MUST preserve
   `lineage_id`, `bound_template_id_ref`,
   `bound_template_revision_ref`,
   `generated_project_lineage_state_class`,
   `generated_project_update_or_rebase_state_class`, and
   `template_reopen_preflight_class` byte-for-byte (subject
   to the redaction class).
5. **Recovery route always named.** Every lineage record
   names a
   `delete_generated_output_recovery_route_default_class`
   so a future delete-generated-output flow can resolve the
   recovery route without inventing a parallel registry.
6. **AI-proposed relink pending review.**
   `ai_tool_proposed_relink_pending_review` MUST leave
   `relinked_at` null.

## 8. Generated-project health rules

The following rules pin the "no hidden Aureline-only project
database" property:

1. **Plain reviewable files.** Every entry in
   `generated_files` resolves to a
   `generated_file_class` whose body is reviewable in a plain
   text editor (JSON, JSONC, YAML, source, Markdown, license,
   plain-text fixture) **or** is an explicitly-classed binary
   asset (`static_asset_image_or_binary`,
   `test_fixture_or_golden_artifact`). A file row whose class
   is `generated_file_class_unknown_requires_review` is
   admissible only with a paired
   `back_link_to_canonical_source_ref` and resolves to the
   structured-artifact review seed's regenerate-or-review
   posture.
2. **Lineage metadata is on-disk and reviewable.** The
   `lineage_metadata_file_for_generated_project` generated-
   file class is the on-disk projection of the
   `generated_project_lineage_record` (§7) and MUST be a
   plain JSON or JSONC file under version control. A
   manifest that hides lineage in a binary blob, an opaque
   sqlite database, or an Aureline-only RPC store is non-
   conforming and denies with
   `lineage_metadata_must_be_plain_reviewable_file`.
3. **No hidden mutation outside declared rows.** A scaffold
   run that writes any file not in the manifest's
   `generated_files`, invokes any hook not in
   `declared_hooks`, or enqueues any setup task not in
   `declared_setup_tasks` is non-conforming; the run record
   denies under §5.2 rule 1.
4. **Comment-preservation honoured.** A manifest that emits a
   `configuration_jsonc_with_comments_preserved` file MUST
   inherit the comment-preservation policy from
   [`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md);
   the scaffold runner MUST NOT canonicalise the JSONC body
   or strip comments on apply, on update, or on rebase.
5. **Generated-only artifacts back-link.** Every
   `generated_only_artifact_back_linked_to_canonical_source`
   row cites the structured-artifact review seed's
   regenerate-or-review posture so a downstream review
   surface can resolve the back-link without inventing a
   parallel registry.

## 9. Acceptance mapping

- **No hook, dependency install, or setup task can execute
  unless declared in the manifest fixture.** §3.6 rule 1,
  §3.7 rule 1, §4.7 rule 1, §5.2 rule 1, and the closed
  denial-reason vocabulary entry
  `undeclared_hook_or_setup_task_must_not_run` mechanically
  enforce this. Fixture
  `denied_undeclared_hook_run.yaml` exercises the denial.
- **Dry-run and rollback metadata remain first-class.** §5.1
  reserves `dry_run_preview_ref`,
  `scaffold_dry_run_posture_class`, and
  `rollback_checkpoint_ref`; §5.2 rules 2 and 3 force them
  on the relevant outcomes. Fixtures
  `dry_run_preview_first_party_local_template.yaml` and
  `partial_apply_rollback.yaml` exercise both.
- **Generated-project lineage survives export / import.**
  §7.1 reserves the lineage record, §7.2 rule 4 pins the
  byte-for-byte preservation invariant, and the
  `lineage_metadata_file_for_generated_project` generated-
  file class makes the record a plain on-disk file. Fixtures
  `lineage_linked_to_template_revision.yaml` and
  `lineage_unknown_imported_without_lineage_packet.yaml`
  exercise the linked and unknown cases.
- **Template and scaffold artifacts keep known-issue,
  freshness, and reopen-preflight notes explicit.** §4.1
  reserves `known_issue_rows`, `declared_freshness_class`,
  `template_health_check_descriptors`, and
  `reopen_preflight_default_class`. Fixture
  `template_known_issue_active_workaround.yaml` carries a
  known-issue row with workaround; fixture
  `community_template_signature_review_required.yaml`
  carries a freshness chip and reopen-preflight class.

## 10. Worked examples

Each example has a companion fixture under
[`/fixtures/scaffolding/template_cases/`](../../fixtures/scaffolding/template_cases/).

### 10.1 First-party local-only template manifest

A first-party Rust CLI template
(`template_archetype_class = cli_tool`,
`template_source_class = first_party`,
`support_class = officially_supported`,
`runtime_and_toolchain_scope = local_only`,
`template_trust_posture_class =
workspace_trust_required_before_post_create_actions`,
`template_egress_posture_class =
no_network_egress_required` because the manifest emits no
`package_restore_for_ecosystem` task at scaffold time —
package restore happens after open via the standard package
flow). Generated files cover Cargo manifests, sources,
license, and a JSONC settings overlay. Declared hooks: post-
write format and post-create handoff print only. See
[`first_party_local_template_manifest.yaml`](../../fixtures/scaffolding/template_cases/first_party_local_template_manifest.yaml).

### 10.2 Dry-run preview for a first-party local template

Scaffold-run record carrying
`scaffold_run_outcome_class =
dry_run_preview_only_no_files_written` against the §10.1
manifest, with `scaffold_dry_run_posture_class =
dry_run_required_before_apply` and a non-null
`dry_run_preview_ref`. See
[`dry_run_preview_first_party_local_template.yaml`](../../fixtures/scaffolding/template_cases/dry_run_preview_first_party_local_template.yaml).

### 10.3 Partial apply with rollback checkpoint

Scaffold-run record where post-create
`package_restore_for_ecosystem` failed, the runner rolled
back the partially-applied workspace via lockfile checkpoint,
and the record carries `scaffold_run_outcome_class =
rolled_back_after_partial_apply` plus a non-null
`rollback_checkpoint_ref`. See
[`partial_apply_rollback.yaml`](../../fixtures/scaffolding/template_cases/partial_apply_rollback.yaml).

### 10.4 Denial: undeclared hook attempted

Scaffold-run record where a runner tried to invoke a hook id
not present in the manifest's `declared_hooks`. Outcome
resolves to `denied_pre_apply_undeclared_hook_or_task` with
`denial_reason_class =
undeclared_hook_or_setup_task_must_not_run`. See
[`denied_undeclared_hook_run.yaml`](../../fixtures/scaffolding/template_cases/denied_undeclared_hook_run.yaml).

### 10.5 Generated-project lineage linked to a template
revision

`generated_project_lineage_record` for a workspace produced
by §10.1, with
`generated_project_lineage_state_class =
linked_to_template_revision`,
`generated_project_update_or_rebase_state_class =
in_sync_with_bound_template_revision`,
`template_reopen_preflight_class =
reopen_preflight_in_sync_admissible`, and a non-null
`bound_template_revision_ref`. See
[`lineage_linked_to_template_revision.yaml`](../../fixtures/scaffolding/template_cases/lineage_linked_to_template_revision.yaml).

### 10.6 Lineage unknown after import without lineage packet

`generated_project_lineage_record` for a workspace re-
imported through a portable profile that did not carry the
lineage packet, with
`generated_project_lineage_state_class =
lineage_unknown_imported_without_lineage_packet`,
`generated_project_update_or_rebase_state_class =
no_update_path_unlinked_or_lineage_unknown`,
`template_reopen_preflight_class =
reopen_preflight_lineage_unknown_imported_without_lineage_packet`,
and a null `bound_template_revision_ref`. See
[`lineage_unknown_imported_without_lineage_packet.yaml`](../../fixtures/scaffolding/template_cases/lineage_unknown_imported_without_lineage_packet.yaml).

### 10.7 Community template requiring signature review

`template_manifest_record` with
`template_source_class = community`,
`signer_continuity_class = signer_changed_review_required`,
and an `availability_narrowing_class =
signature_review_required` row in the gallery surface that
reads it. The manifest also carries
`template_egress_posture_class =
egress_to_community_origin_user_review_required` so the
preflight summary surfaces the egress chip. See
[`community_template_signature_review_required.yaml`](../../fixtures/scaffolding/template_cases/community_template_signature_review_required.yaml).

### 10.8 Template with active known issue and workaround

`template_manifest_record` carrying a `known_issue_rows[]`
entry with
`template_known_issue_class =
known_issue_active_workaround_documented`,
a non-null `workaround_summary_ref`, and a non-null
`affected_template_revision_ref_set`. The post-create
handoff surface reads this row so the user sees the
known-issue chip without leaving the surface. See
[`template_known_issue_active_workaround.yaml`](../../fixtures/scaffolding/template_cases/template_known_issue_active_workaround.yaml).

### 10.9 AI-proposed template pending review

`template_manifest_record` with
`template_lifecycle_class =
ai_tool_proposed_pending_review` and
`template_trust_posture_class =
ai_tool_proposed_template_pending_review`. The scaffold-run
record fixture
[`ai_tool_proposed_run_pending_admission.yaml`](../../fixtures/scaffolding/template_cases/ai_tool_proposed_run_pending_admission.yaml)
shows the matching run record with
`scaffold_run_outcome_class =
ai_tool_proposed_run_pending_admission` and a non-null
`ai_tool_proposed_run_review_ticket_ref`.

## 11. Changing this contract

- **Additive-minor** changes (new
  `template_archetype_class`, new
  `supported_ecosystem_class`, new
  `supported_platform_class`, new
  `required_parameter_kind_class`, new
  `generated_file_class`, new `declared_hook_class`, new
  `declared_setup_task_class`, new
  `template_trust_posture_class`, new
  `template_egress_posture_class`, new
  `template_known_issue_class`, new
  `template_reopen_preflight_class`, new
  `scaffold_run_outcome_class`, new
  `scaffold_dry_run_posture_class`, new
  `generated_project_lineage_state_class`, new
  `generated_project_update_or_rebase_state_class`, new
  `delete_generated_output_recovery_route_class`, new
  `denial_reason_class` value) land here, in the companion
  schemas, and in at least one fixture in the same change.
  Adding a value bumps the relevant `*_schema_version` const.
- **Repurposing** an existing vocabulary value is breaking
  and opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (UX template-and-prebuild
  contract, recipe / macro contract, structured-artifact
  diff / merge / review seed, artifact-format / migration
  policy) happen at source and this contract re-exports by
  reference; it MUST NOT shadow the change.

## 12. Linked artifacts

- UX template-and-prebuild gallery / picker / preflight /
  post-create / health / policy contract:
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Automation recipe / macro contract (capability-declaration,
  trust / policy / approval / dry-run / preview posture,
  deferred-intent / queueable-action / reconciliation-intent
  vocabulary, signature / redaction / export posture this
  contract re-exports):
  [`/docs/automation/recipe_and_macro_contract.md`](../automation/recipe_and_macro_contract.md).
- Recipe-manifest schema (boundary the
  `post_create_invoke_declarative_recipe` and
  `post_create_invoke_recorded_macro_*` hook classes resolve
  recipes / macros against):
  [`/schemas/automation/recipe_manifest.schema.json`](../../schemas/automation/recipe_manifest.schema.json).
- Run-record schema (boundary the
  `dispatched_scaffold_run_id_ref` slot composes with for
  hook / setup-task invocations):
  [`/schemas/automation/run_record.schema.json`](../../schemas/automation/run_record.schema.json).
- Artifact-format and comment-preservation policy:
  [`/docs/config/artifact_format_and_migration_policy.md`](../config/artifact_format_and_migration_policy.md).
- Structured-artifact diff / merge / review seed (back-link
  registry the
  `generated_only_artifact_back_linked_to_canonical_source`
  generated-file class resolves):
  [`/docs/review/structured_artifact_review_seed.md`](../review/structured_artifact_review_seed.md).
- Worked-example fixtures:
  [`/fixtures/scaffolding/template_cases/`](../../fixtures/scaffolding/template_cases/).
