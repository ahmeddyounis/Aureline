# Container engine, preflight, port-lease, and log-channel contract

This document freezes the contract Aureline uses before a container or
devcontainer start, attach, exec, forward, or inspect-only flow begins.
It complements the broader
[`container_session_record`](../../schemas/runtime/container_session.schema.json)
and
[`devcontainer_profile_record`](../../schemas/runtime/devcontainer_profile.schema.json)
contract by splitting preflight truth, engine capability truth, port
leases, and log-channel attribution into records that support packets can
replay without a live engine.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or Design
System Style Guide, those source documents win and this document MUST be
updated in the same change.

## Companion artifacts

- [`/schemas/runtime/container_preflight_result.schema.json`](../../schemas/runtime/container_preflight_result.schema.json)
  - boundary schema for `container_preflight_result_record`.
- [`/schemas/runtime/port_lease.schema.json`](../../schemas/runtime/port_lease.schema.json)
  - boundary schema for `port_lease_record`.
- [`/fixtures/runtime/container_preflight_cases/`](../../fixtures/runtime/container_preflight_cases/)
  - YAML fixtures covering verified local Docker/devcontainer
    preflight, remote Podman inspect-only fallback, managed executor
    mirror/auth prerequisites, unsupported custom backend privilege
    blocking, port collision, and restart/rebind log-channel linkage.

Composes by reference, without restating payloads:

- [`/docs/runtime/container_devcontainer_contract.md`](./container_devcontainer_contract.md)
  - session/profile posture, mounts, attach admissibility, and
    multi-service topology.
- [`/schemas/runtime/preview_route.schema.json`](../../schemas/runtime/preview_route.schema.json)
  - route identity, mapped route handles, policy scope, expiry, and
    share posture for externally visible routes.
- [`/schemas/remote/forwarded_endpoint.schema.json`](../../schemas/remote/forwarded_endpoint.schema.json)
  - remote forwarded endpoint identity when the port lease crosses a
    remote attach/tunnel boundary.
- [`/schemas/execution/task_channel.schema.json`](../../schemas/execution/task_channel.schema.json)
  - log stream and evidence channel records; a port lease links to task
    channels rather than embedding raw log bodies.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support exports consume preflight and lease records as replayable
    evidence items rather than requiring a live engine.

## Why this exists

Container start and attach failures are otherwise easy to collapse into
raw engine errors. That hides whether the engine is absent, unsupported,
blocked by policy, missing a capability, unable to parse compose or
devcontainer input, blocked by an unsupported directive, colliding on a
port, waiting for credentials, or intentionally degraded to inspect-only
mode. It also lets a "workspace started" banner hide per-service logs,
port ownership, collision history, and restart state.

This contract makes those states explicit before execution begins:

- every engine is classified by support posture and capability flags;
- every parse, prerequisite, mount/path, directive, collision, and
  inspect-only fallback becomes a typed preflight finding;
- every claimed port exposure is represented by a lease tied to one
  service identity, route reference, collision state, restart state, and
  log/evidence channel set;
- helper containers, helper sockets, and elevated permissions must be
  declared as reviewable side effects or blocked as implicit authority.

## Engine classes and support posture

`container_engine_class` is the closed engine lane:

- `docker_local_engine` - Docker-compatible local engine.
- `docker_remote_daemon` - Docker-compatible remote daemon reached
  through an explicit remote context or connector boundary.
- `podman_local_engine` - local Podman-compatible engine, rootless or
  rooted according to its capability flags.
- `podman_remote_daemon` - Podman-compatible remote daemon reached
  through an explicit remote context or connector boundary.
- `managed_executor_runtime` - managed or self-hosted workspace runtime
  whose control plane owns provisioning and routing.
- `custom_engine_backend` - adapter-provided backend that must declare
  exactly which capabilities it implements.
- `unsupported_engine_backend` - detected backend that cannot safely
  start, attach, exec, or forward in-product.
- `engine_class_unknown_requires_review` - detected or imported engine
  identity that cannot be classified without review.

Every engine binding also emits `engine_support_posture_class`:

- `supported_full_contract`
- `supported_with_degraded_capabilities`
- `preview_supported_requires_review`
- `inspect_only_supported`
- `blocked_unsupported`
- `support_posture_unknown_requires_review`

The `engine_capability_flags` and `missing_capability_flags` arrays use a
closed vocabulary covering devcontainer parse, compose parse, image
build, container run, inspect, exec/attach, port forward, log stream,
volume mount, brokered secret mount, rootless mode, remote context,
managed routes, and custom adapter bridging. A start or attach flow MUST
read the capability flags before attempting work; missing capabilities
are surfaced as preflight findings, not as raw runtime errors.

## Preflight result model

