# Presentation Claim Narrowing Tool

`tools/release/presentation_claim_narrowing.py` certifies the claimed M5
presentation and walkthrough surfaces and auto-narrows presentation-bearing
public claims when speaker-note privacy, follow/break-away truth, authority
separation, layout restore, accessibility, or verification evidence is stale,
failing, or limited to a narrower client/scope.

The canonical truth is the presentation qualification matrix support export
(`artifacts/presentation/m5-presentation-qualification-matrix/support_export.json`).
This tool ingests it, **independently re-evaluates** each claimed surface's axes,
and projects a single governed claim matrix that release notes, Help/About,
restore, accessibility, diagnostics, support export, and public-proof surfaces
ingest instead of cloning presentation-state text by hand.

## Public claim ladder

A row's effective public claim is the strongest of:

- `presentation_capable_on_claimed_surface` — fully qualified, current, every
  axis verified.
- `presentation_capable_narrowed_surface` — qualified but narrowed (shared/
  preview, or a full claim auto-narrowed by a failing/stale axis).
- `labs_unadvertised_not_claimed` — Labs/unadvertised; no public claim.
- `presentation_unsupported_keyboard_walkthrough` — overlay unavailable;
  keyboard-first walkthrough path only.

A claimed surface auto-narrows strictly below its headline claim when any axis
fails (`speaker_note_privacy_unverified`, `follow_state_truth_unverified`,
`authority_separation_unverified`, `layout_restore_unverified`,
`accessibility_unverified`, `session_boundary_widened`,
`surface_unavailable_downgraded`) or its proof is stale/missing/imported-on-local
(`stale_verification_proof`, `missing_verification_proof`,
`imported_proof_on_local_surface`). Once the matrix verification-freshness window
elapses, any claim resting on a local proof narrows — so surfaces cannot imply
broad stable presentation support without current evidence. An unavailable
overlay floors at `presentation_unsupported_keyboard_walkthrough`. Labs surfaces
never widen.

## Usage

```sh
# Regenerate the surface-facing artifacts (generated, never hand-edited):
python3 tools/release/presentation_claim_narrowing.py emit-matrix
python3 tools/release/presentation_claim_narrowing.py emit-report

# Gate: re-derive from the source matrix and fail on any overclaim/mismatch:
python3 tools/release/presentation_claim_narrowing.py validate

# Exercise the narrowing engine over the fixture corpus:
python3 tools/release/presentation_claim_narrowing.py corpus

# End-to-end: emit round-trips clean, the checked-in artifacts are fresh, corpus passes:
python3 tools/release/presentation_claim_narrowing.py self-test
```

`validate` fails (non-zero) when a surface projection renders wider than the
row's effective claim, a narrowed entry lacks a precise label or trigger, the
recorded claim/reasons drift from the re-derived truth, or the summary/publication
decision does not match the recomputed state.

## Outputs

- `artifacts/presentation/m5-presentation-profile-matrix.json` — the governed claim matrix.
- `artifacts/presentation/m5-presentation-qualification-report.md` — the certification report.
