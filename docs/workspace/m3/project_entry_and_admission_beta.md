# Project Entry and Admission Review Contract

Project entry is reviewed as a distinct verb before Aureline writes files,
widens workspace scope, grants trust, runs setup, or rehydrates prior state.
The workspace model lives in:

- `crates/aureline-workspace/src/entry/`
- `schemas/workspace/entry_review.schema.json`
- `fixtures/workspace/m3/entry_and_clone_review/`

## Review Model

Every entry activation builds one `project_entry_review_record`.
The record includes:

- the resolved `entry_verb`, `target_kind`, and `resulting_mode`;
- a verb-specific review sheet for open, open workspace, clone, add root,
  import, or restore;
- a shared source/trust/destination/write-scope/next-step vocabulary review;
- a destination collision review when the destination already contains data,
  a repository, a worktree, a workspace manifest, a nested repository, a prior
  clone target, or a policy block;
- a post-entry handoff card naming what Aureline intentionally did not run;
- failed-attempt repair state that preserves typed input, destination, and
  redacted diagnostics;
- cross-surface parity rows for Start Center, command palette, drag/drop,
  system file association, deep link, CLI/headless, and workspace switcher;
- the existing admission packet and admission checkpoint route.

## Entry Verb Rules

`Open`, `Open workspace`, `Clone repository`, `Add root`, `Import`, and
`Restore` stay separate review paths.

Clone review names normalized remote URL, host or certificate posture, auth
mode, branch or ref, clone depth, LFS and submodule posture, destination,
route posture, and post-clone action. Clone does not grant trust, run hooks,
restore dependencies, install bundles, or execute tasks.

Import review distinguishes inspect-only from write-capable behavior. It
surfaces artifact class, schema or version, lossy mapping posture, extraction
or restore target, machine-local exclusions, and cleanup posture before any
state is applied.

Restore review names source, target, retained state, replaced state, checkpoint
requirement, missing-dependency placeholder behavior, and the no-rerun rule.

Add-root review names the active workspace, new root, scope impact, per-root
trust review, and checkpoint requirement. It never inherits trust from sibling
roots without review.

## Failure And Recovery

Failed open, clone, import, add-root, or restore attempts preserve the typed
source input, chosen destination, and redacted diagnostics in
`failure_repair_state`. The safe repair actions depend on the verb: clone can
choose another destination or reveal the target, import can inspect only or set
up later, and restore/open paths can open minimal or defer setup.

Mirror-first, air-gapped, offline, and direct-online paths all use the same
source, trust, destination, write-scope, and next-step labels through
`EntryVocabularyReview`.
