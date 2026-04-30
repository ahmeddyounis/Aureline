# Corruption-rescue compare-sheet and quarantined-copy fixtures

These fixtures anchor the vocabulary frozen in
[`/docs/reliability/corruption_rescue_compare_contract.md`](../../../docs/reliability/corruption_rescue_compare_contract.md)
and validated by:

- [`/schemas/recovery/corruption_rescue_sheet.schema.json`](../../../schemas/recovery/corruption_rescue_sheet.schema.json)
- [`/schemas/recovery/quarantined_copy_record.schema.json`](../../../schemas/recovery/quarantined_copy_record.schema.json)

Each fixture names the corrupt-artifact class it covers, the
artifact-value posture it exercises, the rescue verb and
destructive-risk class it routes the user toward, and the section
of the contract it motivates.

**Scope rules**

- Fixtures validate against either the corruption-rescue compare
  sheet schema or the quarantined-copy record schema; they do not
  redefine recovery-scenario, continuity-status, restore-preview,
  repair-transaction, scenario-picker, or escalation-packet
  vocabularies (those are cited by opaque ref).
- A new fixture MUST exercise at least one `corrupt_artifact_class`,
  `artifact_value_posture`, `healthy_candidate_source_class`,
  `rescue_verb_class`, `post_action_state_class`, `preserve_reason_class`,
  `retention_class`, `visibility_class`, or `inspectable_action_class`
  value the existing set does not already cover, AND MUST cite the
  contract section it motivates.
- Monotonic timestamps, opaque ids, and stable refs read well rather
  than reflect any real clock or path.
- Sheet fixtures whose recommended action is `replace`, `discard`, or
  any `restore` variant point at a quarantined-copy record fixture by
  `preservation.preserved_copy_ref`; the paired quarantined-copy
  fixture cites the sheet by `linkage.rescue_sheet_ref`.

**Index — compare sheets**

| Fixture | Corrupt artifact / posture | Verb / risk | Recommended action | Doc section |
|---|---|---|---|---|
| [`corrupted_workspace_search_index.yaml`](./corrupted_workspace_search_index.yaml) | `workspace_search_index` / `derived_disposable_only` | `rebuild` / `writes_disposable_state_only` | `rebuild_disposable_state` | §1, §2, §3, §4, §7, §8, §11 |
| [`suspect_profile_settings_store.yaml`](./suspect_profile_settings_store.yaml) | `profile_settings_store` / `user_authored_durable` | `replace` / `destructive_user_authored_no_undo_export_required` | `replace_with_default_after_quarantine` | §1, §2, §3, §4, §7, §8, §9, §11, §12 |
| [`failed_update_metadata.yaml`](./failed_update_metadata.yaml) | `update_install_metadata` / `install_chain_metadata` | `restore` / `mutates_profile_state_with_checkpoint_and_export` | `restore_from_local_checkpoint` | §1, §2, §3, §4, §6, §7, §8, §9, §11 |
| [`local_forensics_only_inspect.yaml`](./local_forensics_only_inspect.yaml) | `workspace_authority_checkpoint_index` / `local_forensics_only` | `inspect` / `non_destructive_read_only` | `inspect_only` | §1, §3, §6, §7, §8, §11, §12 |

**Index — quarantined-copy records**

| Fixture | Origin / storage | Visibility / retention | Preserve reasons | Doc section |
|---|---|---|---|---|
| [`suspect_profile_settings_store_quarantined_copy.yaml`](./suspect_profile_settings_store_quarantined_copy.yaml) | `profile_local` / `local_quarantine_lane` | `support_visible` / `retain_until_repair_outcome_ack` | `carries_user_authored_bytes`, `carries_diagnostics_value` | §10.2, §10.3, §10.4, §10.5 |
| [`failed_update_metadata_quarantined_copy.yaml`](./failed_update_metadata_quarantined_copy.yaml) | `install_chain_local` / `local_quarantine_lane` | `support_visible` / `retain_until_repair_outcome_ack` | `carries_diagnostics_value`, `mixed_user_and_derived_indeterminate` | §10.2, §10.3, §10.4, §10.5 |
| [`local_forensics_only_held_copy.yaml`](./local_forensics_only_held_copy.yaml) | `workspace_local` / `local_forensics_lane` | `local_forensics_only` / `retain_until_pending_investigation_resolved` | `forensics_required_by_pending_investigation`, `carries_diagnostics_value`, `ownership_indeterminate` | §10.4 visibility constraint, §10.5, §12 surface rule 9 |
