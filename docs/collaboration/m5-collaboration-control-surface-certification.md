# M5 collaboration-control surface certification (M05-1313)

This contract is the **closing B155 surface-certification capstone** over the frozen M5 shared-terminal/debug-view,
control-grant, presenter-token, consent-envelope, retention-review, and session-restore-view matrix
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`). Where the
freeze matrix defines the six governed collaboration-control object classes — **shared terminal / debug view,
control grant, presenter / moderator token, consent envelope, retention review, and session-restore view** — the
M05-1305–1310 implementation lanes resolve their shared-terminal/debug-view stream, control-grant /
presenter-handoff sheet, consent-envelope / join-review sheet, retention-review / sealed-archive manifest, and
session-restore view / restore-grant posture registry truth; this capstone **certifies** that the shared
collaboration-control truth holds on every claimed M5 **desktop, browser-companion, incident / support, and audit /
export surface** — the control authority, single active driver, presenter handoff, join-time consent scope,
recording / retention state, and replay-free restore safety — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_collaboration_control_surface_certification/`
- **Schema:** `schemas/collaboration/m5-collaboration-control-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-collaboration-control-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/collaboration/m5-collaboration-control-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a shared-session owner, a control-grant / presenter-handoff
flow, a companion-follow consumer, or a support / export consumer reads a collaboration control through, not on the
underlying object class it renders:

1. **Fully-certified collaboration-control lane** — a shared terminal / debug session whose control authority,
   single active driver, presenter handoff, join-time consent scope, recording / retention state, and replay-free
   restore safety all converge on one export-safe, provider-authoritative, internally consistent record identical
   across every consumer. The **only** profile that may certify a `certified_collaboration_control_truth` claim.
2. **Reviewable collaboration-control record structure** — a self-sufficient, inspectable session-bound
   consent-envelope / join-review record; certifies at most `reviewable_collaboration_control_record`.
3. **Unproven-control-authority profile** — a shared terminal / debug view whose control authority can no longer be
   confirmed explicitly granted; auto-narrows to `control_authority_unverified_projection`.
4. **Inferred-active-driver profile** — a control grant whose single active driver cannot be confirmed (a second
   participant risks driving the same sensitive surface); auto-narrows to `active_driver_unverified_projection`.
5. **Silently-transferred-presenter profile** — a presenter token whose handoff cannot be confirmed reviewed;
   auto-narrows to `presenter_handoff_unverified_projection`.
6. **Undisclosed-consent-scope profile** — a consent envelope whose join-time scope cannot be proven disclosed
   (recording, retention, guest scope, or route visibility undisclosed or silently widened); auto-narrows to
   `consent_scope_unverified_projection`.
7. **Stale-retention-state profile** — a retention review whose recording / retention state is stale or was
   broadened silently; auto-narrows to `retention_state_unverified_projection`.
8. **Unproven-replay-free-restore profile** — a session-restore view whose replay-free restore safety and recovery
   evidence is unproven; auto-narrows to `restore_replay_safety_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and collaboration-control-truth behavior — and resolves to
a derived verdict:

- **green** — every axis certified, every invariant held, the claimed collaboration-control tier delivered;
- **yellow** — a truth axis is not current, so the collaboration-control claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified collaboration-control record, or the narrowing is
  inconsistent.

The eight seeded rows cover all six frozen object classes (shared terminal / debug view and consent envelope each
appear on a green and a yellow row), so the certification runs across the full matrix rather than a single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_collaboration_control_truth` / `reviewable_collaboration_control_record` claim while one of its truth
   axes is not current over-claims and blocks.
2. **Only a fully-certified collaboration-control lane may certify a certified collaboration-control record.** Every
   other profile is at most a reviewable collaboration-control record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the control
   authority, single active driver, presenter handoff, consent scope, retention state, and restore safety as
   text / JSON / Markdown.
4. **Every B155 hard invariant holds per row.** No profile may acquire terminal / debug control from presence,
   reconnect, or follow without an explicit grant; allow more than one active driver on a sensitive surface; start
   recording, transcript retention, or guest-scope widening silently; replay prior terminal / debug input on join
   or restore; or reveal raw secrets, command text, variable bodies, or clipboard contents without an explicit
   consent posture and visible guardrail.
5. **One canonical proof bundle.** Every row cites exactly one canonical collaboration-control matrix proof bundle
   (`artifacts/release/m5-collaboration-control-proof/support_export.json`) — the frozen collaboration-control
   matrix proof — so support, docs / help, release, and public-proof surfaces consume a single collaboration-control
   certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_COLLABORATION_CONTROL_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_collaboration_control_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
