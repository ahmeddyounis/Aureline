# Artifact: Hardened keymap, theme, settings, snippet, task, and launch import

**Task:** M04-105
**Lane:** Daily-driver credibility — migration fidelity
**Status:** Stable

## Claim summary

The artifact-import hardening contract for keymap, theme, settings, snippet, task, and launch configurations is now stable. Every imported item receives an explicit outcome label (exact, translated, partial, shimmed, unsupported), rollback checkpoints are preserved, and diagnostics are surfaced when mapping fails.

## Evidence

1. **Implementation** — `crates/aureline-workspace/src/harden_keymap_theme_settings_snippet_task_and_launch/mod.rs` defines the bounded beta contract.
2. **Schema** — `schemas/review/harden_keymap_theme_settings_snippet_task_and_launch.schema.json` is the machine-readable boundary schema.
3. **Fixtures** — `fixtures/review/m4/harden-keymap-theme-settings-snippet-task-and-launch/` contains worked cases for all six artifact types:
   - Keymap: exact mappings for common shortcuts
   - Theme: translated color theme with token-level parity
   - Settings: exact and partial settings with diagnostics
   - Snippet: exact snippet imports with unsupported template gaps
   - Task: translated build tasks with shimmed test runners
   - Launch: partial launch configs with manual review required
4. **Tests** — `crates/aureline-workspace/tests/harden_keymap_theme_settings_snippet_task_and_launch_alpha.rs` validates parsing, projection, invariants, and redaction rules.

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
