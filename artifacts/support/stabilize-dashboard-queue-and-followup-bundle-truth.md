# Dashboard, Queue, and Follow-up Bundle Truth Evidence

This artifact records the stable support/export contract for dashboard
freshness, queue-order explainability, and provider-linked follow-up bundles.

## Canonical Inputs

- Rust model:
  `crates/aureline-support/src/stabilize_dashboard_queue_and_followup_bundle_truth/`
- Schema: `/schemas/support/followup-bundle.schema.json`
- Fixtures:
  `/fixtures/support/stabilize-dashboard-queue-and-followup-bundle-truth/canonical_packet.json`
  and
  `/fixtures/support/stabilize-dashboard-queue-and-followup-bundle-truth/support_export_projection.json`
- Support doc:
  `/docs/support/stabilize-dashboard-queue-and-followup-bundle-truth.md`

## Evidence Covered

- Dashboard cards with imported, cached, and blocked source data visibly
  downgrade from green and retain open-evidence refs.
- Queue rows expose rank, ordering reason, grouping reason, provider blockers,
  policy blockers, and hidden-scope counts.
- Follow-up bundle checklist completion is non-mutating for linked
  provider-owned objects.
- Provider mutation commands are separate reviewed records with exact target,
  actor, and mode.
- Support export preserves bundle scope, owner, filters, freshness, linked
  object refs, ownership tokens, checklist semantics, and reviewed commands.

## Stable-Line Rule

A dashboard, queue, or follow-up bundle that cannot validate against the packet
model must narrow or drop its stable claim until freshness, ordering,
ownership, mutation, and export parity are explicit again.
