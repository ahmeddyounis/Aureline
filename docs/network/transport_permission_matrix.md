# Shared Transport Permission Matrix

This document publishes the shared permission-class, mirror/offline,
and audit-requirement matrix for every network-capable Aureline
surface. It sits above the lower-level transport governance seed:
`transport_governance_seed.md` freezes proxy, trust, route, egress,
and attribution vocabularies; this matrix freezes which product
surfaces may use those vocabularies and which audit fields they must
emit.

Companion artifacts:

- [`/artifacts/network/permission_classes.yaml`](../../artifacts/network/permission_classes.yaml)
  is the machine-readable permission-class and audit-requirement
  register.
- [`/artifacts/network/mirror_offline_matrix.yaml`](../../artifacts/network/mirror_offline_matrix.yaml)
  is the machine-readable mirror/offline deployment matrix.
- [`/fixtures/network/audit_event_examples/`](../../fixtures/network/audit_event_examples/)
  contains worked audit-event examples.
- [`/artifacts/network/egress_classes.yaml`](../../artifacts/network/egress_classes.yaml)
  remains the lower-level enum and admissible-egress matrix.
- [`/docs/network/transport_governance_packet_seed.md`](transport_governance_packet_seed.md)
  remains the transport-decision packet contract.

The source architecture appendix names the first permission classes
with hyphenated display tokens. Repository artifacts use the existing
snake_case tokens from `egress_classes.yaml`; for example
`core-update` maps to `core_update`.

## Contract Rules

1. Every network-capable surface MUST resolve a shared
   `permission_class` before it resolves an endpoint, route, proxy,
   trust source, mirror, or retry posture.
2. Every network attempt MUST emit either a
   `transport_decision_record` or a compatible audit event that carries
   the same endpoint, route, outcome, deny-reason, and redaction
   fields.
3. Desktop, CLI/headless, extension-host, AI, remote, managed-service,
   and browser-handoff paths MUST reuse the same permission class for
   the same action family.
4. Mirror-only, offline, air-gapped, deny-all, and local-only posture
   are normal profiles. They are not subsystem exceptions.
5. Any network-capable surface that claims special handling outside the
   shared transport governance layer MUST attach both an exception
   packet and an unexpired waiver. The exception must name what is
   bypassed, who approved it, when it expires, and which audit evidence
   remains.

## Permission Rows

| Permission class | Surface family | Default posture | Mirror/offline support | Required audit spine |
|---|---|---|---|---|
| `core_update` | Updates and release metadata | Channel-governed by policy; signed metadata required before fetch | Mirrors and offline bundles required for managed, self-hosted, and air-gapped claims | endpoint class, channel/ref, artifact digest or update manifest ref, signer/trust-root refs, outcome |
| `registry_read` | Extension registry read | Allowed by user/admin policy; public only when profile permits | Private registries, approved mirrors, and offline bundles are first-class | endpoint class, registry source class, artifact digest/ref, publisher identity, mirror snapshot/ref, outcome |
| `registry_write` | Extension registry publish/update | Requires publisher identity plus approval or step-up where policy requires | No offline mutation; publish-later requires a separate provider/offline handoff packet | endpoint class, publisher identity, artifact digest/ref, approval ticket, channel/registry ref, outcome |
| `ai_context_egress` | AI context sent to a model route | Opt-in and policy-bounded; disabled by deny-all and air-gap unless local/model-pack route exists | Local model packs and enterprise gateways are the mirror/offline substitutes; no hidden public fallback | endpoint class, provider entry, model entry, context class, data-class allowlist, redaction posture, outcome |
| `ai_tool_egress` | AI tool or external-tool call | Denied unless tool grant and approval posture are satisfied | Local tools may run local-only; remote tools require live or org-approved route | endpoint class, provider/model when relevant, tool entry, side-effect class, approval ticket, target class, outcome |
| `docs_fetch` | Docs/help/tutorial packs | Optional and low risk, but citation and freshness claims depend on manifest state | Mirrors and offline docs bundles required for air-gap and sovereign claims | endpoint class, docs pack id/revision, source class, mirror snapshot/ref, freshness class, outcome |
| `remote_connect` | Remote SSH, remote agents, managed workspaces, brokers | Explicit user action or policy-controlled reconnect | Self-hosted targets and managed relays are supported; air-gap only when route stays inside the boundary | endpoint class, broker/target class, target witness, auth mode, host proof or client cert state, outcome |
| `vcs_network` | Git clone/fetch/pull/push and hosted Git remotes | Workspace/user initiated; local Git remains available without network | Enterprise mirrors and target-local remotes supported; offline mutation is not silently queued | endpoint class, remote class, operation class, ref/update ref, auth mode, failure or deny reason |
| `extension_network` | Extension-authored egress | Denied unless an extension permission grant admits the route | Mirrors/proxy allowed where possible; extension egress never bypasses host governance | endpoint class, extension id/ref, permission grant, publisher identity, requested target class, outcome |
| `policy_refresh` | Policy, entitlement, trust-root, emergency bundles | Signed bundle/cache path; pinned control-plane trust for live refresh | File import, mirror publication, and offline bundle evaluation required for enterprise claims | endpoint class, bundle ref, policy epoch, signer/trust-root refs, freshness/expiry, outcome |
| `telemetry_send` | Telemetry, diagnostics summaries, usage envelopes | Opt-in or managed-policy admitted; off by default for local-only | Local capture/export remains available when send is denied | endpoint class, telemetry stream class, event batch ref, consent/policy ref, redaction class, outcome |
| `tunnel_publish` | Public or org-visible port/tunnel exposure | Explicit user action plus approval on managed/enterprise profiles | Not available in air-gap or local-only posture unless route stays same-device | endpoint class, tunnel/route ref, exposure class, target class, approval ticket, outcome |
| `provider_callback_inbound` | OAuth callbacks, webhooks, provider return events | Bound to a prior browser-handoff or provider packet | Local callback capture supported; provider-originated live mutation requires fresh admission | endpoint class, callback envelope, provider actor class, handoff/correlation ref, outcome |
| `mirror_refresh` | Mirror snapshot refresh | Idempotent refresh with mirror provenance and signer continuity | Offline-deferred allowed only for idempotent refresh; stale mirrors fail typed | endpoint class, mirror endpoint, mirror snapshot, artifact family, freshness class, outcome |
| `browser_handoff_launch` | System-browser handoff | Explicit packet with reason, destination class, and return anchor | In air-gap, only local/file handoff is allowed; public browser launch is blocked unless policy admits | endpoint class, handoff packet, destination class, provider actor class, return anchor, outcome |
| `api_client_request` | First-class HTTP/GraphQL/API request, package restore, symbol/test provider fetch | User/workspace action or policy-admitted automation | Mirror/local bundle path required when the surface claims offline support | endpoint class, API surface class, target class, auth handle class, request document/ref, outcome |

