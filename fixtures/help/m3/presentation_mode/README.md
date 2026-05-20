# Presentation-mode (beta) fixture corpus

Reviewable fixtures for the beta presentation overlays that live in
[`crates/aureline-shell/src/presentation_mode/`](../../../../crates/aureline-shell/src/presentation_mode/).

Each JSON file is a literal projection of the seeded `PresentationModeCorpus`
produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_presentation_mode.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_presentation_mode.rs)).
The inspector is the only mint-from-truth path for these fixtures, so the
checked-in JSON cannot drift from the Rust types. Every session is a
`presentation_session_record` that conforms to the boundary schema at
[`schemas/help/presentation_session.schema.json`](../../../../schemas/help/presentation_session.schema.json)
and is documented in
[`docs/help/m3/presentation_mode_beta.md`](../../../../docs/help/m3/presentation_mode_beta.md).

All records carry the shared contract ref `shell:presentation_mode_beta:v1` so
shell UI rows, the headless CLI rows, and the support-export rows pivot to the
same `case_id` and `session_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`corpus.json`](./corpus.json) | Full corpus: aggregate coverage summary, one session case per scenario (solo rehearsal, shared-workspace following audience, invited-guests shared note, local-user breakaway, following-presenter graph, requesting-follow re-sync), each paired with its reversible overlay projection, plus the restore outcomes. |
| [`sessions.json`](./sessions.json) | The session case vector embedded in the corpus, broken out for row-level review. |
| [`restore_outcomes.json`](./restore_outcomes.json) | Restore outcomes proving the prior layout, focus, panels, and accessibility posture return under exit, cancel, and crash recovery. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes each session through support-safe enums, counts, refs, and guardrail booleans. Note bodies, step titles, scenario copy, and raw file paths are excluded by construction. |

## What the corpus proves

Across the six sessions and the restore outcomes the fixtures prove the
exit-gate contract:

- **A thin layer, not a parallel product.** Every waypoint targets an existing
  `editor`, `diff`, `docs`, `graph`, or `notebook` surface
  (`reuses_existing_surface = true`, `creates_parallel_artifact = false`); no
  session mints a new artifact type.
- **Provenance survives the overlay.** Each waypoint preserves its file path,
  symbol anchor, branch/workspace context, and local/remote/shared boundary
  label, and the overlay's provenance strip keeps source identity visible.
- **Explicit follow / breakaway / return.** Leader/follow, breakaway, and
  return states are recorded on the session and surfaced through the follow
  chip; a broken-away local user always sees the breakaway banner, and a
  non-broken-away user never does.
- **Notes do not leak.** Speaker notes default to a `local` scope; the one
  `shared` note carries an explicit promotion marker so the share is auditable.
  The support export records only `has_body` and posture, never note bodies.
- **No widened authority.** `grants_mutation_authority`,
  `grants_control_authority`, and `establishes_private_data_ownership` are
  always false; every overlay action is attention-only (non-destructive,
  non-mutating) and reachable by keyboard with an accessible label.
- **Reversible by construction.** Exit, cancel, and crash recovery all replay
  the same checkpoint, so the prior layout, focus, panel visibility, and
  accessibility posture come back exactly and the user is never left in an
  improvised shell.

## Fixture rules

- The fixtures are regenerated only by the headless inspector:

  ```sh
  cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- corpus > fixtures/help/m3/presentation_mode/corpus.json
  cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- sessions > fixtures/help/m3/presentation_mode/sessions.json
  cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- restore-outcomes > fixtures/help/m3/presentation_mode/restore_outcomes.json
  cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- support-export > fixtures/help/m3/presentation_mode/support_export.json
  ```

- The replay test
  [`crates/aureline-shell/tests/presentation_mode_beta_fixtures.rs`](../../../../crates/aureline-shell/tests/presentation_mode_beta_fixtures.rs)
  fails if the JSON drifts from the seeded corpus, if any session widens
  authority or duplicates a surface, if a speaker note leaks or a shared note
  loses its explicit promotion, if any overlay control becomes pointer-only or
  mutating, if a restore stops matching its checkpoint, or if the surface /
  state / scope / restore-trigger coverage shrinks.
