# M5 resolve-or-close-sheet and resolution-outcome registries

Implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the matrix's `resolve_close_sheet`
object class operable by carrying resolved, honest projections of two registries so the work-item
detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces
inherit one canonical resolve-or-close descriptor rather than a hand-authored parallel prose
that has to be kept consistent. It makes a terminal status transition explicit through one
resolve-or-close sheet that stays honest about what the provider has actually accepted, what stays
local, and what remains blocked, reusing the already-landed provider-boundary work-item components,
hosted-review rows, Git worktree identity, AI evidence rows, and review-pack / local-parity truth.

## Registry-A — resolve-or-close sheet

One reusable, machine-readable resolve-or-close sheet per tracked work item, showing:

- a stable sheet identity that survives export packets, support bundles, and reopened work-item
  workspaces;
- the current state;
- the requested terminal state;
- the unresolved blockers;
- the linked evidence;
- the permission scope;
- the confirm / reopen / export actions (each side effect disclosed separately);
- the final side-effect preview;
- the resolution-form coverage (canonical object, accessible summary, audit record).

Blockers, provider authority, and reopen behaviour are always named before the transition can be
confirmed. A sheet that would confirm a terminal state while an engineering blocker remains
unresolved, that is a hand-copied per-item assumption instead of tracing to the shared registry, that
drops its reopen / export path, or that publishes an incomplete object degrades honestly instead of
implying an acceptance the provider has not given. The registry reuses the matrix
`m5-change-intent.schema.json` domain schema for the tracked item and the
`m5-resolve-close-sheet.schema.json` domain schema for the sheet layout.

## Registry-B — resolution-outcome

The typed terminal outcome a resolve-or-close can take, keeping the resolution mode and commit state
explicit — resolved locally, provider updated, queued for publish, blocked by missing permission, or
blocked by unresolved engineering state — so a local-only resolution never reads as a provider-accepted
terminal state and a target that is offline, policy-blocked, or only partially writable stays visible
and actionable instead of implying provider acceptance. The registry keeps the resolution outcomes
distinct rather than flattening them into one generic close. The registry reuses the matrix
`m5-resolve-close-sheet.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Resolve flows preserve reopen / export continuity when provider mutation fails or remains queued:
   every surface resolves the same sheet and resolution-outcome from the shared registry, and a sheet
   that would drop the reopen / export path or confirm a terminal state while an engineering blocker
   remains degrades instead of reading as a clean render.
2. No claimed M5 surface treats a local-only resolution as if the provider already accepted the
   terminal state: the resolution-outcome and commit state stay visible in the UI projection, the
   CSV / export, and the support packet instead of a local-only resolution reading as a
   provider-updated close.
3. Resolved locally, provider updated, queued for publish, blocked by missing permission, and blocked
   by unresolved engineering state stay differentiated in both live UI and exported packets; no linked
   evidence is dropped and the binding registry keeps each resolution-outcome dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-resolve-close-sheet-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
