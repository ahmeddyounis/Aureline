# Shared Window-Restore Consumers: One Registry Across Surfaces

- Packet: `m5-window-restore-shared-consumers:stable:0001`
- Surface: `M5 window-restore shared consumers (one registry across surfaces)`
- Consumer bindings: 15 (6 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer bindings

- **Shared workspace authority (one authority backs many windows)** [`wrsc-shared-authority-coordinator`]: family `shared_workspace_authority` on `restore_coordinator`, representation `desktop_full`, role `workspace_authority`
- **Shared workspace authority (one authority backs many windows)** [`wrsc-shared-authority-shell`]: family `shared_workspace_authority` on `shell_ui`, representation `desktop_full`, role `workspace_authority`
- **Shared workspace authority (one authority backs many windows)** [`wrsc-shared-authority-cli`]: family `shared_workspace_authority` on `cli_export`, representation `exported_redacted`, role `workspace_authority`
- **Window-local topology (window-scoped, versioned pane tree)** [`wrsc-window-topology-workspace`]: family `window_local_topology` on `workspace_service`, representation `desktop_full`, role `window_topology`
- **Window-local topology (window-scoped, versioned pane tree)** [`wrsc-window-topology-shell`]: family `window_local_topology` on `shell_ui`, representation `desktop_full`, role `window_topology`
- **Window-local topology (window-scoped, versioned pane tree)** [`wrsc-window-topology-support`]: family `window_local_topology` on `support_export`, representation `exported_redacted`, role `window_topology`
- **Skeleton-first restore (layout skeleton before heavy hydration)** [`wrsc-skeleton-diagnostics`]: family `skeleton_first_restore` on `diagnostics`, representation `desktop_full`, role `layout_skeleton`
- **Skeleton-first restore (layout skeleton before heavy hydration)** [`wrsc-skeleton-coordinator`]: family `skeleton_first_restore` on `restore_coordinator`, representation `desktop_full`, role `layout_skeleton`
- **Skeleton-first restore (layout skeleton before heavy hydration)** [`wrsc-skeleton-product`]: family `skeleton_first_restore` on `product_ui`, representation `remote_projected`, role `layout_skeleton`
- **No-rerun session hydration (terminals / debug / notebooks never silently rerun)** [`wrsc-no-rerun-session`]: family `no_rerun_session_hydration` on `session_service`, representation `desktop_full`, role `session_hydration`
- **No-rerun session hydration (terminals / debug / notebooks never silently rerun)** [`wrsc-no-rerun-diagnostics`]: family `no_rerun_session_hydration` on `diagnostics`, representation `desktop_full`, role `session_hydration`
- **No-rerun session hydration (terminals / debug / notebooks never silently rerun)** [`wrsc-no-rerun-product`]: family `no_rerun_session_hydration` on `product_ui`, representation `remote_projected`, role `session_hydration`
- **Display-topology recovery (monitor remap keeps windows reachable)** [`wrsc-display-docs`]: family `display_topology_recovery` on `docs_help`, representation `desktop_full`, role `display_affinity`
- **Display-topology recovery (monitor remap keeps windows reachable)** [`wrsc-display-workspace`]: family `display_topology_recovery` on `workspace_service`, representation `compact_narrowed`, role `display_affinity`
- **Display-topology recovery (monitor remap keeps windows reachable)** [`wrsc-display-support`]: family `display_topology_recovery` on `support_export`, representation `exported_redacted`, role `display_affinity`
