# Menu and command-bar fixtures

These fixtures exercise the menu, context-menu, command-bar, and
inline-toolbar projection contract in
[`docs/ux/menu_command_bar_contract.md`](../../../docs/ux/menu_command_bar_contract.md).
They are projected from command-registry, diagnostics, keybinding, and
target-snapshot records rather than authored as independent command
definitions.

| Fixture | Schema | Purpose |
|---|---|---|
| [`global_menu_open_folder_parity.json`](./global_menu_open_folder_parity.json) | `menu_item.schema.json` | Application-menu command item preserving canonical command ID, label, shortcut, docs/help anchor, and alternate routes. |
| [`editor_context_menu_stale_refactor.json`](./editor_context_menu_stale_refactor.json) | `menu_item.schema.json` | Context-menu row invalidated after target snapshot drift, with typed disabled reason and refresh/re-preview route. |
| [`editor_command_bar_toggle_choice.json`](./editor_command_bar_toggle_choice.json) | `menu_item.schema.json` | Command-bar toggle and choice rows carrying current state while preserving command identity and disabled-reason parity. |
| [`source_control_destructive_and_provider.json`](./source_control_destructive_and_provider.json) | `menu_item.schema.json` | Destructive source-control row separated from safe actions plus provider-backed dynamic action with partial provider disclosure. |
