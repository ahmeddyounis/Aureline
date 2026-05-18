# Graph topology-map view, impact-explorer row, and ownership-card contract

This document is the cross-tool contract every graph-backed
*understanding* surface reads when it presents a topology map, an
impact explorer, or an ownership card. Surfaces covered: architecture-
domain map, runtime-domain map, deployment-relation map, dependency
map, ownership map, generated-artifact lineage map, provider-resource
map, imported-root map, workset-scope map, the matching list / table
/ outline alternates, and the impact-explorer panels embedded in
review packs, AI evidence panels, support bundles, and the public
query-family surface.

The machine-readable schemas live at:

- [`/schemas/graph/topology_map_view.schema.json`](../../schemas/graph/topology_map_view.schema.json)
- [`/schemas/graph/impact_reason.schema.json`](../../schemas/graph/impact_reason.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/topology_impact_cases/`](../../fixtures/graph/topology_impact_cases/)

This contract is normative. It layers on top of the workspace-graph
seed
([`/docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md))
and the provenance / confidence / source-anchor-drift contract
([`/docs/graph/provenance_and_confidence_contract.md`](provenance_and_confidence_contract.md))
without restating their identity rules. Where this document
disagrees with the seed, the seed wins on identity rules and this
document is updated in the same change. Where this document
disagrees with the provenance / confidence contract, that contract
wins on freshness, confidence, and anchor drift, and this document
is updated in the same change. Where a downstream understanding
surface mints its own topology, impact, or ownership vocabulary,
this document wins and the surface is non-conforming.

## Why freeze this now

A topology map that flattens an architecture-domain graph, a runtime
domain graph, and a deployment-relation graph into one undifferentiated
node-and-edge picture has already lied once about what its picture
means. An impact explorer that hides "this row was inferred", "this
row was hydrated from an imported bundle", "this row sits inside a
policy-limited workset and we cannot tell you what is hidden", or
"this row's anchor was deleted" has lied a second time. An ownership
card that renders blank tiles on partial evidence — instead of saying
*which* sections were unavailable and *why* — has lied a third time.

Freezing the topology-map view, the impact-explorer row, and the
ownership-card record shapes pins those three honesty contracts to
machine-readable form. The same node renders the same freshness /
confidence / anchor-drift / imported-root chips on every surface,
because every surface reads the same record family.

The second hazard is divergence between the surfaces. Topology maps,
impact panels, and ownership cards all walk the same nodes and edges
out of the workspace graph but have historically rendered three
private vocabularies for "which scope are we under", "what is
hidden", "how confident are we", and "what does the export carry".
Pinning the loaded-scope state vocabulary, the hidden-count summary,
the freshness/provenance strip, and the export-format vocabulary in
this document makes the vocabularies one.

## Scope

- Freeze one `topology_map_view_record` covering legend, filters,
  node entries, edge entries, freshness/provenance strip, hidden
  counts, export hooks, and review handoffs.
- Freeze one `ownership_card_record` covering ownership identity,
  card sections, partial-evidence disclosure, and the same export /
  review hooks.
- Freeze one `impact_explorer_view_record` and one
  `impact_explorer_row_record` covering reason class, confidence
  class (rolled-up level + floor reason + conflict state),
  anchor-drift state, imported-root status, loaded-scope note,
  hidden counts, export hooks, and review handoffs.
- Pin the closed `topology_family_class`, `presentation_mode`,
  `loaded_scope_state`, `legend_section_class`, `filter_class`,
  `ownership_card_section_class`, and `export_format_class`
  vocabularies so the same record drives map / list / table /
  outline alternates and review-pack / support-bundle / AI-evidence
  / csv / json / yaml / dot / mermaid exports without renaming
  identity.
- Reuse `graph_node_record.node_id`, `graph_edge_record.edge_id`,
  `provenance_stamp_record.stamp_id`, `confidence_rollup_record.rollup_id`,
  and `source_anchor_drift_record.drift_record_id` verbatim.

## Out of scope

- Graph layout algorithms, the canvas renderer, force-directed
  placement, edge-bundling heuristics. Those land later; this
  contract freezes only the record shapes a layout engine reads.
