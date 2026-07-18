# M5 shared-terminal-debug-view and control-channel-badge registries

Second implement lane over the frozen [M5 collaboration-control component matrix][matrix]
(`m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix`).
It makes the matrix's read-only shared terminal / debugger view stream and its control-channel badge
operable — as durable, resolved records — by carrying honest projections of two registries so the
claimed M5 shared terminal / debug view, companion-follow flow, control-grant prompt, paste / secret
guard, support / export packets, and help / docs surfaces inherit one canonical view-stream descriptor
and one control-channel badge rather than a hand-authored parallel prose that has to be kept
consistent. It closes the gap between the already-landed companion notification / session-follow
lanes, terminal / session restore / no-rerun truth, presence avatar stacks and role / follow badges,
embedded / browser auth handoff components, and incident / support export packets, and the explicit
high-risk shared-control plane the source set now expects: high-risk terminal / debugger authority is
separated from ordinary presence and follow state, the active target context stays visible on every
shared surface, and a presence reconnect, cursor-follow change, or companion resume never implicitly
upgrades a session from view-only to control-capable.

## Registry-A — shared-terminal-debug view stream

One durable, canonical read-only shared terminal / debugger view stream per sensitive session,
carrying:

- a stable session start / stop identity that survives export packets, support bundles, companion
  handoff, and session restore;
- the surface type (shared terminal versus debugger view) and the target context the stream is
  attached to, kept mechanically distinct so a terminal stream never reads as a debugger stream (and
  vice-versa) and the active target is always visible;
- the participant scope and the observing roles the session shows;
- the command or frame markers the stream carries, so viewers can observe terminal or debugger state
  without inheriting input authority or secret visibility;
- the read-only default a session begins in before any control is granted, and the control-channel
  state kept distinct from text / presence channels;
- the resolution-form coverage (canonical object, accessible summary, audit record).

A view stream that cannot bind its identity to its session / target scope, that is a hand-copied
per-entry assumption instead of tracing to the shared registry, or that publishes an incomplete object
degrades honestly instead of letting presence read as control-capable. The registry reuses the matrix
`m5-shared-terminal-debug-view.schema.json` domain schema.

## Registry-B — control-channel badge

The typed control-channel badge a viewer reads before assuming any authority — the surface type, the
target context, and whether input control is unavailable, requestable, granted to a single driver, or
expired — plus the fresh, visible authority event any change to an already-active session must raise: a
control request, a grant to a single driver, an expiry, or a presence reconnect / cursor-follow /
companion resume. The badge stays mechanically distinct from text / presence channels rather than being
flattened into one generic presence badge, and never lets a presence reconnect, follow-mode change, or
companion resume silently upgrade a session from view-only to control-capable, and never lets more than
one active driver hold a sensitive surface. The registry reuses the matrix
`m5-control-grant.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Viewers can observe terminal or debugger state without inheriting input authority or secret
   visibility: a view stream that would let a viewer read as control-capable from presence alone, or
   that reveals raw secrets, command text, variable bodies, or clipboard contents without an explicit
   policy / consent posture and visible guardrail, degrades instead of reading as a clean, observable
   stream.
2. Active badges identify surface type, target context, and whether control is unavailable,
   requestable, granted, or expired: the surface type, target context, observing scope, command or
   frame markers, control-channel state, and read-only default stay visible in the UI projection, the
   CSV / export, and the support packet instead of collapsing into a generic status pill.
3. Collaboration presence never implies terminal / debug control, no sensitive surface carries more
   than one active driver, recording / retention / guest-scope widening / route-visibility expansion
   never starts silently, and prior terminal / debug input never replays on join or restore; the
   registries keep each view-stream and control-channel dimension distinct.

Raw secrets, raw command text, variable bodies, clipboard contents, and private endpoints never cross
this boundary. The Rust validator in `crates/aureline-ui` is the authoritative gate; the checked-in
combined registries schema
(`schemas/collaboration/m5-shared-terminal-debug-view-and-control-channel-badge-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_shared_terminal_debug_control_grant_presenter_consent_retention_and_session_restore_view_matrix/mod.rs
