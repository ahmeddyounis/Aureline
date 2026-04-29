# Proxy, certificate, SSH trust, egress-class, and network-attribution governance seed

This document is the narrative seed for Aureline's shared transport
governance layer. It freezes one proxy resolution vocabulary, one
TLS / OS trust-store + org CA-bundle + pinned-control-plane trust
vocabulary, one SSH host-key provenance vocabulary, one egress-class
vocabulary, one inspectable current transport-posture object, and one
network attribution-record shape that the desktop shell, CLI /
headless runner, extension host, AI broker, update client, extension
registry, docs pack, policy distribution, telemetry sink, remote
tunnel, remote SSH, remote agent, managed control plane, git VCS,
notebook kernel, debugger / symbol tooling, API client, mirror
refresh, browser handoff launcher, and provider callback receiver
components all resolve against.

It exists so every later network-capable surface lands on one
transport vocabulary instead of inventing its own proxy, trust,
egress, or attribution schema. Without this seed, each subsystem
would grow a private "is the network on?" badge, a private
"allowed domains?" list, a private "managed relay vs direct?"
banner, a private CA-bundle override path, and a private rationale
for why an extension could egress. Aureline cannot defend
desktop-first, CLI-equivalent, enterprise-supportable networking
claims that way.

Companion artifacts:

- [`/docs/network/transport_permission_matrix.md`](transport_permission_matrix.md):
  published permission-class, mirror/offline, and audit-requirement
  matrix that maps every network-capable surface to this shared
  transport vocabulary.
- [`/artifacts/network/permission_classes.yaml`](../../artifacts/network/permission_classes.yaml):
  machine-readable permission-class and audit-requirement register
  consumed by UI, CLI, support export, policy, and fixture rows.
- [`/artifacts/network/mirror_offline_matrix.yaml`](../../artifacts/network/mirror_offline_matrix.yaml):
  machine-readable air-gapped, sovereign, enterprise-mirror, and
  local-only mirror/offline posture matrix.
- [`/docs/network/transport_governance_packet_seed.md`](transport_governance_packet_seed.md):
  decision-packet layer that projects the shared route, trust,
  mirror/public, failure, no-bypass, and repair vocabulary into
  summary strips, endpoint rows, certificate cards, denied-attempt
  histories, CLI explain output, support exports, and release
  evidence.
- [`/docs/network/transport_explainability_surface_contract.md`](transport_explainability_surface_contract.md):
  user-facing projection layer for transport summary strips,
  endpoint rows, certificate/detail cards, denied-attempt history
  rows, repair actions, and no-bypass explanations.
- [`/schemas/network/transport_summary_strip.schema.json`](../../schemas/network/transport_summary_strip.schema.json),
  [`/schemas/network/endpoint_history_row.schema.json`](../../schemas/network/endpoint_history_row.schema.json),
  and
  [`/schemas/network/certificate_detail_card.schema.json`](../../schemas/network/certificate_detail_card.schema.json):
  machine-readable boundaries for the explainability surfaces that
  consume the transport decision and attribution records.
- [`/schemas/network/transport_decision.schema.json`](../../schemas/network/transport_decision.schema.json):
  machine-readable boundary for one `transport_decision_record`,
  reusing the proxy, trust, SSH host-key, mirror, egress, outcome,
  and deny-reason values from this seed.
- [`/artifacts/network/proxy_lab_matrix.yaml`](../../artifacts/network/proxy_lab_matrix.yaml):
  enterprise proxy, custom CA, PAC, manual proxy, strict SSH, mTLS
  client-certificate, mirror-only, deny-all, and offline-deferred lab
  matrix for claimed profiles.
- [`/schemas/network/network_attribution_record.schema.json`](../../schemas/network/network_attribution_record.schema.json)
  — machine-readable boundary for
  `network_attribution_record`, including the embedded
  `transport_posture` object.
- [`/artifacts/network/egress_classes.yaml`](../../artifacts/network/egress_classes.yaml)
  — machine-readable matrix binding every frozen
  permission class, component class, target class, route class, auth
  mode, egress class, proxy-resolution mode, trust-store source, SSH
  host-key provenance class, mirror-route class, offline / deny-all
  state class, outcome class, and deny-reason class with per-profile
  default-egress defaults and admissible deny-reason sets.
- [`/fixtures/network/connectivity_cases/`](../../fixtures/network/connectivity_cases/)
  — worked fixtures for individual-profile public-registry read
  allowed under a system proxy, enterprise PAC update allowed under
  an org CA bundle, extension AI-tool egress denied by admin policy,
  self-hosted remote SSH host-key-mismatch strict rejection,
  air-gapped docs refresh via an offline bundle mirror, managed-cloud
  system-browser auth callback, enterprise telemetry denied by user
  disable, and an individual-profile deny-all refusal covering every
  egress class.