- Final visual styling, colour palette, and chip pixels. The
  legend names which axes the surface MUST disclose; the visual
  treatment is a UX decision separate from this schema.
- Impact computation engines (which mutation produces which
  impact-edge in the graph). The seed's `impacts` edge and the
  provenance / confidence contract pin the inputs; the engine that
  mints the edges is a separate workstream.

## 1. Topology-map view

Every topology surface emits exactly one
`topology_map_view_record`. The record answers seven questions in
one shape:

| Field                          | Question answered                                                                |
|--------------------------------|----------------------------------------------------------------------------------|
| `topology_family_class`        | Which graph family is this map summarising (ownership / architecture / runtime / deployment / dependency / generated-artifact / provider-resource / pack-mirror / imported-root / workset-scope)? |
| `presentation_mode`            | Is this the canvas, a list alternate, a table alternate, or an outline tree?      |
| `active_scope_class` + `active_scope_ref` | Which workset / scope artifact is the map drawn under?                |
| `loaded_scope_state`           | Is the loaded set fully loaded, partial, sparse, policy-limited, imported-only, warming, stale, or evidence-missing? |
| `legend_sections`              | Which honesty axes does the legend disclose?                                     |
| `filter_state`                 | Which filters has the user engaged?                                              |
| `freshness_provenance_strip`   | Per-entry freshness / extraction-or-inference mode / imported-root status / rolled-up confidence / anchor-drift state. |

Plus per-entry slots:

- `node_entries[]` — one `topology_node_entry` per node, each
  reusing `graph_node_record.node_id` verbatim and pairing it with
  the per-node visibility / freshness / provenance / confidence /
  anchor-drift state every alternate (map / list / table /
  outline) renders identically.
- `edge_entries[]` — one `topology_edge_entry` per edge, each
  reusing `graph_edge_record.edge_id` verbatim. Topology edges
  carry the seed's `topology_edge_slot.topology_kind` as a typed
  field so the same map cannot mint a parallel topology edge.

### 1.1 `topology_family_class`

Closed vocabulary. Every view names exactly one family so a map
header reads, for example, "ownership topology" or "deployment
relation topology" verbatim, never "graph view".

| Family                                  | Edges it summarises                                                                              |
|-----------------------------------------|--------------------------------------------------------------------------------------------------|
| `ownership_topology`                    | `owned_by` edges from any node to an `ownership_node`.                                           |
| `architecture_domain_topology`          | `depends_on` between packages / crates, `imports_module` between files, `contains` for the package tree. |
| `runtime_domain_topology`               | `runs_in`, `hosted_by`, `notebook_kernel_executes_cell`, `preview_runtime_renders_snapshot`.     |
| `deployment_relation_topology`          | `deployed_to`, `deploy_target_runs_service`, `provider_hosts_resource`.                          |
| `dependency_topology`                   | Pure `depends_on` walk (lockfile / package-resolver projection).                                 |
| `generated_artifact_lineage_topology`   | `generated_from`, `produces_artifact`, `consumes_artifact`, paired with the lineage record.      |
| `provider_resource_topology`            | `hosted_by` and inter-provider relations on `provider_resource_node`s.                           |
| `pack_mirrors_upstream_topology`        | `mirrors_upstream` edges projecting mirrored upstream pack lineage.                              |
| `imported_root_topology`                | `imported_root_node` visibility map.                                                             |
| `workset_scope_topology`                | `scoped_by` edges plus `policy_view_node` projections; the scope-narrowing map.                  |

A family covers exactly one disclosure axis. A surface that wants to
show "ownership and deployment together" emits two view records and
links them through the cross-reference fields rather than minting a
new family.

### 1.2 `presentation_mode`

Closed vocabulary. The same record drives every alternate:

| Mode                       | Purpose                                                                  |
|----------------------------|--------------------------------------------------------------------------|
| `map_canvas`               | The 2-D map.                                                             |
| `node_list_alternate`      | A list of node entries (no edges).                                       |
| `node_table_alternate`     | A table whose rows are node entries.                                     |
| `edge_table_alternate`     | A table whose rows are edge entries.                                     |
| `outline_tree_alternate`   | An outline tree (most useful for `contains` / `depends_on`).             |