## Source-Surface Mapping

| Source-doc surface | Shared permission class | Canonical component class examples | Audit requirement |
|---|---|---|---|
| Update feeds and release metadata | `core_update` | `update_client_component` | artifact digest or update manifest ref, signer/trust root, channel |
| Extension registry discovery/install metadata | `registry_read` | `extension_registry_component`, `desktop_shell_component`, `cli_headless_component` | registry source, publisher identity, artifact digest/ref |
| Extension publication or registry mutation | `registry_write` | `extension_registry_component`, `cli_headless_component` | publisher identity, approval ticket, artifact digest/ref |
| Extension-authored network egress | `extension_network` | `extension_host_component` | extension id/ref, permission grant, publisher identity |
| AI provider context send | `ai_context_egress` | `ai_broker_component` | provider entry, model entry, context/data class, redaction posture |
| AI tool, MCP, retrieval, or external-tool call | `ai_tool_egress` | `ai_broker_component`, `extension_host_component` when mediated by an extension | tool entry, side-effect class, approval/permission grant |
| Docs/help/tutorial pack fetch | `docs_fetch` | `docs_pack_component` | docs pack revision, source class, mirror snapshot, freshness |
| Remote SSH, agent attach, managed workspace connect | `remote_connect` | `remote_ssh_component`, `remote_agent_component`, `remote_tunnel_component`, `managed_control_plane_component` | target witness, broker/target class, auth mode, trust proof |
| Tunnel or port-forward publication | `tunnel_publish` | `remote_tunnel_component` | route ref, exposure class, approval ticket, revoke path |
| Git remote clone/fetch/pull/push | `vcs_network` | `git_vcs_component`, `desktop_shell_component`, `cli_headless_component` | remote class, operation class, auth mode, failure/deny reason |
| Policy endpoints, offline entitlement, trust-root refresh | `policy_refresh` | `policy_distribution_component`, `managed_control_plane_component` | bundle ref, policy epoch, signer/trust-root refs, freshness |
| Telemetry and diagnostics send | `telemetry_send` | `telemetry_sink_component` | consent/policy ref, stream class, event batch ref, redaction class |
| Mirror refresh for updates, docs, registry, models, or policy | `mirror_refresh` | `mirror_refresh_component` | mirror endpoint, snapshot/ref, artifact family, freshness |
| Browser-mediated auth or provider handoff launch | `browser_handoff_launch` | `browser_handoff_launcher_component`, `desktop_shell_component`, `cli_headless_component` | handoff packet, destination class, return anchor |
| Provider callback, webhook, OAuth return | `provider_callback_inbound` | `provider_callback_receiver_component`, `browser_handoff_launcher_component` | callback envelope, provider actor class, correlation/handoff ref |
| Request workspace, browser runtime network, API client, package restore, symbol/test provider fetch | `api_client_request` | `api_client_component`, `notebook_kernel_component`, `debugger_symbol_component` | endpoint class, API surface class, request document/ref, auth handle |
| Notebook remote kernel or kernel-side endpoint | `remote_connect` or `api_client_request` depending on route | `notebook_kernel_component`, `remote_agent_component` | kernel target class, route origin, auth mode, outcome |
| Debugger/test infrastructure that fetches remote symbols, maps, test provider data, or attaches remotely | `api_client_request` or `remote_connect` depending on route | `debugger_symbol_component`, `remote_agent_component`, `cli_headless_component` | artifact/ref or target witness, operation class, failure/deny reason |

