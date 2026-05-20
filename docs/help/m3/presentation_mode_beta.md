# Presentation Mode Beta

This beta contract turns presentation overlays, speaker-note trays, and
follow-or-breakaway controls into one governed runtime object. Presentation mode
is a **thin, reversible layer over existing Aureline surfaces** — editors,
diffs, topology graphs, docs, and notebooks — not a parallel product with hidden
authority, lost context, or pointer-only controls. It guides attention; it never
mints a new artifact type, opens a mutation shortcut, widens collaboration
authority, or claims private data ownership.

## Runtime Contract

- A presentation session is a versioned `presentation_session_record` with a
  session id, lifecycle state, leader/follow state, layout preset, audience
  scope, current focus ref, a waypoint list, an audience-participant list, and a
  restore checkpoint.
- Each follow waypoint binds to a stable object on an **existing** surface
  (`editor`, `diff`, `docs`, `graph`, or `notebook`) and preserves the file
  path, symbol anchor, branch/workspace context, and local/remote/shared
  boundary label. Waypoints set `reuses_existing_surface = true` and
  `creates_parallel_artifact = false`; presentation mode reuses surfaces rather
  than duplicating them.
- Leader/follow, breakaway, and return states are explicit and attributable.
  `Follow`, `Break away`, `Request follow`, and `Take over` are distinct verbs
  on the follow chip and audience strip; the shell never infers them from cursor
  movement alone. A broken-away local user always sees the breakaway banner with
  a keyboard return to the presenter's anchor.
- Speaker notes default to a `local` scope and never leave the machine. A note
  becomes `shared` only through an explicit promotion that sets
  `shared_promotion_explicit = true`, so the share decision stays auditable.
- The session never widens authority: `grants_mutation_authority`,
  `grants_control_authority`, and `establishes_private_data_ownership` are always
  false; `speaker_notes_default_local_only`, `preserves_source_provenance`, and
  `reuses_existing_surfaces_only` are always true. Mutation-capable teaching
  still flows through the ordinary command graph, never through a presentation
  shortcut.

## Overlay Surfaces

The overlay projects the six design-system presentation surfaces plus a
provenance strip and a restore affordance. Every actionable control is a
keyboard-reachable, attention-only action (a stable command id, a key-binding
ref, and an accessible label; never destructive, never mutating). The overlay
declares `keyboard_complete = true`, `pointer_only = false`, and
`screen_reader_reachable = true`.

- **Presenter bar** — context title, zoom preset, spotlight toggle, follow-state
  control, notes toggle, and exit action.
- **Agenda / waypoint rail** — ordered steps with completion state, the current
  step marked, and a keyboard jump per step.
- **Spotlight frame** — highlighted region with dimmed surroundings, an
  accessible region label, a clear-spotlight action, and preserved keyboard
  order and reduced-motion preferences.
- **Speaker-notes tray** — per-note scope, `has_body`, next-step cue, and
  explicit-promotion state; defaults local-only.
- **Audience strip / follow chip** — participant counts, follow/breakaway/
  requesting-follow counts, external-guest count, and the follow / break-away /
  return / request-follow / take-over verbs.
- **Breakaway banner** — the explicit "You are browsing independently" state and
  a keyboard return to the presenter's current anchor; shown only while the
  local user is broken away.

## Reversibility

Entering presentation mode checkpoints the prior layout, focus, panel
visibility, and accessibility posture. Exiting, cancelling, or recovering from a
crash all replay that same checkpoint, so the prior environment returns exactly
and the user is never left in an improvised shell. `restore_from_checkpoint`
emits a `presentation_restore_outcome_record` with `matches_checkpoint = true`
and `left_in_improvised_shell = false` for each trigger (`exit`, `cancel`,
`crash_recovery`).

## Runtime Consumers

The shell beta model is implemented in
`crates/aureline-shell/src/presentation_mode/`.

The headless inspector emits:

- `fixtures/help/m3/presentation_mode/corpus.json`
- `fixtures/help/m3/presentation_mode/sessions.json`
- `fixtures/help/m3/presentation_mode/restore_outcomes.json`
- `fixtures/help/m3/presentation_mode/support_export.json`

The support export is metadata-only. It records lifecycle, leader/follow, and
audience-scope enums, layout preset, surface kinds, boundary labels, waypoint
and participant counts, note scope counts, the guardrail booleans, and the
keyboard / screen-reader posture. It omits note bodies, step titles, scenario
copy, and raw file paths.

## Schemas

- `schemas/help/presentation_session.schema.json` defines the presentation
  session, follow waypoint, speaker note, audience participant, and restore
  checkpoint records.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- validate
cargo test -p aureline-shell --lib presentation_mode
cargo test -p aureline-shell --test presentation_mode_beta_fixtures
```