The record never claims map_canvas while emitting an empty
`node_entries[]`; the loaded-scope state and the hidden-count
summary together explain why.

### 1.3 `loaded_scope_state`

Closed vocabulary. Every view names exactly one state so a partial
or policy-limited workset is honest:

| State                          | Meaning                                                                        |
|--------------------------------|--------------------------------------------------------------------------------|
| `fully_loaded`                 | The active scope is fully loaded and every hidden-count is `0`.                |
| `partial_workset_loaded`       | A workset narrowed the scope; `hidden_by_workset_count > 0` is admissible.     |
| `sparse_slice_loaded`          | A sparse slice limited the loaded set.                                         |
| `policy_limited_loaded`        | A policy projection hides one or more members.                                 |
| `imported_root_only_loaded`    | The only loaded contributor is an imported root.                               |
| `warming_loaded`               | The producer is warming; `not_loaded_count` reflects what is still pending.    |
| `stale_loaded`                 | The producer is stale; surfaces render the staleness badge on the strip.       |
| `evidence_missing_loaded`      | One or more entries' evidence (lineage, anchor, citation) cannot be reconstructed. |

`fully_loaded` is gated by an `allOf` rule that forces every
`hidden_count_summary` count field to `0` and every `*_known` field
to `true`. The view cannot claim completeness while disclosing
hidden rows.

`policy_limited_loaded` is gated by an `allOf` rule that forces
`hidden_by_policy_known = true` and forces `policy_overlay_legend`
into `legend_sections[]`.

`imported_root_only_loaded` is gated by an `allOf` rule that forces
`imported_root_known = true`, `imported_root_count >= 1`, and
forces `imported_root_legend` into `legend_sections[]`.

### 1.4 Legend sections

Closed vocabulary. The legend names which honesty axes are
disclosed; the surface MUST NOT drop an axis silently. Required
axes:

- `topology_family_legend` (always).
- `node_class_legend` whenever node entries render.
- `edge_class_legend` whenever edge entries render.
- `edge_evidence_state_legend` whenever any edge entry's
  `evidence_state` is anything other than `direct_evidence`.
- `freshness_legend` whenever any strip entry's `freshness_class`
  is non-`authoritative`.
- `confidence_legend` whenever any strip entry's
  `rolled_up_confidence` is non-`high`.
- `anchor_drift_legend` whenever any strip entry's
  `anchor_drift_state` is non-`anchor_present_no_drift`.
- `imported_root_legend` whenever any strip entry's
  `imported_root_status` is non-`not_imported`.
- `policy_overlay_legend` whenever `loaded_scope_state ==
  policy_limited_loaded` (enforced by `allOf`).
- `missing_anchor_legend` whenever any node entry's `node_class`
  is `missing_anchor_node`.

### 1.5 Filter state

`filter_state[]` lists which filters the user engaged. Each entry
carries `filter_class` (closed vocabulary), `engagement`
(`disengaged` / `engaged_inclusive` / `engaged_exclusive`), and an
optional `selected_values[]` token list. The list/table alternates
and the export hooks apply the same filtered set so exports cannot
quietly carry extra rows.

### 1.6 Freshness/provenance strip

`freshness_provenance_strip[]` is required (`minItems: 1`). Each
entry summarises one node, edge, or ownership card with:

- `freshness_class` and `stale_reason` (re-export of seed/ADR-0005).
- `extraction_or_inference_mode` (re-export of provenance contract).
- `imported_root_status` (re-export).
- `rolled_up_confidence` plus an opaque `confidence_rollup_ref`.
- `anchor_drift_state` plus an opaque `anchor_drift_record_ref` for
  the non-`anchor_present_no_drift` case.
- `provenance_stamp_ref` (always).

The strip is the cross-surface invariant: an ownership card, a
topology map, and an impact panel that reference the same
`subject_ref` MUST render the same chips because they read the same
strip entry.

