# Voice / dictation conformance corpus

This corpus is the regression-gated proof lane for the bounded voice/dictation
beta surface. Every drill is one canonical `VoicePreviewRow` owned by
[`aureline-shell::voice`](../../../../crates/aureline-shell/src/voice/mod.rs). The
runner in
[`crates/aureline-shell/src/voice/conformance/`](../../../../crates/aureline-shell/src/voice/conformance)
parses each fixture into the real record type, replays it through the canonical
validation `build_voice_preview_row`, and compares the result against the
per-drill truth pinned in `manifest.json`. The ruleset is never re-implemented in
the corpus.

## What it proves

The corpus guards the voice qualification exit-gate:

> If Aureline claims a beta or preview voice row, it has current proof that
> microphone capture, transcript handling, mode separation, command-equivalence,
> high-risk confirmation, policy blocks, and keyboard fallback behave honestly
> enough to qualify the surface rather than merely label it.

- **Positive drills** MUST validate cleanly (zero blocking findings), carry no
  raw transcript / audio / URL leak, and match every pinned `expected_*` token
  (claim posture, voice mode, default activation, processing locality, retention
  mode, background-listening state, unavailable reason, keyboard fallback, and
  the high-impact command id a risky resolution must bind).
- **Negative drills** MUST be rejected. A `validator` drill MUST raise a blocking
  finding whose class token contains `expected_violation_class`; a
  `redaction_scan` drill MUST trip the raw-transcript/audio leak scan.

## Coverage

Positive drills cover command mode and dictation mode on a local speech engine,
a hosted provider with disclosed handoff and approval-gated publish,
policy-blocked Labs capture, offline / provider-unavailable / no-microphone /
noisy-environment unavailable states (each with a keyboard fallback), transcript
correction, and high-risk command confirmation that keeps its preview.

Negative drills pin the conditions the lane MUST catch: hidden always-listening
behavior (non-explicit activation and background listening without a wake-phrase
opt-in), missing provider/privacy disclosure, a high-impact resolution that skips
its required preview, a weakened no-bypass guard, a broken keyboard fallback on an
unavailable row, a hidden mic indicator during active capture, a claimed row that
is not keyboard reachable or screen-reader narratable, a hidden command/dictation
mode split, a resolution that binds no canonical command id, a resolution that
resolves an uncanonical verb, a disabled resolution missing its typed reason, a
Labs row that starts advertising broad support, and transcript leakage (a raw URL
in a transcript label).

## Regenerate

The whole corpus is minted from a deterministic seed so the fixtures never drift:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- write-corpus
```

The conformance test additionally asserts the on-disk corpus is bit-for-bit equal
to the seed, so a hand edit that diverges fails CI.

## Validate

```sh
cargo test -p aureline-shell --test voice_conformance_corpus
cargo run -q -p aureline-shell --bin aureline_shell_voice_conformance -- run
```
