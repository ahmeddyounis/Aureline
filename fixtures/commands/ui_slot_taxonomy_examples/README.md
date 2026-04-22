# UI-slot taxonomy fixtures

Worked fixtures for the command-graph/UI-slot seed frozen in
[`/docs/commands/command_graph_and_ui_slots_seed.md`](../../../docs/commands/command_graph_and_ui_slots_seed.md).
Every JSON file here conforms to
[`/schemas/commands/ui_slot_taxonomy.schema.json`](../../../schemas/commands/ui_slot_taxonomy.schema.json).

The fixtures exist so shell chrome, docs/help renderers,
onboarding surfaces, and companion handoff surfaces can validate
against one published slot taxonomy before any runtime slot
registry lands.

Files:

- [`ui_slot_taxonomy_seed.json`](./ui_slot_taxonomy_seed.json) —
  the seed taxonomy record. Publishes stable slot families,
  stable slot keys, direct-projection rules, and slot-token
  publication rules.
- [`workspace_open_folder_surface_projection.json`](./workspace_open_folder_surface_projection.json)
  — one command projected consistently into command palette,
  global menu, keybinding help, and CLI help. Reuses the same
  `command_id`, `command_revision_ref`, `primary_label_ref`, and
  `docs_help_anchor_ref` already frozen in
  [`/fixtures/commands/command_descriptor_examples/workspace_open_folder.json`](../command_descriptor_examples/workspace_open_folder.json).
