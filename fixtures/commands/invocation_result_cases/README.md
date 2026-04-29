# Invocation result fixtures

Worked YAML fixtures for the command invocation result and
cross-surface parity contract in
[`/docs/commands/invocation_result_and_parity_contract.md`](../../../docs/commands/invocation_result_and_parity_contract.md).

| Fixture | Schema | Purpose |
| --- | --- | --- |
| [`palette_import_profile_succeeded.yaml`](./palette_import_profile_succeeded.yaml) | `result_packet.schema.json` | Palette invocation that succeeds, emits artifact / notification / activity / evidence refs, and links an available rollback handle. |
| [`ai_push_branch_approval_pending.yaml`](./ai_push_branch_approval_pending.yaml) | `result_packet.schema.json` | AI-initiated publish command that rendered preview, opened approval, and scheduled background work without applying yet. |
| [`cli_deprecated_alias_warning.yaml`](./cli_deprecated_alias_warning.yaml) | `result_packet.schema.json` | CLI invocation through a deprecated alias that resolves to the canonical command while preserving support-window and migration traceability. |
| [`companion_reset_denied.yaml`](./companion_reset_denied.yaml) | `result_packet.schema.json` | Mobile companion mutation attempt denied by trust / handoff rules with typed result semantics. |
| [`rollback_handle_import_profile.yaml`](./rollback_handle_import_profile.yaml) | `rollback_handle.schema.json` | Rollback handle referenced by the palette import result. |
| [`parity_expectation_git_push_branch.yaml`](./parity_expectation_git_push_branch.yaml) | `parity_expectation.schema.json` | Cross-surface parity expectation covering menu/button/context menu, palette/keybinding/leader, CLI/recipe/AI/voice, and browser/mobile companion rows. |

