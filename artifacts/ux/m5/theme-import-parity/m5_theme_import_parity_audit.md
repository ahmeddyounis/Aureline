# M5 theme-package, appearance-session, token-overlay, and import-parity audit

Canonical rendered form of the frozen M5 appearance-object matrix and the
theme-import-parity qualification audit. It is the human-readable projection of
[`/fixtures/ux/m5/theme-package-interop/report.json`](../../../../fixtures/ux/m5/theme-package-interop/report.json),
so the live shell appearance inspector, the support-export wrapper, the
cross-surface hardening matrix, the release-center packets, and the CI gate at
`tools/ci/m5/theme_import_parity_check.py` never disagree on what each M5 surface
certifies for imported themes, token overlays, and extension inheritance.

- Report id: `shell:m5_theme_import_parity:audit:v1`
- Source schema ref: `schemas/ux/m5-theme-import-parity.schema.json`
- Registered M5 surfaces: `6`
- High-salience surfaces: `3`
- Marketed surfaces: `6`
- Parity rows checked: `30`
- Blocking findings: `0`
- Narrowable marketed rows: `0`
- Status: **clean**
- Generated at: `2026-06-12T00:00:00Z`

## Frozen object-model index

Every M5 surface inherits one canonical representation of each appearance object;
the matrix indexes the already-frozen schemas rather than re-minting them.

| Object family | Canonical schema | Record kind |
| ------------- | ---------------- | ----------- |
| `theme_package` | `schemas/ux/theme_package_manifest.schema.json` | `theme_package_manifest_record` |
| `appearance_session` | `schemas/ux/appearance_checkpoint.schema.json` | `appearance_session_record` |
| `token_overlay` | `schemas/design/token_overlay.schema.json` | `token_overlay_record` |
| `theme_import_report` | `schemas/ux/theme_import_report.schema.json` | `theme_import_report_record` |
| `extension_appearance_descriptor` | `schemas/design/extension_ui_appearance_descriptor.schema.json` | `extension_ui_appearance_descriptor_record` |

## Per-row coverage

| Parity row | Qualified | Narrowed | Not applicable | Declared gap | Hidden downgrade | Missing evidence |
| ---------- | --------: | -------: | -------------: | -----------: | ---------------: | ---------------: |
| Theme-package compatibility | 6 | 0 | 0 | 0 | 0 | 0 |
| Appearance-session integrity | 6 | 0 | 0 | 0 | 0 | 0 |
| Token-overlay validation | 6 | 0 | 0 | 0 | 0 | 0 |
| Imported-theme parity | 4 | 0 | 2 | 0 | 0 | 0 |
| Extension-surface inheritance | 1 | 0 | 4 | 1 | 0 | 0 |

## Findings summary

| Finding class | Count |
| ------------- | ----: |
| Total blocking findings | 0 |
| `hidden_downgrade` | 0 |
| `missing_evidence` | 0 |
| `unresolved_mapping_hidden` | 0 |
| `token_silently_dropped` | 0 |
| `inheritance_gap_hidden` | 0 |
| `rollback_path_missing` | 0 |
| `restart_reload_undisclosed` | 0 |
| `stale_evidence_on_marketed_row` | 0 |
| `protected_cue_color_only` | 0 |
| `parity_claim_without_report` | 0 |
| `object_model_index_drift` | 0 |
| `dimension_drift` | 0 |
| `missing_narrowing_reason` | 0 |
| `missing_projection` | 0 |
| `descriptor_missing_appearance_anchor` | 0 |
| `missing_accessibility_note` | 0 |
| `surface_not_on_appearance_session` | 0 |

## Per-surface rows

### `surface:shell.chrome`

