# Voice preview/beta qualification audit

This audit is the qualification packet a release reviewer reads before any
Aureline row may keep voice in **Preview/Beta**. It sits on top of the bounded
voice surface described in
[`voice_preview_beta.md`](voice_preview_beta.md) and the frozen
[voice / dictation / speech-privacy contract](../voice_and_dictation_contract.md),
and turns the claimed voice rows into regression-gated qualified rows: speech
input cannot harden into product truth without current evidence for privacy,
confirmation, accessibility, and command-parity behavior.

## How a row qualifies

A claimed voice row keeps its Preview/Beta posture only when **all** of the
following hold; otherwise the qualification packet forces it back to
Labs/unadvertised before stable-facing language can overclaim it:

1. The voice/dictation conformance corpus is clean (every drill passes).
2. The row carries no blocking finding from the canonical validator
   (`build_voice_preview_row`).
3. The row's privacy/parity proof is **fresh** and **complete**.

The packet is rendered to
[`/artifacts/ux/m3/voice_privacy_and_parity_report.md`](../../../artifacts/ux/m3/voice_privacy_and_parity_report.md),
which lists each row's verdict (`keep_claimed`, `downgrade_to_labs`, or
`remains_labs`) and the downgrade reasons. A stale or incomplete row shows
`proof_stale` / `proof_incomplete`; an unclean corpus downgrades every claimed
row with `conformance_corpus_not_clean`.

## What the corpus proves

The corpus lives at
[`/fixtures/ux/m3/voice_conformance_corpus/`](../../../fixtures/ux/m3/voice_conformance_corpus/)
and is replayed by the harness in
[`crates/aureline-shell/src/voice/conformance/`](../../../crates/aureline-shell/src/voice/conformance/).
Every drill is one canonical `VoicePreviewRow`; the runner never re-implements
the ruleset.

- **Speech privacy.** Positive drills pin processing locality (local engine vs
  disclosed hosted provider), retention mode, and an `off_default`
  background-listening state. The corpus fails hidden always-listening behavior
  (non-explicit activation, or background listening without a wake-phrase
  opt-in) and missing provider/privacy disclosure.
- **Command equivalence.** Every claimed spoken command resolves through the
  same canonical command id, capability scope, lifecycle label, disabled reason,
  preview/approval posture, and result-packet schema as the keyboard,
  command-palette, CLI/help, and support-export lanes. The cross-surface table
  is published to
  [`/artifacts/ux/m3/voice_command_equivalence_audit.md`](../../../artifacts/ux/m3/voice_command_equivalence_audit.md).
- **High-risk confirmation.** A high-impact resolution
  (`recoverable_durable_mutation`, `destructive_bulk_mutation`,
  `irreversible_publish`) keeps `preview_required` true and keeps strict
  no-bypass guards; the corpus fails any resolution that skips the preview or
  weakens a guard.
- **Keyboard fallback and accessibility.** Offline, provider-unavailable,
  no-microphone, and noisy-environment states always offer a keyboard fallback;
  capturing rows narrate start/stop/mute/cancel actions with accessibility
  labels so focus returns honestly when capture starts, ends, fails, or is
  cancelled. The corpus fails a broken keyboard fallback or a hidden mic
  indicator during active capture.
- **Transcript privacy in exports.** Positive fixtures and the support-export
  wrapper are scanned for raw transcript/audio/URL leaks; the corpus keeps a
  negative drill that proves a raw URL in a transcript label is rejected.
- **Labs suppression.** Unclaimed rows stay `labs_unadvertised`; the corpus
  fails a Labs row that starts advertising broad support.

## Records and schemas

The corpus reuses — and never re-mints — the boundary schemas frozen for the
bounded surface:

- [`/schemas/ux/voice_session_state.schema.json`](../../../schemas/ux/voice_session_state.schema.json)
- [`/schemas/ux/voice_command_resolution.schema.json`](../../../schemas/ux/voice_command_resolution.schema.json)
- [`/docs/ux/voice_and_dictation_contract.md`](../voice_and_dictation_contract.md)

## Regenerate and verify

```sh
# Regenerate the corpus fixtures, the qualification report, and the audit table.
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- write-corpus
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- privacy-report > \
  artifacts/ux/m3/voice_privacy_and_parity_report.md
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- equivalence-audit > \
  artifacts/ux/m3/voice_command_equivalence_audit.md

# Run the regression gate.
cargo test -p aureline-shell --test voice_conformance_corpus
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- run
```

The conformance test `voice_conformance_corpus` fails release if the on-disk
corpus diverges from the seed, if any drill is silently accepted or fails for the
wrong reason, if the corpus stops covering a required case, if voice/keyboard
command parity drifts, if the support export leaks raw bytes, if the
qualification packet stops downgrading stale/incomplete rows, or if these
artifacts drift from the seeded rendering.
