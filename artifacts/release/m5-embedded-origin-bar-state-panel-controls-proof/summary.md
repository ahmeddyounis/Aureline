# M5 Embedded-Origin-Bar and Embedded-State-Panel Controls

- Packet: `m5-embedded-origin-bar-state-panel-controls:stable:0001`
- Label: `M5 embedded-origin-bar and embedded-state-panel controls with extension/publisher, origin, permission, capability-limit, reload, open-in-browser, and offline/stale/policy-blocked/certificate-denied/cross-origin-limited state truth`
- Consumer surfaces: 5
- State classes: live_healthy, stale_snapshot, offline_snapshot, policy_blocked, certificate_denied, cross_origin_limited, state_unknown
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## Consumer surfaces

- **embedded_webview_ui**: `stable`
  - Owner: Embedded webview owner
  - Scope: Every extension-owned webview renders an origin bar naming the extension, publisher, origin class, permission state, and capability limits, and never imitates native permission or trust chrome; its embedded-state panel explains stale and offline states with the shared first-party vocabulary
  - Origin-bar examples: 2 / state-panel examples: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace webview owner
  - Scope: Marketplace listing webviews name their provider or publisher on the origin bar and degrade honestly when an extension-owned surface hides its publisher; policy-blocked content is explained rather than shown as fresh first-party truth
  - Origin-bar examples: 2 / state-panel examples: 1
- **remote_dashboard_ui**: `stable`
  - Owner: Remote dashboard owner
  - Scope: Remote / service dashboard webviews disclose the owner/origin chrome or degrade when it is undisclosed, explain cross-origin-limited states with the shared severity vocabulary, and never imitate native permission UI
  - Origin-bar examples: 1 / state-panel examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved origin-bar and state-panel truth, so an undisclosed capability limit or an unexplained embedded state is visible in evidence rather than hidden
  - Origin-bar examples: 1 / state-panel examples: 2
- **product_ui**: `stable`
  - Owner: In-product surface owner
  - Scope: In-product embedded surfaces reuse the same owner/origin and capability-limit vocabulary the embedded webview shows, degrading honestly when a stale state is rendered as fresh first-party truth rather than inventing local prose
  - Origin-bar examples: 1 / state-panel examples: 2
