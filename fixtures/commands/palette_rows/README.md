# Command palette row fixtures

These fixtures exercise the palette projection contract in
[`docs/commands/palette_row_contract.md`](../../../docs/commands/palette_row_contract.md).
They are projected from existing command-registry seed records rather
than authored as independent command definitions.

The combined row/modifier/automation-cue cases live in
[`fixtures/commands/palette_row_cases/`](../palette_row_cases/) and are
the cross-surface row records desktop, docs, CLI discovery, support, and
automation surfaces may all cite. The fixtures in this directory remain
the split query-result and action-footer materializations.

| Fixture | Schema | Purpose |
|---|---|---|
| [`workspace_open_folder_enabled.palette_row.json`](./workspace_open_folder_enabled.palette_row.json) | `palette_result.schema.json` | Enabled core row with assigned shortcut, badges, docs anchor, and multi-surface projection targets. |
| [`workspace_restore_from_checkpoint_preview.palette_row.json`](./workspace_restore_from_checkpoint_preview.palette_row.json) | `palette_result.schema.json` | Preview-required mutating row with approval, rollback/evidence, and automation cues. |
| [`labs_open_command_trace_disabled.palette_row.json`](./labs_open_command_trace_disabled.palette_row.json) | `palette_result.schema.json` | Disabled Labs row with typed policy reason, repair hook, and why-not-automatable detail. |
| [`workspace_open_folder_action_footer.palette_footer.json`](./workspace_open_folder_action_footer.palette_footer.json) | `palette_action_footer.schema.json` | Footer variants for primary run, split/open-alt, copy ID, copy CLI/headless form, add to recipe, and docs/help. |
| [`labs_open_command_trace_blocked_footer.palette_footer.json`](./labs_open_command_trace_blocked_footer.palette_footer.json) | `palette_action_footer.schema.json` | Blocked footer with inspect-unavailability, copy ID, why-not-automatable, and disabled recipe insertion. |
