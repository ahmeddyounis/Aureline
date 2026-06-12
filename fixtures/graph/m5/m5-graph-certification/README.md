# Fixtures: M5 graph-depth certification report

This directory contains fixture metadata for the `m5_graph_certification_report` packet.

The canonical full corpus is checked in at:

`artifacts/graph/m5/m5-graph-certification.json`

## Coverage

- `workset_scope`, `graph_topology`, `impact_query`, `ownership_source`,
  `architecture_explainer`, `graph_freshness`, and `navigation_recall` are the only claimed
  subjects, and each carries exactly one row — no row inherits a certification from an adjacent
  one.
- Every row binds to its row in the governance matrix at
  `artifacts/graph/m5/m5-graph-governance.json` via `governance_row_ref`, and its
  `governance_claim` equals that row's `published_claim`, so the certification ingests the
  governance packet rather than a parallel spreadsheet.
- Every row covers all seven required drills — `workset_scope`, `topology_identity`,
  `impact_query`, `ownership_source`, `explainer_citation`, `accessibility`, and `export_join`
  — exactly once, and every drill that ran carries an `evidence_ref`.
- Drill outcomes cover `passed`, `narrowed`, `failed`, and `not_run`; evidence freshness covers
  `current`, `aging`, `expired`, and `missing`. The recovery path covers `rerun_drills`,
  `refresh_evidence`, `adopt_governance_narrowing`, `withhold_row`, and `none`, and the four
  downgrade reasons — `governance_narrowed`, `evidence_stale`, `drill_narrowed`, and
  `drill_failed` — are each exercised by at least one row.
- Published label covers `authoritative`, `scope_qualified`, `provisional`, and `withheld`, and
  the certification decision covers `publish`, `qualify_scope`, `mark_provisional`, and
  `withhold`.
- The gate is exercised in every direction: the clean `graph_topology` and `ownership_source`
  rows certify authoritative; the governance-qualified `workset_scope` row adopts that
  narrowing to scope-qualified; the aging/narrowed `impact_query`, the expired
  `architecture_explainer`, and the aging `graph_freshness` rows are marked provisional; and
  the governance-withheld, missing-evidence, failed/not-run `navigation_recall` row is withheld
  entirely. `graph_topology` and `ownership_source` prove the certifier is not a blanket
  downgrade, while the five narrowed or withheld rows prove a stale or unproven row cannot stay
  green by inertia.
- Every row's `published_label`, `certification_decision`, `downgrade_reasons`, and
  `downgrade_path` equal the recomputed gate, the published label never exceeds the
  `governance_claim`, and all six consumer surfaces — `release_evidence`, `docs_help`,
  `onboarding`, `review`, `ai_context`, and `support_export` — bind to one packet, so release,
  docs/help, onboarding, review, AI context, and support exports ingest the same certification
  and a narrowed row cannot stay authoritative downstream.
