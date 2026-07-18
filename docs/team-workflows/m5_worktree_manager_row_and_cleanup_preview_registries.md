# M5 worktree-manager-row and cleanup-preview registries

Implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `worktree_manager_row`
object class operable — as a worktree manager / switcher row and the cleanup preview that safely
removes it — by carrying resolved, honest projections of two registries so the Git, patch-stack /
queue, review, provider-landing, help / docs, and support / export surfaces inherit one canonical
worktree-manager-row descriptor and one cleanup-preview posture rather than a hand-authored parallel
prose that has to be kept consistent. It closes the gap between the already-landed AI branch / worktree
agent lifecycle, merge-readiness and stack-dependency chips, Git worktree / history / rebase mutation
review, work-item change-intent / start-work / handoff flows, review bundles, and provider mutation
boundaries and the explicit change-orchestration continuity contract the source set now expects:
alternate working contexts are made discoverable and recoverable, and the real worktree path, checked-out
ref, divergence, dirty state, running-task / open-editor presence, and recovery / checkpoint lineage stay
visible metadata instead of implicit.

The goal: make alternate working contexts discoverable and recoverable by shipping worktree manager /
switcher rows and cleanup previews for orphaned worktrees, abandoned side branches, and stale stack
members.

## Registry-A — worktree-manager row

One durable, canonical switcher row per alternate working context, carrying:

- the **real worktree path** and the **checked-out ref**, disclosed rather than inferred from ambient
  branch state;
- the **divergence** from base and the **dirty / uncommitted state** of the worktree;
- the **running-task / open-editor presence** so a worktree that still holds live work is never treated
  as idle;
- the **recovery / checkpoint lineage** (reflog and checkpoint references) that keeps the worktree
  recoverable;
- the active-versus-orphaned-versus-abandoned-versus-cleanup-ready state, so a user can distinguish
  active, orphaned, abandoned, and cleanup-ready worktrees and side branches without shelling out;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A worktree-manager row that cannot bind its path to its checked-out ref, that is a hand-copied per-entry
assumption instead of tracing to the shared registry, or that publishes an incomplete object degrades
honestly instead of letting Aureline treat ambient branch state as a discoverable, recoverable worktree.
The registry reuses the matrix `m5-worktree-manager-row.schema.json` domain schema.

## Registry-B — cleanup preview

Each worktree keeps its explicit **cleanup preview** available, so an orphaned worktree or stale stack
member stays blocked from removal for background agents and broad automation unless the user explicitly
previews and confirms the cleanup. Cleanup never feels like `rm -rf and hope`: the preview names the
worktree status (active, orphaned, abandoned, or cleanup-ready), the affected running work it must
preserve (running tasks, open editors, and the uncommitted-change scope), the recovery posture (reflog /
checkpoint recovery and export-safe evidence), the removing actor (an explicit user command, a background
agent, or broad automation), and the resolved cleanup disposition (preview only, cleanup-ready confirmed,
blocked on a running task, blocked on an open editor, blocked on uncommitted changes, or recoverable via
reflog / checkpoint) — so a cleanup is never confirmed while affected work, recovery paths, and
export-safe evidence stay hidden. The registry reuses the newly minted
`m5-worktree-cleanup-preview.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can distinguish active, orphaned, abandoned, and cleanup-ready worktrees / side branches without
   shelling out: a worktree-manager row missing its path or checked-out ref degrades instead of reading as
   a discoverable, recoverable worktree, and the real path, checked-out ref, divergence, dirty state, and
   running-task / open-editor presence stay visible in the UI projection, the CSV / export, and the support
   packet.
2. Cleanup never feels like `rm -rf and hope`; the preview names affected work, recovery paths, and what
   stays exportable: the worktree status, the removing actor, and the cleanup disposition stay distinct
   instead of collapsing into a generic status pill, and a cleanup blocked on a running task, open editor,
   or uncommitted change keeps the affected work preserved for export or retry through reflog / checkpoint
   recovery.
3. No worktree is mutated without an explicit selected change object and binding, stack members are never
   silently reordered, collapsed, or retargeted, and nothing is deleted from ambient branch state; the
   cleanup-preview registry keeps each removal-safety posture distinct and never lets a stale stack member
   read as safe-to-delete.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-worktree-manager-row-and-cleanup-preview-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
