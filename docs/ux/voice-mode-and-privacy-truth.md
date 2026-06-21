# Voice-mode and privacy truth

Voice is an **explicit, privacy-bounded input mode** in Aureline. Command mode
and dictation mode are always separate and visible; push-to-talk (or an
equivalent explicit activation) is the default; provider locality, transport, and
transcript retention are inspectable before capture; dictated edits ride the same
edit model and grouped undo/history as keyboard input; and no voice path widens
authority, bypasses preview/confirmation, or creates hidden transcript/audio
retention.

This doc is the human-readable face of the canonical object model. The machine
truth is the qualification matrix that binds every **claimed** voice profile to a
versioned provider descriptor, a voice-session state, a bounded
transcript-retention posture, and full command parity, plus a claimed and
effective qualification grade.

## What this lane adds

The M3 voice preview and the M4 voice/dictation surface qualification already
model the per-surface session, mode-state, mic indicators, and command
resolution. This lane freezes the remaining implicit promise — that a *claimed*
voice profile rests on a **disclosed provider**, a **bounded transcript-retention
posture**, and **complete command parity** — into one verification-bound matrix
that command, accessibility, diagnostics, support-export, and release surfaces
read instead of cloning voice state text by hand.

It does **not** broaden voice into speech-model selection UX, a general
always-on assistant, or a speculative multimodal conversation layer. Claimed
voice profiles stay clearly separate from broader future voice ambitions.

## Canonical objects

The canonical record is the
[`M5VoiceQualificationMatrixPacket`](../../crates/aureline-shell/src/freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix/mod.rs)
in `aureline-shell`. Each
[`VoiceClaimedProfileRow`](../../crates/aureline-shell/src/freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix/mod.rs)
embeds two versioned, independently-serializable objects:

- a [`VoiceProviderDescriptor`](../../schemas/voice/voice-provider-descriptor.schema.json)
  — provider class, processing locality, transport class, capability disclosure
  (with opt-in guards), and a bounded transcript-retention posture;
- a [`VoiceSessionState`](../../schemas/voice/voice-session.schema.json)
  — mode, activation, mic-indicator class, processing locality, bound provider
  id, background-listening state, command-preview posture, transcript-correction
  posture, and policy state.

## Boundary

- Matrix schema: [`schemas/voice/m5-voice-qualification-matrix.schema.json`](../../schemas/voice/m5-voice-qualification-matrix.schema.json)
- Provider descriptor schema: [`schemas/voice/voice-provider-descriptor.schema.json`](../../schemas/voice/voice-provider-descriptor.schema.json)
- Voice-session schema: [`schemas/voice/voice-session.schema.json`](../../schemas/voice/voice-session.schema.json)
- Support export: [`artifacts/voice/m5-voice-qualification-matrix/support_export.json`](../../artifacts/voice/m5-voice-qualification-matrix/support_export.json)
- Markdown summary: [`artifacts/voice/m5-voice-qualification-matrix.md`](../../artifacts/voice/m5-voice-qualification-matrix.md)
- Upstream cross-surface contract: [`docs/ux/voice_and_dictation_contract.md`](./voice_and_dictation_contract.md)
- Upstream M4 surface qualification: [`artifacts/release/m4/voice-and-dictation-surface-qualification.json`](../../artifacts/release/m4/voice-and-dictation-surface-qualification.json)

Raw audio bytes, raw transcript text, raw provider payloads, private paths, and
credentials never cross this boundary; the packet carries only typed class
tokens, booleans, opaque ids, fingerprint digests, and redaction-aware reviewable
labels.

## Provider truth

Each claimed profile names exactly one provider descriptor. The descriptor pins:

| Axis | Field | Honest states | Disclosure rule |
| --- | --- | --- | --- |
| Provider class | `provider_class` | `on_device_local`, `approved_remote_disclosed`, `enterprise_relay_managed`, `mocked_test_provider`, `provider_disabled` | a disabled provider keeps the keyboard path |
| Processing locality | `processing_locality` | `local_on_device`, `hosted_remote_disclosed`, `processing_unavailable` | hosted processing is disclosed before capture |
| Transport | `transport_class` | `local_in_process_only`, `policy_bounded_disclosed_endpoint`, `explicit_opt_in_disclosed_endpoint`, `transport_blocked` | a hosted provider must declare a disclosed remote-handoff transport and be `audit_capable` |
| Background listening | `capability_disclosure.background_listening_default_off` | must be `true` | continuous listening and wake-word always require an explicit opt-in |

