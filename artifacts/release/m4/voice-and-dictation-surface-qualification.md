# Voice and Dictation Surface Qualification

Canonical packet:
`artifacts/release/m4/voice-and-dictation-surface-qualification.json`

Schema:
`schemas/release/voice-and-dictation-surface-qualification.schema.json`

Voice and dictation surfaces do not inherit Stable from command palette, AI, or
general accessibility packets. The JSON packet is the source of truth for the
label rendered by product, docs, Help/About, privacy diagnostics, and support
exports.

## Voice command overlay

Stable coverage is limited to local push-to-talk command mode. Spoken commands
resolve to canonical command IDs and keep disabled reasons, preview/apply/revert,
approval, undo grouping, audit lineage, and support lineage identical to keyboard
and palette paths.

## Dictation input

Stable dictation inserts text only while Dictation mode is visibly active.
Correction buffers are bounded, delete/export actions are inspectable, and raw
transcripts are excluded from support export by default.

## Transcript correction

The transcript strip is a correction and confirmation surface, not a durable
chat log. It carries confidence/correction state, mode truth, delete/export
review, and redaction state before content leaves the default local scope.

## Provider privacy row

Provider rows name the processing class: `local`, `trusted_enterprise`,
`third_party_provider`, or `unavailable`. Stable enterprise routing requires a
persistent indicator, retention posture, policy/revoke/open-settings actions,
and support-export metadata that omits raw transcripts by default.

## Unavailable fallback

No-microphone, policy-blocked, offline, provider-unavailable, and noisy
environment states must name the reason and expose keyboard fallback. Dead
controls or silent mode changes are release blockers.

## High-impact review

Spoken delete, apply, push, run, remote-control, and similar high-impact actions
require transcript confirmation or review before run. Voice may be stricter than
keyboard, but never looser.

## Support export projection

Support exports may include mode, processing class, fallback state, command ID,
approval posture, undo group, packet ID, and evidence refs. They must not include
raw transcripts by default.

## Verification

```sh
cargo test -p aureline-release --test voice_and_dictation_surface_qualification
```
