# M5 Presence Avatar Stack and Role-or-Follow Badge Primitive Contract

Task: **M05-856** — Ship presence avatar stacks and role-or-follow badges with names,
roles, recording state, and local-fallback continuity across claimed M5 collaboration
surfaces. Batch **B100** (runtime-boundary and repair component truth).

This primitive narrows the `presence_avatar_stack` family frozen in the runtime-boundary
component matrix
(`schemas/ui/m5-runtime-boundary-components.schema.json`) into one reusable resolver plus
a companion role-or-follow badge, and binds them across every claimed M5 collaboration
surface. It is the presence twin of the terminal-tab (M05-853), remote-target /
environment-strip (M05-854), and toolchain-pin / switch-review (M05-855) primitives.

## What it guarantees

- **One primitive, every surface.** The collaboration strip, shared terminal header,
  shared debug pane, review / session header, presenter HUD, follow-mode banner, session
  roster panel, activity-center presence entry, and shared preview header all read the
  same resolved presence stack. No surface invents a second presence grammar.
- **Who is present, who presents, who is followed.** `resolve_presence_stack` derives the
  presenter (the participant presenting to others, or, failing that, the one holding the
  presenter role), the control holder (the shared-control-token holder), and whether the
  local view is being followed — surfaced directly from the component (AC1).
- **Collaboration stays visible when the link degrades.** The continuity posture is
  derived from the collaboration link state: `live → live`, `degraded → degraded_visible`,
  `reconnecting → reconnecting_visible`, `offline_local_fallback → local_fallback_visible`,
  `session_ended → ended_last_known_visible`. A degraded, reconnecting, offline, or ended
  link keeps the roster, roles, and follow badges visible instead of collapsing into a
  generic session banner (AC2), and a reconnect action stays attached while the link is
  recoverable.
- **Local-fallback continuity preserves who had control.** When the link is lost, the
  last-known roster — including who held control — remains visible; collaboration loss
  downgrades the badges/stack rather than erasing presence (third implementation
  requirement).
- **Never avatar-only.** A textual participant list (`text_participant_list`) is a
  mandatory avatar-stack part and the role/follow labels are mandatory badge parts, so no
  presence truth is encoded in avatar imagery alone; every row is keyboard focusable and
  screen-reader announced (AC3).

## Resolver

`resolve_presence_stack(&M5PresenceStackResolutionInput) -> Result<M5ResolvedPresenceStack, M5PresenceStackResolutionError>`

Input: `session_title` (opaque), `participants` (`participant_repr` opaque, `role`,
`follow_state`, `liveness`, `is_self`), `link_state`, `recording_cue`.

Derivations:

- **Presenter** = first participant with `follow_state = presenting_to_others`, else first
  with `role = presenter`.
- **Control holder** = first participant with `role = control_holder`.
- **Ordered stack** = participants sorted by role salience (presenter, control holder,
  session host, collaborator, observer), self first within a tie, then declared order.
- **`current_view_being_followed`** = the local user is being followed or presenting.
- **`self_is_following_presenter`** = the local user's follow state is
  `following_presenter`.
- **Continuity posture** = `link_state.continuity_posture()`.
- **Actions** = `view_participant_list` always; `follow_presenter` when a presenter exists
  and the local user is not already following and is not the presenter; `stop_following`
  when the local user is following; `reconnect_collaboration` when the link is
  reconnectable (degraded / reconnecting / offline).

Errors: `empty_session_title`, `empty_participants`, `empty_participant_repr`,
`duplicate_participant`, `duplicate_self_participant`, `forbidden_presence_material`.

## Reused vs minted vocabulary

Reused verbatim from the frozen runtime-boundary matrix: `M5CollaborationRole` (5),
`M5FollowState` (5), `M5RuntimeBoundaryAccessibilityRoute` (6),
`M5RuntimeBoundaryQualificationClass`, `M5RuntimeBoundaryDowngradeTrigger`. Shell topology
(zones, responsive classes, window classes, consumer surfaces) is reused from the
shell-zone matrix.

Minted here (only what the frozen matrix left implicit about the stack and the badge):
`M5PresenceConsumerSurface` (9), `M5PresenceParticipantLiveness` (5),
`M5CollaborationLinkState` (5), `M5PresenceContinuityPosture` (5),
`M5RecordingRetentionCue` (5), `M5PresenceAvatarStackPart` (7, mandatory 4),
`M5RoleFollowBadgePart` (5, mandatory 2), `M5PresenceAction` (4),
`M5PresenceExportField` (11, mandatory 6).

## Hard invariants (every row, all `false`)

`masks_collaboration_role`, `leaves_follow_state_ambiguous`,
`relies_on_avatar_imagery_alone`, `drops_presence_when_degraded`.

## Acceptance-criterion coverage lints

- `presenter_visibility_unproven` — some worked case must identify a presenter and some
  must show the local view being followed (AC1).
- `degraded_continuity_unproven` — some worked case must keep collaboration visible through
  a degraded / reconnecting link (AC2).
- `local_fallback_continuity_unproven` — some worked case must preserve who was present and
  who had control through a local-fallback / ended posture (AC3 / continuity requirement).

## Artifacts

- Packet schema: `schemas/ui/m5-presence-avatar-stack.schema.json`
- Companion badge fragment: `schemas/ui/m5-role-follow-badge.schema.json`
- Support export: `artifacts/release/m5-presence-avatar-stack-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-presence-avatar-stack-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-presence-avatar-stack-primitive.md`
- Design matrix: `artifacts/design/m5-presence-avatar-stack-primitive-matrix.md`
- Narrowed fixtures: `fixtures/ui/m5-presence-avatar-stack-primitive/`

All artifacts are minted from the single seed builder via the headless emitter
`aureline_shell_m5_presence_avatar_stack_primitive`; the inline tests assert the
checked-in export and fixtures never drift from the seed.
