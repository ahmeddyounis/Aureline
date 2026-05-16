# Test runner beta fixtures (M3)

These fixtures back the beta promote of the test runner: tree views, inline
results, rerun-last parity, and structured artifact identity.

The reviewer-facing landing page lives at
[`/docs/runtime/m3/test_runner_beta.md`](../../../../docs/runtime/m3/test_runner_beta.md).
The runtime implementation lives at
[`/crates/aureline-runtime/src/testing/`](../../../../crates/aureline-runtime/src/testing/).
The shell consumer lives at
[`/crates/aureline-shell/src/test_runner_beta/`](../../../../crates/aureline-shell/src/test_runner_beta/).

## Cases

| Fixture | Scenario |
| --- | --- |
| `coverage_manifest.json` | Canonical coverage manifest checked in for round-trip parity. The integration test asserts that `TestRunnerBetaCoverageManifest::canonical(...)` matches this file byte-for-byte after a `serde_json` round-trip. |

The integration test at
[`/crates/aureline-runtime/tests/test_runner_beta.rs`](../../../../crates/aureline-runtime/tests/test_runner_beta.rs)
projects the existing
[`fixtures/runtime/python_task_discovery_alpha/ready_uv`](../../../runtime/python_task_discovery_alpha/ready_uv)
pytest workspace through the beta layer and asserts:

- the test-tree projection contains a workspace-root row, one row per
  pytest-compatible file, and one row per discovered test case;
- every case row, every inline row, and the rerun-last command quote the
  same canonical test-item identity;
- before any rerun-last launch is remembered, every parity row is in
  `rerun_lane_unset`;
- after `RerunLastLoop::remember_pytest` records the first attempt and
  `prepare_last_test` is called against a matching execution context, the
  parity rows for the remembered case transition to `rows_agree`;
- every artifact ref carried by the alpha attempt republishes as a
  structured `test_runner_beta_artifact_identity_record` with a closed
  `artifact_kind_token`;
- the support-export packet bundles the manifest, projection ids, parity
  rows, and alpha attempt-packet refs without leaking raw run output.
