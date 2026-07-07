# M5 test-explorer / watch / triage component matrix

Status: frozen (M05-908, batch B107)

This contract freezes Aureline's reusable **test-explorer, watch, and triage
components** so discovery, watch, triage, and suppression state stop drifting across
tree, editor, panel, status, and export consumers. It is the shared component layer
that sits on top of the already-claimed M5 test discovery, session/attempt, watch, and
quarantine objects — it does **not** re-architect discovery, execution scheduling, or
verdict storage.

- Authoritative validator: `crates/aureline-runtime`, module
  `freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`.
- Boundary schema:
  `schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json`.
- Checked proof: `artifacts/release/m5-test-explorer-watch-triage-proof/`
  (`support_export.json`, `matrix.csv`).
- Design report:
  `artifacts/design/m5-test-explorer-watch-triage-component-matrix.md`.
- Narrowed fixtures: `fixtures/ui/m5-test-explorer-watch-triage-components/`.
- Headless emitter:
  `cargo run -q -p aureline-runtime --bin aureline_runtime_test_explorer_watch_triage_component_matrix -- <support-export|report|csv|validate|fixture-...>`.

## Component families

The matrix freezes seven reusable component families:

1. **test-tree-row** — names its test identity class and imported/live result origin.
2. **inline-result-marker** — names its verdict, result freshness, and origin.
3. **session-summary-bar** — names its session outcome and attempt lineage.
4. **watch-mode-banner** — names its watch fidelity and degrade reason.
5. **failure-triage-panel** — names its failure category and triage disposition.
6. **quarantine-review-sheet** — names its quarantine ownership and release impact.
7. **environment-matrix-card** — names its test target class and environment lane.

## Controlled vocabularies

Consumers bind to **one** controlled vocabulary each for: test identity class, result
origin (imported versus live), inline marker verdict, result freshness, session
outcome, attempt lineage, watch fidelity (`live` / `reduced` / `polling` /
`unavailable` / `paused` / `reconnecting`), watch degrade reason, failure category,
triage disposition, quarantine ownership, release impact, test target class, and
environment lane. The frozen `vocabulary_set` is the single source of these tokens.

## Hard invariants

Every governed component row must satisfy these (each is a `const false` flag in the
schema and a `ComponentInvariantViolated` in the validator):

- `masks_identity_or_origin` — never mask the test identity class or whether a result
  was produced live-locally or imported.
- `hides_quarantine_release_impact` — never hide what a mute / quarantine hides from
  release and support surfaces.
- `invents_alternate_state_label` — never invent an alternate label for stale/imported
  results or any other governed state.
- `widens_rerun_scope_silently` — never silently widen rerun scope.

## Acceptance criteria coverage

- **Single controlled vocabulary** for `live`, `reduced`, `polling`, `unavailable`,
  freshness, target class, and quarantine ownership — frozen in `vocabulary_set` and
  enforced by `VocabularySetDrift`.
- **No alternate labels** for stale/imported results, widened rerun scope, or hidden
  quarantine impact — enforced by the four hard invariants above, the
  `no_surface_invents_alternate_state_label` governance flag, and the
  `AlternateStateLabelInvented` / `RerunScopeWidened` /
  `QuarantineReleaseImpactHidden` downgrade triggers.
