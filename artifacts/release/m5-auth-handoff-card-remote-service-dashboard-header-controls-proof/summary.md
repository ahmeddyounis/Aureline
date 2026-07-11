# M5 Auth Handoff-Card and Remote/Service Dashboard-Header Controls

- Packet: `m5-auth-handoff-card-remote-service-dashboard-header-controls:stable:0001`
- Label: `M5 auth handoff-card and remote/service dashboard-header controls with provider/domain, reason-for-handoff, fallback state, local-safe continuity, device-code expiry, target/service identity, freshness, export/open-console, and no-embedded-high-risk-approval truth`
- Consumer surfaces: 5
- Handoff postures: embedded_sign_in_checkpoint, system_browser_handoff, passkey_handoff, device_code_handoff, provider_content_handoff
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **auth_handoff_ui**: `stable`
  - Owner: Auth-handoff surface owner
  - Scope: Every auth handoff card distinguishes an embedded sign-in checkpoint from a system-browser or passkey handoff, names its provider/domain and reason, and keeps the local-safe continuity note explicit; an embedded surface that imitates native approval chrome or embeds a high-risk approval without a native step-up degrades rather than reading as a clean checkpoint
  - Auth-card examples: 2 / dashboard-header examples: 2
- **remote_dashboard_ui**: `stable`
  - Owner: Remote dashboard owner
  - Scope: Remote / service dashboard headers name their target/service identity and ownership boundary, disclose freshness/offline state, and offer export/open-console actions; a dashboard that substitutes for the primary local recovery controls degrades rather than replacing them, and its auth cards never leave the provider unstated
  - Auth-card examples: 2 / dashboard-header examples: 2
- **account_ui**: `stable`
  - Owner: Account surface owner
  - Scope: Account sign-in cards distinguish passkey handoff from embedded checkpoints and keep continuity explicit; their service dashboards disclose freshness/offline state, degrading when the freshness signal is hidden so an offline snapshot is never rendered as fresh first-party truth
  - Auth-card examples: 2 / dashboard-header examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved auth-card and dashboard-header truth, so a device-code posture that omits its code/expiry disclosure or a dashboard header that hides its service identity is visible in evidence rather than obscured behind chrome
  - Auth-card examples: 2 / dashboard-header examples: 2
- **product_ui**: `stable`
  - Owner: In-product surface owner
  - Scope: In-product auth and service-dashboard surfaces reuse the same handoff-posture, reason, freshness, and ownership vocabulary the auth-handoff surface shows, keeping a high-risk approval out of embedded chrome and an export/open-console path always reachable rather than inventing local prose
  - Auth-card examples: 2 / dashboard-header examples: 2
