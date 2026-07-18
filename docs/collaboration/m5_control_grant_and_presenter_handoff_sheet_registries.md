# M5 control-grant and presenter-handoff-sheet registries

Third implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's explicit control-grant and presenter-handoff objects operable — as durable,
resolved records — by carrying honest projections of two registries so the claimed M5 shared terminal /
debug view, companion-follow flow, control-grant prompt, paste / secret guard, support / export
packets, and help / docs surfaces inherit one canonical control-grant sheet and one presenter-handoff
sheet rather than a hand-authored parallel prose that has to be kept consistent. It closes the gap
between the already-landed shared-terminal / debug view stream, control-channel badge, session-policy
manifest, and join-review lanes and the explicit high-risk control plane the source set now expects:
terminal / debugger write control is granted only through an explicit, time-boxed, revocable grant with
a single active driver, and a presenter / moderator handoff never silently transfers shell / debugger
control.

## Registry-A — control-grant sheet

One durable, canonical control-grant sheet per sensitive terminal / debug session, carrying:

- the requester, the issuer, and the accepter identities of the grant, so request-control, grant,
  deny, revoke, and expire each name who acted;
- the granted scope and the target context the grant is attached to, kept mechanically distinct so a
  grant on one surface never reads as authority on another;
- the time-box and expiry the grant is bounded by, and the revoke path that ends it early;
- the single-active-driver binding, so no two participants can simultaneously hold mutating control of
  the same terminal or debugger surface;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A control grant that cannot bind its identity to its session / target scope, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete object
degrades honestly instead of letting presence read as control-capable. The registry reuses the matrix
`m5-control-grant.schema.json` domain schema.

## Registry-B — presenter-handoff sheet

The typed presenter-handoff sheet a moderator reads before assuming any authority — the presenter /
moderator token, its holder, the requester / issuer / accepter, the granted scope, and whether write
control is unavailable, requestable, granted to a single driver, or expired — plus the fresh, visible
authority event any change to an already-active session must raise: a request, a grant to a single
driver, a deny, a revoke, an expiry, or a presenter handoff along the handoff chain. Presenter /
moderator moderation paths stay mechanically distinct from shell / debugger write control rather than
being flattened into one generic presence badge, and never let a presence reconnect, follow-mode
change, or companion resume silently upgrade a session from view-only to control-capable, and never let
more than one active driver hold a sensitive surface. The registry reuses the matrix
`m5-presenter-handoff-sheet.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. No two participants can simultaneously hold mutating control of the same terminal or debugger
   surface: a control grant that would let a second driver acquire mutating control, or that reveals
   raw secrets, command text, variable bodies, or clipboard contents without an explicit policy /
   consent posture and visible guardrail, degrades instead of reading as a clean, single-driver grant.
2. Control-grant history remains visible in-session and exportable as audit-safe metadata without
   requiring raw command capture: the requester, issuer, accepter, scope, time-box, revoke path, and
   handoff chain stay visible in the UI projection, the CSV / export, and the support packet instead of
   collapsing into a generic status pill — and never carry raw command text.
3. Collaboration presence never implies terminal / debug control, no sensitive surface carries more
   than one active driver, recording / retention / guest-scope widening / route-visibility expansion
   never starts silently, and prior terminal / debug input never replays on join or restore; the
   registries keep each control-grant and presenter-handoff dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-control-grant-and-presenter-handoff-sheet-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
