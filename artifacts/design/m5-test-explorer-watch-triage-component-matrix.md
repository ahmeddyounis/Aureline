# M5 Test-Tree-Row, Inline-Result-Marker, Session-Summary-Bar, Watch-Mode-Banner, Failure-Triage-Panel, Quarantine-Review-Sheet, and Environment-Matrix-Card Component Matrix

- Packet: `m5-test-explorer-watch-triage-components:stable:0001`
- Label: `M5 test-tree-row, inline-result-marker, session-summary-bar, watch-mode-banner, failure-triage-panel, quarantine-review-sheet, and environment-matrix-card component matrix`
- Component families: 7 (7 stable)
- Watch fidelity states: live, reduced, polling, unavailable, paused, reconnecting
- Result origins: live_local, imported_ci, imported_teammate, replayed_snapshot, synthetic_seed, unknown_origin
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Component families

- **test_tree_row**: `stable`
  - Owner: Test-tree row owner
  - Scope: One test-tree-row model naming how a test is identified — a durable keyed identity, a path-derived identity, a discovery-only identity, an imported record, a parametrized case, or an ambiguous identity — and whether its latest result was produced live-locally or imported, so a user never has to guess whether a red mark is local or imported or which durable test a row represents
  - Required labels: identity, state, keyboard_route, origin_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **inline_result_marker**: `stable`
  - Owner: Inline result marker owner
  - Scope: One inline-result-marker model naming the verdict a marker asserts — passed, failed, errored, skipped, flaky-suspected, or not-run — how fresh that result is, and whether it was produced live-locally or imported, so a marker in the editor gutter never shows a stale or imported result as if it were freshly produced here
  - Required labels: identity, state, keyboard_route, origin_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **session_summary_bar**: `stable`
  - Owner: Session-summary bar owner
  - Scope: One session-summary-bar model naming the overall outcome of a run — all passed, some failed, errored, partial discovery, cancelled, or in progress — and how the current attempt relates to prior attempts, so retry lineage and rerun scope are explicit and a partial discovery is never shown as a complete green run
  - Required labels: identity, state, keyboard_route, origin_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **watch_mode_banner**: `stable`
  - Owner: Watch-mode banner owner
  - Scope: One watch-mode-banner model naming how faithfully watch mode is observing — live, reduced, polling, unavailable, paused, or reconnecting — and why fidelity dropped, so a user never assumes results are current when watch has silently degraded and always sees why watch degraded
  - Required labels: identity, state, keyboard_route, watch_fidelity
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **failure_triage_panel**: `stable`
  - Owner: Failure-triage panel owner
  - Scope: One failure-triage-panel model naming what class of failure a test hit — assertion failure, runtime error, timeout, environment error, flaky-under-review, or unknown failure — and where it sits in triage, so a failure is never left uncategorized and its disposition is always explicit
  - Required labels: identity, state, keyboard_route, origin_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **quarantine_review_sheet**: `stable`
  - Owner: Quarantine-review sheet owner
  - Scope: One quarantine-review-sheet model naming who owns a mute or quarantine — unowned, self-owned, team-owned, CI-enforced, imported from policy, or owner-expired — and what it hides from release and support surfaces, so a user always sees what a mute or quarantine will hide from release and never mistakes an unowned quarantine for a governed one
  - Required labels: identity, state, keyboard_route, quarantine_and_release_impact
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **environment_matrix_card**: `stable`
  - Owner: Environment-matrix card owner
  - Scope: One environment-matrix-card model naming what kind of test a card represents — unit, integration, end-to-end, UI snapshot, benchmark, or contract — and where it runs, so the target and environment behind a result are always explicit and a local result is never confused with a remote or CI-matrix result
  - Required labels: identity, state, keyboard_route, origin_and_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
