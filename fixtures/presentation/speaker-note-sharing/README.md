# Presentation speaker-note sharing fixtures

These fixtures are the literal projection of the seeded speaker-note sharing
corpus in
[`aureline-shell::presentation::speaker_notes`](../../../crates/aureline-shell/src/presentation/speaker_notes/corpus.rs).
They prove that presentation speaker notes default to a local/private scope, only
become shared through an explicit, separately recorded promotion, preserve typed
citations to files, symbols, docs, and graph objects, and never leak a private
note onto an audience / follower surface.

## Files

- `speaker-note-sharing-corpus.json` — the in-product/inspector truth: one case
  per scenario, each carrying the governed notes (with their bodies and typed
  citations), the share records for any explicit promotions, and the audience
  disclosures a follower surface may see. This is the canonical object truth and
  carries the presenter-facing note bodies the in-product tray renders.
- `speaker-note-sharing-support-export.json` — the support-safe projection:
  one diagnostics row per note carrying scope, retention / shared-state / export
  posture, citation counts and kinds, and presence flags only. Raw note bodies
  and next-step cues are excluded by construction. Validated against
  [`schemas/presentation/speaker-note-export.schema.json`](../../../schemas/presentation/speaker-note-export.schema.json).

## Cases

- `speaker-note-case:solo-rehearsal-local-notes` — solo rehearsal; every note
  stays local-only with citations across all four kinds, and there is no audience
  disclosure to leak to.
- `speaker-note-case:shared-workspace-one-promoted` — shared workspace; one note
  is explicitly promoted and retained in the session store while a private aside
  stays local. Only the promoted note reaches the audience disclosure.
- `speaker-note-case:policy-retention-disabled-share` — a note is explicitly
  promoted but retention is disabled by policy, so the shared note is delivered
  live only and never persisted.

## Regenerating

These files are generated, not hand-edited. After changing the governance model
or the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_speaker_note_sharing -- corpus \
  > fixtures/presentation/speaker-note-sharing/speaker-note-sharing-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_speaker_note_sharing -- support-export \
  > fixtures/presentation/speaker-note-sharing/speaker-note-sharing-support-export.json
```
