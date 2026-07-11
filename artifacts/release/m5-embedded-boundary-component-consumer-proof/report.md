# M5 Embedded-Boundary Component Consumers

- Packet: `m5-embedded-boundary-component-consumers:stable:0001`
- As of: `2026-07-10T00:00:00Z`
- Rows: 15 across 6 consumer classes and 8 / 8 frozen families
- Controls lanes adopted: 4 / 4
- Boundary dispositions preserved: 9 / 9
- Families reused across classes: 5

## Rows

- **consumer:docs-help:docs-pane-header** — surface=docs_browser_ui class=docs_help_pane family=docs_pane_header lane=docs_boundary_facts authority=full label_parity=preserved handoff=none
- **consumer:docs-help:boundary-fact-grid** — surface=docs_browser_ui class=docs_help_pane family=boundary_fact_grid lane=docs_boundary_facts authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:marketplace:account-boundary-card** — surface=marketplace_ui class=marketplace_account family=marketplace_account_boundary_card lane=marketplace_handoff authority=full label_parity=preserved handoff=none
- **consumer:account:account-boundary-card** — surface=account_ui class=marketplace_account family=marketplace_account_boundary_card lane=marketplace_handoff authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:marketplace:open-in-browser-row** — surface=marketplace_ui class=marketplace_account family=open_in_browser_handoff_row lane=marketplace_handoff authority=read_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:embedded-webview:origin-bar** — surface=embedded_webview_ui class=embedded_webview family=embedded_origin_bar lane=origin_state authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:embedded-webview:state-panel** — surface=embedded_webview_ui class=embedded_webview family=embedded_state_panel lane=origin_state authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:product:embedded-origin-bar** — surface=product_ui class=embedded_webview family=embedded_origin_bar lane=origin_state authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:auth-handoff:auth-card** — surface=auth_handoff_ui class=auth_handoff family=auth_handoff_card lane=auth_dashboard authority=override_gated label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:auth-handoff:open-in-browser-row** — surface=auth_handoff_ui class=auth_handoff family=open_in_browser_handoff_row lane=marketplace_handoff authority=read_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:remote-dashboard:dashboard-header** — surface=remote_dashboard_ui class=remote_service_dashboard family=remote_service_dashboard_header lane=auth_dashboard authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:remote-dashboard:state-panel** — surface=remote_dashboard_ui class=remote_service_dashboard family=embedded_state_panel lane=origin_state authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:support-export:docs-pane-header** — surface=support_export class=support_export_help family=docs_pane_header lane=docs_boundary_facts authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:remote-dashboard-header** — surface=support_export class=support_export_help family=remote_service_dashboard_header lane=auth_dashboard authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:auth-card** — surface=support_export class=support_export_help family=auth_handoff_card lane=auth_dashboard authority=export_only label_parity=disclosed_narrowed handoff=support_packet
