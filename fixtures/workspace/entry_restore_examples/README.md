# First-run, entry, restore, and migration-result fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/workspace/entry_restore_object_model.md`](../../../docs/workspace/entry_restore_object_model.md)
and validated by the schema at
[`/schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json).

Each fixture names the record kind it exercises, the entry verb /
target kind / resulting mode / admission class / restore level /
missing-target state / session-execution posture /
checkpoint-linked recovery class / migration-item outcome set /
next-step decision hooks it covers, and the worked-example
section of the entry-restore document it motivates.

**Scope rules**

- Fixtures validate against the single entry-restore schema; they
  do not encode wire bytes, ADR-0005 subscription envelopes, or
  ADR-0004 RPC envelopes.
- A new fixture MUST exercise at least one frozen entry verb, one
  target kind, one restore level, one missing-target state, one
  checkpoint-linked recovery class, or one migration-item outcome,
  and MUST cite the section of the entry-restore model that
  motivates it.
- Monotonic timestamps and stable IDs are opaque; they are chosen
  to read well rather than to reflect any real clock or system
  state.
- Filesystem-identity records reuse the vocabulary frozen in
  [`/schemas/filesystem/save_target_token.schema.json`](../../../schemas/filesystem/save_target_token.schema.json);
  no identity fields are redefined here.
- ADR-0001 trust-state, ADR-0003 recovery-journal, ADR-0006
  filesystem-identity, ADR-0007 credential-handle, and ADR-0010
  connected-provider / browser-handoff approval-ticket
  vocabularies are quoted by reference and never redefined.

**Index**

| Fixture                                                                              | Record kind                       | Key classes exercised                                                                                                                                         | Doc section |
|--------------------------------------------------------------------------------------|-----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------|
| [`open_local_folder.json`](./open_local_folder.json)                                 | `project_entry_action_record`     | `open` / `local_folder` / `folder` / `no_write` / `admitted` / `review_archetype_match`                                                                        | §6.1        |
| [`clone_remote_repo.json`](./clone_remote_repo.json)                                 | `project_entry_action_record`     | `clone` / `remote_repository` / `clone_then_review` / `trust_review_required` / `ssh_agent` / `authority_reevaluation_required`                                | §6.2        |
| [`vs_code_settings_import.json`](./vs_code_settings_import.json)                     | `project_entry_action_record`     | `import` / `competitor_config_root` / `apply_to_active_workspace` / migration-handoff next-step hook quartet                                                   | §6.3        |
| [`vs_code_settings_import_result.json`](./vs_code_settings_import_result.json)       | `migration_result_record`         | all seven outcomes (`exact`, `translated`, `approximated`, `skipped` + `unsupported_by_target`, `blocked` + `admin_policy_excludes`, `needs_review` + `keybinding_conflict`, `rollback_available`) / parity scores / equivalence map / post-import validators | §6.3        |
| [`restore_last_session.json`](./restore_last_session.json)                           | `restore_prompt_record`           | `compatible_restore` / `binary_or_extension_version_changed` + `missing_extension_host` / `transcript_restored_not_rerun` + `rerun_required` + `session_ended` / `restore_from_session_checkpoint` + `restore_from_recovery_journal` | §6.4        |
| [`resume_managed_workspace.json`](./resume_managed_workspace.json)                   | `project_entry_action_record`     | `resume` / `managed_cloud_workspace` / `resume_live_session` / `needs_reauth` / `managed_session_ticket` / `authority_reevaluation_required` / `reauth_required` + `continue_in_restricted_mode` | §6.5        |
| [`start_from_prebuild.json`](./start_from_prebuild.json)                             | `project_entry_action_record`     | `start_from_snapshot` / `template_or_prebuild_snapshot` / `open_prebuild_with_setup_actions` / `mixed_setup` + `long_running` + `rollback_checkpoint_retained` / bypass `open_prebuild_minimal` | §6.6        |
| [`recent_work_missing_target.json`](./recent_work_missing_target.json)               | `recent_work_entry_record`        | `local_repo_root` / `missing_target` / `layout_only` / `locate_missing_target` + `remove_from_recents` + `reveal_in_explorer`                                 | §6.7        |