A `container_preflight_result_record` answers one question: "What did
Aureline know before it tried to start, attach, exec, forward, or replay
container evidence?"

Each record carries:

- `subject_class` - which flow is being evaluated (`start_flow`,
  `attach_flow`, `exec_flow`, `forward_flow`, `inspect_only_flow`, or
  `support_replay_flow`);
- engine binding - class, support posture, reachability, build identity,
  endpoint ref, capability flags, missing capability flags, and a support
  note;
- source binding - refs to execution context, environment capsule,
  container session, devcontainer profile, imported handoff, support
  packet, and declarative inputs;
- parse state - devcontainer and compose parse outcomes with parser
  identity refs;
- findings - typed rows for engine reachability, missing capabilities,
  parse state, unsupported directives, mount/path risks, port
  collisions, credential prerequisites, mirror prerequisites, service
  readiness, policy/trust blockers, inspect-only fallback, and implicit
  sidecar or privilege violations;
- readiness summary - the final outcome class and action-admissibility
  posture the start/attach/exec/forward surface must honor;
- no-hidden-sidecar attestation - declarations or blockers for helper
  containers, helper sockets, and elevated permissions;
- support replay - whether the record can be replayed without a live
  engine, what evidence refs are present, and which fields were
  intentionally omitted or redacted.

Preflight findings use `finding_severity_class` and
`preflight_blocking_class` separately. A warning can be non-blocking; a
policy denial or unsupported engine can block start while still leaving
inspect-only metadata available.

## Port lease and log-channel linkage

A `port_lease_record` answers one question: "Which service owns this
claimed exposure, route, restart state, collision state, and log/evidence
channel set?"

Every lease carries:

- service binding - service identity class, service handle, optional
  label, container session ref, devcontainer profile ref, preflight ref,
  and execution context ref;
- exposure binding - exposure class, route refs, host/container port
  handles, bind-interface class, created/expiry/revocation refs, and
  route summary;
- collision binding - typed collision state and refs to colliding leases,
  external process handles, preflight findings, and repair hooks;
- restart binding - restart/rebind state, previous lease ref, restart
  attempt ref, and evidence refs;
- log-channel linkages - one or more task-channel/output-viewer/evidence
  refs tied to the same service handle and classified as service stdout,
  service stderr, engine events, compose logs, devcontainer lifecycle
  logs, port-forward proxy logs, health probes, route audit logs, or
  support snapshots;
- sidecar visibility - helper container/socket declarations or blocked
  implicit helper state.

A log channel MUST be attributable to a service handle. A lease MUST NOT
represent a whole workspace-start banner as the only log owner. Compose,
managed workspace, and devcontainer orchestration can aggregate views in
the UI, but the underlying record remains per service, per exposure.

## No-hidden-sidecar and no-ambient-privilege rules

The following rules apply to both schemas and all fixtures:

1. Helper containers, helper sockets, proxy processes, devcontainer
   feature helpers, and managed executor helpers are either declared
   through opaque helper refs or blocked as `implicit_*_detected_blocked`.
2. A detected helper that widens network, mount, socket, credential, or
   privilege scope MUST create a preflight finding before execution
   begins.
3. Engine sockets, host filesystem mounts, unrestricted host binds,
   full-privilege requests, disabled user namespaces, and custom backends
   without declared capabilities never become ambient authority.
4. Approval tickets can admit a declared elevated action only when the
   preflight record, session record, and lease record all cite the same
   ticket or policy ref family. They cannot convert an implicit helper
   into hidden ambient privilege.
5. Support replay records preserve the blocked/degraded outcome even
   when the live engine is no longer present.

## Support packet replay

Support exports MAY include `container_preflight_result_record` and
`port_lease_record` bodies directly or by stable refs. A replay reader
MUST be able to reconstruct:

- which engine class and capability flags were known;
- whether devcontainer and compose parsing succeeded, degraded, or
  failed;
- which unsupported directives, mount/path risks, port collisions,
  mirror/credential prerequisites, and policy/trust blockers existed;
- which action posture was admitted, degraded to inspect-only, handed
  off, or blocked;
- which service owned each lease and which log/evidence viewers were
  tied to it;
- which helpers and privilege-affecting side effects were declared or
  blocked.

Replay MUST NOT require a live Docker, Podman, remote daemon, custom
backend, or managed control plane. If live-only data was intentionally
omitted, the support replay block names the omission class.

## Compatibility and versioning

Both schemas use a const integer schema version. Additive optional
fields and additive enum values are additive-minor. Repurposing a field,
loosening the no-hidden-sidecar rule, or changing the semantics of an
existing action-admissibility, finding, exposure, collision, or restart
class is breaking and requires a new decision row.
