# Desktop summary-surface cases

Reviewer-side worked fixtures for
[`/docs/platform/desktop_summary_surface_matrix.md`](../../../docs/platform/desktop_summary_surface_matrix.md)
and its machine-readable companion
[`/artifacts/platform/desktop_summary_surface_matrix.yaml`](../../../artifacts/platform/desktop_summary_surface_matrix.yaml).

These cases bind platform-facing summary surfaces (Dock, taskbar, jump-list,
launcher equivalents) to the upstream contracts that own the behavior:

- `desktop_summary_affordance_record` (progress/badge mirrors) in
  [`/schemas/ux/notification_suppression_record.schema.json`](../../../schemas/ux/notification_suppression_record.schema.json)
  and
  [`/docs/ux/os_notification_and_quiet_hours_contract.md`](../../../docs/ux/os_notification_and_quiet_hours_contract.md)
- `deep_link_intent_record` (recents and jump actions) in
  [`/schemas/platform/deep_link_intent.schema.json`](../../../schemas/platform/deep_link_intent.schema.json)
  and the route audit in
  [`/artifacts/platform/system_affordance_route_audit.md`](../../../artifacts/platform/system_affordance_route_audit.md)
- `recent_work_entry_record` (recent-item identity and state) in
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json)
  and
  [`/docs/workspace/entry_restore_object_model.md`](../../../docs/workspace/entry_restore_object_model.md)

## Fixture rules

- Every case lists one or more `matrix_row_ids` from
  `artifacts/platform/desktop_summary_surface_matrix.yaml`.
- Every case cites upstream fixture or contract refs that exercise the row(s)
  (for example, a `desktop_summary_affordance_record` example for mirrors, or a
  `deep_link_intent_record` replay-deny case for recents).
- No case introduces raw paths, raw URLs, provider payloads, secret material, or
  customer-owned identifiers. Use opaque ids and closed vocabulary from the
  upstream schemas/contracts.

## Cases

- [`macos_dock_summary_surfaces.yaml`](./macos_dock_summary_surfaces.yaml)
- [`windows_taskbar_jump_list_summary_surfaces.yaml`](./windows_taskbar_jump_list_summary_surfaces.yaml)
- [`linux_gnome_launcher_summary_surfaces_degraded.yaml`](./linux_gnome_launcher_summary_surfaces_degraded.yaml)
- [`side_by_side_ownership_disclosure_no_last_writer_wins.yaml`](./side_by_side_ownership_disclosure_no_last_writer_wins.yaml)

The authoritative roster and short summaries live in
[`manifest.yaml`](./manifest.yaml).
