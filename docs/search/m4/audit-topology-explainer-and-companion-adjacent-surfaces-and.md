# Audit topology, explainer, and companion-adjacent surfaces — stable contract

Status: Stable lane proof for the M4 audit of every row presented on the
topology canvas, the impact explainer, and the companion-adjacent
navigator/filter/export/history surfaces.

This document is the reviewer-facing contract for the audit packet. The
packet is the single source of truth that the topology canvas, the
explainer panel, docs/help, the CLI/headless inspector, the support
export bundle, and the release proof index all read; surfaces MUST NOT
mint local copies or paraphrase status text.

## What the packet asserts

For each audited row on a governed surface, the packet asserts:

1. A **stable surface binding** (`surface_class`) picked from a closed
   vocabulary: `topology_canvas`, `topology_table`, `impact_explainer`,
   `evidence_card`, `companion_navigator`, `companion_filter`,
   `companion_export`, `companion_history`.
2. A **stable row binding** (`row_class`) picked from a closed
   vocabulary: `topology_node`, `topology_edge`, `impact_edge`,
   `evidence_card_row`, `companion_action`, `companion_filter_row`,
   `companion_export_row`, `companion_history_row`. The packet refuses
   row classes that are not permitted on the bound surface.
3. A **qualification state** (`qualified_stable`,
   `narrowed_below_stable`, `not_qualified_stable`). A row is never
   `qualified_stable` while any audit pillar is unbound.
4. Per-pillar disclosure for the four audit pillars:
   - `scope_disclosure` — one of `current_repo`, `selected_workset`,
     `full_workspace`, `remote_cache`, `outside_current_scope`, or
     `scope_unbound`.
   - `freshness_disclosure` — one of `live`, `partially_warmed`,
     `stale_disclosed`, `archived_frozen`, `imported_snapshot`, or
     `freshness_unbound`.
   - `provenance_disclosure` — one of `workspace_canonical`,
     `partial_index_inferred`, `archive_preserved`,
     `imported_provider_derived`, `heuristic_derived`, or
     `provenance_unbound`.
   - `downgrade_state_disclosure` — one of `none`,
     `narrowed_below_stable`, `blocks_stable`, `imported_snapshot`,
     `archived_frozen`, or `downgrade_state_unbound`.
5. A **confidence class** (`high_confidence`, `medium_confidence`,
   `low_confidence`). A row at `low_confidence` cannot claim
   `qualified_stable`; the validator narrows it below stable.
6. A **disclosure ref** whenever the row is not `qualified_stable`, a
   **provenance disclosure ref** whenever provenance is not
   `workspace_canonical`, and a **downgrade disclosure ref** whenever
   the downgrade state is not `none`.
7. A **raw-material exclusion** flag on every row; stable packets never
   admit raw query text, raw source bodies, secrets, or ambient
   credentials.

## Closed vocabulary

**Audit surfaces** — `topology_canvas`, `topology_table`,
`impact_explainer`, `evidence_card`, `companion_navigator`,
`companion_filter`, `companion_export`, `companion_history`.

**Audit row classes** — `topology_node`, `topology_edge`,
`impact_edge`, `evidence_card_row`, `companion_action`,
`companion_filter_row`, `companion_export_row`,
`companion_history_row`.

**Qualification states** — `qualified_stable`,
`narrowed_below_stable`, `not_qualified_stable`.

**Audit pillars** — `scope`, `freshness`, `provenance`,
`downgrade_state`. Each row carries one closed-vocabulary value per
pillar; the four `*_unbound` tokens never satisfy the pillar.

**Required consumer projections** — `topology_canvas`,
`explainer_panel`, `docs_help`, `cli_headless`, `support_export`,
`release_proof_index`. Each projection MUST preserve the same packet
id, the surface vocabulary, the row-class vocabulary, the
qualification-state vocabulary, and all four audit-pillar vocabularies;
MUST support JSON export; and MUST exclude raw private material and
ambient authority.

## Promotion states

A materialized packet is one of:

- `stable` — every covered surface has at least one audited row, every
  row satisfies all four audit pillars when claiming
  `qualified_stable`, every narrowed row carries a disclosure ref, and
  every required projection preserves the packet verbatim.
- `narrowed_below_stable` — a warning-class finding is present (for
  example, a row claims `qualified_stable` at `low_confidence` and the
  row is intentionally narrowed below stable until evidence grows).
- `blocks_stable` — a blocker finding is present (for example, a row
  claims `qualified_stable` while a pillar is unbound, a row's row
  class is not permitted on its surface, a narrowed row drops its
  disclosure ref, an imported-provider provenance drops its disclosure
  ref, a downgrade disclosure that requires an explicit ref drops it,
  a projection collapses any of the closed vocabularies, raw query
  material slips past the boundary, or the stored promotion state
  disagrees with derived findings).

## Why this matters

The track invariant for this lane is *keep search, graph, and docs
surfaces useful before fully warm and explicit about scope, freshness,
provenance, and downgrade state at all times*. The packet's validation
rules implement that invariant directly: a non-qualified row cannot
inherit the stable claim from an adjacent qualified row, the companion
-adjacent surfaces cannot mint a quieter audit posture than the
topology canvas, and an imported, heuristic, archived, or partial row
cannot masquerade as canonical workspace truth.

## Where the packet lives

- Schema: `schemas/search/audit_topology_explainer_companion_truth.schema.json`
- Reviewer artifact: `artifacts/search/m4/audit-topology-explainer-and-companion-adjacent-surfaces-and.md`
- Stable packet artifact: `artifacts/search/m4/audit_topology_explainer_companion_truth_packet.json`
- Fixture corpus: `fixtures/search/m4/audit_topology_explainer_companion_truth_packet/`
- Rust module: `crates/aureline-graph/src/audit_topology_explainer_companion_truth_packet/mod.rs`
