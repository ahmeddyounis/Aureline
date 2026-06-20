# Runtime-materialization proof

This report is the human-readable proof for the explicit runtime-instance
object and its parity engine. The canonical implementation is
[`crates/aureline-env/src/runtime_materialization/mod.rs`](../../crates/aureline-env/src/runtime_materialization/mod.rs);
the corpus and its expected parity outcomes are checked in under
[`fixtures/env/runtime-materialization/`](../../fixtures/env/runtime-materialization/)
and validated by `crates/aureline-env/tests/runtime_materialization.rs`.

## What the runtime instance is

A `RuntimeInstance` is the concrete place code ran, derived from a typed
environment capsule and carried in the same vocabulary as the capsule:

- a `ProcessNamespace` naming the namespace kind the processes ran in,
- a `mount_set` of working-tree, service-volume, and tool-cache mounts,
- a `port_map` of the service ports that published,
- a `readiness_graph` with one node per declared service, and
- a set of `secret_projections`, each carrying a handle and a state — never a
  secret value.

The instance is metadata-first: namespaces, mounts, ports, and secret
projections are ids, digests, handles, and review-safe prose, so raw pids, raw
paths, raw payloads, and secret values never cross the boundary.

## One engine, one object

`materialize_runtime` derives the declared target from the capsule, folds in
the observed instance, and returns one `RuntimeMaterialization` carrying the
parity, the per-facet evaluation, the per-service readiness, and a
`where_code_ran` line. Desktop (`desktop_runtime_materialization`), CLI /
headless (`headless_runtime_materialization`), AI
(`ai_runtime_materialization`), and support (`support_runtime_materialization`)
all read the **same** object, so a wrong-target run or a partial-service stack
downgrades visibly and identically on every surface — not behind a private
explainability format. `RuntimeParity::materialization_parity_state` maps the
parity back onto the governance materialization-parity evidence state, so the
runtime lane narrows the capsule's materialization-parity dimension in lockstep.

## Aligned corpus

| Target class | Materialization | Transport | Namespace | Parity | Target matched |
| --- | --- | --- | --- | --- | --- |
| `local` | `local_native` | `local_process` | `host_process` | `aligned` | yes |
| `ssh` | `remote_host` | `ssh` | `remote_host_session` | `aligned` | yes |
| `container` | `container` | `container` | `container_namespace` | `aligned` | yes |
| `devcontainer` | `devcontainer` | `container` | `container_namespace` | `aligned` | yes |
| `vm` | `remote_host` | `virtual_machine` | `vm_guest` | `aligned` | yes |
| `managed_workspace` | `managed_cloud` | `cloud_managed` | `managed_pod` | `aligned` | yes |

Every claimed target class derives a runtime instance that materializes its
capsule exactly as declared and is aligned on all six facets. Each keeps a
distinct target, materialization, transport, and namespace identity — none is
collapsed into a generic label.

## Degraded and mismatched scenarios

| Scenario | Target class | Injected | Parity | Headline facet | Named element |
| --- | --- | --- | --- | --- | --- |
| `runtime_container_partial_service` | `container` | backing service unready | `degraded` | `service_readiness` | `svc.db` |
| `runtime_ssh_degraded_mount` | `ssh` | service volume missing | `degraded` | `mount_set` | `mount.svc.db` |
| `runtime_vm_unpublished_port` | `vm` | primary port unpublished | `degraded` | `port_map` | `svc.app:8080` |
| `runtime_devcontainer_secret_pending` | `devcontainer` | secret projection pending | `degraded` | `secret_projection` | `APP_ENV` |
| `runtime_container_wrong_target` | `container` | ran on the local host | `mismatched` | `target_identity` | local vs container |
| `runtime_devcontainer_wrong_namespace` | `devcontainer` | ran in a host process | `mismatched` | `process_namespace` | host vs container namespace |

These prove the guardrails end-to-end:

- **Identity never collapses.** A container capsule that ran on the local host
  is `mismatched`, and `where_code_ran` says code ran on the local host while
  the capsule declared a container — not a generic "workspace started".
- **Wrong namespace is visible.** A devcontainer whose processes ran in a host
  process is `mismatched` on the process-namespace facet even though its target
  identity matched.
- **Partial is partial.** An unready backing service, a missing mount, an
  unpublished port, and a pending secret projection each degrade the parity and
  name the exact element rather than presenting the stack as fully up.
- **No secret values.** The pending-secret scenario degrades the runtime while
  the projection still carries only a handle, never the value.

## How to verify

```
cargo test -p aureline-env
cargo run -p aureline-env --example dump_runtime_materialization fixtures
```
