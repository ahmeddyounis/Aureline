# M5 session-restore-view and restore-grant-posture registries

Implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's explicit session-restore view operable — as durable,
resolved records — and adds the restore-grant posture, by carrying honest projections of two
registries so the claimed M5 shared terminal / debug view, companion-follow flow, control-grant prompt,
paste / secret guard, support / export packets, and help / docs surfaces inherit one canonical session-restore
view and one restore-grant posture rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed session-policy, control-grant, presenter-handoff,
paste / secret-guard, and retention-review lanes and the explicit replay-free restore contract the source
set now expects: collaboration continuity is preserved after disconnect or restart without silently
replaying input or reacquiring shell / debugger authority.

## Registry-A — session-restore view

One durable, canonical session-restore view per reconnect, carrying:

- the transcript class — replay-free render summary, metadata restore summary, text / comment timeline
  summary, or elevated support / regulatory evidence summary — so each restore names exactly what it renders;
- the restore path and the target context being rejoined, and whether live control was re-requested, shown at
  restore time, so a reconnect cannot carry authority forward silently;
- the replay-free render summary and the view-only default, kept mechanically distinct from ordinary presence
  and follow state;
- the single-restore binding, so the restore posture on one session never reads as control on another;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A restore that cannot bind its restore path / target context to its session / restore scope, that is a
hand-copied per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete
object degrades honestly instead of letting a reconnect read as control-carrying. The registry reuses the
matrix `m5-session-policy-manifest.schema.json` domain schema.

## Registry-B — restore-grant posture

The typed restore-grant posture a participant reads before any restored session touches control — the
transcript class it renders under, the restore path the reconnect took, the target context being rejoined,
the restore outcome (restore disclosed view-only, observing view-only, control re-request pending, control
re-granted, reopen target required, or replay blocked with no rerun), and the audit-safe attribution of who
reconnected or re-requested control — plus the fresh, visible authority event any restore must raise: a
restore-disclosed-view-only, an observing-view-only, a control-re-request, a control-re-grant, a
reopen-target-required, or a replay-blocked-no-rerun outcome. The posture describes restored sessions by
replay-free, view-only control state rather than generic "session reconnected" language, and never replays a
prior terminal / debug input, signal, breakpoint edit, command text, variable body, or clipboard content on
restore. A reconnect or reopen event stays attributable in-session and on export. The registry mints the
`m5-restore-grant-posture.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Restored sessions always declare whether the user is observing, requesting control again, or needs to
   reopen the target from scratch: a restore that would let a reconnect resume under a replay-free render
   summary, metadata restore summary, text / comment timeline summary, or elevated support / regulatory
   evidence summary without disclosing its restore path and control-re-request posture, or without a fresh
   control grant and visible badge, degrades instead of reading as a clean, disclosed restore.
2. No hidden rerun or hidden authority carry-over occurs when reconnecting from desktop, companion, or
   support-follow lanes: the transcript class, restore path, control-re-request posture, restore outcome, and
   attribution stay visible in the UI projection, the CSV / export, and the support packet instead of
   collapsing into a generic status pill — and never carry raw terminal / debug input, command, variable-body,
   or clipboard material or replay it.
3. Collaboration presence never implies terminal / debugger control; join / restore never replays prior
   input, signals, breakpoint edits, or debug actions, and restore stays view-only until a fresh control grant
   is accepted; the registries keep each session-restore-view and restore-grant-posture dimension distinct.

Prior terminal / debug input, prior signals, breakpoint edits, command text, variable bodies, clipboard
contents, and private endpoints never cross this boundary or get replayed on restore. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/collaboration/m5-session-restore-view-and-restore-grant-posture-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
