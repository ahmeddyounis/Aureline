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
- Entries: `8`
- High-risk entries: `3`
- Deprecated entries: `0`
- Generated at: `2026-05-18T00:00:00Z`

## Catalog summary

| Command | Lifecycle | Risk | Preview | Idempotency | Deprecation | Aliases |
| ------- | --------- | ---- | ------- | ----------- | ----------- | -------:|
| `cmd:workspace.open_folder` | `stable` | `reversible_local_mutation` | `no_preview_required` | `idempotent_with_visible_redirect` | active | 3 |
| `cmd:workspace.clone_repository` | `stable` | `recoverable_durable_mutation` | `structured_diff_preview` | `non_idempotent_observable_only` | active | 2 |
| `cmd:workspace.import_profile` | `stable` | `recoverable_durable_mutation` | `structured_diff_preview` | `non_idempotent_observable_only` | active | 2 |
| `cmd:workspace.restore_from_checkpoint` | `stable` | `recoverable_durable_mutation` | `structured_diff_preview` | `non_idempotent_observable_only` | active | 2 |
| `cmd:command_palette.open` | `stable` | `reversible_local_mutation` | `no_preview_required` | `idempotent` | active | 4 |
| `cmd:docs.open_in_browser` | `stable` | `reversible_local_mutation` | `no_preview_required` | `idempotent` | active | 1 |
| `cmd:terminal.toggle` | `stable` | `reversible_local_mutation` | `no_preview_required` | `non_idempotent_destructive` | active | 2 |
| `cmd:explorer.toggle` | `stable` | `reversible_local_mutation` | `no_preview_required` | `idempotent` | active | 2 |

## Per-command detail

### `cmd:workspace.open_folder` -- Open Folder

Open a local folder as the active workspace or add it to the current workspace.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `idempotent_with_visible_redirect`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:workspace.open_folder:2026.04.21-01`
- Primary label ref: `label:workspace.open_folder:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:workspace:open_folder_overview`

#### Aliases

- `alias:workspace.open_folder:legacy_file_open_folder` (`legacy_command_id`, `deprecated`), introduced release:aureline:0.8, retires release:aureline:next, import impact `rewrite_recipe`
- `alias:workspace.open_folder:cli_open` (`alternate_cli_verb`, `active`), introduced release:aureline:1.0, import impact `no_action_required`
- `alias:workspace.open_folder:ai_tool_open_workspace` (`ai_tool_handle`, `active`), introduced release:aureline:1.0, import impact `ai_tool_handle_renames`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `workspace_scope_ref` | `workspace_scope_ref` | `true` | `-` |
| `add_to_workspace` | `boolean_flag` | `false` | `-` |

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, ai_tool_surface, docs_help, onboarding`
- Current disabled reasons: `execution_context_unavailable, freshness_floor_unmet`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+o` | `macos` | `default` | - |
| `chord:ctrl+k_ctrl+o` | `windows` | `default` | - |
| `chord:ctrl+k_ctrl+o` | `linux` | `default` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `true`
- AI eligible: `true`
- Automation labels: `macro_safe, recipe_safe, headless_safe`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Open Folder` |
| `command_id` | `cmd:workspace.open_folder` |
| `canonical_verb` | `workspace.open_folder` |
| `alias_id` | `alias:workspace.open_folder:legacy_file_open_folder` |
| `alias_id` | `alias:workspace.open_folder:cli_open` |
| `key_sequence` | `chord:cmd+o` |
| `key_sequence` | `chord:ctrl+k_ctrl+o` |
| `key_sequence` | `chord:ctrl+k_ctrl+o` |

#### Discoverability links

- `docs_help` -> `projection:workspace.open_folder:docs_help_page`
- `onboarding` -> `projection:workspace.open_folder:onboarding_hint`
- `command_palette` -> `projection:workspace.open_folder:palette_row`

### `cmd:workspace.clone_repository` -- Clone Repository

