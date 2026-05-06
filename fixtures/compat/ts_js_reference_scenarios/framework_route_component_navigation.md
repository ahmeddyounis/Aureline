# Framework route → component navigation (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:framework_route_to_component_workflow`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.react`
  - `framework_pack:typescript_web.next_js`

## Scenario goal

Prove that framework-aware navigation works end-to-end:

- route entry points resolve to the component tree;
- related tests/config resolve as “attached” context; and
- the framework-derived graph is visible as project-graph truth (not a
  private heuristic hidden inside one surface).

## Workspace shape (representative)

- A Next.js app with:
  - at least one route module;
  - at least one shared component module;
  - at least one test module that exercises the route/component pair; and
  - framework configuration files that influence routing/build (for
    example, Next config, TS config, lint config).

## Required truth and disclosures

- Framework-derived facts must name their evidence state (exact/imported/
  derived/partial/stale), per `docs/graph/project_graph_and_indexing_seed.md`.
- Navigation must degrade honestly when framework packs are absent or
  incomplete; it must not silently fall back to misleading “best guess”
  claims of framework structure.

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_next_route_to_component_navigation`

## Evidence hooks

- Project-graph and indexing contract seed:
  `docs/graph/project_graph_and_indexing_seed.md`
- Framework-pack inventory and ownership:
  `artifacts/product/framework_pack_owners.yaml`

## Known-limit expectations

- Any narrowing (for example “route graph incomplete under partial index”)
  must be captured as a known-limit note before the row can be used for a
  certified claim.

