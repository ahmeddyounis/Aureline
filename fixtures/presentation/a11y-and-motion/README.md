# Presentation accessibility and motion fixtures

These fixtures are the literal projection of the seeded presentation-accessibility
corpus in
[`aureline-shell::presentation::a11y`](../../../crates/aureline-shell/src/presentation/a11y/corpus.rs).
They prove that the presentation overlay surfaces — presenter bar, agenda /
waypoint rail, spotlight inset, speaker-notes tray, audience strip, breakaway
banner, and provenance strip — meet the **same accessibility and boundary-truth
bar as the rest of the shell**, not a softer "presentation looks fine" bar:

- every surface is keyboard reachable in a single contiguous focus ring, renders
  a visible focus indicator, respects reduced motion, is screen-reader reachable,
  and carries a non-empty accessible label;
- spotlight, zoom, and follow controls stay operable without pointer-only or
  motion-only dependence and never trap focus;
- the local / remote / shared boundary labels are preserved in the boundary
  posture and on the source-bearing surfaces — never flattened to a generic
  badge — and survive into the support export.

The per-surface fidelity and the report's conformance class reuse the shell's
accessibility-support vocabulary in
[`aureline-shell::a11y::tree_contract`](../../../crates/aureline-shell/src/a11y/tree_contract.rs):
`fully_accessible` → `full_accessible` / `exact`, and an honest high-zoom degrade
to a `summarized_reachable` surface → `degraded_accessible` / `degraded`. A
presentation surface therefore reads exactly like any other claimed shell surface.

## Files

- `accessibility-corpus.json` — the in-product / inspector truth: one case per
  scenario, each carrying an accessibility report with its conformance class, the
  boundary posture, and one conformance record per active overlay surface. Each
  report conforms to
  [`schemas/presentation/accessibility-and-boundary-report.schema.json`](../../../schemas/presentation/accessibility-and-boundary-report.schema.json).
- `accessibility-support-export.json` — the support-safe projection: one row per
  report carrying the conformance class (and its shell-vocabulary mapping), zoom
  tier, boundary posture, surface counts, and the accessibility guardrail
  booleans only. Accessible-label bodies and source refs are excluded; the
  boundary labels are intentionally kept so diagnostics stay boundary-honest.

## Cases

- `a11y-case:presenter-standard-local` — a solo rehearsal at standard zoom on a
  local target; every surface is **fully accessible** and the local boundary
  label stays visible.
- `a11y-case:presenter-high-zoom-summarized` — a shared session at high zoom; the
  dense agenda rail and audience strip reflow to a labeled, keyboard-reachable
  **summarized-reachable** form → **degraded-announced**, never a silent
  truncation.
- `a11y-case:broken-away-shared-banner` — a follower who has broken away; the
  durable breakaway banner joins the focus ring as a keyboard-reachable, announced
  control that never traps focus.
- `a11y-case:invited-guests-remote` — an invited-guests session on a remote
  target; the **remote** boundary label is carried explicitly and exported, never
  flattened to a generic shared badge.
- `a11y-case:mixed-boundary-rail` — a walkthrough spanning local, shared, and
  remote targets at high zoom; the distinct boundary labels are all preserved in
  the boundary posture rather than collapsed to one badge.

## Regenerating

These files are generated, not hand-edited. After changing the report shape or
the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_accessibility -- corpus \
  > fixtures/presentation/a11y-and-motion/accessibility-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_accessibility -- support-export \
  > fixtures/presentation/a11y-and-motion/accessibility-support-export.json
```
