# M5 embedded-boundary component consumer contract (M05-1074)

This contract is the consumer-adoption lane over the frozen M5 embedded-boundary
component matrix. It proves that the eight reusable embedded / browser-handoff
component families are adopted as **primitives** across every claimed M5
embedded/browser-handoff consumer, rather than being reinvented as per-pane
embedded chrome.

## The eight canonical component families

Each consumer points back to exactly one canonical component family (its
per-family matrix schema) and to the one canonical **controls contract** its
family group belongs to:

| Component family | Controls lane |
| --- | --- |
| `docs_pane_header` | `docs_boundary_facts` |
| `boundary_fact_grid` | `docs_boundary_facts` |
| `embedded_origin_bar` | `origin_state` |
| `embedded_state_panel` | `origin_state` |
| `marketplace_account_boundary_card` | `marketplace_handoff` |
| `open_in_browser_handoff_row` | `marketplace_handoff` |
| `auth_handoff_card` | `auth_dashboard` |
| `remote_service_dashboard_header` | `auth_dashboard` |

A consumer must reuse the lane's canonical schema, doc, and release-proof
artifact rather than forking a pane-local one. A family always resolves to the
same controls lane across every surface (no fork by consumer).

## The six claimed consumer classes

1. `docs_help_pane` — the documentation / help pane.
2. `marketplace_account` — marketplace / account content.
3. `embedded_webview` — extension-owned embedded webviews.
4. `auth_handoff` — browser / device-code auth handoffs.
5. `remote_service_dashboard` — remote / service dashboards.
6. `support_export_help` — support / export + release packets (AC2).

Each concrete surface (`docs_browser_ui`, `marketplace_ui`, `account_ui`,
`remote_dashboard_ui`, `embedded_webview_ui`, `auth_handoff_ui`,
`support_export`, `product_ui`) maps to exactly one class.

## Preserved boundary truth

Every consumer keeps the identical controlled label families — owner/origin,
data boundary, source/version/last-updated, network/offline state, browser
fallback, account scope, freshness, capability limits, and the
no-embedded-high-risk-approval promise — and the identical frozen
**boundary-disposition** vocabulary
(`live_first_party_local`, `live_first_party_hosted`, `live_provider_owned`,
`stale_snapshot`, `offline_snapshot`, `provider_blocked`, `browser_handoff_only`,
`capability_limited`, `not_evaluated`).

Every row also preserves its family's **primary boundary label** — the boundary
axis the family exists to name — so an embedded pane never hides whose content it
renders or why the flow crosses the browser or provider boundary. A boundary-
crossing consumer (marketplace/account, embedded webview, provider auth handoff,
or remote/service dashboard) never drops it.

A narrower consumer (read-only, inspect-only, override-gated, export-only,
policy-blocked) discloses the reduction with a reduced-capability banner whose
`capability_state` matches the authority mode, and carries a handoff note
whenever it punts to the desktop shell, companion, browser, or support packet.

## Guardrails (all must stay false)

- `imitates_native_permission_or_approval_ui`
- `hides_owner_origin_or_browser_fallback_in_menus_only`
- `renders_stale_or_blocked_as_fresh_first_party_truth`
- `embeds_high_risk_approval_without_native_step_up`

## Acceptance criteria

- **AC1** — the same origin/boundary state renders with one vocabulary and one
  component family across every claimed embedded/browser-handoff consumer.
- **AC2** — help, support, and release packets no longer need bespoke prose to
  explain how different embedded panes cross the browser or provider boundary;
  both a docs/help consumer and a support/export consumer reference the canonical
  component families.

## Truth source

- Schema: `schemas/ui/m5-embedded-boundary-component-consumer.schema.json`
- Release proof:
  `artifacts/release/m5-embedded-boundary-component-consumer-proof/`
- Fixtures: `fixtures/ui/m5-embedded-boundary-component-consumers/`

The packet is metadata-only: raw provider tokens, credential material, and
cookies never cross this boundary. Regenerate the checked-in artifacts with
`GEN_EMBEDDED_BOUNDARY_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
