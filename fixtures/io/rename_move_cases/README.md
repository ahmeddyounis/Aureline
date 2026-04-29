# Rename and move review fixtures

These fixtures anchor the rename and move review contract:

- [`/docs/io/rename_move_review_contract.md`](../../../docs/io/rename_move_review_contract.md)
- [`/schemas/io/rename_move_plan.schema.json`](../../../schemas/io/rename_move_plan.schema.json)

Each YAML file is one `rename_move_plan_record`. The records are
pre-implementation examples: ids are opaque and chosen for readability;
they are not planning identifiers.

| Fixture | Coverage | Expected next safe action |
|---|---|---|
| [`local_case_only_rename_insensitive.yaml`](./local_case_only_rename_insensitive.yaml) | Local case-only rename on an insensitive-preserving root. | `rename_in_place` |
| [`local_unicode_normalization_rename.yaml`](./local_unicode_normalization_rename.yaml) | Unicode-normalization-only rename on a local root. | `rename_in_place` |
| [`local_symlink_escape_blocked.yaml`](./local_symlink_escape_blocked.yaml) | Symlink escape from selected root to policy-sensitive canonical target. | `block_and_explain` |
| [`windows_junction_escape_blocked.yaml`](./windows_junction_escape_blocked.yaml) | Junction escape on a local Windows-like root. | `block_and_explain` |
| [`cross_root_local_to_remote_copy_review.yaml`](./cross_root_local_to_remote_copy_review.yaml) | Local-to-remote move where copy is reviewable but source delete is not implicit. | `copy_then_review` |
| [`remote_revision_drift_recompute.yaml`](./remote_revision_drift_recompute.yaml) | Remote move plan invalidated by revision drift. | `recompute_against_current_root_capabilities` |
| [`mirrored_root_freshness_lag_review.yaml`](./mirrored_root_freshness_lag_review.yaml) | Mirrored root transfer with stale mirror digest. | `recompute_against_current_root_capabilities` |
| [`generated_companion_detach_alias.yaml`](./generated_companion_detach_alias.yaml) | Generated companion relation detaches from canonical source authority. | `detach_alias` |
| [`same_path_different_object_block.yaml`](./same_path_different_object_block.yaml) | Same presentation path now resolves to a different canonical object. | `block_and_explain` |

Rules for new fixtures:

1. Keep `silent_best_effort_forbidden: true`.
2. Include at least one `policy_trust_boundary_notes` row, using
   `boundary_class: none` when there is no boundary change.
3. Preserve the same `parity_requirements` field family so file tree,
   breadcrumb, refactor, import, save conflict, CLI, automation, and
   support surfaces stay comparable.
