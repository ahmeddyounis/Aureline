# Change-object orchestration fixtures (M04-099)

These fixtures prove that parent, child, and sibling repo change objects remain distinct in preview, publish, rollback, and support-export paths.

## Fixtures

| Fixture | Scenario |
|---|---|
| `parent_repo_worktree_switch.json` | Worktree switch in the parent repo root (`current_repo_root` only). No submodule, nested repo, or shallow boundaries. Checkpoint captured and ready. |
| `child_submodule_patch_stack_publish.json` | Patch stack publish inside a submodule (`submodule_root` + `current_repo_root`). Carries `submodule_boundary_ref` linking back to the parent repo. Publish proposal requires browser handoff with explicit return anchor. |
| `sibling_nested_repo_change_object_apply.json` | Change object apply inside a nested independent repo (`nested_independent_repo_root` + `current_repo_root`). Carries `nested_repo_boundary_ref` so the mutation scope never leaks into the parent. |
| `shallow_history_pointer_asset_rollback.json` | Change object publish in a shallow clone (`shallow_history_root` + `current_repo_root`). Carries `shallow_history_boundary_ref` and `lfs_pointer_present` posture. Flow ended in `rolled_back` after publish failure; checkpoint restored. |

## Invariant checks

- Every fixture parses, validates, and projects without error.
- Repo-root refs are distinct across fixtures and never collide.
- Sub-module, nested-repo, and shallow-boundary refs are surfaced as separable inspection booleans.
- Support export keeps every `raw_*_export_allowed` flag false.
- Restart snapshots mirror current packet truth including `repo_root_ref`.
