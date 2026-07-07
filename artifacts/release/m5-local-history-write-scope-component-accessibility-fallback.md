# M5 Local-History / Write-Scope Component Accessibility & Auto-Narrowing

- Packet: `m5-local-history-write-scope-component-accessibility-fallback:stable:0001`
- As of: `2026-07-07T00:00:00Z`
- Families: 7 certified across 7 / 7 frozen families
- Status: 2 green / 5 yellow / 0 red

## Rows

- **a11y:checkpoint-group-card** (checkpoint_group_card) — family=checkpoint_group_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=unavailable_checkpoint status=narrowed_disclosed
  - Auto-narrow: restorable_checkpoint → unavailable_checkpoint (dimension=checkpoint_availability, trigger=checkpoint_lineage_unstated) — Checkpoint has expired past retention and can no longer be restored — shown unavailable with its lineage and actor still preserved
- **a11y:history-export-manifest** (history_export_manifest) — family=history_export_manifest keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=reviewable_history effective_claim=reviewable_history status=parity
- **a11y:local-history-row** (local_history_row) — family=local_history_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=metadata_only_history status=narrowed_disclosed
  - Auto-narrow: restorable_checkpoint → metadata_only_history (dimension=capture_fidelity, trigger=capture_fidelity_masked) — Only metadata was captured for this entry — shown metadata-only with actor and timestamp preserved, not a full-body restorable snapshot
- **a11y:restore-preview-card** (restore_preview_card) — family=restore_preview_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=narrowed_restore status=narrowed_disclosed
  - Auto-narrow: restorable_checkpoint → narrowed_restore (dimension=restore_granularity, trigger=restore_granularity_collapsed) — Restore covers only a partial, manually chosen scope — shown narrowed to the selected files, not the whole snapshot
- **a11y:retention-export-card** (retention_export_card) — family=retention_export_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=metadata_only_history status=narrowed_disclosed
  - Auto-narrow: restorable_checkpoint → metadata_only_history (dimension=export_disclosure, trigger=retention_or_redaction_undisclosed) — History export is redaction-limited to metadata only — shown metadata-only with retention disclosed, never the full redacted bodies
- **a11y:write-scope-preview-tree** (write_scope_preview_tree) — family=write_scope_preview_tree keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=stale_scope_history status=narrowed_disclosed
  - Auto-narrow: restorable_checkpoint → stale_scope_history (dimension=scope_freshness, trigger=write_scope_understated) — Write scope has drifted from the working tree — shown stale and held for re-resolution before any multi-file apply commits
- **a11y:restore-granularity-selector** (restore_granularity_selector) — family=restore_granularity_selector keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=restorable_checkpoint effective_claim=restorable_checkpoint status=parity
