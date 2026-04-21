# Semantic-workspace-graph seed object model

This document seeds the shared semantic-workspace-graph so every
later surface that reads the graph — search-planner, graph overlay,
topology map, impact explorer, cited-explainer overlay, AI context
assembler, review pack, support bundle, public query-family surface —
reuses **one identity layer** instead of minting a private node
class, private edge class, private identity token, or private
freshness / confidence vocabulary.

The machine-readable schema lives at:

- [`/schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/example_workspace_graphs/`](../../fixtures/graph/example_workspace_graphs/)

A minimal in-memory prototype that loads the fixtures and enforces
the identity / label rules spelled out in §6 lives under:

- [`/prototypes/graph/README.md`](../../prototypes/graph/README.md)
- [`/crates/aureline-graph-proto/`](../../crates/aureline-graph-proto)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or a superseding ADR, those documents win and this
document MUST be updated in the same change. Where this document
disagrees with a downstream search, docs, AI, public-graph, review,
or support surface's mint of its own fields, this document wins and
the surface is non-conforming.

## Why freeze this now

An IDE that promises a unified graph — "ask any question once, get
one answer with provenance" — cannot land honest search, AI, docs,
or review UX until node identity, edge identity, provenance,
freshness, confidence, and scope are one vocabulary. Left implicit,
each lane answers "what is this node, really?" slightly differently:
the symbol-jump resolver calls it a symbol, the graph overlay calls
it a node, the AI context assembler calls it a context item, the
review pack calls it an impacted object, the support bundle exports
it as an entity. The goal here is one frozen identity layer so every
surface tells the same story about the same node.

The second hazard is silent inference. A graph that renders a
heuristic-inferred relation with the same weight as a parser-
confirmed relation has already lied once. Freezing the evidence-state
axis (`direct_evidence`, `imported_evidence`, `inferred_relation`,
`stale_relation`, `missing_anchor`) in the schema — at the edge
level, in machine-readable form — makes that honesty contract
enforceable rather than aspirational.

## Scope

- Freeze one `graph_node_record` and one `graph_edge_record` shape
  that covers file, directory, symbol, doc, ownership, topology,
  provider-resource, generated-artifact, imported-root, workset /
  scope, policy-view, and missing-anchor nodes.
- Freeze one `workspace_graph_record` that bundles a snapshot so
  fixtures, support bundles, replay captures, and the eventual
  public query-family surface all read the same record shape.
- Freeze the cross-cutting slots every surface needs — `source_class`,
  `provenance_class`, `freshness`, `confidence_level`,
  `confidence_rollup`, `query_family_tags`, `shard_affinity_tags`,
  `invalidation_producer_tags`, `workset_scope_ref`,
  `source_anchor`, `impact_reason_slot`, `explainer_citation_slot`,
  and `topology_edge_slot` — so later graph UX reuses one identity
  layer.
- Pin the `edge_evidence_state` axis so the graph distinguishes
  direct evidence, imported evidence, inferred relation, stale
  relation, and missing-anchor states in machine-readable form.
- Seed example fixtures covering **files, symbols, docs, ownership,
  topology, provider resources, and generated-artifact lineage**,
  plus scenarios covering **imported roots, generated artifacts,
  provider-owned objects, and partial workset visibility** with
  typed provenance and freshness.
- Keep identity stable: every node and edge carries an opaque
  `node_id` / `edge_id` quoted verbatim by every citation; human-
  friendly labels MAY be shown alongside but never substituted.

## Out of scope

- Full indexing, retrieval ranking, or graph query execution. Those
  land with the search-planner and public query-family work; this
  seed freezes only the record shape and vocabulary each consumer
  will read.
- The on-disk encoding, sharding strategy, or materialization plan.
  `shard_affinity_tags` name the shard families provisionally so the
  later materialization work partitions without renaming identity;
  the concrete shard layout is a separate decision.
