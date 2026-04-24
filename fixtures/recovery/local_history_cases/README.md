# Local-history case fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/local_history_contract.md`](../../../docs/reliability/local_history_contract.md)
and validated by
[`/schemas/recovery/local_history_entry.schema.json`](../../../schemas/recovery/local_history_entry.schema.json).

Each fixture names the originator class it covers, the actor /
source / snapshot / truth-source classes it exercises, and the
section of the contract it motivates.

**Scope rules**

- Fixtures validate against the local-history entry / group /
  clear-scope schema; they do not redefine the mutation-journal
  attribution vocabulary (they cite it by `linked_id`) and they do
  not redefine filesystem identity (they cite it by reference to
  `schemas/filesystem/save_target_token.schema.json#filesystem_identity_record`).
- A new fixture MUST exercise at least one `snapshot_class`, one
  `actor_class`, or one `truth_source_class` that the existing set
  does not already cover, and MUST cite the contract section it
  motivates.
- Monotonic timestamps and stable ids are opaque; they are chosen
  to read well rather than to reflect any real clock.

**Index**

| Fixture                                                                                       | Originator class            | Record kind                              | Key classes exercised                                                                                | Doc section |
|-----------------------------------------------------------------------------------------------|-----------------------------|------------------------------------------|------------------------------------------------------------------------------------------------------|-------------|
| [`typing_edit_save_checkpoint.json`](./typing_edit_save_checkpoint.json)                       | `typing`                    | `local_history_entry`                    | `user_keystroke` / `human_local` / `edit_save_checkpoint` / `content_addressed_snapshot`             | §1, §2, §6  |
| [`paste_import_snapshot.json`](./paste_import_snapshot.json)                                   | `paste_or_drop_import`      | `local_history_entry`                    | `paste_or_drop_import` / `human_local` / `edit_save_checkpoint`                                      | §3, §7      |
| [`ai_apply_group.json`](./ai_apply_group.json)                                                 | `ai_patch_proposal`         | `local_history_group_record`             | `ai_apply` / `ai_hosted_provider` / `automation_ai_checkpoint` / `compensating_undo`                 | §1.2, §7    |
| [`automation_recipe_group.json`](./automation_recipe_group.json)                               | `automation_recipe`         | `local_history_group_record`             | `automation_recipe_runner` / `machine_local` / `automation_ai_checkpoint` / `restore_from_checkpoint`| §1.2, §7, §9|
| [`generated_artifact_metadata_reference.json`](./generated_artifact_metadata_reference.json)   | `automation_recipe`         | `local_history_entry`                    | `codegen_runner` / `metadata_plus_reference_only` / `omitted_generated_artifact_use_lineage`         | §6          |
| [`repair_recovery_checkpoint.json`](./repair_recovery_checkpoint.json)                         | `repair`                    | `local_history_entry`                    | `decode_recovery` / `metadata_plus_reference_only` / `restore_from_checkpoint`                       | §3, §6      |
| [`external_change_record.json`](./external_change_record.json)                                 | `external_change`           | `local_history_entry`                    | `external_change_detector` / `external_state_checkpoint` / `external_cause_metadata_only` / `no_reversal_external_event` | §3, §5 |
| [`restore_creates_new_checkpoint.json`](./restore_creates_new_checkpoint.json)                 | `restore`                   | `local_history_entry`                    | `restore_rollback_runner` / `restore_rollback_checkpoint` / `rename_detected` + `restore_preview_required_fields` | §4, §8 |
| [`clear_workspace_history_scope.json`](./clear_workspace_history_scope.json)                   | `clear_history_review`      | `local_history_clear_scope_record`       | `this_workspace` / `export_before_delete_completed` / `ordinary_cache_clear_origin: false`           | §11         |
