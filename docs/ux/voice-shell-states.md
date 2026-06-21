# Voice shell states

Voice is an **explicit, privacy-bounded input mode** in Aureline. This doc is the
human-readable face of the *always-visible shell state* a user reads to answer
one question at a glance: **is Aureline in command mode, dictation mode, idle, or
blocked — and where is my speech being processed?**

It complements the two other voice lanes rather than duplicating them:

- the M3 voice preview ([`crate::voice`](../../crates/aureline-shell/src/voice/mod.rs))
  models the bounded preview/beta surface — transcript strips, disambiguation
  sheets, and per-command resolution proof; and
- the M5 [voice-mode and privacy truth](voice-mode-and-privacy-truth.md) matrix
  freezes the provider / retention / command-parity *qualification* for every
  **claimed** voice profile.

This lane does **not** mint a second interaction model. It reuses the same mode,
activation, mic-indicator, processing-locality, retention, and policy vocabulary
those lanes own and projects it into the persistent shell affordances: a mode
strip, a push-to-talk control, a mic-state pill, an inline provider/locality
disclosure, and a keyboard-first recovery.

## Canonical object

The canonical record is the
[`VoiceShellStatePacket`](../../crates/aureline-shell/src/voice_shell_state/mod.rs)
in `aureline-shell`. Each
[`VoiceShellStateRow`](../../crates/aureline-shell/src/voice_shell_state/mod.rs)
carries one claimed surface's current shell state:

- a [`VoiceModeStrip`] — command and dictation segments are **both always
  visible**, so a user reads the active mode directly instead of inferring it
  from the absence of the other;
- a [`PushToTalkControl`] — push-to-talk (or an equivalent explicit activation)
  is the default; any continuous/wake activation is a non-default path gated by
  an explicit opt-in, so capture can never become silently always-on;
- a [`VoiceMicIndicator`] (mic-state pill) — visible **whenever capture is
  active**, always carrying the active mode and the local-versus-hosted
  processing cue, so decorative mic chrome can never obscure mode or locality;
- a [`ProviderLocalityDisclosure`] — provider / local-engine identity, processing
  locality, and retention posture shown **inline**, not behind a deep settings
  dive; and
- a [`VoiceRecoveryAffordance`] — whenever voice is unavailable or
  policy-blocked, an **immediate keyboard-first recovery** with a typed reason,
  never a dead end.

The packet carries metadata only — typed class tokens, booleans, opaque ids, and
redaction-aware label refs. Raw audio bytes, raw transcript text, raw provider
payloads, private paths, and credentials never cross this boundary.

## Lifecycle vocabulary

Every row carries exactly one
[`VoiceShellLifecycleState`](../../crates/aureline-shell/src/voice_shell_state/mod.rs)
from a small, controlled vocabulary — never provider-specific prose:

| State | Meaning | Mic indicator |
| --- | --- | --- |
| `idle` | Microphone off; nothing captured. | Visible, idle |
| `listening` | Capture active; audio being received. | Visible, active |
| `processing` | Capture finished; utterance being recognized/resolved. | Visible, active |
| `needs_confirmation` | A resolved high-impact command awaits explicit confirm. | Visible, idle |
| `unavailable` | No microphone / offline / provider down; keyboard recovery offered. | Unavailable, degraded |
| `policy_blocked` | Voice blocked by policy or envelope; keyboard recovery offered. | Hidden, capture disabled |

## Seeded rows

The seed covers every lifecycle state across both local and disclosed-hosted
processing localities. It is the single mint-from-truth source for the checked-in
fixtures under `fixtures/voice/mode-and-mic-state/`.

| Row | Lifecycle | Active mode | Activation | Locality | Retention | Policy |
| --- | --- | --- | --- | --- | --- | --- |
| `voice-shell:command-overlay:local:idle` | idle | idle_microphone_off | push_to_talk_held | local_on_device | no_audio_retained_no_transcript_retained | user_controlled |
| `voice-shell:command-overlay:local:listening` | listening | command_mode_active | push_to_talk_held | local_on_device | ephemeral_audio_local_only_no_transcript_retained | user_controlled |
| `voice-shell:dictation-input:hosted:processing` | processing | dictation_mode_active | push_to_talk_toggle | hosted_remote_disclosed | transcript_retained_redacted_in_support_bundle | user_controlled |
| `voice-shell:command-overlay:local:needs-confirmation` | needs_confirmation | command_mode_active | push_to_talk_held | local_on_device | transcript_retained_local_only | user_controlled |
| `voice-shell:command-overlay:local:unavailable` | unavailable | idle_microphone_off | push_to_talk_held | processing_unavailable | retention_unavailable_in_envelope | user_controlled |
| `voice-shell:command-overlay:managed:policy-blocked` | policy_blocked | voice_mode_blocked_by_policy | push_to_talk_held | processing_unavailable | retention_blocked_by_policy | policy_blocked |

## Invariants

[`VoiceShellStatePacket::validate`](../../crates/aureline-shell/src/voice_shell_state/mod.rs)
refuses any packet that breaks these, and the invariant manifest on the packet
records each as a boolean:

- **Mode stays explicit** — command and dictation segments are both visible on
  every row.
- **Mic visible whenever capturing** — a `listening`/`processing` row must show
  the active mic indicator.
- **Push-to-talk or opt-in continuous default** — default activation is explicit,
  or a continuous/wake class backed by an explicit opt-in.
- **No hidden continuous listening** — background listening is off by default;
  only an explicit opt-in turns it on.
- **Provider/locality disclosed inline** — every claimed row discloses where
  speech is processed without a settings dive.
- **Blocked states offer keyboard-first recovery** — every `unavailable` /
  `policy_blocked` row carries an immediate keyboard fallback and a typed reason.
- **Capture always announced** — mode and lifecycle state are announced to
  assistive tech, and capture rows carry an accessibility label.

## Out of scope

This lane does not add speech shortcuts that bypass command routing or preview,
a speech-model picker, or an always-on assistant. High-impact spoken commands
still ride the same preview/approval/trust/audit path a keyboard or palette
invocation rides.

## Regenerating

The doc table and the fixtures are minted from the seed. Do not edit either by
hand; regenerate with:

```bash
cargo run -p aureline-shell --example dump_voice_shell_state -- write
```

The `on_disk_fixtures_match_seed_bit_for_bit` test fails if the checked-in
fixtures drift from the seed.
