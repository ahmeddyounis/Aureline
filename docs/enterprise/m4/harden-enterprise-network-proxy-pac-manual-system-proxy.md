# Harden enterprise network proxy — PAC, manual, system proxy, custom CA, client certificate, and SSH trust

This lane makes every enterprise proxy route — system, PAC, manual,
policy-pinned, mirror-only, and offline — visible and verifiable enough that
product, security review, support export, and admin surfaces can all explain:
which proxy route is active and why, what trust material is declared (custom
CA, client certificate, SSH host key), what TLS verification posture is in
force, and whether local-only fallback routing is explicitly documented for
enterprise-only routes. The runtime owner is
`aureline_policy::harden_enterprise_network_proxy_pac_manual_system_proxy`.

The packet does **not** re-derive raw proxy configurations, raw credentials,
raw CA certificates, raw PAC script content, or raw SSH key material. The
upstream `aureline_auth::network_trust` beta audit remains canonical for its
own slice. This packet re-exports those qualification tokens verbatim and adds
the proxy-hardening invariants needed for a single evidence packet.

## Contract

For the stable claim to hold, **all nine** of the following conditions must be
verified simultaneously:

1. **Upstream network trust clean** — `aureline_auth::network_trust::audit_network_trust_beta_rows` returns zero defects for the embedded network-trust page.
2. **All six proxy routes covered** — at least one row exists for each of: `system`, `pac`, `manual`, `policy_pinned`, `mirror_only`, and `offline`.
3. **Raw secret material excluded** — every row carries `raw_secret_or_private_material_excluded: true`; no credential bodies, private keys, PAC script content, or CA certificates cross this boundary.
4. **Selector reasons declared** — every row carries a non-empty `selector_reason_token` identifying why this route was selected.
5. **Enterprise routes declare local fallbacks** — rows whose `proxy_route` is `policy_pinned`, `mirror_only`, or `offline` must carry a non-empty `local_only_fallback_route_token` and a human-readable `fallback_condition_label`.
6. **Bootstrap credentials declared** — every row carries at least one `BootstrapCredentialDeclaration`; the minimum acceptable declaration is `none_required`.
7. **TLS verification posture declared** — every row carries a non-empty `tls_verification_posture_token`.
8. **Local-core continuity explicit** — `mirror_only` and `offline` rows carry `local_core_continuity_explicit: true`.
9. **Managed attribution declared** — `policy_pinned` and `mirror_only` rows carry a non-empty `managed_attribution_ref`.

## Required behavior

`validate_harden_enterprise_network_proxy_page` rejects a page when its `defects` list is non-empty.

`audit_harden_enterprise_network_proxy_page` runs the combined check and returns a typed
`Vec<HardenEnterpriseNetworkProxyDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row with `raw_secret_or_private_material_excluded: false` (narrow reason:
  `raw_secret_or_private_material_exposed`). The function returns immediately
  with this single defect and skips all other checks.

A missing required proxy route narrows to `Preview` rather than `Beta` because
the coverage gap prevents any verifiable claim for that route class.

An empty `selector_reason_token` on any row narrows to `Beta`.

## Proxy routes

| Route token | Precedence rank | Description |
| --- | --- | --- |
| `policy_pinned` | 1 (highest) | Route locked by an admin policy override; cannot be bypassed by user settings. |
| `manual` | 2 | Explicit proxy address entered manually by the user or admin. |
| `process_environment` | 3 | Proxy resolved from process-level environment variables (`HTTP_PROXY`, `HTTPS_PROXY`, etc.). |
| `pac` | 4 | Proxy resolved by evaluating a PAC (Proxy Auto-Config) script. |
| `system` | 5 | Proxy resolved from the OS system proxy settings. |
| `direct_no_proxy` | 6 (lowest) | No proxy; direct connections only (offline or local-only fallback). |

The six required rows in the proof packet correspond to: `system`, `pac`,
`manual`, `policy_pinned`, `mirror_only`, and `offline`. All six must be
covered for a stable claim.

## Bootstrap credential kinds

| Kind token | Description |
| --- | --- |
| `none_required` | No additional trust material required; system defaults suffice. |
| `tls_system_ca` | Trust anchored to the OS platform CA bundle. |
| `custom_ca` | One or more custom CA certificates declared by admin policy. |
| `client_certificate` | A client TLS certificate is presented for mutual authentication. |
| `ssh_host_key` | An SSH host-key fingerprint is declared for Git-over-SSH trust. |
| `admin_signed_proxy_credential` | A credential signed by an enterprise admin authority is required. |

## TLS verification posture

| Token | Description |
| --- | --- |
| `full_verification` | Standard TLS verification against the declared CA chain. |
| `custom_ca_verification` | TLS verification against a custom CA bundle declared by admin policy. |
| `not_applicable` | TLS is not applicable for this route (e.g., SSH or direct/offline). |

## Local-only fallback

Enterprise-only routes (`policy_pinned`, `mirror_only`, `offline`) must
explicitly declare a `local_only_fallback_route_token` — the route that takes
over when connectivity to the primary managed endpoint is unavailable — and a
human-readable `fallback_condition_label` describing the trigger condition.
This prevents silent fail-open routing by requiring explicit documentation of
the continuity path.

## Boundary

The following material stays outside this packet's support boundary:

- Raw proxy hostnames, raw PAC script bodies, raw environment variable values.
- Raw CA certificates, raw private keys, raw client-certificate PEM blobs.
- Raw SSH host-key material.
- Raw credential tokens, bearer tokens, or proxy authentication secrets.
- Raw admin policy rule bodies or raw policy bundle content.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

The seeded proof packet is `seeded_harden_enterprise_network_proxy_page()` in
[`/crates/aureline-policy/src/harden_enterprise_network_proxy_pac_manual_system_proxy/mod.rs`](../../../crates/aureline-policy/src/harden_enterprise_network_proxy_pac_manual_system_proxy/mod.rs).

That function is the single inspectable record for this lane. Dashboards,
Help/About surfaces, and support exports should ingest it rather than cloning
status text or maintaining parallel proxy-configuration checks.

## Canonical paths

- Runtime owner: `aureline_policy::harden_enterprise_network_proxy_pac_manual_system_proxy`
- Artifact: `artifacts/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md`
- Fixtures: `fixtures/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy/`
- Schema: `schemas/enterprise/harden-enterprise-network-proxy-pac-manual-system-proxy.schema.json`
