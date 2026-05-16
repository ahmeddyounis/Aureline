# Test quality truth quality packets (beta)

This directory pins the canonical beta artifacts for the four claimed test-
quality dimensions — coverage, flaky verdict, snapshot review, and per-case
baseline. The reviewer-facing doc lives at
[`/docs/runtime/m3/test_quality_truth_beta.md`](../../../../docs/runtime/m3/test_quality_truth_beta.md);
the boundary schema lives at
[`/schemas/testing/test_quality_truth_beta.schema.json`](../../../../schemas/testing/test_quality_truth_beta.schema.json).

Files:

- `coverage_manifest.json` — canonical
  `test_quality_beta_coverage_manifest_record`. Integration tests under
  [`/crates/aureline-runtime/tests/testing_quality_beta.rs`](../../../../crates/aureline-runtime/tests/testing_quality_beta.rs)
  load this file verbatim and assert it equals the canonical manifest the
  runtime mints, so widening the closed framework / quality-kind /
  backing-artifact-kind vocabulary requires updating this file alongside the
  schema and the doc.

Adding a fixture is a vocabulary or coverage change that MUST update the
canonical schema, the reviewer doc, and the runtime tests together.
