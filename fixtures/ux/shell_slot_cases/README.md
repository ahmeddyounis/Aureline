# Shell Slot Close/Reopen Cases

These fixtures exercise the shell close, reopen, remembered-last-surface,
placeholder, and focus-return contract in
[`docs/ux/shell_close_reopen_contract.md`](../../../docs/ux/shell_close_reopen_contract.md).

The boundary schema is
[`schemas/ux/shell_slot_memory.schema.json`](../../../schemas/ux/shell_slot_memory.schema.json).

Coverage:

- `shell_slot_close_reopen_catalog.json` lists every shell zone and
  closeable slot class.
- `sidebar_search_hide_reopen_exact.json` covers safe sidebar
  remembered-state restore.
- `bottom_panel_terminal_reopen_evidence_only.json` covers bottom-panel
  tab close/reopen without live effect replay.
- `inspector_extension_missing_placeholder.json` covers missing
  extension placeholders in the inspector.
- `collapsed_inspector_pane_reopen_focus.json` covers collapsed-pane
  recovery with visible and command-backed routes.
- `dialog_invoker_removed_focus_fallback.json` covers focus return when
  the exact invoker disappears.
