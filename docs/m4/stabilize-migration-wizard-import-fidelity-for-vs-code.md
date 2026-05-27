# Stabilized migration-wizard import fidelity for VS Code, IntelliJ, Vim, and Emacs launch paths

**Scope:** M04-104 — Stabilize migration-wizard import fidelity for VS Code, IntelliJ, Vim, and Emacs launch paths.

**Status:** Stable workspace lane — implemented in `crates/aureline-workspace`.

## Goal

Every migration-wizard import from VS Code / Code-OSS, JetBrains family, Vim / Neovim, and Emacs must produce an exact, translated, partial, shimmed, or unsupported outcome label for each imported item. Rollback checkpoints and diagnostics are preserved when mapping fails, and no destructive apply may proceed without an explicit preview and checkpoint.

## Design principles

1. **Explicit outcome labels** — Every imported item carries one of: `exact`, `translated`, `partial`, `shimmed`, `unsupported`. No silent drop or heuristic parity.
2. **Previewable and checkpointed** — No import apply may execute until the user has reviewed the preview and a rollback checkpoint exists.
3. **Per-editor launch path fidelity** — Each editor ecosystem has its own launch path record with outcome breakdown per target family, diagnostic refs, and checkpoint linkage.
4. **Exact provider-linked behavior** — Browser handoff and provider publish proposals are explicit about source, freshness, actor, target, and return path.
5. **Separable inspectable truths** — Launch path state, checkpoint state, outcome breakdown, diagnostic count, and manual-review requirement are all independent fields.
6. **Redaction-safe support export** — Raw source profile paths, bodies, and secret-bearing values are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `workspace_migration_wizard_import_fidelity_packet` | Top-level packet consumed by migration center, entry surfaces, and support exports. |
| `workspace_migration_wizard_import_fidelity_record` | Stable identity, source ecosystem, import target family, outcome label, and diagnostic refs. |
| `workspace_editor_launch_path_record` | Per-editor launch path fidelity with outcome breakdown and checkpoint linkage. |
| `workspace_import_mapping_diagnostic_record` | Diagnostics when mapping fails, with reason class, suggested action, and fallback posture. |
| `workspace_rollback_checkpoint_record` | Rollback checkpoint before destructive apply. |
| `workspace_migration_wizard_import_fidelity_command_record` | Command-graph operations (preview, approve, capture, apply, validate, rollback, abort, review diagnostics). |
| `workspace_migration_wizard_import_fidelity_support_export_packet` | Redaction-safe export with editor launch path summaries. |
| `workspace_migration_wizard_import_fidelity_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Source editor ecosystems
- `vs_code_code_oss`, `jetbrains_family`, `vim_neovim`, `emacs`

### Import target families
- `settings`, `keybindings`, `snippets`, `tasks`, `launch_configs`, `themes`, `compatible_extensions`, `selected_run_debug_configs`, `project_roots`, `code_style_hints`, `modal_editing_profiles`, `command_aliases`, `clipboard_search_defaults`, `syntax_bundles`, `project_defaults`, `selected_build_task_hints`, `workspace_metadata`

### Outcome labels
- `exact`, `translated`, `partial`, `shimmed`, `unsupported`

### Launch path states
- `preview_pending`, `preview_approved`, `checkpoint_pending`, `checkpoint_captured`, `applied`, `validated`, `rolled_back`, `aborted`

### Checkpoint states
- `none_required`, `captured_ready`, `captured_pending`, `restored`, `expired`, `missing_blocks_apply`

### Command classes
- `preview`, `approve`, `capture_checkpoint`, `apply`, `validate`, `rollback`, `abort`, `review_diagnostics`

## Key invariants

- `applied` launch path state requires `checkpoint_state` to be `captured_ready`, `captured_pending`, or `restored`.
- `validated` launch path state requires `applied` to be true.
- `rolled_back` launch path state requires `checkpoint_state` to be `captured_ready` or `restored`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Consumer surfaces must include both `support_export` and `audit_lane`.
- Every unsupported or partial outcome must have at least one diagnostic record.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-workspace/src/stabilize_migration_wizard_import_fidelity_for_editor_launch_paths/mod.rs` |
| Schema | `schemas/workspace/migration_wizard_import_fidelity.schema.json` |
| Fixtures | `fixtures/review/m4/stabilize-migration-wizard-import-fidelity-for-vs-code/` |
| Tests | `crates/aureline-workspace/tests/stabilize_migration_wizard_import_fidelity_alpha.rs` |

## Integration with existing lanes

- Consumes [`PortableProfileExport`] from the `profiles` module.
- References migration session and importer outcome records via opaque `migration_session_ref`.
- Projects into the same migration center, entry surface, and support-export surfaces as the `entry` and `profiles` modules.

## Verification

```bash
cargo test -p aureline-workspace --test stabilize_migration_wizard_import_fidelity_alpha
```
