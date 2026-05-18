# Command reference parity report

Generated from the seeded command-reference catalog in
[`crate::command_reference`](../../../crates/aureline-shell/src/command_reference/mod.rs).
Regenerate by running the fixture-protected integration test:

```sh
cargo test -p aureline-shell --test command_reference_fixtures
```

- Catalog id: `shell:command_reference_beta:catalog:v1`
- Shared contract ref: `shell:command_reference_beta:v1`
- Descriptor schema ref: `schemas/commands/command_descriptor.schema.json`
- Entries: `5`
- High-risk entries: `3`
- Deprecated entries: `0`
- Generated at: `2026-05-18T00:00:00Z`

## Catalog summary

| Command | Lifecycle | Risk | Preview | Idempotency | Deprecation | Aliases |
| ------- | --------- | ---- | ------- | ----------- | ----------- | -------:|
| `cmd:command_palette.open` | `stable` | `inert_metadata_only` | `no_preview_required` | `idempotent` | active | 1 |
| `cmd:workspace.clone_repository` | `stable` | `recoverable_durable_mutation` | `structured_diff_preview` | `non_idempotent_observable_only` | active | 1 |
| `cmd:workspace.import_profile` | `beta` | `recoverable_durable_mutation` | `structured_diff_preview` | `non_idempotent_observable_only` | active | 1 |
| `cmd:workspace.open_folder` | `stable` | `reversible_local_mutation` | `no_preview_required` | `idempotent` | active | 2 |
| `cmd:workspace.restore_from_checkpoint` | `beta` | `destructive_bulk_mutation` | `destructive_bulk_mutation_preview` | `non_idempotent_destructive` | active | 1 |

## Per-command detail

### `cmd:command_palette.open` -- Open Command Palette

Open the command palette to search and run any command.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `inert_metadata_only`
- Preview class: `no_preview_required`
- Idempotency: `idempotent`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:command_palette.open:2026.04.22-01`
- Primary label ref: `label:command_palette.open:primary`
- Docs/help anchor: `docs:anchor:command_palette:open_overview`

#### Aliases

- `alias:command_palette.open:vscode_show_all_commands` (`alternate_palette_phrasing`, `active`), introduced 2026.04, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

- none

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, keybinding_help, docs_help, onboarding`
- Current disabled reasons: none

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+shift+p` | `macos` | `default` | - |
| `chord:ctrl+shift+p` | `windows` | `default` | - |
| `chord:ctrl+shift+p` | `linux` | `default` | - |

#### Automation

- Headless eligible: `false`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `ui_only, ai_not_callable`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Open Command Palette` |
| `command_id` | `cmd:command_palette.open` |
| `canonical_verb` | `command_palette.open` |
| `alias_id` | `alias:command_palette.open:vscode_show_all_commands` |
| `key_sequence` | `chord:cmd+shift+p` |
| `key_sequence` | `chord:ctrl+shift+p` |

#### Discoverability links

- `docs_help` -> `docs:anchor:command_palette:open_overview`
- `onboarding` -> `onboarding:tip:command_palette:open`
- `keybinding_help` -> `keybinding_help:row:command_palette.open`

### `cmd:workspace.clone_repository` -- Clone Repository

Clone a remote repository into a new workspace.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `recoverable_durable_mutation`
- Preview class: `structured_diff_preview`
- Idempotency: `non_idempotent_observable_only`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.clone_repository:2026.04.22-01`
- Primary label ref: `label:workspace.clone_repository:primary`
- Docs/help anchor: `docs:anchor:workspace:clone_repository_overview`

#### Aliases

- `alias:workspace.clone_repository:cli_clone` (`alternate_cli_verb`, `active`), introduced 2026.04, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `remote_url` | `remote_url` | `true` | `-` |
| `destination_scope_ref` | `workspace_scope_ref` | `true` | `-` |

#### Availability

- Trust gate: `trusted_workspace_required`
- Policy gate: `network_egress_allowed`
- Dependency presence: `provider_linked_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, ai_tool_surface, docs_help`
- Current disabled reasons: `provider_not_linked, network_egress_blocked`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:unassigned` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `false`
- AI eligible: `true`
- Automation labels: `headless_safe, recipe_safe, ai_callable_with_approval`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Clone Repository` |
| `command_id` | `cmd:workspace.clone_repository` |
| `canonical_verb` | `workspace.clone_repository` |
| `alias_id` | `alias:workspace.clone_repository:cli_clone` |

#### Discoverability links

- `docs_help` -> `docs:anchor:workspace:clone_repository_overview`
- `command_palette` -> `palette:row:workspace.clone_repository`

### `cmd:workspace.import_profile` -- Import Profile

Import a profile bundle and apply it as a workspace preset.

- Lifecycle: `beta`
- Origin: `core`
- Risk class: `recoverable_durable_mutation`
- Preview class: `structured_diff_preview`
- Idempotency: `non_idempotent_observable_only`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.import_profile:2026.04.22-01`
- Primary label ref: `label:workspace.import_profile:primary`
- Docs/help anchor: `docs:anchor:migration:import_profile_overview`

