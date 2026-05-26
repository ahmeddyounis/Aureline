# Audit topology, explainer, and companion-adjacent surfaces — proof packet

This reviewer artifact accompanies the stable
[`audit_topology_explainer_companion_truth_packet.json`](audit_topology_explainer_companion_truth_packet.json)
and is the human-readable proof for the M4 stable lane that every row
on the topology canvas, the impact explainer, and the
companion-adjacent surfaces is audited against scope, freshness,
provenance, and downgrade-state disclosure, and that any row narrower
than stable is labeled below stable instead of inheriting an adjacent
qualified row.

## Stable claim

Every governed audit surface (`topology_canvas`, `topology_table`,
`impact_explainer`, `evidence_card`, `companion_navigator`,
`companion_filter`, `companion_export`, `companion_history`) carries at
least one audited row. Each row pins:

- a stable `surface_class` and `row_class` (the row class MUST be
  permitted on the surface),
- a `qualification_state` (`qualified_stable`,
  `narrowed_below_stable`, or `not_qualified_stable`),
- one disclosure per audit pillar
  (`scope_disclosure`, `freshness_disclosure`, `provenance_disclosure`,
  `downgrade_state_disclosure`),
- a `confidence_class`,
- a `disclosure_ref` whenever the row is not `qualified_stable`, a
  `provenance_disclosure_ref` whenever provenance is not
  `workspace_canonical`, and a `downgrade_disclosure_ref` whenever the
  downgrade state is not `none`.

## Required consumer projections

The packet is preserved verbatim across six consumer projections:

| Projection            | Surface                                                          |
| --------------------- | ---------------------------------------------------------------- |
| `topology_canvas`     | Graph topology canvas and table fallback                         |
| `explainer_panel`     | Impact explainer panel and evidence cards                        |
| `docs_help`           | Docs/help reviewer surface                                       |
| `cli_headless`        | CLI/headless inspector                                           |
| `support_export`      | Support export bundle                                            |
| `release_proof_index` | Release proof index entry                                        |

A projection that collapses any closed vocabulary, drops the packet
id, drops one of the four audit-pillar vocabularies, or leaks raw
private material immediately blocks the stable claim.

## What blocks the stable claim

The packet blocks publication when any of the following appears:

- a row claims `qualified_stable` while any audit pillar (scope,
  freshness, provenance, downgrade state) is unbound,
- a row's row class is not permitted on its surface class,
- a row is `narrowed_below_stable` or `not_qualified_stable` but drops
  its disclosure ref,
- a row's provenance disclosure is not `workspace_canonical` and drops
  its provenance disclosure ref,
- a row's downgrade-state disclosure is not `none` and drops its
  downgrade disclosure ref,
- any of the six required consumer projections is missing or collapses
  one of the closed vocabularies (surface, row class, qualification,
  or any audit pillar),
- raw query text, source bodies, secrets, or ambient credentials slip
  past the boundary,
- the stored promotion state disagrees with the derived findings.

## How to read the packet

Consumers materialize the packet through
`AuditTopologyExplainerCompanionTruthPacket::materialize` and then
read the projection that matches their surface. The packet is
metadata-only and suitable for inclusion in any support export or
release proof bundle.

## Where the packet lives

- Schema: [`schemas/search/audit_topology_explainer_companion_truth.schema.json`](../../../schemas/search/audit_topology_explainer_companion_truth.schema.json)
- Reviewer doc: [`docs/search/m4/audit-topology-explainer-and-companion-adjacent-surfaces-and.md`](../../../docs/search/m4/audit-topology-explainer-and-companion-adjacent-surfaces-and.md)
- Fixture corpus: [`fixtures/search/m4/audit_topology_explainer_companion_truth_packet/`](../../../fixtures/search/m4/audit_topology_explainer_companion_truth_packet/)
- Rust module: [`crates/aureline-graph/src/audit_topology_explainer_companion_truth_packet/mod.rs`](../../../crates/aureline-graph/src/audit_topology_explainer_companion_truth_packet/mod.rs)
