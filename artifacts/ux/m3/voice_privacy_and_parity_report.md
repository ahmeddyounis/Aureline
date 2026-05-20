# Voice privacy and parity report (beta qualification)

Generated from the seeded voice page in
[`crate::voice`](../../../crates/aureline-shell/src/voice/mod.rs) and the
qualification packet in
[`crate::voice::conformance`](../../../crates/aureline-shell/src/voice/conformance/report.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- privacy-report > \
  artifacts/ux/m3/voice_privacy_and_parity_report.md
```

- Page id: `shell:voice_preview_beta:page:v1`
- Packet id: `voice-qualification-packet:beta:001`
- Rows: `5`
- Claimed beta/preview rows: `4` (kept: `4`, downgraded: `0`)
- Labs/unadvertised rows: `1`
- Corpus clean: `true`
- All claimed rows qualified: **true**
- Generated at: `2026-05-20T00:00:00Z`

## Privacy and availability

| Row | Processing | Retention | Background listening | Unavailable reason | Keyboard fallback | Redaction |
| --- | ---------- | --------- | -------------------- | ------------------ | ----------------- | --------- |
| `voice:row:command_mode_local_beta` | `local_on_device` | `no_audio_retained_no_transcript_retained` | `off_default` | `-` | true | `metadata_safe_default` |
| `voice:row:dictation_local_beta` | `local_on_device` | `ephemeral_audio_local_only_no_transcript_retained` | `off_default` | `-` | true | `metadata_safe_default` |
| `voice:row:hosted_command_preview` | `hosted_remote_disclosed` | `transcript_retained_provider_per_contract` | `off_default` | `-` | true | `metadata_safe_default` |
| `voice:row:labs_unadvertised_continuous` | `processing_unavailable` | `retention_unavailable_in_envelope` | `off_default` | `policy_locked_or_blocked` | true | `metadata_safe_default` |
| `voice:row:provider_unavailable_fallback` | `processing_unavailable` | `no_audio_retained_no_transcript_retained` | `off_default` | `no_microphone` | true | `metadata_safe_default` |

## Accessibility and lifecycle narration

Every claimed row is keyboard reachable and screen-reader narratable; capturing rows carry start/stop/cancel actions and an accessibility label so narration and focus return honestly when capture starts, ends, fails, or is cancelled.

| Row | Mic narration | Transcript narration | Stop action | Mute action | Cancel action |
| --- | ------------- | -------------------- | ----------- | ----------- | ------------- |
| `voice:row:command_mode_local_beta` | `a11y:voice:mic_active_command_mode_local` | `a11y:voice:transcript_strip` | `cmd:voice.stop_capture` | `cmd:voice.mute_microphone` | `cmd:voice.cancel_transcript` |
| `voice:row:dictation_local_beta` | `a11y:voice:mic_active_dictation_local` | `a11y:voice:transcript_strip` | `cmd:voice.stop_capture` | `cmd:voice.mute_microphone` | `cmd:voice.cancel_transcript` |
| `voice:row:hosted_command_preview` | `a11y:voice:mic_active_command_mode_hosted` | `a11y:voice:transcript_strip` | `cmd:voice.stop_capture` | `cmd:voice.mute_microphone` | `cmd:voice.cancel_transcript` |
| `voice:row:provider_unavailable_fallback` | `a11y:voice:mic_unavailable_no_microphone` | `-` | `cmd:voice.stop_capture` | `cmd:voice.mute_microphone` | `-` |

## Qualification packet

The packet keeps a claimed row Preview/Beta only when the conformance corpus is clean, the row carries no blocking finding, and its privacy/parity proof is fresh and complete. Otherwise the row is forced back to Labs before stable-facing language can overclaim it.

| Row | Posture | Verdict | Downgrade reasons |
| --- | ------- | ------- | ----------------- |
| `voice:row:command_mode_local_beta` | `claimed_beta` | `keep_claimed` | - |
| `voice:row:dictation_local_beta` | `claimed_beta` | `keep_claimed` | - |
| `voice:row:hosted_command_preview` | `claimed_preview` | `keep_claimed` | - |
| `voice:row:labs_unadvertised_continuous` | `labs_unadvertised` | `remains_labs` | - |
| `voice:row:provider_unavailable_fallback` | `claimed_beta` | `keep_claimed` | - |

## Verification

```sh
cargo test -p aureline-shell --test voice_conformance_corpus
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- run
```
