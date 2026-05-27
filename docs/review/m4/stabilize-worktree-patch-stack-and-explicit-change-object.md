# Stabilized worktree, patch-stack, and explicit change-object orchestration for stable review lanes

**Scope:** M04-099 — Stabilize worktree, patch-stack, and explicit change-object orchestration for stable review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Every worktree operation, patch-stack mutation, and change-object orchestration must be previewable, checkpointed, and rooted in exact repo topology. Preview and recovery never target the wrong root because every packet carries repo-root identity, submodule or nested-repo boundary, shallow/partial state, and pointer-backed asset posture.

## Design principles

1. **Explicit repo-root identity** — Every orchestration record carries a `repo_root_ref` and `repo_topology_classes` so that parent repos, submodules, nested independent repos, shallow histories, and pointer-backed assets never resolve ambiguously.
2. **Previewable and checkpointed** — No worktree switch, patch-stack reorder, rebase, publish, or change-object apply may execute until the user has reviewed the preview and explicitly approved it. A mutation checkpoint is captured before any destructive apply.
3. **Restart-resilient session truth** — Every flow carries a `restart_session_ref` and the support-export packet embeds a `ChangeObjectOrchestrationRestartSnapshot` so the same structured session truth can be reopened after an IDE or CLI restart.
4. **Exact provider-linked behavior** — Browser handoff and provider publish proposals are explicit about source (`handoff_origin_class`), freshness (`freshness_class`), actor (`actor_ref`), target (`handoff_destination_class`), and return path (`return_anchor_ref`).
5. **Separable inspectable truths** — Flow state, checkpoint state, repo topology classes, pointer-backed asset posture, approval state, and checks-freshness state are all independent fields. No single "status" column hides the underlying truth.
6. **Redaction-safe support export** — Raw paths, branch names, patch bodies, provider payloads, and URLs are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_change_object_orchestration_packet` | Top-level packet consumed by review surfaces and support exports. |
| `review_change_object_orchestration_record` | Stable identity, operation provenance, repo-root ref, topology classes, boundary refs, asset posture. |
| `review_worktree_orchestration_record` | Worktree-specific operation state with source/target refs, kind, attachment, and checked-out ref. |
| `review_patch_stack_orchestration_record` | Patch-stack-specific operation state with target class, patch state, patch count, and affected patch refs. |
| `review_publish_proposal_record` | Provider publish proposal with readiness class, provider publish posture, and explicit handoff metadata. |
| `review_mutation_checkpoint_record` | Checkpoint summary before destructive apply. |
| `review_change_object_command_record` | Command-graph operations (preview, approve, capture, apply, rollback, abort, handoff). |
| `review_change_object_orchestration_support_export_packet` | Redaction-safe export with restart snapshot. |
| `review_change_object_orchestration_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Operation kinds
- `worktree_switch`, `worktree_create`, `worktree_remove`
- `patch_stack_reorder`, `patch_stack_rebase`, `patch_stack_publish`
- `change_object_publish`, `change_object_merge`, `change_object_apply`

### Flow states
- `preview_pending`, `preview_approved`, `checkpoint_pending`, `checkpoint_captured`, `executing`, `completed`, `failed`, `rolled_back`, `aborted`

### Checkpoint states
- `none_required`, `captured_ready`, `captured_pending`, `restored`, `expired`, `missing_blocks_apply`

### Repo topology classes
- `current_repo_root`, `worktree_root`, `submodule_root`, `nested_independent_repo_root`, `shallow_history_root`, `partial_clone_promisor_root`, `lfs_hydration_boundary`

### Pointer-backed asset postures
- `no_pointer_assets`, `lfs_pointer_present`, `submodule_gitlink_present`, `promisor_partial_object_present`, `mixed_pointer_assets`

### Publish readiness classes
- `ready_to_publish`, `ready_to_merge`, `ready_to_apply`, `blocked_by_conflicts`, `blocked_by_authority`, `blocked_by_review_required`, `not_applicable_inspect_only`, `readiness_unknown_requires_review`

## Key invariants

- `flow_state` in `executing`, `completed`, or `rolled_back` requires `checkpoint_state` to be `captured_ready`, `captured_pending`, or `restored`.
- `completed` flow cannot be marked `failed` or `rolled_back`.
- `worktree_switch`, `worktree_create`, `worktree_remove` require a `worktree_orchestration` record.
- `patch_stack_reorder`, `patch_stack_rebase`, `patch_stack_publish` require a `patch_stack_orchestration` record.
- Publish proposals requiring browser handoff must declare `handoff_destination_class` and `return_anchor_ref`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- `repo_root_ref` must be distinct across parent, child, and sibling repo change objects.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/stabilize_worktree_patch_stack_and_explicit_change_object/mod.rs` |
| Schema | `schemas/review/change_object_orchestration.schema.json` |
| Fixtures | `fixtures/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object/` |
| Tests | `crates/aureline-review/tests/stabilize_worktree_patch_stack_and_explicit_change_object_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- References [`aureline_git::change_objects`] records via opaque `change_object_ref`.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` and `harden_merge_rebase_cherry_pick_revert_and_reset` modules.

## Verification

```bash
cargo test -p aureline-review --test stabilize_worktree_patch_stack_and_explicit_change_object_alpha
```