- The explainer UI and the AI-context assembly algorithm. Both
  consume `explainer_citation_slot`s and `impact_reason_slot`s from
  here; their surface design is a separate workstream.
- The public graph API's transport. The schema is the cross-tool
  boundary; the transport will quote these records rather than
  re-inventing them.

## 1. Node identity

Every graph node carries exactly one `node_class` from the frozen
vocabulary:

| Class                       | Anchors                                                                                                  | Key body slot                                                                  |
|-----------------------------|----------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------|
| `file_node`                 | A workspace-resident file                                                                                | ADR-0006 five-layer `filesystem_identity_ref`                                  |
| `directory_node`            | A workspace-resident directory (root, source tree, docs tree, fixtures tree, generated tree)             | `filesystem_identity_ref` + optional `role`                                    |
| `symbol_node`               | A language symbol (function, method, type, trait, enum variant, constant, macro, package, schema node)   | `symbol_kind` + `declared_in_file_node_id` + stable `qualified_path`            |
| `doc_node`                  | A workspace doc file, docs-pack entry, ADR, RFC, README, inline docstring, changelog, migration note     | `doc_kind` + `doc_ref` + optional `anchor_filesystem_identity`                 |
| `ownership_node`            | A person, team, service account, or rotation role                                                        | `ownership_kind` + `ownership_ref` + redaction-aware `display_label`           |
| `topology_node`             | A package, crate, service, deploy target, runtime host, build target, test target, pack                  | `topology_kind` + `topology_ref` + optional `environment_class`                |
| `provider_resource_node`    | A connected-provider object (repo, issue, PR, CI run, package coord, registry entry, cloud resource)     | `provider_kind` + opaque `resource_handle` + optional `reachability_state`     |
| `generated_artifact_node`   | A build output, codegen sibling, lockfile, notebook output, preview snapshot, mirrored pack artifact     | `lineage_record_ref` into `generated_artifact_lineage.schema.json`, drift_state |
| `imported_root_node`        | A workspace root imported from an external source (vendor drop, migration import, signed upstream, pack) | `import_kind` + `import_ref` + optional `filesystem_identity`                  |
| `workset_scope_node`        | A named workset / slice the graph first-classes so scope narrowing is visible in the graph itself        | `workset_scope_ref` + redaction-aware `display_label`                          |
| `policy_view_node`          | A policy-limited projection of a broader scope; the underlying node ids are NOT exposed through this     | `underlying_scope_id` + `policy_ref` + `hidden_member_count`                   |
| `missing_anchor_node`       | A referenced-but-unresolved target (deleted file, dropped symbol, unreachable provider object)           | `expected_node_class` + `missing_reason` + optional `last_known_ref`           |

Identity rules the prototype enforces and the schema encodes:

1. Every node carries a stable, opaque `node_id`. Ids are never
   reused after a node is retired; retirement surfaces through a
   `graph_audit_event_record` with `audit_event_id =
   graph_node_retired`.
2. File, directory, and generated-artifact identity routes through
   the ADR-0006 five-layer `filesystem_identity_ref`. The graph
   does NOT mint a parallel filesystem identity.
3. Generated-artifact identity is paired: the graph node's
   `lineage_record_ref` MUST resolve to a record validated by
   `schemas/workspace/generated_artifact_lineage.schema.json`. A
   generated artifact without a lineage ref is a contract
   violation; surfaces render an `unknown_lineage` badge rather
   than hiding the node.
4. Imported-root nodes carry a typed `import_kind` and `import_ref`.
   Surfaces that hide imported content (signed upstream bundles
   users cannot edit) render the imported root explicitly so it is
   visible in the graph rather than silently folded into a local
   root.
5. Policy-view nodes project a scope without exposing the
   underlying node ids; only `hidden_member_count` projects. The
   exact hidden list is never exposed outside the policy-admin
   surface.
