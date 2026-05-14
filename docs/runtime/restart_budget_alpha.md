# Alpha Runtime Restart Budgets

This document publishes the alpha runtime restart-budget skeleton for
fault-domain consumers. The machine-readable source of truth is
[`/artifacts/runtime/fault_domain_taxonomy_alpha.yaml`](../../artifacts/runtime/fault_domain_taxonomy_alpha.yaml),
paired with
[`/artifacts/runtime/supervisor_health_events_alpha.yaml`](../../artifacts/runtime/supervisor_health_events_alpha.yaml).

The older restart-budget packet remains the architectural source for domain
budgets and state vocabulary:
[`/artifacts/runtime/restart_budgets.yaml`](../../artifacts/runtime/restart_budgets.yaml).
This alpha packet binds that vocabulary to the runtime lanes that UI state
copy, support bundles, and incident packets need to reference.

## Scope

The alpha taxonomy covers the supervised runtime lanes named in the packet:

| Lane | Host class | Fault domain | Strike budget | Fail-closed state |
| --- | --- | --- | --- | --- |
| Desktop shell | `desktop_shell` | `shell_interaction_core` | 0 restarts / 5 s | `lost` |
| Workspace knowledge | `index_workers` | `workspace_knowledge_workers` | 3 restarts / 30 s | `quarantined` |
| Language service | `language_hosts` | `session_scoped_execution_hosts` | 3 restarts / 30 s | `quarantined` |
| Debug adapter | `debug_hosts` | `session_scoped_execution_hosts` | 3 restarts / 30 s | `quarantined` |
| Git worker | `git_worker` | `session_scoped_execution_hosts` | 3 restarts / 30 s | `quarantined` |
| Terminal PTY | `pty_host` | `session_scoped_execution_hosts` | 3 restarts / 30 s | `quarantined` |
| Wasm tool host | `wasm_host_pool` | `extension_and_tool_hosts` | 3 restarts / 120 s | `disabled` |
| External extension | `external_extension_hosts` | `extension_and_tool_hosts` | 3 restarts / 120 s | `disabled` |
| AI runtime | `ai_runtime` | `ai_and_external_tool_brokers` | 4 restarts / 120 s | `quarantined` |
| Remote connector | `remote_connector` | `remote_connectors` | 4 restarts / 120 s | `offline` |
| Local supervisor | `local_supervisor` | `policy_entitlement_and_verifier_helpers` | 2 restarts / 30 s | `degraded` |
| Observability collector | `observability_collector` | `policy_entitlement_and_verifier_helpers` | 2 restarts / 30 s | `degraded` |

## Supervisor Health Events

Supervisor events are intentionally small and export-safe. Every event carries
the lane id, fault-domain id, host-class id, state transition, strike count,
restart-budget ref, support packet ref, incident packet ref, and forensic
packet ref when one exists.

The closed transition classes are:

| Transition | Event class | Required meaning |
| --- | --- | --- |
| `start` | `supervisor_health_started` | supervision began; the lane is `warming` or `ready` |
| `degrade` | `supervisor_health_degraded` | health narrowed but protected local work continues where applicable |
| `restart` | `supervisor_health_restart_scheduled` | an automatic restart remains inside budget |
| `quarantine` | `supervisor_health_quarantined` | budget or policy moved the lane into an explicit fail-closed state |
| `recover` | `supervisor_health_recovered` | the lane returned to `ready` after verified recovery or explicit resume |

Outside the declared budget, automatic restart is not allowed. The event stream
must project `quarantined`, `disabled`, `offline`, or `lost` and include the
repair or resume path.

## UI State Copy Projection

State copy consumes the same packet fields as support and incident flows. A
runtime lane card or banner should be able to render:

- affected lane and fault domain;
- current state and prior state;
- strike count and budget window;
- what still works;
- what is paused or reduced;
- next safe action;
- support or incident packet ref for evidence review.

### Desktop Shell Lane

The shell lane has no silent restart path. A shell crash records `lost`; the
next recovery path is relaunch with crash evidence and durable-state restore
review.

### Workspace Knowledge Lane

The knowledge lane may degrade to partial search or graph freshness while
editing, save, and local navigation continue. Budget exhaustion quarantines
the lane and routes mutating recovery through the cache/index repair preview.

### Language Service Lane

Language-service failure is scoped by workspace, root, and language. A
quarantined language service must not imply other sessions are unhealthy.

### Debug Adapter Lane

Debug adapter recovery never silently reattaches or reruns debug work.
Replay requires a fresh reviewed action.

### Git Worker Lane

Git worker failure preserves inspectable repository state but does not replay
mutating Git operations.

### Terminal PTY Lane

Terminal failure can preserve transcript evidence, but commands are not rerun
by restore or restart.

### Wasm Tool Lane

Wasm tool host recovery re-admits permissions from scratch. Prior capability
manifests are not reused after restart.

### External Extension Lane

Extension failure degrades or disables by package. Reviewed extension isolation
is the handoff path when local repair is safe.

### AI Runtime Lane

AI runtime degradation narrows provider and tool capabilities without blocking
the local editing path.

### Remote Connector Lane

Remote connector failure projects `offline` while local-safe work remains
available. Reconnect starts from `warming` with fresh transport authority.

### Local Supervisor Lane

Supervisor degradation prevents child lanes from claiming more health than
the current evidence proves.

### Observability Collector Lane

Collector failure degrades support and telemetry assembly. Protected emitters
must not block on collector availability.

## Repair Handoff

Mutating recovery does not live in this packet. When a lane names a repair
transaction, that ref is owned by
[`/schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
and the reviewed flow described in
[`/docs/support/repair_preview_alpha.md`](../support/repair_preview_alpha.md).

The runtime packet only records which repair transaction is safe to offer for
the lane. It never implies that repair may run without preview, checkpoint
truth, reversal-class disclosure, and support/export lineage.

## Protected Drills

Protected drill fixtures live under
[`/fixtures/runtime/fault_domain_drills_alpha/`](../../fixtures/runtime/fault_domain_drills_alpha/).
They cover:

- knowledge-worker budget exhaustion into quarantine;
- extension host fail-closed disablement and reviewed recovery;
- remote connector offline degradation, bounded restart, and recovery.

The support consumer validates these fixtures with:

```bash
cargo test -p aureline-support --test runtime_fault_domains_alpha
```
