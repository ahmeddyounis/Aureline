# Command palette row contract fixtures

These fixtures exercise the combined row, modifier-action, and
automation-cue contract in
[`docs/commands/palette_row_and_modifier_contract.md`](../../../docs/commands/palette_row_and_modifier_contract.md).
They are projections from the existing command descriptor, registry,
keybinding, diagnostics, and shareability records rather than new
command definitions.

| Fixture | Purpose |
| --- | --- |
| [`workspace_open_folder_enabled.json`](./workspace_open_folder_enabled.json) | Enabled core row with all required row elements, current shortcut, CLI copy form, and recipe insertion. |
| [`workspace_restore_from_checkpoint_preview.json`](./workspace_restore_from_checkpoint_preview.json) | Preview-required mutating row proving modifier actions preserve preview and approval paths. |
| [`docs_open_in_browser_provider_degraded_ui_only.json`](./docs_open_in_browser_provider_degraded_ui_only.json) | Provider-backed UI-only row where headless/recipe automation is denied with an inspectable reason. |
| [`legacy_bridge_hidden_deprecated.json`](./legacy_bridge_hidden_deprecated.json) | Hidden/deprecated bridge row that remains explainable in support, docs, migration, and CLI discovery surfaces. |
