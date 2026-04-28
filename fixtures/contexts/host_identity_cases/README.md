# Host identity cases

Worked fixtures for the host-identity contract frozen in
[`/docs/ux/host_identity_contract.md`](../../../docs/ux/host_identity_contract.md).
Every fixture conforms to
[`/schemas/contexts/host_identity_chip.schema.json`](../../../schemas/contexts/host_identity_chip.schema.json).

The corpus covers the six host classes and the boundary-change cases
that must remain visible across shell, execution, preview, AI evidence,
notebook, history, and export surfaces:

- [`local_desktop_workspace.json`](./local_desktop_workspace.json)
  — ordinary local desktop work with copy-path and run-here labels that
  explicitly name the local host.
- [`remote_reconnect_terminal.json`](./remote_reconnect_terminal.json)
  — remote host reconnect; local editing continues while remote
  execution is narrowed.
- [`managed_workspace_failover_export.json`](./managed_workspace_failover_export.json)
  — managed workspace failover that preserves prior host lineage in
  history and export receipts.
- [`devcontainer_operator_switch.json`](./devcontainer_operator_switch.json)
  — operator-driven switch from local desktop execution to a
  devcontainer with path mapping and confirmation labels.
- [`browser_bridge_preview.json`](./browser_bridge_preview.json)
  — browser/runtime bridge preview whose output origin and path basis
  differ from the logical source object.
- [`service_plane_tenant_route_change.json`](./service_plane_tenant_route_change.json)
  — service-plane tenant and route change that blocks service writes
  while local-safe work remains available.

Each fixture carries all required surface projections
(`title_context_bar`, `terminal_header`, `task_launcher`,
`debug_launcher`, `notebook_kernel`, `ai_evidence`, `preview_strip`,
`history_row`, and `export_receipt`) so consumers cannot mint
surface-local host labels.
