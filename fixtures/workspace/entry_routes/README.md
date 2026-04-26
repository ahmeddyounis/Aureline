# Workspace Entry Route Fixtures

Seed corpus for the contract frozen in
[`/docs/ux/archetype_detection_contract.md`](../../../docs/ux/archetype_detection_contract.md)
and the schema at
[`/schemas/workspace/archetype_detection.schema.json`](../../../schemas/workspace/archetype_detection.schema.json).

Each file is a single JSON `workspace_archetype_admission_record`.
The fixtures exercise the required detection outcomes, readiness
buckets, admission checkpoints, setup-later invariants, and first-useful
route matrix.

Every fixture:

- separates detected facts, recommendations, policy, and user choice;
- labels material signals by source (`manifest`, `bundle_marker`,
  `workspace_file`, `import_packet`, `admin_policy`,
  `extension_contribution`, `previous_user_choice`, or a lower-trust
  source class);
- keeps `blocking_now`, `recommended_soon`, and `optional_later` in
  distinct buckets;
- asserts that `Set up later` and remembered routing do not widen trust,
  install packages, or suppress required review;
- carries no raw absolute paths, raw URLs with credentials, raw manifest
  bodies, raw command lines, or raw secrets.

## Cases

| Fixture | Detection outcome | Entry source | Main route |
| --- | --- | --- | --- |
| [`certified_web_repo_open.json`](./certified_web_repo_open.json) | `certified_archetype_match` | `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` |
| [`probable_single_file_workspace_candidate.json`](./probable_single_file_workspace_candidate.json) | `probable_archetype` | `single_file_open` | `file_editor_with_root_cues` |
| [`mixed_workspace_boundary_choice.json`](./mixed_workspace_boundary_choice.json) | `mixed_or_ambiguous_workspace` | `folder_or_repo_open` | `nested_root_choice_sheet` |
| [`generic_folder_plain_open.json`](./generic_folder_plain_open.json) | `unknown_or_generic_workspace` | `folder_or_repo_open` | `generic_shell_with_diagnostics` |
| [`restricted_clone_policy_blocked_setup.json`](./restricted_clone_policy_blocked_setup.json) | `restricted_or_policy_blocked` | `repository_clone` | `post_clone_handoff` |
| [`missing_remote_agent_deep_link.json`](./missing_remote_agent_deep_link.json) | `missing_prerequisite` | `review_or_incident_deep_link` | `linked_review_incident_or_work_item` |
| [`restore_remembered_plain_open.json`](./restore_remembered_plain_open.json) | `probable_archetype` | `restore_last_session` | `restored_layout_with_placeholders` |
| [`imported_handoff_compare_first.json`](./imported_handoff_compare_first.json) | `probable_archetype` | `imported_state_or_handoff_packet` | `import_compare_or_restore_sheet` |
