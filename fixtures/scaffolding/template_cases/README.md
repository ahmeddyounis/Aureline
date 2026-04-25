# Template-manifest, scaffold-run, and lineage worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/scaffolding/template_and_scaffold_contract.md`](../../../docs/scaffolding/template_and_scaffold_contract.md)
and the schemas at
[`/schemas/scaffolding/template_manifest.schema.json`](../../../schemas/scaffolding/template_manifest.schema.json)
and
[`/schemas/scaffolding/scaffold_run.schema.json`](../../../schemas/scaffolding/scaffold_run.schema.json).

Every file is a single YAML document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, and the acceptance bullets it backs. The runtime
payload conforms to one of these shapes:

- `template_manifest_record`
- `scaffold_run_record`
- `generated_project_lineage_record`

No fixture embeds raw bearer tokens, raw API keys, raw
passwords, raw signing keys, raw certificate / key material,
raw absolute filesystem paths, raw repository URLs, raw author
email addresses, raw stdout / stderr, raw post-install script
bodies, raw devcontainer image tags, raw container registry
URLs, raw lockfile bodies, raw manifest bodies, or raw user-
supplied literal values. Every such field is an opaque ref into
a per-classification registry, an integer-bucket count, a typed
enum value, or a redaction-aware reviewable sentence.

## Cases

### Template-manifest cases (acceptance bullets 1, 4)

- [`first_party_local_template_manifest.yaml`](./first_party_local_template_manifest.yaml)
  — First-party Rust CLI starter under
  `template_source_class = first_party`,
  `support_class = officially_supported`,
  `runtime_and_toolchain_scope = local_only`, and
  `template_egress_posture_class = no_network_egress_required`.
  Generated files cover the Cargo manifests, sources, license,
  a JSONC settings overlay, and a plain-JSON
  `lineage_metadata_file_for_generated_project`. Declared
  hooks: pre-write parameter validation, post-write format,
  post-write apply lineage metadata, post-create print
  handoff. Declared setup tasks: toolchain detect and settings
  materialize. No undeclared hook or task can run.
- [`community_template_signature_review_required.yaml`](./community_template_signature_review_required.yaml)
  — Community Python data-workbench starter under
  `template_source_class = community` with
  `signer_continuity_class =
  signer_changed_review_required` and
  `template_egress_posture_class =
  egress_to_community_origin_user_review_required`. The
  gallery surface that lists this row resolves a typed
  signature-review notice in-place; the manifest cannot be
  admitted without the notice.
- [`template_known_issue_active_workaround.yaml`](./template_known_issue_active_workaround.yaml)
  — Team-managed Go backend-service starter carrying a
  `known_issue_rows[]` entry with
  `template_known_issue_class =
  known_issue_active_workaround_documented`, a non-null
  workaround summary ref, and a non-null
  `affected_template_revision_ref_set`. Demonstrates that
  known-issue, freshness, and reopen-preflight notes stay
  explicit on the manifest.

### Scaffold-run cases (acceptance bullets 1, 2)

- [`dry_run_preview_first_party_local_template.yaml`](./dry_run_preview_first_party_local_template.yaml)
  — Scaffold run resolving to
  `dry_run_preview_only_no_files_written` against the
  first-party local Rust CLI template. The run carries a
  non-null `dry_run_preview_ref`,
  `scaffold_dry_run_posture_class =
  dry_run_required_before_apply`, `applied_at = null`, and
  no file / hook / setup-task ids because nothing was
  written. Dry-run metadata is first-class.
- [`partial_apply_rollback.yaml`](./partial_apply_rollback.yaml)
  — Scaffold run for a self-hosted-org Node.js / pnpm
  template where `package_restore_pnpm` failed post-apply,
  `scaffold_run_outcome_class =
  rolled_back_after_partial_apply`. The record carries a
  non-null `rollback_checkpoint_ref`, a non-null
  `delete_generated_output_recovery_route_ref` resolving to
  `delete_with_lockfile_checkpoint_recovery`, and a non-null
  `dry_run_preview_ref` so rollback metadata stays first-
  class.
- [`denied_undeclared_hook_run.yaml`](./denied_undeclared_hook_run.yaml)
  — Scaffold run that attempted to invoke a hook id not
  present in the template's `declared_hooks` list.
  `scaffold_run_outcome_class =
  denied_pre_apply_undeclared_hook_or_task` and
  `denial_reason_class =
  undeclared_hook_or_setup_task_must_not_run`. Demonstrates
  that no hook, dependency install, or setup task can
  execute unless declared on the manifest.
- [`ai_tool_proposed_run_pending_admission.yaml`](./ai_tool_proposed_run_pending_admission.yaml)
  — Scaffold run dispatched by an AI tool but pending user
  admission. `scaffold_run_outcome_class =
  ai_tool_proposed_run_pending_admission`, `applied_at =
  null`, and `ai_tool_proposed_run_review_ticket_ref` is
  non-null. The schema's allOf gate forbids the run from
  applying pending review.

### Lineage cases (acceptance bullet 3)

- [`lineage_linked_to_template_revision.yaml`](./lineage_linked_to_template_revision.yaml)
  — `generated_project_lineage_record` for a workspace
  produced by the first-party local Rust CLI template, with
  `generated_project_lineage_state_class =
  linked_to_template_revision`,
  `generated_project_update_or_rebase_state_class =
  in_sync_with_bound_template_revision`,
  `template_reopen_preflight_class =
  reopen_preflight_in_sync_admissible`, and a non-null
  `bound_template_revision_ref`. Demonstrates that the
  lineage record explains whether the project is still
  linked to its template — and that the record is the on-
  disk truth that survives portable export / import.
- [`lineage_unknown_imported_without_lineage_packet.yaml`](./lineage_unknown_imported_without_lineage_packet.yaml)
  — `generated_project_lineage_record` for a workspace
  re-imported through a portable profile that did not carry
  the lineage packet, with
  `generated_project_lineage_state_class =
  lineage_unknown_imported_without_lineage_packet`,
  `generated_project_update_or_rebase_state_class =
  no_update_path_unlinked_or_lineage_unknown`,
  `template_reopen_preflight_class =
  reopen_preflight_lineage_unknown_imported_without_lineage_packet`,
  and a null `bound_template_revision_ref`. Demonstrates
  that lineage truth narrows honestly when no lineage packet
  was carried across the export / import boundary.

## Acceptance mapping

| Acceptance bullet | Demonstrating fixtures |
|---|---|
| No hook, dependency install, or setup task can execute unless declared | `denied_undeclared_hook_run.yaml`, `first_party_local_template_manifest.yaml` |
| Dry-run and rollback metadata remain first-class | `dry_run_preview_first_party_local_template.yaml`, `partial_apply_rollback.yaml`, `ai_tool_proposed_run_pending_admission.yaml` |
| Generated-project lineage survives export / import and explains link-state | `lineage_linked_to_template_revision.yaml`, `lineage_unknown_imported_without_lineage_packet.yaml` |
| Known-issue, freshness, reopen-preflight notes explicit | `template_known_issue_active_workaround.yaml`, `community_template_signature_review_required.yaml`, `first_party_local_template_manifest.yaml` |