#### Aliases

- `alias:workspace.import_profile:cli_import_profile` (`alternate_cli_verb`, `active`), introduced 2026.04, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `import_source_ref` | `import_source_ref` | `true` | `-` |
| `apply_scope` | `enum` | `false` | `default_from_descriptor` |
| `create_restore_checkpoint` | `bool` | `false` | `default_from_descriptor` |

#### Availability

- Trust gate: `trusted_workspace_required`
- Policy gate: `labs_beta_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, ai_tool_surface, docs_help`
- Current disabled reasons: `labs_beta_disabled`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:unassigned` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `false`
- AI eligible: `true`
- Automation labels: `headless_safe, recipe_safe, ai_callable_with_approval`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Import Profile` |
| `command_id` | `cmd:workspace.import_profile` |
| `canonical_verb` | `workspace.import_profile` |
| `alias_id` | `alias:workspace.import_profile:cli_import_profile` |

#### Discoverability links

- `docs_help` -> `docs:anchor:migration:import_profile_overview`
- `onboarding` -> `onboarding:tip:migration:import_profile`

### `cmd:workspace.open_folder` -- Open Folder

Open a folder as the active workspace root.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `idempotent`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:workspace.open_folder:2026.04.21-01`
- Primary label ref: `label:workspace.open_folder:primary`
- Docs/help anchor: `docs:anchor:workspace:open_folder_overview`

#### Aliases

- `alias:workspace.open_folder:cli_open` (`alternate_cli_verb`, `active`), introduced 2026.04, import impact `no_action_required`
- `alias:workspace.open_folder:legacy_file_open_folder` (`legacy_command_id`, `deprecated`), introduced 2025.10, retires 2026.10, import impact `rewrite_recipe`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `workspace_scope_ref` | `workspace_scope_ref` | `true` | `-` |
| `add_to_workspace` | `bool` | `false` | `default_from_descriptor` |

#### Availability

- Trust gate: `trusted_workspace_required`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, ai_tool_surface, docs_help, onboarding`
- Current disabled reasons: none

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+o` | `macos` | `default` | - |
| `chord:ctrl+o` | `windows` | `default` | - |
| `chord:ctrl+o` | `linux` | `default` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `true`
- AI eligible: `true`
- Automation labels: `headless_safe, recipe_safe, macro_safe, ai_callable_with_approval`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Open Folder` |
| `command_id` | `cmd:workspace.open_folder` |
| `canonical_verb` | `workspace.open_folder` |
| `alias_id` | `alias:workspace.open_folder:cli_open` |
| `alias_id` | `alias:workspace.open_folder:legacy_file_open_folder` |
| `key_sequence` | `chord:cmd+o` |
| `key_sequence` | `chord:ctrl+o` |

#### Discoverability links

- `docs_help` -> `docs:anchor:workspace:open_folder_overview`
- `onboarding` -> `onboarding:tip:workspace:open_folder`
- `command_palette` -> `palette:row:workspace.open_folder`

### `cmd:workspace.restore_from_checkpoint` -- Restore From Checkpoint

Roll the workspace back to a recorded checkpoint.

- Lifecycle: `beta`
- Origin: `core`
- Risk class: `destructive_bulk_mutation`
- Preview class: `destructive_bulk_mutation_preview`
- Idempotency: `non_idempotent_destructive`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01`
- Primary label ref: `label:workspace.restore_from_checkpoint:primary`
- Docs/help anchor: `docs:anchor:workspace:restore_from_checkpoint_overview`

#### Aliases

- `alias:workspace.restore_from_checkpoint:cli_restore` (`alternate_cli_verb`, `active`), introduced 2026.04, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `checkpoint_ref` | `checkpoint_ref` | `true` | `-` |
| `confirm_destructive` | `bool` | `true` | `-` |

#### Availability

- Trust gate: `trusted_workspace_required`
- Policy gate: `destructive_action_review_required`
- Dependency presence: `checkpoint_present_required`
- Supported surfaces: `command_palette, keybinding_help, cli_headless, docs_help`
- Current disabled reasons: `no_checkpoint_available, destructive_review_pending`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:unassigned` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `headless_safe, ai_not_callable`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Restore From Checkpoint` |
| `command_id` | `cmd:workspace.restore_from_checkpoint` |
| `canonical_verb` | `workspace.restore_from_checkpoint` |
| `alias_id` | `alias:workspace.restore_from_checkpoint:cli_restore` |

#### Discoverability links

- `docs_help` -> `docs:anchor:workspace:restore_from_checkpoint_overview`
- `command_palette` -> `palette:row:workspace.restore_from_checkpoint`

## Verification

```sh
cargo test -p aureline-shell --test command_reference_fixtures
```
