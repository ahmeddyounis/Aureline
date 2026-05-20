# Teaching and Classroom Conformance Beta

This beta contract turns teaching and classroom walkthroughs into one governed
runtime object. A teaching/classroom session is a **thin, reversible layer over
existing learning mode** — guided tours, exercise packs, glossary cards, and
speaker notes — not a parallel collaboration product, a hidden progress model, or
a cohort/grading flow. It guides attention over content the user already has; it
never mints a new artifact type, opens a mutation shortcut, widens authority, or
claims private data ownership.

## Runtime Contract

- A teaching session is a versioned `teaching_session_record` with a session id,
  session kind (`teaching` or `classroom`), lifecycle state, the local user's
  role, a replay policy, a retention class, exercise-pack refs, a segment list, a
  participant list, and a restore checkpoint.
- Each **segment** reuses a learning-mode object. Its `learning_object_ref`,
  `docs_node_refs`, `graph_node_refs`, and `citation_refs` are the same ids
  learning mode ships (for example `tour:aureline.safe-start.command-backed`,
  `step:safe-start.import-profile-preview`,
  `docs-node:help.guided-tours.safe-start`, and
  `graph-node:command.workspace.import_profile`), so teaching content can never
  fork from learning mode. Every segment sets `cites_learning_mode_object = true`
  and is resumable across restart and reconnect
  (`resumable_across_restart = true`, `resumable_across_reconnect = true`, plus a
  stable `resume_ref`).
- Each segment carries an explicit `docs_pack_state`
  (`installed`, `cached`, `mirrored`, `offline`, or `not_installed`). Any state
  other than `installed` requires a `docs_pack_disclosure_ref`, so a guided
  session stays explicit about what is locally available, what is stale, and what
  requires reconnect or install instead of pretending remote enrichment is live.
- A **demonstrated action** is non-mutating by default. `explain`, `open_docs`,
  and `preview_only` never mutate. A `mutation_through_fences` demonstration is
  the only mutating kind, and it must reuse the ordinary command id, preview
  sheet, approval fence, rollback semantics, and evidence-packet rule — never a
  teaching-only shortcut.
- The session never widens authority: `grants_mutation_authority`,
  `grants_terminal_or_debug_control`, `grants_broader_authority_than_workspace`,
  `establishes_private_data_ownership`, `creates_hidden_progress_model`, and
  `creates_cohort_or_grading_flow` are always false; while
  `demonstrations_non_mutating_by_default`, `preserves_source_citations`,
  `reuses_learning_mode_objects_only`, and `restore_on_exit_guaranteed` are always
  true.

## Roles

Teaching roles describe **participation, not control**. No role grants terminal
or debug authority, and no role implies broader authority than the underlying
workspace already permits.

| Role | May drive | Mutation affordance | Note-taking |
|---|---|---|---|
| `moderator` | yes | yes (through fences) | yes |
| `participant` | no | yes (through fences) | yes |
| `approver` | no | approve (through the ordinary fence) | no |
| `scribe` | no | no | yes |
| `observer` | no | no | no |

Only the moderator advances segments or spotlights. Observers and scribes never
see a mutation affordance. Any mutation affordance, regardless of role, routes
through the ordinary command path rather than a teaching shortcut.

## Role-Aware Affordances and Constrained Clients

The session projects a `teaching_affordance_projection_record` describing the
controls each seat actually sees. The projection generates **only** affordances
that are actionable for a given (role, client) pair, so "no broken controls" is
readable from the absence of disabled rows rather than trusted from a flag.

- A client is `full`, `limited`, or `low_bandwidth`. A **constrained** (limited
  or low-bandwidth) client is never handed a drive or mutation control it cannot
  use; heavy live affordances are *omitted* rather than rendered disabled.
- Limited and low-bandwidth clients still join cleanly as observers or
  note-takers: open-cited-docs (read-only) and note-taking (where the role allows
  it) stay available, so a constrained guest is productive without a misleading
  affordance.
- Every projected control is keyboard-reachable and screen-reader reachable: the
  projection declares `keyboard_complete = true`, `pointer_only = false`, and
  `screen_reader_reachable = true`.

## Reversibility

Entering a teaching session checkpoints the prior layout, focus, panel
visibility, and accessibility posture. Exiting, leaving, or recovering from a
crash all replay that same checkpoint, so the prior environment returns exactly
and the user is never left in an improvised shell. `restore_from_checkpoint`
emits a `teaching_restore_outcome_record` with `matches_checkpoint = true` and
`left_in_improvised_shell = false` for each trigger (`exit`, `leave`,
`crash_recovery`).

## Replay and Retention

Replay and retention default to the most private, reversible posture and only
reach a shared scope through an explicit, separately recorded opt-in:

- `replay_policy` is `ephemeral`, `local_replay_user_owned`, or
  `shared_archive_opt_in`. `shared_archive_opt_in` sets
  `replay_archive_opt_in_explicit = true`.
- `retention_class` is `discard_on_exit`, `local_user_owned`, or
  `shared_workspace_retained`. `shared_workspace_retained` sets
  `shared_retention_opt_in_explicit = true`.

## Runtime Consumers

The shell beta model is implemented in
`crates/aureline-shell/src/teaching_session/`.

The headless inspector emits:

- `fixtures/help/m3/teaching_classroom/corpus.json`
- `fixtures/help/m3/teaching_classroom/sessions.json`
- `fixtures/help/m3/teaching_classroom/restore_outcomes.json`
- `fixtures/help/m3/teaching_classroom/support_export.json`

The support export is metadata-only. It records the session-kind, lifecycle,
role, replay-policy, and retention-class enums, segment kinds, docs-pack states,
demonstration kinds, segment/participant counts, role and client-class sets, the
guardrail booleans, and the keyboard / screen-reader posture. It omits segment
titles, scenario copy, and raw file paths.

## Schemas

- `schemas/help/teaching_session.schema.json` defines the teaching session,
  teaching segment, demonstrated action, teaching participant, and restore
  checkpoint records.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_teaching_session -- validate
cargo test -p aureline-shell --lib teaching_session
cargo test -p aureline-shell --test teaching_classroom_beta_fixtures
```
