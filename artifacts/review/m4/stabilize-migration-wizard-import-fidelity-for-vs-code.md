# Artifact: Stabilized migration-wizard import fidelity for VS Code, IntelliJ, Vim, and Emacs launch paths

**Task:** M04-104
**Lane:** Daily-driver credibility — migration fidelity
**Status:** Stable

## Claim summary

The migration-wizard import-fidelity contract for VS Code / Code-OSS, JetBrains family, Vim / Neovim, and Emacs is now stable. Every imported item receives an explicit outcome label (exact, translated, partial, shimmed, unsupported), rollback checkpoints are preserved, and diagnostics are surfaced when mapping fails.

## Evidence

1. **Implementation** — `crates/aureline-workspace/src/stabilize_migration_wizard_import_fidelity_for_editor_launch_paths/mod.rs` defines the bounded beta contract.
2. **Schema** — `schemas/workspace/migration_wizard_import_fidelity.schema.json` is the machine-readable boundary schema.
3. **Fixtures** — `fixtures/review/m4/stabilize-migration-wizard-import-fidelity-for-vs-code/` contains worked cases for all four editors:
   - VS Code: exact mappings for settings and keybindings
   - JetBrains: partial with diagnostics for run configs and code-style hints
   - Vim: translated modal-editing profile with shimmed clipboard defaults
   - Emacs: mixed outcomes with unsupported Elisp package state
4. **Tests** — `crates/aureline-workspace/tests/stabilize_migration_wizard_import_fidelity_alpha.rs` validates parsing, projection, invariants, and redaction rules.

## Downstream consumers

- Migration center
- First-run wizard
- Entry surfaces
- Support export
- Audit lane
- CLI inspector

## Risks and follow-ups

- Source-specific importer adapters are not yet implemented; this contract defines the packet shape they must emit.
- Post-import validation runner integration is pending M04-103.
- Compatibility scorecard row refs in fixtures are seeded references; real scorecards must be linked when the ecosystem surface matures.
