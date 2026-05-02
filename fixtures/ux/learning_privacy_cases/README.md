# Learning, presentation, and classroom retention / share / delete / export fixtures

Worked-example fixtures for the learning / presentation privacy and
export contract frozen in
[`/docs/ux/learning_privacy_export_contract.md`](../../../docs/ux/learning_privacy_export_contract.md)
and validated by the companion schema:

- [`/schemas/ux/learning_progress_retention.schema.json`](../../../schemas/ux/learning_progress_retention.schema.json)

Each YAML file is a single record: a `__fixture__` envelope plus
the record body whose top level validates against the schema. The
fixtures carry only opaque ids, typed vocabulary, short privacy-safe
labels, monotonic placeholder timestamps, and opaque policy-bundle /
profile-package / pack / support-bundle refs — no raw URLs, no raw
absolute paths, no raw prompt / completion text, no raw audience
identifiers, no raw account or billing identifiers, and no raw
cryptographic material.

## Index

| Fixture                                                                         | Record kind                                       | Exercises                                                                                                  |
|---------------------------------------------------------------------------------|---------------------------------------------------|------------------------------------------------------------------------------------------------------------|
| `progress_local_only_private.yaml`                                              | `learning_artifact_retention_row_record`          | Per-device-dismissable progress; `local_only_private`; `not_exported_machine_local`; air-gapped admissible. |
| `speaker_note_shared_presentation.yaml`                                         | `learning_artifact_retention_row_record`          | Speaker note; `shared_to_facilitator_only`; `until_session_ends`; never crosses to audience.               |
| `classroom_artifact_privacy_bounded.yaml`                                       | `learning_artifact_retention_row_record`          | Classroom artifact under `classroom_artifact_privacy_bounded`; `privacy_bounded_classroom_artifact` hook.  |
| `cited_pack_offline_export.yaml`                                                | `learning_artifact_retention_row_record`          | Offline-verified docs pack; `until_pack_revision_change`; `cached_pack_continuity` hook reserved.          |
| `policy_disabled_sharing.yaml`                                                  | `learning_artifact_retention_row_record`          | Teaching-session bundle; `policy_managed_pushed`; `shared_blocked_by_policy`; `not_deletable_locally`.     |
| `progress_imported_from_support_bundle.yaml`                                    | `learning_artifact_retention_row_record`          | Progress imported from a support bundle (read-only audit); `not_deletable_locally`.                        |
| `dont_show_again_per_profile.yaml`                                              | `learning_artifact_retention_row_record`          | Don't-show-again toggle; `per_profile_dismissable`; `in_portable_profile_package_redacted`.                |
| `replay_blocked_by_policy.yaml`                                                 | `learning_artifact_retention_row_record`          | Replay data; `replay_blocked_by_policy`; `deletable_by_policy_reset`.                                      |
| `export_review_local_only_and_policy_blocked.yaml`                              | `learning_artifact_export_review_record`          | Export review with one selected row and two excluded rows under typed reasons.                             |
| `delete_review_preserves_workspace.yaml`                                        | `learning_artifact_delete_review_record`          | User-initiated delete preserving unrelated workspace content.                                              |
| `learning_privacy_export_manifest_default_individual_learner.yaml`              | `learning_privacy_export_manifest_record`         | Manifest binding all eight retention rows, the export review, and the delete review; twelve invariants.    |

Every fixture cites the contract sections it exercises and binds
each axis by reference rather than redefinition.
