# M5 Coverage-Summary-Bar, Coverage-Overlay-Marker, Flaky-State-Badge, Retry-History-Row, Snapshot-Review-Card, Coverage-Import-Merge-Sheet, and Test-Generation-Suggestion-Card Component Matrix

- Packet: `m5-test-intelligence-components:stable:0001`
- Label: `M5 coverage-summary-bar, coverage-overlay-marker, flaky-state-badge, retry-history-row, snapshot-review-card, coverage-import-merge-sheet, and test-generation-suggestion-card component matrix`
- Component families: 7 (7 stable)
- Provenance classes: verified_current_run, imported_ci_artifact, cached_local_result, stale_prior_result, suspected_flaky, reproduced_flaky, stable_again, manually_muted, unknown
- Coverage metric kinds: line_coverage, branch_coverage, function_coverage, statement_coverage, region_coverage, combined_metric
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Component families

- **coverage_summary_bar**: `stable`
  - Owner: Coverage-summary bar owner
  - Scope: One coverage-summary-bar model naming the included run set behind a coverage number — full suite, changed files only, a single shard, a merged multi-shard run, an imported report, or a partial incomplete scope — and which measure it summarizes so a single percentage never hides a shard omission or conflates line, branch, function, statement, region, and combined measures
  - Required labels: identity, state, keyboard_route, provenance_and_freshness, baseline_or_scope_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **coverage_overlay_marker**: `stable`
  - Owner: Coverage-overlay marker owner
  - Scope: One coverage-overlay-marker model naming what a per-line gutter glyph asserts — covered, uncovered, partially covered, branch missed, excluded, or no data — with changed-file emphasis and its provenance so an editor overlay never shows a stale or imported measurement as if it were freshly produced here and a regression on a changed line is never lost
  - Required labels: identity, state, keyboard_route, provenance_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **flaky_state_badge**: `stable`
  - Owner: Flaky-state badge owner
  - Scope: One flaky-state-badge model naming the classification a badge asserts — stable, suspected flaky, reproduced flaky, stable again, manually muted, or unknown — and the classifier confidence behind it so a single intermittent failure is never labelled as confirmed flakiness and a suspicion never presents with the authority of a reproduced verdict
  - Required labels: identity, state, keyboard_route, provenance_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **retry_history_row**: `stable`
  - Owner: Retry-history row owner
  - Scope: One retry-history-row model naming what a single attempt resulted in — passed first try, passed on retry, failed all retries, errored, skipped, or aborted — and how the rerun behind it was scoped so a pass-on-retry is never shown as a clean first-try pass and a widened rerun is never presented as the same selection
  - Required labels: identity, state, keyboard_route, provenance_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **snapshot_review_card**: `stable`
  - Owner: Snapshot-review card owner
  - Scope: One snapshot-review-card model naming which baseline a snapshot or golden compares against — committed, pending new, updated, imported, missing, or ambiguous — and its diff state with a raw or text fallback so a binary-only change is never blind-accepted and an imported baseline never reads as a local accept
  - Required labels: identity, state, keyboard_route, baseline_or_scope_identity, provenance_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **coverage_import_merge_sheet**: `stable`
  - Owner: Coverage-import merge sheet owner
  - Scope: One coverage-import-merge-sheet model naming where a report was drawn from — a local run, an imported CI artifact, a cached local report, a stale prior report, an uploaded report, or an unknown source — and how overlapping reports resolved so a shard omission is never hidden behind a merged total and a stale or imported report never reads as a fresh local run
  - Required labels: identity, state, keyboard_route, provenance_and_freshness, baseline_or_scope_identity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **test_generation_suggestion_card**: `stable`
  - Owner: Test-generation suggestion card owner
  - Scope: One test-generation-suggestion-card model naming what an AI-generated test assumed — a fixture, an inferred assertion, a generated snapshot, a synthesized mock, an assumed dependency, or an unverified behavior — and what it would apply so assertion, fixture, and snapshot changes are never silently bundled into one opaque apply path and a generated test always discloses its assumptions and recovery boundary
  - Required labels: identity, state, keyboard_route, assumption_and_recovery_boundary, provenance_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