- [`/fixtures/network/transport_explainability_cases/`](../../fixtures/network/transport_explainability_cases/)
  — worked projection fixtures for PAC-sourced proxy, custom CA,
  mirror unavailable, policy blocked, and SSH host-proof mismatch
  cases.
- [`/fixtures/network/audit_event_examples/`](../../fixtures/network/audit_event_examples/)
  — worked audit-event examples that demonstrate the published
  permission classes, audit requirements, no-bypass rules, and
  mirror/offline rows.

Upstream contracts this seed rides on:

- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md)
  for the `account_free_local` / `self_hosted_org` /
  `managed_workspace` identity modes.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  for credential-handle shape, redaction class, and org CA-bundle
  boundary rules.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  for browser-handoff, approval-ticket, and provider-callback
  semantics.
- [`/docs/runtime/origin_target_route_taxonomy.md`](../runtime/origin_target_route_taxonomy.md)
  for the action origin / target / route / exposure vocabulary the
  network target and route classes cross-walk with.
- [`/docs/deployment/locality_and_continuity_seed.md`](../deployment/locality_and_continuity_seed.md)
  for deployment-profile, locality, tenant / org scope, region scope,
  mirror-freshness, and offline / air-gap vocabulary.
- [`/docs/identity/offline_entitlement_and_policy_seed.md`](../identity/offline_entitlement_and_policy_seed.md)
  for the policy-bundle identity (`active_policy_bundle_ref`,
  `policy_epoch_ref`) carried on the transport posture.
- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
  for the runtime authority-ticket and external-effect lineage that
  external-mutating network attributions link to.
- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
  for per-OS proxy / trust-store / SSH conformance rows this seed's
  `transport_posture.proxy_resolution_mode` /
  `trust_store_source` / `ssh_host_key_default_provenance` resolve
  against.

Normative sources this seed projects from:

- `.t2/docs/Aureline_PRD.md` §5.42 *Network, proxy, certificates, and
  transport-governance architecture*, §5.27 platform conformance
  row for proxy and trust stores, §5.45 managed-service and
  enterprise-control section.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  Appendix G *Network and Transport Policy Matrix*, Appendix O
  enterprise-control boundaries, §9.6 managed-service separation.
