# M5 managed-workspace-lifecycle-card and suspend-resume-rebuild-review-sheet controls

This is the third implement lane over the frozen
[M5 build/remote-boundary component matrix](m5_build_remote_boundary_components_contract.md). It
turns the two managed-workspace components the matrix names — the **managed-workspace lifecycle
card** and the **suspend/resume/rebuild review sheet** — into resolvers that produce export-safe,
honest projections instead of operator-only lifecycle prose.

The authoritative gate is the Rust validator in `crates/aureline-remote`
(`implement_the_m5_managed_workspace_lifecycle_card_and_suspend_resume_rebuild_review_sheet_..._primitive`).
The checked-in support export under
`artifacts/release/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls-proof/`
and the narrowed fixtures under
`fixtures/ui/m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls/` are
minted only by the seed builders through the headless emitter
`cargo run -p aureline-remote --example dump_m5_managed_lifecycle_controls`.

## Goal

Make managed-workspace lifecycle an everyday reviewed concept instead of an operator-only
implementation detail, so a user can tell whether a workspace resumed, rebuilt, recreated, or
degraded to local-safe continuation — and what changed materially — before they trust or mutate it.

## Components

### Managed-workspace lifecycle card

`resolve_managed_workspace_lifecycle_card` renders every card with:

- the current lifecycle state (`provision`, `warm`, `ready`, `suspended`, `resumed`,
  `reconnecting`, `rebuild_required`, `recreate_required`, `expired`, or
  `local_safe_continuation`), bound from `LifecycleStateClass`,
- the persistence class (bound from `PersistenceClass`),
- the continuity class (bound from `ContinuityClass`),
- the expiry timing (bound from `ExpiryClass`) whenever an expiry window governs the state, and
- the recovery options and local-safe continuation affordance offered on an outage / expiry state.

A card that leaves its lifecycle state, persistence class, or continuity class unstated degrades
rather than reading as a clean pass. A card that claims exact continuity over a material change
degrades to `exact_continuity_overclaimed`, and an outage / expiry state that hides local-safe
continuation degrades to `local_safe_continuation_unavailable`, so a materially different workspace
never masquerades as the one the user last saw and local-safe continuation is never hidden.

### Suspend / resume / rebuild review sheet

`resolve_suspend_resume_rebuild_review_sheet` renders every sheet with:

- the action class it gates (bound from `M5ManagedWorkspaceAction`),
- the template / image provenance (bound from `ProvenanceClass`),
- the changed persistence class (bound from `PersistenceClass`),
- the preserved-vs-lost state and reattach / rerun consequences, and
- the continuity class and any caveats (bound from `ContinuityClass` / `CaveatClass`).

A sheet that leaves its action class, template / image provenance, changed persistence class,
preserved-vs-lost state, or consequences unstated degrades. A sheet that claims exact continuity
over a material change degrades to `exact_continuity_overclaimed`. A sheet that would appear after
the destructive / continuity-affecting action it gates degrades to `review_shown_after_commit`
(AC2), so lifecycle review always happens before the fact.

## Acceptance criteria

- **AC1** — a user can tell whether a workspace resumed, rebuilt, recreated, or degraded to
  local-safe continuation, and what changed materially.
- **AC2** — lifecycle review sheets appear before destructive or continuity-affecting actions rather
  than after the fact.
- **AC3** — companion, preview, and support/export paths reuse the same lifecycle cards and review
  language.

Each criterion is proven by the resolved examples the packet carries, not merely asserted by
governance flags.

## Hard invariants (every controls row)

- never imply exact continuity after a material change,
- never hide local-safe continuation or companion handoff behind overflow-only affordances,
- never let a review sheet appear after the destructive / continuity-affecting action it gates, and
- never conceal lifecycle or continuity truth behind generic status wording.

Raw secret values and private endpoints never cross this boundary.
