# Learning artifact authoring fixtures

Worked-example fixtures for the learning artifact object model frozen
in
[`/docs/ux/learning_artifact_object_model.md`](../../../docs/ux/learning_artifact_object_model.md)
and validated by the six companion artifact schemas:

- [`/schemas/ux/glossary_pack_item.schema.json`](../../../schemas/ux/glossary_pack_item.schema.json)
- [`/schemas/ux/tour_step.schema.json`](../../../schemas/ux/tour_step.schema.json)
- [`/schemas/ux/exercise_step.schema.json`](../../../schemas/ux/exercise_step.schema.json)
- [`/schemas/ux/learning_mode_profile.schema.json`](../../../schemas/ux/learning_mode_profile.schema.json)
- [`/schemas/ux/teaching_session.schema.json`](../../../schemas/ux/teaching_session.schema.json)
- [`/schemas/ux/speaker_note.schema.json`](../../../schemas/ux/speaker_note.schema.json)

Each YAML file is a single artifact record: a `__fixture__` envelope
plus the artifact body whose top level validates against the relevant
schema. The fixtures carry only opaque ids, typed vocabulary, short
privacy-safe labels, monotonic placeholder timestamps, and opaque
policy-bundle refs — no raw URLs, no raw absolute paths, no raw
prompt / completion text, no raw audience identifiers, and no raw
cryptographic material.

## Index

| Fixture                                                  | Schema                                        | Exercises                                                                                              |
|----------------------------------------------------------|-----------------------------------------------|--------------------------------------------------------------------------------------------------------|
| `glossary_pack_item_workspace_trust.yaml`                | `glossary_pack_item.schema.json`              | Glossary item citing a `glossary_term_anchor`; jargon `intermediate`; portable privacy scope.          |
| `tour_step_open_folder.yaml`                             | `tour_step.schema.json`                       | Tour step on `start_center` layout; `command_invocation_detector`; allowed action set.                 |
| `tour_step_indexing_paraphrased.yaml`                    | `tour_step.schema.json`                       | Paraphrased step body; `derived_with_upstream_anchors`; docs-anchor-seen detector.                     |
| `exercise_step_format_document_reversible.yaml`          | `exercise_step.schema.json`                   | Mutating exercise; `reversible_via_undo_history`; completion via canonical command id.                 |
| `exercise_step_blocked_until_trust.yaml`                 | `exercise_step.schema.json`                   | Trust-blocked exercise; empty allowed actions; manual-acknowledge detector.                            |
| `speaker_note_open_folder_preserves_identity.yaml`       | `speaker_note.schema.json`                    | Facilitator-only note preserving paired-step command identity; shared to named audience.               |
| `speaker_note_facilitator_only_local.yaml`               | `speaker_note.schema.json`                    | Presenter-owned-local note; not shared; paired with rehearsal exercise.                                |
| `teaching_session_shared_classroom.yaml`                 | `teaching_session.schema.json`                | Classroom-managed session; replay-retention `classroom_artifact_privacy_bounded`; envelope enforced.   |
| `teaching_session_local_only_dry_run.yaml`               | `teaching_session.schema.json`                | Local-only dry-run rehearsal; no follow; no replay retained.                                           |
| `learning_mode_profile_default_individual_learner.yaml`  | `learning_mode_profile.schema.json`           | Default individual learner; portable progress export; twelve invariants asserted.                      |
| `learning_mode_profile_air_gapped_offline.yaml`          | `learning_mode_profile.schema.json`           | Air-gapped offline pack; share unavailable; machine-local diagnostic portability.                      |

Every fixture cites the contract sections it exercises and binds
each axis by reference rather than redefinition.
