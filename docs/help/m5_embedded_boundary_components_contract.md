# M5 embedded / browser-handoff boundary component contract

This contract freezes Aureline's reusable **embedded / browser-handoff boundary** UI components
into one export-safe matrix so that every M5 surface that crosses into browser-like or
provider-owned content speaks the same owner/origin, data-boundary, freshness, browser-fallback,
account-scope, and capability-limit vocabulary — regardless of which pane renders it.

The authoritative producer is the Rust module
`freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix`
in `crates/aureline-shell`. The checked-in support export under
`artifacts/release/m5-embedded-boundary-proof/` and the narrowed fixtures under
`fixtures/ui/m5-embedded-boundary-components/` are minted from its seed builders by the
`dump_m5_embedded_boundary_component_matrix` example; the schemas under `schemas/ui/` document the
shape but the Rust validator is the gate.

## Governed component families

| Component family | Canonical schema | Names |
| --- | --- | --- |
| `docs_pane_header` | `schemas/ui/m5-docs-pane-header.schema.json` | Owner/origin + source/version/last-updated + freshness of a docs pane |
| `embedded_origin_bar` | `schemas/ui/m5-embedded-origin-bar.schema.json` | Who owns embedded content + capability limits vs native trusted chrome |
| `boundary_fact_grid` | `schemas/ui/m5-boundary-fact-grid.schema.json` | Owner/origin, data boundary, and freshness in one place |
| `marketplace_account_boundary_card` | `schemas/ui/m5-marketplace-account-boundary-card.schema.json` | Account scope + data boundary of marketplace/account content |
| `auth_handoff_card` | `schemas/ui/m5-auth-handoff-card.schema.json` | Browser fallback + data boundary + account scope of a sign-in |
| `remote_service_dashboard_header` | `schemas/ui/m5-remote-service-dashboard-header.schema.json` | Owner + data boundary + provider health + freshness of a dashboard |
| `open_in_browser_handoff_row` | `schemas/ui/m5-open-in-browser-handoff-row.schema.json` | Browser fallback + data boundary for a surface |
| `embedded_state_panel` | `schemas/ui/m5-embedded-state-panel.schema.json` | Stale/offline/provider-blocked/capability-limited state, explicitly |

## The one frozen acceptance-criteria vocabulary

Every row carries `boundary_dispositions`, drawn from the single controlled
`M5EmbeddedBoundaryDisposition` vocabulary. No embedded surface may invent a parallel word for any
of these dispositions:

`live_first_party_local`, `live_first_party_hosted`, `live_provider_owned`, `stale_snapshot`,
`offline_snapshot`, `provider_blocked`, `browser_handoff_only`, `capability_limited`,
`not_evaluated`.

Only `live_first_party_local` is fresh first-party local truth. A stale, offline, or
provider-blocked pane must never read as that state.

## Bound object model (no forking)

The matrix binds directly to the frozen M5 auth-boundary and public-truth object model so no later
consumer forks its own owner/origin or browser-handoff wording:

- **Owner/origin** reuses `WebviewOwnerClass` from `crate::m5_auth_boundaries`.
- **Browser fallback / handoff** reuses `BrowserHandoffKind` from `crate::m5_auth_boundaries`.
- **Capability limits** reuse `CapabilityLimitClass` from `crate::m5_auth_boundaries`.
- **Data boundary** reuses `DataExitBoundary` from `crate::public_truth`.

The matrix's `source_contract_refs` therefore always include the auth-boundary contract doc, the
browser-handoff-card schema, and the webview-origin-bar schema
(`M5_EMBEDDED_BOUNDARY_BINDING_REFS`).

## Hard invariants (guardrails)

Each row carries four boolean guardrails that MUST be `false`, enforced by the validator:

1. `imitates_native_permission_or_approval_ui` — embedded surfaces never imitate native permission
   or irreversible-approval UI.
2. `hides_owner_origin_or_browser_fallback_in_menus_only` — owner/origin and browser fallback are
   never hidden behind menus only.
3. `renders_stale_or_blocked_as_fresh_first_party_truth` — a stale/offline/provider-blocked pane is
   never rendered as fresh first-party local truth.
4. `embeds_high_risk_approval_without_native_step_up` — a high-risk approval is never embedded
   without a native step-up.

## Narrowing honestly

A component whose evidence is not yet proven across every deployment line is narrowed (Beta,
Preview, …) rather than dropped, and every component stays visible. The two checked-in narrowed
fixtures — `docs_pane_header_beta_narrowed.json` and `embedded_state_panel_preview_narrowed.json` —
demonstrate honest narrowing with all eight components still present.
