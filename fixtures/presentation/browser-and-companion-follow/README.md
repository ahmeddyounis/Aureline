# Browser / companion follow-state fixtures

These fixtures are the literal projection of the seeded cross-client
follow-state corpus in
[`aureline-shell::presentation::follow_state`](../../../crates/aureline-shell/src/presentation/follow_state/corpus.rs).
They prove that follow, break away, request follow, take over, and cached
snapshot are **explicit, attributable states** spoken with one vocabulary on the
desktop, browser, and companion clients — never inferred from viewport drift,
connection timing, or a transient toast.

## Files

- `follow-state-truth-corpus.json` — the full follow-state corpus: one case per
  scenario, each carrying a [`FollowStateTruth`] packet with one client view per
  participating surface. This is the in-product / inspector truth and conforms to
  [`schemas/presentation/follow-state-truth.schema.json`](../../../schemas/presentation/follow-state-truth.schema.json).
- `follow-state-truth-support-export.json` — the support-safe projection: one row
  per client view carrying enums, recovery-action kinds, and guardrail booleans
  only. Anchor refs, accessible labels, and scenario copy are excluded.

## Cases

- `follow-case:all-live-cross-client` — desktop presenting while the browser and
  companion follow the live route; all three read as **live**.
- `follow-case:mixed-independent` — the browser has **broken away** behind a
  durable banner with a return-to-presenter path, while the companion has
  **requested follow** and is waiting to resync; neither reads as live.
- `follow-case:companion-cached-snapshot` — the provider went offline for the
  companion, which now shows a self-labeled **cached snapshot** (it never claims
  to be a live shared route) while the desktop presents and the browser follows.
- `follow-case:browser-take-over-request` — a browser co-presenter explicitly
  **requests take-over** while still seeing the live route; a distinct,
  attributable state, not an inferred control grab.

## Regenerating

These files are generated, not hand-edited. After changing the follow-state shape
or the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_follow_state -- corpus \
  > fixtures/presentation/browser-and-companion-follow/follow-state-truth-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_follow_state -- support-export \
  > fixtures/presentation/browser-and-companion-follow/follow-state-truth-support-export.json
```
