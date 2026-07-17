# M5 ready-for-review-handoff-sheet and publish-action registries

Implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the matrix's `ready_for_review_handoff`
object class operable by carrying resolved, honest projections of two registries so the work-item
detail, review detail, Git / worktree, provider-handoff, help / docs, and support / export surfaces
inherit one canonical ready-for-review handoff descriptor rather than a hand-authored parallel prose
that has to be kept consistent. It packages the current engineering state into one explicit
ready-for-review handoff that stays honest about what will publish now, what stays local, and what
evidence is attached, reusing the already-landed provider-boundary work-item components, hosted-review
rows, Git worktree identity, AI evidence rows, and review-pack / local-parity truth.

## Registry-A — ready-for-review handoff sheet

One reusable, machine-readable ready-for-review handoff sheet per tracked work item, summarizing:

- a stable sheet identity that survives export packets, support bundles, and reopened work-item
  workspaces;
- the changed scope the sheet packages;
- the checks / test state;
- the linked review object;
- the comment draft;
- the attached evidence;
- the provider mutation list (each side effect a publish would perform, disclosed separately);
- the export-versus-publish actions;
- the resolution-form coverage (canonical object, accessible summary, audit record).

Summary-first evidence ordering is preserved: changed files, failing tests, the review target, and
profile / incident references always appear before raw logs, diffs, or large attachments. A sheet that
runs raw logs / diffs / large attachments ahead of the summary, that is a hand-copied per-item
assumption instead of tracing to the shared registry, or that publishes an incomplete object degrades
honestly instead of implying an acceptance the target has not given. The registry reuses the matrix
`m5-change-intent.schema.json` domain schema for the tracked item and the
`m5-ready-for-review-handoff-sheet.schema.json` domain schema for the sheet layout.

## Registry-B — publish-action

The typed outcome a handoff can take when it is packaged, keeping the publish mode and commit state
explicit — publish now, queue for publish, or export a local packet — so a local-only draft or queued
publish never reads as a provider-committed update and a target that is offline, policy-blocked, or
only partially writable stays visible and actionable instead of implying provider acceptance. The
registry keeps the publish modes distinct rather than flattening them into one generic action. The
registry reuses the matrix `m5-ready-for-review-handoff-sheet.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can compare publish-now, queue-for-publish, and export-local-packet outcomes from the same
   handoff sheet: every surface resolves the same sheet and publish-action from the shared registry,
   and a sheet that would let a local-only draft or queued publish read as provider-committed degrades
   instead of reading as a clean render.
2. Handoff sheets never imply provider acceptance when the target is offline, policy-blocked, or only
   partially writable: the publish-action and commit state stays visible in the UI projection, the
   CSV / export, and the support packet instead of a local packet reading as a provider-committed
   update.
3. Summary-first evidence ordering is preserved and no attached evidence is dropped; the binding
   registry keeps each publish-action dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-ready-for-review-handoff-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