Clone a remote repository into a local destination and optionally open it as the next workspace.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `recoverable_durable_mutation`
- Preview class: `structured_diff_preview`
- Idempotency: `non_idempotent_observable_only`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.clone_repository:2026.04.22-01`
- Primary label ref: `label:workspace.clone_repository:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:workspace:clone_repository_overview`

#### Aliases

- `alias:workspace.clone_repository:get_code` (`alternate_palette_phrasing`, `active`), introduced release:aureline:1.0, import impact `no_action_required`
- `alias:workspace.clone_repository:cli_clone` (`alternate_cli_verb`, `active`), introduced release:aureline:1.0, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `remote_repository_ref` | `provider_ref` | `true` | `-` |
| `destination_root_ref` | `path_ref_opaque` | `false` | `-` |
| `open_after_clone` | `boolean_flag` | `false` | `-` |

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, ai_tool_surface, docs_help, onboarding`
- Current disabled reasons: `required_provider_unlinked, execution_context_unavailable`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `false`
- AI eligible: `true`
- Automation labels: `recipe_safe, headless_safe`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Clone Repository` |
| `command_id` | `cmd:workspace.clone_repository` |
| `canonical_verb` | `workspace.clone_repository` |
| `alias_id` | `alias:workspace.clone_repository:get_code` |
| `alias_id` | `alias:workspace.clone_repository:cli_clone` |

#### Discoverability links

- `docs_help` -> `projection:workspace.clone_repository:docs_help_page`
- `onboarding` -> `projection:workspace.clone_repository:onboarding_hint`
- `command_palette` -> `projection:workspace.clone_repository:palette_row`

### `cmd:workspace.import_profile` -- Import Profile

Import settings, keybindings, snippets, and compatible state from a portable profile or competitor export.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `recoverable_durable_mutation`
- Preview class: `structured_diff_preview`
- Idempotency: `non_idempotent_observable_only`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.import_profile:2026.04.22-01`
- Primary label ref: `label:workspace.import_profile:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:migration:import_profile_overview`

#### Aliases

- `alias:workspace.import_profile:import_settings` (`alternate_palette_phrasing`, `active`), introduced release:aureline:1.0, import impact `no_action_required`
- `alias:workspace.import_profile:cli_import_profile` (`alternate_cli_verb`, `active`), introduced release:aureline:1.0, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `import_source_ref` | `opaque_id_ref` | `true` | `-` |
| `apply_scope` | `string_enum` | `false` | `-` |
| `create_restore_checkpoint` | `boolean_flag` | `false` | `-` |

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `policy_gate_present`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, docs_help, onboarding`
- Current disabled reasons: `capability_disabled_by_policy, required_argument_unresolved`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `true`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `recipe_safe, headless_safe, ai_callable_with_approval`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Import Profile` |
| `command_id` | `cmd:workspace.import_profile` |
| `canonical_verb` | `workspace.import_profile` |
| `alias_id` | `alias:workspace.import_profile:import_settings` |
| `alias_id` | `alias:workspace.import_profile:cli_import_profile` |

#### Discoverability links

- `docs_help` -> `projection:workspace.import_profile:docs_help_page`
- `onboarding` -> `projection:workspace.import_profile:onboarding_hint`
- `command_palette` -> `projection:workspace.import_profile:palette_row`

### `cmd:workspace.restore_from_checkpoint` -- Restore from Checkpoint

Compare the current workspace against a saved checkpoint, then restore the selected scope with a visible rollback path.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `recoverable_durable_mutation`
- Preview class: `structured_diff_preview`
- Idempotency: `non_idempotent_observable_only`
- Supports dry run: `true`
- Descriptor revision: `cmd-rev:workspace.restore_from_checkpoint:2026.04.22-01`
- Primary label ref: `label:workspace.restore_from_checkpoint:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:workspace:restore_from_checkpoint_overview`

#### Aliases

- `alias:workspace.restore_from_checkpoint:restore_workspace` (`alternate_palette_phrasing`, `active`), introduced release:aureline:1.0, import impact `no_action_required`
- `alias:workspace.restore_from_checkpoint:cli_restore` (`alternate_cli_verb`, `active`), introduced release:aureline:1.0, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `checkpoint_ref` | `opaque_id_ref` | `true` | `-` |
| `restore_scope` | `string_enum` | `false` | `-` |
| `create_safety_checkpoint` | `boolean_flag` | `false` | `-` |

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, cli_headless, docs_help, onboarding`
- Current disabled reasons: `required_argument_unresolved, basis_snapshot_drifted`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `true`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `headless_safe, ai_callable_with_approval`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Restore from Checkpoint` |
| `command_id` | `cmd:workspace.restore_from_checkpoint` |
| `canonical_verb` | `workspace.restore_from_checkpoint` |
| `alias_id` | `alias:workspace.restore_from_checkpoint:restore_workspace` |
| `alias_id` | `alias:workspace.restore_from_checkpoint:cli_restore` |

