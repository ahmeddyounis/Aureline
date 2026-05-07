# Command surface-projection cases

Worked cases for
[`/docs/commands/command_surface_projection.md`](../../../docs/commands/command_surface_projection.md)
and
[`/schemas/commands/command_projection.schema.json`](../../../schemas/commands/command_projection.schema.json).

Each file contains one `command_surface_projection_case_record` with an
embedded `command_surface_projection_packet_record` showing how canonical
command identity, aliases, shortcut display, enablement, and help/migration
anchors project across surfaces.

## Cases

| File | Focus |
| --- | --- |
| `enabled_open_folder.yaml` | Fully enabled command projected into palette, menu, keybinding, help, and migration surfaces. |
| `disabled_labs_command_trace.yaml` | Intentionally disabled command (policy gate) with typed disabled reason and repair hook. |
| `deprecated_alias_open_folder.yaml` | Deprecated alias history projects into help/migration without forking command identity. |
| `imported_keymap_open_command_palette.yaml` | Imported keymap bridge anchors project into keybinding/help/migration surfaces. |
| `preview_required_import_profile.yaml` | Preview-required command projects preview cues consistently across surfaces. |
| `dependency_narrowed_clone_repository.yaml` | Environment-narrowed command (missing provider linkage) projects a typed disabled reason. |

