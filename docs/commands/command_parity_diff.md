# Command parity diff seed report

This report is emitted from `artifacts/commands/command_parity_seed.yaml` by `tools/commands/parity_diff_seed.py`. It is a seed-only parity corpus for launch-bearing command surfaces, not a claim that live runtime surfaces already ship.

## Report contract

### Surface families

| Surface family | Meaning | Default authority |
|---|---|---|
| `command_palette` | Palette rows and inline palette actions. | `user_initiated_local` |
| `menu_or_button` | Application menus, context menus, toolbar buttons, or other launch-bearing buttons. | `user_initiated_local` |
| `keybinding_help` | Keybinding help, shortcut help, and conflict-resolution surfaces that narrate a direct keybinding route. | `user_initiated_local` |
| `cli_help` | CLI or headless help surfaces that document the command route. | `user_initiated_local` |
| `ai_tool_surface` | AI or governed tool surfaces that call the command by stable identity. | `ai_initiated` |

### Comparison axes

| Axis | Canonical source |
|---|---|
| `stable_command_id` | Descriptor `command_id` projected onto every claimed surface row. |
| `label_or_alias` | Descriptor `primary_label_ref`, `canonical_verb`, and declared alias ids. |
| `enablement_rules` | Descriptor `ui_slot_hints`, `palette_visibility`, `client_scopes`, and typed disabled-reason disclosure. |
| `preview_posture` | Descriptor `preview_class` carried through palette, menu/button, help, CLI, and AI routes. |
| `authority_class` | Surface-family default authority lane (`user_initiated_local` or `ai_initiated`) without widening command semantics. |
| `result_contract` | Descriptor `result_contract_class` and `evidence_ref_class_required`. |

### Failure categories

| Category | Severity | Trigger |
|---|---|---|
| `unknown_high_risk_gap` | `actionable` | A high-risk command has a required surface family marked as unknown rather than claimed or explicitly narrowed. |
| `mismatched_preview_posture` | `actionable` | A claimed surface reports a preview posture that differs from the canonical command descriptor. |
| `missing_disabled_reason` | `actionable` | A claimed surface does not preserve typed disabled-reason disclosure for an unavailable command. |
| `missing_help_docs_anchor` | `actionable` | A claimed surface omits the canonical docs/help anchor or cannot point back to the command reference. |
| `surface_specific_hidden_alias` | `actionable` | A surface carries an alias that is hidden from parity consumers instead of being explicitly disclosed. |
| `command_id_drift` | `actionable` | A claimed surface does not carry the canonical stable command ID. |
| `label_or_alias_drift` | `actionable` | A claimed surface renames the command or exposes aliases outside the descriptor-owned alias set. |
| `enablement_rule_drift` | `actionable` | A claimed surface widens or narrows the descriptor-owned enablement signature without an explicit narrowing entry. |
| `authority_class_drift` | `actionable` | A claimed surface reports an unexpected authority class for the route it represents. |
| `result_contract_drift` | `actionable` | A claimed surface disagrees with the descriptor-owned result contract class or required evidence refs. |

## Seed summary

- Seed id: `command_parity.seed.launch_bearing.01`
- Commands: `5`
- Claimed surface rows: `21`
- Explicit narrowings: `3`
- Unknown gaps: `1`
- Actionable findings: `5`

### Findings by category

| Category | Count |
|---|---|
| `mismatched_preview_posture` | `1` |
| `missing_disabled_reason` | `1` |
| `missing_help_docs_anchor` | `1` |
| `surface_specific_hidden_alias` | `1` |
| `unknown_high_risk_gap` | `1` |

## Actionable findings

| Category | Command | Surface | Detail |
|---|---|---|---|
| `surface_specific_hidden_alias` | `cmd:search.find_in_workspace` | `cli_help` | surface hides alias ids from parity consumers: alias:search.find_in_workspace:cli_search |
| `mismatched_preview_posture` | `cmd:git.push_branch` | `cli_help` | surface reports no_preview_required but descriptor requires irreversible_publish_preview |
| `unknown_high_risk_gap` | `cmd:workspace.reset_to_snapshot` | `ai_tool_surface` | high-risk command has no claim or explicit narrowing for this surface family |
| `missing_disabled_reason` | `cmd:settings.edit_managed_policy` | `command_palette` | surface drops typed disabled-reason disclosure |
| `missing_help_docs_anchor` | `cmd:settings.edit_managed_policy` | `menu_or_button` | surface cannot point back to the canonical docs/help anchor |

## cmd:workspace.open_folder

- Descriptor: `fixtures/commands/command_descriptor_examples/workspace_open_folder.json`
- Canonical verb: `workspace.open_folder`
- Risk class: `standard-risk`
- Seed notes: Clean baseline covering all serious-user surfaces with no deliberate parity drift.

| Surface | Status | Slot | Preview | Authority | Result | Findings |
|---|---|---|---|---|---|---|
| `command_palette` | `claimed` | `command_palette.result_row` | `no_preview_required` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `menu_or_button` | `claimed` | `global_menu.command_item` | `no_preview_required` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `keybinding_help` | `claimed` | `keybinding_help.command_row` | `no_preview_required` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `cli_help` | `claimed` | `cli_help.command_row` | `no_preview_required` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `ai_tool_surface` | `claimed` | `ai_tool_surface.tool_entry` | `no_preview_required` | `ai_initiated` | `journal_entry_appended_ref` | - |

### Findings

- None.

## cmd:search.find_in_workspace

- Descriptor: `fixtures/commands/command_descriptor_examples/search_find_in_workspace.json`
- Canonical verb: `search.find_in_workspace`
- Risk class: `standard-risk`
- Seed notes: Deliberate hidden-alias mismatch on CLI help to prove the diff marks surface-local alias drift as actionable.

