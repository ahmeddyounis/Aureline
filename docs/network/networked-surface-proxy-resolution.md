# Networked-surface proxy-resolution governance

This packet makes the **proxy-resolution step itself** a first-class governed
object. The sibling
[transport-decision log](./networked-surface-transport-decision.md) emits one
decision per action and records *which* proxy tier selected the route; this
packet freezes, per surface, the ordered candidate chain resolution walked, the
winning tier, and a typed `deny_proxy_resolution` reason when no tier may be
honored — so the precedence is inspectable and no M5 client, helper, or
extension can ship a private proxy stack, a direct CA override, or a hidden
direct-connect fallback.

The surfaces it governs are the AI inference gateway, documentation and
in-product browser fetchers, generic request/API clients, database and cloud
connectors, extension and model registry reads, companion device handoffs,
provider mutation lanes, sync and offboarding traffic, and the richer remote
preview routes.

The owner is `aureline_remote::networked_surface_proxy_resolution`; the boundary
schema is `schemas/network/networked_surface_proxy_resolution.schema.json`.

The packet does **not** re-derive raw proxy hosts, raw PAC bodies, raw CA
bundles, raw certificate bytes, raw credentials, or raw bearer/session tokens.
Every proxy config source is named by opaque handle only.

## Precedence ladder

Proxy resolution walks an ordered candidate chain, **highest precedence first**:

1. `pac_script` — a proxy-auto-config script selects the route.
2. `manual_pinned` — a manually-configured or policy-pinned proxy.
3. `environment_proxy` — a declared, governed `HTTP(S)_PROXY`-style value.
4. `system_proxy` — the platform/OS proxy.
5. `direct_no_proxy` — a *declared* direct connection (never a silent fallback).

Two tiers sit outside the ladder: `mirror_pinned` (a signed-mirror route, no
proxy participates) and `offline_no_route` (no route resolved). The selected
tier must be the highest-precedence **available** candidate; a record that
selects a lower tier while a higher one is available narrows to `beta`
(`precedence_not_respected`).

## What one record holds

- **Candidate chain** — each considered tier in precedence order, flagged
  `available`/`selected`/`is_private_stack`, named by opaque handle.
- **Selected tier** — the winning tier, or `null` when resolution was denied,
  degraded, or offline.
- **Outcome** — `resolved`, `mirror_pinned_no_proxy`,
  `degraded_awaiting_policy`, `denied_proxy_resolution` (with a typed reason), or
  `offline_no_route`.
- **`deny_proxy_resolution` reason** — `contradictory_proxy_state`,
  `private_proxy_stack_detected`, `direct_ca_override_detected`,
  `undeclared_public_fallback`, `pac_unreachable`, `proxy_unreachable`,
  `no_resolvable_route`, `mirror_only_no_proxy_permitted`, or
  `policy_epoch_unavailable`. A contradictory or unresolvable proxy state is
  labeled **degraded** or **denied** rather than silently direct-connecting.
- **Guardrail flags** — `no_private_proxy_stack`, `no_direct_ca_override`, and
  `no_silent_direct_fallback`.

## Contract

For the stable claim to hold, **all** of the following must be verified
simultaneously for every covered record:

1. **All required surfaces resolved** — one record for each of: `ai_gateway`,
   `docs_browser_fetcher`, `request_api_client`, `database_cloud_connector`,
   `registry_read`, `companion_handoff`, `provider_mutation`,
   `sync_offboarding`, `remote_preview_route`.
2. **No raw private material** — every record carries
   `raw_private_material_excluded: true`.
3. **No private proxy stack** — every record carries
   `no_private_proxy_stack: true` (no candidate is a private/undeclared stack).
4. **No direct CA override** — every record carries
   `no_direct_ca_override: true`.
5. **No silent direct fallback** — every record carries
   `no_silent_direct_fallback: true`.
6. **Local-core continuity preserved** — every record carries
   `local_core_continuity_preserved: true`.
7. **Denials are typed** — every `denied_proxy_resolution` record carries a
   typed `denial_reason`.
8. **Precedence respected** — every record's selected tier is the
   highest-precedence available candidate.
9. **Policy epoch traceable** — every record whose egress class requires it
   (`public_internet`, `managed_endpoint`, `mirror_only`) carries a
   `policy_epoch_ref`.
10. **Classification complete** — every record carries a non-empty candidate
    chain, a typed outcome, and a consistent selected tier.

## Narrowing

The qualification tier is derived, never asserted:

- **Withdrawn (hard, non-overridable):** `raw_private_material_exposed`,
  `private_proxy_stack_shipped`, `direct_ca_override_shipped`, or
  `silent_direct_fallback_resolved`.
- **Preview:** `required_surface_missing` — a coverage gap prevents any
  verifiable claim for the missing surface.
- **Beta:** `deny_reason_missing`, `precedence_not_respected`,
  `local_core_continuity_not_preserved`, `policy_epoch_ref_missing`, or
  `proxy_chain_classification_incomplete`.

Because under-qualified rows narrow to beta, release and support tooling that
ingests this packet can detect them and automatically narrow the affected proxy
claims before publication. The packet is bound into the canonical evidence index
at `artifacts/release/m5/xt12-evidence-index.md`.

## Consuming the packet

Dashboards, Help/About surfaces, CLI/headless output, diagnostics, support
exports, and release tooling should ingest the `ProxyResolutionGovernancePage`
(and its `ProxyResolutionSupportExport` envelope, or the
`render_cli_view` rendering for terminal parity) rather than reconstructing
precedence from raw proxy config. The packet, its rows, summary, defects,
support export, and CLI view are emitted by the headless example
`dump_networked_surface_proxy_resolution_fixtures` and pinned as fixtures under
`fixtures/network/networked_surface_proxy_resolution/`.
