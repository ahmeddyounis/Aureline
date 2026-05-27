# Fixtures: Hardened keymap, theme, settings, snippet, task, and launch import

These fixtures demonstrate the artifact-import hardening contract for all four supported source editor ecosystems across the six hardened artifact types.

## Fixtures

- `vs_code_all_exact.json` — VS Code / Code-OSS import where all six artifact types map exactly. Zero diagnostics, no manual review required.
- `jetbrains_partial_with_diagnostics.json` — JetBrains family import with partial theme mapping, unsupported snippets, and shimmed launch configs. Diagnostics surface reason classes and suggested actions.
- `vim_translated_shimmed.json` — Vim / Neovim import with translated keymaps, shimmed tasks, and unsupported launch configs. Shows how modal editing profiles and makeprg shims are labeled.
- `emacs_mixed_outcomes.json` — Emacs import with mixed outcomes across all artifact types. Highlights unsupported task and launch configs, partial theme mapping, and translated snippets.

## Invariant checks

Every fixture must:
1. Parse as [`ArtifactImportHardeningPacket`].
2. Validate against the packet invariants (`schema_version`, `record_kind`, non-empty `artifact_records`, non-empty `consumer_surfaces`).
3. Use only the closed vocabularies defined in the schema.
4. Keep all `raw_*_export_allowed` flags `false`.
5. Include `support_export` and `audit_lane` in `consumer_surfaces`.
