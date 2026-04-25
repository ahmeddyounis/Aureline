# Container, devcontainer, mount-boundary, attach / exec, and multi-service topology fixtures

Worked-example fixtures for the container / devcontainer contract
frozen in
[`/docs/runtime/container_devcontainer_contract.md`](../../../docs/runtime/container_devcontainer_contract.md)
and the boundary schemas at
[`/schemas/runtime/container_session.schema.json`](../../../schemas/runtime/container_session.schema.json)
and
[`/schemas/runtime/devcontainer_profile.schema.json`](../../../schemas/runtime/devcontainer_profile.schema.json).

Every fixture carries only opaque engine / image / container / mount /
port / volume / secret-broker / preview-route / handoff / capture /
approval-ticket / policy-bundle / policy-epoch / actor / service /
feature / hook / compose-file handles plus monotonic placeholder
timestamps and redaction-aware labels. No raw container ids, no raw
image digests outside opaque image-handle classes, no raw repository /
registry URLs, no raw OCI references, no raw socket paths, no raw
absolute filesystem paths, no raw mount option bytes, no raw bind-
mount source paths, no raw OAuth / registry / SSH tokens, no raw TLS
material, no raw inspect bodies, no raw devcontainer.json bodies,
no raw Dockerfile bodies, no raw compose YAML bodies, no raw
lifecycle-hook command lines, no raw stdout / stderr, no raw error
strings, and no raw author identity strings appear in any fixture.

## Devcontainer-profile fixtures

| Fixture                                            | Profile class                                | Acceptance bullet covered                                                                                       |
|----------------------------------------------------|----------------------------------------------|-----------------------------------------------------------------------------------------------------------------|
| `profile_user_authored_local_devcontainer.json`    | `user_authored_local_profile`                | Default user-authored profile; non-root user; digest-pinned base image; workspace-folder mount.                 |
| `profile_managed_workspace_pinned.json`            | `managed_workspace_provisioned_profile`      | Managed-locked workspace trust; admin-pinned base image; managed egress envelope.                               |
| `profile_compose_with_services.json`               | `workspace_shared_committed_profile`         | Compose topology with three typed services (application, database, cache) and explicit dependency edges.       |
| `profile_ai_tool_proposed_pending_review.json`     | `ai_tool_proposed_profile_pending_review`    | AI-proposed profile forbidden from leaving pending review state; every chip surfaces user-review.               |

## Container-session fixtures

| Fixture                                                            | Session class                                                          | Acceptance bullet covered                                                                                                |
|--------------------------------------------------------------------|------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| `session_user_authored_dockerfile_local_workspace_trusted.json`    | `user_authored_dockerfile_session`                                     | Live local rootless session; mounts and writable paths explicit; full attach admissibility under workspace trust.        |
| `session_devcontainer_provisioned_local.json`                      | `devcontainer_provisioned_session`                                     | Provisioned through a devcontainer profile; workspace RW + named volume + secret-broker mount; loopback preview-route.   |
| `session_compose_orchestrated_multi_service.json`                  | `compose_orchestrated_container_session`                               | Six typed services with explicit dependency edges and per-service health states; no opaque "workspace started" collapse. |
| `session_managed_workspace_provisioned.json`                       | `managed_workspace_provisioned_container_session`                      | Managed runtime; managed-workspace tunneled forward; long-running exec gated to approval ticket.                         |
| `session_inspect_only_engine_socket_missing.json`                  | `inspect_only_uncertain_engine_session`                                | Engine socket missing → inspect-only; preflight surfaces typed engine_unreachable / engine_socket_missing triggers.      |
| `session_blocked_forbidden_socket_mount.json`                      | `blocked_engine_unreachable_session`                                   | Engine-socket mount triggers no-hidden-privileged-shell; every attach action blocked pending admin admit.                |
| `session_imported_from_container_handoff.json`                     | `imported_from_container_handoff_packet_session`                       | Pinned to captured handoff packet; never attaches to a live engine.                                                      |
| `session_replayed_from_container_capture.json`                     | `replayed_from_container_capture_session`                              | Pinned to container capture; every attach action `not_applicable_imported_or_replayed_session`.                          |
| `session_external_handoff_engine_remote_unreachable.json`          | `external_handoff_only_engine_remote_unreachable_session`              | Remote engine unreachable → external handoff path; no raw engine error surfaced.                                         |
| `session_ai_tool_proposed_pending_review.json`                     | `ai_tool_proposed_container_session_pending_review`                    | AI-proposed session forbidden from leaving pending review state.                                                         |
| `session_compose_dependency_cycle_blocked.json`                    | `blocked_engine_unreachable_session`                                   | Compose dependency cycle blocks start; every attach action blocked; multi-service topology surfaces typed cycle state.   |

## Cross-cutting acceptance coverage

- **Writable paths, mount boundaries, and privilege posture explicit
  before attach or exec actions.** Every live session enumerates a
  typed `mounts` list with `mount_kind_class` /
  `mount_writability_class`, a typed `privilege_posture_class`, and a
  typed `user_namespace_mode_class` before any attach action resolves
  to `admissible_under_*`. The blocked / inspect-only fixtures keep
  the same fields populated so a reviewer can read the mount model
  and privilege posture before the action ever fires.
- **No contract path implies ambient Docker-socket or privileged-
  shell escalation.** The
  `session_blocked_forbidden_socket_mount.json` fixture pairs a
  `forbidden_engine_socket_mount_pending_admin_admit` mount with
  `privilege_posture_class = privileged_socket_mount_blocked_pending_admin_admit`,
  resolves the session to `blocked_engine_unreachable_session`, and
  forces every attach action to `blocked_privileged_directive_pending_admin_admit`.
  No fixture admits an ambient socket mount or an ambient privileged
  shell.
- **Container / devcontainer sessions reuse the same execution-
  context, approval, and evidence vocabulary as terminal / task /
  debug paths.** Every fixture cites `execution_context_record_ref`
  and `environment_capsule_ref`, every approval-ticket gated entry
  cites the same `approval_ticket_ref` family integration / approval-
  ticket vocabulary uses, and every preview-route binding is cited by
  reference rather than redefined.
- **Start and attach flows can already report why a target is
  blocked, degraded, or read-only without dropping to raw engine
  errors.** Every blocked / inspect-only / external-handoff fixture
  cites a non-empty `preflight_results` list (where applicable) or a
  non-empty `downgrade_trigger_observations` list with typed values
  drawn from the closed downgrade-trigger vocabulary. No fixture
  collapses an engine error into a string.
- **Multi-service topology and dependency-order labels — API, queue,
  web app, and database do not collapse into one opaque "workspace
  started" state.** The
  `session_compose_orchestrated_multi_service.json` fixture
  enumerates six typed services with role / dependency edges / health
  state, and the `session_compose_dependency_cycle_blocked.json`
  fixture shows what a typed dependency cycle looks like rather than
  a generic "compose failed" string.
