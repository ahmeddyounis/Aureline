# Rerun Exact-vs-Current Fixtures

These protected fixtures exercise the runtime rerun-last loop for task and
test lanes. The tests build real package-script and pytest contracts, remember
their exact execution context, then compare that prior target against a freshly
resolved current target before a rerun attempt is prepared.

| Fixture | Acceptance state |
|---|---|
| `last_task_current_target_drift.json` | rerun-last-task uses the current resolved target and requires review when the target changes from local to managed. |
| `last_test_exact_prior_target.json` | rerun-last-test keeps the exact prior target while still disclosing current SSH target drift before dispatch. |

Verify with:

```sh
cargo test -p aureline-runtime rerun
```
