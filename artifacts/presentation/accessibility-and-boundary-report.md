# Presentation accessibility and boundary-truth report

- Packet: `presentation-accessibility-and-boundary-report:stable:0001`
- Label: `Presentation accessibility / reduced-motion / boundary conformance report`
- Surfaces: 7 / 7 (`presenter_bar`, `waypoint_rail`, `spotlight_frame`, `speaker_notes_tray`, `audience_strip`, `breakaway_banner`, `provenance_strip`)
- Conformance classes: 2 / 2 shippable (`fully_accessible`, `degraded_announced`); `non_conformant` is the never-ship class validation rejects
- Zoom tiers: 2 / 2 (`standard`, `high_zoom`)
- Boundary labels: 3 / 3 (`local`, `remote`, `shared`)
- Audience scopes: 3 / 3 (`solo_rehearsal`, `shared_workspace`, `invited_guests`)
- Source of truth: `aureline-shell::presentation::a11y` seed corpus
- Fixtures: `fixtures/presentation/a11y-and-motion/`
- Schema: `schemas/presentation/accessibility-and-boundary-report.schema.json`
- Contract: `docs/ux/presentation-accessibility.md`

This report is a human-readable projection of the seeded presentation-accessibility
corpus. It shows, per scenario, how each overlay surface meets the shell's
accessibility bar (keyboard order, visible focus, reduced motion, screen-reader
reachability, high-zoom support, accessible labels) and how the local / remote /
shared boundary labels stay visible through the overlay and into the support
export. The conformance class and per-surface support state reuse the shell
accessibility-support vocabulary in `aureline-shell::a11y::tree_contract`; the
machine packet asserts the no-pointer-only, no-motion-only, no-focus-trap, and
no-flattened-boundary guardrails. Accessibility here is governed and inspectable,
not a post-hoc manual signoff.

## Guardrails

| Invariant                                                                | Holds |
| ------------------------------------------------------------------------ | ----- |
| Every overlay surface is keyboard reachable                              | yes   |
| Actionable surfaces form a single contiguous focus ring (`1..=N`)        | yes   |
| Every surface renders a visible focus indicator                          | yes   |
| Every surface is screen-reader reachable                                 | yes   |
| Every surface respects reduced motion (no motion-only state)             | yes   |
| Spotlight, zoom, and follow stay operable without a pointer              | yes   |
| Every surface stays operable at high zoom (reflowed or summarized)       | yes   |
| High-zoom degrades are announced, never silent truncations               | yes   |
| No surface traps keyboard focus                                          | yes   |
| Source / boundary labels are preserved, never erased or flattened        | yes   |
| Support export keeps boundary posture but drops labels and source refs   | yes   |

## Conformance-class mapping

| Conformance class    | Shell support state    | Shell role confidence | Meaning                                                |
| -------------------- | ---------------------- | --------------------- | ------------------------------------------------------ |
| `fully_accessible`   | `full_accessible`      | `exact`               | Every surface reflows in place; fully accessible.       |
| `degraded_announced` | `degraded_accessible`  | `degraded`            | A surface summarizes at high zoom, reachable + announced.|
| `non_conformant`     | `unsupported_blocked`  | `unavailable`         | A hard a11y invariant failed; not shippable (rejected). |

## High-zoom reflow mapping

| Reflow strategy         | Shell support state   | Behavior at high zoom / large text                              |
| ----------------------- | --------------------- | --------------------------------------------------------------- |
| `reflows`               | `full_accessible`     | The surface reflows in place with no loss of content.           |
| `summarized_reachable`  | `degraded_accessible` | A dense list collapses to a labeled, reachable, expandable summary. |

## Scenarios

### `a11y-case:presenter-standard-local`

A solo rehearsal at standard zoom on a local target. Every surface — presenter
bar, agenda rail, spotlight inset, notes tray, audience strip, provenance strip —
is keyboard reachable in a single contiguous focus ring, has a visible focus
indicator, respects reduced motion, is screen-reader reachable, and carries an
accessible label. The local boundary label stays visible: **fully accessible**.

### `a11y-case:presenter-high-zoom-summarized`

A shared session driven at high zoom / large text. The dense agenda rail and
audience strip reflow to a labeled, keyboard-reachable summary that expands on
demand; every other surface reflows in place. Nothing is truncated silently and
nothing becomes pointer-only — the degrade is announced: **degraded-announced**.

### `a11y-case:broken-away-shared-banner`

A follower who has broken away to browse independently. The durable breakaway
banner joins the focus ring as a keyboard-reachable, announced control with a
return-to-presenter action; it never traps focus and the shared boundary label
stays visible: **fully accessible**.

### `a11y-case:invited-guests-remote`

An invited-guests session anchored on a remote target. The remote boundary label
is carried explicitly on the provenance strip and the spotlight inset and exported
as `remote` — never flattened to a generic shared badge — so diagnostics can
explain exactly where the audience is looking: **fully accessible**.

### `a11y-case:mixed-boundary-rail`

A walkthrough whose steps span local, shared, and remote targets, driven at high
zoom. The current step's shared boundary is shown, and the distinct
local / shared / remote labels are all kept in the boundary posture rather than
collapsed to one badge. The summarized rail and audience strip stay reachable:
**degraded-announced**, boundary-honest.
