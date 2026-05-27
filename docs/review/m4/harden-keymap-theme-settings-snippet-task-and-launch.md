# Hardened keymap, theme, settings, snippet, task, and launch import

**Scope:** M04-105 — Harden keymap, theme, settings, snippet, task, and launch import with Exact/Translated/Partial/Shimmed/Unsupported reports.

**Status:** Stable review lane — implemented in `crates/aureline-workspace`.

## Goal

Every imported keymap, theme, setting, snippet, task, and launch configuration from VS Code / Code-OSS, JetBrains family, Vim / Neovim, and Emacs must receive an explicit outcome label (exact, translated, partial, shimmed, unsupported). Rollback checkpoints are preserved before destructive apply, and diagnostics are surfaced when mapping fails. The migration center, first-run wizard, CLI inspector, and support export all consume the same structured packet.

## Design principles

1. **Per-artifact-type truth** — Each of the six artifact types has a dedicated [`ArtifactImportHardeningRecord`] with an independent outcome breakdown, so a strong keymap result does not hide a weak launch-config result.
2. **Exact outcome labels** — The five outcome labels (`exact`, `translated`, `partial`, `shimmed`, `unsupported`) are a closed vocabulary. No producer may invent a softer label like "mostly ok."
3. **Rollback checkpoint required** — Destructive apply must capture a rollback checkpoint or explicitly state that none is required. Missing checkpoints block apply.
4. **Diagnostic transparency** — Every unsupported, partial, or shimmed mapping carries a [`ArtifactImportDiagnosticRecord`] with reason class, suggested action, and fallback availability.
5. **Redaction-safe support export** — Raw source profile paths, raw profile bodies, and secret-bearing values are explicitly forbidden from crossing the support boundary.
6. **Separable inspectable truths** — Outcome breakdown, checkpoint state, diagnostic count, manual-review requirement, and command availability are all independent fields. No single "status" column hides the underlying truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_artifact_import_hardening_packet` | Top-level packet consumed by migration center and support exports. |
| `review_artifact_import_hardening_record` | Per-artifact-type record with outcome breakdown and checkpoint ref. |
| `review_artifact_import_diagnostic_record` | Diagnostic for failed or degraded mappings. |
| `review_artifact_import_rollback_checkpoint_record` | Checkpoint state, auto-restore availability, expiration. |
| `review_artifact_import_hardening_command_record` | Command-graph operations (preview, approve, capture, apply, validate, rollback, abort, review diagnostics). |
| `review_artifact_import_hardening_support_export_packet` | Redaction-safe export with per-artifact summaries. |
| `review_artifact_import_hardening_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Artifact types
- `keymap`, `theme`, `settings`, `snippet`, `task`, `launch`

### Source editor ecosystems
- `vs_code_code_oss`, `jetbrains_family`, `vim_neovim`, `emacs`

### Outcome labels
- `exact`, `translated`, `partial`, `shimmed`, `unsupported`

### Checkpoint states
- `none_required`, `captured_ready`, `captured_pending`, `restored`, `expired`, `missing_blocks_apply`

### Command classes
- `preview`, `approve`, `capture_checkpoint`, `apply`, `validate`, `rollback`, `abort`, `review_diagnostics`

### Diagnostic reason classes
- `no_semantic_equivalent`, `ambiguous_mapping`, `secret_material_excluded`, `policy_locked`, `capability_missing`, `version_mismatch`, `corrupted_source`, `partial_schema_match`

### Diagnostic action classes
- `manual_review`, `use_bridge`, `use_native_alternative`, `skip_and_continue`, `rollback_and_repair`, `contact_support`

## Key invariants

- The packet must contain at least one `artifact_record`.
- `consumer_surfaces` must include at least one surface and must include both `support_export` and `audit_lane` for stable qualification.
- `overall_outcome` in an artifact record must match the highest-severity non-zero count in its `outcome_breakdown` (unsupported > partial > shimmed > translated > exact).
- `requires_manual_review = true` implies at least one diagnostic with `outcome_label` of `partial`, `shimmed`, or `unsupported`.
- `raw_source_profile_paths_export_allowed`, `raw_source_profile_bodies_export_allowed`, and `secret_bearing_values_export_allowed` must all be `false`.
- `checkpoint_state = missing_blocks_apply` implies `applied = false` in the inspection record.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-workspace/src/harden_keymap_theme_settings_snippet_task_and_launch/mod.rs` |
| Schema | `schemas/review/harden_keymap_theme_settings_snippet_task_and_launch.schema.json` |
| Fixtures | `fixtures/review/m4/harden-keymap-theme-settings-snippet-task-and-launch/` |
| Tests | `crates/aureline-workspace/tests/harden_keymap_theme_settings_snippet_task_and_launch_alpha.rs` |

## Integration with existing lanes

- Reuses [`ImportOutcomeLabel`] from the `stabilize_migration_wizard_import_fidelity_for_editor_launch_paths` module.
- Consumed by the same migration-center, entry-surface, and support-export surfaces as the M04-104 migration-wizard packet.
- Referenced by post-import validation runners via opaque refs.

## Verification

```bash
cargo test -p aureline-workspace --test harden_keymap_theme_settings_snippet_task_and_launch_alpha
```