### 1.7 Hidden-count summary

`hidden_count_summary` mirrors
`schemas/workspace/cross_repo_result_group.schema.json#hidden_count_summary`
and adds an `imported_root_count` axis (and on impact panels, an
`inferred_transitive_count` axis). The view emits counts even when
`*_known = false`; surfaces render "unknown" for that case rather
than silently showing `0`.

### 1.8 Export hooks and review handoffs

`offered_export_hooks[]` and `offered_review_handoffs[]` are both
required (`minItems: 1`).

Export formats:

| Format                              | Preserves hidden counts | Preserves freshness strip | Preserves imported-root chip |
|-------------------------------------|--------------------------|---------------------------|------------------------------|
| `review_pack_topology_packet`       | yes                      | yes                       | yes                          |
| `support_bundle_topology_packet`    | yes                      | yes                       | yes                          |
| `ai_evidence_topology_packet`       | yes                      | yes                       | yes                          |
| `csv_node_table` / `csv_edge_table` | no                       | no                        | optional                     |
| `json_topology_snapshot`            | yes                      | yes                       | yes                          |
| `yaml_topology_snapshot`            | yes                      | yes                       | yes                          |
| `graphviz_dot` / `mermaid_flowchart`| no                       | no                        | no                           |

Each export-hook entry declares `preserves_hidden_counts` and
`preserves_freshness_strip` explicitly. A surface that claims an
export carries the strip while actually dropping it is non-
conforming.

Review handoffs cover review-pack, support-bundle, AI-evidence, and
admin-only policy-review jumps plus copy-id actions.

## 2. Ownership card

Every ownership-card surface emits one `ownership_card_record`.
The card reuses the seed's `ownership_node` identity verbatim
(`ownership_node_id_ref`) and projects the same freshness/provenance
strip every other surface reads. Partial-evidence cards disclose
which sections were unavailable instead of silently rendering blank
tiles.

### 2.1 Card sections

Closed vocabulary. `sections_present[]` lists the sections the
card actually rendered:

| Section                          | Carries                                                                  |
|----------------------------------|--------------------------------------------------------------------------|
| `ownership_identity`             | `ownership_kind`, `display_label`, `codeowners_rule_ref`.                |
| `freshness_provenance_strip`     | One or more strip entries (always).                                      |
| `covered_node_summary`           | `covered_node_count` plus optional `covered_node_id_refs[]`.             |
| `missing_evidence_summary`       | `missing_evidence_count` and the strip entries that made it non-zero.    |
| `active_scope_summary`           | `active_scope_class`, `active_scope_ref`, `loaded_scope_state`.          |
| `supporting_artifact_refs`       | The card's supporting-artifact list (CODEOWNERS rules, mutation journal entries, support packets). |
| `offered_actions`                | The export and review handoff entries.                                   |

`freshness_provenance_strip` MUST be present (`minItems: 1`) so a
partial-evidence card still discloses what it knew about the
ownership identity itself.

### 2.2 Disclosing partial evidence

When the active scope, the workset, or the policy projection
narrows the card's coverage, the card sets `loaded_scope_state` to
the matching value (`partial_workset_loaded`,
`sparse_slice_loaded`, `policy_limited_loaded`,
`imported_root_only_loaded`, `warming_loaded`, `stale_loaded`,
`evidence_missing_loaded`) and emits non-zero counts on the
`hidden_count_summary`. The card never falls back to a generic
"unknown" or blank tile.

### 2.3 Card export and review handoffs

`offered_review_handoffs[]` and `offered_export_hooks[]` reuse the
topology-view vocabulary so the same handoff jumps from a card to
a review pack, a support bundle, an AI evidence packet, an impact
explorer, or a cited-explainer overlay without re-deriving the
identity.

### 2.4 Linking cards to topology maps

A `topology_map_view_record` whose `topology_family_class ==
ownership_topology` MUST list at least one `ownership_card_refs[]`
entry (gated by `allOf`). The card and the map share ownership
identity through that ref; the surface never lets them disagree.

## 3. Impact explorer

