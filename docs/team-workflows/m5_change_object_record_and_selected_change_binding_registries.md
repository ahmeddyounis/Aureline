# M5 change-object-record and selected-change-binding registries

First implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `change_object`
object class operable — as a durable record — and adds the selected-change binding a broad-scope
flow must pass, by carrying resolved, honest projections of two registries so the Git,
patch-stack / queue, review, provider-landing, help / docs, and support / export surfaces inherit
one canonical change-object descriptor and one selected-change binding rather than a hand-authored
parallel prose that has to be kept consistent. It closes the gap between the already-landed AI
branch / worktree agent lifecycle, merge-readiness and stack-dependency chips, Git worktree /
history / rebase mutation review, work-item change-intent / start-work / handoff flows, review
bundles, and provider mutation boundaries and the explicit multi-change orchestration contract the
source set now expects: every non-trivial multi-file change binds to an explicit change object with
selected worktree / base identity before any broad mutation runs.

## Registry-A — change-object record

One durable, canonical change-object record per non-trivial multi-file change, carrying:

- a stable record identity that survives export packets, support bundles, portable shelves, and
  reopened change-object workspaces;
- the change-object kind, kept mechanically distinct so a bounded **working-set patch** never reads
  as a **side-branch work unit** (and vice-versa);
- the worktree ID and the base commit or dirty-tree fingerprint the change is bound to;
- the intent class and the affected path set the change touches;
- the validation plan and the checkpoint lineage the change carries forward;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A record that cannot bind its identity to its selected worktree / base identity, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes an
incomplete object degrades honestly instead of reading as a reviewed, landing-ready change. The
registry reuses the matrix `m5-change-object.schema.json` domain schema.

## Registry-B — selected-change binding

The typed binding surfaced before any broad-scope flow — a broad refactor, a migration / import, a
scaffold / update flow, or a provider-backed mutation — so the selected change object and the
explicit worktree identity are named before the flow can begin, and a broad mutation is never run
against ambient branch state. The binding keeps the four stack-membership sources (declared in the
change object, declared locally, inferred from a branch name, stale or broken) distinct rather than
flattening them into one generic badge, and never infers stack membership from branch names alone.
The registry reuses the newly minted `m5-selected-change-binding.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Broad-scope flows cannot begin without a selected change object and explicit worktree identity:
   an unbound broad flow degrades instead of reading as a clean start, so no broad refactor,
   migration / import, scaffold / update flow, or provider-backed mutation runs against ambient
   branch state.
2. Users can inspect current base / worktree / intent / checkpoint truth before applying or
   exporting the change: the worktree ID, base commit or dirty-tree fingerprint, intent class,
   affected path set, validation plan, and checkpoint lineage stay visible in the UI projection, the
   CSV / export, and the support packet instead of collapsing into a generic status pill.
3. Stack membership is never inferred from branch names alone, no worktree is mutated without an
   explicit selected change object and binding, and stack members are never silently reordered,
   collapsed, or retargeted; the binding registry keeps each disclosure dimension distinct.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-change-object-record-and-selected-change-binding-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
