# Bounded voice preview and privacy (beta)

Generated from the seeded voice-preview projection in
[`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- report-md > \
  artifacts/ux/m3/voice_preview_beta.md
```

- Page id: `shell:voice_preview_beta:page:v1`
- Contract: `docs/ux/voice_and_dictation_contract.md`
- Session-state schema: `schemas/ux/voice_session_state.schema.json`
- Command-resolution schema: `schemas/ux/voice_command_resolution.schema.json`
- Rows: `5`
- Claimed beta/preview rows: `4`
- Labs/unadvertised rows: `1`
- Spoken-command resolutions: `5` (high-impact: `2`)
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-05-20T00:00:00Z`

## Rows

| Row | Posture | Command mode | Dictation mode | Default activation | Background listening | Capturing | Resolutions |
| --- | ------- | ------------ | -------------- | ------------------ | -------------------- | --------- | ----------: |
| `voice:row:command_mode_local_beta` | `claimed_beta` | true | true | `push_to_talk_held` | `off_default` | true | 2 |
| `voice:row:dictation_local_beta` | `claimed_beta` | true | true | `push_to_talk_held` | `off_default` | true | 1 |
| `voice:row:hosted_command_preview` | `claimed_preview` | true | true | `manual_command_activation` | `off_default` | true | 1 |
| `voice:row:labs_unadvertised_continuous` | `labs_unadvertised` | false | false | `activation_unavailable_in_envelope` | `off_default` | false | 0 |
| `voice:row:provider_unavailable_fallback` | `claimed_beta` | true | true | `push_to_talk_held` | `off_default` | false | 1 |

## Command-graph parity

Every spoken command resolves through the same canonical command id, capability scope, lifecycle label, disabled reason, preview/approval posture, and result-packet schema as the keyboard and command-palette lanes.

| Resolution | Command id | Scope | Preview required | Approval required | Enablement | Disabled reason |
| ---------- | ---------- | ----- | ---------------- | ----------------- | ---------- | --------------- |
| `voice:resolution:rename_symbol_across_project` | `cmd:edit.rename_symbol_across_project` | `recoverable_durable_mutation` | true | false | `enabled` | `-` |
| `voice:resolution:go_to_definition` | `cmd:navigation.go_to_definition` | `reversible_local_read` | false | false | `enabled` | `-` |
| `voice:resolution:insert_dictated_text` | `cmd:editor.insert_dictated_text` | `reversible_local_mutation` | false | false | `enabled` | `-` |
| `voice:resolution:push_current_branch` | `cmd:git.push_current_branch` | `irreversible_publish` | true | true | `enabled` | `-` |
| `voice:resolution:blocked_no_microphone` | `cmd:edit.rename_symbol_across_project` | `inert_metadata_only` | false | false | `disabled_with_reason` | `execution_context_unavailable` |

## Privacy and availability

| Row | Processing | Retention | Background listening | Unavailable reason | Keyboard fallback |
| --- | ---------- | --------- | -------------------- | ------------------ | ----------------- |
| `voice:row:command_mode_local_beta` | `local_on_device` | `no_audio_retained_no_transcript_retained` | `off_default` | `-` | true |
| `voice:row:dictation_local_beta` | `local_on_device` | `ephemeral_audio_local_only_no_transcript_retained` | `off_default` | `-` | true |
| `voice:row:hosted_command_preview` | `hosted_remote_disclosed` | `transcript_retained_provider_per_contract` | `off_default` | `-` | true |
| `voice:row:labs_unadvertised_continuous` | `processing_unavailable` | `retention_unavailable_in_envelope` | `off_default` | `policy_locked_or_blocked` | true |
| `voice:row:provider_unavailable_fallback` | `processing_unavailable` | `no_audio_retained_no_transcript_retained` | `off_default` | `no_microphone` | true |

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- validate
cargo test -p aureline-shell --test voice_preview_beta_fixtures
python3 tools/ci/m3/voice_preview_check.py
```
