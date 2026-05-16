# Beta launch-language runtime support

This document is the reviewer-facing landing page for the beta debugger
and execution-context support matrix per launch-language wedge. The
machine-readable boundary lives at
[`/schemas/runtime/support_matrix_beta.schema.json`](../../../schemas/runtime/support_matrix_beta.schema.json).
The runtime manifest lives at
[`/crates/aureline-runtime/src/support_matrix_beta/`](../../../crates/aureline-runtime/src/support_matrix_beta/);
the shell panel projection lives at
[`/crates/aureline-shell/src/support_matrix_beta/`](../../../crates/aureline-shell/src/support_matrix_beta/);
the publishable matrix lives at
[`/artifacts/compat/m3/debug_execution_matrix.md`](../../../artifacts/compat/m3/debug_execution_matrix.md).

The beta promise:

- one typed row per claimed launch-language wedge covering launch, attach,
  test, and execution-context support with **one** closed support-class
  vocabulary (`supported`, `preview`, `limited`, `unsupported`);
- missing capabilities surface as `preview` or `limited` — never as
  implied parity with the `supported` class;
- every row carries an explicit downgrade-rule set drawn from a closed
  vocabulary so support evidence cannot fork its own dialect;
- migration packets, partner intake packets, and release evidence packets
  quote the same runtime-emitted manifest the shell panel renders.

## Claimed wedges

| Wedge id | Reviewer label | Launch | Attach | Test | Execution-context rollup |
| --- | --- | --- | --- | --- | --- |
| `python` | Python service / data app | `supported` | `preview` | `supported` (claimed: `pytest`) | `preview` |
| `typescript_javascript` | TypeScript / JavaScript web app or service | `preview` | `preview` | `limited` (no claimed framework) | `limited` |

Held-for-later language wedges (Java / Kotlin, Rust workspace, Go service)
are explicitly excluded from this matrix and remain out of beta scope per
the alpha-scope register.

## Closed support-class vocabulary

| Class | Meaning |
| --- | --- |
| `supported` | Capability is exercised end-to-end by a claimed integration test and a fixture. Protected dispatch is allowed on a fresh, in-sync context. |
| `preview` | Capability is wired and inspectable but not exercised by claimed beta workflows. Rows render but protected dispatch requires review. |
| `limited` | Capability is narrowly available (inspection only, imported metadata only). Protected dispatch is blocked. |
| `unsupported` | Capability is explicitly out of beta scope. Protected dispatch is blocked. |

Adding a class is a vocabulary change that MUST update the canonical
manifest, the schema, the matrix, and the fixture set together.

## Execution-context lanes

Every wedge declares one class per canonical execution-context lane drawn
from the beta lane manifest (see
[`execution_context_beta.md`](execution_context_beta.md)):

- `local_host` — local-host terminal, task, test, debug, ai_tool_call.
- `remote_attach` — SSH / remote-kernel attach.
- `container` — devcontainer / container.
- `request_workspace` — managed workspace / prebuild runtime / AI sandbox.

The overall `execution_context.overall_class_token` is the canonical
rollup of the per-lane classes. The rollup demotes to `preview` or
`limited` when at least one lane is below `supported`, and to `limited`
when a `supported` lane is mixed with an `unsupported` lane.

## Downgrade-rule contract

Every row carries the canonical closed downgrade-rule set:

- `narrow_launch_on_adapter_capability_drop`
- `narrow_attach_to_inspect_only_on_capability_drop`
- `block_on_unclaimed_test_framework`
- `block_on_unclaimed_target_class`
- `block_protected_dispatch_on_ticket_drift`
- `block_protected_dispatch_on_capsule_drift`
- `block_protected_dispatch_on_trust_state_regression`
- `block_protected_dispatch_on_policy_epoch_regression`
- `block_on_target_unreachable`
- `block_on_adapter_negotiation_refused`

Rows MUST quote the exact tokens from the closed set; free-form downgrade
prose is forbidden so support evidence cannot disagree with the matrix.

## Support export

The
[`SupportMatrixBetaSupportExport`](../../../crates/aureline-runtime/src/support_matrix_beta/mod.rs)
packet bundles the canonical manifest and the resolved input fixtures the
caller wants to ship alongside it. Migration, partner intake, and release
evidence packets embed this record verbatim; raw env values, raw command
lines, and raw secrets are out of scope.

## Failure-drill fixtures

Reviewer input fixtures live under
[`/fixtures/runtime/m3/support_matrix_inputs/`](../../../fixtures/runtime/m3/support_matrix_inputs/):

- `python.json` — Python wedge with launch=`supported`, attach=`preview`,
  test=`supported` (claimed framework `pytest`), and the canonical
  per-lane classes for `local_host`, `container`, `remote_attach`, and
  `request_workspace`.
- `typescript_javascript.json` — TS/JS wedge with launch=`preview`,
  attach=`preview`, test=`limited` (no claimed framework; previewed
  frameworks `jest`, `vitest`, `node_test`), and the canonical per-lane
  classes.

The integration test that replays these fixtures lives at
[`/crates/aureline-runtime/tests/support_matrix_beta.rs`](../../../crates/aureline-runtime/tests/support_matrix_beta.rs).
Each fixture is compared byte-for-byte against the canonical row for the
same wedge; any new wedge, lane, support class, or downgrade rule must
land in every artefact in the same change set or the test fails.

The shell consumer test at
[`/crates/aureline-shell/src/support_matrix_beta/tests.rs`](../../../crates/aureline-shell/src/support_matrix_beta/tests.rs)
asserts the plaintext panel quotes the same closed tokens visible in the
runtime manifest and in the matrix markdown.

## How to verify

```
cargo test -p aureline-runtime --lib support_matrix_beta
cargo test -p aureline-runtime --test support_matrix_beta
cargo test -p aureline-shell --lib support_matrix_beta
```

## Out of scope for this beta

- M5 notebook-kernel launch and attach depth.
- Cross-workspace ticket import.
- Launch-language breadth outside the two claimed beta wedges.
- Full collaboration / multi-user productization of any column.

## Cross-references

- Publishable matrix —
  [`/artifacts/compat/m3/debug_execution_matrix.md`](../../../artifacts/compat/m3/debug_execution_matrix.md)
- Beta DAP host — [`debugger_host_beta.md`](debugger_host_beta.md)
- Beta execution-context resolver — [`execution_context_beta.md`](execution_context_beta.md)
- Beta run / debug profile model — [`run_debug_profiles_beta.md`](run_debug_profiles_beta.md)
- Beta test runner — [`test_runner_beta.md`](test_runner_beta.md)
- Beta env-inspect contract — [`env_inspect_beta.md`](env_inspect_beta.md)
- Beta task-event model — [`task_event_model_beta.md`](task_event_model_beta.md)
