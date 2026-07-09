# M5 Learning-Component Accessibility & Auto-Narrowing (M05-1010)

This lane is the accessibility / keyboard / screen-reader / localization / export parity and
honest auto-narrowing capstone over the frozen M5 learning-component matrix
(`freeze_the_m5_learning_mode_toggle_...`). Where the freeze matrix defines the reusable
learning-mode toggle, tip card, guided exercise step, glossary chip / card, safe explanation
banner, and progress marker primitives — and the 1005–1009 implementation / consumer lanes resolve
their per-surface truth — this lane certifies, per component family, that learning claims stay
**keyboard-complete, assistive-tech-reachable, localization/export-safe, and self-narrowing**.

## What it guarantees

- **Keyboard / screen-reader / localization reach.** Every family exposes a keyboard-complete,
  screen-reader-reachable, and localized (source-language-fallback-preserving) path into the same
  command binding, learning-mode state, exercise step / success-criteria, glossary citation,
  explain-versus-do boundary, and progress ownership the rich component shows — never a hover-only
  chip. The hierarchy-heavy guided-exercise step (nested lesson / step / sub-step / hint /
  success-criteria lineage) additionally binds its tree to a flat list / textual path.
- **Export parity.** The support / release / evaluation export reconstructs each component's
  meaning from typed tokens and opaque refs without a screenshot, preserving stable command IDs,
  learning-mode states, exercise success-criteria, cited glossary sources, explain-versus-do
  boundaries, progress ownership, and narrowing reasons — so support, docs, and release proof can
  reconstruct exactly what the user was actually taught.
- **Honest auto-narrowing.** When a learning mode is paused, a tip is snoozed, an exercise pack's
  freshness drifted, a glossary citation is stale, an explain-versus-do boundary cannot be proven,
  or progress portability is blocked, the component's learning claim auto-narrows from
  `exact_learning` / `reviewable_guidance` to a paused-mode / snoozed-tip / stale-pack /
  uncited-glossary / unprovable-boundary / blocked-progress projection, discloses the narrowing
  with a precise trigger and binding dimension, and preserves the canonical command-binding /
  citation / progress-ownership lineage. A stale / uncited / unprovable / blocked state can never
  keep an exact learning claim.
- **Cross-surface disclosure.** The same narrowed state surfaces in the onboarding, tour-overlay,
  learning-panel, glossary, exercise, help-panel, CLI-help, product-UI, and support/release
  surfaces so product, docs, and release publication stay aligned on downgrade behavior.

## Model

- **Learning claim tiers** (strongest first): `exact_learning`, `reviewable_guidance`,
  `paused_mode_projection`, `snoozed_tip_projection`, `stale_pack_projection`,
  `uncited_glossary_projection`, `unprovable_boundary_projection`, `blocked_progress_projection`.
- **Claim dimensions** (1:1 with the six families): `learning_mode_delivery`, `tip_delivery`,
  `exercise_pack_freshness`, `citation_freshness`, `explain_do_boundary`, `progress_portability`.
- **Condition states**: `live_exact_learning` (baseline) plus the two delivery states
  `learning_mode_paused` and `tip_snoozed`, and the four spec "cannot-be-proven" narrowing axes
  `exercise_pack_stale`, `citation_stale`, `explain_do_unprovable`, and
  `progress_portability_blocked`.

Each condition state maps 1:1 to a permitted claim ceiling and names the on-topic frozen downgrade
trigger (`learning_mode_state_unstated`, `tip_command_binding_unstated`,
`exercise_step_state_unstated`, `glossary_citation_severed`,
`explanation_apply_boundary_unstated`, `progress_ownership_unstated`) so certified reasons stay
byte-identical to the freeze matrix. Only the four cannot-be-proven states can never keep an exact
learning claim; a paused mode and a snoozed tip are delivery states, not exactness overstatements.

## Artifacts

- Schema: `schemas/ui/m5-learning-component-accessibility-fallback.schema.json`
- Support export (canonical): `artifacts/release/m5-learning-component-accessibility-fallback/support_export.json`
- Matrix CSV: `artifacts/release/m5-learning-component-accessibility-fallback/matrix.csv`
- Report: `artifacts/release/m5-learning-component-accessibility-fallback.md`
- Fixtures: `fixtures/ui/m5-learning-component-accessibility-fallback/`

Regenerate the checked-in artifacts with:

```
GEN_LEARNING_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-learning generate_artifacts
```
