# M5 test-intelligence component matrix

Status: frozen (M05-1028, batch B122)

This contract freezes Aureline's reusable **test-intelligence, quality-evidence, and
AI-generated-test review components** so coverage, flake, snapshot, and generated-test
review stop drifting across editor, coverage-report, test-tree, review, CI-summary, and
export consumers. It is the shared component layer that sits on top of the
already-claimed M5 coverage, flaky-classification, snapshot/golden, retry-history, and
test-generation objects — it does **not** re-architect coverage import backends, runner
backends, or CI provider integrations.

- Authoritative validator: `crates/aureline-runtime`, module
  `freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`.
- Boundary schema:
  `schemas/ui/m5-test-intelligence-component-matrix.schema.json`.
- Checked proof: `artifacts/release/m5-test-intelligence-component-proof/`
  (`support_export.json`, `matrix.csv`).
- Design report:
  `artifacts/design/m5-test-intelligence-component-matrix.md`.
- Narrowed fixtures: `fixtures/ui/m5-test-intelligence-components/`.
- Headless emitter:
  `cargo run -q -p aureline-runtime --bin aureline_runtime_test_intelligence_component_matrix -- <support-export|report|csv|validate|fixture-...>`.

## Component families

The matrix freezes seven reusable component families:

1. **coverage-summary-bar** — names its included-run scope and line-versus-branch metric.
2. **coverage-overlay-marker** — names its gutter state, changed-line emphasis, and provenance.
3. **flaky-state-badge** — names its classification and classifier confidence.
4. **retry-history-row** — names its attempt outcome and rerun scope.
5. **snapshot-review-card** — names its baseline identity and diff state with a raw/text fallback.
6. **coverage-import-merge-sheet** — names its source and merge-resolution state.
7. **test-generation-suggestion-card** — names its assumptions and apply scope.

## Controlled vocabularies

Every component binds **one** controlled provenance vocabulary. The frozen tokens are:
`verified_current_run`, `imported_ci_artifact`, `cached_local_result`,
`stale_prior_result`, `suspected_flaky`, `reproduced_flaky`, `stable_again`,
`manually_muted`, and `unknown`. Later implementation rows reuse this vocabulary rather
than minting feature-local coverage, flake, or generated-test chrome and wording.

Each component also carries family-specific controlled vocabularies: coverage scope
class and line/branch metric kind (coverage-summary-bar); overlay state and changed-line
emphasis (coverage-overlay-marker); flaky classification and classifier confidence
(flaky-state-badge); retry attempt outcome and rerun scope (retry-history-row); snapshot
baseline identity and diff state (snapshot-review-card); coverage import source and
merge-resolution state (coverage-import-merge-sheet); and generated-test assumption
class and apply scope (test-generation-suggestion-card).

## Hard invariants

Every component row must satisfy the following invariants (each recorded as a
`const: false` boolean on the row):

- `masks_provenance_or_freshness_class` — a coverage number, overlay glyph, flaky
  verdict, retry outcome, snapshot accept, or generated test never leaves its
  local/imported/cached/stale origin implicit.
- `hides_shard_omission_behind_single_percentage` — a single percentage never hides an
  omitted shard or stale provenance.
- `labels_intermittent_failure_as_confirmed_flaky` — a single intermittent failure is
  never labelled as confirmed flakiness; classifier confidence is always explicit.
- `bundles_generated_changes_into_opaque_apply` — generated assertion, fixture, and
  snapshot changes are never bundled into one opaque apply path.
- `invents_alternate_state_label` — no surface invents an alternate label for a governed
  state.

Raw/text fallback and rerun/open-logs actions stay explicit. Raw log bodies, raw local
paths, raw usernames, tokens, and credentials never cross the export boundary.

## Source contracts

The matrix binds against the canonical checked-in test-evidence objects:
`schemas/testing/coverage_merge_result.schema.json`,
`schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json`,
`schemas/testing/flaky_verdict.schema.json`,
`schemas/testing/test_attempt.schema.json`,
`schemas/testing/snapshot_acceptance_review.schema.json`, and
`schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json`.
