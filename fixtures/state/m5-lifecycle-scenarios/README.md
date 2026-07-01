# M5 lifecycle scenario fixtures

Narrowed variants of the frozen M5 lifecycle-state and journey-checkpoint matrix
(task M05-732). Each fixture proves that a downgrade **narrows** an object's
claim rather than hiding the object.

- `remote_session_degraded_narrowed.json` — the remote session narrows to Beta
  after a degraded finding; the object stays present with its full state binding.
- `notebook_runtime_retest_narrowed.json` — the notebook runtime narrows to
  Preview after a retest-pending finding; the object stays present.

These files are generated from the seed builders and must match them
bit-for-bit. Regenerate with the headless emitter (see
`docs/lifecycle/m5_lifecycle_matrix_contract.md`):

```sh
BIN=./target/debug/aureline_shell_m5_lifecycle_matrix
$BIN fixture-remote-session-degraded-narrowed  > fixtures/state/m5-lifecycle-scenarios/remote_session_degraded_narrowed.json
$BIN fixture-notebook-runtime-retest-narrowed  > fixtures/state/m5-lifecycle-scenarios/notebook_runtime_retest_narrowed.json
```