- `.t2/docs/Aureline_Technical_Design_Document.md` transport-
  governance layer section.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` status-item transport
  posture surface.

If this document disagrees with those sources, those sources win and
this document plus the companion schema, matrix, and fixtures update
in the same change. If this document and the YAML matrix disagree,
this document wins and the YAML updates in the same change. If this
document and the schema disagree on an enum or invariant, the schema
wins and this document is updated in the same change.

## Why this exists

A modern IDE is a transport product. Aureline touches git remotes,
extension registries, update feeds, docs mirrors, policy endpoints,
AI gateways, managed control planes, remote SSH hosts, remote agents,
devcontainers, tunnel-exposed previews, browser-mediated auth
callbacks, webhooks, telemetry sinks, notebook kernels, and
debugger / symbol lookups. If each of those paths handles proxy
discovery, trust evaluation, SSH host-key provenance, egress
boundaries, offline posture, and outbound-denied evidence differently,
enterprise rollout, supportability, and desktop-first truth all
break.

This seed makes the network-facing rule explicit:

- Any network-capable component MUST declare **why** it needs the
  network, **which permission gate** it uses, and **what happens**
  when egress is denied. Silent egress is non-conforming.
- Desktop and CLI / headless flows MUST resolve proxy, trust,
  SSH host-key, mirror, and offline / deny-all posture against the
  same matrix. A CLI-only or desktop-only override is not
  admissible.
- Extension network access is permissioned and attributable, not
  ambient. Any extension-host outbound attempt MUST carry a
  non-null `permission_grant_ref` on egress classes above
  `local_only`.
- Remote mode MUST make it explicit whether egress originates on the
  **client**, the **remote target**, or an intermediate **managed
  relay**.
- Recent outbound-allowed, outbound-denied, and outbound-failed
  events MUST be representable without leaking raw URLs, raw host
  names, raw IPs, raw SSH host keys, raw proxy credentials, raw CA
  bundle bytes, raw tokens, raw request bodies, or raw response
  bodies.

Live provider implementations, full enterprise PKI integration, and
the full network stack remain out of scope at this revision; the
vocabulary and invariants below are what those integrations will
honour.

## Two questions the seed answers

Any Aureline surface claiming a network-aware behaviour MUST answer
both questions mechanically, without relying on human-written copy:

1. **What is the current transport posture?** Which proxy resolution
   mode is in force, which trust-store source is resolving certs,
   which org CA bundle or pinned control-plane trust is active,
   which SSH host-key default provenance applies, which mirror
   route is live, which deployment profile and identity mode are
   active, which policy epoch is bound, and what is the offline /
   deny-all state?
2. **What did this specific attempt do?** Which component attempted
   the call, under which permission class, against which target
   class and opaque target ref, using which auth mode, on which
   route, at which egress class, with what outcome (allowed,
   allowed-with-warning, in-flight, denied by one of the eight
   typed deny reasons, failed by one of the five typed failure
   reasons, or inbound callback received or refused)?

Generic prose such as "network error", "request failed", "proxy
issue", or "try again" is forbidden on these paths when a more
precise state is known.

## Frozen vocabularies

Every enum value below is reserved across this seed, its schema
companion, and its matrix. Adding a value is **additive-minor** and
requires a `schema_version` bump plus updates to the schema, the
matrix, and this document in the same change. Repurposing a value is
**breaking** and requires a new decision row in
`artifacts/governance/decision_index.yaml` co-signed by
`security_trust_review` and `product_scope_review`.

### Component class (21 values)

`desktop_shell_component`, `cli_headless_component`,
`update_client_component`, `extension_registry_component`,
`extension_host_component`, `ai_broker_component`,
`docs_pack_component`, `policy_distribution_component`,
`telemetry_sink_component`, `remote_tunnel_component`,
`remote_ssh_component`, `remote_agent_component`,
`managed_control_plane_component`, `git_vcs_component`,
`notebook_kernel_component`, `debugger_symbol_component`,
`api_client_component`, `mirror_refresh_component`,
`browser_handoff_launcher_component`,
`provider_callback_receiver_component`, and
`heuristic_unknown_component`.

`heuristic_unknown_component` is a repair-only placeholder admissible
only while a new component is being seated; it MUST resolve to one
of the other classes before any egress proceeds.

### Permission class (16 values)

`core_update`, `registry_read`, `registry_write`,
`ai_context_egress`, `ai_tool_egress`, `docs_fetch`,
`remote_connect`, `vcs_network`, `extension_network`,
`policy_refresh`, `telemetry_send`, `tunnel_publish`,
`provider_callback_inbound`, `mirror_refresh`,
`browser_handoff_launch`, and `api_client_request`.

These re-export the Technical Architecture Appendix G matrix rows and
extend them with `policy_refresh`, `telemetry_send`,
`provider_callback_inbound`, `mirror_refresh`,
`browser_handoff_launch`, and `api_client_request` so mirror,
policy, telemetry, callback, handoff, and first-class HTTP / GraphQL
client paths all land in the same vocabulary.

### Target class (19 values)

`local_host_target`, `device_local_loopback_target`,
`remote_ssh_target`, `remote_agent_target`,
`managed_workspace_target`, `connected_provider_target`,
`system_browser_target`, `embedded_webview_target`,
`mirror_endpoint_target`, `private_registry_target`,
`public_registry_target`, `ai_provider_endpoint_target`,
`docs_mirror_target`, `policy_endpoint_target`,
`update_feed_target`, `telemetry_sink_target`,
`tunnel_exposed_target`, `bridged_helper_target`, and
`unknown_target`.

Execution targets cross-walk with the action-origin-target-labels
matrix; network-only targets (mirror endpoint, private / public
registry, AI provider endpoint, docs mirror, policy endpoint, update
feed, telemetry sink) are defined here so surfaces never need a
synonymous term.

### Auth mode class (11 values)

`no_auth_public`, `api_key_handle`, `oauth_delegated_handle`,
`device_code_handle`, `mtls_client_cert`, `ssh_host_key_pinned`,
`system_browser_callback_token`, `org_ca_bundle_pinned`,
`managed_service_identity`, `step_up_authenticator_required`, and
`auth_unknown`.

Every credential form rides on an opaque handle ref. Raw tokens, raw
passwords, raw basic-auth strings, raw refresh tokens, raw client
certificates, and raw SSH keys never cross this boundary.

### Route class (12 values)

`direct_local_loopback_route`, `direct_target_local_route`,
`system_proxy_route`, `manual_proxy_route`, `pac_resolved_route`,
`environment_proxy_route`, `mirror_proxy_route`,
`managed_relay_route`, `remote_target_originated_route`,
`browser_system_route`, `air_gap_no_network_route`, and
`heuristic_unknown_route`.

The four proxy route values correspond one-to-one with the four
non-direct proxy-resolution modes. `remote_target_originated_route`
names traffic that originated on the remote target (not the client)
so remote-mode attributions make egress origin explicit.

### Egress class (5 values)

`local_only`, `target_local`, `org_approved_external`,
`public_internet`, and `deny_all`.

- **`local_only`** — device-local traffic (loopback, process-local
  RPC, device-local mirror).
- **`target_local`** — traffic local to a remote target (SSH host,
  remote agent, managed workspace, devcontainer) that does not
  traverse the public internet.
- **`org_approved_external`** — traffic to a customer- or admin-
  approved external endpoint. Enterprise mirrors, private registries,
  managed AI gateways, managed control planes, and customer-operated
  telemetry sinks resolve here.
- **`public_internet`** — traffic to any endpoint outside the
  org-approved allowlist. Public package mirrors, vendor-public AI
  endpoints, public docs mirrors, tunnel-exposed routes, public
  provider callbacks, and unrestricted extension egress resolve here.
- **`deny_all`** — refusal class used by deny-all profiles or
  offline enforcement. Any outbound attempt MUST collapse to an
  `outbound_denied_*` outcome.

### Proxy resolution mode (6 values)

`system_proxy`, `pac_proxy`, `environment_proxy`, `manual_proxy`,
`direct_no_proxy`, and `proxy_unknown`.

Desktop and CLI / headless MUST resolve identically on a given
platform profile. A CLI bypass (for example, "CLI does not use the
system proxy") is not admissible.

### Trust-store source (5 values)

`os_trust_store`, `os_trust_store_plus_org_ca_bundle`,
`pinned_control_plane_trust_only`, `air_gap_offline_trust_root`, and
`trust_store_unknown`.

`os_trust_store_plus_org_ca_bundle` is the enterprise CA-bundle
overlay. `pinned_control_plane_trust_only` is the narrowed path for
the managed control plane and policy endpoints; the set of pinned
trust roots is disclosed by opaque ref. `air_gap_offline_trust_root`
is the offline-bundle-derived trust root used in air-gapped profiles.

### SSH host-key provenance (6 values)

`known_hosts_entry_trusted`, `first_use_trust_on_first_use`,
`pinned_by_policy`, `admin_policy_strict_required`,
`mismatched_rejected`, and `not_applicable_non_ssh`.

Enterprise and managed profiles typically forbid
`first_use_trust_on_first_use`. `mismatched_rejected` is the only
admissible provenance under an
`outbound_failed_ssh_host_key_mismatch` outcome.

### Mirror route class (6 values)

`no_mirror_direct_origin`, `customer_operated_mirror_active`,
`vendor_published_mirror_active`,
`offline_bundle_derived_mirror_active`,
`mirror_only_no_direct_allowed`, and `mirror_not_applicable`.

Mirror snapshot refs cross-walk with the locality-and-continuity
seed's `mirror_continuity_fields.mirror_snapshot_id`.

### Offline / deny-all state (7 values)

`online_live_allowed`, `online_mirror_only`,
`offline_grace_preserved`, `offline_air_gapped`,
`deny_all_enforced`, `network_disabled_by_user`, and
`network_degraded_heuristic`.

### Outcome class (18 values)

`outbound_allowed_completed`, `outbound_allowed_in_flight`,
`outbound_allowed_completed_with_warning`,
`outbound_denied_by_policy`, `outbound_denied_by_user_setting`,
`outbound_denied_by_offline_mode`,
`outbound_denied_by_egress_class_boundary`,
`outbound_denied_by_permission_missing`,
`outbound_denied_by_trust_failure`,
`outbound_denied_by_proxy_auth_required`,
`outbound_denied_by_deny_all_profile`,
`outbound_failed_transport_error`,
`outbound_failed_host_unreachable`,
`outbound_failed_tls_verification`,
`outbound_failed_ssh_host_key_mismatch`,
`outbound_failed_unknown`, `inbound_callback_received`, and
`inbound_callback_refused`.

### Deny-reason class (22 values)

`admin_policy_denied`, `user_setting_denied`, `offline_mode_denied`,
`air_gap_profile_denied`, `egress_class_boundary_denied`,
`permission_not_granted`, `permission_floor_blocked`,
`extension_permission_missing`,
`extension_network_capability_revoked`, `approval_ticket_missing`,
`approval_ticket_expired`, `step_up_required_not_satisfied`,
`trust_store_verification_failed`, `org_ca_bundle_not_loaded`,
`pinned_control_plane_trust_mismatch`,
`ssh_host_key_strict_mode_rejected`, `proxy_auth_required`,
`proxy_auth_denied`, `proxy_unreachable`,
`deny_all_profile_enforced`, `region_pinned_target_out_of_region`,
`freshness_floor_unmet`, and `deny_reason_unknown_requires_review`.

Every `outbound_denied_*` outcome MUST carry a typed
`deny_reason_detail` block naming one of these classes, a
reviewable deny-reason sentence, and (optionally) an opaque
`policy_rule_ref` or `user_setting_ref`.

### Redaction class (4 values)

`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, and `signing_evidence_only`.

