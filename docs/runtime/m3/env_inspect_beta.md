# Beta env-inspect contract

This document is the reviewer-facing landing page for the env-inspect
contract: one canonical snapshot every claimed beta surface renders so
users and support can answer "why this target, runtime, toolchain, and
policy decision?" without forking the projection per surface.

The machine-readable boundary lives at
[`/schemas/execution/env_inspect.schema.json`](../../../schemas/execution/env_inspect.schema.json).
The canonical record lives in
[`/crates/aureline-runtime/src/env_inspect/`](../../../crates/aureline-runtime/src/env_inspect/);
the chrome panel projection that the UI inspector consumes lives in
[`/crates/aureline-shell/src/env_inspect/`](../../../crates/aureline-shell/src/env_inspect/);
the headless inspector binary lives at
[`/crates/aureline-shell/src/bin/aureline_shell_env_inspect.rs`](../../../crates/aureline-shell/src/bin/aureline_shell_env_inspect.rs).

The beta promise:

- the env-inspect snapshot is projected from one
  [`ExecutionContext`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
  shared by terminal, task, test, debug, AI-tool, and request-workspace
  surfaces; consumers never invent their own subsets or re-derive labels
  from raw env state;
- UI inspector chrome and the headless CLI binary emit the **same core
  fields, in the same render order, with the same labels and value
  tokens**, and the **same degradation labels with the same severity
  bands**;
- the support-export wrapper bundles one or more snapshots with a pinned
  `structured_tokens_only` redaction class — no raw env values, no raw
  command lines, no secrets, and no unmanaged credentials are emitted;
- seeded snapshots are deterministic: reviewer / partner runs of the
  headless inspector reproduce reviewer-fixture records byte-for-byte.

## Render-order sections

Every snapshot emits the following sections in fixed order. UI and CLI
consumers iterate the list verbatim.

| Section | Reviewer label | Truth source |
| --- | --- | --- |
| `lane` | Lane | Lane manifest + boundary-cue posture |
| `target` | Target | `target_identity` |
| `toolchain` | Toolchain | `toolchain_identity` |
| `environment_capsule` | Environment capsule | `environment_capsule_ref` |
| `policy_and_trust` | Policy & trust | `policy_and_trust` |
| `scope` | Workset scope | `workset_scope_class` |
| `cache` | Cache disposition | `cache_disposition` |
| `prebuild` | Prebuild | `prebuild_metadata` |
| `mixed_version` | Mixed-version posture | `mixed_version_drift` |
| `target_confidence` | Target confidence | `target_confidence` |
| `provenance` | Provenance | `provenance` |

## Degradation severity bands

Each canonical
[`DegradedFieldReason`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
maps to one of three severity bands:

| Severity | Meaning | Examples |
| --- | --- | --- |
| `notice` | Honesty marker; dispatch allowed | `toolchain_fallback`, `confidence_low`, `provenance_gap` |
| `warning` | Review required before dispatch | `trust_state_unresolved`, `capsule_drift_detected`, `activator_blocked_by_trust`, `policy_epoch_stale`, `remote_agent_scope_mismatch` |
| `blocking` | Dispatch must be re-authorised or blocked | `target_unreachable`, `capsule_unresolved`, `activator_unsupported_on_target`, `workset_member_unavailable` |

Adding a new degraded-field reason without mapping it to a severity is a
build-time regression: the unit test
[`every_degraded_field_reason_maps_to_a_severity`](../../../crates/aureline-runtime/src/env_inspect/mod.rs)
exercises every canonical variant.

## Support-export contract

The
[`EnvInspectSupportExport`](../../../crates/aureline-runtime/src/env_inspect/mod.rs)
packet bundles one or more snapshots and stamps the
[`EnvInspectRedactionClass::StructuredTokensOnly`](../../../crates/aureline-runtime/src/env_inspect/mod.rs)
class. Reviewer / support consumers MAY embed the packet verbatim. The
integration test
[`support_export_round_trips_and_pins_the_redaction_class`](../../../crates/aureline-runtime/tests/env_inspect_beta.rs)
asserts the packet never contains markers such as `LD_LIBRARY_PATH`,
`AWS_SECRET_ACCESS_KEY`, `SSH_PRIVATE_KEY`, or `BEARER` strings.

## Seeded scenarios

The headless inspector binary and the integration test replay the same
four seeded scenarios:

| Scenario token | Lane | Boundary cue | Review posture |
| --- | --- | --- | --- |
| `local_terminal` | `local_host` | hidden | none |
| `remote_attach_pending_trust` | `remote_attach` | visible | warning (pending trust) |
| `container_devcontainer` | `container` | visible | none |
| `managed_workspace_restricted` | `request_workspace` | visible | none (target-confidence narrows posture) |

## Failure-drill fixtures

Reviewer fixtures live under
[`/fixtures/runtime/env_inspect_beta/`](../../../fixtures/runtime/env_inspect_beta/):

- `local_terminal.json` — local-host terminal seed, no degradation;
- `remote_attach_pending_trust.json` — remote-attach task seed with
  pending trust, warning-severity degradation;
- `container_devcontainer.json` — devcontainer task seed, no degradation;
- `managed_workspace_restricted.json` — managed-workspace task seed with
  restricted trust narrowing the target-confidence reason set.

The integration tests live at
[`/crates/aureline-runtime/tests/env_inspect_beta.rs`](../../../crates/aureline-runtime/tests/env_inspect_beta.rs)
and
[`/crates/aureline-shell/tests/env_inspect_beta_fixtures.rs`](../../../crates/aureline-shell/tests/env_inspect_beta_fixtures.rs).
The shell-side test asserts that the chrome panel projection iterates the
canonical snapshot's core fields verbatim — no UI dialect, no re-derived
severity, no paraphrased labels.

## Out of scope for this beta

- Full activator-decision orchestration (env-manager shims, venv
  activation, devcontainer build, OCI image fetch) — the snapshot records
  the activation-strategy class only.
- M5 notebook-kernel inspection depth.
- Cross-workspace inspect import. Snapshots are workspace-scoped.

## How to verify

```
cargo test -p aureline-runtime env_inspect
cargo test -p aureline-runtime --test env_inspect_beta
cargo test -p aureline-shell env_inspect
cargo test -p aureline-shell --test env_inspect_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- scenarios
cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- snapshot remote_attach_pending_trust
cargo run -q -p aureline-shell --bin aureline_shell_env_inspect -- support-export
```

## Cross-references

- Beta execution-context resolver — [`execution_context_beta.md`](execution_context_beta.md)
- Beta task-event model — [`task_event_model_beta.md`](task_event_model_beta.md)
- Beta debugger / DAP host — [`debugger_host_beta.md`](debugger_host_beta.md)
- Beta run / debug profile model — [`run_debug_profiles_beta.md`](run_debug_profiles_beta.md)
