# Execution-context alpha fixtures

These fixtures exercise the canonical runtime resolver object used by
terminal, task, test, debug-prep, and AI-tool entry points.

The cases focus on structured resolver explanations rather than raw debug
text:

- `reusable_launch_surfaces.json` proves one resolved object advertises the
  reusable launch-capable surfaces and exports target confidence, policy, and
  prebuild posture.
- `restricted_remote_policy_narrow.json` proves trust and policy narrowing
  remain visible when the target is helper-backed.
- `prebuild_drift_rejected.json` proves stale capsule/prebuild metadata
  downgrades to a rejected cache state with an honesty marker.

Verify with:

```sh
cargo test -p aureline-runtime execution_context_alpha
cargo test -p aureline-shell runtime::context_inspector
```
