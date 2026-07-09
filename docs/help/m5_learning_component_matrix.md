# M5 Learning-Component Matrix

This doc describes the frozen M5 **learning-mode-toggle**, **tip-card**,
**guided-exercise-step**, **glossary-chip-or-card**, **safe-explanation-banner**, and
**progress-marker** component matrix. The matrix is the single source of truth for whether a
claimed M5 onboarding, tour, learning-mode, glossary, or help surface may publish a
learning-mode toggle, a teaching tip, a guided exercise step, a glossary term, a safe
explanation, or a progress marker.

- **Authoritative validator**: `crates/aureline-learning` module
  `freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`.
- **Combined schema (shape only)**: `schemas/ui/m5-learning-component-matrix.schema.json`.
- **Per-component schemas**: `schemas/ui/m5-learning-mode-toggle.schema.json`,
  `schemas/ui/m5-tip-card.schema.json`, `schemas/ui/m5-guided-exercise-step.schema.json`,
  `schemas/ui/m5-glossary-chip-card.schema.json`,
  `schemas/ui/m5-safe-explanation-banner.schema.json`,
  `schemas/ui/m5-progress-marker.schema.json`.
- **Support export (mint-from-truth)**: `artifacts/release/m5-learning-component-proof/support_export.json`.
- **Machine-readable matrix**: `artifacts/release/m5-learning-component-proof/matrix.csv`.
- **Design report**: `artifacts/design/m5-learning-component-matrix.md`.
- **Narrowed fixtures**: `fixtures/ui/m5-learning-components/`.

Regenerate every checked-in artifact from truth with the headless emitter:

```sh
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- support-export
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- csv
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- report
cargo run -q -p aureline-learning --bin aureline_learning_m5_learning_component_matrix -- validate
```

## Component families

| Family | What it names |
| --- | --- |
| `learning_mode_toggle` | Whether learning mode is on/off/paused/per-feature/sandboxed-only/ended and how widely it applies. Learning stays opt-in. |
| `tip_card` | Why a teaching tip appears (trigger), the cited source behind it, and how it can be dismissed. |
| `guided_exercise_step` | A practice step's state and how it validates the learner's work — replayable, sandboxed, and never a hidden apply. |
| `glossary_chip_or_card` | Where a definition comes from and how current its citation is. Never severs the canonical citation. |
| `safe_explanation_banner` | How an explanation separates explain from do and what it will actually apply. Never a hidden apply. |
| `progress_marker` | Who owns a learner's progress and where it stands. User-owned and default-local. |

## Controlled vocabularies

### Shared disposition vocabulary (acceptance criteria)

Every consumer binds one controlled disposition vocabulary so no surface invents a parallel
word for these states:

`learning_on`, `paused`, `replayable`, `sandboxed`, `cached`, `local_only`,
`not_installed`, `no_hidden_apply`.

### Family-specific vocabularies

- **Learning-mode states**: `off`, `on`, `paused`, `per_feature_family`, `sandboxed_only`,
  `ended`.
- **Learning-mode scopes**: `global`, `workspace`, `feature_family`, `session`, `surface`,
  `unavailable`.
- **Tip trigger classes**: `first_encounter`, `feature_discovery`, `error_recovery`,
  `mode_change`, `idle_hint`, `contextual_followup`.
- **Tip dismissal states**: `dismissible`, `dismissed`, `snoozed`, `persistent_until_acted`,
  `auto_expired`, `suppressed_by_preference`.
- **Exercise step states**: `not_started`, `active`, `passed`, `failed_retryable`,
  `replayable`, `sandboxed`.
- **Exercise validation modes**: `command_backed`, `sandboxed_practice`,
  `read_only_walkthrough`, `checkpoint_gated`, `self_paced`, `no_hidden_apply`.
- **Glossary source classes**: `cited_docs`, `cited_spec`, `cited_help_pack`,
  `community_note`, `uncited_draft`, `unknown_source`.
- **Glossary citation states**: `citation_current`, `citation_versioned`, `citation_stale`,
  `citation_cached`, `citation_offline_unavailable`, `citation_missing`.
- **Explanation boundary classes**: `explain_only`, `explain_then_offer_do`,
  `preview_required`, `approval_required`, `sandboxed_only`, `no_hidden_apply`.
- **Explanation apply states**: `no_apply`, `preview_available`, `approval_pending`,
  `applied_with_undo`, `blocked_apply`, `mutation_declined`.
- **Progress ownership classes**: `local_only`, `user_owned_synced`, `exported_by_choice`,
  `workspace_shared`, `cached_snapshot`, `not_installed`.
- **Progress states**: `not_started`, `in_progress`, `completed`, `paused`, `reset`,
  `offline_local`.

## Hard invariants

Every row asserts, and the validator enforces, that a component:

- **never masks its privacy / offline / local-only or cached state**
  (`masks_privacy_or_offline_state = false`),
- **never hides its cited source** (`hides_citation_source = false`),
- **never implies a hidden apply or widened mutating authority**
  (`implies_hidden_apply_or_mutation = false`), and
- **never invents an alternate label for a governed state**
  (`invents_alternate_state_label = false`).

Learnability stays opt-in, citation-backed, command-backed, and privacy-bounded: explain and
do remain separate, progress is user-owned by default, cached/offline/source-class truth
stays visible, and no learning component widens trust or mutating authority.

## Mandatory labels

Every claimed component must be able to show `identity`, `state`, and `keyboard_route`. The
matrix additionally binds `citation_source`, `explain_versus_do_boundary`, and
`progress_ownership_and_privacy` where the family requires them.

## Narrowed fixtures

Two narrowed variants prove a component can be held below Stable without hiding it:

- `fixtures/ui/m5-learning-components/learning_mode_toggle_beta_narrowed.json` — the
  learning-mode toggle held at `beta`.
- `fixtures/ui/m5-learning-components/progress_marker_preview_narrowed.json` — the progress
  marker narrowed to `preview`.
