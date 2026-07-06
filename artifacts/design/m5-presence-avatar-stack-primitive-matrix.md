# M5 Presence Avatar Stack and Role-or-Follow Badge Primitive — Design Matrix

Task **M05-856** / batch **B100**. Design-side companion to the machine-minted proof under
`artifacts/release/m5-presence-avatar-stack-proof/` and the contract at
`docs/components/m5_presence_avatar_stack_primitive_contract.md`.

One reusable presence primitive (avatar stack + role-or-follow badge) projected across
nine claimed M5 collaboration surfaces so shared state stays legible — and never
decorative or vanishing — when a session degrades.

## Surfaces × shell zone × qualification

| # | Collaboration surface | Shell zone | Qualification | Worked resolutions |
|---|-----------------------|------------|---------------|--------------------|
| 1 | Collaboration Strip | `status_bar` | stable | follow-presenter (live); degraded-visible + reconnect |
| 2 | Shared Terminal Header | `bottom_panel` | stable | control-holder driving; self presenting (view followed) |
| 3 | Shared Debug Pane | `main_workspace` | stable (Beta in narrowed fixture) | follow-available; reconnecting keeps paused-follow roster |
| 4 | Review / Session Header | `title_context_bar` | stable (Preview in narrowed fixture) | being-followed (retained); ended keeps last-known control holder |
| 5 | Presenter HUD | `transient_overlay` | stable | offline local-fallback keeps presenter + control holder; live follow |
| 6 | Follow-Mode Banner | `title_context_bar` | stable | active follow; degraded paused-follow + reconnect |
| 7 | Session Roster Panel | `right_inspector` | stable | full five-role roster; reconnecting keeps a departed participant |
| 8 | Activity-Center Presence | `activity_rail` | stable | quiet live; offline local-fallback + reconnect |
| 9 | Shared Preview Header | `main_workspace` | stable | follow designer; quiet live (not-recorded) |

## Anatomy

- **Avatar stack parts** (mandatory ★): avatar_stack, participant_identity ★, role_badge ★,
  follow_state_badge ★, recording_retention_cue, overflow_count, text_participant_list ★.
- **Role-or-follow badge parts** (mandatory ★): role_label ★, follow_label ★,
  presenter_marker, control_holder_marker, self_marker.

The textual participant list and the text role/follow labels are mandatory so presence is
never avatar-only (AC3).

## Derived state vocabulary

- **Collaboration roles** (reused): session_host, collaborator, presenter, observer,
  control_holder.
- **Follow states** (reused): following_presenter, being_followed, not_following,
  presenting_to_others, follow_paused.
- **Participant liveness**: active, idle, reconnecting, departed, last_known_local.
- **Link state → continuity posture**: live → live; degraded → degraded_visible;
  reconnecting → reconnecting_visible; offline_local_fallback → local_fallback_visible;
  session_ended → ended_last_known_visible.
- **Recording-or-retention cue**: not_applicable, recording, retained, retention_pending,
  not_recorded.
- **Actions**: view_participant_list, follow_presenter, stop_following,
  reconnect_collaboration.

## Acceptance-criterion mapping

| AC | Guarantee | Proof lint |
|----|-----------|------------|
| AC1 | Tell who is present / presenting / followed from the component | `presenter_visibility_unproven` |
| AC2 | Collaboration stays visible through degraded / reconnecting flows | `degraded_continuity_unproven` |
| AC3 + continuity | Local-fallback keeps who was present / in control; keyboard + SR reachable, never avatar-only | `local_fallback_continuity_unproven`; mandatory text list + keyboard route |

## Hard invariants (all `false` on every row)

`masks_collaboration_role`, `leaves_follow_state_ambiguous`,
`relies_on_avatar_imagery_alone`, `drops_presence_when_degraded`.
