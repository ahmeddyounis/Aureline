# Execution-context resolver alpha

The runtime crate now emits one exportable
[`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
object for terminal, task, test, debug-prep, and AI-tool entry points.
The alpha export keeps the original resolver-precedence record and adds
structured explanation rows for target confidence, policy/trust narrowing,
prebuild reuse, mixed-version helper posture, and the surfaces allowed to
reuse the same object.

## Contract

The canonical implementation lives in
[`crates/aureline-runtime/src/execution_context`](../../crates/aureline-runtime/src/execution_context).
The cross-tool export schema is
[`schemas/runtime/execution_context_alpha.schema.json`](../../schemas/runtime/execution_context_alpha.schema.json).

Every resolved context carries:

- `target_confidence` with a level plus reason tokens;
- `prebuild_metadata` with reuse/rejection posture and compatibility
  fingerprint;
- `mixed_version_drift` for helper-backed targets;
- `reusable_surfaces` covering terminal, task, test, debug, and AI-tool
  entry points;
- `explanations[]`, where each row names the field, effect class, reason
  code, source, resolved token, and applicable surfaces.

## Consumer

The shell execution-context inspector projects these fields through its
`Resolver explanations` section. It reads the same runtime object used by
terminal, task, and debug-prep seeds, so policy/trust and target-confidence
truth are not restated in a shell-only contract.

## Fixtures

Protected fixtures live under
[`fixtures/runtime/execution_context_alpha`](../../fixtures/runtime/execution_context_alpha):

- `reusable_launch_surfaces.json`
- `restricted_remote_policy_narrow.json`
- `prebuild_drift_rejected.json`

## Verify

```sh
cargo test -p aureline-runtime execution_context_alpha
cargo test -p aureline-shell runtime::context_inspector
```
