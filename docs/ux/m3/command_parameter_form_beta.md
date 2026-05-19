# Command parameter forms and invocation review sheets

This report is the published parity surface for the schema-driven parameter form and invocation review sheet records. Every bundle below is projected from the same command descriptor, so palette parameter forms, CLI inspect surfaces, AI tool envelopes, automation-recipe step editors, request / run / debug / template / repair workspaces, and voice grammars render identical typed fields, source-layer labels, validation findings, restart classes, and review semantics.

Catalog id: `shell:command_forms_beta:catalog:v1`

Source descriptor schema: `schemas/commands/command_descriptor.schema.json`

Source form schema: `schemas/commands/parameter_form_state.schema.json`

Source review-sheet schema: `schemas/commands/invocation_review_sheet.schema.json`

Generated at: `2026-05-19T00:00:00Z`

## Scenario: `bulk_replace_in_files_desktop_apply`

Bundle id: `bundle:bulk_replace:scenario-01`

Command: `cmd:workspace.bulk_replace_in_files.apply` (revision `cmd-rev:workspace.bulk_replace_in_files:2026.05.18-01`)

### Parameter form

- Form surface: `template_or_generator_form`
- Client scope: `desktop_product`
- Trust state: `trusted`
- Overall validation severity: `informational`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `scope_glob` | `glob_expression` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |
| `find_pattern_ref` | `opaque_id_ref` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |
| `replace_template_ref` | `opaque_id_ref` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |
| `case_sensitive` | `boolean_flag` | no | `editable` | Default | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |

### Invocation review sheet

- Review surface: `template_or_generator_review`
- Capability scope: `recoverable_durable_mutation`
- Preview class: `destructive_bulk_mutation_preview`
- Approval posture: `explicit_confirmation_required`
- Execution intent: `apply_after_preview`
- Rollback: `checkpoint_backed_revert`
- Preview/dry-run: `preview_structured_diff` (available: yes)
- Invocable from this sheet: yes

Scope axes:

- `selected_files_or_globs`: included=42, excluded=7, hidden/blocked=0 (count truth `exact`)
- `workspace_root`: included=1, excluded=0, hidden/blocked=0 (count truth `exact`)

Side effects:

- `local_filesystem_mutation`
- `workspace_state_mutation`

Secret handling: any-secret-bearing=no, all-handle-only=yes, runtime-reveal-armed=no, redaction=`metadata_safe_default`

## Scenario: `debug_attach_to_process_desktop`

Bundle id: `bundle:debug_attach:scenario-02`

Command: `cmd:debug.attach_to_process` (revision `cmd-rev:debug.attach_to_process:2026.05.18-01`)

### Parameter form

- Form surface: `run_debug_profile_form`
- Client scope: `desktop_product`
- Trust state: `trusted`
- Overall validation severity: `informational`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `target_process_ref` | `opaque_id_ref` | yes | `editable` | From selection | `value_visible` | `metadata_safe_default` | `debugger_restart_required` |
| `debugger_profile_ref` | `opaque_id_ref` | yes | `editable` | Workspace value | `value_visible` | `metadata_safe_default` | `debugger_restart_required` |
| `request_capability_token_ref` | `capability_ref` | yes | `policy_pinned_read_only` | Org policy | `value_handle_only` | `operator_only_restricted` | `no_restart_required` |
| `stop_on_attach` | `boolean_flag` | no | `editable` | Default | `value_visible` | `metadata_safe_default` | `no_restart_required` |

### Invocation review sheet

- Review surface: `run_debug_review`
- Capability scope: `managed_workspace_control`
- Preview class: `remote_attach_preview`
- Approval posture: `step_up_authentication_required`
- Execution intent: `apply_with_approval`
- Rollback: `auto_revert_on_failure`
- Preview/dry-run: `preview_inline_summary` (available: yes)
- Invocable from this sheet: yes

Scope axes:

