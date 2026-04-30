# Generated-project handoff, recovery, and delete-generated-output worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/scaffolding/generated_project_handoff_contract.md`](../../../docs/scaffolding/generated_project_handoff_contract.md)
and the schemas at
[`/schemas/scaffolding/post_create_handoff.schema.json`](../../../schemas/scaffolding/post_create_handoff.schema.json)
and
[`/schemas/scaffolding/generated_project_recovery.schema.json`](../../../schemas/scaffolding/generated_project_recovery.schema.json).

Every file is a single YAML document carrying a `__fixture__`
prelude summarising the scenario, the contract sections it
exercises, and the acceptance bullets it backs. The runtime
payload conforms to one of these shapes:

- `post_create_handoff_record`
- `reopen_preflight_record`
- `delete_generated_output_record`

No fixture embeds raw bearer tokens, raw API keys, raw
passwords, raw signing keys, raw certificate / key material,
raw absolute filesystem paths, raw repository URLs, raw author
email addresses, raw container registry URLs, raw devcontainer
image tags, raw lockfile bodies, raw extension marketplace
urls, raw stdout / stderr, raw post-install script bodies, or
raw user-supplied parameter values. Every such field is an
opaque ref into a per-classification registry, an integer-
bucket count, a typed enum value, or a redaction-aware
reviewable sentence.

## Cases

### Post-create handoff cases (acceptance bullets 1, 2, 3)

- [`handoff_apply_admitted_run_now_offered.yaml`](./handoff_apply_admitted_run_now_offered.yaml)
  — Successful create against the first-party Rust CLI starter.
  `post_create_handoff_disposition_class =
  handoff_admissible_run_now_offered`,
  `setup_outcome_class = all_actions_succeeded`,
  `provenance_anchor_class = lineage_metadata_file_under_vcs`,
  `delete_generated_output_recovery_route_class =
  delete_with_lockfile_checkpoint_recovery`. Bypass routes render
  at equal weight even on the success path.
- [`handoff_bypass_taken_no_workspace_materialised.yaml`](./handoff_bypass_taken_no_workspace_materialised.yaml)
  — Create-without-starter route taken from the picker. No
  declared hook or setup task ran; no workspace was materialised.
  `created_workspace_lineage_record_ref = null`,
  `provenance_anchor_class = bypass_anchor_no_workspace_materialised`.
- [`handoff_partial_setup_failure_recovery_offered.yaml`](./handoff_partial_setup_failure_recovery_offered.yaml)
  — Partial setup failure on a self-hosted-org Node / pnpm
  template after package_restore_pnpm failed. Rollback to lockfile
  checkpoint, review-files, delete-generated-output, report-to-
  template-owner, and bypass-to-open-without-starter recovery
  routes render at equal weight.

### Reopen-preflight cases (acceptance bullet 3)

- [`reopen_preflight_admitted_in_sync.yaml`](./reopen_preflight_admitted_in_sync.yaml)
  — Reopen of a workspace that is in sync with its bound template
  revision. `reopen_preflight_disposition_class =
  reopen_preflight_admitted_in_sync`,
  `reopen_preflight_blocker_class_set = [no_blocker_admissible]`,
  `denial_reason_class = no_denial_recovery_admissible`.
- [`reopen_preflight_blocked_policy_changed.yaml`](./reopen_preflight_blocked_policy_changed.yaml)
  — Reopen blocked after a policy epoch change. The bypass routes
  render at equal weight; recovery_via_request_policy_review and
  recovery_via_bypass_to_open_without_starter are offered.
- [`reopen_preflight_blocked_lineage_unknown_after_import.yaml`](./reopen_preflight_blocked_lineage_unknown_after_import.yaml)
  — Reopen blocked after a portable-profile re-import without a
  lineage packet. `generated_project_lineage_state_class_carried =
  lineage_unknown_imported_without_lineage_packet`,
  `template_reopen_preflight_class_carried =
  reopen_preflight_lineage_unknown_imported_without_lineage_packet`.

### Delete-generated-output cases (acceptance bullet 2)

- [`delete_admissible_lockfile_checkpoint_recovery.yaml`](./delete_admissible_lockfile_checkpoint_recovery.yaml)
  — Delete admissible via the lockfile checkpoint planted before
  apply. Provenance is preserved through the rollback checkpoint
  plus the on-disk lineage_metadata_file_under_vcs.
- [`delete_blocked_unrecoverable_user_review_required.yaml`](./delete_blocked_unrecoverable_user_review_required.yaml)
  — Delete blocked because no checkpoint, no snapshot, no local-
  history breadcrumb, and no template revision reference are
  available. `denial_reason_class =
  unrecoverable_delete_user_review_required`. Bypass routes render
  at equal weight.

## Acceptance mapping

| Acceptance bullet | Demonstrating fixtures |
|---|---|
| Post-create handoff always exposes a no-surprises recovery route after generation or failed setup | `handoff_apply_admitted_run_now_offered.yaml`, `handoff_bypass_taken_no_workspace_materialised.yaml`, `handoff_partial_setup_failure_recovery_offered.yaml`, `reopen_preflight_admitted_in_sync.yaml`, `reopen_preflight_blocked_policy_changed.yaml` |
| Generated output can be reviewed or deleted with preserved provenance rather than leaving stray files with no tracked lineage | `handoff_apply_admitted_run_now_offered.yaml`, `handoff_partial_setup_failure_recovery_offered.yaml`, `delete_admissible_lockfile_checkpoint_recovery.yaml`, `delete_blocked_unrecoverable_user_review_required.yaml`, `reopen_preflight_blocked_lineage_unknown_after_import.yaml` |
| Fixtures cover successful create, create without starter, partial setup failure, and reopen-preflight after a policy or environment change | `handoff_apply_admitted_run_now_offered.yaml` (successful create), `handoff_bypass_taken_no_workspace_materialised.yaml` (create without starter), `handoff_partial_setup_failure_recovery_offered.yaml` (partial setup failure), `reopen_preflight_blocked_policy_changed.yaml` and `reopen_preflight_blocked_lineage_unknown_after_import.yaml` (reopen-preflight after a policy / environment change) |
