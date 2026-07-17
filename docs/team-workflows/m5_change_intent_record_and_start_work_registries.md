# M5 change-intent-record and start-work-sheet registries

First implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the matrix's `change_intent_record`
and `start_work_sheet` object classes operable by carrying resolved, honest projections of two
registries so the work-item, start-work, linked-change, provider-handoff, help / docs, and
support / export surfaces inherit one canonical change-intent descriptor and start-work sheet
rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap
between the already-landed provider-boundary work-item components, local-draft / publish-now /
open-in-provider mutation flows, hosted-review rows, Git worktree identity, and queued-publish
packets and the governed loop a user actually follows from a tracked item into a branch, a
review, and a final resolution.

## Registry-A — change-intent record

One durable, machine-readable change-intent record per tracked work item, carrying:

- a stable record identity that survives export packets, support bundles, queued-publish packets,
  and reopened work-item workspaces;
- the canonical work-item identity plus provider ownership, kept mechanically distinct from a
  local-only draft so a local record never reads as a provider-committed update;
- the linked workspace / root and the proposed or existing branch / worktree refs the intent is
  bound to;
- the optional linked review target, and the task / test preset refs the intent carries forward;
- actor lineage and the local-versus-provider lifecycle state (provider-committed, local-only
  draft, queued for publish, publish-failed-retained, provider-unavailable, offline handoff
  packet, or stale-relative-to-provider);
- the resolution-form coverage (canonical object, accessible summary, audit record).

A record that cannot bind its work-item identity to its provider ownership, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete
object degrades honestly instead of reading as a committed, authoritative tracked-item update. The
registry reuses the matrix `m5-change-intent.schema.json` domain schema.

## Registry-B — start-work sheet

The typed sheet that launches work from a tracked item and separately discloses each side effect
it creates — create-new-linked branch / worktree, link-existing-branch-or-review, the provider-link
mutation, the optional task / test preset, and the local-only alternative — before commit, so a
branch, worktree, review draft, or provider link is never silently created and a local handoff
packet or queued publish never masquerades as a provider-committed update. The sheet keeps the
four relation sources (linked by provider, linked locally, suggested by Aureline, stale or broken
relation) distinct rather than flattening them into one generic badge. The registry reuses the
matrix `m5-start-work-sheet.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can start work from a tracked item without Aureline silently creating extra linked
   objects: every start-work side effect is disclosed as its own entry, and a sheet that would
   create a side effect without disclosure degrades instead of reading as a clean start.
2. Change-intent records survive local draft, queued publish, provider publish, and reopen flows
   with stable IDs and explicit authority state: the local-versus-provider lifecycle state stays
   visible in the UI projection, the CSV / export, and the support packet instead of collapsing a
   local draft or queued publish into a provider-committed reading.
3. No tracked item is auto-resolved while an engineering blocker remains, and no local notes,
   handoff packet, or linked evidence are dropped when a provider write fails; the binding registry
   keeps each disclosure dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-change-intent-record-and-start-work-registries.schema.json`) documents the
shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