## The inspectable current transport-posture object

`transport_posture` is the single inspectable object that status
items, diagnostics cards, support exports, admin-audit packets, and
boundary-manifest rows reference without recomputing field names. It
carries:

- `proxy_resolution_mode`
- `effective_proxy_ref` (opaque)
- `trust_store_source`
- `org_ca_bundle_fingerprint` (opaque, non-null whenever
  `trust_store_source` is `os_trust_store_plus_org_ca_bundle`)
- `pinned_control_plane_trust_refs` (opaque)
- `ssh_host_key_default_provenance`
- `mirror_route_class`
- `mirror_snapshot_ref` (opaque)
- `active_deployment_profile`
- `active_identity_mode`
- `active_policy_bundle_ref` (opaque)
- `policy_epoch_ref` (opaque)
- `offline_or_deny_all_state`
- `offline_since_at`
- `posture_note` (reviewable sentence)
- `captured_at`

Every `network_attribution_record` embeds one
`transport_posture_at_event` capturing the posture at the instant of
the event. Status items and diagnostics cards read the same object
from the live transport-governance layer; support exports serialize
the exact same shape. One object, one vocabulary, one redaction
class.

### Sample posture

A worked example capturing proxy mode, trust-store source, mirror
route, active policy / deployment profile, and the offline / deny-all
state:

