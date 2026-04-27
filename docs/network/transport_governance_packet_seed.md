# Transport governance packet seed

This packet seed freezes the decision vocabulary that sits above the
existing transport posture and network attribution records. It exists so
updates, registries, docs packs, AI providers, request-runtime surfaces,
remote attach, browser handoff, CLI, extension hosts, support exports, and
release evidence all explain network behavior with one packet family.

The packet is not a transport stack implementation. It is the shared shape
every future transport stack must emit before a network-capable surface
claims a route, refuses a route, defers a safe offline action, or explains a
trust failure.

## Companion Artifacts

- [`/docs/network/transport_governance_seed.md`](transport_governance_seed.md)
  defines the underlying proxy, trust, SSH host-key, mirror, egress, posture,
  and attribution vocabulary.
- [`/schemas/network/transport_decision.schema.json`](../../schemas/network/transport_decision.schema.json)
  defines the machine-readable `transport_decision_record`.
- [`/artifacts/network/proxy_lab_matrix.yaml`](../../artifacts/network/proxy_lab_matrix.yaml)
  defines the enterprise proxy, CA, PAC, manual proxy, strict SSH, and client
  certificate lab rows for claimed profiles.
- [`/fixtures/network/transport_cases/`](../../fixtures/network/transport_cases/)
  contains worked decision records that project into summary strips, endpoint
  rows, certificate cards, and denied-attempt history rows.
