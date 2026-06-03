# Qualify learning-mode, guided tours, exercise rails, glossary packs, and teaching-session flows — milestone note

Milestone-level qualification gate for the M4 stable release. Turns every
learnability surface — learning-mode profiles, guided tours, glossary packs,
exercise rails, progress snapshots, and teaching/presentation sessions — into
explicit product truth by attaching citation proofs, privacy postures,
offline/cached degradation states, explain-vs-apply class, restore proofs, and
role/authority separation to each surface kind. Verdicts are derived from typed
evidence rather than trusted from input claims.

## Surface matrix

| Surface | Kind | Verdict | Narrowing reason |
|---|---|---|---|
| Core-terms glossary pack | `glossary_pack` | **qualified_stable** | — |
| Getting-started tour package | `tour_package` | **narrowed_beta** | Cached citation anchors (not live-authoritative) |
| Open-and-save exercise rail | `exercise_rail` | **qualified_stable** | — |
| Default individual-learner profile | `learning_mode_profile` | **qualified_stable** | — |
| Individual-learner progress snapshot | `progress_snapshot` | **qualified_stable** | — |
| Standard teaching session | `teaching_session` | **qualified_stable** | — |

**Overall manifest verdict: narrowed_beta.** The tour package's cached citation
anchor propagates to the manifest overall verdict via the lattice
`meet()` operation. All other surfaces qualify Stable individually.

## What the qualification proves

- **Citation is derived, not claimed.** Each record's citation proof is derived
  from the installed pack revision; cached revisions narrow the record and are
  disclosed rather than silently accepted as authoritative.

- **Privacy is local-first.** Progress state is local by default, never
  repo-visible, never telemetry-grade. Sharing requires an explicit user
  promotion. This holds for every surface kind including progress snapshots.

- **Offline has a named state, not silence.** No surface disappears silently
  when offline or when the docs pack is absent. Each record declares whether it
  operates in `cached_disclosed` or `local_only_disclosed` mode.

- **Explain and Apply are separated.** Exercise rails and teaching sessions
  carry `apply_requires_approval`: any Apply step flows through the standard
  command/preview/approval/rollback path. Read-only surfaces carry `read_only`.

- **Learning mode is opt-in and non-blocking.** The learning-mode profile record
  proves `opt_in_only: true` and `blocks_first_useful_work: false`. Pause,
  snooze, reset, skip, and resume are always available to the user.

- **Teaching sessions restore prior state on exit.** All restore axes — layout,
  focus/selection, panel visibility, accessibility posture, and crash recovery —
  are proved. Exiting presentation mode always returns the user to the exact
  pre-session state.

- **Speaker notes are facilitator-only.** Default locality is
  `facilitator_only_local`; promotion to co-presenter visibility requires an
  explicit action. Notes are never audience-visible under any share state.

- **Teaching roles stay participation-only.** Roles carry
  `participation_only_no_authority_grant` — they never grant terminal/debug
  control or broader workspace authority.

- **Follow mode requires an explicit grant** for every surface kind.

## How to reproduce

```sh
# Full qualification manifest as JSON.
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- manifest

# Human-readable summary.
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- summary

# Validate the seeded corpus (exits 0 on pass, non-zero on violation).
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- validate

# Full test suite (11 unit tests in the qualification module).
cargo test -p aureline-learning
```

## Sources

- Reviewer artifact: [`artifacts/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md`](../../artifacts/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md)
- Schema: [`schemas/learning/guided-learning-contracts.schema.json`](../../schemas/learning/guided-learning-contracts.schema.json)
- Fixtures: [`fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/`](../../fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/)
- Rust types and verdict logic: `crates/aureline-learning/src/qualify_learning_mode_guided_tours_and_teaching_sessions/mod.rs`
- Headless emitter: `crates/aureline-learning/src/bin/aureline_learning_qualify.rs`
