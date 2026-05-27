# Finalize migration rollback checkpoints, diff review, and retained diagnostics for failed imports

**Scope:** M04-106 — Finalize migration rollback checkpoints, diff review, and retained diagnostics for failed imports.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Every migration import from VS Code / Code-OSS, JetBrains family, Vim / Neovim, and Emacs must be previewable, checkpointed, and reversible. If validation fails after apply, retained diagnostics preserve reason classes, suggested actions, and fallback posture so the user can recover without losing context.

## Design principles

1. **Diff-first review gate** — No destructive import may execute until the diff is approved and a rollback checkpoint exists.
2. **Explicit rollback checkpoints** — Every import flow captures a checkpoint before apply, or explicitly declares `none_required` for preview-only flows.
3. **Retained diagnostics for failed imports** — Validation-failed flows must retain at least one diagnostic record with a reason class, suggested action, and fallback availability.
4. **Separable inspectable truths** — Flow state, diff review state, checkpoint state, diagnostic count, and fallback availability are all independent fields.
5. **Redaction-safe support export** — Raw URLs, provider payloads, paths, branch names, and patch bodies are explicitly forbidden from crossing the support boundary.
6. **Exact provider-linked behavior** — Browser handoff and provider publish proposals are explicit about source, freshness, actor, target, and return path.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_migration_rollback_diff_review_packet` | Top-level packet consumed by migration center, review workspace, and support exports. |
| `review_migration_rollback_diff_review_record` | Stable identity, operation provenance, source editor, target families, and flow state. |
| `review_migration_diff_review_record` | Diff review gate that blocks apply until approved and checkpointed. |
| `review_migration_rollback_checkpoint_record` | Rollback checkpoint summary before destructive apply. |
| `review_retained_diagnostic_record` | Diagnostics retained for failed imports with reason class, suggested action, and fallback posture. |
| `review_migration_command_record` | Command-graph operations (preview diff, approve, capture checkpoint, apply, rollback, abort, review diagnostics). |
| `review_migration_support_export_packet` | Redaction-safe export with restart snapshot and command refs. |
| `review_migration_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Migration operation kinds
- `settings_import`, `keymap_import`, `snippet_import`, `theme_import`, `task_import`, `launch_config_import`, `workspace_metadata_import`, `extension_manifest_import`

### Flow states
- `preview_pending`, `diff_pending_review`, `diff_review_approved`, `checkpoint_pending`, `checkpoint_captured`, `applying`, `applied`, `validation_failed`, `rolled_back`, `aborted`

### Diff review states
- `pending`, `approved_with_checkpoints`, `rejected`, `requires_manual_review`

### Checkpoint states
- `none_required`, `captured_ready`, `captured_pending`, `restored`, `expired`, `missing_blocks_apply`

### Diagnostic reason classes
- `no_semantic_equivalent`, `ambiguous_mapping`, `secret_material_excluded`, `policy_locked`, `capability_missing`, `version_mismatch`, `corrupted_source`, `partial_schema_match`

### Diagnostic action classes
- `manual_review`, `use_bridge`, `use_native_alternative`, `skip_and_continue`, `rollback_and_repair`, `contact_support`

### Command classes
- `preview_diff`, `approve_diff`, `reject_diff`, `capture_checkpoint`, `restore_checkpoint`, `apply_migration`, `rollback_migration`, `abort_flow`, `review_diagnostics`, `continue_after_resolve`

## Key invariants

- `applied` flow state requires `diff_review_state` to be `approved_with_checkpoints` and `checkpoint_state` to be `captured_ready`, `captured_pending`, or `restored`.
- `validation_failed` flow state requires at least one `retained_diagnostic_record`.
- `rolled_back` flow state requires `checkpoint_state` to be `captured_ready` or `restored`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Consumer surfaces must include both `support_export` and `audit_lane`.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/finalize_migration_rollback_checkpoints_diff_review_and_retained/mod.rs` |
| Schema | `schemas/review/migration_rollback_diff_review.schema.json` |
| Fixtures | `fixtures/review/m4/finalize-migration-rollback-checkpoints-diff-review-and-retained/` |
| Tests | `crates/aureline-review/tests/finalize_migration_rollback_checkpoints_diff_review_and_retained_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- References migration session and importer outcome records via opaque `migration_session_ref`.
- Projects into the same migration center, review workspace, and support-export surfaces as the `stabilize_migration_wizard_import_fidelity_for_editor_launch_paths` module.

## Verification

```bash
cargo test -p aureline-review --test finalize_migration_rollback_checkpoints_diff_review_and_retained_alpha
```