Every impact-explorer surface emits one
`impact_explorer_view_record`. Each row is one
`impact_explorer_row_record`. The contract reuses every honesty axis
the topology view uses and adds the impact-only axes the spec
requires.

### 3.1 Per-row identity

Each row carries:

- `impacted_node_id_ref` — the seed's `graph_node_record.node_id`.
- `impacts_edge_id_ref` — the seed's `graph_edge_record.edge_id`
  whose `edge_class == impacts`.
- Optional `impact_source_node_id_ref` — the impacts edge's
  `from_node_id`.

Impact panels never mint a private impacted-object identifier.

### 3.2 `reason_class`

Re-export of
`schemas/graph/workspace_graph_seed.schema.json#impact_reason_slot.reason_class`:
`direct_edit`, `symbol_rename`, `signature_change`,
`dependency_bump`, `generated_artifact_regeneration`,
`policy_change`, `ownership_change`, `provider_resource_update`,
`imported_bundle_rollover`, `workset_scope_narrowed`,
`workset_scope_widened`, `inferred_transitive_impact`,
`exact_edge`, `shared_target`, `ownership_rule`,
`generated_linkage`, `heuristic_similarity`, `policy_coupling`.
The last six tokens are the shared UI, CLI/headless, export, and
support-packet vocabulary required for impact rows. Impact panels
never invent a reason class.

### 3.3 Confidence class

Each row carries `rolled_up_confidence` (re-export of
`confidence_level`), `floor_reason` (re-export of
`floor_reason`), and `conflict_state` (re-export of
`conflict_state`). The trio is the per-row "confidence class"
the spec names: a single rolled-up level plus the typed reason
the rollup landed there plus the typed conflict (if any). The
optional `confidence_rollup_ref` points at the
`confidence_rollup_record` that produced the trio so a reviewer can
replay the rollup deterministically.

`reason_class == inferred_transitive_impact` is gated by an `allOf`
rule that caps `rolled_up_confidence` at `medium` and forces
`floor_reason` into the closed set of floors that are admissible
for an inferred row (see schema §allOf).

### 3.4 Anchor drift and imported-root status

Per row, the same axes the topology view carries:

- `anchor_drift_state` (re-export of `source_anchor_drift_state`).
  An `allOf` gate forces `anchor_drift_record_ref` non-null when
  the state is non-`anchor_present_no_drift` so the drift event
  remains auditable.
- `imported_root_status` (re-export). An `allOf` gate forces at
  least one offered export hook to set
  `preserves_imported_root_chip = true` when the row is imported,
  so an imported row cannot silently fold into a local row on
  export.

### 3.5 Loaded-scope note

Required on every row (`loaded_scope_note`). Carries
`loaded_scope_state`, `active_scope_class`, and `active_scope_ref`.
The note rides every export and every review handoff so the
reviewer always sees which scope the impact was computed under.

### 3.6 Hidden-result counts

`hidden_count_summary` per view (and per row, for the standalone
row payload). Mirrors the topology hidden-count summary and adds
`inferred_transitive_known` / `inferred_transitive_count` so the
panel header can render "N% of impact rows are inferred-
transitive". The exact hidden-row ids never project.

### 3.7 Export hooks and review handoffs

The impact-explorer export formats are:

| Format                              | Preserves hidden counts | Preserves confidence chip |
|-------------------------------------|--------------------------|---------------------------|
| `review_pack_impact_packet`         | yes                      | yes                       |
| `support_bundle_impact_packet`      | yes                      | yes                       |
| `ai_evidence_impact_packet`         | yes                      | yes                       |
| `csv_impact_table`                  | no                       | no                        |
| `json_impact_snapshot`              | yes                      | yes                       |
| `yaml_impact_snapshot`              | yes                      | yes                       |

Review handoffs reuse the topology-view vocabulary so the same row
jumps to a review pack, support bundle, AI evidence packet, the
parent topology map, or a cited-explainer overlay without re-
deriving identity.

### 3.8 Linking impact views to topology views

