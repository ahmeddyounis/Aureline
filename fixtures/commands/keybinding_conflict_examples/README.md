# Keybinding resolver fixtures

Worked fixtures for the keybinding resolver contract frozen in
[`/docs/ux/keybinding_resolver_contract.md`](../../../docs/ux/keybinding_resolver_contract.md)
and the schema at
[`/schemas/commands/keybinding_resolver.schema.json`](../../../schemas/commands/keybinding_resolver.schema.json).

Each file here is one top-level record from the frozen resolver
boundary:

- `keybinding_resolution_packet_record`
- `keybinding_conflict_review_packet_record`
- `disabled_command_explanation_packet_record`
- `keybinding_import_bridge_record`
- `leader_overlay_row_record`
- `shortcut_diff_after_import_row_record`

The fixtures exist so shell, settings, docs/help, migration
review, and support surfaces can write against the same typed
resolver outputs instead of inventing local shortcut dialects.

## Validation notes

- Every fixture MUST validate against
  [`/schemas/commands/keybinding_resolver.schema.json`](../../../schemas/commands/keybinding_resolver.schema.json)
  after stripping the fixture-only `$schema` and `__fixture__`
  keys.
- The top-level record itself is authoritative. `__fixture__`
  summarises the scenario and the contract sections it exercises,
  but it is not part of the schema.
- Fixtures that represent imported shortcut truth may also be
  embedded by migration-center examples through
  `shortcut_diff_after_import_row_record`.

## Cases

- [`profile_override_beats_extension.json`](./profile_override_beats_extension.json)
  — conflict-review packet showing a user-profile binding beating
  an extension binding on the same sequence, plus explicit
  outcome-change paths.
- [`platform_reserved_blocks_command_palette.json`](./platform_reserved_blocks_command_palette.json)
  — resolution packet showing a host-reserved shortcut that never
  reaches Aureline dispatch even though a core command exists on a
  lower layer.
- [`rename_symbol_disabled_in_terminal.json`](./rename_symbol_disabled_in_terminal.json)
  — disabled-command explanation packet showing inherited command
  semantics, typed disabled reason, and focus-based recovery.
- [`vscode_save_all_translated_to_leader.json`](./vscode_save_all_translated_to_leader.json)
  — import-bridge row proving that a leader conversion cannot be
  mislabeled `exact`.
- [`leader_prefix_waiting_state.json`](./leader_prefix_waiting_state.json)
  — leader-overlay row exercising waiting-state, timeout posture,
  and pivot actions.
- [`high_frequency_shortcut_diff_after_import.json`](./high_frequency_shortcut_diff_after_import.json)
  — post-import digest row for a daily-driver shortcut whose
  gesture and sequence semantics changed.
