# Transport Policy Inspector — Stable Packet

- Packet: `policy:transport-policy-inspector:seeded:0001`
- Schema version: `1`
- Contract ref: `policy:transport_policy_network_event_mirror_route:v1`
- Qualification: `stable` (derived)
- Stable proof index ref:
  `artifacts/release/stable_proof_index.json#proof:transport_policy_inspector_truth`
- Stabilize defects: 0

## Coverage

| Truth set | Stable coverage |
| --- | --- |
| Endpoint classes | `update`, `marketplace`, `docs`, `ai`, `provider`, `remote`, `bootstrap` |
| Route-source precedence | `policy_pinned`, `manual`, `system`, `pac`, `mirror_only`, `offline` |
| Trust-store layers | `os_roots`, `custom_ca_bundle`, `pinned_ssh_host_proof`, `client_certificate`, `mirror_trust_root` |
| Egress decisions | `allow`, `allow_mirror`, `offline_deferred`, `deny_policy`, `deny_contract_mismatch`, `deny_trust`, `deny_transport` |
| Mirror/offline route states | Present on every policy record and every network-event record |
| Control/data plane truth | Separate typed states on every record |
| Local-core continuity | Explicit on every policy record |
| Redaction | Raw secrets, PAC bodies, certificate bodies, private keys, URLs, hostnames, and IPs excluded |

## Key invariants verified

1. Enterprise inspectors and support exports can reconstruct route choice from
   typed endpoint, route-source, egress, proxy-chain, and mirror-state fields.
2. Trust material remains layered; OS roots, custom CA bundles, SSH host
   proofs, client certificates, and mirror trust roots never collapse into one
   generic trust-store label.
3. Policy block, contract mismatch, trust failure, transport failure, mirror
   routing, and offline deferral use distinct product states.
4. Managed service degradation preserves control-plane versus data-plane truth
   and keeps local-core continuity explicit.
5. The support export is redaction-safe by construction.

## Canonical paths

- Runtime owner:
  `aureline_policy::stabilize_transport_policy_proxy_resolution_trust_store_and_mirror_route`
- Doc:
  `docs/enterprise/m4/stabilize-transport-policy-proxy-resolution-trust-store-and-mirror-route.md`
- Fixtures:
  `fixtures/enterprise/m4/stabilize-transport-policy-proxy-resolution-trust-store-and-mirror-route/`
- Schema:
  `schemas/policy/transport-policy-network-event-mirror-route.schema.json`
