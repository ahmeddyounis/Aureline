# Command-registry seed fixtures

Worked fixtures for the seeded canonical command-registry entry contract
frozen in
[`/schemas/commands/command_registry_entry.schema.json`](../../../schemas/commands/command_registry_entry.schema.json).
Each fixture here is a `command_registry_entry_record` that embeds a
fully conforming
[`command_descriptor_record`](../../../schemas/commands/command_descriptor.schema.json)
and then adds the registry-owned discoverability, alias-lifecycle,
keybinding, badge, diagnostic, and machine-name metadata that future
palette, help-search, onboarding, migration-bridge, keybinding-help,
and diagnostics surfaces are expected to share.

The fixtures exist for three reasons:

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/commands/command_registry_entry.schema.json`](../../../schemas/commands/command_registry_entry.schema.json).
- **Descriptor reuse.** The embedded `descriptor` object MUST also
  validate directly against
  [`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
  without translation.
- **Seed coverage.** The set covers the minimum command seed called for
  by the registry task: `Open`, `Clone`, `Import`, `Restore`, `Command
  Palette`, `Open in Browser`, one preview-required mutating command,
  one intentionally disabled command, and one command carrying
  migration/keymap-bridge aliases.

## Fixtures

- [`workspace_open_folder.registry.json`](./workspace_open_folder.registry.json)
  — baseline core `Open` entry reusing the frozen `workspace.open_folder`
  descriptor.
- [`workspace_clone_repository.registry.json`](./workspace_clone_repository.registry.json)
  — `Clone` entry with recoverable-durable mutation posture and
  cross-surface discoverability refs.
- [`workspace_import_profile.registry.json`](./workspace_import_profile.registry.json)
  — `Import` entry with migration-oriented discoverability and rollback
  evidence expectations.
- [`workspace_restore_from_checkpoint.registry.json`](./workspace_restore_from_checkpoint.registry.json)
  — `Restore` entry and the seed’s preview-required mutating command.
- [`command_palette_open.registry.json`](./command_palette_open.registry.json)
  — `Command Palette` entry with migration and keymap-bridge aliases.
- [`docs_open_in_browser.registry.json`](./docs_open_in_browser.registry.json)
  — `Open in Browser` entry with a built-in-extension origin badge and a
  browser-handoff result contract.
- [`terminal_toggle.registry.json`](./terminal_toggle.registry.json)
  — `Toggle Terminal` entry with bottom-panel, workspace-target, and
  keybinding-help projection refs.
- [`labs_open_command_trace.registry.json`](./labs_open_command_trace.registry.json)
  — intentionally disabled `Labs` command carrying typed repair guidance
  and discoverability-only projection refs.

## Validation notes

- Validation scripts should strip the fixture-only `$schema` and
  `__fixture__` keys before validating the top-level record, matching the
  established command-descriptor fixture workflow.
- The authoritative seed manifest lives in
  [`/artifacts/commands/command_registry_seed.yaml`](../../../artifacts/commands/command_registry_seed.yaml).
  These fixtures mirror the embedded entry records so validators and
  future generators can inspect each command independently.
