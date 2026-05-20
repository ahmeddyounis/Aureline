# Teaching/classroom (beta) fixture corpus

Reviewable fixtures for the beta teaching/classroom sessions that live in
[`crates/aureline-shell/src/teaching_session/`](../../../../crates/aureline-shell/src/teaching_session/).

Each JSON file is a literal projection of the seeded `TeachingClassroomCorpus`
produced by the headless inspector
([`crates/aureline-shell/src/bin/aureline_shell_teaching_session.rs`](../../../../crates/aureline-shell/src/bin/aureline_shell_teaching_session.rs)).
The inspector is the only mint-from-truth path for these fixtures, so the
checked-in JSON cannot drift from the Rust types. Every session is a
`teaching_session_record` that conforms to the boundary schema at
[`schemas/help/teaching_session.schema.json`](../../../../schemas/help/teaching_session.schema.json)
and is documented in
[`docs/help/m3/teaching_and_classroom_conformance.md`](../../../../docs/help/m3/teaching_and_classroom_conformance.md).

All records carry the shared contract ref `shell:teaching_classroom_beta:v1` so
shell UI rows, the headless CLI rows, and the support-export rows pivot to the
same `case_id` and `session_id`.

## Index

| Fixture | Coverage |
| --- | --- |
| [`corpus.json`](./corpus.json) | Full corpus: aggregate coverage summary, one session case per scenario (installed full-coverage teaching, cached every-role classroom, offline shared-archive classroom, mirrored preview-only teaching, not-installed blocked teaching), each paired with its role-aware affordance projection, plus the restore outcomes. |
| [`sessions.json`](./sessions.json) | The session case vector embedded in the corpus, broken out for row-level review. |
| [`restore_outcomes.json`](./restore_outcomes.json) | Restore outcomes proving the prior layout, focus, panels, and accessibility posture return under exit, leave, and crash recovery. |
| [`support_export.json`](./support_export.json) | Support-export wrapper that quotes each session through support-safe enums, counts, refs, and guardrail booleans. Segment titles, scenario copy, and raw file paths are excluded by construction. |

## What the corpus proves

Across the five sessions and the restore outcomes the fixtures prove the
exit-gate contract:

- **A thin layer over learning mode, not a parallel product.** Every segment
  reuses a learning-mode object (`tour:aureline.safe-start.command-backed`,
  `step:safe-start.import-profile-preview`, docs nodes, and graph nodes) and
  cites at least one docs/graph/citation reference; every segment is resumable
  across restart and reconnect.
- **Offline/cached/not-installed states stay visible.** Each segment carries an
  explicit `docs_pack_state`; any state other than `installed` carries a
  `docs_pack_disclosure_ref`. Offline content discloses that reconnect is
  required, and not-installed content is blocked behind an explicit install.
- **Roles stay separate from terminal/debug control.** No role grants terminal
  or debug authority; only the moderator drives; observers and scribes never see
  a mutation affordance; every mutation affordance routes through the ordinary
  command path.
- **Non-mutating by default.** Demonstrations default to `explain`, `open_docs`,
  or `preview_only`; the only mutating kind, `mutation_through_fences`, reuses the
  ordinary command id, preview sheet, approval fence, and rollback semantics.
- **Constrained clients join safely.** Limited and low-bandwidth clients join as
  observers or note-takers with no broken or misleading control; every projected
  control is keyboard- and screen-reader reachable.
- **Reversible by construction.** Exit, leave, and crash recovery all replay the
  same checkpoint, so the prior layout, focus, panel visibility, and
  accessibility posture come back exactly and the user is never left in an
  improvised shell.
- **No widened authority.** `grants_mutation_authority`,
  `grants_terminal_or_debug_control`, `grants_broader_authority_than_workspace`,
  `establishes_private_data_ownership`, `creates_hidden_progress_model`, and
  `creates_cohort_or_grading_flow` are always false.

## Fixture rules

- The fixtures are regenerated only by the headless inspector:

  ```sh
  cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- corpus > fixtures/help/m3/teaching_classroom/corpus.json
  cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- sessions > fixtures/help/m3/teaching_classroom/sessions.json
  cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- restore-outcomes > fixtures/help/m3/teaching_classroom/restore_outcomes.json
  cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- support-export > fixtures/help/m3/teaching_classroom/support_export.json
  ```

- The replay test
  [`crates/aureline-shell/tests/teaching_classroom_beta_fixtures.rs`](../../../../crates/aureline-shell/tests/teaching_classroom_beta_fixtures.rs)
  fails if the JSON drifts from the seeded corpus, if any session widens
  authority or forks from learning mode, if a degraded docs-pack state hides its
  disclosure, if a demonstration mutates outside the ordinary fence, if a role or
  constrained client gains a drive/mutation control it must not have, if a
  restore stops matching its checkpoint, or if the role / client / pack-state /
  segment / demonstration / replay / retention / restore-trigger coverage
  shrinks.