## Shared Audit Field Groups

Every audit event must include the common transport spine:

- `event_id`, `event_name`, `event_captured_at`, and
  `invocation_session_ref`
- `component_class`, `permission_class`, `origin_scope`,
  `target_class`, `endpoint_class`, and opaque `target_identity_ref`
- `active_deployment_profile`, `active_identity_mode`,
  `active_policy_bundle_ref`, and `policy_epoch_ref` where available
- `egress_class`, `route_class`, `proxy_resolution_mode`,
  `trust_store_source`, `mirror_route_class`, and
  `offline_or_deny_all_state`
- `transport_decision_code`, `outcome_class`,
  `deny_reason_detail_class` or `failure_reason_class` where relevant
- `redaction_class`, `export_safe`, and `narrative_refs`

Permission-specific fields are required where relevant:

- Artifact paths: `artifact_ref`, `artifact_digest_ref`,
  `channel_ref`, `pack_revision_ref`, `bundle_ref`, or
  `mirror_snapshot_ref`
- Publisher and provider paths: `publisher_identity_ref`,
  `provider_entry_ref`, `model_entry_ref`, `tool_entry_ref`, and
  `provider_actor_class`
- Remote paths: `broker_class`, `target_witness_ref`,
  `host_proof_ref`, `client_certificate_handle_ref`, and
  `route_exposure_class`
- Git/API paths: `remote_class`, `operation_class`,
  `request_document_ref`, and `api_surface_class`
- Denials and failures: `policy_rule_ref`, `user_setting_ref`,
  `trust_failure_state`, `proxy_failure_state`,
  `mirror_freshness_class`, and a reviewable failure sentence

Raw URLs, hostnames, IP addresses, request bodies, response bodies,
tokens, cookies, proxy credentials, private keys, certificate PEM, PAC
script bodies, and raw SSH keys are never audit fields. They may be
resolved from opaque refs only inside the narrowest authorized support
or local diagnostic boundary.

## Mirror And Offline Rules

- Mirrorable artifact families are updates, extension registry rows,
  docs packs, policy/trust bundles, AI model assets, and package or
  symbol artifacts where the source contract claims mirror support.
- Mirror-only routes must carry `mirror_snapshot_ref`,
  `mirror_freshness_class`, `mirror_route_class`, and
  `public_origin_allowed = false`.
- Offline bundles are admissible only when the artifact identity,
  signature/trust state, revocation snapshot, and freshness window are
  inspectable without vendor reachability.
- Only idempotent actions may be `offline_deferred`. Mutations that
  publish, delete, expose a tunnel, rotate trust, call an external AI
  tool, or mutate a remote provider must fail typed and request fresh
  approval later.
- Local-only mode does not imply "try the vendor endpoint later." A
  local-only row must state which features continue locally and which
  networked actions are denied.

## Exception Path

The only admissible bypass path is:

1. A transport exception packet based on
   [`/docs/governance/templates/exception_packet_template.md`](../governance/templates/exception_packet_template.md)
   names the exact bypassed rule, surface, component class,
   permission class, endpoint class, and expiry.
2. An unexpired waiver based on
   [`/docs/governance/templates/waiver_template.md`](../governance/templates/waiver_template.md)
   names the approver, owner, compensating controls, audit evidence,
   and reapproval trigger.
3. The emitted transport decision or audit event carries both
   `exception_ref` and `waiver_ref`.

Without all three, direct origin fallback, insecure TLS fallback, proxy
skip, CLI/desktop divergence, extension ambient egress, destructive
offline deferral, and surface-local retry loops are non-conforming.
