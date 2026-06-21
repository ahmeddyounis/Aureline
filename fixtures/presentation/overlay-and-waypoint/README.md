# Presentation overlay / waypoint fixtures

These fixtures are the literal projection of the seeded overlay/navigation
binding corpus in
[`aureline-shell::presentation`](../../../crates/aureline-shell/src/presentation/corpus.rs).
They prove that the presentation overlays — presenter bar, agenda / waypoint
rail, spotlight frame, zoom presets, speaker-notes tray, audience strip, and
breakaway banner — sit on top of Aureline's existing shell zones and navigation
provenance as a **thin, reversible layer**, never a second workspace shell.

## Files

- `overlay-navigation-corpus.json` — the full binding corpus: one case per
  scenario, each binding every active overlay surface to a canonical shell zone,
  with the navigation provenance and the layout checkpoint it restores. This is
  the in-product/inspector truth and carries the navigation refs (file path,
  symbol anchor, branch/workspace) the overlay keeps visible.
- `overlay-navigation-support-export.json` — the support-safe projection: counts,
  enums, host zones, and guardrail booleans only. Raw provenance bodies,
  accessible labels, and scenario copy are excluded.

## Cases

- `overlay-case:solo-rehearsal` — expanded desktop; every surface rides an
  existing zone and the spotlight is a strict inset within the main workspace.
- `overlay-case:shared-breakaway` — expanded desktop with a broken-away leader;
  the durable breakaway banner floats over the transient overlay while the
  presenter anchor and provenance stay visible.
- `overlay-case:compact-docs` — compact desktop with a collapsed sidebar; the
  waypoint rail floats into the transient overlay zone rather than reclaiming the
  editor's space.

## Regenerating

These files are generated, not hand-edited. After changing the binding shape or
the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_overlay_navigation -- corpus \
  > fixtures/presentation/overlay-and-waypoint/overlay-navigation-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_overlay_navigation -- support-export \
  > fixtures/presentation/overlay-and-waypoint/overlay-navigation-support-export.json
```
