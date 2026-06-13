# Networked-Surface Transport Automation — Stable Packet

- Packet: `remote:networked_surface_transport_automation:default`
- Schema version: `1`
- Contract ref: `remote:networked_surface_transport_automation:v1`
- Schema: `schemas/network/networked_surface_transport_automation.schema.json`
- Runtime owner: `aureline_remote::networked_surface_transport_automation`
- Evidence index: `artifacts/release/m5/xt12-evidence-index.md`
- Qualification: `stable` (derived, not asserted)
- Defects: 0
- Withdrawn rows: 0
- Stable rows: all (9)

This automation packet is the audit and automation truth source of the
networked-surface transport-governance lane. It gives M5 network activity
history, support exports, and headless automation one canonical transport-denial
vocabulary, one set of network-activity filters, and one redaction-safe
route/origin history join, so an M5 network failure is explained from a single
packet set rather than reconstructed from per-feature logs.

## Canonical denial vocabulary

The eight required denial codes, now visible and reusable across claimed M5
rows: `proxy_misconfigured`, `proxy_auth_required`, `ca_untrusted`,
`ssh_host_key_unknown`, `egress_blocked_policy`, `mirror_unreachable`,
`offline_mode`, `origin_scope_ambiguous`. `none` is the allowed-action sentinel.
The per-feature matrix, proxy, and trust denial classes all map into this
vocabulary.

## Activity coverage

| Surface | Origin scope | Route choice | Disposition | Denial code |
|---|---|---|---|---|
| `ai_gateway` | `third_party` | `manual_proxy` | `allowed` | `none` |
| `docs_browser_fetcher` | `third_party` | `direct` | `denied` | `ca_untrusted` |
| `request_api_client` | `user_configured` | `manual_proxy` | `denied` | `proxy_auth_required` |
| `database_cloud_connector` | `user_configured` | `system_proxy` | `denied` | `proxy_misconfigured` |
| `registry_read` | `first_party` | `mirror_first` | `denied` | `mirror_unreachable` |
| `companion_handoff` | `loopback_local` | `offline` | `deferred` | `offline_mode` |
| `provider_mutation` | `third_party` | `direct` | `denied` | `egress_blocked_policy` |
| `sync_offboarding` | `managed_tenant` | `direct` | `denied` | `ssh_host_key_unknown` |
| `remote_preview_route` | `third_party` | `direct` | `denied` | `origin_scope_ambiguous` |

All nine required M5 surfaces appear, and all eight canonical denial codes are
surfaced across the activity history.

## Stable field catalog (product / CLI / support parity)

Every activity record renders these fields, in this order, on every surface, so
the denial codes and field names match across product UI, CLI/headless output,
and support exports:

`surface`, `origin_scope`, `endpoint_class`, `egress_class`, `route_choice`,
`disposition`, `denial_code`, `occurred_at`.

## Network-activity filters

Six filter dimensions are exposed as facets: `surface`, `origin_scope`,
`endpoint_class`, `route_choice`, `disposition`, `denial_code`. `ActivityFilter`
composes any subset; the `denied-filter` headless view lists every denied action
with its canonical denial code.

## Route/origin history joins

The activity history is joined by `(route_choice, origin_scope)`; each join row
carries the total/allowed/denied/deferred counts, the surfaces present, and the
canonical denial codes present. Join totals reconstruct the full activity count.

## Key invariants verified

1. All nine required surfaces have an activity record, and all eight canonical denial codes appear.
2. No raw private material is present on any record (`raw_private_material_excluded: true`).
3. Every record resolved through the shared transport-governance layer (`no_bypass: true`).
4. The only `deferred` action (`companion_handoff`) is idempotent; no non-idempotent action enters a replay queue.
5. Every denied record carries a non-`none` canonical denial code.
6. Every allowed record carries the `none` denial code; dispositions and codes agree.
7. The page exposes the complete canonical denial vocabulary.

## Hard guardrails — withdrawal conditions

Any one of these forces `withdrawn` immediately and cannot be overridden:

- a record with `raw_private_material_excluded: false` (narrow reason `raw_private_material_exposed`),
- a record with `no_bypass: false` (narrow reason `bypassed_shared_governance`),
- a deferred record that queues a non-idempotent action (narrow reason `non_idempotent_replay_queued`).

A missing required surface narrows the packet to `preview`. A denied record
without a canonical code, a disposition/denial-code mismatch, or an incomplete
denial vocabulary narrows the affected row to `beta`, which lets release and
support tooling detect and automatically narrow stale or under-qualified rows
before publication.

## Drill coverage

| Drill | Record mutated | Outcome |
|---|---|---|
| `drill_missing_surface_preview` | drops `ai_gateway` | `preview` |
| `drill_raw_material_withdrawn` | `docs_browser_fetcher` exposes raw material | `withdrawn` |
| `drill_bypass_withdrawn` | `request_api_client` bypasses shared governance | `withdrawn` |
| `drill_non_idempotent_withdrawn` | `companion_handoff` defers a non-idempotent action | `withdrawn` |
| `drill_denied_no_code_beta` | `provider_mutation` denied without a code | row `beta` |
| `drill_disposition_mismatch_beta` | allowed `ai_gateway` carries a denial code | row `beta` |

Fixtures: `fixtures/network/networked_surface_transport_automation/`.
