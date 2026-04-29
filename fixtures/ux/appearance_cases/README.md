# Appearance import and checkpoint fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/appearance_import_and_checkpoint_contract.md`](../../../docs/ux/appearance_import_and_checkpoint_contract.md)
and the schemas:

- [`/schemas/ux/appearance_checkpoint.schema.json`](../../../schemas/ux/appearance_checkpoint.schema.json)
- [`/schemas/ux/theme_import_report.schema.json`](../../../schemas/ux/theme_import_report.schema.json)

Each YAML file is a single record; a `# yaml-language-server:
$schema=...` header pins the editor to the correct boundary schema.

| Fixture | Record kind | Why it is here |
|---|---|---|
| `live_preview_single_checkpoint.yaml` | `appearance_session_record` | Live preview carries one explicit checkpoint and rollback ref while preserving trust, policy-lock, severity, and source-integrity cues. |
| `checkpoint_rollback_after_import_preview.yaml` | `appearance_checkpoint_record` | Import preview can be reverted atomically from one checkpoint. |
| `extension_webview_partial_inheritance.yaml` | `extension_surface_appearance_record` | Extension/webview body declares `inherits`, `partial`, and `does_not_inherit` rows plus theme/contrast support rows. |
| `import_report_unresolved_fallbacks.yaml` | `theme_import_report_record` | Imported theme report surfaces translated slots, unsupported slots, syntax coverage, unresolved mappings, fallback classes, and rollback path. |
| `token_overlay_policy_blocked_honesty.yaml` | `token_overlay_report_record` | Token overlay is blocked because a policy-lock cue would become color-only; unknown tokens round-trip as inert rows. |
