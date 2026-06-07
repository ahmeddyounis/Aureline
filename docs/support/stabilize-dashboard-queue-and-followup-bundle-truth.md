# Dashboard, Queue, and Follow-up Bundle Truth

Stable support, observability, provider-linked follow-up, copy, and export
surfaces consume the same packet:

- Schema: `/schemas/support/followup-bundle.schema.json`
- Fixture corpus:
  `/fixtures/support/stabilize-dashboard-queue-and-followup-bundle-truth/`
- Rust source:
  `crates/aureline-support/src/stabilize_dashboard_queue_and_followup_bundle_truth/`

## Dashboard Freshness

Dashboard cards carry source, scope, freshness, and an open-evidence reference.
A card whose upstream data is cached, stale, imported, truncated, or blocked
cannot remain healthy when it was declared green. It must render a downgraded
or blocked effective state, include a downgrade reason token, and keep the
evidence object addressable with a canonical `aureline://` reference.

## Queue Explainability

Follow-up queues carry row order and hidden-scope truth in the packet. Each row
has a one-based rank, order reason, grouping reason, and open ref. Hidden scope
records include the narrowing reason and count, so a queue can explain whether
the visible set was narrowed by policy, provider scope, workspace scope,
freshness floor, or user filters.

## Follow-up Bundles

Follow-up bundles preserve scope, owner, filter state, freshness, linked object
refs, ownership class, and checklist semantics. Checklist completion is local:
it may close a reminder or record evidence, but it does not mutate a linked
provider-owned object.

Provider-owned mutation is represented only by a separate reviewed command. The
command must name the exact target, actor, and mode: `draft`, `publish-later`,
`publish-now`, or `handoff-only`.

## Export Parity

Support/export packets preserve the same meaning outside the live shell:

- scope and owner
- active filters and hidden-scope counts
- freshness token
- linked object refs and ownership tokens
- checklist completion semantics
- reviewed provider mutation commands

If any of those fields cannot be preserved, the surface must not make a stable
claim for that export or follow-up bundle.
