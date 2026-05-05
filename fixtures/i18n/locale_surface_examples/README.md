# Locale-surface examples

These YAML files are worked, cross-surface examples for the locale-surface matrix:

- `docs/i18n/locale_surface_matrix.md`
- `artifacts/i18n/locale_surface_matrix.yaml`

They are intentionally **illustrative** (not schema-pinned) and focus on:

- which elements are allowed to localize (human prose),
- which elements must stay machine-stable (ids/keys/anchors/flags),
- how fallback is disclosed, and
- what “parity checks” reviewers should expect to hold.

If you want schema-validated localization fixtures (message catalog entries, locale-pack manifests, fallback-state records), see:

- `fixtures/ux/localization_cases/`

## Files

- `shell_commands_and_palette_localized_label_stable_ids.yaml`
- `settings_help_localized_prose_stable_setting_and_schema_ids.yaml`
- `docs_tour_localized_text_citation_parity.yaml`
- `cli_help_localized_prose_stable_flags_and_json_keys.yaml`
- `extension_ui_locale_overlay_compat_and_host_id_protection.yaml`

