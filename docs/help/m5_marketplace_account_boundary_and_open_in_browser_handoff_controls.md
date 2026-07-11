# M5 marketplace/account boundary-card and open-in-browser handoff-row controls

The third implement lane over the frozen [M5 embedded-boundary component matrix](m5_embedded_boundary_components_contract.md). It turns the two provider-pane / browser-handoff embedded-boundary components — the **marketplace/account boundary card** and the **open-in-browser handoff row** — into resolvers that produce export-safe, honest projections, so account and marketplace content becomes an explicit, bounded product object naming *whose* content it is, *which scope* it affects, and *why* the flow is still in-product or handed off to the browser, instead of anonymous product chrome.

- Controls packet schema: `schemas/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls.schema.json`
- Support export: `artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_marketplace_account_boundary_card_and_open_in_browser_handoff_row_...`)

## Reused, not re-minted

The lane binds directly to the frozen embedded-boundary and auth-boundary object model so it can never fork its own owner, origin, account-scope, fallback, or handoff-reason wording:

- **Boundary disposition** reuses the single controlled `M5EmbeddedBoundaryDisposition` vocabulary from the matrix (live_first_party_local, live_first_party_hosted, live_provider_owned, stale_snapshot, offline_snapshot, provider_blocked, browser_handoff_only, capability_limited, not_evaluated).
- **Owner / origin** reuses `WebviewOwnerClass`; **account scope** reuses the matrix `M5EmbeddedAccountScope` (no_account_local, personal_account, org_workspace, managed_tenant, account_scope_unknown); **freshness** reuses the matrix `M5EmbeddedFreshnessState`.
- **Browser fallback** reuses `BrowserHandoffKind`, **handoff reason** reuses `HandoffReasonClass`, and **fallback state** reuses `FallbackStateClass`.
- **Network state** is minted here as `M5MarketplaceNetworkState` (online, degraded_connectivity, offline, captive_portal_or_blocked, network_state_unknown).

## Marketplace/account boundary card resolver

`resolve_marketplace_account_boundary_card` degrades first rather than ever letting a generic, identity-concealing card read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Owner / origin (service ownership) undisclosed or untrusted | `owner_or_origin_unstated` |
| Generic product chrome conceals identity, region, or ownership | `generic_chrome_conceals_identity` |
| Account scope unstated | `account_scope_unstated` |
| Current profile or region/tenant cue unstated where relevant | `profile_or_region_unstated` |
| Network state or browser fallback / retry path unstated | `network_state_or_fallback_unstated` |
| Proof stale | `proof_stale` |

A clean card names its owner/origin, account scope, current profile, region/tenant (where relevant), network state, and browser fallback or retry path — the **AC1** guarantee that a marketplace/account pane never hides identity, region, or service ownership behind generic product chrome. An offline or blocked network state never reads as fresh first-party local truth.

## Open-in-browser handoff row resolver

`resolve_open_in_browser_handoff_row` keeps the current object identity, reason-for-handoff, and local-safe continuity explicit:

| Condition | Degrade reason |
| --- | --- |
| Current object identity dropped | `object_identity_dropped` |
| Handoff lands on a generic page | `lands_on_generic_page` |
| Reason the in-product lane ended unstated | `handoff_reason_unstated` |
| Local-safe continuity left implicit | `local_continuity_unstated` |
| Browser fallback / retry path unavailable | `browser_fallback_unavailable` |
| Proof stale | `proof_stale` |

The `object_identity_dropped` and `lands_on_generic_page` degrades are the **AC2** guarantee: a browser handoff preserves object identity and reason-for-handoff instead of dropping users onto a generic landing page. A clean row carries a `browser_handoff_only` disposition with local continuity intact.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- masquerades as native permission or irreversible approval UI;
- hides owner/origin or the browser handoff behind menus only;
- renders a stale, offline, or provider-blocked pane as fresh first-party local truth;
- embeds a high-risk approval without a native step-up.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
