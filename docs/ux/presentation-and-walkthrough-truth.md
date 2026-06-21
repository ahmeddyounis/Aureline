# Presentation and walkthrough truth

Presentation and walkthrough are a **thin, reversible layer** over Aureline's
existing panes — never a parallel product. File, symbol, branch, workspace, and
local/remote/shared boundary labels stay visible under the overlay; speaker notes
default local/private; follow, break away, request follow, and take over stay
distinct, attributable states; teaching and classroom roles stay separate from
edit, debug, and approval authority; entering presentation checkpoints the prior
layout; and exiting restores it exactly, with no hidden reruns or widened rights.

This doc is the human-readable face of the canonical object model. The machine
truth is the qualification matrix that binds every **claimed** presentation
surface to the canonical presentation-session object it drives, plus verified
speaker-note privacy, audience-follow truth, layout restore, authority
separation, and accessibility evidence — and a claimed and effective
qualification grade.

## What this lane adds

The canonical object model already exists: the
[`PresentationSession`](../../schemas/presentation/presentation-session.schema.json),
[`FollowWaypoint`](../../schemas/presentation/follow-waypoint.schema.json), and
[`SpeakerNote`](../../schemas/presentation/speaker-note.schema.json) boundary
objects in
[`aureline-shell::presentation_mode`](../../crates/aureline-shell/src/presentation_mode/session.rs),
and the classroom roles in
[`aureline-shell::teaching_session`](../../crates/aureline-shell/src/teaching_session/session.rs).
This lane freezes the remaining implicit promise — that a *claimed* presentation
surface rests on **verified** speaker-note privacy, audience-follow/breakaway
truth, layout restore, authority separation, and accessibility evidence — into
one verification-bound matrix that restore, help, accessibility, diagnostics,
support-export, and release surfaces read instead of cloning presentation-state
text by hand.

It does **not** broaden presentation into generic co-editing, a cohort/grading
flow, or a collaboration platform. Claimed presentation surfaces stay clearly
separate from broader future collaboration ambitions, and walkthrough artifacts
never detach from normal source or graph identity.

## Canonical objects

The three boundary schemas under
[`schemas/presentation/`](../../schemas/presentation) are canonical:

- [`presentation-session.schema.json`](../../schemas/presentation/presentation-session.schema.json)
  — the single governed session: layout preset, current anchor, ordered
  follow-waypoints, audience scope/participants, leader/follow posture, the
  restore checkpoint, and the safe guardrail flags (mutation/control/private-data
  authority always denied; local-default notes, preserved provenance, and
  existing-surface reuse always true).
- [`follow-waypoint.schema.json`](../../schemas/presentation/follow-waypoint.schema.json)
  — one prepared anchor bound to a stable object on an *existing* surface
  (editor, diff, docs, graph, notebook), preserving file path, symbol anchor,
  branch/workspace, and boundary label so chrome never erases provenance.
- [`speaker-note.schema.json`](../../schemas/presentation/speaker-note.schema.json)
  — a presenter-only prompt that defaults to a local scope and only becomes
  shared through an explicit, separately recorded promotion.

The qualification matrix
([`M5PresentationQualificationMatrixPacket`](../../crates/aureline-shell/src/freeze_the_m5_presentation_session_walkthrough_waypoint_speaker_note_and_audience_follow_matrix/mod.rs))
embeds a real, redacted-for-export `PresentationSession` in each row and layers
five qualification axes on top:

- **speaker-note privacy** — notes default local-only, sharing is an explicit
  promotion, and raw note bodies never enter any export;
- **audience-follow truth** — follow / break away / request follow / take over
  are distinct states, a breakaway shows a durable banner, and following grants
  no control;
- **authority separation** — a teaching/classroom role drives attention but never
  edit, debug, or approval authority, and presentation opens no mutation shortcut;
- **layout restore** — entering checkpoints the prior layout and exit, cancel,
  and crash recovery all restore it exactly (proven by replaying the canonical
  restore path, not asserted);
- **accessibility** — every affordance is keyboard-first and announced, reveal
  motion honors reduced motion, and provenance labels stay visible.

## Verification-bound, not asserted

Each row names a proof currency and, unless the proof is missing, a reopenable
proof ref keyed by a non-display fingerprint, so a reviewer reopens the same
evidence object that backs the claim. A claimed surface **auto-downgrades** —
strictly below its claim, with a recorded trigger and a precise label, never a
generic non-answer — whenever speaker-note privacy, follow/breakaway truth,
layout restore, authority separation, or accessibility evidence goes unverified,
provenance is erased, the surface goes unavailable, or the proof is stale,
missing, or imported-on-local. Unclaimed (Labs/unadvertised) surfaces make no
claim to downgrade from and stay separate from claimed scope.

## Absolute invariants

These hold for every row, even a downgraded or Labs one, and the packet refuses
any export that breaks them:

- a complete keyboard-first path is always available — presentation is never a
  dead end;
- raw speaker-note bodies never enter support/diagnostics/telemetry exports;
- no row widens mutation, control, or private-data authority;
- every row restores the checkpointed layout under exit, cancel, and crash
  recovery, never stranding the user in an improvised shell;
- every waypoint reuses an existing surface and preserves its source provenance.

## Published artifacts

- [`artifacts/presentation/m5-presentation-qualification-matrix.md`](../../artifacts/presentation/m5-presentation-qualification-matrix.md)
  — the published Markdown qualification matrix.
- [`artifacts/presentation/m5-presentation-qualification-matrix/support_export.json`](../../artifacts/presentation/m5-presentation-qualification-matrix/support_export.json)
  — the export-safe support packet, regenerated from the in-crate builder.
- The schema example artifacts under
  [`artifacts/presentation/`](../../artifacts/presentation) are lifted from the
  clean presenter-walkthrough row.

Regenerate the artifacts after any change to the builder with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_presentation_qualification -- write-all
```
