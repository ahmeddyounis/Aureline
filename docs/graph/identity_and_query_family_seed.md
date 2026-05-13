# Semantic-workspace-graph identity, freshness/confidence label,
# and public query-family seed

This document seeds the **public boundary** of the semantic-workspace
graph. Where [`workspace_graph_seed.md`](./workspace_graph_seed.md)
froze the internal object model (every node, every edge, every
cross-cutting slot), this document freezes the **published surface**
three other workstreams read without peeking inside the graph crate:

- **Node identity** — one published shape for every graph node,
  with durable-versus-derived classification and a
  freshness/confidence compatibility class.
- **Edge family** — one published shape for every graph edge,
  grouped into named families and carrying the same durable-versus-
  derived classification.
- **Public query family** — one published vocabulary of safe
  inspect / read / query groups and a versioned result-envelope
  shape every consumer reuses so no surface leaks an ad hoc
  internal query shape into a future public contract.

The machine-readable schemas live at:

- [`/schemas/graph/node_identity.schema.json`](../../schemas/graph/node_identity.schema.json)
- [`/schemas/graph/edge_family.schema.json`](../../schemas/graph/edge_family.schema.json)
- [`/schemas/graph/query_family.schema.json`](../../schemas/graph/query_family.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/query_family_cases/`](../../fixtures/graph/query_family_cases)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or a superseding ADR, those documents win and this
document MUST be updated in the same change. Where this document
disagrees with a downstream search, docs, AI, public-graph, review,
or support surface's mint of its own fields, this document wins and
the surface is non-conforming.

## Why publish this now

The internal graph seed already carries every field a future surface
needs. What it did not yet carry is the **publish-level** contract:
the promise that a node identity, an edge family, and a query-family
result envelope each have one stable shape and one stable vocabulary
every outside consumer reads. Without that, three hazards wait:

- **Forked identity.** Search, docs, AI, review, support, and the
  public graph query surface each ship a private "entity" record,
  five of which drift.
- **Silent derivation.** A surface promotes an AI-inferred or
  policy-projected row to durable authority because no published
  rule says "this row is derived — render the chip."
- **Envelope creep.** An internal query-result shape leaks out of
  one surface's telemetry, is reused by another, and becomes a de
  facto public API — at which point breaking it breaks every
  consumer that started reading it.

Freezing a published identity-durability class, a published edge-
durability class, a published freshness-confidence compatibility
class, a published query-family vocabulary, and a versioned result
envelope — in machine-readable form — heads off all three.

## Scope

- Publish **one** node-identity shape (`node_identity_record`) with
  stable id, frozen class vocabulary, durable-versus-derived
  classification, and the freshness-confidence compatibility class.
- Publish **one** edge-family shape (`edge_family_record`) with
  stable id, frozen class vocabulary, published edge-family group,
  edge-durability class, and the same compatibility class.
- Publish **one** query-family descriptor shape and **one** versioned
  result-envelope shape so every consumer reads the same result-row
  anchor and the same partial-truth disclosure contract.
- Seed a fixture per representative query family covering the
  durable, derived, stale, warming, missing-anchor, policy-hidden,
  and partial-scope cases.

## Out of scope

- Any full graph engine, index materialization plan, or search-
  ranking implementation. Those land with later search-planner and
  public graph query work; this seed freezes only the record shape
  and vocabulary each consumer will read.
- The RPC transport / wire encoding. These schemas are the cross-
  tool boundary; the transport quotes these records rather than re-
  inventing them.
- The AI-context-assembly algorithm. The query family named
  `ai_context_assembly` is a declared safe group with rules; the
  assembly algorithm itself is a separate workstream.

## 1. Published node identity

Every published node-identity row carries:

| Field                                         | Role                                                                                                                  |
|-----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| `node_id`                                     | Stable, opaque id — quoted verbatim by every consumer citation.                                                       |
| `node_class`                                  | Frozen vocabulary (file, directory, symbol, doc, ownership, topology, provider_resource, generated_artifact, …).      |
| `node_body`                                   | Class-specific body (re-exported from the internal seed).                                                             |
| `provenance_stamp`                            | Which producer family emitted the row, and how.                                                                       |
| `freshness_frame`                             | Current freshness plus typed `stale_reason` when non-authoritative.                                                   |
| `confidence_level`                            | Producer-assigned confidence.                                                                                         |
| `identity_durability_class`                   | `durable_identity` \| `derived_identity` \| `synthetic_identity`.                                                     |
| `freshness_confidence_compatibility_class`    | Named row-render posture consumers read to pick a chip.                                                               |
| `scope_refs`                                  | Workset / scope visibility (fully, partial, policy-hidden, missing-in-scope).                                         |
| `source_anchors`                              | Authoritative anchors (required for durable identity; forbidden for synthetic).                                       |

### 1.1 Durable versus derived identity

Published `source_class` is split into two subsets:

- **Durable source class** — `workspace_filesystem`, `buffer_editor`,
  `symbol_resolver`, `docs_pack`, `codeowners_resolver`,
  `build_toolchain`, `codegen_tool`, `package_resolver`,
  `notebook_kernel`, `preview_runtime`, `connected_provider`,
  `remote_agent`, `imported_bundle`, `replay_capture`.
- **Derived source class** — `ai_inference`, `policy_projection`,
  `manual_annotation`.

A node whose `provenance_stamp.source_class` sits in the durable set
is a `durable_identity` row and MUST carry at least one
`source_anchor` whose `anchor_kind` is in the authoritative anchor
set: `filesystem_identity`, `symbol_definition_site`,
`docs_pack_entry`, `mutation_journal_entry`,
`generated_artifact_lineage`, `provider_resource_handle`,
`imported_bundle_entry`, `replay_capture_entry`, `codeowners_rule`.
A node in the derived set is a `derived_identity` row and renders
the derived chip regardless of `confidence_level`.
`missing_anchor_node`, `policy_view_node`, and `workset_scope_node`
are `synthetic_identity` rows — they stand in for honest absence or
scope projection, and they MUST NOT be rendered as either durable or
derived without the typed body badge.

### 1.2 Freshness-confidence compatibility class

The published compatibility class names a **row-render posture**:

| Class                           | Pairing                                                                              |
|---------------------------------|--------------------------------------------------------------------------------------|
| `live_authoritative`            | freshness = authoritative, confidence = high, durable identity.                      |
| `live_medium_confidence`        | freshness = authoritative, durable identity, confidence = medium.                    |
| `live_low_confidence`           | freshness = authoritative, durable identity, confidence = low.                       |
| `warming_partial`               | freshness = warming; renders the warming chip and partial-truth disclosure.          |
| `cached_authoritative_snapshot` | freshness = cached with a named cache_key_ref.                                       |
| `stale_last_known_good`         | freshness = stale with a typed stale_reason.                                         |
| `replayed_bundle_snapshot`      | freshness = replayed with a replay_capture_ref.                                      |
| `imported_bundle_snapshot`      | freshness = imported with an imported_bundle_ref.                                    |
| `derived_inference`             | derived identity; renders the derived chip regardless of confidence.                 |
| `synthetic_honesty_row`         | synthetic identity; renders the missing-anchor, policy-hidden, or scope badge.       |

The compatibility class is the **contract** every consumer — search
row renderer, graph overlay, AI context assembler, review pack,
support bundle — reads when it decides which chip to render.
Consumers MUST NOT fold one class into another.

## 2. Published edge family

Every published edge row carries the same id / class / provenance /
freshness / confidence / scope / source-anchor skeleton plus:

| Field                                         | Role                                                                                                                  |
|-----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|
| `edge_family_group`                           | Published group the edge_class belongs to (see §2.1).                                                                  |
| `evidence_state`                              | Re-exported: `direct_evidence`, `imported_evidence`, `inferred_relation`, `stale_relation`, `missing_anchor`.         |
| `edge_durability_class`                       | `durable_edge` \| `derived_edge` \| `stale_edge` \| `missing_anchor_edge`.                                            |
| `topology_edge_slot`                          | Required for `deployed_to`, `runs_in`, `hosted_by`, `produces_artifact`, `consumes_artifact`, `mirrors_upstream`, `depends_on`. |
| `freshness_confidence_compatibility_class`    | Same vocabulary as node identity, so chips stay consistent across nodes and edges.                                     |

### 2.1 Edge-family groups

Every `edge_class` maps to exactly one family group:

| Group                                   | Edge classes                                                  |
|-----------------------------------------|---------------------------------------------------------------|
| `containment_family`                    | `contains`                                                    |
| `symbol_graph_family`                   | `defines_symbol`, `references_symbol`                         |
| `module_graph_family`                   | `imports_module`                                              |
| `dependency_topology_family`            | `depends_on`                                                  |
| `ownership_family`                      | `owned_by`                                                    |
| `documentation_and_citation_family`     | `documented_by`, `cites`                                      |
| `generated_artifact_lineage_family`     | `generated_from`                                              |
| `imported_mirror_family`                | `mirrors_upstream`                                            |
| `topology_runtime_family`               | `deployed_to`, `runs_in`, `hosted_by`, `produces_artifact`, `consumes_artifact` |
| `impact_family`                         | `impacts`                                                     |
| `explanation_family`                    | `explains`                                                    |
| `scoping_family`                        | `scoped_by`                                                   |
| `aliasing_family`                       | `aliases`                                                     |
| `missing_anchor_family`                 | `missing_anchor_for`                                          |

Consumers read the group when deciding whether a query family may
render the edge at all (§3.2).

### 2.2 Edge-durability class

- `durable_edge` — `evidence_state` in `{direct_evidence,
  imported_evidence}` and the provenance source is in the durable
  source class set. Requires at least one authoritative anchor.
- `derived_edge` — `evidence_state = inferred_relation` or the
  provenance source is in the derived source class set. Renders the
  derived chip.
- `stale_edge` — `evidence_state = stale_relation`. Renders the
  staleness chip and carries a typed `stale_reason`.
- `missing_anchor_edge` — `evidence_state = missing_anchor`. One
  endpoint points at a `missing_anchor_node`; the missing-anchor
  badge renders instead of hiding the row.

## 3. Published query-family seed

### 3.1 Query-family vocabulary

The published query-family id list is exactly the internal
`query_family_tag` vocabulary:

`lexical_text_search`, `symbol_jump`, `semantic_code_search`,
`docs_search`, `ownership_lookup`, `topology_walk`,
`impact_explorer`, `dependency_walk`,
`generated_artifact_lineage_walk`, `provider_resource_lookup`,
`cited_explainer_walk`, `ai_context_assembly`, `public_graph_query`,
`review_impact_walk`, `support_export_walk`.

A node or edge lists one or more families it is intended to serve
through its `query_family_tags` slot; the query-family descriptor
then declares what a given family may render.

### 3.2 Query-safety classes

Each family is a member of one or more published safety classes:

| Safety class              | Meaning                                                                                             |
|---------------------------|-----------------------------------------------------------------------------------------------------|
| `inspect_read_only`       | Never mutates, never materializes derived rows.                                                     |
| `read_durable_only`       | May render only durable identity rows and durable edges.                                            |
| `read_derived_permitted`  | May render derived rows but MUST mark them.                                                         |
| `walk_scope_bounded`      | Stops at `workset_scope` boundaries; projects policy-hidden members through `policy_view_node`.     |
| `ai_context_restricted`   | Assembles AI context; obeys redaction and uses `explainer_citation_slot` rather than raw bodies.     |
| `support_export_bundle`   | Produces support-export-safe envelopes (no raw credentials, no raw URLs, no raw source bytes).       |
| `public_api_safe`         | Publishable on the public graph query API; envelope version is locked.                               |

### 3.3 Query intent classes

Intent classes name what a query is trying to do:

`lookup_by_identity`, `lookup_by_label`, `walk_neighbors`,
`walk_typed_edges`, `collect_impact_set`, `collect_context_set`,
`enumerate_scope_members`, `enumerate_citations`,
`export_support_bundle_slice`.

### 3.4 Result-envelope shape

A `result_envelope_record` carries:

- `result_envelope_version` (currently 1).
- `envelope_id` — stable, opaque.
- `query_invocation_id` — so readers can join back to the issuing
  invocation.
- `query_family_id` and `query_intent_class`.
- `readiness_state` — re-exported from the search-result-truth
  vocabulary so graph queries and search share one readiness axis.
- `result_truth_class` — `exact` / `imported` / `heuristic` /
  `hybrid`, same search-result-truth vocabulary.
- `envelope_scope_refs` — the scope(s) the envelope is bounded by.
- `rows` — ordered `result_row`s. The public row anchor is
  `(envelope_id, row_index)`; no internal row-pointer leaks out.
- `partial_truth_disclosure` — required when `readiness_state` is
  not `fully_indexed` or `result_truth_class` is `heuristic` /
  `hybrid`.
- `envelope_provenance`, `envelope_freshness`, `envelope_confidence`.

Each `result_row` carries a `row_class` (`node_identity_row`,
`edge_family_row`, `missing_anchor_row`, `policy_hidden_row`,
`scope_partial_row`, `warming_row`, `stale_row`) and an id ref
(`node_identity_ref` or `edge_family_ref`) the consumer joins
against the node-identity / edge-family records. Rows never carry
raw bodies.

### 3.5 Constraint ids

Consumers cite these verbatim when flagging a contract violation:

- `envelope_version_pinned`
- `row_anchor_stable`
- `rows_within_allowed_node_classes`
- `rows_within_allowed_edge_family_groups`
- `readiness_state_allowed_for_family`
- `result_truth_class_allowed_for_family`
- `partial_truth_disclosure_required_when_not_fully_indexed`
- `partial_truth_disclosure_required_when_heuristic_or_hybrid`
- `ai_context_family_uses_citations`
- `public_api_family_locks_envelope_version`
- `support_export_family_preserves_redaction`
- `walk_scope_bounded_stops_at_scope_boundary`
- `derived_rows_render_derived_chip`
- `missing_anchor_row_points_at_missing_anchor_node`
- `no_raw_body_crosses_boundary`

## 4. Surface rules

The same surface-rule contract from `workspace_graph_seed.md`
applies, with three additions:

1. **Consumers read the published identity and family records**, not
   the internal `graph_node_record` / `graph_edge_record`. The
   internal records remain the schema of record for the graph
   crate; the public boundary is `node_identity.schema.json` and
   `edge_family.schema.json`.
2. **Consumers never unpack an envelope into a private row shape.**
   A surface that needs a private row must derive it from
   `(envelope_id, row_index)` without losing the published
   compatibility class, durability class, or disclosure.
3. **A heuristic, stale, warming, imported, replayed, or policy-
   hidden row always renders its chip.** A consumer that hides the
   chip is non-conforming.

## 5. Worked examples

Each example references a companion fixture under
[`/fixtures/graph/query_family_cases/`](../../fixtures/graph/query_family_cases).
The fixtures are hand-authored result envelopes pinning one family
and one representative scenario each.

### 5.1 `symbol_jump_exact_case`

A `symbol_jump` invocation resolves a qualified symbol path to a
durable `symbol_node`. Envelope `readiness_state = fully_indexed`,
`result_truth_class = exact`; one row of class `node_identity_row`
with `identity_durability_class = durable_identity` and
`freshness_confidence_compatibility_class = live_authoritative`. No
partial-truth disclosure.

See [`symbol_jump_exact_case.json`](../../fixtures/graph/query_family_cases/symbol_jump_exact_case.json).

### 5.2 `docs_search_hybrid_case`

A `docs_search` invocation returns a mix of durable docs-pack rows
and imported docs-pack rows; the envelope carries
`result_truth_class = hybrid` and a `hybrid_results_disclosed`
disclosure. Rows mix `live_authoritative` and
`imported_bundle_snapshot` postures.

See [`docs_search_hybrid_case.json`](../../fixtures/graph/query_family_cases/docs_search_hybrid_case.json).

### 5.3 `topology_walk_cached_case`

A `topology_walk` invocation walks `topology_runtime_family` edges
off a `topology_node`. Envelope `readiness_state = warm_index`,
`result_truth_class = exact`, one row is
`cached_authoritative_snapshot` with a `cache_key_ref`. The envelope
carries a `partial_index_disclosed` disclosure.

See [`topology_walk_cached_case.json`](../../fixtures/graph/query_family_cases/topology_walk_cached_case.json).

### 5.4 `impact_explorer_inferred_case`

An `impact_explorer` invocation collects the impact set for a
subject symbol. One row is a `derived_edge` (`inferred_relation`,
`medium` confidence) with a `derived_inference` compatibility class.
The envelope carries a `heuristic_results_disclosed` disclosure and
a stable row anchor.

See [`impact_explorer_inferred_case.json`](../../fixtures/graph/query_family_cases/impact_explorer_inferred_case.json).

### 5.5 `ai_context_assembly_cited_case`

An `ai_context_assembly` invocation gathers the context set for a
subject symbol. Every row carries an `explainer_citation_slot`
pointing at a graph node id; no raw bodies cross the boundary. The
envelope's family is `ai_context_restricted`; rows include a
`derived_identity` annotation row.

See [`ai_context_assembly_cited_case.json`](../../fixtures/graph/query_family_cases/ai_context_assembly_cited_case.json).

### 5.6 `support_export_walk_bundle_case`

A `support_export_walk` invocation emits the support-export slice
for a subject set. Rows include a `missing_anchor_row` (a dropped
provider resource) and a `policy_hidden_row` (a policy-view
projection) so the support bundle exports honest absences rather
than silently omitting them. Envelope `safety_class =
support_export_bundle`.

See [`support_export_walk_bundle_case.json`](../../fixtures/graph/query_family_cases/support_export_walk_bundle_case.json).

### 5.7 `public_graph_query_scoped_case`

A `public_graph_query` invocation bounded by a named workset walks
`ownership_family` edges. Envelope `safety_class = public_api_safe`,
`result_envelope_version = 1`. Every row is a
`live_authoritative` durable row; scope refs carry
`partial_visible` where the walk crosses the scope boundary.

See [`public_graph_query_scoped_case.json`](../../fixtures/graph/query_family_cases/public_graph_query_scoped_case.json).

## 6. Changing this vocabulary

- **Additive-minor** changes (new `query_family_id`, new
  `query_safety_class`, new `query_intent_class`, new
  `result_row_class`, new `identity_durability_class` entry, new
  `edge_durability_class` entry, new
  `freshness_confidence_compatibility_class`, new
  `published_identity_constraint_id`, new
  `published_edge_constraint_id`, new
  `published_query_family_constraint_id`, new audit-event id) land
  here and in the companion schema in the same change. The change
  must cite the motivating fixture.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- `result_envelope_version` is bumped **only** on breaking envelope-
  shape changes; consumers refuse a newer envelope rather than
  guessing.

## 7. Acceptance

- Published node-identity, edge-family, and query-family records
  are versioned and machine-readable through
  [`node_identity.schema.json`](../../schemas/graph/node_identity.schema.json),
  [`edge_family.schema.json`](../../schemas/graph/edge_family.schema.json), and
  [`query_family.schema.json`](../../schemas/graph/query_family.schema.json).
- The internal `graph_node_record` and `graph_edge_record` vocabularies
  are reused by reference; no parallel vocabulary is minted.
- Every query-family fixture under
  [`/fixtures/graph/query_family_cases/`](../../fixtures/graph/query_family_cases)
  is a `result_envelope_record` that pins `query_family_id`,
  `readiness_state`, `result_truth_class`, and
  `result_envelope_version`.
- Durable-versus-derived is distinguished on every identity row and
  every edge row; freshness and confidence project through the
  named `freshness_confidence_compatibility_class`.
- The result envelope is versioned so future consumers refuse a
  newer envelope rather than guessing; no internal query-row shape
  leaks into the public contract.

## Linked artifacts

- Internal workspace-graph seed:
  [`docs/graph/workspace_graph_seed.md`](./workspace_graph_seed.md)
- Internal workspace-graph seed schema:
  [`schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json)
- Search readiness / truth vocabulary:
  [`schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
- Graph query-family alpha runtime:
  [`docs/graph/query_family_alpha.md`](./query_family_alpha.md)
- Reactive-state query-family packet:
  [`docs/verification/reactive_state_packet.md`](../verification/reactive_state_packet.md)
- Generated-artifact lineage schema:
  [`schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)
- Filesystem-identity vocabulary:
  [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