```json
{
  "proxy_resolution_mode": "pac_proxy",
  "effective_proxy_ref": "effective_proxy.corp_primary",
  "trust_store_source": "os_trust_store_plus_org_ca_bundle",
  "org_ca_bundle_fingerprint": "org_ca_bundle:fp_corp_primary_2026_04",
  "pinned_control_plane_trust_refs": [
    "pinned_cp_trust.managed_control_plane"
  ],
  "ssh_host_key_default_provenance": "admin_policy_strict_required",
  "mirror_route_class": "customer_operated_mirror_active",
  "mirror_snapshot_ref": "mirror_snapshot.corp_primary_2026_04_23",
  "active_deployment_profile": "enterprise_online",
  "active_identity_mode": "managed_workspace",
  "active_policy_bundle_ref": "policy_bundle.corp_primary",
  "policy_epoch_ref": "policy_epoch.managed.windowed.2026_04_23",
  "offline_or_deny_all_state": "online_live_allowed",
  "offline_since_at": null,
  "posture_note": "Enterprise online profile; PAC-resolved proxy; org CA bundle overlay; mirror-backed registry and docs.",
  "captured_at": "2026-04-23T12:00:00Z"
}
```

The same object shape is reused unchanged on every deployment profile
(`individual_local`, `self_hosted`, `enterprise_online`,
`air_gapped`, `managed_cloud`) and every identity mode
(`account_free_local`, `self_hosted_org`, `managed_workspace`). A
deny-all posture captures the same object with
`offline_or_deny_all_state = "deny_all_enforced"` and the schema
enforces collapse to `outbound_denied_*` outcomes.

## Per-component declarations and permission gates

Any component declaring network capability MUST publish:

