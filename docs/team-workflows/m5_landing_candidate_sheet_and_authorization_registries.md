# M5 landing-candidate sheet and authorization registries

Implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `landing_candidate_sheet`
object class operable — as a reviewed landing candidate and the authorization that advances it —
by carrying resolved, honest projections of two registries so the Git, patch-stack / queue, review,
provider-landing, help / docs, and support / export surfaces inherit one canonical landing-candidate
descriptor and one authorization posture rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed AI branch / worktree agent lifecycle,
merge-readiness and stack-dependency chips, Git worktree / history / rebase mutation review, work-item
change-intent / start-work / handoff flows, review bundles, and provider mutation boundaries and the
explicit stacked-change orchestration contract the source set now expects: a land is treated as an
explicit reviewed candidate rather than ambient branch state, and the target branch, merge strategy,
required checks, approval state, and queue eligibility stay visible metadata instead of implicit.

The goal: treat landing as an explicit reviewed candidate rather than ambient branch state by adding
landing-candidate sheets that disclose the exact target, strategy, required checks, approval state, and
queue eligibility.

## Registry-A — landing-candidate sheet

One durable, canonical reviewed candidate per proposed land, carrying:

- the **exact target branch** the land would write to, disclosed rather than inferred from ambient
  branch state;
- the **merge strategy**: a local squash plan, a local rebase plan, a merge-queue enqueue, or a
  review-ready export bundle;
- the **required checks** that gate the land, and the **approval state** of the reviewed candidate;
- the **queue eligibility**: whether the candidate is queue-eligible, queue-blocked, or
  protected-branch-blocked, with queue-position ambiguity kept explicit;
- the **validation freshness** and the provider-authoritative-versus-local-estimate distinction, so a
  stale base invalidates the candidate rather than reading as still-current;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A landing candidate that cannot bind its target branch to its merge strategy, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete object
degrades honestly instead of letting Aureline land from anything but an explicit reviewed candidate. The
registry reuses the matrix `m5-landing-candidate-sheet.schema.json` domain schema.

## Registry-B — landing authorization

Each reviewed candidate keeps its explicit **advance** paths available, so a protected branch stays
blocked for background agents and broad automation unless the user explicitly advances the reviewed
candidate through the correct command path. The authorization names the landing path under advance
(a local squash plan, a local rebase plan, a merge-queue enqueue, or a review-ready export bundle), the
authority source (provider-authoritative or local estimate), the advancing actor (an explicit user
command, a background agent, or broad automation), the protected-branch posture, the resolved
authorization state (reviewed-candidate advance-ready, advanced via the explicit command path, blocked
with no reviewed candidate, blocked on a protected branch, blocked on a stale base, or queue-position
ambiguous), and whether a reviewed candidate is bound — so a local estimate is never shown as a
provider-authoritative landing. The registry reuses the newly minted
`m5-landing-authorization.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Aureline only lands from an explicit reviewed landing candidate, never from ambient branch state: a
   landing candidate missing its target branch or merge strategy degrades instead of reading as a
   reviewed, land-ready candidate, and the target branch, merge strategy, required checks, approval
   state, and queue eligibility stay visible in the UI projection, the CSV / export, and the support
   packet.
2. Protected branches remain blocked for background agents and broad automation unless the user
   explicitly advances the reviewed candidate through the correct command path: the advancing actor,
   the protected-branch posture, and the authorization state stay distinct instead of collapsing into a
   generic status pill, and a protected-branch-blocked authorization keeps the reviewed candidate
   preserved for export or retry through the correct command path.
3. No worktree is mutated without an explicit selected change object and binding, stack members are
   never silently reordered, collapsed, or retargeted, and nothing lands from ambient branch state; the
   authorization registry keeps each advance path distinct and never lets a local estimate read as a
   provider-authoritative landing.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-landing-candidate-sheet-and-authorization-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
