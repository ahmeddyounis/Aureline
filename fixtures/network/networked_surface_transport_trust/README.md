# Fixtures: networked-surface transport-trust governance

These fixtures document the stable networked-surface transport-trust governance
proof packet — the layer that makes the trust inputs and host proof behind every
network-capable surface a first-class governed object alongside the
[networked-surface proxy-resolution governance](../networked_surface_proxy_resolution/README.md)
and the
[networked-surface transport decision log](../networked_surface_transport_decision/README.md).
The canonical source of truth is the seeded packet produced by
`aureline_remote::networked_surface_transport_trust::seeded_transport_trust_page()`.

Regenerate every file with the headless dump example (do not hand-edit):

```sh
cargo run -q -p aureline-remote \
  --example dump_networked_surface_transport_trust_fixtures -- <subcommand>
```

## Files

| File | Subcommand | Content |
|------|------------|---------|
| `page.json` | `page` | Full `TransportTrustPage` proof packet (stable, zero defects) |
| `rows.json` | `rows` | Per-record `TrustRow` records (all required surfaces) |
| `defects.json` | `defects` | Empty defect list (clean stable packet) |
| `summary.json` | `summary` | `TrustSummary` with counts, source/state/outcome/cue roll-up, and overall qualification |
| `support_export.json` | `support-export` | `TrustSupportExport` envelope for support/diagnostics |
| `cli_view.txt` | `cli-view` | Stable CLI/headless rendering for terminal parity |
| `drills/drill_missing_surface_preview.json` | `drill-missing-surface-preview` | Missing `ai_gateway` record narrows to `preview` |
| `drills/drill_raw_material_withdrawn.json` | `drill-raw-material-withdrawn` | Raw trust material on `request_api_client` withdraws the packet |
| `drills/drill_private_key_withdrawn.json` | `drill-private-key-withdrawn` | Raw private-key material on `database_cloud_connector` withdraws the packet |
| `drills/drill_ca_override_withdrawn.json` | `drill-ca-override-withdrawn` | A direct CA override on `docs_browser_fetcher` withdraws the packet |
| `drills/drill_silent_downgrade_withdrawn.json` | `drill-silent-downgrade-withdrawn` | A silent trust downgrade on `registry_read` withdraws the packet |
| `drills/drill_denied_no_reason_beta.json` | `drill-denied-no-reason-beta` | A `deny_trust` record on `sync_offboarding` with no reason narrows that row to `beta` |
| `drills/drill_missing_rotation_cue_beta.json` | `drill-missing-rotation-cue-beta` | A `rotation_due` trust root on `docs_browser_fetcher` with no active cue narrows that row to `beta` |

## Required surfaces

All of the following network-capable surfaces must have a trust-evaluation
record for a stable claim: `ai_gateway`, `docs_browser_fetcher`,
`request_api_client`, `database_cloud_connector`, `registry_read`,
`companion_handoff`, `provider_mutation`, `sync_offboarding`,
`remote_preview_route`.

## Trust inputs (per record)

- **Trust-store source** — `system_trust_store`, `pinned_ca_set`,
  `managed_org_bundle`, `mirror_root`, `ssh_known_hosts`, or `no_tls_loopback`.
- **CA bundle review** — `system_default`, `org_reviewed`, `pinned_set`,
  `mirror_root`, or `not_applicable`, plus an opaque bundle handle and pin count.
- **Host-proof state** — `pinned_match`, `known_tofu`, `first_use_pending`,
  `changed_mismatch`, `revoked`, or `not_applicable`, plus a host-proof history
  depth.
- **Client-certificate posture** — `not_required`, `optional_presented`,
  `required_presented`, `managed_provisioned`, or `required_absent`.
- **Trust-root freshness / rotation cue** — `fresh`, `rotation_due`,
  `rotation_in_progress`, `expired`, or `pinned_static`, paired with a
  `none` / `rotate_soon` / `rotating` / `rotate_now` / `pinned_no_rotation` cue.

## deny_trust vocabulary

A missing or unverifiable trust input surfaces as a typed `deny_trust` reason:
`trust_store_unavailable`, `ca_bundle_missing`, `ca_bundle_stale`,
`managed_bundle_unverified`, `host_proof_missing`, `host_proof_changed`,
`host_proof_revoked`, `client_cert_required_absent`, `trust_root_expired`,
`pin_set_mismatch`, or `mirror_root_mismatch` — never generic offline copy.

## Schema

`schemas/network/networked_surface_transport_trust.schema.json`

## Contract ref

`remote:networked_surface_transport_trust:v1`