- `selected_runs_or_sessions`: included=1, excluded=0, hidden/blocked=0 (count truth `exact`)

Side effects:

- `process_spawn_local`
- `credential_or_secret_use`

Secret handling: any-secret-bearing=yes, all-handle-only=yes, runtime-reveal-armed=no, redaction=`operator_only_restricted`

## Scenario: `request_workspace_send_request_desktop`

Bundle id: `bundle:send_request:scenario-03`

Command: `cmd:request_workspace.send_request` (revision `cmd-rev:request_workspace.send_request:2026.05.18-01`)

### Parameter form

- Form surface: `request_workspace_form`
- Client scope: `desktop_product`
- Trust state: `trusted`
- Overall validation severity: `informational`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `endpoint_ref` | `provider_ref` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `request_reissue_required` |
| `method` | `string_enum` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `request_reissue_required` |
| `auth_handle_ref` | `credential_handle_ref` | yes | `secret_handle_swap_only` | Secret reference | `value_handle_only` | `operator_only_restricted` | `request_reissue_required` |
| `request_body_ref` | `opaque_id_ref` | no | `editable` | Workspace value | `value_visible` | `metadata_safe_default` | `request_reissue_required` |
| `include_response_in_history` | `boolean_flag` | no | `editable` | Workspace value | `value_visible` | `metadata_safe_default` | `no_restart_required` |

### Invocation review sheet

- Review surface: `request_workspace_review`
- Capability scope: `externally_visible_mutation`
- Preview class: `externally_mutating_preview`
- Approval posture: `explicit_confirmation_required`
- Execution intent: `apply_after_preview`
- Rollback: `compensating_action_required`
- Preview/dry-run: `preview_simulate_dry_run` (available: yes)
- Invocable from this sheet: yes

Scope axes:

- `selected_requests_or_endpoints`: included=1, excluded=0, hidden/blocked=0 (count truth `exact`)

Side effects:

- `network_request_outbound`
- `credential_or_secret_use`

Secret handling: any-secret-bearing=yes, all-handle-only=yes, runtime-reveal-armed=no, redaction=`operator_only_restricted`

## Scenario: `recipe_run_blocked_by_policy`

Bundle id: `bundle:recipe_run:scenario-04`

Command: `cmd:automation.recipe_run_step` (revision `cmd-rev:automation.recipe_run_step:2026.05.18-01`)

### Parameter form

- Form surface: `automation_recipe_step_editor`
- Client scope: `desktop_product`
- Trust state: `trusted`
- Overall validation severity: `blocking`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `recipe_ref` | `opaque_id_ref` | yes | `editable` | Recipe supplied | `value_visible` | `metadata_safe_default` | `no_restart_required` |
| `target_branch_ref` | `workspace_scope_ref` | yes | `policy_pinned_read_only` | Org policy | `value_visible` | `metadata_safe_default` | `no_restart_required` |
| `auto_apply` | `boolean_flag` | no | `editable` | Recipe supplied | `value_visible` | `metadata_safe_default` | `no_restart_required` |

### Invocation review sheet

- Review surface: `automation_recipe_review`
- Capability scope: `recoverable_durable_mutation`
- Preview class: `batch_scope_preview`
- Approval posture: `admin_policy_approval_required`
- Execution intent: `propose_preview_only`
- Rollback: `named_undo_group_revert`
- Preview/dry-run: `preview_batch_scope` (available: yes)
- Invocable from this sheet: no

Scope axes:

- `selected_recipe_steps`: included=5, excluded=0, hidden/blocked=1 (count truth `exact`)

Side effects:

- `workspace_state_mutation`

Blocked prerequisites:

- `missing_policy_admission` -> repair hook `request_admin_policy_change` (id `hook:recipe_run:request_admin_policy`), disabled_reason=`policy_blocked_in_context`

