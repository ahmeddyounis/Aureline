# M5 Test-Explorer / Watch / Triage Component Surface Certification

- Packet: `m5-test-explorer-watch-triage-component-certification:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-test-explorer-watch-triage-proof/support_export.json`
- Surfaces: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Lineage preserved on every surface: true
- Auto-narrowed surfaces: 4
- Report clean: true

## Surfaces

- **cert:test-explorer-tree** — surface=test_explorer_tree claimed=trusted_live_result certified=trusted_live_result status=green narrowed_axes=0 lineage_preserved=true
- **cert:editor-notebook-markers** — surface=editor_notebook_markers claimed=trusted_live_result certified=trusted_live_result status=green narrowed_axes=0 lineage_preserved=true
- **cert:triage-panel** — surface=triage_panel claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0 lineage_preserved=true
- **cert:cli-export** — surface=cli_export claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0 lineage_preserved=true
- **cert:imported-ci-view** — surface=imported_ci_view claimed=trusted_live_result certified=imported_or_stale_result status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:watch-banner** — surface=watch_banner claimed=trusted_live_result certified=reduced_watch_result status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:status-bar-session-summary** — surface=status_bar_session_summary claimed=trusted_live_result certified=widened_selection_result status=yellow narrowed_axes=1 lineage_preserved=true
- **cert:quarantine-review-sheet** — surface=quarantine_review_sheet claimed=trusted_live_result certified=restricted_quarantine_result status=yellow narrowed_axes=1 lineage_preserved=true
