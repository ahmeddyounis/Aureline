# Container, devcontainer, mount-boundary, attach / exec, and multi-service topology contract

This document freezes the cross-surface contract Aureline uses to
represent **container and devcontainer execution**, **mount and
writable-path boundaries**, **attach / exec / forward posture**, and
**multi-service topology** before container runtimes or devcontainer
orchestrators land. The goal is simple: a container-aware surface
(terminal attach, task / test / debug runner bound to a container,
devcontainer "reopen in container" lane, multi-service compose
orchestrator, support / export reader, hosted-review reader,
automation run review, AI tool review surface) MUST cite **one**
container-session record and (when the session was provisioned
through a devcontainer profile) **one** devcontainer-profile record,
instead of inventing per-surface labels for engine identity, mount
admissibility, attach posture, dependency state, or preflight reason.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document MUST be updated in the same change.

## Companion artifacts

- [`/schemas/runtime/container_session.schema.json`](../../schemas/runtime/container_session.schema.json)
  — boundary schema for the `container_session_record`.
- [`/schemas/runtime/devcontainer_profile.schema.json`](../../schemas/runtime/devcontainer_profile.schema.json)
  — boundary schema for the `devcontainer_profile_record`.
- [`/fixtures/runtime/container_cases/`](../../fixtures/runtime/container_cases/)
  — concrete container-session and devcontainer-profile fixtures
  covering local Docker, rootless Podman, devcontainer reopen, compose
  orchestrated topology, managed-workspace provisioned runtime,
  imported handoff, replayed capture, inspect-only uncertain engine,
  external handoff (engine remote unreachable), blocked engine
  unreachable, and AI-tool-proposed pending review cases.

Composes by reference (no payload restated):

- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  and [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — execution-context record. The session carries
  `execution_context_record_ref` and never restates target identity,
  sandbox posture, or policy epoch. The session record narrows
  `target_class` to the five values that resolve through a container
  engine (`container_local`, `devcontainer`, `managed_workspace`,
  `remote_workspace_vm`, `prebuild_runtime`); `local_host`,
  `ssh_remote`, `notebook_kernel_*`, and `ai_sandbox` are never
  admissible because they do not resolve through a container engine.
- [`/schemas/runtime/environment_capsule.schema.json`](../../schemas/runtime/environment_capsule.schema.json)
  and [`/docs/runtime/environment_capsule_contract.md`](./environment_capsule_contract.md)
  — environment-capsule record. The session and the profile resolve
  toolchain identity, secret-projection markers, and export path
  through the capsule. They never restate those fields.
- [`/schemas/runtime/preview_route.schema.json`](../../schemas/runtime/preview_route.schema.json)
  and [`/docs/runtime/browser_runtime_contract.md`](./browser_runtime_contract.md)
  — preview-route record. Each port-forward entry MAY cite a
  `preview_route_record_ref` so the route record stays the source of
  truth for service identity, mapped port / route, target context,
  actor, expiry, and policy scope.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — secret-broker handle and raw-secret-forbidden boundary. Every
  `secret_or_config_mount_via_broker_handle` mount cites a non-null
  `secret_broker_handle_ref`.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — approval-ticket envelope referenced whenever an attach action
  resolves to `admissible_under_approval_ticket`.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state vocabulary the session and profile re-export.

If this document disagrees with those sources, those sources win and
this contract plus the companion schemas update in the same change.

## Why freeze this now

Container and devcontainer surfaces are easy places for hidden truth
to leak. A "running" workspace can hide that the engine socket is
missing and silently fall back to a stale local toolchain. An attach
button can quietly resolve to a privileged shell. A bind mount can
expose `/`, `/var/run/docker.sock`, or `~` without an admin admit.
A port forward can quietly bind on `0.0.0.0` and expose the service
on the local network. A devcontainer "reopen in container" can launch
a community feature whose post-create command runs an unscoped shell.
A compose topology can surface "all services started" while the
queue and database actually failed to start in the right order.

Without one contract:

- a session can claim live attach when the engine socket is missing
  or permission-denied;
- an attach button can launch a privileged shell without an admin
  admit;
- a bind mount can mount the host engine socket inside the container
  and re-execute as `root` on the host;
- a port forward can expose the service on `0.0.0.0` without warning;
- a devcontainer profile can declare a community feature whose post-
  create command runs an unscoped shell;
- a compose orchestrator can collapse "API started", "queue started",
  and "database started" into one opaque "workspace started" state;
- a captured handoff can re-attach to a live engine and replay
  destructive actions against a freshened target.

This contract closes those gaps by freezing one
`container_session_record`, one `devcontainer_profile_record`, one
mount-kind ladder, one attach-admissibility ladder, one multi-service
dependency-state vocabulary, one preflight-result vocabulary, and one
denial-reason vocabulary every container-aware surface MUST project
the same way.

## Scope

Frozen at this revision:

- one shared `container_session_record` shape covering user-authored
  Dockerfile sessions, compose-orchestrated sessions, devcontainer-
  provisioned sessions, managed-workspace-provisioned sessions,
  imported-from-handoff sessions, replayed-from-capture sessions,
  inspect-only-uncertain-engine sessions, external-handoff-only-engine-
  remote-unreachable sessions, blocked-engine-unreachable sessions,
  and AI-tool-proposed-pending-review sessions;
- the engine-identity / engine-reachability / privilege-posture / user-
  namespace-mode / mount-kind / mount-writability / port-forward /
  attach-posture / attach-admissibility / multi-service-dependency-
  state / service-role / preflight-result / preflight-blocking /
  downgrade-trigger / audit-event-id / denial-reason vocabularies
  every session MUST disclose;
- one shared `devcontainer_profile_record` shape covering user-
  authored, workspace-shared committed, managed-workspace provisioned,
  imported-from-export-bundle, captured-from-handoff, and AI-tool-
  proposed-pending-review profiles;
- the declared-base-image-source / declared-image-freshness /
  declared-feature / lifecycle-hook / declared-compose-topology /
  declared-remote-user / workspace-mount-declaration / declared-
  capability / declared-unsupported-directive / declared-egress-
  posture / declared-port-forward fields every profile MUST disclose;
- the per-action attach-admissibility ladder that gates attach,
  exec one-shot, exec long-running, port forward, external-browser
  handoff, and inspect-only metadata read;
- the multi-service dependency-state vocabulary every compose / managed-
  workspace orchestrator MUST emit so API, queue, web-app, and
  database services do not collapse into one opaque "workspace
  started" state;
- the preflight-result hooks for engine reachability, unsupported
  directives, port collisions, and mirror or auth prerequisites so
  start, attach, and exec flows can explain inspect-only or blocked
  posture before work begins.

Out of scope (named explicitly so the schema does not creep):

- implementing any container runtime (Docker engine, Podman, containerd,
  nerdctl, managed-workspace runtime) or devcontainer orchestrator
  (devcontainer CLI, devcontainer features registry, prebuild warm-
  start backend, managed-cloud provisioning path) at this milestone;
- the desktop "reopen in container" UX, the multi-service topology
  inspector, the mount-boundary review surface, or the preflight
  result chip (this contract only declares the typed records the
  later UX MUST emit);
- per-engine wire formats (Docker socket, Podman REST, containerd
  gRPC) — the session record narrows through the typed engine class
  and reachability state, not raw engine errors;
- any container-runtime-specific telemetry; payloads narrow through
  the telemetry / support registry like every other surface.

## Shared session record

Every container-aware surface emits the same record from
[`container_session.schema.json`](../../schemas/runtime/container_session.schema.json).
The record does not replace the execution-context record (target
identity, sandbox posture, policy epoch live there) or the environment-
capsule record (toolchain identity, secret projection markers live
there). It is the compact projection consumers read when they need
one stable answer to five questions:

1. What is the **container-engine identity** (engine class, build
   identity, reachability state) this session is bound to?
2. What **mount and writable-path boundaries** are admissible right
   now?
3. What **privilege and attach / exec / forward posture** is
   admissible right now?
4. What **multi-service topology and dependency-order labels**
   resolve the surface against?
5. Which **preflight results** gate start / attach / exec before
   work begins?

### 1. Container-session class

`container_session_class` is the ten-value lane:

- `user_authored_dockerfile_session` — local Dockerfile or single-
  service profile authored by the user. Admissible only when engine
  reachability is verified and privilege posture is rootless or
  rooted-default.
- `compose_orchestrated_container_session` — compose topology with a
  non-empty multi-service topology list.
- `devcontainer_provisioned_session` — provisioned through a
  `devcontainer_profile_record`. Requires a non-null
  `devcontainer_profile_record_ref`.
- `managed_workspace_provisioned_container_session` — provisioned by
  managed-workspace policy. Requires a non-null
  `devcontainer_profile_record_ref` and a non-empty multi-service
  topology list.
- `imported_from_container_handoff_packet_session` — non-live session
  hydrated from a captured container-handoff packet; no live attach.
  Requires a non-null `container_handoff_packet_ref`.
- `replayed_from_container_capture_session` — non-live session
  rehydrated from a container capture; no live attach. Requires a
  non-null `container_capture_ref`.
- `inspect_only_uncertain_engine_session` — engine reachability,
  privilege posture, or mount boundary is uncertain; attach is denied.
- `external_handoff_only_engine_remote_unreachable_session` — surface
  routes the user to an external engine; no in-app attach.
- `blocked_engine_unreachable_session` — strictest stop. Every attach
  entry resolves to a blocked / inspect-only / external-handoff class.
- `ai_tool_proposed_container_session_pending_review` — forbids
  leaving pending review state.

The schema's `allOf` gates enforce that the four live classes carry
`engine_reachability_class = engine_reachable_verified` and a non-
escalating privilege posture, that imported / replayed sessions cite
the matching capture / handoff ref, and that compose-orchestrated and
managed-workspace-provisioned sessions resolve a non-empty multi-
service topology list.

### 2. Container engine, build identity, and reachability

`container_engine_class` names the engine the session is bound to:
`docker_engine_local`, `docker_engine_remote`, `podman_engine_local`,
`podman_engine_remote`, `containerd_engine_local`,
`nerdctl_engine_local`, `managed_workspace_container_runtime`,
`captured_engine_handoff`, `engine_class_unknown_requires_review`.

`engine_build_identity_ref` is the opaque engine-build-identity handle
(engine-build-id-class). Raw socket paths, raw context names, and raw
connection strings never project through this record.

`engine_reachability_class` is the nine-value vocabulary:
`engine_reachable_verified`, `engine_warming_provisioning`,
`engine_degraded_pending_diagnostics`,
`engine_unreachable_socket_missing`,
`engine_unreachable_permission_denied`,
`engine_unreachable_remote_endpoint`,
`engine_blocked_by_workspace_trust`, `engine_blocked_by_policy`,
`engine_reachability_unknown_requires_review`.

Only `engine_reachable_verified` admits live attach / exec / forward;
every other state forces inspect-only / blocked / external-handoff
posture and the matching downgrade trigger
(`engine_unreachable`, `engine_socket_missing`,
`engine_permission_denied`, `engine_remote_endpoint_unreachable`).

### 3. Privilege posture and user namespace

`privilege_posture_class` is the eight-value vocabulary:
`rootless_user_namespaced`,
`rootless_user_namespaced_no_new_privileges`,
`rooted_default_no_added_capabilities`,
`rooted_with_added_capabilities_user_review_required`,
`privileged_full_capabilities_blocked_pending_admin_admit`,
`privileged_socket_mount_blocked_pending_admin_admit`,
`privileged_host_filesystem_mount_blocked_pending_admin_admit`,
`privilege_posture_unknown_requires_review`.

The three `privileged_*` classes are blocked-pending-admit terminal
states. A session that resolves to one of them MUST resolve
`container_session_class` to `blocked_engine_unreachable_session` or
pin attach admissibility to `blocked_pending_policy`. **Ambient
privileged-shell escalation is forbidden mechanically by the
`ambient_privileged_shell_escalation_forbidden` denial reason.** No
contract path resolves to a privileged shell without an admin admit.

`rooted_with_added_capabilities_user_review_required` is admissible
only when every admissible attach entry resolves to
`admissible_under_approval_ticket` with a cited ticket; ambient
capability addition is forbidden.

`user_namespace_mode_class` is the five-value vocabulary
(`userns_remap_host_uid_mapped`, `userns_keep_id_managed_workspace`,
`userns_default_engine_managed`, `userns_disabled_user_review_required`,
`userns_unknown_requires_review`). The two non-default disabled /
unknown classes force inspect-only fallback for every attach action
through the `user_namespace_disabled` downgrade trigger.

### 4. Mount model and writable-path boundaries

`mounts` is the per-mount entry list. Each entry carries a closed
`mount_kind_class`:

- `workspace_bind_mount_read_write` — workspace bound RW.
- `workspace_bind_mount_read_only` — workspace bound RO.
- `non_workspace_bind_mount_read_only` — non-workspace bind RO.
- `non_workspace_bind_mount_read_write_user_review_required`
  — non-workspace bind RW, surfaces a chip.
- `named_volume_workspace_owned`,
  `named_volume_managed_workspace_owned`,
  `anonymous_volume_session_only`, `tmpfs_in_memory_session_only`
  — volume / tmpfs lanes (no host bind).
- `secret_or_config_mount_via_broker_handle` — MUST cite a non-null
  `secret_broker_handle_ref` (ADR-0007).
- `forbidden_engine_socket_mount_pending_admin_admit` — host engine
  socket mount; blocked.
- `forbidden_host_filesystem_mount_pending_admin_admit` — host
  filesystem mount (covers `/`, `/etc`, `~`, etc.); blocked.
- `mount_kind_class_unknown_requires_review`.

`mount_writability_class` is the four-value vocabulary
(`read_only`, `read_write`, `read_write_session_only`,
`writability_class_unknown_requires_review`).

The schema's `allOf` gate enforces that **any** mount entry resolving
to `forbidden_engine_socket_mount_pending_admin_admit` or
`forbidden_host_filesystem_mount_pending_admin_admit` forces the
session into a blocked / external-handoff class and forbids
`admissible_under_*` on every attach entry. **No contract path
implies an ambient Docker-socket or privileged-shell escalation.**

This satisfies the acceptance rule that **fixtures make writable
paths, mount boundaries, and privilege posture explicit before
attach or exec actions** and the rule that **no contract path
implies ambient Docker-socket or privileged-shell escalation**.

### 5. Port-forward model

`port_forwards` is the per-port-forward entry list. Each entry carries
a closed `port_forward_class`:

- `loopback_to_loopback_local_only` — loopback inside the workspace
  boundary only.
- `loopback_to_workspace_only_share` — shared inside one workspace.
- `loopback_to_managed_workspace_route`,
  `loopback_to_remote_managed_workspace_route` — tunneled through
  managed-workspace policy.
- `no_forward_internal_service_only` — internal service; no forward.
- `captured_handoff_no_forward` — pinned to capture; no live forward.
- `forbidden_host_unrestricted_bind_pending_admin_admit` — covers
  `0.0.0.0`, public-interface, and IPv6 `::` binds; blocked.
- `port_forward_class_unknown_requires_review`.

Each entry MAY cite a `preview_route_record_ref` so the
`preview_route_record` stays the source of truth for service identity,
mapped port / route, target context, actor, expiry, and policy scope.
This contract does not redefine route binding; it just references it.

The schema's `allOf` gate enforces that any
`forbidden_host_unrestricted_bind_pending_admin_admit` entry strips
`admissible_under_*` on `forward_port_to_*` actions and forces the
`host_unrestricted_bind_pending_admin_admit` downgrade trigger.

### 6. Attach posture and attach-admissibility ladder

`attach_posture_class` is the nine-value lane the session declares
the attach surface in:
`attach_to_running_container_interactive_pty`, `exec_one_shot_no_pty`,
`exec_long_running_pseudo_tty`, `forward_only_no_attach`,
`inspect_only_no_attach`, `attach_blocked_pending_user_admit`,
`attach_blocked_pending_policy`, `attach_blocked_pending_workspace_trust`,
`attach_posture_unknown_requires_review`.

The session enumerates every attach action it intends to offer or
grey out through `effective_admissible_attach_actions`, where each
entry carries one of eight `attach_action_class` values
(`attach_interactive_terminal`, `exec_one_shot_command`,
`exec_long_running_command`, `forward_port_to_loopback`,
`forward_port_to_workspace_only`,
`forward_port_to_managed_workspace_route`,
`open_external_browser_handoff`, `inspect_container_metadata_read_only`)
and one of fourteen `attach_admissibility_class` values:

- `admissible_under_workspace_trust`
- `admissible_under_session_only_override`
- `admissible_under_approval_ticket` (requires non-null
  `approval_ticket_ref` on the entry)
- `inspect_only_engine_unreachable`
- `inspect_only_engine_unsupported`
- `inspect_only_privilege_posture_uncertain`
- `inspect_only_mount_boundary_uncertain`
- `external_handoff_only_engine_remote_unreachable`
- `blocked_pending_user_admit`
- `blocked_pending_policy`
- `blocked_pending_workspace_trust`
- `blocked_engine_unreachable`
- `blocked_privileged_directive_pending_admin_admit`
- `not_applicable_imported_or_replayed_session`

Live attach / exec / forward (any `admissible_under_*` class) is
admissible **only** when:

- `engine_reachability_class` is `engine_reachable_verified`,
- `privilege_posture_class` is `rootless_user_namespaced` /
  `rootless_user_namespaced_no_new_privileges` /
  `rooted_default_no_added_capabilities`
  (or `rooted_with_added_capabilities_user_review_required` with
  every admissible entry resolving to
  `admissible_under_approval_ticket`),
- `user_namespace_mode_class` is not
  `userns_disabled_user_review_required` /
  `userns_unknown_requires_review`,
- the mount model has no `forbidden_*` entries,
- the port-forward model has no
  `forbidden_host_unrestricted_bind_pending_admin_admit` entry on
  forward actions,
- `workspace_trust_state_class` is `workspace_trust_session_only_temporary`
  or `workspace_trust_trusted` (or the entry is
  `admissible_under_approval_ticket` with a cited ticket),
- the session class is one of the live classes,
- every blocking-pre-* preflight entry resolves to
  `preflight_passed_admissible` or `preflight_engine_reachable`.

This satisfies the acceptance rule that **container / devcontainer
sessions reuse the same execution-context, approval, and evidence
vocabulary as terminal / task / debug paths.**

### 7. Multi-service topology

`multi_service_topology` is the per-service entry list every compose-
orchestrated and managed-workspace-provisioned session resolves
non-empty. Each entry carries:

- `service_handle_ref` — opaque service-handle id.
- `service_role_class` — closed eleven-value vocabulary
  (`application_service`, `api_service`, `web_app_service`,
  `worker_or_queue_service`, `database_service`, `cache_service`,
  `message_broker_service`, `search_service`, `object_store_service`,
  `mock_or_fixture_service`, `service_role_unknown_requires_review`).
- `multi_service_dependency_state_class` — closed ten-value vocabulary:
  `service_not_started_pending_dependency`,
  `service_starting_dependencies_in_progress`,
  `service_started_healthy`,
  `service_started_unhealthy_user_review_required`,
  `service_started_health_unknown_no_probe`,
  `service_failed_to_start_blocked`,
  `service_terminated_unexpectedly`,
  `service_terminated_after_dependent_failed`,
  `service_dependency_cycle_detected_blocked`,
  `service_state_unknown_requires_review`.
- `depends_on_service_handle_refs` — opaque service-handle refs.
- `health_probe_handle_ref`, `service_label`, `summary`.

The schema's `allOf` gate enforces that any
`service_dependency_cycle_detected_blocked` entry forces the session
into a blocked / inspect-only class with the matching denial reason
`service_dependency_cycle_must_block_start` and the matching
`service_dependency_cycle_detected` downgrade trigger.

This satisfies the acceptance rule that **multi-service topology and
dependency-order labels mean API, queue, web app, and database do
not collapse into one opaque "workspace started" state.**

### 8. Preflight result hooks

`preflight_results` is the list of preflight-result hooks gate start /
attach / exec / forward flows. Each entry carries a closed
`preflight_result_class`:

- `preflight_passed_admissible`, `preflight_engine_reachable`
- `preflight_engine_unreachable_user_review_required`
- `preflight_unsupported_directive_user_review_required`
- `preflight_port_collision_user_review_required`
- `preflight_mirror_unreachable_user_review_required`
- `preflight_registry_auth_pending_user_review`
- `preflight_image_unverifiable_user_review_required`
- `preflight_workspace_trust_unset_blocked`
- `preflight_policy_blocked`
- `preflight_unknown_requires_review`

…and a `preflight_blocking_class` lane:
`non_blocking_advisory`, `blocking_pre_apply`, `blocking_pre_attach`,
`blocking_pre_exec`, `blocking_pre_forward`,
`not_applicable_inspect_only`.

Live container-session classes resolve a non-empty preflight list, so
engine reachability, unsupported directives, port collisions, and
mirror or auth prerequisites are explicit before live attach. Any
`blocking_pre_*` entry with a non-passed `preflight_result_class`
strips `admissible_under_*` on every attach entry through the schema
gate.

This satisfies the acceptance rule that **start and attach flows can
already report why a target is blocked, degraded, or read-only
without dropping to raw engine errors.**

### 9. Downgrade triggers

The first four `downgrade_trigger` values mirror the safe-preview
ladder (`workspace_trust_revoked`, `policy_narrowed`,
`approval_scope_changed`, `support_export_boundary`).

The remaining seventeen values are container-runtime-specific
extensions: `engine_unreachable`, `engine_socket_missing`,
`engine_permission_denied`, `engine_remote_endpoint_unreachable`,
`privileged_directive_pending_admin_admit`,
`host_filesystem_mount_pending_admin_admit`,
`engine_socket_mount_pending_admin_admit`,
`mount_boundary_uncertain`, `user_namespace_disabled`,
`port_collision_observed`,
`host_unrestricted_bind_pending_admin_admit`, `mirror_unreachable`,
`registry_auth_pending`, `image_unverifiable`,
`service_dependency_cycle_detected`, `service_health_unknown`,
`captured_handoff_must_not_attach_live`.

A session that needs to narrow MUST cite a typed trigger from this
list rather than describing the cause in prose.

### 10. Audit events and denial reasons

`audit_event_id` is the closed twenty-value vocabulary every container-
session admit / deny / preflight / topology event resolves to.

`denial_reason` is the closed twenty-two-value vocabulary every audit
denial event cites. The denial vocabulary mechanically forbids the
attack surface this contract closes:

- `ambient_engine_socket_mount_forbidden` and
  `ambient_privileged_shell_escalation_forbidden` — no contract path
  implies ambient Docker-socket or privileged-shell escalation.
- `host_filesystem_mount_forbidden_pending_admin_admit` /
  `host_unrestricted_bind_forbidden_pending_admin_admit` — host
  filesystem mounts and unrestricted port binds require admin admit.
- `engine_unreachable_user_review_required` /
  `engine_blocked_by_workspace_trust` /
  `engine_blocked_by_policy` — engine reachability is honest.
- `mount_writability_must_resolve_to_typed_class` /
  `mount_kind_must_resolve_to_typed_class` — mount truth is typed.
- `privilege_posture_must_resolve_to_typed_class` /
  `user_namespace_mode_must_resolve_to_typed_class` /
  `attach_posture_must_resolve_to_typed_class` /
  `engine_class_must_resolve_to_typed_class` — postures are typed.
- `secret_or_config_mount_must_cite_broker_handle` — secret mounts
  resolve through ADR-0007.
- `ai_tool_proposed_session_must_not_attach_pending_review` /
  `captured_handoff_must_not_attach_to_live_engine` — AI-proposed
  sessions and captured handoffs cannot attach pending admit.
- `preflight_blocking_pre_attach_unsatisfied` /
  `preflight_blocking_pre_exec_unsatisfied` /
  `preflight_blocking_pre_forward_unsatisfied` — blocking preflight
  is honest.
- `service_dependency_cycle_must_block_start` — cycles block start.
- `workspace_trust_unset_or_restricted` /
  `policy_epoch_expired_re_evaluation_required` — trust / policy.

## Devcontainer-profile record

A devcontainer profile is the manifest that backs a
`devcontainer_provisioned_session` or
`managed_workspace_provisioned_container_session`. The profile is
read-once at admit and the session is provisioned against it; the
profile record is the cross-surface projection every devcontainer-
aware reader uses to inspect the manifest before provisioning.

### 1. Profile class

`devcontainer_profile_class` is the seven-value lane:

- `user_authored_local_profile` — default for individual workspaces.
- `workspace_shared_committed_profile` — committed and shared.
- `managed_workspace_provisioned_profile` — provisioned by managed-
  workspace policy. Resolves `declared_base_image_source_class` to
  `managed_workspace_image_pinned_by_admin` or
  `image_ref_pinned_to_digest` and `workspace_trust_state_class` to
  `workspace_trust_managed_locked`.
- `imported_from_export_bundle_profile` — hydrated from an export
  bundle.
- `captured_from_handoff_profile` — hydrated from a handoff.
- `ai_tool_proposed_profile_pending_review` — forbids
  `declared_base_image_source_class` outside
  `ai_proposed_image_pending_user_review`; lifecycle hooks resolve
  through `ai_proposed_lifecycle_hook_pending_user_review`.

### 2. Declared base image source

`declared_base_image_source_class` is the seven-value vocabulary
(`dockerfile_in_workspace`, `image_ref_pinned_to_digest`,
`image_ref_pinned_to_tag_user_review_required`,
`compose_service_image`, `managed_workspace_image_pinned_by_admin`,
`ai_proposed_image_pending_user_review`,
`base_image_source_class_unknown_requires_review`).

`declared_image_freshness_class` is the six-value freshness lane the
profile reader projects (`declared_pinned_to_digest_admissible`,
`declared_pinned_to_tag_user_review_required`,
`declared_unpinned_floating_user_review_required`,
`declared_freshness_unknown_requires_review`,
`not_applicable_built_from_dockerfile`,
`not_applicable_compose_per_service`).

The schema's `allOf` gate enforces that:

- `dockerfile_in_workspace` requires a non-null `dockerfile_handle_ref`
  and resolves freshness to `not_applicable_built_from_dockerfile`.
- `compose_service_image` requires a non-null
  `compose_service_handle_ref` and resolves freshness to
  `not_applicable_compose_per_service`.
- `image_ref_pinned_to_digest` resolves freshness to
  `declared_pinned_to_digest_admissible`.
- `image_ref_pinned_to_tag_user_review_required` resolves freshness
  to `declared_pinned_to_tag_user_review_required`.

### 3. Declared features

`declared_features` is the per-feature entry list. Each entry carries
one of thirteen `declared_feature_class` values
(`language_runtime_feature`, `package_manager_feature`,
`shell_or_terminal_feature`, `vcs_feature`,
`container_in_container_feature_user_review_required`,
`kubernetes_in_container_feature_user_review_required`,
`cloud_cli_feature`, `ide_or_editor_feature`,
`credential_helper_feature_user_review_required`,
`ssh_agent_feature_user_review_required`,
`gpu_runtime_feature_user_review_required`,
`ai_proposed_feature_pending_user_review`,
`feature_class_unknown_requires_review`) and an opaque
`feature_handle_ref`. Raw feature URLs, raw OCI references, and raw
feature versions never project here.

### 4. Lifecycle hooks

`lifecycle_hooks` is the per-hook entry list, mirroring the
devcontainer lifecycle: `initialize_command_runs_on_host`,
`on_create_command_runs_in_container`,
`update_content_command_runs_in_container`,
`post_create_command_runs_in_container`,
`post_start_command_runs_in_container`,
`post_attach_command_runs_in_container`,
`wait_for_command_runs_in_container`,
`ai_proposed_lifecycle_hook_pending_user_review`,
`lifecycle_hook_class_unknown_requires_review`.

`initialize_command_runs_on_host` is special-cased: it runs on the
host rather than the container, and the schema requires it to cite
a non-null `hook_capability_class` so a reviewer reads what the
host-side hook is allowed to do
(`read_only_workspace_inspection`, `write_workspace_state`,
`spawn_subprocess_in_container`,
`spawn_subprocess_on_host_user_review_required`,
`network_egress_user_review_required`,
`secret_access_via_broker_handle`,
`hook_capability_class_unknown_requires_review`).

`ai_proposed_lifecycle_hook_pending_user_review` hooks are admissible
only on `ai_tool_proposed_profile_pending_review` profiles.

### 5. Compose topology declaration

`declared_compose_topology` is the typed compose-topology block.
`declared_compose_topology_class` ∈
{`single_service_no_compose`, `compose_with_services`,
`compose_with_overrides_user_review_required`,
`compose_with_remote_extends_user_review_required`,
`compose_topology_unknown_requires_review`}.

`compose_with_services` and the two `_user_review_required` classes
require a non-null `compose_file_handle_ref` and a non-empty
`declared_compose_services` list, where each entry pairs a
`service_handle_ref` with a `declared_service_role_class` and a
`depends_on_service_handle_refs` list. This is the manifest the
session record's `multi_service_topology` is materialised against.

### 6. Remote user, mount declaration, capability declaration

`declared_remote_user_class` ∈ {`non_root_user_default`,
`non_root_user_custom`, `root_user_user_review_required`,
`managed_workspace_user_pinned_by_admin`,
`ai_proposed_remote_user_pending_user_review`,
`remote_user_class_unknown_requires_review`}.

`workspace_mount_declaration_class` ∈
{`workspace_bound_to_default_workspace_folder`,
`workspace_bound_to_custom_workspace_folder`,
`workspace_bound_to_named_volume`,
`workspace_mount_omitted_user_review_required`,
`workspace_mount_unknown_requires_review`}. The two
`*_user_review_required` / `*_unknown_requires_review` classes
surface a chip and force inspect-only fallback at session-provisioning
time.

`declared_capability_class` ∈ {`no_added_capabilities`,
`add_capabilities_user_review_required`,
`drop_capabilities_admissible`,
`privileged_full_capabilities_blocked_pending_admin_admit`,
`no_new_privileges_admissible`,
`declared_capability_class_unknown_requires_review`}. The
`privileged_full_capabilities_blocked_pending_admin_admit` class
forces the session record into a blocked / external-handoff class
through the no-hidden-privileged-shell rule.

### 7. Declared unsupported directives and egress

`declared_unsupported_directives` enumerates every directive the
profile asks the engine to apply that the contract has flagged as
"admin-admit required" or "review required":
`host_filesystem_bind_mount_outside_workspace`,
`engine_socket_mount_inside_container`,
`host_unrestricted_port_bind`,
`privileged_full_capabilities_request`,
`ssh_agent_socket_mount_user_review_required`,
`credential_helper_with_host_socket_user_review_required`,
`remote_extends_compose_user_review_required`,
`user_namespace_disabled_request`,
`declared_unsupported_directive_class_unknown_requires_review`.

`declared_egress_posture_class` ∈ {`no_network_egress_required`,
`loopback_only`, `workspace_mirror_or_proxy_only`,
`managed_workspace_egress_via_envelope`,
`egress_to_public_internet_user_review_required`,
`egress_to_community_origin_user_review_required`,
`declared_egress_posture_class_unknown_requires_review`}.

### 8. Declared port forwards

`declared_port_forwards` is the per-port-forward declaration. Each
entry pairs an opaque `container_port_handle_ref` with a
`declared_share_class` (`loopback_only`, `workspace_only`,
`organization_only_user_review_required`,
`public_link_user_review_required`,
`declared_share_class_unknown_requires_review`).

## Re-use across desktop, CLI, evidence, and companion handoff paths

The container labels in this contract are deliberately surface-
agnostic. Desktop UX, CLI runners, evidence readers, hosted-review
readers, support / export readers, automation run review, AI tool
review, and companion handoff packets all read **the same**
`container_session_class`, `container_engine_class`,
`engine_reachability_class`, `privilege_posture_class`,
`mount_kind_class`, `mount_writability_class`, `port_forward_class`,
`attach_posture_class`, `attach_action_class`,
`attach_admissibility_class`, `multi_service_dependency_state_class`,
`service_role_class`, `preflight_result_class`,
`preflight_blocking_class`, `downgrade_trigger`, `audit_event_id`,
and `denial_reason` vocabularies.

Adding a label that a CLI surface needs but a desktop surface doesn't
is **not** admissible at the surface boundary; it lands additive-minor
on the relevant schema and every consumer surface picks it up
automatically.

This satisfies the acceptance rule that **container / devcontainer
sessions reuse the same execution-context, approval, and evidence
vocabulary as terminal / task / debug paths.**

## Composition with the execution-context, environment-capsule, and preview-route contracts

- The execution-context contract stays the source of truth for target
  identity, sandbox posture, and policy epoch. The session record
  cites `execution_context_record_ref` and never restates those
  fields. The `target_class` field on the session record is narrowed
  to the five values that resolve through a container engine.
- The environment-capsule contract stays the source of truth for
  toolchain identity, secret-projection markers, and export path.
  The session and the profile cite `environment_capsule_ref` (when
  present) and never restate those fields.
- The preview-route contract stays the source of truth for service
  identity, mapped port / route, target context, actor, expiry, and
  policy scope. Each port-forward entry MAY cite
  `preview_route_record_ref` and never restates those fields.

If a future change widens the container vocabulary, it MUST land
additive-minor on the relevant schema (container-runtime-specific
values on the session schema; profile-specific values on the profile
schema; execution-context / environment-capsule / preview-route
values on their owning schemas) and bump the corresponding
`*_schema_version` const.

## Change discipline

Adding a new `container_session_class`, `container_engine_class`,
`engine_reachability_class`, `privilege_posture_class`,
`user_namespace_mode_class`, `mount_kind_class`,
`mount_writability_class`, `port_forward_class`,
`attach_posture_class`, `attach_action_class`,
`attach_admissibility_class`, `multi_service_dependency_state_class`,
`service_role_class`, `preflight_result_class`,
`preflight_blocking_class`, `downgrade_trigger`, `audit_event_id`,
or `denial_reason` value on the session schema, or a new
`devcontainer_profile_class`, `declared_base_image_source_class`,
`declared_image_freshness_class`, `declared_feature_class`,
`lifecycle_hook_class`, `declared_compose_topology_class`,
`declared_remote_user_class`, `workspace_mount_declaration_class`,
`declared_capability_class`, `declared_unsupported_directive_class`,
or `declared_egress_posture_class` value on the profile schema, is
additive-minor and bumps the relevant `*_schema_version` const.
Repurposing an existing value is breaking and requires a new decision
row.

Re-exporting a vocabulary from another schema is preferred over
minting a parallel one. Where this contract narrows or extends a
re-export, the gate is documented above; if a future contributor
needs to narrow further, that change lands on the owning schema, not
through a private fork in this directory.
