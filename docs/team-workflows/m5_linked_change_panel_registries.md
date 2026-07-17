# M5 linked-change-panel and linked-change-relation registries

Implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the matrix's `linked_change_panel`
object class operable by carrying resolved, honest projections of two registries so the work-item
detail, review detail, Git / worktree, linked-change, provider-handoff, help / docs, and
support / export surfaces inherit one canonical linked-change descriptor rather than a hand-authored
parallel prose that has to be kept consistent. It keeps the active engineering artifacts tied to the
tracked item in one reusable linked-change panel that stays explicit about relation source,
freshness, and blockage, reusing the already-landed provider-boundary work-item components,
hosted-review rows, Git worktree identity, AI evidence rows, and review-pack / local-parity truth.

## Registry-A — linked-change panel

One reusable, machine-readable linked-change panel per tracked work item, showing:

- a stable panel identity that survives export packets, support bundles, and reopened work-item
  workspaces;
- the branch / worktree state the change is bound to;
- the hosted review state;
- the validation summary;
- the AI run / evidence refs;
- the incident / docs links;
- the relation-source class each artifact carries (linked by provider, linked locally, suggested by
  Aureline, stale or broken relation);
- the resolution-form coverage (canonical object, accessible summary, audit record).

A panel that cannot bind an artifact to its relation source, that is a hand-copied per-item
assumption instead of tracing to the shared registry, or that publishes an incomplete object degrades
honestly instead of flattening the relation sources into one generic badge. The registry reuses the
matrix `m5-change-intent.schema.json` domain schema for the tracked item and the
`m5-linked-change-panel.schema.json` domain schema for the panel layout.

## Registry-B — linked-change relation

The typed relation each linked change sits in, keeping the relation source and freshness explicit —
linked by provider, linked locally, suggested by Aureline, a stale relation, a broken relation, or
queued for publish — so a locally linked, suggested, or stale relation never reads as a
provider-authoritative link and a stale or broken relation stays visible and actionable instead of
collapsing into missing context. The registry keeps the relation sources distinct rather than
flattening them into one generic badge. The registry reuses the matrix
`m5-linked-change-panel.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Work-item detail, review detail, Git / worktree, and support / export surfaces can all render the
   same linked-change truth without contradiction: every surface resolves the same panel and relation
   from the shared registry, and a surface that would let a locally linked or suggested relation read
   as provider-authoritative degrades instead of reading as a clean render.
2. Stale or broken relations remain visible and actionable instead of collapsing into missing
   context: the relation-source and freshness state stays visible in the UI projection, the
   CSV / export, and the support packet instead of a dead link reading as live.
3. The relation sources are never flattened into one generic badge, and no linked evidence is dropped;
   the binding registry keeps each relation dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-linked-change-panel-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
