# Presentation Overlay Review

## Scope

This packet publishes the beta presentation-mode contract used by the shell
presenter bar, agenda/waypoint rail, spotlight frame, speaker-notes tray,
audience strip / follow chip, breakaway banner, provenance strip, restore
affordance, and support export. Presentation mode is a thin, reversible layer
over existing editor, diff, docs, graph, and notebook surfaces.

## Published Artifacts

| Artifact | Path |
| --- | --- |
| Presentation session schema | `schemas/help/presentation_session.schema.json` |
| Runtime model | `crates/aureline-shell/src/presentation_mode/` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_presentation_mode.rs` |
| Corpus fixture | `fixtures/help/m3/presentation_mode/corpus.json` |
| Sessions fixture | `fixtures/help/m3/presentation_mode/sessions.json` |
| Restore-outcomes fixture | `fixtures/help/m3/presentation_mode/restore_outcomes.json` |
| Support export fixture | `fixtures/help/m3/presentation_mode/support_export.json` |
| Contract docs | `docs/help/m3/presentation_mode_beta.md` |
| Fixture corpus replay | `crates/aureline-shell/tests/presentation_mode_beta_fixtures.rs` |

## Exit-Gate Evidence

Presentation mode behaves like a thin, reversible layer over existing surfaces
rather than a parallel product with hidden authority, lost context, or
pointer-only controls. The seeded corpus proves each clause:

- **Reuses existing surfaces.** Every waypoint targets an existing `editor`,
  `diff`, `docs`, `graph`, or `notebook` object; `reuses_existing_surface` is
  true and `creates_parallel_artifact` is false for all of them.
- **Preserves provenance.** Each waypoint and the overlay provenance strip keep
  the file path, symbol anchor, branch/workspace context, and local/remote/
  shared boundary label visible.
- **Explicit, attributable follow/breakaway/return.** Leader/follow, breakaway,
  and return states live on the session and the follow chip; the breakaway
  banner appears exactly when, and only when, the local user is broken away.
- **Notes do not leak.** Notes default local-only; the single shared note in the
  corpus carries an explicit promotion marker. The support export records only
  `has_body` and posture, never note bodies.
- **No widened authority.** `grants_mutation_authority`,
  `grants_control_authority`, and `establishes_private_data_ownership` are false
  across the corpus, and every overlay action is non-destructive and
  non-mutating.
- **Keyboard and screen-reader complete.** Every overlay declares
  `keyboard_complete`, `screen_reader_reachable`, and not `pointer_only`, and
  every action carries a command id, key-binding ref, and accessible label.
- **Reversible.** Restore outcomes for exit, cancel, and crash recovery all
  match the captured checkpoint and never leave an improvised shell.

## Coverage

The corpus exercises, and the replay test enforces, full coverage of:

- surface kinds: `editor`, `diff`, `docs`, `graph`, `notebook`;
- leader/follow states: `presenting`, `following_presenter`, `broken_away`,
  `requesting_follow`;
- audience scopes: `solo_rehearsal`, `shared_workspace`, `invited_guests`;
- boundary labels: `local`, `remote`, `shared`;
- speaker-note scopes: `local`, `shared`;
- restore triggers: `exit`, `cancel`, `crash_recovery`.

## Support Export

The support export is metadata-only. It records active session lifecycle,
leader/follow, and audience-scope enums, layout preset, surface kinds, boundary
labels, waypoint and participant counts, local/shared note counts, the guardrail
booleans, and the keyboard / screen-reader posture, plus the privacy-safe
restore outcomes. It omits raw note bodies, step titles, scenario copy, and raw
file paths.

## Out of Scope

This packet does not add conferencing or video infrastructure, classroom
analytics, or a separate presentation document format. Presentation mode remains
a collaboration mode over existing surfaces.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_presentation_mode -- validate
cargo test -p aureline-shell --lib presentation_mode
cargo test -p aureline-shell --test presentation_mode_beta_fixtures
```
