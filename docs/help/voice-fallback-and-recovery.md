# Voice fallback and recovery

Voice is an **optional, privacy-bounded input mode** in Aureline. When the
conditions a voice surface depends on are not met, that surface enters a
**named, recoverable degraded state** instead of collapsing into a generic
error, failing silently, or flickering between working and broken. This doc is
the human-readable face of that contract.

It builds on the always-visible
[voice shell states](../ux/voice-shell-states.md) lane: a degraded flow always
lands on one of that lane's controlled lifecycle states — `unavailable`,
`policy_blocked`, or `needs_confirmation` — and reuses the same mode, locality,
and policy vocabulary. It does **not** add a second interaction model.

## Canonical object

The canonical record is the
[`VoiceDegradedStatePacket`](../../crates/aureline-shell/src/voice_degraded_state/mod.rs)
in `aureline-shell`. Each
[`VoiceDegradedFlow`](../../crates/aureline-shell/src/voice_degraded_state/mod.rs)
covers exactly one failure class and carries:

- a durable [`DegradedBanner`] — names the **specific** failed capability, the
  cause, and the consequence; never a generic "Voice unavailable" line;
- one or more concrete inline [`RecoveryAction`]s — each bound to a canonical
  command id, so every failure class has a real next step;
- a [`KeyboardFirstFallback`] — returns the user to the keyboard / command
  palette **without losing focus or in-progress work**; and
- a narration-safe [`DegradedNarration`] — announces the cause and the recovery
  exactly once per controlled transition (no oscillation, no chatter).

The packet carries metadata only — typed class tokens, booleans, opaque ids, and
redaction-aware label refs. Raw audio bytes, raw transcript text, raw provider
payloads, private paths, and credentials never cross this boundary.

## Failure classes

Each major failure class has its own flow, controlled state, and recovery:

| Cause | What happened | Controlled state | Primary recovery |
| --- | --- | --- | --- |
| `missing_microphone_hardware` | No microphone is present or permitted. | `unavailable` | Open input settings; keep using the keyboard / command palette. |
| `noisy_environment` | The room is too noisy for reliable recognition. | `needs_confirmation` | Every result is held for confirmation; confirm, retry quieter, or type. |
| `provider_offline` | The hosted speech provider is unreachable. | `unavailable` | Switch to the on-device engine, retry, or use the keyboard. |
| `language_pack_missing` | The requested language / locale pack is missing. | `unavailable` | Install or switch the language pack; keyboard stays available. |
| `policy_blocked` | Managed policy or the envelope disables voice. | `policy_blocked` | Open policy details; the keyboard / command palette still works. |

Where a canonical cross-surface
[`VoiceUnavailableReason`](../../crates/aureline-shell/src/voice/mod.rs) token
exists, the flow records it too, so diagnostics and support export line up with
the rest of the voice truth set. `language_pack_missing` has no canonical reason
token, so the lane-local cause is authoritative.

## Invariants

[`VoiceDegradedStatePacket::validate`](../../crates/aureline-shell/src/voice_degraded_state/mod.rs)
refuses any flow that breaks these, and the invariant manifest on the packet
records each as a boolean:

- **Durable, specific banner** — no flow hides its cause behind generic copy or a
  transient toast.
- **Concrete recovery** — every flow offers at least one inline recovery action
  bound to a command id, plus a keyboard-first fallback action.
- **Keyboard fallback preserves continuity** — falling back keeps focus and
  uncommitted work, and returns focus to a named target.
- **Controlled state** — every flow lands on `unavailable`, `policy_blocked`, or
  `needs_confirmation`; never a silent or oscillating failure.
- **Narration-safe** — the state change is announced once, naming both the cause
  and the recovery.
- **Non-voice recovery preserved** — a degraded voice state never suppresses or
  overwrites existing non-voice recovery affordances.
- **All failure classes covered** — every major class above has a flow.

## Out of scope

This lane only governs truthful degraded-state handling and recovery; it does not
expand normal-path voice breadth, and it never lets a degraded voice state hide,
suppress, or overwrite a non-voice recovery path that already exists on the same
surface.

## Regenerating

The doc tables, the [degraded-state matrix](../../artifacts/voice/degraded-state-matrix.md),
and the fixtures under `fixtures/voice/fallback-and-noisy-env/` are minted from
the seed. Do not edit them by hand; regenerate with:

```bash
cargo run -p aureline-shell --example dump_voice_degraded_state -- write
cargo run -p aureline-shell --example dump_voice_degraded_state -- summary > artifacts/voice/degraded-state-matrix.md
```

The `on_disk_fixtures_match_seed_bit_for_bit` test fails if the checked-in
fixtures drift from the seed.