- Surface family: `shell_chrome`
- Descriptor revision: `surface-rev:shell.chrome:2026.06.01-01`
- Semantic salience: `trust_bearing` (high salience: true)
- Appearance anchor: `appearance:anchor:shell:chrome`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `current` | yes | `fresh` |  |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `qualified` | `current` | yes | `fresh` |  |
| Extension-surface inheritance | `not_applicable` | `—` | — | `—` | Shell chrome is first-party; extension-surface inheritance does not apply. |

Findings: none.

### `surface:docs_help.pane`

- Surface family: `docs_help_service_health_canvas`
- Descriptor revision: `surface-rev:docs_help.pane:2026.06.01-01`
- Semantic salience: `informational` (high salience: false)
- Appearance anchor: `appearance:anchor:docs_help:pane`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `restart_or_reload_required` | yes | `fresh` | Help pane reloads to adopt a theme-package swap; the reload is disclosed before it applies. |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `qualified` | `current` | yes | `fresh` |  |
| Extension-surface inheritance | `not_applicable` | `—` | — | `—` | Docs/help pane is first-party; extension-surface inheritance does not apply. |

Findings: none.

### `surface:support_export.canvas`

- Surface family: `support_export_canvas`
- Descriptor revision: `surface-rev:support_export.canvas:2026.06.01-01`
- Semantic salience: `informational` (high salience: false)
- Appearance anchor: `appearance:anchor:support_export:canvas`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `current` | yes | `fresh` |  |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `qualified` | `unsupported_slot` | yes | `fresh` | Imported VS Code theme leaves three slots unsupported; the export discloses each slot and its fallback class. |
| Extension-surface inheritance | `not_applicable` | `—` | — | `—` | Support/export is first-party; extension-surface inheritance does not apply. |

Findings: none.

### `surface:extension.hosted_panel`

- Surface family: `extension_hosted_surface`
- Descriptor revision: `surface-rev:extension.hosted_panel:2026.06.01-01`
- Semantic salience: `lifecycle_bearing` (high salience: true)
- Appearance anchor: `appearance:anchor:extension:hosted_panel`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `current` | yes | `fresh` |  |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `not_applicable` | `—` | — | `—` | Extension-hosted panel renders the host theme; it does not import external theme files. |
| Extension-surface inheritance | `qualified` | `partial_inheritance` | yes | `fresh` | Theme, contrast, and density inherit; reduced-motion inheritance is partial and is disclosed. |

Findings: none.

### `surface:embedded.webview`

- Surface family: `embedded_webview_surface`
- Descriptor revision: `surface-rev:embedded.webview:2026.06.01-01`
- Semantic salience: `trust_bearing` (high salience: true)
- Appearance anchor: `appearance:anchor:embedded:webview`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `current` | yes | `fresh` |  |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `not_applicable` | `—` | — | `—` | Embedded webview renders the host theme; it does not import external theme files. |
| Extension-surface inheritance | `declared_capture_gap` | `—` | — | `—` | Embedded webview cannot inherit the host reduced-motion posture; the gap is declared, not hidden. |

Findings: none.

### `surface:marketplace_account.surface`

- Surface family: `marketplace_account_surface`
- Descriptor revision: `surface-rev:marketplace_account.surface:2026.06.01-01`
- Semantic salience: `informational` (high salience: false)
- Appearance anchor: `appearance:anchor:marketplace_account:surface`

| Parity row | Status | Compatibility | Disclosed | Evidence | Detail |
| ---------- | ------ | ------------- | --------- | -------- | ------ |
| Theme-package compatibility | `qualified` | `current` | yes | `fresh` |  |
| Appearance-session integrity | `qualified` | `current` | yes | `fresh` |  |
| Token-overlay validation | `qualified` | `current` | yes | `fresh` |  |
| Imported-theme parity | `qualified` | `current` | yes | `fresh` |  |
| Extension-surface inheritance | `not_applicable` | `—` | — | `—` | Marketplace/account surface is first-party; extension-surface inheritance does not apply. |

Findings: none.

## Verification

```sh
python3 tools/ci/m5/theme_import_parity_check.py --repo-root .
```
