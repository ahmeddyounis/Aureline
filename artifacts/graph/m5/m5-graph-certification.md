# M5 graph-depth certification report — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/graph/m5/m5-graph-certification.json`. The full contract and gate semantics live in
`docs/graph/m5/m5-graph-certification.md`; the typed model lives in the `aureline-graph` crate
(`m5_graph_certification`).

This artifact certifies the M5 graph and codebase-understanding rows by ingesting the
[graph-governance matrix](../../../docs/graph/m5/m5-graph-governance.md) and graduating each
row **only where its evidence is current and provable**. Stale, unproven, or governance-narrowed
rows are automatically downgraded to a narrower label before publication.

## Certification roll-up (as of 2026-06-11)

| Subject | Governance claim | Evidence | Published label | Decision | Recovery |
| --- | --- | --- | --- | --- | --- |
| `workset_scope` | scope_qualified | current | **scope_qualified** | qualify_scope | adopt_governance_narrowing |
| `graph_topology` | authoritative | current | **authoritative** | publish | none |
| `impact_query` | provisional | aging | **provisional** | mark_provisional | rerun_drills |
| `ownership_source` | authoritative | current | **authoritative** | publish | none |
| `architecture_explainer` | provisional | expired | **provisional** | mark_provisional | refresh_evidence |
| `graph_freshness` | provisional | aging | **provisional** | mark_provisional | refresh_evidence |
| `navigation_recall` | withheld | missing | **withheld** | withhold | withhold_row |

Two rows certify authoritative (`graph_topology`, `ownership_source`), proving the certifier is
not a blanket downgrade; five rows are automatically narrowed or withheld. The published label
of every row equals the gate's recomputed ceiling and never exceeds the governance claim.

## Consumer surfaces

Release evidence, docs/help, onboarding, review, AI context, and support export each bind to
this one packet, ingest it, preserve its labels and recovery paths, and narrow with it, so a
row narrowed here cannot stay green downstream by inertia. Every binding is stamped with the
active scope snapshot for replay.
