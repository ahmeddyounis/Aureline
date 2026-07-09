# M5 guided exercise steps and progress markers

The guided exercise step and the progress marker are two of the six governed learning
components frozen by the
[M5 learning-component matrix](m5_learning_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`GuidedExerciseStepProgressMarkerControlsPacket`](../../crates/aureline-learning/src/implement_guided_exercise_steps_and_progress_markers_with_target_object_success_criteria_hint_reveal_reset_skip_sandbox_or_preview_preference_and_privacy_bounded_resume_export_truth_across_claimed_m5_learnability_lanes/mod.rs),
so a claimed M5 onboarding, guided-exercise, learning-mode, or progress surface can teach a
structured task **without hiding state or creating an irreversible trap**: a learner can always
tell what to act on, what counts as success, and how to recover their progress.

## What the resolvers decide

The module has two derived resolvers so the honesty of each control is computed, never
asserted.

### `resolve_exercise_progress`

Given a step's exercise step state, the resolver derives a **progress class**:

- `not_started` → `pending`
- `active` → `in_progress`
- `passed` / `replayable` → `completed`
- `failed_retryable` → `retryable` (must carry an explicit retry note), never completed
- `sandboxed` → `sandbox_practice` (must carry an explicit sandbox note)

A learner can therefore always tell **what counts as success**; a failed-retryable step can
never read as passed.

### `resolve_progress_standing`

Given a marker's progress state, the resolver derives a **standing**:

- `not_started` → `unstarted`
- `in_progress` → `underway`
- `completed` → `complete`
- `paused` / `reset` → `interrupted` (must carry an explicit interrupted note), never complete
- `offline_local` → `offline_cached` (must carry an explicit offline note)

A paused or reset marker can never read as complete, and progress stays **resumable,
resettable, and exportable** rather than trapped inside a transient banner.

## Target object, success criteria, and sandbox-or-preview

- **Target object** — every step names exactly what to act on: a `command_reference`,
  `file_location`, `surface_location`, or `docs_anchor` target with a resolvable reference and a
  human-readable target-object label.
- **Success criteria** — every step names its observable success criteria, so the learner knows
  what counts as done.
- **Hint / reveal / reset / skip** — every step offers the mandatory `reset_step` action so a
  lesson is never an irreversible trap, plus `show_hint`, `reveal_solution`, `skip_step`,
  `check_success`, and `open_target_object` as appropriate.
- **Sandbox-or-preview** — a step that mutates state must declare a `sandbox_practice` or
  `preview_then_apply` preference, so an educational lesson never mutates live state without the
  same preview / approval model as ordinary work.

## Completed / remaining, resume / reset / export, and privacy

- **Completed / remaining** — every marker names its `completed_units` and `total_units`; a
  marker counts as `complete` only when the two are equal, so progress is never overstated.
- **Resume / reset / export** — every marker offers the mandatory `resume_progress`,
  `reset_progress`, and `export_progress` actions, so progress stays user-owned and recoverable.
- **Privacy-bounded sharing** — progress is user-owned and default-local. A marker may claim to
  share beyond local scope only when its ownership is `user_owned_synced`, `exported_by_choice`,
  or `workspace_shared`, and it must then carry an explicit sharing-disclosure note — so progress
  is never silently shared beyond the supported scope.

## Hard invariants

Every control keeps five bools `false`, and validation flags any that is `true`:

- `masks_privacy_or_offline_state` — cached / offline / local-only state stays visible.
- `hides_success_criteria_or_target_identity` — what to act on and what counts as success stays
  explicit.
- `implies_hidden_apply_or_mutation` — explain and do stay separate; nothing applies without the
  ordinary preview / approval model.
- `invents_alternate_state_label` — no surface invents a second word for a governed state.
- `traps_progress_without_resume_reset_export` — progress always has a reset / resume / export
  route; a lesson is never an irreversible trap.

Progress stays user-owned and default-local; no control widens trust or mutating authority.

## Coverage

The checked-in support export exercises every progress class, every exercise step state, and
every validation mode across the six seeded exercise steps, and every standing, every progress
state, and every ownership class across the six seeded progress markers.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-guided-exercise-step-progress-marker-controls.schema.json`](../../schemas/ui/m5-guided-exercise-step-progress-marker-controls.schema.json)
- Support export: [`artifacts/release/m5-guided-exercise-step-progress-marker-proof/support_export.json`](../../artifacts/release/m5-guided-exercise-step-progress-marker-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-guided-exercise-step-progress-marker-proof/matrix.csv`](../../artifacts/release/m5-guided-exercise-step-progress-marker-proof/matrix.csv)
- Design report: [`artifacts/design/m5-guided-exercise-step-progress-marker.md`](../../artifacts/design/m5-guided-exercise-step-progress-marker.md)
- Scenario fixtures: [`fixtures/ui/m5-guided-exercise-step-progress-marker-controls/`](../../fixtures/ui/m5-guided-exercise-step-progress-marker-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- fixture-guided-exercise-step-retryable
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- fixture-progress-marker-reset
cargo run -q -p aureline-learning --bin aureline_learning_m5_guided_exercise_step_progress_marker_primitive -- validate
```
