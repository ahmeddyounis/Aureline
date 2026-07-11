# M5 Embedded-Boundary Component Accessibility & Auto-Narrowing

- Packet: `m5-embedded-boundary-component-accessibility-parity:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Families: 8 certified across 8 / 8 frozen families
- Status: 2 green / 6 yellow / 0 red

## Rows

- **a11y:docs-pane-header** (docs_pane_header) — family=docs_pane_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=full_truth status=parity
- **a11y:embedded-origin-bar** (embedded_origin_bar) — family=embedded_origin_bar keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=resolved_truth status=parity
- **a11y:boundary-fact-grid** (boundary_fact_grid) — family=boundary_fact_grid keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: full_truth → degraded (dimension=data_boundary_truth, trigger=data_boundary_unstated) — Data boundary partially resolved — grid shown degraded until the mirror-versus-provider exit path settles
- **a11y:marketplace-account-boundary-card** (marketplace_account_boundary_card) — family=marketplace_account_boundary_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: resolved_truth → degraded (dimension=account_scope_truth, trigger=account_scope_unstated) — Account scope partially resolved — shown degraded until the org-versus-managed-tenant profile resolves
- **a11y:remote-service-dashboard-header** (remote_service_dashboard_header) — family=remote_service_dashboard_header keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=full_truth effective_claim=stale status=narrowed_disclosed
  - Auto-narrow: full_truth → stale (dimension=freshness_truth, trigger=freshness_or_last_updated_unstated) — Provider snapshot stale — shown as a stale snapshot with its last-updated time, not a fresh first-party value, pending refresh
- **a11y:auth-handoff-card** (auth_handoff_card) — family=auth_handoff_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=offline status=narrowed_disclosed
  - Auto-narrow: resolved_truth → offline (dimension=browser_fallback_truth, trigger=browser_fallback_hidden_in_menus_only) — Browser handoff offline — shown as offline with local-safe continuity intact, not an available live sign-in, until the network returns
- **a11y:open-in-browser-handoff-row** (open_in_browser_handoff_row) — family=open_in_browser_handoff_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=offline status=narrowed_disclosed
  - Auto-narrow: resolved_truth → offline (dimension=browser_fallback_truth, trigger=browser_fallback_hidden_in_menus_only) — Open-in-browser offline — shown as offline with the object identity and reason-for-handoff preserved, not a generic landing page, until the network returns
- **a11y:embedded-state-panel** (embedded_state_panel) — family=embedded_state_panel keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=resolved_truth effective_claim=provider_blocked status=narrowed_disclosed
  - Auto-narrow: resolved_truth → provider_blocked (dimension=capability_limit_truth, trigger=capability_limits_unstated) — Embedded content provider-blocked — shown as blocked-by-provider with its capability limits named, not fresh first-party truth, and never imitating native permission chrome
