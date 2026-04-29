# Container preflight and port-lease fixtures

Worked-example fixtures for the container engine, devcontainer
preflight, port lease, and log-channel contract frozen in
[`/docs/runtime/container_engine_and_preflight_contract.md`](../../../docs/runtime/container_engine_and_preflight_contract.md).

The fixtures use the boundary schemas at
[`/schemas/runtime/container_preflight_result.schema.json`](../../../schemas/runtime/container_preflight_result.schema.json)
and
[`/schemas/runtime/port_lease.schema.json`](../../../schemas/runtime/port_lease.schema.json).

Every fixture carries only opaque engine, endpoint, parser, input,
directive, mount, port, lease, service, route, forwarded-endpoint,
task-channel, viewer, evidence, support, helper, credential, mirror,
policy, approval-ticket, and repair refs plus redaction-aware labels. No
raw socket paths, raw context names, raw URLs, raw hostnames, raw IPs,
raw integer ports, raw absolute filesystem paths, raw mount option
bytes, raw compose/devcontainer bodies, raw command lines, raw log
bodies, raw process ids, raw credentials, or raw author identity strings
appear in any fixture.

| Fixture | Schema | Coverage |
|---|---|---|
| `preflight_local_docker_devcontainer_ready.yaml` | `container_preflight_result_record` | Verified local Docker/devcontainer parse, full capability set, no port collisions, no implicit helpers, replayable support packet. |
| `preflight_remote_podman_inspect_only.yaml` | `container_preflight_result_record` | Remote Podman endpoint unreachable, degraded inspect-only fallback, support packet can replay without the daemon. |
| `preflight_managed_executor_mirror_auth.yaml` | `container_preflight_result_record` | Managed executor reachable but blocked on mirror and registry credentials before start. |
| `preflight_custom_backend_privilege_blocked.yaml` | `container_preflight_result_record` | Custom/unsupported backend with undeclared adapter capability, forbidden privilege request, and implicit helper blocker. |
| `port_lease_compose_web_collision.yaml` | `port_lease_record` | Web service lease collision tied to service identity, preflight finding, repair hook, route refs, and log/evidence refs. |
| `port_lease_managed_route_restart_rebound.yaml` | `port_lease_record` | Managed route lease after service restart, same service attribution, restart evidence, and multiple log-channel linkages. |

Acceptance coverage:

- Start/attach preflight outcomes never hide engine reachability,
  missing capabilities, unsupported directives, mirror/auth
  prerequisites, mount/path risks, or port collisions behind raw engine
  errors.
- Port leases and log channels remain attributable to service identity
  and route refs rather than one workspace-start banner.
- Support replay blocks preserve enough evidence refs and omission
  classes to reproduce findings without a live Docker, Podman, custom
  backend, or managed control plane.
- Helper containers, helper sockets, and elevated permissions are
  declared or blocked; no fixture admits hidden sidecars or ambient
  privilege.
