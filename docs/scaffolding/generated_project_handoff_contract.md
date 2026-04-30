# Generated-project handoff, recovery, and create-without-starter contract

This document freezes the cross-tool contract every **post-create
handoff sheet**, **reopen-preflight surface**, and
**delete-generated-output flow** inherits before scaffold
generators, post-create renderers, recent-work / restore cards,
support / export readers, and doctor surfaces harden into the
Aureline shell. The goal is to make post-create truth honest and
recoverable: a generated project never strands the user after a
partial or failed setup, every materialised file has a tracked
recovery posture (not a stray file with no lineage), and the
plain create-without-starter route remains at equal weight on
every health state, every disposition, and every blocker.

The companion schemas live at:

- [`/schemas/scaffolding/post_create_handoff.schema.json`](../../schemas/scaffolding/post_create_handoff.schema.json)
- [`/schemas/scaffolding/generated_project_recovery.schema.json`](../../schemas/scaffolding/generated_project_recovery.schema.json)

The companion fixture corpus lives under:

- [`/fixtures/scaffolding/generated_project_handoff_cases/`](../../fixtures/scaffolding/generated_project_handoff_cases/)

This contract is normative for the post-create handoff, reopen-
preflight, and delete-generated-output record shapes. Where it
disagrees with the PRD, TAD, TDD, UI/UX spec, or milestone
document, those sources win and this document plus its companion
schemas and fixtures update in the same change. Where a downstream
surface (post-create handoff renderer, recent-work / restore
card, doctor surface, support / export reader, delete-recovery
flow) mints a parallel handoff / recovery / delete vocabulary,
this contract wins and the surface is non-conforming.

This contract mints **no** new `template_source_class`,
`support_class`, `runtime_and_toolchain_scope`,
`template_lifecycle_class`, `declared_freshness_class`,
`starter_setup_cost_class`, `template_availability_narrowing_class`,
`bypass_path_id`, `template_health_state_class`,
`host_boundary_class`, `signing_or_trust_badge_class`,
`required_extension_install_class`,
`required_remote_provisioning_class`,
`required_managed_service_class`,
`required_credential_provisioning_class`,
`card_disposition_class`, `preflight_axis_class`,
`file_write_disposition_class`, `dependency_impact_class`,
`lockfile_mutation_class`, `execution_phase_class`,
`immediate_or_deferred_action_class`, `checkpoint_kind_class`,
`environment_use_kind_class`, `preflight_disposition_class`,
`generated_file_class`, `declared_hook_class`,
`declared_setup_task_class`, `template_trust_posture_class`,
`template_egress_posture_class`, `template_known_issue_class`,
`template_reopen_preflight_class`,
`scaffold_run_outcome_class`, `scaffold_dry_run_posture_class`,
`generated_project_lineage_state_class`,
`generated_project_update_or_rebase_state_class`, or
`delete_generated_output_recovery_route_class` values. Every row
here re-exports — by reference — the vocabularies frozen in:

- [`/docs/scaffolding/template_and_scaffold_contract.md`](./template_and_scaffold_contract.md)
  (§3.1–§3.16) — manifest / scaffold-run / lineage closed sets the
  handoff and recovery records compose with.
- [`/docs/scaffolding/template_health_and_preflight_contract.md`](./template_health_and_preflight_contract.md)
  (§3.1–§3.8) — card / preflight / health-state vocabulary the
  handoff and recovery records carry forward.
