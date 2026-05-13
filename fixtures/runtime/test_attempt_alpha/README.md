# Test Attempt Alpha Fixtures

These fixtures exercise the runtime test-attempt alpha packet emitted by
`crates/aureline-runtime/src/tests`.

- `controlled_states_imported_ci.json` preserves imported CI as read-only
  evidence, links it to a fresh local rerun plan, and exports the controlled
  coverage, flaky, snapshot-review, AI-test-generation, watch, and stability
  states used by launch-wedge rows and support packets.

Verify with:

```sh
cargo test -p aureline-runtime test_attempt_alpha
```
