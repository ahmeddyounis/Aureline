# M5 Marketplace/Account Boundary-Card and Open-in-Browser Handoff-Row Controls

- Packet: `m5-marketplace-account-boundary-open-in-browser-handoff-controls:stable:0001`
- Label: `M5 marketplace/account boundary-card and open-in-browser handoff-row controls with origin, account scope, current profile, region/tenant, network state, browser fallback, object identity, reason-for-handoff, and local-safe continuity truth`
- Consumer surfaces: 5
- Account scopes: no_account_local, personal_account, org_workspace, managed_tenant, account_scope_unknown
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace surface owner
  - Scope: Every marketplace listing card names its provider ownership, account scope, current profile, network state, and browser fallback, and degrades honestly when generic product chrome conceals identity, region, or ownership; its open-in-browser handoff rows preserve the current listing identity and never land on a generic page
  - Boundary-card examples: 2 / handoff-row examples: 2
- **account_ui**: `stable`
  - Owner: Account surface owner
  - Scope: Account panes name owner/origin, account scope, current profile, and region/tenant cues where relevant, and degrade when the service ownership is undisclosed; their outbound handoff rows preserve the current object identity rather than dropping the user onto an anonymous portal
  - Boundary-card examples: 2 / handoff-row examples: 2
- **remote_dashboard_ui**: `stable`
  - Owner: Remote dashboard owner
  - Scope: Remote / service dashboard account cards disclose the managed tenant and region cue and degrade when the account scope is unstated; their handoff rows explain why the in-product lane ended rather than silently opening a generic page
  - Boundary-card examples: 2 / handoff-row examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved boundary-card and handoff-row truth, so a concealed account scope or an implicit local-safe continuity is visible in evidence rather than hidden behind generic chrome
  - Boundary-card examples: 1 / handoff-row examples: 2
- **product_ui**: `stable`
  - Owner: In-product surface owner
  - Scope: In-product account and marketplace surfaces reuse the same owner/origin, account-scope, and browser-fallback vocabulary the marketplace shows, keeping local-safe continuity explicit after every browser handoff rather than inventing local prose
  - Boundary-card examples: 1 / handoff-row examples: 2
