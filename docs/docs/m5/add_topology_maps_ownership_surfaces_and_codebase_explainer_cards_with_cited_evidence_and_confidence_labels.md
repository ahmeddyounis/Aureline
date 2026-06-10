# Topology Maps, Ownership Surfaces, and Codebase Explainer Cards

This document is the contract for the M5 codebase-understanding feature. The
workspace graph and ownership sources are projected into three kinds of cited,
confidence-labelled **cards** the docs and code-understanding surfaces render:

- a **topology map** card — a region of the dependency / containment topology;
- an **ownership surface** card — who owns a region, and on what basis;
- a **codebase explainer** card — a natural-language explanation of a region or
  symbol.

Each card carries the same source/version/freshness/locality/confidence chip set
the docs-recall lanes use, an explicit confidence reason, a non-empty list of
cited evidence, inline provenance, and the open-raw / open-source escapes. An
evidence export preserves the source/confidence/citation truth that support, AI
evidence, and review surfaces ingest rather than cloning status text. The
codebase explorer, docs browser, graph panel, AI context, retrieval inspector,
CLI/headless output, support exports, and Help/About all consume the checked-in
packet.

- Record kind: `topology_ownership_and_codebase_explainer_cards`
- Schema: [`schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json`](../../../schemas/docs/add-topology-maps-ownership-surfaces-and-codebase-explainer-cards-with-cited-evidence-and-confidence-labels.schema.json)
- Canonical support export: [`artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/support_export.json`](../../../artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/support_export.json)
- Summary artifact: [`artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md`](../../../artifacts/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels.md)
- Fixtures: [`fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/`](../../../fixtures/docs/m5/add_topology_maps_ownership_surfaces_and_codebase_explainer_cards_with_cited_evidence_and_confidence_labels/)
- Producer: `aureline_docs::current_stable_codebase_understanding_cards_export`
- Emitter: `cargo run -p aureline-docs --bin aureline_docs_codebase_understanding_cards`

## The cards and their chips

`cards` is the set of understanding cards for one region scope. Every card points
at a `subject_ref`, carries a `card_kind` (`topology_map`, `ownership_surface`,
`codebase_explainer`), a `title`, a `headline`, and a `chips` block — the five
chips a consumer projects verbatim:

| Chip | Tokens |
| --- | --- |
| `source_class` | `workspace_code`, `dependency_source`, `graph_index`, `codeowners_file`, `ownership_registry`, `project_docs`, `generated_reference`, `mirrored_official_docs` |
| `version_match` | `exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build` |
| `freshness` | `authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`, `refresh_pending` |
| `locality` | `local`, `mirrored_pack`, `remote_helper`, `managed` |
| `confidence` | `high`, `medium`, `low`, `heuristic` |

Every packet must include at least one card of each of the three kinds — a
partial set (topology without ownership, say) is `required_card_kind_missing` and
blocks promotion, so the lane stays the full topology + ownership + explainer
surface rather than a slice that overstates coverage.

Each card carries an explicit `confidence_reason` that says *why* it earned its
confidence label; a card with no reason blocks promotion, so a confidence chip
can never be presented unexplained.

### Topology cards

A `topology_map` card carries `topology_edges`, each a
`{from_ref, to_ref, edge_kind, note}` entry (`edge_kind`: `depends_on`,
`contains`, `implements`, `calls`, `references`). A topology card with no edges is
`topology_card_missing_edges`; an edge missing an endpoint is
`topology_edge_endpoint_missing`. Both block promotion.

### Ownership cards

An `ownership_surface` card carries `owners`, each a
`{owner_ref, ownership_basis, coverage_note}` entry. `ownership_basis` is one of
`codeowners_entry`, `declared_registry` (authoritative declarations),
`directory_convention`, `git_history_heuristic`, or `unassigned`. An ownership
card with no owner is `ownership_card_missing_owner`. A card that claims `high`
confidence while any owner rests on a heuristic or unassigned basis is
`ownership_basis_unattributed` — a guess never presents as an authoritative
owner.

## Cited evidence and citation

Each card carries a non-empty `evidence` list — `{evidence_id, subject_kind,
subject_ref, derivation, cited, citation_ref?, open_raw_escape_ref,
open_source_escape_ref, note}`. `subject_kind` is one of `code_symbol`,
`code_file`, `code_module`, `crate_node`, `dependency_edge`, `owner_entry`,
`docs_node`. A card with no evidence is `card_evidence_missing`.

A derived or inferred card (its `provenance.derivation` is `derived_summary` or
`inferred_explanation`) must stay cited (`card_not_cited` otherwise), and a
derived or inferred *evidence* item must be cited too (`evidence_not_cited`). An
inferred card may not be presented as `high` confidence
(`inferred_card_looks_authoritative`). These keep topology, ownership, and
explainer cards honest: a heuristic or generated explanation never wears a
verified-truth label.

The `evidence_export` is the cited projection support and AI evidence surfaces
ingest. It declares `scope` and the `preserves_*` invariants and carries one row
per card mirroring `card_kind`, `source_class`, `confidence`, `derivation`,
`cited`, `evidence_count`, and the escapes. An export row whose card kind, source
class, or confidence disagrees with the card, a card without an export row, or an
export that drops a preservation flag all block promotion — the export can never
quietly upgrade a card.

## Promotion and downgrade

`promotion_state` is computed from the worst severity across the validation
findings and the attached `understanding_degradations`:

- a `blocking` finding → `blocks_stable`;
- otherwise a `narrowing` finding → `narrowed_below_stable`;
- otherwise → `stable`.

Degradations (`graph_index_stale`, `ownership_unresolved`,
`mirror_offline_snapshot`, `partial_topology`,
`embedder_unavailable_lexical_fallback`, `quarantined_pack`, `broken_anchor`)
carry their own severity, so a degraded but honest card set narrows rather than
hides. The fixtures show a stale-graph card set narrowing
(`narrowed_below_stable`, no blocking findings) and two blocked cases
(`uncited_topology_card`, `inferred_explainer_over_authoritative`).
`current_stable_codebase_understanding_cards_export` re-materializes the
checked-in packet and fails if the recorded promotion state drifts from the
freshly computed one, so a stale or under-attributed card set cannot be promoted
silently.

## Boundary

Raw source files, raw document bodies, raw provider payloads, and credentials
never cross this boundary. The packet carries only metadata, chip truth,
confidence reasons, cited evidence refs, provenance, finding summaries, and
contract references; `region_digest_ref` and the evidence refs are opaque,
never raw bodies.

## Out of scope

This feature does not broaden general web-mode or browser-runtime claims beyond
the narrow docs/review/light-edit surfaces qualified in M5. Browser handoff and
scoped browser-surface qualification stay in their own contracts.
