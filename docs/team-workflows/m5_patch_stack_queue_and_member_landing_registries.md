# M5 patch-stack / queue and member-landing registries

Implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `patch_stack_queue`
object class operable — as a durable stack record and an ordered member-card view — by carrying
resolved, honest projections of two registries so the Git, patch-stack / queue, review,
provider-landing, help / docs, and support / export surfaces inherit one canonical patch-stack
descriptor and one member-landing posture rather than a hand-authored parallel prose that has to be
kept consistent. It closes the gap between the already-landed AI branch / worktree agent lifecycle,
merge-readiness and stack-dependency chips, Git worktree / history / rebase mutation review, work-item
change-intent / start-work / handoff flows, review bundles, and provider mutation boundaries and the
explicit stacked-change orchestration contract the source set now expects: stacked changes are
first-class, and stack membership, order, parent/child dependencies, landing intent, and stale
validation stay visible metadata instead of implicit.

## Registry-A — patch-stack record

One durable, canonical patch-stack record per stacked change, carrying:

- a stable stack ID that survives export packets, support bundles, portable shelves, and reopened
  change-object workspaces;
- the ordered member IDs, kept as visible metadata so a stack member position never has to be
  inferred from a branch-naming convention;
- the parent/child relation, so which member blocks another is explicit;
- the landing order and the rebase epoch the stack carries forward;
- the inherited blockers a member picks up from an ancestor member;
- the current validation freshness of each member;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A stack row that cannot bind its stack ID to its ordered-member set, that is a hand-copied per-entry
assumption instead of tracing to the shared registry, or that publishes an incomplete object degrades
honestly instead of reading as a reviewed, landing-ready stack. The registry reuses the matrix
`m5-patch-stack-queue.schema.json` domain schema.

## Registry-B — member-landing posture

Each stack member card keeps its **local stack state**, its **provider-linked review state**, and
its **queue/landing posture** explicitly separate, so a member's local stack state is never shown as
a provider-accepted landing candidate and the queue-eligible / queue-blocked / protected-branch-blocked
posture stays inspectable. The posture names the member's membership source (declared in the change
object, declared locally, inferred from a branch name, stale or broken) distinctly rather than
flattening the four sources into one generic badge, and never infers stack membership from branch
names alone. The registry reuses the newly minted `m5-stack-member-landing.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Stack membership is visible metadata rather than a branch-naming convention: a member whose
   membership is inferred from a branch name alone degrades instead of reading as reviewed stack
   membership, and the stack ID, ordered member IDs, and parent/child relation stay visible in the UI
   projection, the CSV / export, and the support packet.
2. Users can tell which member blocks another, which checks are stale, and which members are ready,
   blocked, or not yet landing candidates: the parent/child relation, the validation freshness, and
   the queue/landing posture stay distinct instead of collapsing into a generic status pill.
3. No worktree is mutated without an explicit selected change object and binding, stack members are
   never silently reordered, collapsed, or retargeted, and nothing lands from ambient branch state;
   the member-landing registry keeps each disclosure dimension distinct.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-patch-stack-queue-and-member-landing-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
