# Top-incumbent migration scoreboard (beta)

Generated from the seeded migration corpus in
[`crate::migration_corpus`](../../../crates/aureline-shell/src/migration_corpus/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- scoreboard-md > \
  artifacts/migration/m3/migration_scoreboard.md
```

- Scoreboard id: `shell:migration_corpus_beta:scoreboard:v1`
- Wizard session: `shell:migration-wizard:import-review-3bcae9aef7bd1cab`
- Wizard mapping report: `mapping-report:import-review-3bcae9aef7bd1cab`
- Rollback checkpoint: `rollback-checkpoint:import-review-3bcae9aef7bd1cab`
- Generated at: `2026-05-15T00:00:00Z`

## Overall classification summary

| Exact | Translated | Partial | Shimmed | Unsupported | Total |
|------:|-----------:|--------:|--------:|------------:|------:|
| 4 | 4 | 4 | 4 | 4 | 20 |

## VS Code / Code-OSS (`vs_code_code_oss`)

Source ecosystem row: `migration_source:vs_code_code_oss`

| Exact | Translated | Partial | Shimmed | Unsupported |
|------:|-----------:|--------:|--------:|------------:|
| 1 | 1 | 1 | 1 | 1 |

| Flow | Domain | Classification | Source | Aureline target |
| ---- | ------ | -------------- | ------ | --------------- |
| `migration-corpus-flow:vs_code_code_oss:command-palette-shortcut` -- Command palette shortcut | Shortcuts | **Translated** | .vscode/keybindings.json -- workbench.action.showCommands | aureline:command.palette.open |
| `migration-corpus-flow:vs_code_code_oss:eslint-native-replacement` -- ESLint extension as native replacement | Extensions and providers | **Shimmed** | vscode:extension:dbaeumer.vscode-eslint | aureline:package:eslint-native-lint |
| `migration-corpus-flow:vs_code_code_oss:high-frequency-keymap-chord` -- High-frequency keymap chord remap | Keymaps | **Partial** | .vscode/keybindings.json -- multi-key chord | aureline:keymaps.shortcut_delta_digest |
| `migration-corpus-flow:vs_code_code_oss:settings` -- User and workspace settings | Settings | **Exact** | .vscode/settings.json | Aureline user and workspace setting records |
| `migration-corpus-flow:vs_code_code_oss:webview-extension-runtime` -- Webview-heavy extension runtime | Extensions and providers | **Unsupported** | vscode:extension:sample.webview-tool | (no safe target) |

### `migration-corpus-flow:vs_code_code_oss:command-palette-shortcut` -- Command palette shortcut (Translated)

- Domain: Shortcuts
- Source: .vscode/keybindings.json -- workbench.action.showCommands
- Aureline target: aureline:command.palette.open
- Before/after: VS Code command palette gesture maps to the Aureline palette command id.
- Caveat: Translation depends on the Aureline command id remaining stable across the keybinding resolver.
- Downgrade triggers:
  - `aureline_command_id_renamed`
  - `vscode_command_id_renamed`
  - `keybinding_resolver_layer_changed`
- Evidence:
  - `fixtures/migration/equivalence_cases/vscode_shortcut_translated.yaml`
  - `fixtures/migration/m3/migration_wizard/mapping_report.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vscode`
  - `docs/migration/keymap_presets.md`

### `migration-corpus-flow:vs_code_code_oss:eslint-native-replacement` -- ESLint extension as native replacement (Shimmed)

- Domain: Extensions and providers
- Source: vscode:extension:dbaeumer.vscode-eslint
- Aureline target: aureline:package:eslint-native-lint
- Before/after: Source extension is recorded and the native lint package is recommended; runtime is not bridged silently.
- Caveat: Native replacement does not import source extension authority, storage, or webview state.
- Downgrade triggers:
  - `native_package_or_command_changed`
  - `permission_or_policy_vocab_changed`
  - `extension_recommendation_evidence_expired`
- Evidence:
  - `fixtures/migration/compatibility_scorecards/native_alternative_recommendation.json`
  - `artifacts/migration/top_imported_workflow_rows.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vscode`
  - `docs/migration/compatibility_scorecard_contract.md`

### `migration-corpus-flow:vs_code_code_oss:high-frequency-keymap-chord` -- High-frequency keymap chord remap (Partial)

- Domain: Keymaps
- Source: .vscode/keybindings.json -- multi-key chord
- Aureline target: aureline:keymaps.shortcut_delta_digest
- Before/after: Most chord targets map, but the destination chord capacity remaps a small set of high-frequency shortcuts.
- Caveat: Muscle-memory risk for the remapped chord stays visible in the shortcut delta digest until the user accepts the change.
- Downgrade triggers:
  - `shortcut_delta_digest_changed`
  - `platform_reserved_chord_changed`
  - `keybinding_resolver_layer_changed`
- Evidence:
  - `fixtures/commands/keybinding_conflict_examples/high_frequency_shortcut_diff_after_import.json`
  - `fixtures/migration/m3/migration_wizard/mapping_report.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vscode`
  - `docs/migration/migration_restore_and_shortcut_delta_packet.md`

### `migration-corpus-flow:vs_code_code_oss:settings` -- User and workspace settings (Exact)

- Domain: Settings
- Source: .vscode/settings.json
- Aureline target: Aureline user and workspace setting records
- Before/after: settings.json keys map to stable Aureline setting ids without semantic loss.
- Evidence:
  - `fixtures/migration/equivalence_cases/vscode_setting_exact.yaml`
  - `fixtures/migration/m3/migration_wizard/mapping_report.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vscode`
  - `docs/migration/source_ecosystem_coverage_matrix.md`

### `migration-corpus-flow:vs_code_code_oss:webview-extension-runtime` -- Webview-heavy extension runtime (Unsupported)

- Domain: Extensions and providers
- Source: vscode:extension:sample.webview-tool
- Aureline target: (no safe target)
- Before/after: Source webview runtime state has no governed Aureline target and apply is denied.
- Caveat: Extension runtime parity, arbitrary webview state, and source extension storage are not imported.
- Downgrade triggers:
  - `webview_governance_contract_changed`
  - `extension_runtime_policy_changed`
- Evidence:
  - `fixtures/migration/compatibility_scorecards/unsupported_webview_extension.json`
  - `fixtures/migration/m3/migration_wizard/unsupported_gaps.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vscode`
  - `docs/migration/source_ecosystem_coverage_matrix.md`

## JetBrains IDEs (`jetbrains_family`)

Source ecosystem row: `migration_source:jetbrains_family`

| Exact | Translated | Partial | Shimmed | Unsupported |
|------:|-----------:|--------:|--------:|------------:|
| 1 | 1 | 1 | 1 | 1 |

| Flow | Domain | Classification | Source | Aureline target |
| ---- | ------ | -------------- | ------ | --------------- |
| `migration-corpus-flow:jetbrains_family:code-style-hints` -- Code style and formatter hints | Settings | **Exact** | JetBrains formatter profile (common keys) | Aureline settings and formatter hint records |
| `migration-corpus-flow:jetbrains_family:common-keymap-preset` -- Common IDE keymap preset | Keymaps | **Translated** | JetBrains keymap export (common preset) | aureline:keymaps.jetbrains_preset |
| `migration-corpus-flow:jetbrains_family:plugin-runtime` -- Source IDE plugin runtime | Extensions and providers | **Unsupported** | JetBrains plugin runtime (arbitrary plugin) | (no safe target) |
| `migration-corpus-flow:jetbrains_family:project-root-handoff` -- Project root and module content roots | Workspace profile | **Shimmed** | JetBrains project root and module content roots | aureline:workspace.manifest.roots |
| `migration-corpus-flow:jetbrains_family:run-debug-config` -- Application run/debug configuration | Launch and debug | **Partial** | JetBrains run configuration: app-server | aureline:task-candidate:app-server |

### `migration-corpus-flow:jetbrains_family:code-style-hints` -- Code style and formatter hints (Exact)

- Domain: Settings
- Source: JetBrains formatter profile (common keys)
- Aureline target: Aureline settings and formatter hint records
- Before/after: Common formatter and indentation keys map directly to Aureline settings.
- Evidence:
  - `fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#jetbrains`

### `migration-corpus-flow:jetbrains_family:common-keymap-preset` -- Common IDE keymap preset (Translated)

- Domain: Keymaps
- Source: JetBrains keymap export (common preset)
- Aureline target: aureline:keymaps.jetbrains_preset
- Before/after: Common navigation and editing chords translate to Aureline command ids through the keymap preset.
- Caveat: Preset translation excludes plugin-specific actions and source-only IDE concepts.
- Downgrade triggers:
  - `aureline_command_id_renamed`
  - `jetbrains_action_id_renamed`
  - `keybinding_resolver_layer_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml`
  - `fixtures/migration/equivalence_cases/jetbrains_run_debug_needs_manual_review.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#jetbrains`
  - `docs/migration/keymap_presets.md`

### `migration-corpus-flow:jetbrains_family:plugin-runtime` -- Source IDE plugin runtime (Unsupported)

- Domain: Extensions and providers
- Source: JetBrains plugin runtime (arbitrary plugin)
- Aureline target: (no safe target)
- Before/after: Source IDE plugin runtime has no Aureline native or bridge path; apply is denied for the runtime.
- Caveat: Source IDE indexes, generated project models, plugin runtime state, and run history are not imported as native truth.
- Downgrade triggers:
  - `extension_runtime_policy_changed`
  - `compat_row_extension_host_changed`
- Evidence:
  - `artifacts/migration/source_ecosystem_rows.yaml`
  - `fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#jetbrains`
  - `docs/migration/source_ecosystem_coverage_matrix.md`

### `migration-corpus-flow:jetbrains_family:project-root-handoff` -- Project root and module content roots (Shimmed)

- Domain: Workspace profile
- Source: JetBrains project root and module content roots
- Aureline target: aureline:workspace.manifest.roots
- Before/after: Module content roots map through a workspace-manifest shim that preserves provenance without claiming index parity.
- Caveat: Shimmed continuity does not preserve source IDE indexing semantics or generated project models.
- Downgrade triggers:
  - `workspace_manifest_schema_changed`
  - `compat_row_workspace_profile_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#jetbrains`

### `migration-corpus-flow:jetbrains_family:run-debug-config` -- Application run/debug configuration (Partial)

- Domain: Launch and debug
- Source: JetBrains run configuration: app-server
- Aureline target: aureline:task-candidate:app-server
- Before/after: Common fields map to a candidate execution-context record; full runnable parity needs review.
- Caveat: Run/debug import is limited to configurations with a clear Aureline execution-context equivalent.
- Downgrade triggers:
  - `source_profile_fixture_changed`
  - `native_package_or_command_changed`
  - `post_import_validation_state_changed`
- Evidence:
  - `fixtures/migration/equivalence_cases/jetbrains_run_debug_needs_manual_review.yaml`
  - `fixtures/migration/compatibility_scorecards/partial_run_debug_translation.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#jetbrains`
  - `docs/migration/post_import_validation_contract.md`

## Vim / Neovim (`vim_neovim`)

Source ecosystem row: `migration_source:vim_neovim`

| Exact | Translated | Partial | Shimmed | Unsupported |
|------:|-----------:|--------:|--------:|------------:|
| 1 | 1 | 1 | 1 | 1 |

| Flow | Domain | Classification | Source | Aureline target |
| ---- | ------ | -------------- | ------ | --------------- |
| `migration-corpus-flow:vim_neovim:clipboard-search-defaults-shim` -- Clipboard and search defaults | Settings | **Shimmed** | Vim/neovim clipboard and search option defaults | aureline:settings.modal_profile_shim |
| `migration-corpus-flow:vim_neovim:leader-key-mappings` -- Leader-key mappings | Shortcuts | **Translated** | Vim leader-key mappings | aureline:keymaps.leader_overlay |
| `migration-corpus-flow:vim_neovim:lua-plugin-runtime` -- Lua plugin runtime | Extensions and providers | **Unsupported** | Arbitrary Lua plugin runtime | (no safe target) |
| `migration-corpus-flow:vim_neovim:modal-editing-profile` -- Modal editing profile (normal, visual, operator) | Keymaps | **Exact** | Curated vim/neovim modal profile | aureline:modal_editing.profile |
| `migration-corpus-flow:vim_neovim:snippet-directories` -- Selected snippet directories | Snippets and templates | **Partial** | Selected vim/neovim snippet directories | Aureline snippet/template records |

### `migration-corpus-flow:vim_neovim:clipboard-search-defaults-shim` -- Clipboard and search defaults (Shimmed)

- Domain: Settings
- Source: Vim/neovim clipboard and search option defaults
- Aureline target: aureline:settings.modal_profile_shim
- Before/after: Clipboard and search options are preserved through a modal-profile shim that names the source semantics explicitly.
- Caveat: Shim preserves source intent; source register history and macro history are not imported.
- Downgrade triggers:
  - `modal_profile_shim_changed`
  - `clipboard_capability_layer_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/vim_neovim_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vim`

### `migration-corpus-flow:vim_neovim:leader-key-mappings` -- Leader-key mappings (Translated)

- Domain: Shortcuts
- Source: Vim leader-key mappings
- Aureline target: aureline:keymaps.leader_overlay
- Before/after: Leader-key chains map to a leader overlay that names every translated Aureline command id.
- Caveat: Translation excludes mappings that call into Vimscript or Lua plugin runtimes.
- Downgrade triggers:
  - `leader_overlay_schema_changed`
  - `aureline_command_id_renamed`
- Evidence:
  - `fixtures/migration/source_profile_examples/vim_neovim_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vim`

### `migration-corpus-flow:vim_neovim:lua-plugin-runtime` -- Lua plugin runtime (Unsupported)

- Domain: Extensions and providers
- Source: Arbitrary Lua plugin runtime
- Aureline target: (no safe target)
- Before/after: Plugin runtime execution has no governed Aureline path; apply is denied for the runtime.
- Caveat: Arbitrary Vimscript/Lua plugin execution, register history, and macro history are outside the migration claim.
- Downgrade triggers:
  - `lua_runtime_policy_changed`
  - `compat_row_extension_host_changed`
- Evidence:
  - `fixtures/migration/equivalence_cases/vim_plugin_unsupported.yaml`
  - `fixtures/migration/compatibility_scorecards/blocked_lua_plugin_runtime.json`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vim`
  - `docs/migration/source_ecosystem_coverage_matrix.md`

### `migration-corpus-flow:vim_neovim:modal-editing-profile` -- Modal editing profile (normal, visual, operator) (Exact)

- Domain: Keymaps
- Source: Curated vim/neovim modal profile
- Aureline target: aureline:modal_editing.profile
- Before/after: Normal, visual, and operator mappings map directly to the Aureline modal-editing profile record.
- Evidence:
  - `fixtures/migration/source_profile_examples/vim_neovim_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vim`
  - `docs/migration/keymap_presets.md`

### `migration-corpus-flow:vim_neovim:snippet-directories` -- Selected snippet directories (Partial)

- Domain: Snippets and templates
- Source: Selected vim/neovim snippet directories
- Aureline target: Aureline snippet/template records
- Before/after: Standard snippet bodies import; trigger metadata that depends on plugin runtime is held for manual review.
- Caveat: Snippets that depend on a runtime plugin engine require manual review before they are recommended.
- Downgrade triggers:
  - `snippet_engine_compat_changed`
  - `source_profile_fixture_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/vim_neovim_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#vim`

## Emacs (`emacs`)

Source ecosystem row: `migration_source:emacs`

| Exact | Translated | Partial | Shimmed | Unsupported |
|------:|-----------:|--------:|--------:|------------:|
| 1 | 1 | 1 | 1 | 1 |

| Flow | Domain | Classification | Source | Aureline target |
| ---- | ------ | -------------- | ------ | --------------- |
| `migration-corpus-flow:emacs:command-aliases` -- Interactive command aliases | Shortcuts | **Exact** | Emacs M-x command aliases | Aureline command aliases and palette metadata |
| `migration-corpus-flow:emacs:elisp-runtime` -- Elisp package runtime | Extensions and providers | **Unsupported** | Arbitrary Elisp package runtime | (no safe target) |
| `migration-corpus-flow:emacs:global-keymap` -- Global keymap and selected presets | Keymaps | **Translated** | Emacs global keymap | aureline:keymaps.emacs_preset |
| `migration-corpus-flow:emacs:project-defaults` -- Project root and file-exclude defaults | Workspace profile | **Partial** | Emacs project.el roots and excludes | aureline:workspace.manifest_and_settings |
| `migration-corpus-flow:emacs:theme-token-shim` -- Selected color theme | Themes and visuals | **Shimmed** | Emacs selected color theme | aureline:themes.token_mapping |

### `migration-corpus-flow:emacs:command-aliases` -- Interactive command aliases (Exact)

- Domain: Shortcuts
- Source: Emacs M-x command aliases
- Aureline target: Aureline command aliases and palette metadata
- Before/after: Common interactive command aliases map directly to Aureline command palette metadata.
- Evidence:
  - `fixtures/migration/source_profile_examples/emacs_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#emacs`

### `migration-corpus-flow:emacs:elisp-runtime` -- Elisp package runtime (Unsupported)

- Domain: Extensions and providers
- Source: Arbitrary Elisp package runtime
- Aureline target: (no safe target)
- Before/after: Elisp execution has no governed Aureline path; apply is denied for the runtime.
- Caveat: Elisp execution, package runtime parity, live buffers, and org-mode runtime parity are outside the claim.
- Downgrade triggers:
  - `elisp_runtime_policy_changed`
  - `compat_row_extension_host_changed`
- Evidence:
  - `fixtures/migration/compatibility_scorecards/blocked_elisp_package_runtime.json`
  - `fixtures/migration/source_profile_examples/emacs_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#emacs`
  - `docs/migration/source_ecosystem_coverage_matrix.md`

### `migration-corpus-flow:emacs:global-keymap` -- Global keymap and selected presets (Translated)

- Domain: Keymaps
- Source: Emacs global keymap
- Aureline target: aureline:keymaps.emacs_preset
- Before/after: Common global chords translate to Aureline command ids through the keymap preset.
- Caveat: Preset translation excludes Elisp-driven and mode-specific dynamic bindings.
- Downgrade triggers:
  - `aureline_command_id_renamed`
  - `emacs_command_alias_changed`
  - `keybinding_resolver_layer_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/emacs_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#emacs`
  - `docs/migration/keymap_presets.md`

### `migration-corpus-flow:emacs:project-defaults` -- Project root and file-exclude defaults (Partial)

- Domain: Workspace profile
- Source: Emacs project.el roots and excludes
- Aureline target: aureline:workspace.manifest_and_settings
- Before/after: Project roots and common excludes import; Elisp-driven overrides require manual review.
- Caveat: Project semantics derived from Elisp init files are not evaluated and may need manual review.
- Downgrade triggers:
  - `workspace_manifest_schema_changed`
  - `post_import_validation_state_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/emacs_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#emacs`
  - `docs/migration/post_import_validation_contract.md`

### `migration-corpus-flow:emacs:theme-token-shim` -- Selected color theme (Shimmed)

- Domain: Themes and visuals
- Source: Emacs selected color theme
- Aureline target: aureline:themes.token_mapping
- Before/after: Theme palette imports through a token-mapping shim that names every translated face explicitly.
- Caveat: Faces that depend on Elisp evaluation are surfaced as unsupported sub-rows rather than silently dropped.
- Downgrade triggers:
  - `theme_token_schema_changed`
  - `design_token_vocabulary_changed`
- Evidence:
  - `fixtures/migration/source_profile_examples/emacs_profile.yaml`
- Docs/help:
  - `docs/migration/m3/incumbent_flow_matrix.md#emacs`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_migration_corpus -- validate
cargo test -p aureline-shell --test migration_corpus_fixtures
```
