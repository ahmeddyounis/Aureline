# Voice disambiguation and confirmation

Generated from the `voice_bridge` seed. Do not edit by hand; regenerate with `cargo run -p aureline-commands --example dump_voice_command_bridge -- write`.

- Packet: `commands:voice_command_bridge:packet:v1`
- Descriptor contract: `docs/commands/command_descriptor_contract.md`
- Result-packet schema: `schemas/commands/command_result_packet.schema.json`
- Voice/dictation contract: `docs/ux/voice_and_dictation_contract.md`
- Fixtures: `fixtures/voice/disambiguation`

## Recognized utterances

| Row | Intent | Gate | Selected | Impact | Correction | Undo group |
| --- | --- | --- | --- | --- | --- | --- |
| `voice:bridge:rename_symbol_across_project_confirm` | resolves_to_single_command | confirmation_required_before_commit | `cmd:edit.rename_symbol_across_project` | recoverable_durable_mutation | required_before_commit | `undo-group:edit.rename_symbol_across_project:voice:01` |
| `voice:bridge:rename_symbol_ambiguous` | ambiguous_requires_disambiguation | disambiguation_required_before_commit | `-` | - | offered_before_commit | `-` |
| `voice:bridge:push_current_branch_confirm` | resolves_to_single_command | confirmation_required_before_commit | `cmd:git.push_current_branch` | irreversible_publish | required_before_commit | `undo-group:git.push_current_branch:voice:01` |
| `voice:bridge:insert_dictated_text` | resolves_to_dictation_text | direct_commit_low_impact | `cmd:editor.insert_dictated_text` | reversible_local_mutation | offered_before_commit | `undo-group:editor.insert_dictated_text:voice:01` |
| `voice:bridge:go_to_definition_direct` | resolves_to_single_command | direct_commit_low_impact | `cmd:navigation.go_to_definition` | reversible_local_read | offered_before_commit | `undo-group:navigation.go_to_definition:voice:01` |
| `voice:bridge:denied_uncanonical_verb` | denied_no_canonical_command | blocked_no_canonical_command | `-` | - | offered_before_commit | `-` |

## Candidate parity

Each candidate carries the same stable command id, description, keyboard shortcut narration, and disabled reason the command palette projects.

| Row | Candidate | Enablement | Disabled reason | Impact | Preview | Approval |
| --- | --- | --- | --- | --- | --- | --- |
| `voice:bridge:rename_symbol_across_project_confirm` | `cmd:edit.rename_symbol_across_project` | enabled | `-` | recoverable_durable_mutation | true | false |
| `voice:bridge:rename_symbol_ambiguous` | `cmd:edit.rename_symbol_across_project` | enabled | `-` | recoverable_durable_mutation | true | false |
| `voice:bridge:rename_symbol_ambiguous` | `cmd:edit.rename_symbol_in_file` | disabled_with_reason | `workspace_trust_restricted` | reversible_local_mutation | false | false |
| `voice:bridge:push_current_branch_confirm` | `cmd:git.push_current_branch` | enabled | `-` | irreversible_publish | true | true |
| `voice:bridge:insert_dictated_text` | `cmd:editor.insert_dictated_text` | enabled | `-` | reversible_local_mutation | false | false |
| `voice:bridge:go_to_definition_direct` | `cmd:navigation.go_to_definition` | enabled | `-` | reversible_local_read | false | false |

## Invariants

- [x] Ambiguous / denied utterances never execute silently
- [x] High-impact commands require confirmation and a correction
- [x] Candidates carry palette parity fields
- [x] Disabled candidates carry a reason
- [x] Voice commands use the canonical command graph
- [x] Voice commits join grouped undo and audit
- [x] Every row offers a keyboard-first fallback