#### Discoverability links

- `docs_help` -> `projection:workspace.restore_from_checkpoint:docs_help_page`
- `onboarding` -> `projection:workspace.restore_from_checkpoint:onboarding_hint`
- `command_palette` -> `projection:workspace.restore_from_checkpoint:palette_row`

### `cmd:command_palette.open` -- Open Command Palette

Open the universal command launcher without minting a second command id for imported keymaps or help surfaces.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `idempotent`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:command_palette.open:2026.04.22-01`
- Primary label ref: `label:command_palette.open:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:command_palette:open_overview`

#### Aliases

- `alias:command_palette.open:vscode_show_commands` (`legacy_command_id`, `active`), introduced migration:vscode:default-keymap, import impact `rewrite_recipe`
- `alias:command_palette.open:show_all_commands` (`alternate_palette_phrasing`, `active`), introduced release:aureline:1.0, import impact `no_action_required`
- `alias:command_palette.open:ctrl_shift_p_target` (`keybinding_target`, `active`), introduced migration:vscode:shortcut-digest, import impact `rebind_keymap`
- `alias:command_palette.open:retired_command_menu` (`legacy_command_id`, `retired`), introduced release:aureline:0.8, retires release:aureline:1.0, import impact `rewrite_recipe`

#### Deprecation

- State: `active`

#### Argument schema

- none

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, docs_help, onboarding`
- Current disabled reasons: `client_scope_excludes_surface, execution_context_unavailable`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+shift+p` | `macos` | `overriding_user_binding` | - |
| `chord:ctrl+shift+p` | `windows` | `overriding_user_binding` | - |
| `chord:ctrl+shift+p` | `linux` | `overriding_user_binding` | - |

#### Automation

- Headless eligible: `false`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `ui_only`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Open Command Palette` |
| `command_id` | `cmd:command_palette.open` |
| `canonical_verb` | `command_palette.open` |
| `alias_id` | `alias:command_palette.open:vscode_show_commands` |
| `alias_id` | `alias:command_palette.open:show_all_commands` |
| `alias_id` | `alias:command_palette.open:ctrl_shift_p_target` |
| `key_sequence` | `chord:cmd+shift+p` |
| `key_sequence` | `chord:ctrl+shift+p` |
| `key_sequence` | `chord:ctrl+shift+p` |

#### Discoverability links

- `docs_help` -> `projection:command_palette.open:docs_help_page`
- `onboarding` -> `projection:command_palette.open:onboarding_hint`
- `command_palette` -> `projection:command_palette.open:palette_row`

### `cmd:docs.open_in_browser` -- Open in Browser

Open the current docs or help destination in the system browser through the governed browser-handoff path.

- Lifecycle: `stable`
- Origin: `built_in_extension`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `idempotent`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:docs.open_in_browser:2026.04.22-01`
- Primary label ref: `label:docs.open_in_browser:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:docs:open_in_browser_overview`

#### Aliases

- `alias:docs.open_in_browser:open_docs_mirror` (`alternate_palette_phrasing`, `active`), introduced release:aureline:1.0, import impact `no_action_required`

#### Deprecation

- State: `active`

#### Argument schema

| Argument | Kind | Required | Default provenance |
| -------- | ---- | -------- | ------------------ |
| `destination_anchor_ref` | `docs_anchor_ref` | `true` | `-` |
| `destination_descriptor_ref` | `opaque_id_ref` | `false` | `-` |

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `policy_gate_present`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, docs_help, onboarding`
- Current disabled reasons: `policy_blocked_in_context, client_scope_excludes_surface`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:` | `all` | `unassigned` | - |

#### Automation

- Headless eligible: `false`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `ui_only`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Open in Browser` |
| `command_id` | `cmd:docs.open_in_browser` |
| `canonical_verb` | `docs.open_in_browser` |
| `alias_id` | `alias:docs.open_in_browser:open_docs_mirror` |

#### Discoverability links

- `docs_help` -> `projection:docs.open_in_browser:docs_help_page`
- `onboarding` -> `projection:docs.open_in_browser:onboarding_hint`
- `command_palette` -> `projection:docs.open_in_browser:palette_row`

### `cmd:terminal.toggle` -- Toggle Terminal

