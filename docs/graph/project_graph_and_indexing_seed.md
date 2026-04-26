# Project Graph And Incremental Indexing Contract Seed

This document seeds the project-graph and indexing contracts that sit
between workspace discovery and the semantic workspace graph. The
semantic graph already owns stable file, symbol, topology, docs,
ownership, generated-artifact, and query-family identity. This seed
adds the earlier project-model layer: repositories, modules, packages,
targets, environments, and framework facts that indexing, search,
review, AI, and Project Doctor must share before deeper semantic
analysis is warm.

The machine-readable schemas live at:

- [`/schemas/graph/project_node.schema.json`](../../schemas/graph/project_node.schema.json)
- [`/schemas/index/index_work_item.schema.json`](../../schemas/index/index_work_item.schema.json)

Companion fixtures live under:

- [`/fixtures/graph/project_graph_cases/`](../../fixtures/graph/project_graph_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, an ADR, or a later stable schema, those documents win and
this document must be updated in the same change.

## Why This Exists

The existing semantic graph answers "what is this symbol, file, edge, or
topology object?" after enough evidence has been harvested. The project
graph answers a slightly earlier question: "which project, package,
target, environment, and framework truth should every later worker use?"

Without this layer, search can point at one target id, build/test
events can use another, AI can assemble context against a guessed
package boundary, review can report impact against a stale workset, and
Project Doctor can diagnose an index row that no other surface can
name. The project graph prevents that drift by freezing one identity
model and one completeness vocabulary before implementation code lands.

## Scope

This seed covers:

- project nodes for repositories, modules, packages, targets,
  environments, and frameworks;
- edge families for containment, package/module declaration, target and
  environment use, dependency, framework, indexing, scoping, and bridge
  projections;
- hot-set and cold-set planning inputs;
- symbol-extraction source families and index shard classes;
- incremental work-item identity, execution locus, status, and
  invalidation triggers;
- bridge rules into semantic-graph identity, task-event target identity,
  execution-context target identity, search/query-family rows, review
  impact, AI context, support export, and Project Doctor evidence.

Out of scope:

- a production graph database;
- a production index scheduler;
- ranking algorithms;
- language/framework-specific analyzer implementations.

## Node Model

Every `project_node_record` carries one stable `project_node_id`, one
`project_node_class`, one class-specific body, and the cross-cutting
slots inherited from the semantic graph contract: provenance, freshness,
confidence, scope refs, source anchors, and completeness.

| Class | Purpose | Typical producers |
|---|---|---|
| `repo_node` | One opened repository, imported root, archive, mirror, or managed checkout. | workspace scanner, VFS, remote agent |
| `module_node` | Source, test, docs, generated, vendor, or slice module inside a repo. | workspace scanner, package resolver, framework pack |
| `package_node` | Ecosystem package or crate with manifest and lockfile identity. | package resolver, build adapter |
| `target_node` | Build, test, run, debug, deploy, service, or notebook target. | target discovery, task adapters, framework pack |
| `environment_node` | Local, remote, container, devcontainer, managed, notebook, or AI sandbox environment. | execution-context resolver, environment capsule |
| `framework_node` | Framework pack object such as route graph, component tree, migration graph, or infrastructure overlay. | framework analyzer, package resolver |

Identity rules:

1. `project_node_id` is opaque and stable for a workspace plus branch or
   snapshot epoch. Labels, paths, package names, and target display names
   are never substituted for it.
2. A target row that maps to execution must carry the same
   `canonical_target_id` that task events use through
   `workspace_or_target_identity.target_ref`.
3. An environment row that maps to launch or repair must carry the
   execution-context or environment-capsule refs that Project Doctor and
   support exports use.
4. Framework rows are project graph facts first. A framework pack may
   add derived routes, components, resources, or generated artifacts, but
   it does not mint a private graph hidden from search, AI, review, or
   support.

## Edge Families

Every `project_edge_record` carries one `project_edge_class`, one
`project_edge_family`, endpoint project node ids, the semantic graph
evidence-state vocabulary, and source anchors.

| Family | Edge classes | Rule |
|---|---|---|
| `containment_family` | `contains` | Repositories contain modules; modules contain packages or targets when the package manager models them that way. |
| `package_module_family` | `declares_module`, `declares_package` | Module and package declaration is targetable by search and review. |
| `target_execution_family` | `builds_target`, `tests_target`, `runs_in_environment`, `uses_environment`, `target_requires_package`, `target_emits_task_event` | Run, build, test, debug, AI, and Project Doctor all reuse target/environment ids. |
| `dependency_family` | `depends_on_package`, `depends_on_module` | Dependency facts name source class and freshness. |
| `framework_family` | `imports_framework`, `framework_owns_route`, `framework_generates_artifact` | Framework-derived truth is labeled as exact, imported, derived, stale, or partial. |
| `indexing_family` | `indexed_by_shard` | Index rows point back at the project nodes they materialize. |
| `scoping_family` | `scoped_by`, `omitted_from_scope` | Workset and policy boundaries are graph facts, not UI-only filters. |
| `bridge_family` | `semantic_graph_projection`, `query_family_surface_projection` | Downstream surfaces cite the bridge instead of copying identities. |

Approximate, imported, stale, and missing-anchor edges must not be
rendered as direct evidence. The `edge_evidence_state` value is the
machine-readable source of that honesty.

## Completeness Vocabulary

Project graph and indexing records share the same
`completeness_label` enum:

| Label | Meaning |
|---|---|
| `complete` | The record is complete for its declared scope, epoch, and producer class. |
| `omitted` | The producer intentionally left the record out because the scope, policy, or user filter excluded it. |
| `unfetched` | The producer knows an external, remote, managed, or imported source exists but has not fetched it. |
| `uninitialized` | The lane, shard, target, or framework source has not started for this scope. |
| `partially_indexed` | Some admitted inputs are indexed, but the declared scope is not complete. |
| `derived` | The record was inferred or generated from another source and must remain labeled as such. |
| `stale` | The record exists, but its freshness floor is older than the current authority epoch. |

Every non-`complete` record carries:

- `completeness_reason`;
- expected and known counts where meaningful;
- hidden or omitted count where meaningful;
- `next_action_class`, such as widen scope, fetch remote shard, start
  indexing, wait for warm-up, or run Project Doctor;
- source refs that let support and replay reconstruct why the label was
  emitted.

This label must travel through search rows, review packets, AI evidence
packets, Project Doctor findings, and support exports unchanged.

## Hot-Set And Cold-Set Planning

The indexing schema freezes `hot_set_plan_record` so the planner can
explain why work is hot, deferred, omitted, stale, or blocked.

Hot-set inputs include:

- open files;
- recent edits;
- changed files;
- active build/test/debug targets;
- nearby tests;
- import or dependency neighborhoods;
- diagnostic neighborhoods;
- user-pinned paths or targets;
- restored session state.

Cold-set work is not a failure. It is recorded as a deferred plan with
scope, expected size, freshness floor, and the work items that will
materialize it later. Worksets and sparse slices are declared scope, so a
work item may be `complete` for a slice while broader repo shards remain
`omitted` or `unfetched`.

## Index Work Items And Shards

Every `index_work_item_record` carries:

- `work_item_id`, `idempotency_key`, and `producer_ref`;
- `work_item_kind`;
- lane and execution locus;
- set temperature (`hot_set`, `cold_set`, or `maintenance_set`);
- project node, target node, shard, and source-family refs;
- status and pause reason;
- completeness;
- invalidation refs;
- expected output refs.

Every `index_shard_record` carries:

- `shard_id` and `shard_class`;
- workspace, project nodes, target nodes, scope refs, and epoch refs;
- source families used to build it;
- execution locus (`local`, `remote_agent`, `managed_workspace`,
  `imported_bundle`, or `support_replay`);
- readiness state from the search-result truth vocabulary;
- completeness, provenance, freshness, and confidence;
- invalidation triggers.

Local, remote, managed, imported, and replayed rows use the same logical
schema. Placement changes where work runs, not the identity model.

## Invalidation Rules

Index invalidation must target the smallest useful scope. Full rebuilds
are reserved for schema, workspace epoch, producer-version, or
cache-format boundaries.

The frozen trigger vocabulary includes file content changes,
filesystem identity changes, root changes, workset changes, dependency
manifest and lockfile changes, build config changes, target graph
changes, environment capsule changes, framework/language/generator
version changes, policy/trust changes, branch changes, remote reconnect,
managed suspend/resume, cache schema change, producer version change,
and explicit user rebuild.

Invalidation records name:

- affected project nodes;
- affected shards;
- affected work items;
- graph epoch before and after;
- rebuild policy;
- provenance and freshness.

## Bridge Rules

Bridge records prevent downstream identity forks.

1. **Semantic graph bridge.** A project node maps to zero or more
   semantic graph node ids through `semantic_graph_node_refs`. A
   semantic graph node may deepen the answer but does not replace the
   project node id for repo, package, target, environment, or framework
   identity.
2. **Build/test event bridge.** Target project nodes carry
   `canonical_target_id`; task events cite that same value through
   `workspace_or_target_identity.target_ref` and `build_target_id`.
3. **Execution-context bridge.** Environment project nodes carry
   `execution_context_id_ref`, `environment_capsule_ref`, and
   capability refs. Project Doctor probes read those refs instead of
   reconstructing environment identity from logs.
4. **Query-family bridge.** Search, symbol jump, topology, review,
   support, and AI context surfaces cite project node ids or bridge ids
   in result envelopes and evidence packets. They may render labels, but
   labels are not identity.
5. **Completeness bridge.** Readiness and completeness ceilings move
   downstream. A search row sourced only from a hot shard cannot render
   as full-workspace complete. A review impact row sourced from a stale
   target graph remains stale until the target graph is refreshed.

## Fixture Coverage

The seed fixtures cover:

- a local Rust workspace with repo, module, package, target,
  environment, and framework nodes;
- hot-set planning over active targets and nearby tests;
- local, remote, and managed shard/work-item rows;
- omitted, unfetched, uninitialized, partially indexed, derived, and
  stale completeness labels;
- invalidation from dependency, target, environment, framework, branch,
  and managed-lifecycle changes;
- bridge records for semantic graph ids, task-event ids, query-family
  surfaces, AI context, review impact, support export, and Project
  Doctor evidence.

