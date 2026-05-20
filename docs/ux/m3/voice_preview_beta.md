# Bounded voice preview and privacy (beta) — consumption guide

This guide describes the **bounded voice-command and dictation preview
surface** Aureline ships as a beta/preview path. It is intentionally
narrow: only the rows Aureline explicitly claims expose live voice, and
those rows ride the frozen
[voice / dictation / speech-privacy contract](../voice_and_dictation_contract.md)
and the canonical command graph in `schemas/commands/`. Voice is **not**
a hidden general assistant and **not** a second command system.

## What the surface guarantees

On every **claimed** Preview/Beta voice row:

- **Command mode and dictation mode are explicit.** Each row carries
  `command_mode_explicit` and `dictation_mode_explicit`, and a mic-state
  pill that names the active `voice_mode_class`. Both modes are keyboard
  reachable and screen-reader narratable.
- **Capture is never implicitly always-on.** The default activation is an
  explicit push-to-talk / manual activation, and `background_listening_state`
  is `off_default` unless the user explicitly opts in to a wake phrase. A
  persistent mic indicator (`persistent_indicator_visible_capture_active`)
  is shown whenever capture is active, alongside a local-or-hosted
  processing cue and mute/stop actions.
- **The transcript stays correctable before it commits.** A transcript
  strip exposes a confidence cue plus edit / correct / confirm / cancel
  actions; a command-disambiguation sheet shows candidate commands with
  confidence cues and a preview state for risky candidates.
- **Provider and privacy state are disclosed.** A provider/privacy row
  names the provider or local engine, the retention mode, the
  background-listening state, any policy lock/block note, the typed
  unavailable reason, and the keyboard fallback. Unavailable states
  (offline, provider unavailable, no microphone, noisy environment, policy
  lock/block, unverified local pack, restricted trust) always raise an
  unavailable banner that points at a keyboard fallback.
- **High-impact actions cannot bypass the existing rules.** Every spoken
  command resolves through a `shell_voice_command_resolution_record` that
  carries the same canonical `command_id`, capability scope, lifecycle
  label, disabled-reason vocabulary, preview/approval posture,
  result-packet schema (`schemas/commands/command_result_packet.schema.json`),
  and strict no-bypass guards a keyboard or command-palette invocation
  carries. A high-impact resolution (`recoverable_durable_mutation`,
  `destructive_bulk_mutation`, `irreversible_publish`) keeps
  `preview_required` true and keeps every no-bypass guard true.

On every **unclaimed** row, voice stays `labs_unadvertised`: capture is
suppressed, no spoken commands resolve, and stable docs / help / About do
not imply broad voice support.

## Records and schemas

| Record | Schema |
| ------ | ------ |
| `shell_voice_preview_beta_page_record`, `shell_voice_preview_beta_row_record`, mic-state pill, transcript strip, command-disambiguation sheet, provider/privacy row, unavailable banner, support export | [`/schemas/ux/voice_session_state.schema.json`](../../../schemas/ux/voice_session_state.schema.json) |
| `shell_voice_command_resolution_record` (command-graph parity) | [`/schemas/ux/voice_command_resolution.schema.json`](../../../schemas/ux/voice_command_resolution.schema.json) |

The bounded surface consumes — and never re-mints — the upstream voice
vocabulary frozen in
[`/docs/ux/voice_and_dictation_contract.md`](../voice_and_dictation_contract.md)
and the command vocabulary frozen in
[`/schemas/commands/command_descriptor.schema.json`](../../../schemas/commands/command_descriptor.schema.json)
and
[`/schemas/commands/command_result_packet.schema.json`](../../../schemas/commands/command_result_packet.schema.json).

## Mint-from-truth and fixtures

The headless inspector is the only mint-from-truth path:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- page > \
  fixtures/ux/m3/voice_preview_and_privacy/page.json
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- support-export > \
  fixtures/ux/m3/voice_preview_and_privacy/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- compact > \
  fixtures/ux/m3/voice_preview_and_privacy/compact.txt
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- report-md > \
  artifacts/ux/m3/voice_preview_beta.md
```

The checked-in fixtures live under
[`/fixtures/ux/m3/voice_preview_and_privacy/`](../../../fixtures/ux/m3/voice_preview_and_privacy/):

- [`page.json`](../../../fixtures/ux/m3/voice_preview_and_privacy/page.json)
- [`support_export.json`](../../../fixtures/ux/m3/voice_preview_and_privacy/support_export.json)
- `compact.txt`

The published markdown report is regenerated to
[`/artifacts/ux/m3/voice_preview_beta.md`](../../../artifacts/ux/m3/voice_preview_beta.md).

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_preview -- validate
cargo test -p aureline-shell --test voice_preview_beta_fixtures
python3 tools/ci/m3/voice_preview_check.py
```

The CI gate `tools/ci/m3/voice_preview_check.py` fails release if the
checked-in page diverges from the seed, if any blocking finding remains,
if a claimed row drops a mode / mic pill / keyboard reach / screen-reader
narration, if a high-impact resolution bypasses preview or weakens a
no-bypass guard, if an unavailable row drops its keyboard fallback, or if
an unclaimed row stops being `labs_unadvertised`.
