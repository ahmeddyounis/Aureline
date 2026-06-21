# M5 Execution-Evidence Projection Overlays

- Packet: `m5-execution-evidence-projections:stable:0001`
- Label: `M5 execution-evidence projection overlays — preserved run/step/provider/artifact lineage`
- As of: `2026-06-21T00:00:00Z`
- Projections: 8
- Effective: 4 certified, 1 narrowed, 2 read-only overlay, 0 unreconstructable, 1 labs

| Projection | Kind | Origin | Claimed | Effective | Remap | Confidence |
| --- | --- | --- | --- | --- | --- | --- |
| projection:coverage-local-test:0001 | coverage_overlay | local_test | projection_certified | projection_certified | exact_current_revision | structured_full |
| projection:flaky-history-local-test:0001 | flaky_test_history | local_test | projection_certified | projection_certified | shifted_tracked | structured_full |
| projection:perf-regression-local-task:0001 | perf_regression_note | local_task | projection_certified | projection_narrowed | approximate_remap | heuristic_high |
| projection:notebook-verdict-cell:0001 | notebook_output_verdict | notebook_run | projection_certified | projection_certified | not_anchored | structured_full |
| projection:pipeline-annotation-provider:0001 | pipeline_annotation | pipeline_provider_run | projection_read_only_overlay | projection_read_only_overlay | shifted_tracked | provider_mapped |
| projection:review-marker-local-task:0001 | review_side_marker | local_task | projection_certified | projection_certified | exact_current_revision | structured_full |
| projection:coverage-imported-provider:0001 | coverage_overlay | imported_provider_evidence | projection_read_only_overlay | projection_read_only_overlay | approximate_remap | provider_mapped |
| projection:notebook-verdict-labs:0001 | notebook_output_verdict | notebook_run | projection_labs_not_claimed | projection_labs_not_claimed | not_anchored | heuristic_medium |

- Narrowed: `projection:perf-regression-local-task:0001` — Held at projection_narrowed below the projection_certified claim: verification proof stale; lineage stays reopenable via the generated artifact until re-verified
