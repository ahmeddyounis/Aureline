# Hardened Enterprise Network Proxy — Proof Packet

- Packet: `policy:harden-enterprise-network-proxy:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:harden_enterprise_network_proxy:v1`
- Qualification: `stable` (derived, not asserted)
- Upstream network-trust defects: 0
- Proxy-harden defects: 0
- Withdrawn rows: 0
- Stable rows: all 6

## Lane coverage

| Row | Proxy route | Precedence rank | Selector reason | TLS posture | Credential kind(s) | Local fallback |
|---|---|---|---|---|---|---|
| `harden-network-proxy:system` | `system` | 5 — system | `os_system_proxy_active` | `full_verification` | `tls_system_ca` | — |
| `harden-network-proxy:pac` | `pac` | 4 — PAC script | `pac_script_evaluated` | `full_verification` | `tls_system_ca` | — |
| `harden-network-proxy:manual` | `manual` | 2 — manual | `admin_manual_entry` | `full_verification` | `tls_system_ca` | — |
| `harden-network-proxy:policy_pinned` | `policy_pinned` | 1 — policy-pinned | `managed_policy_pinned` | `custom_ca_verification` | `custom_ca`, `client_certificate`, `admin_signed_proxy_credential` | `manual` |
| `harden-network-proxy:mirror_only` | `mirror_only` | 1 — policy-pinned | `mirror_only_profile_active` | `custom_ca_verification` | `custom_ca` | `offline` |
| `harden-network-proxy:offline` | `offline` | 6 — direct/no-proxy | `offline_profile_active` | `not_applicable` | `none_required` | `offline` (self) |

## Evidence sources

- Upstream network-trust verifier:
  `security:network_trust_beta:v1`
  — `crates/aureline-auth/src/network_trust/mod.rs`

## Key invariants verified

1. The embedded `NetworkTrustBetaPage` audits with zero defects.
2. All six required proxy routes (`system`, `pac`, `manual`, `policy_pinned`, `mirror_only`, `offline`) have rows.
3. Every row carries `raw_secret_or_private_material_excluded: true`; no credential bodies, private keys, PAC script content, or CA certificates cross this boundary.
4. Every row names a non-empty `selector_reason_token` from the closed vocabulary, explaining why this route was selected.
5. Enterprise-bearing routes (`policy_pinned`, `mirror_only`, `offline`) declare a `local_only_fallback_route_token` and a human-readable `fallback_condition_label`.
6. Every row carries at least one `BootstrapCredentialDeclaration` (minimum: `none_required`).
7. Every row carries a non-empty `tls_verification_posture_token`.
8. `mirror_only` and `offline` rows carry `local_core_continuity_explicit: true`.
9. `policy_pinned` and `mirror_only` rows carry a non-empty `managed_attribution_ref`.

## Hard guardrails — withdrawal condition

One condition forces `Withdrawn` immediately and cannot be overridden:

- A row with `raw_secret_or_private_material_excluded: false`
  (narrow reason: `raw_secret_or_private_material_exposed`).

## Canonical paths

- Doc: `docs/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy.md`
- Runtime owner: `aureline_policy::harden_enterprise_network_proxy_pac_manual_system_proxy`
- Fixtures: `fixtures/enterprise/m4/harden-enterprise-network-proxy-pac-manual-system-proxy/`
- Schema: `schemas/enterprise/harden-enterprise-network-proxy-pac-manual-system-proxy.schema.json`