6. Missing-anchor nodes are first-class. A reference to a deleted
   file, a dropped symbol, an unreachable provider object, or a
   scope-removed target resolves to a missing-anchor node with a
   typed `missing_reason`; surfaces render the missing-anchor
   badge instead of silently hiding the reference.

## 2. Edge identity and evidence state

Every graph edge carries exactly one `edge_class` from the frozen
vocabulary: `contains`, `defines_symbol`, `references_symbol`,
`imports_module`, `depends_on`, `owned_by`, `documented_by`,
`cites`, `generated_from`, `mirrors_upstream`, `deployed_to`,
`runs_in`, `hosted_by`, `produces_artifact`, `consumes_artifact`,
`impacts`, `explains`, `scoped_by`, `aliases`, `missing_anchor_for`.

Edges carry two endpoint refs (`from_node_id`, `to_node_id`), both
of which MUST resolve within the same `workspace_graph_record` (or
resolve to a first-class `missing_anchor_node` that names the
expected class). Topology edges carry a `topology_edge_slot` so
topology maps and impact explorers reuse the same edge record
rather than minting a private topology record.

Every edge also carries one `edge_evidence_state`:

| State                 | Meaning                                                                                                               |
|-----------------------|-----------------------------------------------------------------------------------------------------------------------|
| `direct_evidence`     | The producer observed the relation first-hand (symbol resolver, parser, CODEOWNERS resolver, build-toolchain output). |
| `imported_evidence`   | The relation was imported from an external signed or replayed source.                                                 |
| `inferred_relation`   | The relation was derived by a heuristic, cross-reference, or inference pass and carries lower default confidence.     |
| `stale_relation`      | Observed earlier and not refreshed; surfaces render a staleness badge and carry a typed `stale_reason`.               |
| `missing_anchor`      | One endpoint is a `missing_anchor_node`; surfaces render the missing-anchor badge instead of hiding the edge.         |

This is the honesty axis the graph UX reads when it decides whether
to render a row with a confidence chip, a staleness chip, a
missing-anchor badge, or plain.

## 3. Cross-cutting slots every node and edge carries

These slots are the reason this seed can be one identity layer for
every later graph-reading surface:

- **`provenance_stamp`** — one per record, required. Pairs
  `source_class` (which producer family emitted the record:
  `workspace_filesystem`, `buffer_editor`, `symbol_resolver`,
  `docs_pack`, `codeowners_resolver`, `build_toolchain`,
  `codegen_tool`, `package_resolver`, `notebook_kernel`,
  `preview_runtime`, `connected_provider`, `remote_agent`,
  `ai_inference`, `imported_bundle`, `replay_capture`,
  `policy_projection`, `manual_annotation`) with
  `provenance_class` (how the current record came to carry this
  data: `authoritative_producer`, `projected_from_producer`,
  `imported_external`, `replayed_capture`, `inferred_by_heuristic`,
  `policy_projected`, `manually_annotated`).
- **`freshness_frame`** — one per record. `freshness` re-exports
  ADR-0005 (`authoritative`, `warming`, `cached`, `stale`,
  `replayed`, `imported`); non-authoritative frames carry a typed
  `stale_reason`; warming frames carry an optional
  `warming_progress_hint` that aligns with ADR-0014 readiness.
- **`confidence_level`** — one per record (`high`, `medium`, `low`,
  `unknown`). Consumers render the label verbatim; a cited-
  explainer overlay MUST NOT silently upgrade a low-confidence
  relation.
- **`confidence_rollup`** — optional. Multi-source nodes and edges
  carry one rollup so graph UX, AI explainers, and support
  bundles agree on one confidence label rather than cherry-picking
  a contributor. Rollup policy: any `low` contributor pulls the
  rollup to at least `low`; any `unknown` contributor pulls to at
  least `low`; otherwise the rollup takes the minimum.
