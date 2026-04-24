# Process topology, runtime host classes, and service ownership

This document freezes Aureline's multi-process runtime posture into a
concrete, named set of host classes, per-class attributes, and
service-ownership rows. It exists so shell-spike and core work cannot
drift toward a single-process convenience architecture while the rest
of the system is still being seeded.

The packet has three parts:

- this narrative, which names twelve host classes, their attributes,
  and the ownership rules;
- [`/artifacts/architecture/runtime_host_classes.yaml`](../../artifacts/architecture/runtime_host_classes.yaml),
  the machine-readable host-class vocabulary (failure isolation,
  restart class, state authority, allowed/forbidden capabilities, IPC
  families, visibility surfaces, invariants);
- [`/artifacts/architecture/service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml),
  the machine-readable ownership map (host class → service sub-plane,
  host class → M0 artifact family, diagnostic/export packet owner,
  inline-in-shell exceptions, deferred-host anchors).

## Companion artifacts

- [`/artifacts/architecture/process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
  — service sub-planes, seeded packages, protected paths, inline-work
  policy, and scheduling classes.
- [`/artifacts/architecture/plane_matrix.yaml`](../../artifacts/architecture/plane_matrix.yaml)
  — six primary logical planes, sub-plane rollup, and M0 artifact
  family placement.
- [`/artifacts/architecture/trust_boundaries.yaml`](../../artifacts/architecture/trust_boundaries.yaml)
  — trust-boundary matrix, allowed crossing forms, disclosure
  surfaces.
- [`/artifacts/architecture/protected_path_dependency_rules.yaml`](../../artifacts/architecture/protected_path_dependency_rules.yaml)
  — compile-time dependency classes and sentinel patterns.
- [`/docs/architecture/service_topology_and_process_placement.md`](service_topology_and_process_placement.md)
  — protected-path service plane map and inline-work policy.
- [`/docs/architecture/logical_planes_and_trust_boundaries.md`](logical_planes_and_trust_boundaries.md)
  — primary-plane narrative and cross-plane interaction rules.
- [`.t2/docs/Aureline_Technical_Architecture_Document.md`](../../.t2/docs/Aureline_Technical_Architecture_Document.md)
  sections 8.1–8.4 — source for the process topology this packet
  freezes.

## Why this exists

Three facts are already true in the repo:

1. The service-topology and process-placement map names ten service
   sub-planes, binds seeded crates to them, and forbids blocking I/O,
   network, or process-launch work on the interaction path.
2. The logical-plane map rolls those sub-planes into six primary
   planes and declares eight trust boundaries.
3. The technical architecture document lists twelve process/runtime
   roles with failure-isolation expectations in section 8.2.

What was missing was one place that answers three operational
questions at the same time:

- **Which host class runs a given service sub-plane?**
- **Which host class owns the runtime state for a given M0 artifact
  family?**
- **Where does the shell-spike intentionally deviate from the target
  topology, and what retires the deviation?**

This document and its two companion YAML files are that join point.
They do not invent host classes; they freeze the twelve roles the TAD
already names and bind them to the sub-plane, artifact-family, and
diagnostic-export vocabulary the rest of the repo uses.

## Host classes (frozen at twelve)

These are the twelve canonical runtime host classes. Ids and titles
match the canonical process-role matrix in
[`/artifacts/milestones/M0_architecture_pack/canonical_matrices.yaml`](../../artifacts/milestones/M0_architecture_pack/canonical_matrices.yaml).
Adding a host class is an ADR-gated change that must update this
document, `runtime_host_classes.yaml`, `process_placement_map.yaml`,
and the canonical-matrices entry.

| Host class id | Title | Owning sub-planes | Failure isolation | Restart class | State authority |
| --- | --- | --- | --- | --- | --- |
| `desktop_shell` | Desktop shell process | `shell_ui`, `renderer`, `text_buffer` | `shell_critical` | `never_auto_restart` | `shell_owned_authoritative` |
| `local_supervisor` | Local supervisor | `remote_helper` (routing) | `supervisor_lifecycle` | `restart_children_on_exit` | `supervisor_lifecycle_truth` |
| `index_workers` | Index workers | `vfs_watchers`, `index_search` | `worker_pool_cancellable` | `restart_with_backoff` | `worker_owned_authoritative_knowledge` |
| `language_hosts` | Language hosts | `task_execution` (LSP slice) | `per_session_helper` | `quarantine_after_budget` | `per_session_ephemeral` |
| `debug_hosts` | Debug hosts | `task_execution` (DAP slice) | `per_session_helper` | `per_session_restart` | `per_session_ephemeral` |
| `git_worker` | Git worker | `task_execution` (Git slice) | `per_session_helper` | `per_session_restart` | `per_session_ephemeral` |
| `pty_host` | PTY host | `task_execution` (terminal slice) | `per_session_helper` | `per_session_restart` | `per_session_ephemeral` |
| `wasm_host_pool` | Wasm host pool | `ai_control_plane` (sandboxed runtimes) | `per_extension_sandbox` | `quarantine_after_budget` | `sandbox_ephemeral` |
| `external_extension_hosts` | External extension hosts | `ai_control_plane` (external processes) | `per_external_extension_process` | `quarantine_after_budget` | `external_process_owned` |
| `ai_runtime` | AI runtime / broker | `ai_control_plane` (routing/broker) | `managed_boundary_or_helper` | `restart_with_backoff` | `provider_owned_remote` |
| `remote_connector` | Remote connector | `remote_helper` (transport) | `managed_boundary_or_helper` | `restart_with_backoff` | `provider_owned_remote` |
| `observability_collector` | Observability collector | `support_diagnostics` | `collector_bounded` | `restart_with_backoff` | `collector_owned_bounded` |

Per-class allowed capabilities, forbidden capabilities, IPC families,
and visibility surfaces are authoritative in
[`runtime_host_classes.yaml`](../../artifacts/architecture/runtime_host_classes.yaml).

### Host-class attribute axes

Every host-class row declares the following axes. The closed
vocabularies live in `runtime_host_classes.yaml`.

- **Failure isolation** — the scope over which a crash is contained.
  `shell_critical` is singular and user-visible; `per_session_helper`
  is one session's worth; `collector_bounded` is non-blocking and
  buffered.
- **Allowed capabilities** — the named capability classes the host
  may exercise (for example `language_analysis`, `pty_terminal_session`,
  `ai_routing_and_context`). Every other capability is forbidden.
  Sentinels in the
  [protected-path dependency rules](../../artifacts/architecture/protected_path_dependency_rules.yaml)
  enforce the forbidden set on monitored modules.
- **Restart class** — `never_auto_restart` (desktop shell),
  `restart_children_on_exit` (supervisor), `restart_with_backoff`
  (workers, managed helpers), `per_session_restart` (session helpers),
  `quarantine_after_budget` (language, Wasm, external extension
  hosts), `manual_only`.
- **State authority** — who owns the state the host carries.
  `shell_owned_authoritative` is the shell; `worker_owned_authoritative_knowledge`
  covers canonical-path identity, watcher truth, and index shards;
  `per_session_ephemeral` is bounded to one task/debug/terminal/Git
  session; `collector_owned_bounded` is the support-export lane.
- **IPC families** — the declared families the host participates in:
  `typed_binary_rpc`, `jsonrpc_compat`, `event_stream`,
  `command_graph`, `pty_stdio`, `sandbox_capability_call`,
  `external_extension_channel`, `transport_governance_egress`,
  `remote_proxy_bridge`, `service_api`, `audit_event_stream`.
- **Visibility surfaces** — the disclosure surfaces the host must
  keep visible to the user or the support surface, drawn from the
  closed set shared with
  [`plane_matrix.yaml`](../../artifacts/architecture/plane_matrix.yaml)
  and
  [`trust_boundaries.yaml`](../../artifacts/architecture/trust_boundaries.yaml).

### Process-topology picture

The logical topology matches TAD §8.1 and stays frozen at this shape:

```
                         ┌──────────────────────────┐
                         │   desktop_shell          │
                         │   shell_ui, renderer,    │
                         │   text_buffer            │
                         └──────────────┬───────────┘
                                        │ typed_binary_rpc, event_stream
                                        ▼
                         ┌──────────────────────────┐
                         │   local_supervisor       │
                         │   (typed RPC routing,    │
                         │    lifecycle, health)    │
                         └──────────────┬───────────┘
                                        │
      ┌────────────┬─────────┬──────────┼───────────┬──────────┬─────────┐
      ▼            ▼         ▼          ▼           ▼          ▼         ▼
 index_workers language_hosts debug_hosts git_worker pty_host  wasm_host_pool  external_extension_hosts
 (vfs_watchers, (LSP)         (DAP)       (Git)       (PTY)     (sandbox)        (external
  index_search)                                                                   extensions)
                                        │
                             ┌──────────┼───────────┐
                             ▼          ▼           ▼
                       ai_runtime  remote_connector  observability_collector
                       (AI routing  (transport)      (traces, metrics,
                        and broker)                   support bundles)
```

The shell talks to the supervisor; the supervisor routes typed RPC to
worker and helper host classes; the observability collector receives
typed audit/event streams from every host.

## Shell-owned vs worker-owned state

The packet distinguishes two state kinds that MUST NOT be conflated.

**Shell-owned authoritative state** lives in `desktop_shell` and
covers:

- input, focus, accessibility-tree state;
- visible view models, layout, caret/selection;
- command routing and in-flight command dispatch state;
- local undo/redo view state.

No other host class may mutate this state through shared memory,
globals, or side channels. Visible-state changes arrive through typed
RPC responses or subscription envelopes and are applied by the shell.

**Worker-owned state** is everything else and splits into three
sub-kinds:

- `worker_owned_authoritative_knowledge` on `index_workers` —
  canonical-path identity, save pipeline truth, watcher state, and
  index/graph shards. The shell reads this as authoritative *knowledge*
  under a declared freshness lane; it is still disposable relative to
  disk truth.
- `per_session_ephemeral` on session helpers (`language_hosts`,
  `debug_hosts`, `git_worker`, `pty_host`) — bounded to one
  user-visible session; destroyed with the session.
- `sandbox_ephemeral`, `external_process_owned`, and
  `provider_owned_remote` on extensibility, AI, and remote hosts —
  opaque or external state that only reaches the shell through typed
  contracts.

`collector_owned_bounded` is a distinct lane: the
`observability_collector` owns bounded, redacted diagnostic data; no
other host class may assemble a support or export packet.

## Service ownership at runtime

[`service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml)
records three ownership relations:

1. **Host-class ownership** — for each of the twelve host classes,
   which service sub-planes it owns, which host supervises it, its
   authoritative-state kind, and its diagnostic-export chain.
2. **Service sub-plane ownership** — for each of the ten service
   sub-planes in `process_placement_map.yaml`, the primary host class
   plus any declared co-host host classes.
3. **M0 artifact family ownership** — for each of the sixteen M0
   artifact families in `plane_matrix.yaml`, the primary host class
   plus any declared co-host host classes.

Summary of sub-plane ownership:

| Sub-plane | Primary host class | Co-host host classes |
| --- | --- | --- |
| `shell_ui` | `desktop_shell` | — |
| `renderer` | `desktop_shell` | — |
| `text_buffer` | `desktop_shell` | `index_workers` |
| `vfs_watchers` | `index_workers` | — |
| `index_search` | `index_workers` | — |
| `task_execution` | `language_hosts` | `debug_hosts`, `git_worker`, `pty_host` |
| `remote_helper` | `remote_connector` | `local_supervisor` |
| `ai_control_plane` | `ai_runtime` | `wasm_host_pool`, `external_extension_hosts` |
| `updater_release` | deferred (anchor: `updater_installer_helper_not_yet_a_frozen_host_class`) | `local_supervisor` |
| `support_diagnostics` | `observability_collector` | — |

### Diagnostic and export packet ownership

There is exactly one packet owner. `observability_collector` assembles
every support and export packet. Every other host class emits typed
events through one of `audit_event_stream`, `event_stream`, or
`typed_binary_rpc` into the collector.

Forbidden practices:

- assembling a support or export packet anywhere other than the
  collector;
- emitting diagnostic data through shared memory, globals, or
  file-drop side channels instead of the declared IPC families;
- blocking a protected path while waiting on telemetry acknowledgement;
- exporting a packet without passing redaction policy and
  workspace-trust admission.

### Inline-in-shell exceptions

The shell-spike MAY, for a bounded time, run a slice of a sub-plane
inline in the desktop shell process in place of its target host class.
Every such deviation MUST be an explicit row in
`service_ownership_matrix.yaml` under `inline_in_shell_exceptions`
naming:

- the sub-plane being run inline;
- the crate that contains the inline code;
- the **target host class** the runtime state will move to;
- the narrow inline scope that is allowed today;
- the inline scopes that remain forbidden even during the spike (for
  example, blocking filesystem I/O or subprocess launch);
- the migration trigger (milestone or task) that retires the row.

At the time this packet is frozen, **no sub-plane is deliberately
running inline in the shell in place of its target host class**. The
list is empty and every sub-plane has either a primary host class or
a deferred-host anchor. The schema exists so future shell-spike work
can declare exceptions explicitly rather than silently.

### Deferred host classes

Some sub-planes do not yet have a frozen host class. Each such row in
the matrix carries a `deferred_host_class_anchor` explaining the
target shape and whether inline-in-shell operation is permitted in
the interim. The only anchor today is
`updater_installer_helper_not_yet_a_frozen_host_class`, which covers
`updater_release`: the target is a supervisor-spawned
updater/installer helper, and inline-in-shell operation is explicitly
**not** permitted in the interim — the sub-plane stays dormant until
the helper host class lands.

## Reuse by supervision, support, and mixed-version work

The host-class and ownership vocabulary composes with downstream
packets so they do not reinvent boundary language:

- **Supervision work** — cites `host_classes[*].failure_isolation`,
  `.restart_class`, and the supervisor's `supervises_host_classes`
  list for restart budgets, crash-loop quarantine, and visible
  degradation. Restart budgets, quarantine lanes, and kill-switch
  flows attach to host-class ids, not ad hoc process names.
- **Support and export work** — cites
  `diagnostic_export_ownership.packet_owner_host_class` and the
  per-host `diagnostic_export_chain.support_bundle_role` field for
  which host contributes which slice to a support bundle. Object
  handoff packets (extension runtime quarantine, language-server
  session trace, PTY scrollback handoff) name the emitting host
  class by id.
- **Mixed-version / compatibility work** — cites
  `host_classes[*].ipc_families` to decide which transport, envelope,
  and schema-toolchain version gates apply on a given hop. The remote
  proxy, JSON-RPC compat, and sandbox-capability-call families remain
  distinct lanes with distinct compatibility rules.
- **Policy and audit work** — cites
  `diagnostic_export_ownership.allowed_emitter_ipc_families` and the
  per-host `allowed_capabilities` / `forbidden_capabilities` fields
  to decide whether a new capability row can attach to an existing
  host or needs its own class (ADR-gated).

## Acceptance cross-check

The packet satisfies the acceptance bar in the following ways:

- **Every M0 or M1-seeded subsystem has a declared runtime host
  class or an explicit exception.** Every sub-plane in
  `process_placement_map.yaml` appears in
  `service_sub_plane_ownership`, and every M0 artifact family in
  `plane_matrix.yaml` appears in `m0_artifact_family_ownership`.
  Rows that lack a primary host class (`updater_release` and
  `release_build_identity_and_provenance`) declare a
  `deferred_host_class_anchor` with a migration trigger and an
  explicit `inline_in_shell_permitted_interim: false`.
- **The packet distinguishes shell-owned authoritative state from
  worker-owned disposable state.**
  `runtime_host_classes.yaml` defines the closed
  `state_authority_classes` vocabulary and names each host's class;
  the invariant
  `shell_owned_state_is_distinct_from_worker_owned_disposable_state`
  is recorded in `service_ownership_matrix.yaml`.
- **The vocabulary is reusable by supervision, support, and
  mixed-version tasks.** Host-class ids, failure-isolation classes,
  restart classes, state-authority classes, IPC families,
  capability classes, and visibility surfaces are all closed
  vocabularies defined in one of the two YAML files, referenced by
  id from this document and by the downstream packets listed above.

## Changing this packet

Adding or renaming a host class, a failure-isolation class, a
restart class, a state-authority class, an IPC family, or a
capability class is an ADR-gated change. Adding an
inline-in-shell exception is not ADR-gated but MUST name a migration
trigger and MUST NOT widen the forbidden-scope list; removing an
exception is ADR-gated because downstream tooling may already cite
it. Per-sub-plane and per-artifact-family ownership rows evolve with
the underlying placement files; this packet must be updated in the
same change when they move.
