# M5 Navigation-Content Component Consumers

- Packet: `m5-navigation-content-component-consumers:stable:0001`
- As of: `2026-07-12T00:00:00Z`
- Rows: 17 across 6 consumer classes and 6 / 6 frozen families
- Controls lanes adopted: 4 / 4
- Navigation dispositions preserved: 12 / 12
- Families reused across classes: 6

## Rows

- **consumer:shell-explorer:tab-strip** — surface=shell_ui class=shell_explorer family=tab_strip lane=tab_strip_breadcrumbs authority=full label_parity=preserved handoff=none
- **consumer:shell-explorer:tree-view** — surface=explorer_ui class=shell_explorer family=tree_view lane=tree_view_list_view authority=full label_parity=preserved handoff=none
- **consumer:shell-explorer:breadcrumbs** — surface=explorer_ui class=shell_explorer family=breadcrumbs lane=tab_strip_breadcrumbs authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:shell-explorer:panel-header** — surface=shell_ui class=shell_explorer family=panel_header lane=panel_header_local_action_cluster authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:search-graph:list-view** — surface=search_ui class=search_graph family=list_view lane=tree_view_list_view authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:search-graph:table-grid** — surface=search_ui class=search_graph family=table_grid lane=table_grid_panel_header authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:search-graph:breadcrumbs** — surface=ai_context_ui class=search_graph family=breadcrumbs lane=tab_strip_breadcrumbs authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:search-graph:tab-strip** — surface=search_ui class=search_graph family=tab_strip lane=tab_strip_breadcrumbs authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:review:list-view** — surface=review_ui class=review family=list_view lane=tree_view_list_view authority=full label_parity=preserved handoff=none
- **consumer:review:panel-header** — surface=review_ui class=review family=panel_header lane=panel_header_local_action_cluster authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:request-data:table-grid** — surface=data_ui class=request_data family=table_grid lane=table_grid_panel_header authority=full label_parity=preserved handoff=none
- **consumer:request-data:tree-view** — surface=data_ui class=request_data family=tree_view lane=tree_view_list_view authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:help-center:breadcrumbs** — surface=help_ui class=help_center family=breadcrumbs lane=tab_strip_breadcrumbs authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:help-center:panel-header** — surface=help_ui class=help_center family=panel_header lane=panel_header_local_action_cluster authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:support-export:tab-strip** — surface=support_export class=support_export_help family=tab_strip lane=tab_strip_breadcrumbs authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:table-grid** — surface=support_export class=support_export_help family=table_grid lane=table_grid_panel_header authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:list-view** — surface=support_export class=support_export_help family=list_view lane=tree_view_list_view authority=export_only label_parity=disclosed_narrowed handoff=support_packet
