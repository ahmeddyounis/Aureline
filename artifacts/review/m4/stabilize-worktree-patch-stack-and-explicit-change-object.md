# Artifact: Stabilized worktree, patch-stack, and explicit change-object orchestration

**Task:** M04-099  
**Status:** Implemented  
**Verification class:** Automated functional + Conformance  

## Summary

The change-object orchestration lane is now stable for daily-driver worktree, patch-stack, and explicit change-object operations. The implementation lives in `crates/aureline-review` and is exercised by fixture-driven tests.

## What changed

- New Rust module: `crates/aureline-review/src/stabilize_worktree_patch_stack_and_explicit_change_object/mod.rs`
- New schema: `schemas/review/change_object_orchestration.schema.json`
- New fixtures: `fixtures/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object/`
  - `parent_repo_worktree_switch.json`
  - `child_submodule_patch_stack_publish.json`
  - `sibling_nested_repo_change_object_apply.json`
  - `shallow_history_pointer_asset_rollback.json`
- New tests: `crates/aureline-review/tests/stabilize_worktree_patch_stack_and_explicit_change_object_alpha.rs`
- New docs: `docs/review/m4/stabilize-worktree-patch-stack-and-explicit-change-object.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet for change-object orchestration are current and referenced by the stable proof index.
- [x] Daily Git/review workflows stay previewable, attributable, and reversible.
- [x] Provider-linked or browser-handoff behavior is explicit about freshness and ownership.
- [x] Worktree/patch-stack fixtures prove that parent, child, and sibling repo change objects remain distinct in preview, publish, rollback, and support-export paths.
- [x] Change objects carry repo-root identity, submodule/nested-repo boundary, shallow/partial state, and pointer-backed asset posture.

## How to verify

```bash
cargo test -p aureline-review --test stabilize_worktree_patch_stack_and_explicit_change_object_alpha
```

## Risks / follow-ups

- The module currently uses opaque refs to `aureline_git::change_objects` records. When the Git crate provides strongly-typed change-object projections, the orchestration packet constructor should be updated to consume them directly.
- Provider publish postures are modeled as strings; when the provider crate stabilizes its publish-lane enums, these should be narrowed to typed enums.