`impact_explorer_view_record.topology_view_ref` is the opaque ref to
a `topology_map_view_record`. Topology and impact panels share node
and edge identity through this ref; the surfaces never disagree on
which `node_id` they are talking about.

## 4. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about topology maps, ownership cards, or impact rows.

1. **No surface invents private identity.** Every node entry,
   edge entry, ownership card, and impact row reuses the seed's
   `node_id` / `edge_id` / `ownership_node_id` verbatim. Local-
   only handles ("topology row 5", "impact #3") are forbidden when
   a graph record carries one of the values defined here.
2. **No surface invents private confidence labels.** Surfaces
   render `rolled_up_confidence` verbatim with the
   `floor_reason` and `conflict_state` chips. Local-only labels
   like "verified", "fresh", "trusted", "best guess" are
   forbidden.
3. **Loaded-scope state is honest.** A view that narrowed the
   scope MUST NOT carry `loaded_scope_state == fully_loaded`.
   Schema `allOf` rules enforce the count consistency.
4. **Hidden counts cross the boundary; ids never do.** The exact
   hidden ids project only through the policy-admin surface and
   only when the underlying workset's
   `policy_limitation.hidden_member_list_visible == true`.
5. **Imported entries carry the imported-root chip.** An entry
   with `imported_root_status != not_imported` renders the chip
   verbatim and the export hook preserves it.
6. **Anchor drift remains typed.** An entry with
   `anchor_drift_state != anchor_present_no_drift` renders the
   typed drift chip and (for impact rows) carries the
   `anchor_drift_record_ref` so the drift event is auditable.
7. **Topology family stays distinguishable.** A surface that
   wants to show two families emits two view records; it does
   not flatten them into one.
8. **Ownership cards disclose partial evidence.** A card never
   falls back to a generic "unknown" or blank tile; it sets
   `loaded_scope_state` and emits non-zero hidden counts.
9. **Export hooks declare what they carry.** Each
   `export_hook_entry` sets `preserves_hidden_counts`,
   `preserves_freshness_strip` (topology), and
   `preserves_confidence_chip` (impact) explicitly. A claim of
   completeness in the export receipt that does not match the
   format is non-conforming.
10. **Review handoffs reuse one vocabulary.** Topology, impact,
    and ownership surfaces all draw from the same review-handoff
    closed vocabulary.

## 5. Worked examples

Each example references a companion fixture under
[`/fixtures/graph/topology_impact_cases/`](../../fixtures/graph/topology_impact_cases/).
The fixtures are human-authored YAML and validate against the
schemas under JSON Schema Draft 2020-12.

### 5.1 Full-workset topology

A fully loaded architecture-domain map across one workspace root
covering three crates and their `depends_on` edges. View:
`topology_family_class = architecture_domain_topology`,
`presentation_mode = map_canvas`, `loaded_scope_state =
fully_loaded`. Every hidden-count is zero. Legend sections cover
topology family, node class, edge class, and freshness; the
freshness/provenance strip carries one entry per node at
`freshness_class = authoritative` and `rolled_up_confidence =
high`.

See
[`full_workset_architecture_topology.yaml`](../../fixtures/graph/topology_impact_cases/full_workset_architecture_topology.yaml).

### 5.2 Sparse-workset topology with hidden nodes

A sparse-slice runtime-domain map under a named workset that
narrows to one service. The workset hides eight nodes and a policy
projection hides two more. View: `topology_family_class =
runtime_domain_topology`, `loaded_scope_state =
policy_limited_loaded`, `hidden_by_workset_count = 8`,
`hidden_by_policy_count = 2`, `policy_overlay_legend` and
`policy_overlay_filter` engaged. The view never claims completeness.

See
[`sparse_workset_runtime_topology_with_hidden_nodes.yaml`](../../fixtures/graph/topology_impact_cases/sparse_workset_runtime_topology_with_hidden_nodes.yaml).

### 5.3 Imported-root impact result

