# Fixtures: stabilize transport policy, proxy resolution, trust store, and mirror route

These fixtures document the stable transport policy inspector packet. The
canonical source of truth is
`aureline_policy::stabilize_transport_policy_proxy_resolution_trust_store_and_mirror_route::seeded_transport_policy_inspector_page()`.

## Files

| File | Content |
| --- | --- |
| `page.json` | Full `TransportPolicyInspectorPage` packet |
| `policies.json` | Per-endpoint `TransportPolicyRecord` records |
| `network_events.json` | Bounded `NetworkEventRecord` ledger |
| `trust_layers.json` | Trust-store layer snapshots |
| `defects.json` | Empty defect list for the stable packet |
| `summary.json` | Derived coverage and qualification summary |
| `support_export.json` | Support-export envelope |
| `drills/drill_raw_secret_withdrawn.json` | Withdrawal drill for raw material exposure |
| `drills/drill_missing_endpoint_preview.json` | Preview drill for missing endpoint coverage |
| `drills/drill_precedence_preview.json` | Preview drill for route precedence drift |
| `drills/drill_no_recovery_beta.json` | Beta drill for missing recovery guidance |

## Required coverage

The stable packet covers endpoint classes `update`, `marketplace`, `docs`,
`ai`, `provider`, `remote`, and `bootstrap`; route-source precedence
`policy_pinned`, `manual`, `system`, `pac`, `mirror_only`, and `offline`; and
trust layers `os_roots`, `custom_ca_bundle`, `pinned_ssh_host_proof`,
`client_certificate`, and `mirror_trust_root`.

Schema:
`schemas/policy/transport-policy-network-event-mirror-route.schema.json`