- **`source_anchors`** — one or more per record (empty only for
  synthetic nodes). Each anchor names an `anchor_kind`
  (`filesystem_identity`, `symbol_definition_site`,
  `docs_pack_entry`, `mutation_journal_entry`,
  `generated_artifact_lineage`, `provider_resource_handle`,
  `imported_bundle_entry`, `replay_capture_entry`,
  `codeowners_rule`, `annotation_note`) and carries an
  `anchor_ref` plus an optional `line_range` hint. Raw source
  bytes never cross this boundary; anchors are references only.
- **`impact_reasons`** — optional. Impact explorers, review packs,
  and AI explainers reuse this shape (`direct_edit`,
  `symbol_rename`, `signature_change`, `dependency_bump`,
  `generated_artifact_regeneration`, `policy_change`,
  `ownership_change`, `provider_resource_update`,
  `imported_bundle_rollover`, `workset_scope_narrowed`,
  `workset_scope_widened`, `inferred_transitive_impact`) plus a
  redaction-aware short rationale and optional mutation-journal /
  review refs.
- **`explainer_citations`** — optional. Cited-explainer overlays
  and AI context assemblers emit citations using the SAME node and
  edge ids the graph exposes, via this slot; they do NOT mint a
  private citation pointer.
- **`topology_edge_slot`** — on edges only, optional. Topology
  relations name `topology_kind` (`package_depends_on_package`,
  `service_calls_service`, `build_target_produces_artifact`,
  `runtime_hosts_package`, `deploy_target_runs_service`,
  `provider_hosts_resource`, `pack_mirrors_upstream`,
  `notebook_kernel_executes_cell`,
  `preview_runtime_renders_snapshot`) plus optional
  `environment_class` and `deployment_tag`.

### 3.1 Provisional planning tags

Three tag families are **provisional** on this seed. They are
frozen so that search-planner and public-graph work do not fork
identity rules later, but they are explicitly additive-minor: a new
tag lands by bumping `workspace_graph_schema_version` and updating
this document in the same change.

- **`query_family_tags`** — one or more per record. Named families:
  `lexical_text_search`, `symbol_jump`, `semantic_code_search`,
  `docs_search`, `ownership_lookup`, `topology_walk`,
  `impact_explorer`, `dependency_walk`,
  `generated_artifact_lineage_walk`, `provider_resource_lookup`,
  `cited_explainer_walk`, `ai_context_assembly`,
  `public_graph_query`, `review_impact_walk`,
  `support_export_walk`.
- **`shard_affinity_tags`** — one or more per record. Named
  affinities: `workspace_root_local`, `per_root_index`,
  `symbol_cache_shard`, `docs_pack_shard`, `graph_overlay_shard`,
  `provider_overlay_shard`, `ai_context_shard`,
  `policy_projected_shard`, `ephemeral_session_shard`,
  `imported_bundle_shard`.
- **`invalidation_producer_tags`** — one or more per record. Names
  which producer family invalidates the record so the subscription
  fabric can route invalidation frames without inventing producer
  synonyms: `workspace_vfs_writer`, `buffer_editor_commit`,
  `symbol_resolver_rebuild`, `docs_pack_refresh`,
  `codeowners_rule_change`, `build_toolchain_run`, `codegen_run`,
  `package_resolver_refresh`, `notebook_kernel_execute`,
  `preview_runtime_refresh`, `connected_provider_event`,
  `remote_agent_event`, `ai_inference_refresh`,
  `imported_bundle_update`, `replay_capture_reload`,
  `policy_epoch_roll`.

## 4. Workset / scope references

Every node and edge carries one or more `workset_scope_ref`
entries. `scope_class` re-exports the execution-context /
scope vocabulary: `current_root`, `named_workset`, `sparse_slice`,
`full_workspace`, `policy_limited_view`, `review_workspace`,
`companion_surface`. Each ref adds a `visibility` value:

