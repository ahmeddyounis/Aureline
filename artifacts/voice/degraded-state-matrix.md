# Voice degraded-state and recovery matrix

Generated from the `voice_degraded_state` seed. Do not edit by hand; regenerate with `cargo run -p aureline-shell --example dump_voice_degraded_state -- write`.

- Packet: `shell:voice_degraded_state:packet:v1`
- Builds on: `shell:voice_shell_state:v1`
- Help / recovery doc: `docs/help/voice-fallback-and-recovery.md`
- Fixtures: `fixtures/voice/fallback-and-noisy-env`

| Cause | Controlled state | Recovery posture | Severity | Recovery actions |
| --- | --- | --- | --- | --- |
| `missing_microphone_hardware` | unavailable | fell_back_to_keyboard_first | warning | open_microphone_settings, keyboard_fallback |
| `noisy_environment` | needs_confirmation | held_for_confirmation | informational | confirm_held_result, retry_when_condition_clears, keyboard_fallback |
| `provider_offline` | unavailable | offered_on_device_engine_fallback | warning | switch_to_on_device_engine, retry_when_condition_clears, keyboard_fallback |
| `language_pack_missing` | unavailable | held_until_condition_clears | warning | install_or_switch_language_pack, keyboard_fallback |
| `policy_blocked` | policy_blocked | fell_back_to_keyboard_first | blocked | open_policy_details, keyboard_fallback |

## Invariants

- [x] Every cause shows a durable banner naming the cause
- [x] Every cause offers a concrete recovery action
- [x] Keyboard fallback preserves focus and work
- [x] Every state is controlled (no silent / oscillating failure)
- [x] No generic failure copy
- [x] State changes are narration-safe
- [x] Non-voice recovery affordances preserved
- [x] All five failure classes covered
