# Rollback-checkpoint example rows

Reviewer-facing rollback-checkpoint rows the migration-center
review, first-run import review, post-import follow-up, support-
export row, and release-evidence packet quote when a migration
apply path mutates durable truth. Each file carries one
`rollback_checkpoint_example_record` binding one
`rollback_checkpoint_outcome_class` token to the frozen
`migration_restore_record` fields (`availability_state`, `scope`,
`restore_action_hints`, `cleanup_state`, `partial_apply_note`,
`restore_provenance_ref`, `support_packet_refs`, `export_refs`)
plus the preserved prior-artifact and post-restore validator
linkage the packet requires.

These examples do not redefine the restore-record shape. They cite
the vocabulary frozen in:

- [`/docs/migration/migration_center_object_model.md`](../../../docs/migration/migration_center_object_model.md)
  — canonical `migration_restore_record` fields, restore scope,
  availability, and cleanup states.
- [`/docs/state/migration_and_restore_playbook.md`](../../../docs/state/migration_and_restore_playbook.md)
  — canonical fidelity labels, downgrade reasons, failure states,
  and preserved-prior-artifact rules.
- [`/docs/verification/migration_and_profile_packet.md`](../../../docs/verification/migration_and_profile_packet.md)
  — `rollback_checkpoint_outcome_class` vocabulary and projection
  rules.
- [`/schemas/migration/migration_session.schema.json`](../../../schemas/migration/migration_session.schema.json)
  — schema of record for `migration_restore_record`.
- [`/schemas/state/restore_provenance.schema.json`](../../../schemas/state/restore_provenance.schema.json)
  — schema of record for preserved-prior-artifact and validator
  outcome rows.

Required fields across every example:

- `rollback_checkpoint_outcome_class`
- `migration_session_ref`
- `restore_record_ref`
- `checkpoint_ref`
- `scope` — one of the frozen `migration_restore_scope` tokens.
- `availability_state` — frozen
  `migration_restore_availability_state` token.
- `cleanup_state` — frozen `migration_restore_cleanup_state`
  token.
- `restore_action_hints` — frozen `restore_action_hint_array`.
- `partial_apply_note` — required and non-empty when the outcome
  class is `checkpoint_available_post_apply_partial`; null
  otherwise.
- `preserved_prior_artifact_refs` — required when the apply
  touched `user_authored_durable_truth` or
  `workspace_shared_manifest`.
- `post_restore_validator_outcomes` — required when the outcome
  class is `checkpoint_restored_to_prior_state`.
- `compatibility_report_ref`, `compatibility_row_refs` — preserved
  from the migration session.
- `export_inclusion_posture`, `redaction_class`, `freshness_class`.

## Index

| Example | Outcome class | Notes |
|---|---|---|
| [`checkpoint_created_pre_apply.yaml`](./checkpoint_created_pre_apply.yaml) | `checkpoint_created_pre_apply` | Session at `diff_ready`; checkpoint retained; restore hints include `compare_before_restore`. |
| [`checkpoint_available_post_apply_clean.yaml`](./checkpoint_available_post_apply_clean.yaml) | `checkpoint_available_post_apply_clean` | Clean apply; checkpoint retained for the declared retention window. |
| [`checkpoint_available_post_apply_partial.yaml`](./checkpoint_available_post_apply_partial.yaml) | `checkpoint_available_post_apply_partial` | Partial apply; preserved prior artifacts ridden; partial-apply note required. |
| [`checkpoint_restored_to_prior_state.yaml`](./checkpoint_restored_to_prior_state.yaml) | `checkpoint_restored_to_prior_state` | User rolled back after apply; post-restore validators ridden. |
| [`checkpoint_expired_cleanup_complete.yaml`](./checkpoint_expired_cleanup_complete.yaml) | `checkpoint_expired_cleanup_complete` | Retention window elapsed; cleanup ran; restore hooks denied. |
| [`checkpoint_cleanup_pending_manual.yaml`](./checkpoint_cleanup_pending_manual.yaml) | `checkpoint_cleanup_pending_manual` | Cleanup blocked by an open handle; repair hook rides. |
| [`checkpoint_policy_hidden_support_only.yaml`](./checkpoint_policy_hidden_support_only.yaml) | `checkpoint_policy_hidden_support_only` | Policy pack restricted visibility to support; export posture `operator_only_restricted`. |
