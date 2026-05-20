# Voice command-equivalence audit (beta)

Generated from the seeded voice page in
[`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- equivalence-audit > \
  artifacts/ux/m3/voice_command_equivalence_audit.md
```

Every claimed spoken command resolves through the same canonical command id as the keyboard and command-palette lanes, and carries the same capability scope, lifecycle label, disabled reason, preview/approval posture, and result-packet schema. The `keyboard_equivalent` column proves the voice resolution and the keyboard invocation reach the same command id.

| Resolution | Command id | Keyboard equivalent | Scope | Lifecycle | Preview | Approval | Enablement | Disabled reason | Result schema |
| ---------- | ---------- | ------------------- | ----- | --------- | ------- | -------- | ---------- | --------------- | ------------- |
| `voice:resolution:rename_symbol_across_project` | `cmd:edit.rename_symbol_across_project` | `cmd:edit.rename_symbol_across_project` | `recoverable_durable_mutation` | `beta` | true | false | `enabled` | `-` | `schemas/commands/command_result_packet.schema.json` |
| `voice:resolution:go_to_definition` | `cmd:navigation.go_to_definition` | `cmd:navigation.go_to_definition` | `reversible_local_read` | `stable` | false | false | `enabled` | `-` | `schemas/commands/command_result_packet.schema.json` |
| `voice:resolution:insert_dictated_text` | `cmd:editor.insert_dictated_text` | `cmd:editor.insert_dictated_text` | `reversible_local_mutation` | `beta` | false | false | `enabled` | `-` | `schemas/commands/command_result_packet.schema.json` |
| `voice:resolution:push_current_branch` | `cmd:git.push_current_branch` | `cmd:git.push_current_branch` | `irreversible_publish` | `preview` | true | true | `enabled` | `-` | `schemas/commands/command_result_packet.schema.json` |
| `voice:resolution:blocked_no_microphone` | `cmd:edit.rename_symbol_across_project` | `cmd:edit.rename_symbol_across_project` | `inert_metadata_only` | `beta` | false | false | `disabled_with_reason` | `execution_context_unavailable` | `schemas/commands/command_result_packet.schema.json` |

## Cross-surface equivalence

- **Voice ↔ keyboard ↔ palette:** the command id and `keyboard_equivalent` match, and high-impact scopes keep `preview_required` true with strict no-bypass guards.
- **CLI / help metadata:** each resolution carries a `docs_help_anchor_ref` so the same command is discoverable from help and CLI metadata.
- **Support exports:** the support-export wrapper quotes the same command ids while excluding raw audio/transcript bytes by default.

## Verification

```sh
cargo test -p aureline-shell --test voice_conformance_corpus
```
