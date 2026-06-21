# Presentation overlays and navigation

Presentation overlays are a **thin layer over Aureline's existing
pane-and-navigation system** — never a second workspace shell. Entering
presentation mode adds governed overlay chrome on top of the shell zones the user
already has; it does not open a parallel product, improvise window management, or
take the editor's space. This doc is the human-readable face of that contract;
the machine truth is the overlay/navigation binding produced by
[`aureline-shell::presentation`](../../crates/aureline-shell/src/presentation/binding.rs)
and the placement geometry in
[`aureline-shell::layout::presentation_overlays`](../../crates/aureline-shell/src/layout/presentation_overlays.rs).

The canonical session object model (the presentation session, follow waypoints,
speaker notes, audience-follow state, and the reversible overlay projection)
lives in
[`aureline-shell::presentation_mode`](../../crates/aureline-shell/src/presentation_mode/session.rs)
and is documented in
[presentation-and-walkthrough-truth.md](presentation-and-walkthrough-truth.md).
This lane adds the **binding to the live shell layout**: which zone each overlay
surface rides, how it stays out of the underlying panes, and how provenance and
restore survive.

## Surfaces and their host zones

Each overlay surface attaches to one of the canonical shell zones from the live
zone registry. It never mints a new top-level zone.

| Overlay surface       | Host zone                  | Fallback when collapsed             |
| --------------------- | -------------------------- | ----------------------------------- |
| Presenter bar         | Title / context bar        | —                                   |
| Provenance strip      | Title / context bar        | —                                   |
| Agenda / waypoint rail| Left sidebar               | Floats in the transient overlay     |
| Spotlight frame       | Main workspace (inset)     | —                                   |
| Zoom presets          | Presenter bar (controls)   | —                                   |
| Speaker-notes tray    | Right inspector            | Bottom panel, then floats           |
| Audience strip / follow chip | Status bar          | —                                   |
| Breakaway banner      | Transient overlay          | —                                   |

The presenter bar carries the zoom presets, the spotlight toggle, the notes
toggle, and the explicit exit. The presenter bar and provenance strip share the
title / context bar, with provenance keeping the left edge so source identity is
never pushed off-screen.

## Invariants

The binding makes the spec's acceptance criteria checkable rather than asserted:

- **No improvised window management.** Every surface is placed onto an existing
  shell zone via the zone registry; nothing draws its own top-level chrome.
- **The underlying pane stays visible.** No placement replaces or hides a pane.
  The spotlight frame is a strict **inset** within the main workspace pane — it
  dims the surroundings without removing the pane or its content.
- **Provenance survives the overlay.** The current waypoint's navigation anchor —
  file path, symbol anchor, branch / workspace context, and the
  local/remote/shared boundary label — flows into the provenance strip and stays
  visible. Decorative chrome never replaces these labels.
- **Command-backed and keyboard reachable.** Every actionable surface carries the
  stable command id and key-binding ref from the projected overlay; entering and
  leaving presentation are explicit session actions, not implicit side effects.
- **Reversible and layout-safe.** Entering checkpoints the prior layout; exit,
  cancel, and crash recovery all restore that checkpoint, and the user is never
  left in an improvised shell.
- **No widened authority.** Presentation guides attention only. It opens no
  mutation shortcut and following grants no shared control.

## Responsive fallback

When the responsive zone registry collapses a preferred host zone on a narrow
window (for example, the left sidebar on a compact desktop), the affected surface
**floats into the transient overlay zone** rather than forcing the zone back open
or stealing the editor's space. The waypoint rail and the speaker-notes tray both
carry such fallbacks; the spotlight frame and audience strip ride zones that are
always present. Floated placements are flagged so diagnostics and accessibility
surfaces can explain the adaptation.

## Truth and fixtures

The seeded binding corpus and its support-safe export are the canonical truth for
this lane. They are checked in under
[`fixtures/presentation/overlay-and-waypoint/`](../../fixtures/presentation/overlay-and-waypoint/)
and regenerated from the `dump_presentation_overlay_navigation` example, so the
JSON cannot drift from the Rust types. Restore, diagnostics, help, and
support-export surfaces should ingest this binding rather than cloning overlay
placement text by hand.
