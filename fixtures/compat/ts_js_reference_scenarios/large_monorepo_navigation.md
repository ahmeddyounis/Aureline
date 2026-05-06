# Large TS/JS monorepo navigation (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:monorepo_first_useful_result`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.react`
  - `framework_pack:typescript_web.vite`

## Scenario goal

Prove that opening a large multi-package TypeScript/JavaScript monorepo
reaches a first useful state quickly, without pretending that background
indexing is complete.

“First useful” means:

- repo tree and module/package boundaries are visible;
- search and basic symbol navigation return results with honest
  completeness/confidence labels; and
- the user can begin a safe edit loop before full semantic analysis
  completes.

## Workspace shape (representative)

- A workspace with multiple packages under a single repo root:
  - at least one shared library package consumed by multiple apps;
  - project references (`tsconfig` references) or equivalent multi-project
    TypeScript configuration;
  - path aliases that cross package boundaries;
  - generated artifacts present but non-authoritative (for example,
    generated declaration output directories and build caches).

## Required truth and disclosures

- Index warm-up cannot be hidden: incomplete results must carry the
  shared completeness/freshness labels from the graph/indexing contract.
- “Fast open” must not require unreviewed background execution that widens
  trust or network egress.

## Benchmark/workflow reservations (must be materialised before certification)

- `archetype.ts_web_app_first_open_certified`
- `workflow.first_useful_edit_ts_web_app`

## Evidence hooks

- Project-graph and indexing truth: `docs/graph/project_graph_and_indexing_seed.md`
- Archetype program and evidence burden rules:
  `artifacts/compat/archetype_rubric.yaml` and
  `artifacts/compat/reference_workspace_rows.yaml`
- Certified-archetype report packet shape:
  `docs/release/certified_archetype_report_template.md`
- Compatibility-report packet shape:
  `docs/release/compatibility_report_template.md`

## Known-limit expectations

- If first useful results rely on partial indexing, certification requires
  a current known-limit note that explains the narrowing and the recovery
  route, per `docs/product/known_limits_contract.md`.

