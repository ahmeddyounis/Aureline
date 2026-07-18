# M5 session-policy-manifest and join-review-sheet registries

First implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's session-policy manifest and its join-review / consent-envelope sheet operable —
as durable, resolved records — by carrying honest projections of two registries so the claimed M5
desktop join flow, shared terminal / debug view, companion-follow flow, support / export packets, and
help / docs surfaces inherit one canonical session-policy descriptor and one join-review disclosure
rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap between
the already-landed companion notification / session-follow lanes, terminal / session restore /
no-rerun truth, presence avatar stacks and role / follow badges, embedded / browser auth handoff
components, and incident / support export packets, and the explicit high-risk shared-control plus
consent / retention contract the source set now expects: every sensitive session begins view-first,
discloses its retention / guest / export consequences before join, and never silently broadens
retention or authority on restore or companion follow.

## Registry-A — session-policy manifest

One durable, canonical session-policy manifest per collaboration session, carrying:

- a stable session identity that survives export packets, support bundles, companion handoff, and
  session restore;
- the session type and the tenant / guest policy the session runs under, kept mechanically distinct
  so a tenant-internal session never reads as a guest-scoped one (and vice-versa);
- the participant list or scope class, the active roles, and the active badges the session shows;
- the retention envelope and the export / delete posture the session is bound to;
- the read-only default a session begins in before any control is granted;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A manifest that cannot bind its identity to its session / tenant scope, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete
object degrades honestly instead of reading as a session that is safe to join. The registry reuses the
matrix `m5-session-policy-manifest.schema.json` domain schema.

## Registry-B — join-review / consent envelope

The typed join-review disclosure surfaced before a participant joins — who can see the session, what
may be retained, and what authority is available — plus the fresh, visible consent event any change
to an already-active session must raise: an external guest joining, scope widening, a retention-mode
change, or a route-share visibility change. The envelope keeps the disclosure dimensions distinct
rather than flattening them into one generic badge, and never lets recording, transcript retention,
replayable archives, guest-scope widening, or route visibility expansion start silently. The registry
reuses the matrix `m5-collaboration-join-review-sheet.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Users can join, decline, or open details without guesswork about role, guest presence, or
   retention mode: a manifest missing its role, guest-presence, or retention disclosure degrades
   instead of reading as a clean, joinable session, so no participant joins a sensitive surface
   without seeing who can see it, what may be retained, and what authority is available.
2. The same session-policy manifest is reusable across desktop join flows, companion follow flows,
   support exports, and diagnostics / help without surface-local reinterpretation: the session type,
   tenant / guest policy, participant scope, active roles, active badges, retention envelope, export /
   delete posture, and read-only default stay visible in the UI projection, the CSV / export, and the
   support packet instead of collapsing into a generic status pill.
3. Collaboration presence never implies terminal / debug control, no sensitive surface carries more
   than one active driver, recording / retention / guest-scope widening / route-visibility expansion
   never starts silently, and prior terminal / debug input never replays on join or restore; the
   registries keep each disclosure and consent dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-session-policy-manifest-and-join-review-sheet-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
