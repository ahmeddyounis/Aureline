# Presentation accessibility, motion, and boundary truth

Presentation mode is chrome layered over the real workspace, and that chrome is
exactly where accessibility quietly slips: a spotlight that dims the layout for a
mouse user, a zoom preset reachable only by clicking, an audience strip that reads
as a blur to a screen reader, a "shared" badge that hides whether the audience is
looking at a local file or a remote one. This doc is the human-readable face of
the contract that keeps that from happening: **a claimed presentation surface
passes the same accessibility and boundary-truth expectations as the rest of the
shell.** The machine truth is the accessibility / boundary conformance packet
produced by
[`aureline-shell::presentation::a11y`](../../crates/aureline-shell/src/presentation/a11y/conformance.rs),
seeded and validated by its
[corpus](../../crates/aureline-shell/src/presentation/a11y/corpus.rs),
frozen at
[`schemas/presentation/accessibility-and-boundary-report.schema.json`](../../schemas/presentation/accessibility-and-boundary-report.schema.json),
and covered by the
[accessibility and boundary report](../../artifacts/presentation/accessibility-and-boundary-report.md).

The canonical session object model (the presentation session, follow waypoints,
speaker notes, and the reversible overlay) lives in
[`aureline-shell::presentation_mode`](../../crates/aureline-shell/src/presentation_mode/session.rs);
the thin overlay/navigation binding is in
[`aureline-shell::presentation::binding`](../../crates/aureline-shell/src/presentation/binding.rs).
This lane adds the **per-surface accessibility and boundary conformance** that
proves the overlay is shippable rather than merely drawn.

## Accessibility is proven per surface, not signed off once

Every active overlay surface carries a conformance record asserting the named
dimensions. None of them is allowed to ride on a manual checklist:

| Surface              | In the focus ring | Carries the boundary label |
| -------------------- | ----------------- | -------------------------- |
| `presenter_bar`      | yes               | —                          |
| `waypoint_rail`      | yes               | —                          |
| `spotlight_frame`    | yes               | yes (the current target)   |
| `speaker_notes_tray` | yes               | —                          |
| `audience_strip`     | yes               | —                          |
| `breakaway_banner`   | yes (when shown)  | —                          |
| `provenance_strip`   | no (display-only) | yes (the current target)   |

For every surface the packet asserts, and validation re-derives:

- **Keyboard order.** Actionable surfaces form a single contiguous `1..=N` focus
  ring with no gaps or duplicates; the display-only provenance strip is screen-
  reader reachable but is not in the tab ring.
- **Visible focus.** Each surface renders a visible focus indicator when focused.
- **Reduced motion.** Any motion (spotlight reveal, zoom transition) respects the
  reduced-motion preference and has a non-animated equivalent; no state is
  conveyed by motion alone.
- **Screen-reader reachability.** Each surface is reachable by assistive
  technology and carries a non-empty accessible label.
- **High-zoom support.** Each surface stays operable at high zoom / large text —
  either reflowing in place or, for the dense agenda rail and audience strip,
  collapsing to a labeled, keyboard-reachable summary that expands on demand.
- **Not pointer-only, not motion-only, never a focus trap.** Spotlight, zoom, and
  follow controls are operable without a pointer, and no overlay surface traps
  keyboard focus.

A spotlight or audience overlay that trapped focus, dropped its accessible label,
or became pointer-only fails validation — it can never be reported as conformant.

## High-zoom degrades are announced, never silent

At high zoom the agenda rail and audience strip can carry more rows than fit. They
do not truncate silently: they reflow to a `summarized_reachable` form — a labeled
count that stays in the focus ring and expands on demand. That honest degrade is
reported as **`degraded_announced`** (mapped to the shell's `degraded_accessible`
support state), distinct from a fully reflowing **`fully_accessible`** overlay and
from a **`non_conformant`** overlay that validation rejects. The summary is a
disclosed state, not a quietly dropped one.

## Boundary labels survive the overlay and the export

The local / remote / shared boundary is part of accessibility truth: a follower
must be able to tell whether the spotlighted target is a local file, a remote /
managed object, or a shared collaboration object. The overlay keeps that explicit:

- the **provenance strip** and the **spotlight inset** both carry the current
  target's boundary label, and neither may erase it;
- the report's **boundary posture** keeps the current label *and* the distinct set
  of labels across the walkthrough, so a mixed local + shared + remote session is
  never collapsed to a single generic badge;
- the **support export** drops accessible-label bodies and source refs but keeps
  the boundary posture and conformance class, so a diagnostics or support surface
  can explain *what the audience is looking at, where it lives, and how accessible
  the overlay is* without flattening either posture away.

## Parity with the rest of the shell

The conformance class and per-surface support state are not a presentation-only
vocabulary. They map one-to-one onto the shell accessibility-support model in
[`aureline-shell::a11y::tree_contract`](../../crates/aureline-shell/src/a11y/tree_contract.rs):
`fully_accessible` → `full_accessible` / `exact`, `degraded_announced` →
`degraded_accessible` / `degraded`, `non_conformant` → `unsupported_blocked` /
`unavailable`. A presentation surface is read with the same vocabulary as an
editor, a list, or a status surface — so "the presentation passes accessibility"
means the same thing everywhere.