## Transcript-retention truth

The provider's `retention_posture` pins the retention mode, audio-retention
class, and transcript-export posture:

| Field | States |
| --- | --- |
| `retention_mode` | `no_audio_no_transcript_retained`, `ephemeral_audio_local_only_no_transcript_retained`, `transcript_retained_local_only`, `transcript_retained_redacted_in_support_bundle`, `transcript_retained_provider_per_contract`, `retention_blocked_by_policy`, `retention_unavailable_in_envelope` |
| `audio_retention` | `no_audio_retained`, `ephemeral_audio_local_only`, `bounded_audio_local_window`, `audio_retained_provider_per_contract`, `audio_capture_blocked` |
| `transcript_export` | `no_transcript_export`, `explicit_user_export_redacted`, `metadata_only_support_export`, `provider_contract_retained`, `export_blocked_by_policy` |

Invariants (absolute, never allowed even on a downgraded or Labs row):

- `raw_transcripts_excluded_by_default` is always `true` — raw transcripts never
  enter support bundles or telemetry by default;
- a support-bundle retention mode carries `redaction_before_support_export`;
- provider-per-contract retention is a **deliberate narrowing**, not a
  generally-claimed posture: a `qualified_claimed_profile` may not carry it, while
  a `qualified_narrowed_profile` (enterprise/preview) may.

## Command-parity truth

A claimed profile reuses the command/undo/policy parity of every other input.
Each row's `command_parity` block must be complete to hold a claim:

`stable_command_ids`, `disabled_with_reason`, `preview_apply_revert`,
`approval_requirements`, `undo_grouping`, `audit_support_lineage`,
`high_impact_review`, and `keyboard_fallback_parity`.

High-impact spoken actions (delete, apply, push, run, remote-control) ride the
same preview/approval/audit path a keyboard invocation rides and require
transcript confirmation; voice never widens authority or mints commands outside
the registry.

## Qualification grade and honest downgrade

Each row carries a claimed and an effective
[`VoiceQualificationGrade`](../../crates/aureline-shell/src/freeze_the_m5_voice_mode_provider_transcript_retention_and_command_parity_matrix/mod.rs):
`qualified_claimed_profile` > `qualified_narrowed_profile` >
`labs_unadvertised_profile` > `qualification_withdrawn` > `not_applicable`.

A claimed row auto-downgrades to an effective grade **strictly below** its claim,
with a recorded `downgrade_trigger` and a precise label, whenever it:

- conflates or blocks its command-vs-dictation mode (`mode_separation_unverified`);
- defaults to continuous/wake activation without an opt-in (`push_to_talk_default_missing`);
- leaves hosted/enterprise processing or retention undisclosed (`provider_locality_undisclosed`);
- exceeds bounded retention for a general claim (`transcript_retention_unbounded`);
- ships incomplete command parity (`command_parity_incomplete`);
- loses its keyboard fallback (`keyboard_fallback_missing`);
- has its provider go unavailable (`provider_unavailable_downgraded`);
- carries stale, missing, or imported-on-local proof (`stale_verification_proof`,
  `imported_proof_on_local_profile`).

The downgraded label is always a precise truth, never a generic non-answer
("error", "unavailable", "failed").

## Claimed scope vs future ambitions

Voice is **not** a broad stable surface. The default state for an unqualified
voice surface is Labs/unadvertised. The matrix keeps at least one
`labs_unadvertised` profile explicitly out of public scope, so "available in the
build" never reads as "part of the stable promise". A profile earns public
language only through the evidence its row carries — a reopenable proof keyed by a
non-display fingerprint, fresh within the verification-freshness SLO.

## Consumers

Product, command/help, accessibility, diagnostics, support-export, and
release-control surfaces ingest this one packet rather than cloning voice state
text per feature. Downgraded profiles are labeled below their claim in every
surface.