An impact panel whose only contributor is an imported signed
upstream bundle. One row at `reason_class = imported_bundle_rollover`,
`rolled_up_confidence = medium`, `floor_reason =
imported_unverified_caps_at_medium`, `imported_root_status =
imported_signed_upstream_bundle`, `anchor_drift_state =
anchor_imported_unverified`. View:
`loaded_scope_state = imported_root_only_loaded`,
`hidden_count_summary.imported_root_count = 1`. The export hook
declares `preserves_imported_root_chip = true`.

See
[`imported_root_impact_result.yaml`](../../fixtures/graph/topology_impact_cases/imported_root_impact_result.yaml).

### 5.4 Partial-evidence ownership card

An ownership card for a CODEOWNERS-resolved team whose CODEOWNERS
rule is stale and three covered nodes' anchors were deleted in a
recent refactor. Card: `loaded_scope_state =
evidence_missing_loaded`, `sections_present` includes
`missing_evidence_summary`, `missing_evidence_count = 3`, the
strip entry for the ownership identity carries
`freshness_class = stale` with `stale_reason = upstream_input_stale`.
The card never collapses into a blank tile; the missing-evidence
summary names the count.

See
[`partial_evidence_ownership_card.yaml`](../../fixtures/graph/topology_impact_cases/partial_evidence_ownership_card.yaml).

## 6. Acceptance

- Topology maps and impact explorers can present partial or
  policy-limited scope honestly. The `loaded_scope_state` and
  `hidden_count_summary` axes carry the truth; the schema's
  `allOf` rules enforce the consistency.
- Ownership cards and impact reasons reuse the seed's identity
  layer plus the provenance / confidence / anchor-drift contract
  rather than minting bespoke UI-only text blobs. Every chip
  resolves to a `provenance_stamp_record`,
  `confidence_rollup_record`, or `source_anchor_drift_record`
  defined upstream.
- The `presentation_mode` axis covers map canvas plus list /
  table / outline alternates, and `export_format_class` covers
  review-pack, support-bundle, AI-evidence, csv, json, yaml,
  graphviz-dot, and mermaid-flowchart exports without redefining
  topology identity.
- The fixtures cover full-workset architecture topology, sparse-
  workset runtime topology with hidden nodes, imported-root
  impact result, and partial-evidence ownership card, and
  validate against the schemas.

## 7. Changing this vocabulary

- **Additive-minor.** Adding a new `topology_family_class`, new
  `presentation_mode`, new `legend_section_class`, new
  `filter_class`, new `ownership_card_section_class`, new
  `export_format_class`, new `review_handoff_class`, new
  `loaded_scope_state`, or new hidden-count axis lands here, in
  the schemas, and in the fixtures in the same change. The change
  must cite the motivating fixture.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The workspace-graph seed wins on identity rules; the provenance
  / confidence contract wins on freshness, confidence, and anchor
  drift. This document and its schemas are updated in the same
  change when those documents change.

## Linked artifacts

- Workspace-graph seed:
  [`docs/graph/workspace_graph_seed.md`](workspace_graph_seed.md),
  [`schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json).
- Provenance / confidence / source-anchor-drift contract:
  [`docs/graph/provenance_and_confidence_contract.md`](provenance_and_confidence_contract.md),
  [`schemas/graph/provenance_stamp.schema.json`](../../schemas/graph/provenance_stamp.schema.json),
  [`schemas/graph/confidence_rollup.schema.json`](../../schemas/graph/confidence_rollup.schema.json).
- Topology-map view schema:
  [`schemas/graph/topology_map_view.schema.json`](../../schemas/graph/topology_map_view.schema.json).
- Impact-explorer row schema:
  [`schemas/graph/impact_reason.schema.json`](../../schemas/graph/impact_reason.schema.json).
- Topology / impact case fixtures:
  [`fixtures/graph/topology_impact_cases/`](../../fixtures/graph/topology_impact_cases/).
- Cross-repo result-group contract (hidden-count vocabulary):
  [`schemas/workspace/cross_repo_result_group.schema.json`](../../schemas/workspace/cross_repo_result_group.schema.json).
- Workset / scope contract:
  [`schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json).
- Generated-artifact lineage schema:
  [`schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json).
- Execution-context schema:
  [`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json).
