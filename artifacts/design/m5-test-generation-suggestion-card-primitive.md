# M5 Test-Generation-Suggestion-Card Primitive

- Packet: `m5-test-generation-suggestion-card-primitive:stable:0001`
- Label: `M5 test-generation-suggestion-card primitive: controlled trigger sources (uncovered line/branch, failing bug repro, regression-guard gap, missing-assertion gap, manual request), target symbol/file refs, uncovered-path/bug context, generated-test assumption summaries, distinct assertion/helper-fixture/snapshot-golden review classes, controlled apply scopes, distinct assertion-only/fixture-and-assertion/snapshot-included/full-bundle/review-required/apply-blocked suggestion postures, generated file counts, a required review-class separation before any apply-capable action, a required assumption summary for an apply-capable generated card, a required diff-first preview and rollback for every apply-capable proposal, and bounded reveal/run-in-sandbox/open-diff/apply-reviewed-classes/rollback/export actions`
- Review consumers: 5 (5 stable)
- Suggestion postures: assertion_only_suggestion, fixture_and_assertion_suggestion, snapshot_included_suggestion, full_bundle_suggestion, review_required_suggestion, apply_blocked_suggestion
- Review classes: assertion_change, helper_or_fixture_addition, snapshot_or_golden_update
- Trigger sources: uncovered_line, uncovered_branch, failing_bug_repro, regression_guard_gap, missing_assertion_gap, manual_request
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Review consumers

- **Suggestion Review Panel**: `stable`
  - Owner: Suggestion review panel owner
  - Scope: The suggestion review panel renders the shared test-generation suggestion card so an assertion-only proposal for an uncovered line becomes apply-capable only when it names its assertion review class and keeps a diff-first preview and a rollback, and a fixture-and-assertion proposal for a failing bug repro separates its assertion churn from its helper / fixture churn — with its generated-test assumptions summarised — before any apply-capable action is offered
  - Worked suggestions: 2
    - card `suggestion-card:review-panel::uncovered-line` (`uncovered_line`) -> `assertion_only_suggestion` (apply-capable `true`, churn `false`, assumptions `true`, gen `1`)
    - card `suggestion-card:review-panel::bug-repro-fixture` (`failing_bug_repro`) -> `fixture_and_assertion_suggestion` (apply-capable `true`, churn `true`, assumptions `true`, gen `2`)
- **Editor Inline Suggestion**: `stable`
  - Owner: Editor inline suggestion owner
  - Scope: The editor inline-suggestion surface renders the shared test-generation suggestion card so a snapshot-included proposal for an uncovered branch stays apply-capable only when its scope names its snapshot / golden review class alongside its assertion and helper / fixture churn — never applying a snapshot through an assertion-only click — with its generated snapshot assumption summarised and a diff-first preview and rollback preserved
  - Worked suggestions: 1
    - card `suggestion-card:editor::uncovered-branch-snapshot` (`uncovered_branch`) -> `snapshot_included_suggestion` (apply-capable `true`, churn `true`, assumptions `true`, gen `3`)
- **Test-Tree Suggestion**: `stable`
  - Owner: Test-tree suggestion owner
  - Scope: The test-tree suggestion surface renders the shared test-generation suggestion card so a full-bundle proposal for a regression-guard gap that mixes assertion, helper / fixture, and snapshot / golden churn is held to a review-first path — never a one-click apply — so its assumption, fixture, and snapshot churn are separated and reviewed before anything is applied, with a sandbox run and a diff-first preview always offered
  - Worked suggestions: 1
    - card `suggestion-card:test-tree::regression-full-bundle` (`regression_guard_gap`) -> `full_bundle_suggestion` (apply-capable `false`, churn `true`, assumptions `true`, gen `5`)
- **Headless / CLI Suggestion**: `stable`
  - Owner: Headless / CLI suggestion owner
  - Scope: The headless / CLI suggestion surface renders the shared test-generation suggestion card so a review-required proposal for a missing-assertion gap that mixes assertion and snapshot / golden churn from a cached local result is held to a review-first path without a desktop surface, with its mock and dependency assumptions summarised and a diff-first preview and rollback preserved — proving the same grammar works headless
  - Worked suggestions: 1
    - card `suggestion-card:headless::missing-assertion-review` (`missing_assertion_gap`) -> `review_required_suggestion` (apply-capable `false`, churn `true`, assumptions `true`, gen `2`)
- **Suggestion Export**: `stable`
  - Owner: Suggestion export owner
  - Scope: The suggestion export renders the shared test-generation suggestion card so an apply-blocked proposal for a manual request carries no apply-capable action at all — never presenting a settled apply — and reads with the same trigger, assumption, review-class, and apply-scope vocabulary a reviewer sees in the panel and the editor
  - Worked suggestions: 1
    - card `suggestion-card:export::manual-apply-blocked` (`manual_request`) -> `apply_blocked_suggestion` (apply-capable `false`, churn `false`, assumptions `true`, gen `0`)
