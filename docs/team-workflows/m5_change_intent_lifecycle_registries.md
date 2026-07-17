# M5 change-intent lifecycle-state and reconcile-flow registries

Implement lane over the frozen [M5 change-intent-and-engineering-lifecycle matrix][matrix]
(`m5_change_intent_and_engineering_lifecycle_matrix`). It makes the change-intent lifecycle resilient
across local drafts, queued publishes, provider failure, stale review links, and relation drift by
carrying resolved, honest projections of two registries so the work-item detail, review detail,
start-work sheet, linked-change panel, provider-handoff, help / docs, and support / export surfaces
inherit one canonical lifecycle-state descriptor rather than a hand-authored parallel prose that has to
be kept consistent. A change-intent that moves `local draft` → `queued publish` → `provider linked` →
`stale / broken relation` → `reconcile required` → `superseded` stays one typed record with explicit
provider ownership, commit state, linked branch / worktree / review identity, relation source, and
validation evidence, reusing the already-landed provider-boundary work-item components, hosted-review
rows, Git worktree identity, queued-publish packets, AI evidence rows, and review-pack / local-parity
truth.

## Registry-A — change-intent lifecycle-state record

One reusable, machine-readable lifecycle-state record per tracked change-intent, showing:

- a stable record identity that survives export packets, support bundles, and reopened work-item
  workspaces;
- the provider ownership;
- the local-versus-provider commit state (local draft, queued publish, provider linked,
  publish-failed-retained, provider-unavailable, offline-handoff-packet, or stale-relative-to-provider);
- the linked branch / worktree / review identity;
- the relation source (linked by provider, linked locally, suggested by Aureline, or stale-or-broken
  relation);
- the retained local packet / linked evidence and actor lineage;
- the resolution-form coverage (canonical object, accessible summary, audit record).

Provider ownership, commit state, and linked engineering identity are always named before a relation
can read as trusted. A record that would keep a stale or broken branch / review / provider relation
green after drift is detected, that is a hand-copied per-item assumption instead of tracing to the
shared registry, that drops its reconcile / export / retry path, or that publishes an incomplete object
degrades honestly instead of implying a provider commit that has not happened. The registry reuses the
matrix `m5-change-intent.schema.json` domain schema for the tracked item and the
`m5-change-intent-lifecycle.schema.json` domain schema for the lifecycle-state layout.

## Registry-B — reconcile-flow

The typed compare-reconcile flow a lifecycle-state record resolves toward when provider state diverges
from the local handoff packet, the linked branch / worktree, or the hosted review target, keeping the
relation source and commit state explicit so a queued publish or local handoff packet never reads as a
provider-committed update and a stale, broken, or reconcile-required relation stays visible and
actionable instead of holding a green badge. The registry keeps the linked-by-provider / linked-locally
/ suggested-by-Aureline / stale-or-broken relation sources distinct rather than flattening them into one
generic badge, and preserves the original local packet, linked evidence, and actor lineage for retry and
support export. The registry reuses the matrix `m5-change-intent-lifecycle.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Change-intent surfaces never keep a stale branch / review / provider relation green after drift is
   detected: every surface resolves the same lifecycle-state record and reconcile flow from the shared
   registry, and a record that would keep a stale or broken relation trusted, or clear a relation while
   an engineering blocker remains, degrades instead of reading as a clean render.
2. Queued-publish and reconcile flows preserve the original local packet, linked evidence, and actor
   lineage for retry and support export: the commit state, relation source, and retained local evidence
   stay visible in the UI projection, the CSV / export, and the support packet, and a local draft or
   queued publish never reads as a provider-committed update.
3. Local draft, queued publish, provider linked, stale relation, and broken relation stay differentiated
   in both live UI and exported packets; no local packet, linked evidence, or actor lineage is dropped
   when a provider write fails, and the binding registry keeps each relation-source dimension distinct.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/teamwork/m5-change-intent-lifecycle-registries.schema.json`) documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_change_intent_and_engineering_lifecycle_matrix/mod.rs
