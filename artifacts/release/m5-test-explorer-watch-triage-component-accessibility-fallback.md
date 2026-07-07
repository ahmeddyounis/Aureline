# M5 Test-Explorer / Watch / Triage Component Accessibility & Auto-Narrowing

- Packet: `m5-test-explorer-watch-triage-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 7 certified across 7 / 7 frozen families
- Status: 3 green / 4 yellow / 0 red

## Rows

- **a11y:test-tree-row** (test_tree_row) — family=test_tree_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=trusted_live_result status=parity
- **a11y:inline-result-marker-imported** (inline_result_marker) — family=inline_result_marker keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=imported_or_stale_result status=narrowed_disclosed
  - Auto-narrow: trusted_live_result → imported_or_stale_result (dimension=result_evidence, trigger=result_origin_unstated) — Mark is backed by imported CI evidence, not a fresh local run — shown as an imported-or-stale result with its origin and attempt lineage preserved, never as a live-local certainty
- **a11y:session-summary-bar-widened** (session_summary_bar) — family=session_summary_bar keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=widened_selection_result status=narrowed_disclosed
  - Auto-narrow: trusted_live_result → widened_selection_result (dimension=selection_scope, trigger=rerun_scope_widened) — Rerun covered more than the exact selection — shown as a widened-selection result that names the original selection and what the rerun added, never as an exact-selection run
- **a11y:watch-mode-banner-reduced** (watch_mode_banner) — family=watch_mode_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=reduced_watch_result status=narrowed_disclosed
  - Auto-narrow: trusted_live_result → reduced_watch_result (dimension=watch_fidelity, trigger=watch_fidelity_unstated) — Watch fidelity dropped to reduced under resource pressure — shown as a reduced-watch result that names the degrade reason and last successful cycle, never as a live watch
- **a11y:failure-triage-panel-reviewable** (failure_triage_panel) — family=failure_triage_panel keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_result effective_claim=reviewable_result status=parity
- **a11y:quarantine-review-sheet-restricted** (quarantine_review_sheet) — family=quarantine_review_sheet keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=restricted_quarantine_result status=narrowed_disclosed
  - Auto-narrow: trusted_live_result → restricted_quarantine_result (dimension=quarantine_visibility, trigger=quarantine_release_impact_hidden) — Quarantine ownership has expired and its visibility is policy-restricted — shown as a restricted-quarantine result that names the owner, expiry, and hidden release impact, never as a clean release signal
- **a11y:environment-matrix-card** (environment_matrix_card) — family=environment_matrix_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=trusted_live_result effective_claim=trusted_live_result status=parity