- [`/artifacts/network/egress_classes.yaml`](../../artifacts/network/egress_classes.yaml)
  remains the permission and egress matrix consumed by the packet.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 5.42, network, proxy, certificates, and
  transport-governance architecture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix G, network
  and transport policy matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` Appendices CX, CY, and CZ.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix BS, transport, proxy,
  certificate, and mirror-state templates.

If this packet disagrees with those source specs, the source specs win and
this packet, schema, matrix, and fixtures must update together.

## Packet Boundary

Every network-capable surface emits or consumes a `transport_decision_record`
before it presents network state as user-visible truth. A decision record
answers:

1. Which surface attempted or requested the network action.
2. Where egress originates.
3. Which endpoint class and egress class apply.
4. Which proxy route was selected or refused.
5. Which trust source, SSH host proof, or client-certificate handle was used.
6. Whether the endpoint is public, org-approved, mirrored, local, or not
   routed.
7. Which outcome code applied.
8. Whether any bypass or fallback is forbidden.
9. Which repair hints can be offered without widening policy or trust.

Generic text such as "network error", "proxy issue", "certificate problem",
"offline", or "try again" is non-conforming when a typed code is known.

## Endpoint Classes

The packet uses the existing `network_target_class` vocabulary as endpoint
classes:

- local/device: `local_host_target`, `device_local_loopback_target`.
- remote: `remote_ssh_target`, `remote_agent_target`,
  `managed_workspace_target`, `tunnel_exposed_target`,
  `bridged_helper_target`.
- browser/provider: `system_browser_target`, `embedded_webview_target`,
  `connected_provider_target`.
- ecosystem/service: `mirror_endpoint_target`, `private_registry_target`,
  `public_registry_target`, `ai_provider_endpoint_target`,
  `docs_mirror_target`, `policy_endpoint_target`, `update_feed_target`,
  `telemetry_sink_target`.
- repair-only: `unknown_target`.

No surface may mint a parallel endpoint taxonomy such as "marketplace host",
"AI URL", "docs URL", or "remote box" in decision payloads. Friendly labels may
resolve locally from opaque refs, but packet data stays on the frozen endpoint
classes.

## Origin And Egress Vocabulary

`origin_scope` names where the decision is made or where traffic originates:

- `desktop_client`
- `headless_runner`
- `extension_host`
- `remote_target`
- `managed_service`
- `system_browser`
- `local_helper`

`egress_class` uses the existing five-value set:

- `local_only`
- `target_local`
- `org_approved_external`
- `public_internet`
- `deny_all`

Remote mode must preserve both fields. A request that originates from the
remote target is not the same as a request that originates from the desktop
client, even when both are triggered by the same command.

## Proxy Precedence

The packet records the effective proxy mode through
`chosen_route.proxy_resolution_mode`. Resolution precedence is:

1. Admin policy manual proxy.
2. User manual proxy.
3. Environment proxy (`HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY`, `NO_PROXY`).
4. PAC proxy.
5. System proxy.
6. Direct no-proxy when policy permits direct egress.

The four proxy routes are `manual_proxy_route`, `environment_proxy_route`,
`pac_resolved_route`, and `system_proxy_route`. `direct_no_proxy` is not a
fallback; it is a selected posture that must be allowed by the active profile,
policy bundle, and egress class. `proxy_unknown` is repair-only and must never
silently degrade to direct connection.

Desktop and CLI/headless flows must resolve proxy mode identically on the same
profile. Extension hosts, AI broker paths, docs packs, update clients, remote
connectors, and request-runtime clients must use the same decision stack.

## Trust Store Layering

`trust_evidence.trust_store_source` uses the existing trust-store source
vocabulary:

- `pinned_control_plane_trust_only` for policy and managed control-plane paths
  that require pins.
- `os_trust_store_plus_org_ca_bundle` when an organization CA bundle is active.
- `os_trust_store` for platform-default trust.
- `air_gap_offline_trust_root` for signed offline bundles and air-gapped
  mirror transfer.
- `trust_store_unknown` only as a repair-only placeholder.

Trust failure states come from the TDD trust matrix:

- `ca_untrusted`
- `hostname_mismatch`
- `bundle_stale`
- `pin_mismatch`
- `rotation_required`
- `ssh_host_key_unknown`
- `ssh_host_key_mismatch`
- `client_certificate_required`
- `client_certificate_expired`
- `policy_blocked`
- `none`

Silent insecure TLS fallback is forbidden. A surface may not continue by
disabling verification, ignoring a pin, skipping the org CA bundle, or
presenting a generic error after the trust state is known.

## SSH Host Proof

SSH paths must carry both:

- `auth_mode = ssh_host_key_pinned`
- `trust_evidence.ssh_host_key_provenance`

Strict enterprise/self-hosted profiles use `admin_policy_strict_required` or
`pinned_by_policy`; trust-on-first-use is not admissible unless policy
explicitly allows it for the active profile. A mismatch uses
`mismatched_rejected`, `trust_failure_state = ssh_host_key_mismatch`, and the
decision code `deny_trust` or the attribution outcome
`outbound_failed_ssh_host_key_mismatch`, depending on whether refusal happened
before or during transport.

Raw SSH host keys never appear in the packet. Only opaque fingerprints are
allowed.

## Client Certificate Handles

Client-certificate paths use:

- `auth_mode = mtls_client_cert`
- `trust_evidence.client_certificate_state`
- `trust_evidence.client_certificate_handle_ref`
- `trust_evidence.client_certificate_fingerprint`

The client certificate is referenced by handle only. Raw certificate material
and private keys never appear. `required_missing`, `expired`,
`handle_unavailable`, and `policy_blocked` map to `deny_trust` or
`unsupported_auth_mode` with repair hints that inspect certificate posture or
choose a supported auth path. No decision may downgrade from mTLS to bearer
token, public anonymous access, or direct public origin without a policy-visible
decision.

## Mirror And Public Labels

`mirror_public_label` tells every UI and support surface whether the decision
used:

- `public_origin`
- `org_approved_origin`
- `customer_operated_mirror`
- `vendor_published_mirror`
- `offline_bundle_mirror`
- `mirror_only_no_direct`
- `local_or_target_local`
- `no_route_selected`

Mirror-only profiles set `public_origin_allowed = false`. A stale mirror uses
the decision code `stale_mirror` and may only repair through `refresh_mirror`
or `use_approved_alternate_route` when policy allows. It may not retry against
the public origin silently.

Mirror snapshot refs and mirror endpoint refs are opaque. Raw mirror URLs and
customer hostnames never appear.

## Decision Codes

Transport decision results use the TDD outcome and denial vocabulary:

- `allow`
- `allow_mirror`
- `deny_policy`
- `deny_proxy_resolution`
- `deny_trust`
- `offline_deferred`
- `stale_mirror`
- `unsupported_auth_mode`

Denied decisions also carry `deny_reason_detail` when the existing attribution
vocabulary has a more specific deny reason, such as `admin_policy_denied`,
`proxy_unreachable`, `trust_store_verification_failed`,
`ssh_host_key_strict_mode_rejected`, `client_certificate_expired` via
`trust_failure_state`, `freshness_floor_unmet`, or
`deny_all_profile_enforced`.

## Deny-All Mode

When `offline_or_deny_all_state = deny_all_enforced`, every networked action
must collapse to an explicit denial:

- route class is `air_gap_no_network_route` unless the action is local-only.
- egress class is `deny_all` or a local/target-local class that policy still
  admits.
- decision code is `deny_policy`.
- attribution outcome is normally `outbound_denied_by_deny_all_profile`.
- no feature-specific retry loop may continue in the background.

Local editing, save, local search, local Git operations, local tasks, export,
and diagnostics continue where their own contracts allow them. Networked
surfaces must fail visibly and typed.

## Offline-Deferred Continuity

Only idempotent actions may use `offline_deferred`. The decision must carry:

- `deferred_action_safe = true`
- an opaque deferred queue ref
- an opaque retry posture ref
- a retry-condition sentence

Destructive or externally visible mutations cannot be deferred. If a queued
action would publish, delete, rotate trust, expose a tunnel, mutate a remote
target, or call an AI tool, it must fail typed or request fresh approval after
connectivity returns.

Offline-deferred records are visible in denied-attempt history and support
exports. They are not invisible retries.

## No-Bypass Rules

`no_bypass` freezes whether fallback is available. For claimed profiles,
`no_bypass_status = no_bypass_enforced` sets:

- `direct_fallback_allowed = false`
- `insecure_fallback_allowed = false`

Forbidden bypass classes:

- `silent_direct_origin_fallback`
- `insecure_tls_fallback`
- `proxy_resolution_skip`
- `cli_desktop_proxy_divergence`
- `extension_ambient_egress`
- `remote_origin_hidden`
- `public_origin_on_mirror_only_profile`
- `feature_specific_retry_under_deny_all`
- `destructive_offline_defer`
- `generic_network_error_when_typed_reason_known`

A published exception may exist only as an explicit exception ref with owner,
expiry, evidence, and release/support visibility. Hidden direct-connect or
insecure fallback behavior is never admissible for claimed enterprise,
mirror-only, deny-all, remote, AI, registry, docs, update, browser, or
request-runtime profiles.

## Projection Contract

The decision schema includes `projection_contract` so surfaces can render
without private vocabulary:

- Summary strips read proxy mode, trust source, mirror/public label, decision
  code, no-bypass status, and repair hints.
- Endpoint rows read component, permission, target, origin, egress, auth,
  route, outcome, and decision code.
- Certificate cards read trust source, SSH host proof, client certificate
  state, trust failure state, and repair hints.
- Denied-attempt history rows read decision time, component, target, failure
  reason, deny detail, no-bypass status, and repair hints.

Support exports, CLI explain output, and admin audit packets serialize the
same object. They may filter by redaction class, but they may not translate
reason codes or trust-state values into private enums.

## Release And Support Use

Release, support, and compatibility artifacts reference this packet family
whenever network behavior is part of a claim. A claim that says an enterprise
profile supports PAC, manual proxy, custom CA, strict SSH host proof, mTLS
client certificate, mirror-only, deny-all, or offline-deferred behavior must
cite:

- the transport decision schema version,
- the proxy lab matrix row,
- the fixture or lab run evidence,
- the no-bypass rule status,
- and the redaction posture used for support export.

This keeps enterprise transport behavior from diverging into per-feature
folklore as implementation lands.