| Surface | Status | Slot | Preview | Authority | Result | Findings |
|---|---|---|---|---|---|---|
| `command_palette` | `claimed` | `command_palette.result_row` | `no_preview_required` | `user_initiated_local` | `typed_value_returned` | - |
| `menu_or_button` | `claimed` | `global_menu.command_item` | `no_preview_required` | `user_initiated_local` | `typed_value_returned` | - |
| `keybinding_help` | `claimed` | `keybinding_help.command_row` | `no_preview_required` | `user_initiated_local` | `typed_value_returned` | - |
| `cli_help` | `claimed` | `cli_help.command_row` | `no_preview_required` | `user_initiated_local` | `typed_value_returned` | surface_specific_hidden_alias |
| `ai_tool_surface` | `claimed` | `ai_tool_surface.tool_entry` | `no_preview_required` | `ai_initiated` | `typed_value_returned` | - |

### Findings

- `surface_specific_hidden_alias` on `cli_help`: surface hides alias ids from parity consumers: alias:search.find_in_workspace:cli_search

## cmd:git.push_branch

- Descriptor: `fixtures/commands/command_descriptor_examples/git_push_branch.json`
- Canonical verb: `git.push_branch`
- Risk class: `high-risk`
- Seed notes: High-risk command traced across all five surfaces. The CLI/help row deliberately lies about preview posture so the diff reports a reusable actionable mismatch.

| Surface | Status | Slot | Preview | Authority | Result | Findings |
|---|---|---|---|---|---|---|
| `command_palette` | `claimed` | `command_palette.result_row` | `irreversible_publish_preview` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `menu_or_button` | `claimed` | `context_menu.source_control_item` | `irreversible_publish_preview` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `keybinding_help` | `claimed` | `keybinding_help.command_row` | `irreversible_publish_preview` | `user_initiated_local` | `journal_entry_appended_ref` | - |
| `cli_help` | `claimed` | `cli_help.command_row` | `no_preview_required` | `user_initiated_local` | `journal_entry_appended_ref` | mismatched_preview_posture |
| `ai_tool_surface` | `claimed` | `ai_tool_surface.tool_entry` | `irreversible_publish_preview` | `ai_initiated` | `journal_entry_appended_ref` | - |

### Findings

- `mismatched_preview_posture` on `cli_help`: surface reports no_preview_required but descriptor requires irreversible_publish_preview

## cmd:workspace.reset_to_snapshot

- Descriptor: `fixtures/commands/command_descriptor_examples/workspace_reset_to_snapshot.json`
- Canonical verb: `workspace.reset_to_snapshot`
- Risk class: `high-risk`
- Seed notes: High-risk destructive command used to prove unknown gaps stay visible until a surface is either claimed or explicitly narrowed.

| Surface | Status | Slot | Preview | Authority | Result | Findings |
|---|---|---|---|---|---|---|
| `command_palette` | `claimed` | `command_palette.result_row` | `destructive_bulk_mutation_preview` | `user_initiated_local` | `audit_event_emitted_ref` | - |
| `menu_or_button` | `claimed` | `global_menu.command_item` | `destructive_bulk_mutation_preview` | `user_initiated_local` | `audit_event_emitted_ref` | - |
| `keybinding_help` | `claimed` | `keybinding_help.command_row` | `destructive_bulk_mutation_preview` | `user_initiated_local` | `audit_event_emitted_ref` | - |
| `cli_help` | `claimed` | `cli_help.command_row` | `destructive_bulk_mutation_preview` | `user_initiated_local` | `audit_event_emitted_ref` | - |
| `ai_tool_surface` | `unknown_gap` | `-` | `-` | `-` | `-` | unknown_high_risk_gap |

### Findings

- `unknown_high_risk_gap` on `ai_tool_surface`: high-risk command has no claim or explicit narrowing for this surface family

## cmd:settings.edit_managed_policy

- Descriptor: `fixtures/commands/command_descriptor_examples/settings_edit_managed_policy.json`
- Canonical verb: `settings.edit_managed_policy`
- Risk class: `high-risk`
- Seed notes: Managed-admin policy command used to seed missing disabled-reason and missing help-anchor failure categories.

| Surface | Status | Slot | Preview | Authority | Result | Findings |
|---|---|---|---|---|---|---|
| `command_palette` | `claimed` | `command_palette.result_row` | `policy_authoring_or_waiver_preview` | `user_initiated_local` | `audit_event_emitted_ref` | missing_disabled_reason |
| `menu_or_button` | `claimed` | `global_menu.command_item` | `policy_authoring_or_waiver_preview` | `user_initiated_local` | `audit_event_emitted_ref` | missing_help_docs_anchor |
| `keybinding_help` | `explicitly_narrowed (descriptor_not_claimed_on_surface)` | `-` | `-` | `-` | `-` | - |
| `cli_help` | `explicitly_narrowed (client_scope_excludes_surface)` | `-` | `-` | `-` | `-` | - |
| `ai_tool_surface` | `explicitly_narrowed (descriptor_ai_surface_denied)` | `-` | `-` | `-` | `-` | - |

### Findings

- `missing_disabled_reason` on `command_palette`: surface drops typed disabled-reason disclosure
- `missing_help_docs_anchor` on `menu_or_button`: surface cannot point back to the canonical docs/help anchor

## CI reuse

- Use `--format json` for machine-readable summaries.
- Use `--strict` to make actionable findings fail the invocation with exit code `1`.
- Keep the seed corpus synthetic until runtime surface exports land; later CI can swap the seed rows for generated surface captures without changing the report structure.