1. **Why it needs network.** A short product-term sentence ("fetch
   signed update manifests", "refresh docs pack", "publish branch to
   remote", "stream AI completion"). This sentence is surfaced on
   the attribution record as `reason_sentence` and on the component
   registry.
2. **Which permission class it claims.** One of the 16 values in the
   frozen permission vocabulary. Component-local synonyms are not
   admissible.
3. **Which admissible target classes, egress classes, auth modes,
   and route classes it may use.** The
   `artifacts/network/egress_classes.yaml` matrix enumerates them by
   permission class.
4. **What happens when egress is denied.** Which deny-reason classes
   the component may mint, and what the user-facing recovery hint
   says. The schema requires a typed `deny_reason_detail` block on
   every `outbound_denied_*` outcome.

## Proxy resolution rules

Every network-capable surface uses exactly one proxy-resolution mode
at a time. Resolution order is:

1. `manual_proxy` set by admin policy (a policy-bound manual proxy
   overrides both user-entered manual proxy and environment).
2. `manual_proxy` set by the user (Aureline settings).
3. `environment_proxy` (`HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY`,
   `NO_PROXY`) when the process environment carries them.
4. `pac_proxy` when a PAC / auto-config source is active on the OS.
5. `system_proxy` when the OS exposes a native system proxy.
6. `direct_no_proxy` otherwise.

If resolution is in flight, `proxy_unknown` is used as a **repair-
only** placeholder. Surfaces MUST display a reviewable sentence
explaining the unknown state; silently falling back to direct is
non-conforming.

Desktop and CLI / headless MUST resolve identically on a given
platform profile. A CLI bypass is not admissible. The claimed
platform sources per OS are recorded in
`artifacts/network/egress_classes.yaml` under `proxy_resolution_modes`
and cross-walk with the desktop platform conformance matrix row for
proxy / trust stores.

The effective proxy MUST be disclosed per target / profile via the
opaque `effective_proxy_ref`; raw URLs and raw proxy credentials never
appear. Status items surface the opaque ref next to a reviewable
sentence; admin exports carry the same ref under the
`operator_only_restricted` redaction class when policy so requires.

## TLS, OS trust-store, and org CA-bundle rules

Trust resolution order:

1. `pinned_control_plane_trust_only` for managed-control-plane and
   policy endpoints configured to require pinning. The pinned trust
   roots are disclosed by opaque ref on every attribution.
2. `os_trust_store_plus_org_ca_bundle` when an enterprise / org CA
   bundle is loaded. The bundle's fingerprint is disclosed by opaque
   fingerprint on every attribution.
3. `os_trust_store` as the platform default.
4. `air_gap_offline_trust_root` for air-gapped profiles.
5. `trust_store_unknown` only as a repair-only placeholder; any
   outbound request MUST be gated by an explicit user or policy
   decision before transitioning off this placeholder.

TLS verification failures collapse to
`outbound_denied_by_trust_failure` (when policy refuses continuation)
or `outbound_failed_tls_verification` (when the attempt progressed
but failed verification). Silent insecure fallback ("continue without
verification") is non-conforming across every component class.

Raw PEM bodies, raw CA bundle bytes, raw certificate chains, and raw
private-key material never cross the attribution boundary; the record
carries opaque fingerprints only.

## SSH host-key provenance rules

Every SSH attribution MUST state one of the six provenance classes:

- `known_hosts_entry_trusted` — already trusted via a `known_hosts`
  entry. Fingerprint disclosed opaquely.
- `first_use_trust_on_first_use` — TOFU. Admissible only when the
  active trust-on-first-use policy posture permits it on the active
  deployment profile. Enterprise / managed profiles typically forbid
  TOFU.
- `pinned_by_policy` — pinned by admin policy or policy-bundle
  narrowing rule.
- `admin_policy_strict_required` — strict mode: every new host
  requires an explicit admin-supplied pin or a vetted `known_hosts`
  entry.
- `mismatched_rejected` — fingerprint did not match. The attribution
  MUST surface `outbound_failed_ssh_host_key_mismatch` and the
  schema enforces that pairing.
- `not_applicable_non_ssh` — for non-SSH attributions.

Raw SSH host keys never cross the boundary; `ssh_host_key_fingerprint`
is an opaque fingerprint.

## Pinned-control-plane trust

Managed control-plane and policy endpoints MAY carry pinned trust
roots distinct from the OS trust store and org CA bundle. Attribution
records under `trust_store_source = pinned_control_plane_trust_only`
MUST list a non-empty `pinned_control_plane_trust_refs`. A pinned
trust mismatch collapses to `pinned_control_plane_trust_mismatch`
deny reason under `outbound_denied_by_trust_failure` (or
`outbound_failed_tls_verification` when the attempt proceeded but
failed verification).

## Egress classes

The five egress classes are the closed, ordered set Aureline surfaces
reason about:

| Egress class | Meaning | Proxy applicable | Trust applicable | Typical routes |
|---|---|---|---|---|
| `local_only` | Device-local traffic | No (direct loopback or device-local mirror) | No (off the wire) | `direct_local_loopback_route`, `mirror_proxy_route` |
| `target_local` | Remote-target-local traffic | Yes on target-originated path | Yes | `direct_target_local_route`, `remote_target_originated_route`, `managed_relay_route`, `mirror_proxy_route` |
| `org_approved_external` | Customer- or admin-approved external endpoint | Yes | Yes (often org CA bundle or pinned control-plane) | All four proxy routes, `mirror_proxy_route`, `managed_relay_route`, `direct_target_local_route`, `browser_system_route` |
| `public_internet` | Any endpoint outside the org-approved allowlist | Yes | Yes (OS trust or org CA bundle) | All four proxy routes, `mirror_proxy_route`, `browser_system_route` |
| `deny_all` | Refused | N/A | N/A | `air_gap_no_network_route` |

Per-profile default egress by permission class is published in
`artifacts/network/egress_classes.yaml` under `permission_rows`. The
defaults are frozen: a component cannot silently raise its egress
class above the per-profile default.

## Offline / degraded and mirror-support rules

Desktop and CLI / headless share one offline / degraded vocabulary:

- `online_live_allowed` — nominal online posture. Direct-origin and
  mirror paths both admissible where policy permits.
- `online_mirror_only` — direct origin is refused by policy; every
  admissible path goes through a mirror. Attribution route MUST be
  `mirror_proxy_route` and `egress_class` MUST NOT be
  `public_internet` unless the mirror itself lives on the public
  internet.
- `offline_grace_preserved` — bounded offline grace window during
  which cached posture remains authoritative. Policy refresh, docs
  refresh, and update checks defer to the freshness-floor rules from
  the policy and deployment seeds.
- `offline_air_gapped` — air-gapped deployment-profile posture. The
  schema enforces that `egress_class` MUST NOT be `public_internet`
  and the route MUST be `direct_local_loopback_route`,
  `direct_target_local_route`, `mirror_proxy_route`,
  `air_gap_no_network_route`, or `remote_target_originated_route`.
- `deny_all_enforced` — refuse-all-network posture (user, policy, or
  emergency-action). The schema enforces collapse to an
  `outbound_denied_*` outcome.
- `network_disabled_by_user` — user-initiated global disable.
- `network_degraded_heuristic` — repair-only placeholder while
  degraded state is being classified.

Mirror routes carry an opaque `mirror_endpoint_ref` and an opaque
`mirror_snapshot_ref` that cross-walks with the
`local_core_continuity_packet_record.mirror_continuity_fields`
shape. A mirror past its extended-window freshness class surfaces as
`outbound_allowed_completed_with_warning` (when policy permits
degraded continuation) or `outbound_denied_by_policy` under
`freshness_floor_unmet` (when policy refuses).

## The network attribution record

`network_attribution_record` is the one attribution shape every
network-capable surface emits. One record per attempt (outbound-
allowed, outbound-denied, outbound-failed, inbound-callback-
received, inbound-callback-refused). Fields:

- Identity: `attribution_id`, `invocation_session_id`,
  `component_class`, `component_instance_ref`,
  `permission_class`, `permission_grant_ref`.
- Target: `target_class`, `target_identity_ref`, `target_scope_ref`.
- Auth: `auth_mode`, `auth_handle_ref`.
- Route: `route_class`, `egress_class`.
- Proxy: `proxy_resolution` (mode, effective proxy ref, resolution
  source note, PAC script fingerprint).
- Trust: `trust_posture` (trust-store source, org CA bundle
  fingerprint, pinned control-plane trust refs, SSH host-key
  provenance, SSH host-key fingerprint, TLS server cert chain
  fingerprint, trust posture note).
- Mirror: `mirror_route` (class, endpoint ref, snapshot ref,
  freshness class).
- Reason: `reason_sentence` (reviewable).
- Outcome: `outcome_class`, `outcome_note`, `deny_reason_detail`.
- Timing: `request_started_at`, `request_completed_at`,
  `bytes_sent_estimate`, `bytes_received_estimate`.
- Authority linkage: `linked_approval_ticket_ref`,
  `linked_browser_handoff_packet_ref`,
  `linked_authority_ticket_ref`.
- Posture snapshot: `transport_posture_at_event` (the inspectable
  object above).
- Redaction: `redaction_class`, `export_safe`.
- Narrative refs.

Schema-level invariants:

1. Every `outbound_denied_*` outcome MUST carry a typed
   `deny_reason_detail` block. Generic "blocked" prose is
   non-conforming.
2. When `transport_posture_at_event.offline_or_deny_all_state` is
   `deny_all_enforced`, `outcome_class` MUST be one of the eight
   `outbound_denied_*` values and `egress_class` MUST be
   `local_only`, `target_local`, or `deny_all`.
3. When the active deployment profile is `air_gapped`,
   `egress_class` MUST NOT be `public_internet` and the route MUST
   be one of `direct_local_loopback_route`,
   `direct_target_local_route`, `mirror_proxy_route`,
   `air_gap_no_network_route`, or `remote_target_originated_route`.
4. `extension_host_component` MUST carry a non-null
   `permission_grant_ref` on any egress above `local_only`.
   Extension network access is permissioned and attributable, not
   ambient.
5. `remote_connect` permission MUST resolve to one of
   `remote_ssh_target`, `remote_agent_target`,
   `managed_workspace_target`, `tunnel_exposed_target`, or
   `bridged_helper_target`.
6. `tunnel_publish` permission MUST run under `public_internet`
   egress and a `tunnel_exposed_target`, with a
   non-empty `reason_sentence`.
7. `auth_mode = ssh_host_key_pinned` MUST resolve to a
   `remote_ssh_target` and a concrete `ssh_host_key_provenance`
   (never `not_applicable_non_ssh`).
8. `outbound_failed_ssh_host_key_mismatch` MUST carry
   `ssh_host_key_provenance = mismatched_rejected`.
9. Any terminal outcome (not `outbound_allowed_in_flight`) MUST
   carry a non-null `request_completed_at`.

## Desktop vs. CLI / headless parity

Desktop and CLI / headless consume this seed identically:

- Both resolve proxy via the same resolution order under the same
  `proxy_resolution_mode_class`.
- Both evaluate trust via the same `trust_store_source_class`, the
  same org CA-bundle fingerprint disclosure, and the same pinned
  control-plane trust refs.
- Both resolve SSH host-key provenance via the same six-value class.
- Both emit `network_attribution_record` entries of the exact same
  shape.
- Both read the same `transport_posture` object for status items
  (desktop) and diagnostics lines (CLI).
- Both refuse silent egress: a CLI-only "ignore proxy" override is
  non-conforming.

Extension hosts and AI brokers ride on this same layer; recipe,
remote, automation, and collaboration paths inherit the same
attribution shape.

## Redaction and export posture

Every attribution record carries a `redaction_class` and an
`export_safe` flag. The four redaction classes
(`metadata_safe_default`, `operator_only_restricted`,
`internal_support_restricted`, `signing_evidence_only`) re-export the
support / release vocabulary. Raw URLs, raw host names, raw IPs, raw
SSH host keys, raw proxy credentials, raw CA bundle bytes, raw
tokens, raw request bodies, raw response bodies, raw PAC script
bodies, raw mirror URLs, and raw remote stack traces MUST NOT appear
on this boundary. Diagnostics cards, status items, and support bundle
bodies reference the same opaque refs and opaque fingerprints the
record exposes.

## Out of scope at this revision

- Implementing the actual transport stack, HTTP client, TLS
  verifier, SSH client, or proxy resolver.
- Live enterprise PKI integration (EST, ACME, SCEP, or vendor-
  specific PKI).
- Live OIDC / SCIM / managed-service broker implementation.
- The full set of ecosystem deny-reason mappings for every provider
  (follow-on work will extend `deny_reason_detail_class` with
  provider-specific sub-codes where warranted; those additions are
  additive-minor).
- Live telemetry or support-bundle serialization format; those are
  consumed separately and reference the object shapes here.

## Relationship to other seeds and ADRs

- ADR-0001 — this seed embeds `identity_mode` on every
  `transport_posture`.
- ADR-0007 — credentials referenced via `auth_handle_ref` are
  secret-broker handles under ADR-0007's redaction discipline.
- ADR-0010 — browser-handoff launches and provider-callback inbounds
  cite ADR-0010 packet refs on
  `linked_browser_handoff_packet_ref`; external-mutating requests
  cite approval-ticket refs on `linked_approval_ticket_ref`.
- Runtime authority contract — external-mutating network
  attributions link to the authority ticket that admitted the effect
  via `linked_authority_ticket_ref`; the external-effect lineage
  `preview_fingerprint` and `target_identity_fingerprint` consume
  opaque refs produced here.
- Locality-and-continuity seed — mirror_snapshot_ref cross-walks
  with `mirror_continuity_fields.mirror_snapshot_id`; the
  offline / deny-all state vocabulary aligns with the data-plane
  continuity state vocabulary.
- Policy-bundle / entitlement seed — `active_policy_bundle_ref` and
  `policy_epoch_ref` are the opaque refs into the signed policy
  bundle governing the current transport posture.
- Desktop platform conformance matrix — per-OS proxy, trust-store,
  SSH, and secret-store conformance rows cross-walk with the
  proxy-resolution-mode and trust-store-source vocabularies.

## Adding or changing vocabulary

Adding a value to any vocabulary is additive-minor:

1. Update the schema enum in
   `schemas/network/network_attribution_record.schema.json`.
2. Update the matrix in `artifacts/network/egress_classes.yaml`.
3. Update this document.
4. Add or update a fixture under
   `fixtures/network/connectivity_cases/` exercising the new value.
5. Bump `schema_version`.

Repurposing an existing value is breaking:

1. Open a decision row in
   `artifacts/governance/decision_index.yaml` co-signed by
   `security_trust_review` and `product_scope_review`.
2. Deprecate the old value and introduce the new value through an
   additive-minor landing.
3. Support export rewriters and support bundles translate across
   the deprecation window.