Focus the bottom-panel terminal and ensure a workspace shell session is available.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `non_idempotent_destructive`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:terminal.toggle:2026.05.12-01`
- Primary label ref: `label:terminal.toggle:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:terminal:toggle`

#### Aliases

- `alias:terminal.toggle:vscode_integrated_terminal` (`legacy_command_id`, `active`), introduced release:aureline:0.1, import impact `rewrite_recipe`
- `alias:terminal.toggle:backtick_target` (`keybinding_target`, `active`), introduced release:aureline:0.1, import impact `rebind_keymap`

#### Deprecation

- State: `active`

#### Argument schema

- none

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, docs_help, onboarding`
- Current disabled reasons: `execution_context_unavailable`

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+backquote` | `macos` | `overriding_user_binding` | - |
| `chord:ctrl+backquote` | `windows` | `overriding_user_binding` | - |
| `chord:ctrl+backquote` | `linux` | `overriding_user_binding` | - |

#### Automation

- Headless eligible: `false`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `ui_only`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Toggle Terminal` |
| `command_id` | `cmd:terminal.toggle` |
| `canonical_verb` | `terminal.toggle` |
| `alias_id` | `alias:terminal.toggle:vscode_integrated_terminal` |
| `alias_id` | `alias:terminal.toggle:backtick_target` |
| `key_sequence` | `chord:cmd+backquote` |
| `key_sequence` | `chord:ctrl+backquote` |
| `key_sequence` | `chord:ctrl+backquote` |

#### Discoverability links

- `docs_help` -> `projection:terminal.toggle:docs_help_page`
- `onboarding` -> `projection:terminal.toggle:onboarding_hint`
- `command_palette` -> `projection:terminal.toggle:palette_row`

### `cmd:explorer.toggle` -- Toggle Explorer

Focus the left-sidebar file explorer for the active workspace.

- Lifecycle: `stable`
- Origin: `core`
- Risk class: `reversible_local_mutation`
- Preview class: `no_preview_required`
- Idempotency: `idempotent`
- Supports dry run: `false`
- Descriptor revision: `cmd-rev:explorer.toggle:2026.05.12-01`
- Primary label ref: `label:explorer.toggle:accessibility_primary`
- Docs/help anchor: `pack:project:aureline:01#docs:anchor:explorer:toggle`

#### Aliases

- `alias:explorer.toggle:vscode_show_explorer` (`legacy_command_id`, `active`), introduced release:aureline:0.1, import impact `rewrite_recipe`
- `alias:explorer.toggle:shift_e_target` (`keybinding_target`, `active`), introduced release:aureline:0.1, import impact `rebind_keymap`

#### Deprecation

- State: `active`

#### Argument schema

- none

#### Availability

- Trust gate: `no_trust_gate`
- Policy gate: `no_policy_gate`
- Dependency presence: `no_dependency_required`
- Supported surfaces: `command_palette, menu_or_button, keybinding_help, docs_help, onboarding`
- Current disabled reasons: none

#### Keybindings

| Chord | Platform | State | Shadowed by |
| ----- | -------- | ----- | ----------- |
| `chord:cmd+shift+e` | `macos` | `overriding_user_binding` | - |
| `chord:ctrl+shift+e` | `windows` | `overriding_user_binding` | - |
| `chord:ctrl+shift+e` | `linux` | `overriding_user_binding` | - |

#### Automation

- Headless eligible: `false`
- Recipe eligible: `false`
- Macro eligible: `false`
- AI eligible: `false`
- Automation labels: `ui_only`

#### Search index

| Token class | Value |
| ----------- | ----- |
| `human_label` | `Toggle Explorer` |
| `command_id` | `cmd:explorer.toggle` |
| `canonical_verb` | `explorer.toggle` |
| `alias_id` | `alias:explorer.toggle:vscode_show_explorer` |
| `alias_id` | `alias:explorer.toggle:shift_e_target` |
| `key_sequence` | `chord:cmd+shift+e` |
| `key_sequence` | `chord:ctrl+shift+e` |
| `key_sequence` | `chord:ctrl+shift+e` |

#### Discoverability links

- `docs_help` -> `projection:explorer.toggle:docs_help_page`
- `onboarding` -> `projection:explorer.toggle:onboarding_hint`
- `command_palette` -> `projection:explorer.toggle:palette_row`

## Verification

```sh
cargo test -p aureline-shell --test command_reference_fixtures
```
