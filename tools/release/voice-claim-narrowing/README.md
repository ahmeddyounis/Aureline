# Voice Claim Narrowing Tool

`tools/release/voice_claim_narrowing.py` certifies the claimed M5 voice profiles
and auto-narrows voice-bearing public claims when mode-separation,
transcript-boundary, privacy, or parity evidence is stale, failing, or limited to
a narrower platform/provider matrix.

The canonical truth is the voice qualification matrix support export
(`artifacts/voice/m5-voice-qualification-matrix/support_export.json`). This tool
ingests it, **independently re-evaluates** each claimed profile's axes, and
projects a single governed claim matrix that release notes, Help/About,
service-health, support export, and public-proof surfaces ingest instead of
cloning voice-state text by hand.

## Public claim ladder

A row's effective public claim is the strongest of:

- `voice_capable_on_claimed_profile` — fully qualified, current, parity-complete.
- `voice_capable_narrowed_profile` — qualified but narrowed (hosted/enterprise/
  preview, or a full claim auto-narrowed by a failing/stale axis).
- `labs_unadvertised_not_claimed` — Labs/unadvertised; no public claim.
- `voice_unsupported_keyboard_fallback` — capture disabled; keyboard path only.

A claimed profile auto-narrows strictly below its headline claim when any axis
fails (`mode_separation_unverified`, `push_to_talk_default_missing`,
`provider_locality_undisclosed`, `transcript_retention_unbounded`,
`command_parity_incomplete`, `keyboard_fallback_missing`,
`provider_unavailable_downgraded`) or its proof is stale/missing/imported-on-local
(`stale_verification_proof`, `missing_verification_proof`,
`imported_proof_on_local_profile`). Once the matrix verification-freshness window
elapses, any claim resting on a local proof narrows — so surfaces cannot imply
broad stable voice support without current evidence. Labs profiles never widen.

## Usage

```sh
# Regenerate the surface-facing artifacts (generated, never hand-edited):
python3 tools/release/voice_claim_narrowing.py emit-matrix
python3 tools/release/voice_claim_narrowing.py emit-report

# Gate: re-derive from the source matrix and fail on any overclaim/mismatch:
python3 tools/release/voice_claim_narrowing.py validate

# Exercise the narrowing engine over the fixture corpus:
python3 tools/release/voice_claim_narrowing.py corpus

# End-to-end: emit round-trips clean, the checked-in artifact is fresh, corpus passes:
python3 tools/release/voice_claim_narrowing.py self-test
```

`validate` fails (non-zero) when a surface projection renders wider than the
row's effective claim, a narrowed entry lacks a precise label or trigger, the
recorded claim/reasons drift from the re-derived truth, or the summary/publication
decision does not match the recomputed state.

## Outputs

- `artifacts/voice/m5-voice-profile-matrix.json` — the governed claim matrix.
- `artifacts/voice/m5-voice-qualification-report.md` — the certification report.
- `fixtures/voice/qualification-corpus/` — perturbation corpus for the engine.

Regenerate both artifacts after any change to the source qualification matrix.
