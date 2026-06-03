# Qualify learning-mode, guided tours, exercise rails, glossary packs, and teaching-session flows — release evidence

Reviewer-facing evidence packet for the M4 learning-surface qualification gate.
Every learning-mode surface, guided tour, glossary pack, exercise rail, progress
snapshot, and teaching/presentation session present on a claimed M4 Stable row
is either fully qualified with current citation/privacy/offline/authority packets
or explicitly narrowed below Stable with a named reason. The qualification is
derived from typed proofs rather than trusted from input claims.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/learning/guided-learning-contracts.schema.json`](../../../schemas/learning/guided-learning-contracts.schema.json)
- Fixture dir: [`/fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/`](../../../fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/)
- Public doc: [`/docs/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md`](../../../docs/m4/qualify-learning-mode-guided-tours-and-teaching-sessions.md)
- Typed source: `aureline_learning::qualify_learning_mode_guided_tours_and_teaching_sessions`
- Headless emitter: `aureline_learning_qualify`
- Test: `cargo test -p aureline-learning`

## The qualification matrix

| Surface | Kind | Verdict | Lifecycle | Narrowing reason |
|---|---|---|---|---|
| Core-terms glossary pack | `glossary_pack` | **qualified_stable** | beta | — |
| Getting-started tour package | `tour_package` | **narrowed_beta** | beta | cached citation anchors (not live-authoritative) |
| Open-and-save exercise rail | `exercise_rail` | **qualified_stable** | beta | — |
| Default individual-learner profile | `learning_mode_profile` | **qualified_stable** | beta | — |
| Individual-learner progress snapshot | `progress_snapshot` | **qualified_stable** | beta | — |
| Standard teaching session | `teaching_session` | **qualified_stable** | beta | — |

**Overall manifest verdict: narrowed_beta** — the tour package's cached citation anchor propagates to the manifest overall verdict; all other surfaces qualify Stable individually.

## What this packet proves

1. **Citation derived, not claimed.** Each record's `citation.all_anchors_live_authoritative`
   is derived from the installed pack revision, not trusted from an input claim.
   The tour package is honest about using a cached revision and is narrowed.

2. **Privacy: local-first, user-owned.** Every record's `privacy.progress_local_by_default`,
   `repo_visible: false`, and `telemetry_grade_read_access: false` are proved.
   No record leaks progress state to the repository or a background service.

3. **Offline: explicit named state, no silent disappearance.** Every record's
   `offline.silent_disappearance_on_offline: false` is proved. Cached-pack
   and local-only profiles show a named disclosure state rather than an empty
   surface.

4. **Explain-vs-apply separated.** Exercise rails and teaching sessions prove
   `explain_apply_class: apply_requires_approval` — any Apply step flows through
   the standard command/preview/approval/rollback path, never a teaching shortcut.
   Glossary packs, tour packages, and profiles are `read_only`.

5. **Learning mode is opt-in and non-blocking.** The learning-mode profile record
   proves `opt_in_only: true` and `blocks_first_useful_work: false`. Pause,
   snooze, reset, skip, and resume are always available.

6. **Teaching session restores prior state on exit.** The teaching session records
   prove full restore across layout, focus/selection, panel visibility,
   accessibility posture, and crash recovery.

7. **Speaker notes are facilitator-only by default.** Teaching sessions carry
   `speaker_note_locality: facilitator_only_local`. Promotion to co-presenter
   visibility requires an explicit user action. Notes are never audience-visible.

8. **Role authority stays participation-only.** Teaching roles carry
   `role_authority_class: participation_only_no_authority_grant` — they describe
   participation and never confer terminal/debug control or broader workspace
   authority.

9. **Follow mode requires an explicit grant.** Every record's
   `scope.follow_mode_requires_explicit_grant: true` is proved.

10. **Accessibility covered.** Every record's `accessibility` object covers
    keyboard reachability, screen-reader narration, reset/skip accessibility,
    offline degradation accessibility, and reduced-motion.

## How to reproduce

```sh
# Print the full qualification manifest.
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- manifest

# Print a human-readable summary.
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- summary

# Validate the seeded corpus.
cargo run -q -p aureline-learning --bin aureline_learning_qualify -- validate

# Run the full test suite.
cargo test -p aureline-learning

# Emit the on-disk fixture.
cargo run -q -p aureline-learning --bin aureline_learning_qualify \
  -- emit-fixture fixtures/ux/m4/qualify-learning-mode-guided-tours-and-teaching-sessions/qualification_manifest.json
```

## Verdict

The qualification gate is in place. The tour package is honestly narrowed to
`narrowed_beta` due to a cached citation anchor; all other surfaces qualify
Stable individually. The overall manifest verdict is `narrowed_beta` and will
advance to `qualified_stable` once the tour package's citation anchors are
updated to the live-authoritative pack revision. The checked-in schema,
fixtures, Rust types, and headless emitter are the canonical truth source;
docs/help, Start Center, support export, and release packets ingest these
records rather than cloning status text.
