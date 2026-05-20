# Teaching and Classroom Conformance Packet

## Scope

This packet publishes the beta teaching/classroom session contract used by the
shell to run a teaching walkthrough or a seated classroom over existing learning
mode. A teaching session is a thin, reversible layer over guided tours, exercise
packs, glossary cards, and speaker notes — not a parallel collaboration product,
hidden progress model, cohort dashboard, or grading flow.

Support and docs can consume this packet directly: each session names its role,
retained artifacts, replay/archive posture, and restore behavior.

## Published Artifacts

| Artifact | Path |
| --- | --- |
| Teaching session schema | `schemas/help/teaching_session.schema.json` |
| Runtime model | `crates/aureline-shell/src/teaching_session/` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_teaching_session.rs` |
| Corpus fixture | `fixtures/help/m3/teaching_classroom/corpus.json` |
| Sessions fixture | `fixtures/help/m3/teaching_classroom/sessions.json` |
| Restore-outcomes fixture | `fixtures/help/m3/teaching_classroom/restore_outcomes.json` |
| Support export fixture | `fixtures/help/m3/teaching_classroom/support_export.json` |
| Contract docs | `docs/help/m3/teaching_and_classroom_conformance.md` |
| Offline/cached audit | `artifacts/help/m3/offline_cached_learnability_audit.md` |
| Fixture corpus replay | `crates/aureline-shell/tests/teaching_classroom_beta_fixtures.rs` |

## Seeded Sessions

Each row is a support-readable summary of one seeded session. Replay and
retention default to the most private, reversible posture; a shared scope is only
reached through an explicit opt-in.

| Case | Kind | Local role | Docs-pack states | Replay | Retention | Restore |
| --- | --- | --- | --- | --- | --- | --- |
| `case:teaching-installed-full-coverage` | teaching | moderator | installed | ephemeral | discard on exit | exit |
| `case:classroom-cached-every-role` | classroom | moderator (+ participant, observer, scribe, approver) | cached | local replay, user-owned | local, user-owned | leave |
| `case:classroom-offline-shared-archive` | classroom | moderator (+ low-bandwidth participant) | offline | shared archive (opt-in) | shared workspace (opt-in) | crash recovery |
| `case:teaching-mirrored-preview-only` | teaching | moderator (+ participant) | mirrored | local replay, user-owned | local, user-owned | exit |
| `case:teaching-not-installed-blocked` | teaching | moderator (+ external observer guest) | installed, not-installed | ephemeral | discard on exit | leave |

## Exit-Gate Evidence

Claimed beta presentation/classroom rows are current, cited, role-aware,
reversible, non-mutating by default, and honest about offline/cached behavior.
The seeded corpus proves each clause:

- **Cited and reused from learning mode.** Every segment reuses a learning-mode
  object (tour, exercise step, docs node, graph node) and carries at least one
  docs/graph/citation reference; `reuses_learning_mode_objects_only` and
  `preserves_source_citations` are true across the corpus, and every segment is
  resumable across restart and reconnect.
- **Offline/cached/not-installed states stay visible.** Each segment carries an
  explicit docs-pack state; every state other than `installed` carries a
  disclosure ref. Offline content discloses that reconnect is required;
  not-installed content is blocked behind an explicit install rather than faked.
- **Roles stay separate from terminal/debug control.** No role grants terminal
  or debug authority and none implies broader authority than the workspace. Only
  the moderator drives; observers and scribes never see a mutation affordance;
  every mutation affordance routes through the ordinary command path.
- **Non-mutating by default.** Demonstrations default to `explain`, `open_docs`,
  or `preview_only`. The single mutating kind, `mutation_through_fences`, reuses
  the ordinary command id, preview sheet, approval fence, rollback semantics, and
  evidence rule.
- **Limited and low-bandwidth clients join safely.** Constrained clients are
  never handed a drive or mutation control; they join as observers or
  note-takers with no broken or misleading affordance. Every projected control is
  keyboard- and screen-reader reachable.
- **Reversible.** Restore outcomes for exit, leave, and crash recovery all match
  the captured checkpoint (layout, focus, panel visibility, accessibility
  posture) and never leave an improvised shell.
- **No hidden authority or analytics.** `grants_mutation_authority`,
  `grants_terminal_or_debug_control`, `grants_broader_authority_than_workspace`,
  `establishes_private_data_ownership`, `creates_hidden_progress_model`, and
  `creates_cohort_or_grading_flow` are false across the corpus.

## Coverage

The corpus exercises, and the replay test enforces, full coverage of:

- session kinds: `teaching`, `classroom`;
- roles: `moderator`, `participant`, `observer`, `approver`, `scribe`;
- client classes: `full`, `limited`, `low_bandwidth`;
- docs-pack states: `installed`, `cached`, `mirrored`, `offline`,
  `not_installed`;
- segment kinds: `tour`, `exercise_pack`, `glossary_card`, `speaker_note`;
- demonstration kinds: `explain`, `open_docs`, `preview_only`,
  `mutation_through_fences`;
- replay policies: `ephemeral`, `local_replay_user_owned`, `shared_archive_opt_in`;
- retention classes: `discard_on_exit`, `local_user_owned`,
  `shared_workspace_retained`;
- restore triggers: `exit`, `leave`, `crash_recovery`.

## Support Export

The support export is metadata-only. It records the session-kind, lifecycle,
role, replay-policy, and retention-class enums, segment kinds, docs-pack states,
demonstration kinds, segment/participant counts, role and client-class sets, the
constrained-client safety and keyboard/screen-reader posture, and the guardrail
booleans, plus the privacy-safe restore outcomes. It omits segment titles,
scenario copy, and raw file paths.

## Out of Scope

This packet does not add later-stage analytics, assessment systems, cohort
dashboards, grading flows, or enterprise training-management integrations.
Teaching/classroom behavior remains optional, cited, and reversible over existing
surfaces.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- validate
cargo test -p aureline-shell --lib teaching_session
cargo test -p aureline-shell --test teaching_classroom_beta_fixtures
```
