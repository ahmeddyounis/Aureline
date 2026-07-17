# M5 stack-edit review sheet and disposition registries

Implement lane over the frozen [M5 change-object / patch-stack / landing matrix][matrix]
(`m5_change_object_patch_stack_and_landing_matrix`). It makes the matrix's `stack_edit_review_sheet`
object class operable — as a per-operation review sheet and the disposition of the plan it proposes —
by carrying resolved, honest projections of two registries so the Git, patch-stack / queue, review,
provider-landing, help / docs, and support / export surfaces inherit one canonical stack-edit review
descriptor and one disposition posture rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed AI branch / worktree agent lifecycle,
merge-readiness and stack-dependency chips, Git worktree / history / rebase mutation review, work-item
change-intent / start-work / handoff flows, review bundles, and provider mutation boundaries and the
explicit stacked-change orchestration contract the source set now expects: reorder, split, squash, and
restack are reviewed before apply, and ordering, parent/child consequence, stale-evidence impact, and
target-branch disclosure stay visible metadata instead of implicit.

The goal: review stack-edit operations before apply so Aureline never silently reorders or rewrites a
change series without disclosing parent-child impact, stale evidence, and target-branch consequences.

## Registry-A — stack-edit review sheet

One durable, canonical review sheet per proposed reorder, split, squash, or restack, carrying:

- the **original order** of the change series before the edit;
- the **proposed order** the edit would produce, so a reorder is disclosed before apply and never
  executed from ambient branch state;
- the **affected parent/child links**, so which member the edit reparents, splits, or squashes into
  another is explicit;
- the **stale validation or approval impact**: which local checks and which hosted approvals the
  proposed edit invalidates;
- the **resulting branch/worktree consequences**: the target branch and worktree the plan retargets,
  disclosed rather than silently changed;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A review sheet that cannot bind its original order to its proposed order, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete object
degrades honestly instead of letting a stack edit execute without a review surface. The registry
reuses the matrix `m5-stack-edit-review-sheet.schema.json` domain schema.

## Registry-B — stack-edit disposition

Each proposed stack edit keeps its explicit **continue**, **abort**, **export**, and **defer** paths
available, so a proposed re-stack plan is never lost when a provider write, a hosted approval, a local
validation, or a policy boundary goes stale or blocks publish. The disposition names the stack-edit
operation under review, the source of any staleness that gates a continue (provider state stale,
approval stale, local validation stale, or policy boundary blocks publish), the currently offered
dispositions, the resolved disposition state, and whether the proposed plan stays preserved for retry
or export — so a local-only continue is never shown as a provider-committed landing. The registry
reuses the newly minted `m5-stack-edit-disposition.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. No stack edit executes without a review surface showing ordering, dependency, stale-evidence, and
   landing consequences: a review sheet missing its original-versus-proposed ordering degrades instead
   of reading as a reviewed, apply-ready plan, and the original order, proposed order, affected
   parent/child links, stale-evidence impact, and resulting branch/worktree consequences stay visible
   in the UI projection, the CSV / export, and the support packet.
2. Users can export or defer the proposed re-stack plan instead of losing it when a provider or policy
   boundary blocks publish: the continue / abort / export / defer disposition and the `proposed plan
   preserved` posture stay distinct instead of collapsing into a generic status pill, and a
   publish-blocked disposition keeps the plan preserved for retry or export.
3. No worktree is mutated without an explicit selected change object and binding, stack members are
   never silently reordered, collapsed, or retargeted, and nothing lands from ambient branch state; the
   disposition registry keeps each path distinct and never lets a local-only continue read as a
   provider-committed landing.

Raw paths, raw provider payloads, secret values, and private endpoints never cross this boundary.
The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in combined
registries schema
(`schemas/teamwork/m5-stack-edit-review-sheet-and-disposition-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_object_patch_stack_and_landing_matrix/mod.rs
