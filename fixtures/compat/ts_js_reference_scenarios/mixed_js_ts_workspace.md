# Mixed JS/TS workspace with path aliases (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:mixed_js_ts_path_aliases`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.react`

## Scenario goal

Prove that a mixed JS/TS workspace remains coherent when:

- module systems differ across packages (CJS/ESM boundaries);
- TypeScript is partially adopted (transitional typing states); and
- path aliases are used to bridge package boundaries.

Coherence requirements:

- diagnostics are attributable and do not contradict each other silently;
- navigation does not imply certainty beyond the current index/completeness
  state; and
- edits remain safe under partial semantic warm-up.

## Required truth and disclosures

- Partial and stale results are labeled through the shared completeness and
  evidence-state vocabularies:
  - `docs/graph/project_graph_and_indexing_seed.md`

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_mixed_workspace_path_aliases`

## Known-limit expectations

- Any narrowing (for example “path-alias completion degrades under mixed
  module mode”) must be promoted into a known-limit note before certified
  wording is admissible.

