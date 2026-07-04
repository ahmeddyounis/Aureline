# M5 execution-lifecycle component fixtures

Protected fixtures for task **M05-820** — the frozen reusable execution-lifecycle
component matrix (run/attempt headers, input-request prompts, artifact-publish rows,
rerun comparison sheets, debug session headers, thread/process trees, and dump/crash
artifact cards).

- `m5-execution-lifecycle-component-matrix.json` — the canonical matrix packet,
  byte-identical to the checked support export at
  `artifacts/release/m5-execution-lifecycle-component-proof/support_export.json` and
  to `seeded_execution_lifecycle_component_matrix()`.
- `m5-execution-lifecycle-component-matrix.csv` — the deterministic CSV projection of
  the same matrix rows.

The matrix validates against
[`schemas/ui/m5-execution-lifecycle-component-matrix.schema.json`](../../../schemas/ui/m5-execution-lifecycle-component-matrix.schema.json)
and against `ExecutionLifecycleComponentMatrix::validate`. Regenerate by rebuilding
the seeded packet in
`aureline_runtime::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`;
the `checked_support_export_matches_builder` unit test guards drift.
