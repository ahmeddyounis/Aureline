# Node debug loop + source-map stability (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:node_debug_sourcemaps_stability`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.next_js` (represents build-step semantics)

## Scenario goal

Prove that a Node service debug loop remains stable across:

- dev builds (fast rebuild, HMR-adjacent output where applicable), and
- production-like builds (optimized output, separate output directories),

without losing breakpoint mapping or stack-variable readability in
TypeScript source.

## Required truth and disclosures

- Artifact resolution must not guess. It cites the artifact-resolution
  packet family and names the build identity and symbol/source-map
  lineage:
  - `docs/debug/artifact_resolution_seed.md`
  - `docs/debug/chronology_and_reverse_execution_contract.md`
- Build outputs, generated artifacts, and source maps are treated as
  generated/projection artifacts with explicit posture:
  - `docs/architecture/generated_artifact_safe_edit_policy.md`

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_node_debug_attach_and_sourcemaps`

## Evidence hooks

- Debug artifact-resolution fixtures and proof packets referenced by the
  debug contracts above.

## Known-limit expectations

- Any narrowing (for example “source maps unavailable under build tool X”)
  must be captured as a known-limit note bound to the launch-wedge claim
  row before certified wording is admissible.

