# Migration Rollback Checkpoints, Diff Review, and Retained Diagnostics Fixtures

This directory contains canonical fixtures for the migration rollback diff-review
packet family introduced in M04-106.

## Fixtures

- `settings_import_diff_approved_checkpoint_ready.json` — Happy path where diff is
  approved and a rollback checkpoint is captured before applying a settings import.
- `keymap_import_validation_failed_with_diagnostics.json` — Import failed
  validation; one retained diagnostic is preserved with a fallback path.
- `snippet_import_rolled_back.json` — Flow was rolled back after a captured
  checkpoint was restored.
- `theme_import_aborted.json` — Flow aborted before apply; no checkpoint required.

## Record kind

All fixture envelopes use `record_kind: review_migration_rollback_diff_review_case`.