Secret handling: any-secret-bearing=no, all-handle-only=yes, runtime-reveal-armed=no, redaction=`metadata_safe_default`

## Scenario: `release_publish_artifact_desktop`

Bundle id: `bundle:publish:scenario-05`

Command: `cmd:release.publish_artifact` (revision `cmd-rev:release.publish_artifact:2026.05.18-01`)

### Parameter form

- Form surface: `desktop_parameter_form`
- Client scope: `desktop_product`
- Trust state: `trusted`
- Overall validation severity: `informational`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `artifact_ref` | `opaque_id_ref` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |
| `signing_credential_handle_ref` | `credential_handle_ref` | yes | `secret_handle_swap_only` | Secret reference | `value_handle_only` | `signing_evidence_only` | `no_restart_required` |
| `target_channel` | `string_enum` | yes | `editable` | User override | `value_visible` | `metadata_safe_default` | `preview_recompute_required` |
| `release_notes_anchor` | `docs_anchor_ref` | yes | `editable` | Workspace value | `value_visible` | `metadata_safe_default` | `no_restart_required` |

### Invocation review sheet

- Review surface: `desktop_review_sheet`
- Capability scope: `irreversible_high_blast_mutation`
- Preview class: `irreversible_publish_preview`
- Approval posture: `second_party_review_required`
- Execution intent: `apply_with_approval`
- Rollback: `irreversible_no_revert_possible`
- Preview/dry-run: `preview_inline_summary` (available: yes)
- Invocable from this sheet: yes

Scope axes:

- `selected_remotes_or_origins`: included=1, excluded=0, hidden/blocked=0 (count truth `exact`)

Side effects:

- `credential_or_secret_use`
- `network_request_outbound`
- `git_remote_mutation`

Secret handling: any-secret-bearing=yes, all-handle-only=yes, runtime-reveal-armed=no, redaction=`signing_evidence_only`

## Scenario: `repair_workspace_blocked_on_runtime_prompt`

Bundle id: `bundle:repair:scenario-06`

Command: `cmd:doctor.repair_workspace_health` (revision `cmd-rev:doctor.repair_workspace_health:2026.05.18-01`)

### Parameter form

- Form surface: `cli_inspect_surface`
- Client scope: `cli`
- Trust state: `trusted`
- Overall validation severity: `blocking`

| Field | Kind | Required | State | Source layer | Visibility | Redaction | Restart/reload |
|---|---|---|---|---|---|---|---|
| `repair_action_ref` | `opaque_id_ref` | yes | `editable` | CLI args | `value_visible` | `metadata_safe_default` | `workspace_reload_required` |
| `confirmation_phrase` | `string_free_form` | yes | `runtime_prompt_required` | Runtime prompt | `value_visible` | `metadata_safe_default` | `no_restart_required` |
| `include_remote_caches` | `boolean_flag` | no | `unsupported_in_client_scope` | Default | `value_omitted_for_redaction` | `metadata_safe_default` | `no_restart_required` |

### Invocation review sheet

- Review surface: `cli_inspect_review`
- Capability scope: `recoverable_durable_mutation`
- Preview class: `broad_workspace_scope_preview`
- Approval posture: `explicit_confirmation_required`
- Execution intent: `propose_preview_only`
- Rollback: `checkpoint_backed_revert`
- Preview/dry-run: `preview_simulate_dry_run` (available: yes)
- Invocable from this sheet: no

Scope axes:

- `selected_repair_targets`: included=1, excluded=0, hidden/blocked=0 (count truth `exact`)

Side effects:

- `workspace_state_mutation`
- `local_filesystem_mutation`

Blocked prerequisites:

- `missing_runtime_prompt_value` -> repair hook `refresh_freshness` (id `hook:repair:supply_confirmation`), disabled_reason=`required_argument_unresolved`

Secret handling: any-secret-bearing=no, all-handle-only=yes, runtime-reveal-armed=no, redaction=`metadata_safe_default`