| Visibility         | Meaning                                                                                                                  |
|--------------------|--------------------------------------------------------------------------------------------------------------------------|
| `fully_visible`    | The node or edge is fully visible within the named scope.                                                                |
| `partial_visible`  | The node or edge is visible but one or more contributors are out-of-scope; surfaces render a partial-visibility chip.    |
| `policy_hidden`    | Policy hides the node in this scope; surfaces render a policy-hidden badge and the body is projected through a `policy_view_node`. |
| `missing_in_scope` | The node is referenced but absent from the scope; surfaces render the missing-anchor badge instead of hiding silently.   |

Scope narrowing is therefore a first-class, machine-readable graph
relation (edge class `scoped_by`). Surfaces that narrow or widen
the scope emit `graph_workset_scope_narrowed` /
`graph_workset_scope_widened` audit events.

## 5. Evidence-state × provenance-class interaction

The evidence-state and provenance-class axes are orthogonal. A
relation may be `direct_evidence` emitted by a
`projected_from_producer` (surface is projecting the symbol
resolver's truth) OR `inferred_relation` emitted by an
`authoritative_producer` (an inference engine is the canonical
owner of its inferences). The graph UX renders:

- The evidence-state chip (honesty about how strong the claim is).
- The freshness chip (when the claim was last refreshed).
- The confidence chip (how sure the producer was).

All three may render at once; surfaces MUST NOT silently fold one
into another.

## 6. Identity / label rules the prototype enforces

The minimal in-memory prototype under
[`/crates/aureline-graph-proto/`](../../crates/aureline-graph-proto)
reads each fixture, builds the in-memory graph, and enforces these
rules. A violation is a test failure, not a warning.

1. **Schema version present.** Every record carries
   `workspace_graph_schema_version = 1`.
2. **Record kind discriminator.** Every record carries one of the
   four `record_kind` values.
3. **Unique node ids.** Every `node_id` in a
   `workspace_graph_record.nodes` array is unique.
4. **Unique edge ids.** Every `edge_id` in a
   `workspace_graph_record.edges` array is unique.
5. **Edge endpoints resolve.** `from_node_id` and `to_node_id`
   resolve to a node id present in the same graph record, OR to a
   `missing_anchor_node` that names an `expected_node_class` and a
   typed `missing_reason`.
6. **Node class / body agreement.** A `file_node` carries a
   `filesystem_identity`; a `symbol_node` carries a
   `declared_in_file_node_id` that resolves to a `file_node` in
   the same graph; a `generated_artifact_node` carries a
   `lineage_record_ref` and a `drift_state`.
7. **Label vocabulary is closed.** Every enum field (class,
   evidence-state, freshness, confidence, source class, provenance
   class, query-family tag, shard-affinity tag, invalidation-
   producer tag, impact-reason class, explainer-citation class,
   topology kind) matches the frozen vocabulary.
8. **Freshness ↔ stale-reason consistency.** A record with
   `freshness == authoritative` MUST NOT carry a `stale_reason`;
   records with any other freshness MUST carry one.
9. **Confidence-rollup floor.** When `confidence_rollup` is
   present, the `rolled_up_level` is never strictly higher than
   the minimum of `source_confidences`, and an `unknown`
   contributor pulls the rollup to at least `low`.
10. **Explainer citation id reuse.** Every
    `explainer_citation_slot.citation_ref` on an edge or node is
    either a `node_id` or an `edge_id` present in the graph, or an
    opaque ref with a non-`file_range` / non-`symbol_definition`
    `citation_class` (external refs use their own namespaces).
11. **Topology edge slot presence.** An edge with one of the
    topology `edge_class` values (`deployed_to`, `runs_in`,
    `hosted_by`, `produces_artifact`, `consumes_artifact`,
    `mirrors_upstream`, `depends_on`) carries a
    `topology_edge_slot`. The prototype reports any missing slot.

## 7. Worked examples

Each example references a companion fixture under
[`/fixtures/graph/example_workspace_graphs/`](../../fixtures/graph/example_workspace_graphs/).
The fixtures are human-authored and validated by the prototype.

### 7.1 Local-root workspace

A single local root contains one source file that declares one
symbol; the file is owned by a team; the symbol is documented by a
README entry. Nodes: `file_node`, `symbol_node`, `doc_node`,
`ownership_node`, `directory_node` (the root), `workset_scope_node`
(the current root). Edges: `contains`, `defines_symbol`,
`documented_by`, `owned_by`, `scoped_by`.

See [`local_root_workspace.json`](../../fixtures/graph/example_workspace_graphs/local_root_workspace.json).

### 7.2 Generated-artifact lineage

A build run produces a compiled binary. The graph carries the
`file_node` for the source, a `generated_artifact_node` for the
binary (paired with a `generated_artifact_lineage_record` reference),
a `topology_node` for the build target, a `topology_edge_slot` on
the `produces_artifact` edge, and a `generated_from` edge into the
source. Drift state is `in_sync`; freshness is `authoritative` on
the source and `cached` on the artifact.

See [`generated_artifact_lineage.json`](../../fixtures/graph/example_workspace_graphs/generated_artifact_lineage.json).

### 7.3 Provider-owned resources

A connected code host owns a repository resource; the repo's
issue-tracker has an open issue referencing a workspace symbol.
Nodes: `provider_resource_node` (repo), `provider_resource_node`
(issue), `symbol_node`, `file_node`. Edges: `hosted_by`,
`references_symbol` (evidence state `inferred_relation`,
confidence `medium`), and an `impacts` edge with an
`impact_reason_slot`. A `cites` edge on a `doc_node` proves cited-
explainer IDs reuse the graph node id namespace.

See [`provider_resources_and_citations.json`](../../fixtures/graph/example_workspace_graphs/provider_resources_and_citations.json).

### 7.4 Imported root

A vendor drop imports a third-party source tree into the workspace
under a read-only imported root. Nodes: `imported_root_node`
(trust_state `restricted`), `file_node` (imported), `ownership_node`
(upstream maintainer), `workset_scope_node`. Edges: `contains`,
`owned_by` (`imported_evidence`), `scoped_by`. Provenance class on
the imported rows is `imported_external`; freshness is `imported`;
confidence is `medium` with a rollup note.

See [`imported_root_vendor_drop.json`](../../fixtures/graph/example_workspace_graphs/imported_root_vendor_drop.json).

### 7.5 Partial workset visibility with missing anchors

A named workset narrows visibility to one module; a reference
crosses the scope boundary to a file that is policy-hidden, and a
dropped symbol shows up as a `missing_anchor_node`. Nodes:
`workset_scope_node` (named workset), `policy_view_node`,
`missing_anchor_node`, plus the in-scope `file_node` and
`symbol_node`. Edges: `scoped_by` with
`visibility = partial_visible`, `missing_anchor_for`,
`references_symbol` with evidence state `stale_relation` and
stale_reason `upstream_input_stale`.

See [`partial_workset_visibility.json`](../../fixtures/graph/example_workspace_graphs/partial_workset_visibility.json).

## 8. Surface rules

These rules apply to every surface that renders, logs, exports, or
reasons about the records defined in §1–§5.

1. **No surface invents a private node or edge class.** Every
   consumer reads `node_id`, `edge_id`, `node_class`, `edge_class`,
   `evidence_state`, `source_class`, `provenance_class`,
   `freshness`, and `confidence_level` from the graph records;
   surfaces do not add parallel fields when they render.
2. **Topology maps reuse graph edges.** A `deployed_to`, `runs_in`,
   `hosted_by`, `produces_artifact`, `consumes_artifact`,
   `mirrors_upstream`, or `depends_on` edge with a
   `topology_edge_slot` is the topology edge; topology surfaces
   MUST NOT maintain a parallel topology record.
3. **Impact explorers reuse graph nodes.** Impacted objects and
   their reasons are `impacts` edges carrying an
   `impact_reason_slot`; impact surfaces MUST NOT invent a private
   impact record.
4. **Cited explainers reuse graph ids.** Every explainer citation
   is an `explainer_citation_slot` whose `citation_ref` is a graph
   node id or a graph edge id where applicable; overlays MUST NOT
   mint a private citation pointer.
5. **Generated artifacts always carry lineage.** A
   `generated_artifact_node` without a `lineage_record_ref` is a
   contract violation; surfaces render a degraded / unknown-lineage
   badge instead of hiding the node.
6. **Missing anchors are visible.** A reference to a deleted,
   dropped, or unreachable target resolves to a
   `missing_anchor_node`; surfaces render the badge instead of
   silently hiding the reference.
7. **Policy-hidden is visible as hidden.** A
   `policy_limited_view` scope projects a `policy_view_node` with
   `hidden_member_count`; the exact hidden list never projects.
8. **Scope narrowing is honest.** Every node and edge carries one
   or more `workset_scope_ref`s with explicit `visibility`; a
   surface that narrows or widens scope emits the matching audit
   event.

## 9. Changing this vocabulary

- **Additive-minor** changes (new `node_class`, new `edge_class`,
  new `source_class`, new `provenance_class`, new `freshness`
  token, new `stale_reason`, new `confidence_level`, new
  `query_family_tag`, new `shard_affinity_tag`, new
  `invalidation_producer_tag`, new `impact_reason.reason_class`,
  new `explainer_citation.citation_class`, new
  `topology_edge_slot.topology_kind`, new `audit_event_id`) land
  here and in the companion schema in the same change. The change
  must cite the motivating fixture.
- **Repurposing** an existing value is breaking. It opens a new
  decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The relevant upstream ADR wins on any disagreement; this
  document and the schema are updated in the same change when
  that happens.

## 10. Acceptance

- The object model is versioned
  (`workspace_graph_schema_version = 1`) and machine-readable
  through [`/schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json).
- Every node and edge carries an explicit opaque id; position-only
  references are forbidden.
- Fixtures under
  [`/fixtures/graph/example_workspace_graphs/`](../../fixtures/graph/example_workspace_graphs/)
  include freshness, confidence, and source classification on
  every node and edge, aligning with search, docs, AI, and
  support truth work.
- Later search, docs, AI, and public query-family tasks cite this
  seed instead of inventing private models (rules §6, surface rules
  §8).
- The fixtures include at least one scenario covering
  generated-from edges, provider-resource nodes, and remote-ish /
  imported roots with freshness and invalidation provenance
  (`generated_artifact_lineage.json`,
  `provider_resources_and_citations.json`,
  `imported_root_vendor_drop.json`).
- Topology maps, impact explorers, and cited explainers reuse the
  graph node and edge ids through `topology_edge_slot`,
  `impact_reason_slot`, and `explainer_citation_slot`; no surface
  invents separate private identifiers.
- The seed distinguishes `direct_evidence`, `imported_evidence`,
  `inferred_relation`, `stale_relation`, and `missing_anchor` in
  machine-readable form on every edge.

## Linked artifacts

- ADR (filesystem identity, save pipeline, cache identity):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- ADR (subscription envelope and invalidation semantics):
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- ADR (execution-context and scope):
  [`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md).
- ADR (search readiness, ranking, result truth):
  [`docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md).
- Generated-artifact lineage schema:
  [`schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json).
- Execution-context schema:
  [`schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json).
- Filesystem-identity vocabulary:
  [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
- Workspace-graph seed schema (this document):
  [`schemas/graph/workspace_graph_seed.schema.json`](../../schemas/graph/workspace_graph_seed.schema.json).
- Workspace-graph seed fixtures:
  [`fixtures/graph/example_workspace_graphs/`](../../fixtures/graph/example_workspace_graphs/).
- Workspace-graph seed prototype:
  [`prototypes/graph/README.md`](../../prototypes/graph/README.md).