- [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  (§3.9 bypass-path, §3.13 post-create-handoff-axis, §8 post-
  create handoff summary record) — the upstream UX vocabulary
  this contract specialises with mechanical schema gates.

This contract introduces **eight** new closed sets scoped to the
post-create handoff, reopen-preflight, and delete-generated-output
records.

## Who reads this contract

- **Post-create handoff renderers** wiring the sheet a user sees
  after Generate completes (or after generation fails / is
  blocked / is bypassed). The sheet enumerates the per-action
  setup-task / hook disposition, the typed `Run now`,
  `Run later`, `Review files`, `Open manifest`, and
  `Delete generated output` actions, the recovery routes, and
  the create-without-starter bypass routes — all preserved at
  equal weight on every health state.
- **Reopen-preflight surfaces** (recent-work / restore card,
  workspace switcher, doctor) resolving `do we know enough to
  reopen this generated project?` after a previously-created
  workspace re-enters via a fresh session, an export / import
  round-trip, a policy or environment change, a workspace-trust
  revocation, a target-runtime change, or an AI-proposed relink
  pending review.
- **Delete-generated-output flows** (post-create handoff sheet,
  recent-work / restore card, doctor surface, support / export
  reader) resolving the typed recovery route and the typed
  provenance preservation class so deleted output never leaves
  stray files with no tracked lineage.
- **Docs, support, compatibility, and measurement authors**
  attributing handoff, reopen, and delete evidence to the same
  record kinds the shell renders.

## 1. Scope

- Freeze one `post_create_handoff_record` per completion of a
  starter-driven scaffold flow (whether the run applied all
  files, applied with partial post-create success, failed during
  post-create, was bypassed without materialising a workspace,
  was blocked by post-create policy or workspace trust, or is
  queued pending an AI-proposed-run admission). The record is
  the on-disk projection that the post-create sheet, the
  recent-work / restore card, and the support / export reader
  resolve against; the bound `scaffold_run_record` (schemas/
  scaffolding/scaffold_run.schema.json) remains the source of
  truth for run outcome, applied files, invoked hooks, and
  invoked setup tasks.
- Freeze one `reopen_preflight_record` per reopen of a
  generated workspace. The record enumerates the typed
  blocker class set, the carried lineage state, the carried
  update / rebase state, the carried template-reopen-preflight
  class, and the recovery routes available — never collapsed
  into a generic "cannot open" label.
- Freeze one `delete_generated_output_record` per Delete-
  generated-output flow. The record enumerates the typed
  recovery route, the typed provenance preservation class, the
  bound rollback checkpoint (when present), the bound
  regenerate-target template revision (when applicable), and
  the create-without-starter bypass routes.
- Freeze the closed vocabularies (§3) bounding handoff
  disposition, setup outcome, setup-action class, setup-action
  disposition, handoff-action class, recovery-route class,
  provenance-anchor class, reopen-preflight disposition,
  reopen-preflight blocker class, delete-generated-output
  disposition, delete-provenance preservation, and the closed
  denial-reason vocabulary every record resolves against.
- Out of scope: implementing the post-create renderer, the
  recent-work / restore card, the doctor surface, the support
  / export reader, or the per-ecosystem package adapters. The
  contract pins the closed sets and record shapes those
  surfaces resolve against.

## 2. Out of scope

- Implementing the scaffold generator, the post-create renderer,
  the recent-work / restore card, the doctor surface, the
  support / export reader, the workspace switcher, or the
  delete-generated-output recovery flow. This contract pins the
  closed sets and record shapes those surfaces resolve against.
- Implementing per-ecosystem package adapters (Cargo, npm /
  pnpm / yarn, pip / uv / poetry, Go modules, Maven / Gradle,
  RubyGems / Bundler, NuGet, system-package adapters).
- The prebuild service, warm-start backend, or managed-cloud
  provisioning path.
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
  `template_health_check_class`, `post_create_handoff_axis`,
  `policy_notice_class` — UX template-and-prebuild
  contract §3.2–§3.14.
- `template_archetype_class`, `supported_ecosystem_class`,
  `supported_platform_class`, `required_parameter_kind_class`,
  `generated_file_class`, `declared_hook_class`,
  `declared_setup_task_class`, `template_trust_posture_class`,
  `template_egress_posture_class`, `template_known_issue_class`,
  `template_reopen_preflight_class`,
  `scaffold_run_outcome_class`, `scaffold_dry_run_posture_class`,
  `generated_project_lineage_state_class`,
  `generated_project_update_or_rebase_state_class`,
  `delete_generated_output_recovery_route_class` — scaffolding
  template-and-scaffold contract §3.1–§3.16.
- `template_health_state_class`, `host_boundary_class`,
  `signing_or_trust_badge_class`,
  `required_network_egress_class`,
  `required_extension_install_class`,
  `required_remote_provisioning_class`,
  `required_managed_service_class`,
  `required_credential_provisioning_class`,
  `card_disposition_class`, `preflight_axis_class`,
  `file_write_disposition_class`, `dependency_impact_class`,
  `lockfile_mutation_class`, `execution_phase_class`,
  `immediate_or_deferred_action_class`, `checkpoint_kind_class`,
  `environment_use_kind_class`, `preflight_disposition_class` —
  scaffolding template-health-and-preflight contract §3.1–§3.8.
- `signature_class`, `redaction_class` — recipe / macro contract.
- `signer_continuity_class` — source-acquisition / bootstrap
  seed §1.5.

This contract introduces eight new closed sets:

### 3.1 `post_create_handoff_disposition_class`

Closed disposition for the post-create handoff sheet. Drives
whether the sheet's primary action is `Run now` / `Run later` /
`Review files only`, or whether the sheet routes through a
recovery / policy-review / trust-required / AI-admission path:

- `handoff_admissible_run_now_offered`
- `handoff_admissible_run_later_offered`
- `handoff_review_files_only`
- `handoff_partial_success_recovery_offered`
- `handoff_failure_recovery_required`
- `handoff_policy_blocked_review_required`
- `handoff_blocked_workspace_trust_required`
- `handoff_blocked_signature_review_required`
- `handoff_bypass_taken_no_workspace_materialised`
- `handoff_ai_proposed_pending_admission`
- `handoff_disposition_class_unknown_requires_review`

Rules:

1. Every `post_create_handoff_record` names exactly one
   `post_create_handoff_disposition_class`.
2. The bypass routes (`create_without_starter_route_ids`) MUST
   remain at equal weight in every disposition. A handoff that
   hides the bypass under any disposition denies with
   `create_without_starter_route_must_remain_at_equal_weight`.
3. `handoff_partial_success_recovery_offered` MUST cite
   `setup_outcome_class = partial_success`,
   `denial_reason_class = partial_apply_recovery_required`,
   and a non-null `rollback_checkpoint_ref`.
4. `handoff_failure_recovery_required` MUST cite
   `setup_outcome_class = failed`,
   `denial_reason_class = failed_apply_recovery_required`, and
   a non-null `rollback_checkpoint_ref`.
5. `handoff_policy_blocked_review_required` MUST cite
   `denial_reason_class = policy_blocked_post_create_action`.
6. `handoff_blocked_workspace_trust_required` MUST cite
   `denial_reason_class = workspace_trust_unset_or_restricted`.
7. `handoff_blocked_signature_review_required` MUST cite
   `denial_reason_class =
   signature_review_required_user_review_required`.
8. `handoff_bypass_taken_no_workspace_materialised` MUST resolve
   `setup_outcome_class =
   bypass_taken_no_workspace_materialised`,
   `created_workspace_lineage_record_ref = null`,
   `provenance_anchor_class =
   bypass_anchor_no_workspace_materialised`, and
   `delete_generated_output_recovery_route_class =
   delete_with_no_lineage_packet_local_history_only_review_required`
   (no files were materialised, so there is nothing to delete
   with a real recovery route).
9. `handoff_ai_proposed_pending_admission` MUST resolve
   `template_health_state_class_carried =
   ai_tool_proposed_pending_review`,
   `setup_outcome_class = pending_ai_admission`,
   `denial_reason_class =
   ai_tool_proposed_run_must_not_apply_pending_admission`, and
   a non-null `ai_tool_proposed_review_ticket_ref`.

### 3.2 `setup_outcome_class`

Closed roll-up of the per-action setup outcome. Mirrors the UX
template-and-prebuild contract §3.13 `setup_outcome_summary`
closed subset and extends it so the post-create handoff sheet
never collapses bypass / deferred / pending-AI states into a
generic `setup_succeeded` label:

- `all_actions_succeeded`
- `partial_success`
- `failed`
- `bypass_taken_no_workspace_materialised`
- `deferred_user_must_invoke`
- `pending_ai_admission`
- `setup_outcome_class_unknown_requires_review`

Rules:

1. Every `post_create_handoff_record` names exactly one
   `setup_outcome_class`.
2. `partial_success` and `failed` MUST cite a non-null
   `rollback_checkpoint_ref` (the runner planted a rollback
   checkpoint before apply; the post-create surface preserves
   the handle so recovery is admissible without re-deriving it).
3. `partial_success` and `failed` MUST cite a
   `setup_action_disposition_set` that contains at least one
   entry whose `setup_action_disposition_class` is one of
   `partially_applied_user_review_required`,
   `failed_user_review_required`, `blocked_*`, or
   `pending_user_invoke`. A handoff that says "partial success"
   without naming the failing or blocked action is non-
   conforming.

### 3.3 `setup_action_class` and `setup_action_disposition_class`

Closed class telling the handoff sheet whether each per-action
row is required, optional, deferred, policy-blocked, or
bypassed. A handoff that collapses required-vs-optional setup
tasks is non-conforming.

`setup_action_class`:

- `required_setup_action`
- `optional_setup_action`
- `deferred_setup_action_user_must_invoke`
- `policy_blocked_setup_action`
- `bypassed_setup_action`
- `setup_action_class_unknown_requires_review`

Closed per-action disposition. Each declared hook / setup task
that the scaffold runner attempted resolves to exactly one of
these. Free-form 'failed for unknown reason' verbs are non-
conforming.

`setup_action_disposition_class`:

- `succeeded_immediate`
- `succeeded_after_retry`
- `skipped_under_bypass_path`
- `skipped_user_deferred`
- `skipped_optional_not_needed`
- `partially_applied_user_review_required`
- `failed_user_review_required`
- `blocked_workspace_trust_required`
- `blocked_policy_narrowed`
- `blocked_signature_review_required`
- `blocked_target_runtime_unavailable`
- `blocked_network_unavailable`
- `blocked_secret_broker_handle_unavailable`
- `blocked_managed_workspace_envelope_unavailable`
- `blocked_missing_required_dependency`
- `pending_user_invoke`
- `pending_ai_admission`
- `setup_action_disposition_class_unknown_requires_review`

Rules:

1. Every `setup_action_disposition_descriptor` cites exactly one
   `setup_action_class` and exactly one
   `setup_action_disposition_class`.
2. Every descriptor's `setup_action_id_ref` MUST resolve through
   the bound `template_manifest_record`'s `declared_hooks` /
   `declared_setup_tasks` arrays (the manifest is the source of
   truth). A descriptor that resolves to an undeclared hook or
   setup task denies with
   `undeclared_hook_or_setup_task_must_not_run` (re-exported
   from the scaffold-run / generation-preflight contracts).
3. `bypassed_setup_action` MUST pair with
   `setup_action_disposition_class =
   skipped_under_bypass_path`; a bypassed action that claims
   `succeeded_immediate` is non-conforming.
4. `policy_blocked_setup_action` MUST pair with one of the
   `blocked_*` dispositions.

### 3.4 `handoff_action_class`

Closed set of actions the post-create handoff sheet MAY render.
The sheet MUST always render `review_files_action` and
`open_manifest_action` when files were materialised (so the
user can audit before continuing), and MUST always render at
least one `bypass_open_without_starter_action` so the create-
without-starter route remains at equal weight:

- `run_now_action`
- `run_later_action`
- `review_files_action`
- `open_manifest_action`
- `delete_generated_output_action`
- `rollback_to_checkpoint_action`
- `regenerate_from_template_action`
- `local_only_continuation_action`
- `report_to_template_owner_action`
- `bypass_open_without_starter_action`
- `handoff_action_class_unknown_requires_review`

Rules:

1. Every `post_create_handoff_record`'s `handoff_action_set`
   contains at least one `bypass_open_without_starter_action`.
   The schema's allOf gate enforces this with `contains`.
2. `delete_generated_output_action` MUST cite a non-null
   `handoff_action_handle_ref` resolving through
   [`/schemas/scaffolding/generated_project_recovery.schema.json`](../../schemas/scaffolding/generated_project_recovery.schema.json)
   to a `delete_generated_output_record`.
3. `rollback_to_checkpoint_action` MUST cite a non-null
   `handoff_action_handle_ref` resolving to the rollback
   checkpoint handle.
4. `regenerate_from_template_action` MUST cite a non-null
   `handoff_action_handle_ref` resolving to the target template
   revision.

### 3.5 `recovery_route_class`

Closed set of recovery routes the handoff sheet, the reopen-
preflight surface, and the delete-generated-output flow MAY
enumerate:

- `no_recovery_required_succeeded`
- `recovery_via_run_later_actions`
- `recovery_via_rollback_to_checkpoint`
- `recovery_via_regenerate_from_template`
- `recovery_via_review_files_then_continue`
- `recovery_via_open_manifest_review`
- `recovery_via_delete_generated_output`
- `recovery_via_local_only_continuation`
- `recovery_via_report_to_template_owner`
- `recovery_via_bypass_to_open_without_starter`
- `recovery_via_request_workspace_trust_grant`
- `recovery_via_request_policy_review`
- `recovery_via_admit_ai_proposed_run`
- `recovery_route_class_unknown_requires_review`

Rules:

1. Every `post_create_handoff_record`,
   `reopen_preflight_record`, and `delete_generated_output_record`
   names a non-empty `recovery_route_class_set` (when the field
   is reserved on that record kind). A handoff that resolves to
   failure / partial-success / policy-blocked without
   enumerating a recovery route is non-conforming.
2. Success outcomes name `no_recovery_required_succeeded` plus
   at least one continue route (typically
   `recovery_via_run_later_actions`,
   `recovery_via_review_files_then_continue`, or
   `recovery_via_open_manifest_review`).
3. Every record's recovery-route set MUST include
   `recovery_via_bypass_to_open_without_starter` when any
   bypass route is rendered, so the open-without-starter route
   is reachable from the recovery surface itself.

### 3.6 `provenance_anchor_class`

Closed set naming what survives a delete-generated-output flow
as on-disk provenance (or, on bypass-taken, names that no
workspace was materialised). `lineage_metadata_file_under_vcs`
is the on-disk projection of `generated_project_lineage_record`
and is the preferred anchor:

- `lineage_metadata_file_under_vcs`
- `rollback_checkpoint_handle_only`
- `workspace_snapshot_handle_only`
- `local_history_breadcrumb_only`
- `managed_workspace_envelope_anchor`
- `bypass_anchor_no_workspace_materialised`
- `provenance_anchor_class_unknown_requires_review`

Rules:

1. Every `post_create_handoff_record` and every
   `delete_generated_output_record` names exactly one
   `provenance_anchor_class`.
2. A handoff that materialised files MUST NOT cite
   `bypass_anchor_no_workspace_materialised`; the schema's
   allOf gate enforces this.
3. A handoff or delete record that hides lineage metadata in a
   binary blob, an opaque sqlite database, or an Aureline-only
   RPC store denies with
   `lineage_metadata_must_be_plain_reviewable_file` (re-exported
   from the scaffold-run / generation-preflight contracts).

### 3.7 `reopen_preflight_disposition_class` and `reopen_preflight_blocker_class`

Closed disposition for the reopen-preflight sheet:

- `reopen_preflight_admitted_in_sync`
- `reopen_preflight_admitted_with_update_available`
- `reopen_preflight_admitted_with_local_overrides`
- `reopen_preflight_admitted_after_breaking_rebase_review`
- `reopen_preflight_blocked_template_revision_archived`
- `reopen_preflight_blocked_lineage_unknown_user_review_required`
- `reopen_preflight_blocked_workspace_trust_required`
- `reopen_preflight_blocked_policy_changed_pending_user_admit`
- `reopen_preflight_blocked_environment_changed_user_review_required`
- `reopen_preflight_blocked_missing_dependencies_user_review_required`
- `reopen_preflight_blocked_drifted_from_template_user_review_required`
- `reopen_preflight_blocked_target_runtime_unavailable`
- `reopen_preflight_blocked_signature_review_required`
- `reopen_preflight_pending_ai_admission`
- `reopen_preflight_disposition_class_unknown_requires_review`

Closed per-blocker class:

- `no_blocker_admissible`
- `policy_changed_after_create`
- `workspace_trust_revoked_after_create`
- `target_runtime_unavailable_after_create`
- `network_unavailable_after_create`
- `mirror_or_origin_unreachable_after_create`
- `signature_review_required_after_create`
- `template_revision_archived_after_create`
- `template_revision_unverifiable_user_review_required`
- `lineage_packet_missing_after_import`
- `local_divergence_drift_user_review_required`
- `missing_required_dependencies_after_create`
- `secret_broker_handle_revoked_after_create`
- `managed_workspace_envelope_revoked_after_create`
- `ai_tool_proposed_relink_pending_review`
- `reopen_preflight_blocker_class_unknown_requires_review`

Rules:

1. Every `reopen_preflight_record` names exactly one
   `reopen_preflight_disposition_class` and a non-empty
   `reopen_preflight_blocker_class_set`.
2. Admitted dispositions cite the singleton `no_blocker_admissible`
   and `denial_reason_class = no_denial_recovery_admissible`.
3. Blocked / pending dispositions cite at least one typed blocker
   class (other than `no_blocker_admissible`) and a denial reason
   other than `no_denial_recovery_admissible`.
4. Each blocked disposition forces a typed blocker / denial
   pairing (§3.7 schema gates):
   - `reopen_preflight_blocked_template_revision_archived` ↔
     `template_revision_archived_after_create` blocker +
     `template_revision_archived_no_apply_path` denial.
   - `reopen_preflight_blocked_lineage_unknown_user_review_required`
     ↔ `lineage_packet_missing_after_import` blocker +
     `lineage_unknown_imported_without_lineage_packet` lineage
     state + `reopen_preflight_lineage_unknown_imported_without_lineage_packet`
     reopen-preflight class + `lineage_packet_missing_after_import`
     denial.
   - `reopen_preflight_blocked_workspace_trust_required` ↔
     `workspace_trust_revoked_after_create` blocker +
     `workspace_trust_unset_or_restricted` denial.
   - `reopen_preflight_blocked_policy_changed_pending_user_admit`
     ↔ `policy_changed_after_create` blocker +
     `policy_changed_after_create_user_must_admit` denial.
   - `reopen_preflight_blocked_missing_dependencies_user_review_required`
     ↔ `missing_required_dependencies_after_create` blocker +
     denial.
   - `reopen_preflight_blocked_drifted_from_template_user_review_required`
     ↔ `local_divergence_drift_user_review_required` blocker +
     `drifted_from_template_revision_user_review_required`
     lineage state + denial.
   - `reopen_preflight_blocked_target_runtime_unavailable` ↔
     `target_runtime_unavailable_after_create` blocker + denial.
   - `reopen_preflight_blocked_signature_review_required` ↔
     `signature_review_required_after_create` blocker +
     `signature_review_required_user_review_required` denial.
   - `reopen_preflight_pending_ai_admission` ↔
     `ai_tool_proposed_relink_pending_review` blocker +
     `ai_tool_proposed_relink_pending_review` lineage state +
     `ai_tool_proposed_relink_must_not_apply_pending_admission`
     denial + non-null `ai_tool_proposed_review_ticket_ref`.

### 3.8 `delete_generated_output_disposition_class` and `delete_provenance_preservation_class`

Closed disposition for a Delete-generated-output flow:

- `delete_admissible_lockfile_checkpoint_recovery`
- `delete_admissible_workspace_snapshot_recovery`
- `delete_admissible_local_history_recovery_only`
- `delete_admissible_regenerate_from_template_recovery`
- `delete_admissible_no_lineage_packet_local_history_only_review_required`
- `delete_blocked_unrecoverable_user_review_required`
- `delete_blocked_workspace_trust_required`
- `delete_blocked_policy_narrowed`
- `delete_blocked_lineage_packet_locked`
- `delete_pending_ai_admission`
- `delete_disposition_class_unknown_requires_review`

Closed class naming what survives the delete as on-disk
provenance:

- `provenance_preserved_via_lineage_packet_export`
- `provenance_preserved_via_rollback_checkpoint`
- `provenance_preserved_via_workspace_snapshot`
- `provenance_preserved_via_local_history_only`
- `provenance_preserved_via_regenerate_from_template_only`
- `provenance_preserved_via_managed_workspace_envelope`
- `provenance_not_preserved_user_review_required`
- `provenance_preservation_class_unknown_requires_review`

Rules:

1. Every `delete_generated_output_record` names exactly one
   `delete_generated_output_disposition_class` and exactly one
   `delete_provenance_preservation_class`.
2. Each admissible disposition forces a typed recovery / preservation
   / denial pairing (§3.8 schema gates):
   - `delete_admissible_lockfile_checkpoint_recovery` ↔
     `delete_with_lockfile_checkpoint_recovery` route +
     `provenance_preserved_via_rollback_checkpoint` preservation
     + non-null `rollback_checkpoint_ref`.
   - `delete_admissible_workspace_snapshot_recovery` ↔
     `delete_with_workspace_snapshot_recovery` route +
     `provenance_preserved_via_workspace_snapshot` preservation.
   - `delete_admissible_local_history_recovery_only` ↔
     `delete_with_local_history_recovery_only` route +
     `provenance_preserved_via_local_history_only` preservation.
   - `delete_admissible_regenerate_from_template_recovery` ↔
     `delete_with_regenerate_from_template_recovery` route +
     `provenance_preserved_via_regenerate_from_template_only`
     preservation + non-null
     `regenerate_target_template_revision_ref`.
   - `delete_admissible_no_lineage_packet_local_history_only_review_required`
     ↔ `delete_with_no_lineage_packet_local_history_only_review_required`
     route + `provenance_preserved_via_local_history_only` or
     `provenance_not_preserved_user_review_required` preservation
     + `lineage_packet_missing_after_import` denial.
3. `delete_blocked_unrecoverable_user_review_required` MUST cite
   `delete_unrecoverable_user_review_required` recovery route,
   `provenance_not_preserved_user_review_required` preservation,
   and `unrecoverable_delete_user_review_required` denial. A
   record that quietly admits an unrecoverable delete is non-
   conforming.
4. `delete_pending_ai_admission` MUST cite
   `ai_tool_proposed_relink_must_not_apply_pending_admission`
   denial and a non-null `ai_tool_proposed_review_ticket_ref`.
5. Every admissible disposition MUST cite a non-null
   `delete_generated_output_recovery_route_ref` so the per-action
   handle resolves.

## 4. `post_create_handoff_record`

Every completion of a starter-driven scaffold flow publishes
exactly one `post_create_handoff_record`.

### 4.1 Required fields

- `record_kind = post_create_handoff_record`.
- `post_create_handoff_schema_version` (integer, const 1).
- `handoff_id` (opaque).
- `bound_scaffold_run_id_ref` (opaque) — resolves through
  [`/schemas/scaffolding/scaffold_run.schema.json`](../../schemas/scaffolding/scaffold_run.schema.json);
  the scaffold-run record is the source of truth for run
  outcome, applied files, invoked hooks, and invoked setup
  tasks.
- `bound_template_card_record_ref`,
  `bound_template_manifest_record_ref`,
  `bound_template_id_ref`, `bound_template_revision_ref`.
- `created_workspace_lineage_record_ref` (opaque) — required
  (non-null) on every outcome that materialised files; null on
  `bypass_taken_no_workspace_materialised`.
- `post_create_handoff_disposition_class` (§3.1).
- `setup_outcome_class` (§3.2).
- `setup_action_disposition_set[]` — array of per-action
  descriptors (§3.3). May be empty only on
  `bypass_taken_no_workspace_materialised`.
- `handoff_action_set[]` — non-empty array of
  `handoff_action_descriptor` (§3.4); MUST contain at least
  one `bypass_open_without_starter_action`.
- `recovery_route_class_set[]` — non-empty array of recovery
  routes (§3.5).
- `create_without_starter_route_ids[]` — non-empty
  `bypass_path_id` set.
- `delete_generated_output_recovery_route_class` (re-exported
  from the scaffold-run contract §3.16).
- `delete_generated_output_recovery_route_ref` — required
  (non-null) when the recovery class is anything other than
  `delete_unrecoverable_user_review_required`.
- `provenance_anchor_class` (§3.6).
- `rollback_checkpoint_ref` — required (non-null) on
  partial-success / failure dispositions; otherwise admissible
  to be null.
- `denial_reason_class` (§5.1).
- `template_health_state_class_carried` (re-exported from the
  card / preflight contract).
- `ai_tool_proposed_review_ticket_ref` — required when the
  disposition is `handoff_ai_proposed_pending_admission`.
- `summary` (optional reviewable sentence).
- `notes_ref` (optional opaque ref).
- `minted_at` — monotonic timestamp.

### 4.2 Handoff invariants (allOf gates)

1. **Bypass at equal weight on every disposition.** §3.4 rule 1.
   The schema's `contains` gate forces
   `bypass_open_without_starter_action` into the
   `handoff_action_set` regardless of disposition.
2. **Materialised files anchor lineage.** When
   `setup_outcome_class ∈ {all_actions_succeeded,
   partial_success, failed, deferred_user_must_invoke}`,
   `created_workspace_lineage_record_ref` MUST be non-null and
   `provenance_anchor_class` MUST NOT be
   `bypass_anchor_no_workspace_materialised`.
3. **Partial-success and failure carry recovery handles.**
   §3.1 rules 3–4. The schema forces
   `rollback_checkpoint_ref` non-null on these dispositions.
4. **Bypass-taken disclaims workspace identity.** §3.1 rule 8.
5. **Unrecoverable delete blocks admit.** When
   `delete_generated_output_recovery_route_class =
   delete_unrecoverable_user_review_required`, the disposition
   MUST resolve out of the admissible run-now / run-later /
   review-files set and into a typed recovery / blocked /
   pending class.
6. **AI-proposed admission carries ticket.** §3.1 rule 9.
7. **No undeclared hook or task.** Every entry in
   `setup_action_disposition_set` cites a `setup_action_id_ref`
   resolvable against the bound manifest's declared rows;
   undeclared invocations deny with
   `undeclared_hook_or_setup_task_must_not_run` (re-exported
   from the scaffold-run / generation-preflight contracts).
8. **Health-state agrees with disposition.**
   `template_health_state_class_carried =
   ai_tool_proposed_pending_review` forces the disposition to
   `handoff_ai_proposed_pending_admission`. Other carried
   states do not silently override the disposition; the bound
   scaffold-run outcome remains the source of truth for whether
   files were materialised.

## 5. `reopen_preflight_record` and `delete_generated_output_record`

Every reopen of a generated workspace publishes exactly one
`reopen_preflight_record`. Every Delete-generated-output flow
publishes exactly one `delete_generated_output_record`.

### 5.1 Closed denial-reason vocabulary

Both records resolve `denial_reason_class` against the closed
set:

- `no_denial_recovery_admissible`
- `policy_changed_after_create_user_must_admit`
- `workspace_trust_unset_or_restricted`
- `target_runtime_unavailable_after_create`
- `network_unavailable_after_create`
- `signature_review_required_user_review_required`
- `template_revision_archived_no_apply_path`
- `template_revision_unverifiable_user_review_required`
- `lineage_packet_missing_after_import`
- `local_divergence_drift_user_review_required`
- `missing_required_dependencies_after_create`
- `secret_broker_handle_revoked_after_create`
- `managed_workspace_envelope_revoked_after_create`
- `unrecoverable_delete_user_review_required`
- `provenance_not_preserved_user_review_required`
- `lineage_packet_locked_user_must_unlock`
- `ai_tool_proposed_relink_must_not_apply_pending_admission`
- `lineage_metadata_must_be_plain_reviewable_file`
- `create_without_starter_route_must_remain_at_equal_weight`
- `denial_reason_class_unknown_requires_review`

The post-create handoff resolves `denial_reason_class` against
its own closed vocabulary (§4.1) which overlaps with — but is
not identical to — the recovery-record vocabulary; the handoff
adds `partial_apply_recovery_required`,
`failed_apply_recovery_required`,
`policy_blocked_post_create_action`, and
`provenance_not_preserved_user_review_required`.

### 5.2 `reopen_preflight_record` required fields

- `record_kind = reopen_preflight_record`.
- `reopen_preflight_record_schema_version` (integer, const 1).
- `reopen_preflight_id` (opaque).
- `bound_lineage_record_ref` (opaque) — resolves through
  [`/schemas/scaffolding/scaffold_run.schema.json`](../../schemas/scaffolding/scaffold_run.schema.json).
- `bound_template_id_ref`, `bound_template_revision_ref`
  (may be null).
- `reopen_preflight_disposition_class` (§3.7).
- `reopen_preflight_blocker_class_set[]` (§3.7) — non-empty.
- `generated_project_lineage_state_class_carried` (re-exported).
- `generated_project_update_or_rebase_state_class_carried`
  (re-exported).
- `template_reopen_preflight_class_carried` (re-exported).
- `recovery_route_class_set[]` (§3.5) — non-empty.
- `create_without_starter_route_ids[]` — non-empty.
- `denial_reason_class` (§5.1).
- `ai_tool_proposed_review_ticket_ref` — required when the
  disposition is `reopen_preflight_pending_ai_admission`.
- `summary`, `notes_ref` (optional).
- `minted_at` — monotonic timestamp.

### 5.3 `delete_generated_output_record` required fields

- `record_kind = delete_generated_output_record`.
- `delete_generated_output_schema_version` (integer, const 1).
- `delete_id` (opaque).
- `bound_lineage_record_ref` (opaque).
- `bound_scaffold_run_id_ref` (opaque, may be null when the
  workspace was imported without a lineage packet).
- `delete_generated_output_disposition_class` (§3.8).
- `delete_generated_output_recovery_route_class` (re-exported).
- `delete_generated_output_recovery_route_ref` — required
  (non-null) on every admissible disposition.
- `delete_provenance_preservation_class` (§3.8).
- `provenance_anchor_class` (§3.6).
- `rollback_checkpoint_ref` — required (non-null) on
  `delete_admissible_lockfile_checkpoint_recovery`.
- `regenerate_target_template_revision_ref` — required
  (non-null) on
  `delete_admissible_regenerate_from_template_recovery`.
- `denial_reason_class` (§5.1).
- `create_without_starter_route_ids[]` — non-empty.
- `ai_tool_proposed_review_ticket_ref` — required when the
  disposition is `delete_pending_ai_admission`.
- `summary`, `notes_ref` (optional).
- `minted_at` — monotonic timestamp.

### 5.4 Recovery record invariants (allOf gates)

1. **Bypass at equal weight on every disposition.** Both
   records keep `create_without_starter_route_ids` `minItems:
   1` regardless of disposition.
2. **Admitted reopen pairs with no_blocker_admissible.** §3.7
   rule 2.
3. **Blocked reopen forbids no_blocker_admissible.** §3.7
   rule 3. Each blocked disposition forces a typed blocker /
   denial pairing (§3.7 rule 4).
4. **AI-proposed relink admission carries ticket.** §3.7 rule 4
   (last bullet); §3.8 rule 4.
5. **Unrecoverable delete forbids admission.** §3.8 rule 3.
6. **Recovery route handle non-null on admissible delete.**
   §3.8 rule 5.
7. **Lineage-unknown forces lineage-packet-missing pairing.**
   §3.7 rule 4 (lineage-unknown bullet).
8. **Drift forces drift-pairing.** §3.7 rule 4 (drift bullet).

## 6. Create-without-starter and delete-generated-output continuity rules

The following rules pin the "no hidden Aureline-only state, no
stranded users, no stray files without lineage" property:

1. **Bypass at equal weight on every health state, every
   disposition, every blocker.** Every
   `post_create_handoff_record`, `reopen_preflight_record`, and
   `delete_generated_output_record` keeps
   `create_without_starter_route_ids` `minItems: 1`. A surface
   that hides every bypass route under any state denies with
   `create_without_starter_route_must_remain_at_equal_weight`.
2. **Provenance preserved across delete.** Every admissible
   `delete_generated_output_record` cites a typed
   `delete_provenance_preservation_class` other than
   `provenance_not_preserved_user_review_required` (the latter
   is admissible only on the explicit "no recovery route"
   delete), so deleted output never leaves stray files with no
   tracked recovery posture.
3. **Rollback checkpoint preserved across recovery.** Every
   partial-success or failure handoff cites a non-null
   `rollback_checkpoint_ref`; every
   `delete_admissible_lockfile_checkpoint_recovery` delete cites
   a non-null `rollback_checkpoint_ref`. A surface that
   announces a rollback recovery route without a resolvable
   checkpoint handle is non-conforming.
4. **Lineage packet on disk, plain and reviewable.** The
   `lineage_metadata_file_for_generated_project` generated-file
   class (re-exported from the scaffold-run contract §3.5) is
   the on-disk projection of the
   `generated_project_lineage_record`; the post-create handoff,
   reopen-preflight, and delete-generated-output records cite
   that record by ref so a portable-profile export, a support-
   bundle export, and a re-import round-trip preserve continuity
   byte-for-byte (subject to the redaction class). A handoff
   that hides lineage in a binary blob, an opaque sqlite
   database, or an Aureline-only RPC store denies with
   `lineage_metadata_must_be_plain_reviewable_file`.
5. **Reopenability after policy / environment change.** A
   workspace whose policy epoch expired, whose workspace-trust
   was revoked, whose target runtime became unavailable, whose
   network became unreachable, whose signature review was
   re-armed, or whose required dependencies became missing
   resolves to a `reopen_preflight_blocked_*` disposition with a
   typed blocker and a typed denial — never to a generic "cannot
   open" label. The bypass routes remain available so the user
   can open as a plain folder, plain workspace, plain clone,
   plain create-empty, or continue-without-starter.
6. **Later reopenability after delete.** Every admissible
   `delete_generated_output_record` cites a typed recovery
   posture (lockfile checkpoint, workspace snapshot, local-
   history breadcrumb, or regenerate-from-template). On
   `delete_admissible_regenerate_from_template_recovery`, the
   record cites a non-null
   `regenerate_target_template_revision_ref` so the user can
   regenerate later from the same revision the original run
   resolved against, preserving lineage continuity.

## 7. Acceptance mapping

- **Post-create handoff always exposes a no-surprises recovery
  route after generation or failed setup.** §3.1 rules 3–9 and
  §4.2 rules 3–6 force every disposition to cite a typed
  recovery / denial pairing; the schema's allOf gates force
  `rollback_checkpoint_ref` non-null on partial-success and
  failure dispositions, and force the bypass list non-empty on
  every disposition. Fixtures
  [`handoff_apply_admitted_run_now_offered.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_apply_admitted_run_now_offered.yaml),
  [`handoff_partial_setup_failure_recovery_offered.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_partial_setup_failure_recovery_offered.yaml),
  and
  [`handoff_bypass_taken_no_workspace_materialised.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_bypass_taken_no_workspace_materialised.yaml)
  exercise the disposition matrix.
- **Generated output can be reviewed or deleted with preserved
  provenance rather than leaving stray files with no tracked
  lineage.** §3.6 rules 1–3, §3.8 rule 2 (admissible
  pairings), §6 rule 2 (provenance preserved across delete), and
  §6 rule 4 (lineage packet on disk) pin the invariant. The
  schema forces every admissible
  `delete_generated_output_record` to cite a non-null recovery
  ref and a typed preservation class. Fixture
  [`delete_admissible_lockfile_checkpoint_recovery.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/delete_admissible_lockfile_checkpoint_recovery.yaml)
  exercises the rollback-checkpoint route; fixture
  [`delete_blocked_unrecoverable_user_review_required.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/delete_blocked_unrecoverable_user_review_required.yaml)
  exercises the unrecoverable-delete denial.
- **Fixtures cover successful create, create without starter,
  partial setup failure, and reopen-preflight after a policy
  or environment change.** The fixture corpus under
  [`/fixtures/scaffolding/generated_project_handoff_cases/`](../../fixtures/scaffolding/generated_project_handoff_cases/)
  carries:
  - `handoff_apply_admitted_run_now_offered.yaml` — successful
    create.
  - `handoff_bypass_taken_no_workspace_materialised.yaml` —
    create-without-starter (no workspace materialised; bypass
    routes preserved).
  - `handoff_partial_setup_failure_recovery_offered.yaml` —
    partial setup failure with rollback recovery offered.
  - `reopen_preflight_admitted_in_sync.yaml` — reopen
    in-sync admitted.
  - `reopen_preflight_blocked_policy_changed.yaml` — reopen
    blocked after a policy epoch change.
  - `reopen_preflight_blocked_lineage_unknown_after_import.yaml`
    — reopen blocked after a portable-profile re-import without
    a lineage packet.
  - `delete_admissible_lockfile_checkpoint_recovery.yaml` —
    Delete-generated-output with lockfile-checkpoint recovery.
  - `delete_blocked_unrecoverable_user_review_required.yaml` —
    Delete-generated-output blocked with unrecoverable denial.

## 8. Worked examples

Each example has a companion fixture under
[`/fixtures/scaffolding/generated_project_handoff_cases/`](../../fixtures/scaffolding/generated_project_handoff_cases/).

### 8.1 Successful create — Run now offered

A `post_create_handoff_record` for the first-party Rust CLI
starter (the same template exercised by
[`fixtures/scaffolding/template_preflight_cases/preflight_first_party_local_apply_admitted.yaml`](../../fixtures/scaffolding/template_preflight_cases/preflight_first_party_local_apply_admitted.yaml)).
`post_create_handoff_disposition_class =
handoff_admissible_run_now_offered`,
`setup_outcome_class = all_actions_succeeded`,
`provenance_anchor_class =
lineage_metadata_file_under_vcs`,
`delete_generated_output_recovery_route_class =
delete_with_lockfile_checkpoint_recovery`. Bypass routes
(open folder, create empty workspace) render at equal weight
even on the success path. See
[`handoff_apply_admitted_run_now_offered.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_apply_admitted_run_now_offered.yaml).

### 8.2 Bypass taken — no workspace materialised

A `post_create_handoff_record` where the user took the
create-without-starter route (e.g. `bypass.open_folder_without_starter`).
`post_create_handoff_disposition_class =
handoff_bypass_taken_no_workspace_materialised`,
`setup_outcome_class =
bypass_taken_no_workspace_materialised`,
`created_workspace_lineage_record_ref = null`,
`provenance_anchor_class =
bypass_anchor_no_workspace_materialised`,
`delete_generated_output_recovery_route_class =
delete_with_no_lineage_packet_local_history_only_review_required`.
Setup-action set is empty (no actions ran); the bypass routes
render at equal weight. See
[`handoff_bypass_taken_no_workspace_materialised.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_bypass_taken_no_workspace_materialised.yaml).

### 8.3 Partial setup failure — recovery offered

A `post_create_handoff_record` where post-create
`package_restore_for_ecosystem` failed (the same scenario as
[`fixtures/scaffolding/template_cases/partial_apply_rollback.yaml`](../../fixtures/scaffolding/template_cases/partial_apply_rollback.yaml)).
`post_create_handoff_disposition_class =
handoff_partial_success_recovery_offered`,
`setup_outcome_class = partial_success`,
`rollback_checkpoint_ref` non-null,
`recovery_route_class_set` includes
`recovery_via_rollback_to_checkpoint`,
`recovery_via_review_files_then_continue`, and
`recovery_via_bypass_to_open_without_starter`. The setup-action
set names the failed package-restore action with disposition
`failed_user_review_required`. See
[`handoff_partial_setup_failure_recovery_offered.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/handoff_partial_setup_failure_recovery_offered.yaml).

### 8.4 Reopen-preflight: in-sync admitted

A `reopen_preflight_record` for a workspace produced by §8.1
that is reopening cleanly against the same template revision.
`reopen_preflight_disposition_class =
reopen_preflight_admitted_in_sync`,
`reopen_preflight_blocker_class_set = [no_blocker_admissible]`,
`generated_project_lineage_state_class_carried =
linked_to_template_revision`,
`generated_project_update_or_rebase_state_class_carried =
in_sync_with_bound_template_revision`,
`template_reopen_preflight_class_carried =
reopen_preflight_in_sync_admissible`,
`denial_reason_class = no_denial_recovery_admissible`. See
[`reopen_preflight_admitted_in_sync.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/reopen_preflight_admitted_in_sync.yaml).

### 8.5 Reopen-preflight blocked: policy changed after create

A `reopen_preflight_record` where the workspace's effective
policy epoch advanced after the original create (e.g. the
organisation tightened the connected-provider allowlist).
`reopen_preflight_disposition_class =
reopen_preflight_blocked_policy_changed_pending_user_admit`,
`reopen_preflight_blocker_class_set` contains
`policy_changed_after_create`,
`denial_reason_class =
policy_changed_after_create_user_must_admit`. The bypass routes
remain at equal weight; the recovery routes name
`recovery_via_request_policy_review` and
`recovery_via_bypass_to_open_without_starter`. See
[`reopen_preflight_blocked_policy_changed.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/reopen_preflight_blocked_policy_changed.yaml).

### 8.6 Reopen-preflight blocked: lineage unknown after import

A `reopen_preflight_record` for a workspace that was re-
imported through a portable profile that did not carry the
lineage packet (the same scenario as
[`fixtures/scaffolding/template_cases/lineage_unknown_imported_without_lineage_packet.yaml`](../../fixtures/scaffolding/template_cases/lineage_unknown_imported_without_lineage_packet.yaml)).
`reopen_preflight_disposition_class =
reopen_preflight_blocked_lineage_unknown_user_review_required`,
`reopen_preflight_blocker_class_set` contains
`lineage_packet_missing_after_import`,
`generated_project_lineage_state_class_carried =
lineage_unknown_imported_without_lineage_packet`,
`template_reopen_preflight_class_carried =
reopen_preflight_lineage_unknown_imported_without_lineage_packet`,
`denial_reason_class = lineage_packet_missing_after_import`. The
bypass routes remain at equal weight. See
[`reopen_preflight_blocked_lineage_unknown_after_import.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/reopen_preflight_blocked_lineage_unknown_after_import.yaml).

### 8.7 Delete-generated-output: lockfile-checkpoint recovery

A `delete_generated_output_record` invoked from the post-create
handoff sheet of §8.1.
`delete_generated_output_disposition_class =
delete_admissible_lockfile_checkpoint_recovery`,
`delete_generated_output_recovery_route_class =
delete_with_lockfile_checkpoint_recovery`,
`delete_provenance_preservation_class =
provenance_preserved_via_rollback_checkpoint`,
`provenance_anchor_class = lineage_metadata_file_under_vcs`,
`rollback_checkpoint_ref` non-null,
`denial_reason_class = no_denial_recovery_admissible`. The
bypass routes remain at equal weight. See
[`delete_admissible_lockfile_checkpoint_recovery.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/delete_admissible_lockfile_checkpoint_recovery.yaml).

### 8.8 Delete-generated-output: blocked unrecoverable

A `delete_generated_output_record` where no checkpoint, no
workspace snapshot, no local-history breadcrumb, and no template
revision reference are available.
`delete_generated_output_disposition_class =
delete_blocked_unrecoverable_user_review_required`,
`delete_generated_output_recovery_route_class =
delete_unrecoverable_user_review_required`,
`delete_provenance_preservation_class =
provenance_not_preserved_user_review_required`,
`denial_reason_class =
unrecoverable_delete_user_review_required`. The bypass routes
remain at equal weight; the recovery-route set names
`recovery_via_report_to_template_owner` and
`recovery_via_bypass_to_open_without_starter`. See
[`delete_blocked_unrecoverable_user_review_required.yaml`](../../fixtures/scaffolding/generated_project_handoff_cases/delete_blocked_unrecoverable_user_review_required.yaml).

## 9. Closed forbidden-collapse list

Every surface that reads a `post_create_handoff_record`,
`reopen_preflight_record`, or `delete_generated_output_record`
denies with the matching denial reason when it attempts any of
the following collapses:

1. **Hiding the bypass under any disposition.** Surfaces that
   render any disposition with no
   `bypass_open_without_starter_action` and no
   `create_without_starter_route_ids` deny with
   `create_without_starter_route_must_remain_at_equal_weight`.
2. **Collapsing partial-success / failure into "Setup did not
   finish".** Handoffs that resolve to
   `handoff_partial_success_recovery_offered` or
   `handoff_failure_recovery_required` without a typed
   `setup_action_disposition_set` deny under §3.2 rule 3.
3. **Silently approving an unrecoverable delete.**
   `delete_unrecoverable_user_review_required` recovery class
   forces the disposition out of every admissible class; a
   surface that admits it anyway denies with
   `unrecoverable_delete_user_review_required`.
4. **Treating a bypass-taken handoff as if a workspace was
   materialised.** §3.1 rule 8.
5. **Hiding lineage in a binary blob or Aureline-only RPC store.**
   §3.6 rule 3 — denies with
   `lineage_metadata_must_be_plain_reviewable_file`.
6. **Painting a reopen-preflight as in-sync when the policy
   epoch changed, the workspace-trust was revoked, the target
   runtime is unavailable, the lineage packet is missing, the
   workspace drifted, the required dependencies are missing,
   the secret-broker handle was revoked, the managed-workspace
   envelope was revoked, or the AI-proposed relink is pending
   review.** Each blocker forces a typed disposition (§3.7
   rule 4); collapsing into `reopen_preflight_admitted_in_sync`
   is non-conforming.
7. **Promoting an AI-proposed relink without admission.**
   `reopen_preflight_pending_ai_admission` and
   `delete_pending_ai_admission` are admissible only with a
   non-null `ai_tool_proposed_review_ticket_ref`.
8. **Exposing raw URLs / raw absolute paths / raw author email
   addresses / raw bearer tokens / raw API keys / raw signing
   keys / raw certificate or key material / raw container
   registry URLs / raw devcontainer image tags / raw lockfile
   bodies / raw extension marketplace URLs / raw stdout-stderr /
   raw post-install script bodies / raw user-supplied parameter
   values across either boundary.** All such fields cross only as
   opaque ids, closed vocabulary, content-addresses, or short
   reviewable sentences.

## 10. Changing this contract

- **Additive-minor** changes (new
  `post_create_handoff_disposition_class`, new
  `setup_outcome_class`, new `setup_action_class`, new
  `setup_action_disposition_class`, new `handoff_action_class`,
  new `recovery_route_class`, new `provenance_anchor_class`,
  new `reopen_preflight_disposition_class`, new
  `reopen_preflight_blocker_class`, new
  `delete_generated_output_disposition_class`, new
  `delete_provenance_preservation_class`, or new
  `denial_reason_class` value) land here, in the companion
  schemas, and in at least one fixture in the same change.
  Adding a value bumps the relevant `*_schema_version` const.
- **Repurposing** an existing vocabulary value is breaking and
  opens a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Upstream vocabulary changes** (UX template-and-prebuild
  contract, scaffolding template-and-scaffold contract,
  scaffolding template-health-and-preflight contract, recipe /
  macro contract, source-acquisition / bootstrap seed) happen
  at source and this contract re-exports by reference; it MUST
  NOT shadow the change.

## 11. Linked artifacts

- UX template-and-prebuild gallery / picker / preflight /
  post-create / health / policy contract:
  [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md).
- Scaffolding template-manifest, scaffold-run, and lineage
  contract:
  [`/docs/scaffolding/template_and_scaffold_contract.md`](./template_and_scaffold_contract.md).
- Scaffolding template-card, generation-preflight, and
  template-health contract:
  [`/docs/scaffolding/template_health_and_preflight_contract.md`](./template_health_and_preflight_contract.md).
- Post-create handoff schema:
  [`/schemas/scaffolding/post_create_handoff.schema.json`](../../schemas/scaffolding/post_create_handoff.schema.json).
- Generated-project recovery schema (reopen-preflight +
  delete-generated-output):
  [`/schemas/scaffolding/generated_project_recovery.schema.json`](../../schemas/scaffolding/generated_project_recovery.schema.json).
- Worked-example fixtures:
  [`/fixtures/scaffolding/generated_project_handoff_cases/`](../../fixtures/scaffolding/generated_project_handoff_cases/).
