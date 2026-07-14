# M5 Workspace-Window, Shared-Authority, Skeleton-Restore, and No-Rerun Session-Hydration Matrix

- Packet: `m5-window-restore:stable:0001`
- Label: `M5 workspace-window, shared-authority, skeleton-restore, and no-rerun session-hydration matrix`
- Window-restore families: 5 (5 stable)
- Window-restore roles: workspace_authority, window_topology, pane_role, layout_skeleton, session_hydration, restore_fidelity, display_affinity
- Shared-workspace-authority roles: single_authority_backs_multiple_windows, window_local_selection_and_focus, versioned_attributable_pane_trees, explicit_authority_window_binding, bound_to_window_restore_registry, merged_authority_and_topology_blob_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Window-restore families

- **shared_workspace_authority**: `stable`
  - Owner: Workspace-authority owner
  - Canonical schema: `schemas/shell/m5-window-topology.schema.json`
  - Scope: One shared-workspace-authority profile naming the single authority that backs multiple windows, window-local selection and focus, versioned and attributable pane trees, and the explicit authority-to-window binding so workspace authority and window topology stay separately inspectable and never merge into one opaque blob
  - Required labels: identity, semantic_role, registry_reference, workspace_authority
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **window_local_topology**: `stable`
  - Owner: Window-topology owner
  - Canonical schema: `schemas/shell/m5-window-topology.schema.json`
  - Scope: One window-local-topology profile naming the window-scoped pane tree, versioned pane topology, attributable pane roles, and pane-role placeholder so pane trees stay versioned and attributable and no window collapses into an opaque, unattributable topology
  - Required labels: identity, semantic_role, registry_reference, workspace_authority, display_affinity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **skeleton_first_restore**: `stable`
  - Owner: Restore-coordinator owner
  - Canonical schema: `schemas/shell/m5-restore-fidelity.schema.json`
  - Scope: One skeleton-first-restore profile naming the layout skeleton rebuilt first, heavy dependency hydrated second, pane-role placeholder shown while hydrating, and disclosed restore-fidelity class so restore rebuilds the layout skeleton before hydrating heavy dependencies and a missing extension or remote target never deletes layout structure silently
  - Required labels: identity, semantic_role, registry_reference, restore_fidelity_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **no_rerun_session_hydration**: `stable`
  - Owner: Session-service owner
  - Canonical schema: `schemas/shell/m5-restore-fidelity.schema.json`
  - Scope: One no-rerun-session-hydration profile naming the session-scoped tool that never silently reruns, the privileged session that is never implicitly reattached, the explicit user action required to reacquire broader authority, and the disclosed reopened-versus-rerun context so terminals, debug sessions, notebooks, previews, remote shells, and collaboration surfaces never silently rerun or reacquire broader authority during restore
  - Required labels: identity, semantic_role, registry_reference, restore_fidelity_class
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **display_topology_recovery**: `stable`
  - Owner: Display-topology recovery owner
  - Canonical schema: `schemas/shell/m5-window-topology.schema.json`
  - Scope: One display-topology-recovery profile naming the preserved monitor-affinity hint, windows staying visible after remap, dialogs staying reachable after remap, and preserved follow / presentation intent so a display-topology change keeps every window and dialog reachable and never strands a window off-screen
  - Required labels: identity, semantic_role, registry_reference, display_affinity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
