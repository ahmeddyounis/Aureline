# M5 theme-package manifest audit

Generated from the seeded audit in
[`crate::theme_packages`](../../../../crates/aureline-shell/src/theme_packages/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- report-md > \
  artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md
```

- Report id: `shell:m5_theme_packages:audit:v1`
- Source schema ref: `schemas/ux/m5-theme-package-manifest.schema.json`
- Canonical manifest schema: `schemas/ux/theme_package_manifest.schema.json`
- Registered theme packages: `3`
- Registered M5 surfaces: `7`
- High-salience surfaces: `5`
- Marketed surfaces: `6`
- Blocking findings: `0`
- Narrowable marketed surfaces: `0`
- Status: **clean**
- Generated at: `2026-06-17T00:00:00Z`

## Theme packages

| Package | Version | Provenance | Signature | Modes | Densities | Motion | Compatibility |
| ------- | ------- | ---------- | --------- | ----- | --------- | ------ | ------------- |
| `theme-pkg:aureline-default` | aureline-default-1.4.0 | `built_in_with_product` | `not_applicable_built_in` | 4 | 3 | 5 | `exact_build_match` |
| `theme-pkg:aureline-high-contrast` | aureline-high-contrast-1.4.0 | `built_in_with_product` | `not_applicable_built_in` | 4 | 3 | 3 | `exact_build_match` |
| `theme-pkg:partner-dusk` | partner-dusk-2026.04 | `extension_contributed` | `signed_verified` | 2 | 2 | 2 | `compatible_minor_drift` |

## Provenance index

| Package | Provenance | Signature | Compatibility | Evidence |
| ------- | ---------- | --------- | ------------- | -------- |
| `theme-pkg:aureline-default` | `built_in_with_product` | `not_applicable_built_in` | `exact_build_match` | `current` |
| `theme-pkg:aureline-high-contrast` | `built_in_with_product` | `not_applicable_built_in` | `exact_build_match` | `current` |
| `theme-pkg:partner-dusk` | `extension_contributed` | `signed_verified` | `compatible_minor_drift` | `current` |

## Per-package coverage

| Package | Provenance | Surfaces | Marketed |
| ------- | ---------- | -------: | -------: |
| `theme-pkg:aureline-default` | `built_in_with_product` | 6 | 6 |
| `theme-pkg:aureline-high-contrast` | `built_in_with_product` | 0 | 0 |
| `theme-pkg:partner-dusk` | `extension_contributed` | 1 | 0 |

## Findings summary

| Class | Count |
| ----- | ----: |
| `active_package_unknown` | 0 |
| `unsupported_mode_claimed` | 0 |
| `inheritance_gap_hidden` | 0 |
| `provenance_not_disclosed` | 0 |
| `stale_evidence_on_marketed_surface` | 0 |
| `disabled_package_rendering_undisclosed` | 0 |
| `surface_not_on_appearance_session` | 0 |
| `descriptor_missing_appearance_anchor` | 0 |
| `missing_accessibility_note` | 0 |
| `inheritance_posture_mismatch` | 0 |
| `manifest_token_set_incomplete` | 0 |
| `manifest_missing_required_mode` | 0 |
| `manifest_signature_failed_still_registered` | 0 |

## Per-surface bindings

### `surface:companion.handoff` (companion_surface)

- Descriptor revision: `theme-binding-rev:companion.handoff:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `trust_bearing`
- Appearance anchor: `anchor:companion.handoff`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

### `surface:data.result_grid` (result_grid)

- Descriptor revision: `theme-binding-rev:data.result_grid:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `severity_bearing`
- Appearance anchor: `anchor:data.result_grid`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

### `surface:docs.help_pane` (docs_help_pane)

- Descriptor revision: `theme-binding-rev:docs.help_pane:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `informational`
- Appearance anchor: `anchor:docs.help_pane`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

### `surface:extension.themed_panel` (extension_backed_surface)

- Descriptor revision: `theme-binding-rev:extension.themed_panel:1`
- Active package: `theme-pkg:partner-dusk`
- Semantic salience: `lifecycle_bearing`
- Appearance anchor: `anchor:extension.themed_panel`
- Inheritance posture: `partial_inheritance_disclosed`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `no`
- Honored theme modes: `dark_reference, high_contrast_dark`
- Disclosed inheritance gaps: `focus`

Findings: none.

### `surface:notebook.cell_chrome` (notebook)

- Descriptor revision: `theme-binding-rev:notebook.cell_chrome:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `lifecycle_bearing`
- Appearance anchor: `anchor:notebook.cell_chrome`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

### `surface:preview.browser_pane` (preview_browser_pane)

- Descriptor revision: `theme-binding-rev:preview.browser_pane:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `trust_bearing`
- Appearance anchor: `anchor:preview.browser_pane`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

### `surface:profiler.timeline` (profiler_timeline)

- Descriptor revision: `theme-binding-rev:profiler.timeline:1`
- Active package: `theme-pkg:aureline-default`
- Semantic salience: `informational`
- Appearance anchor: `anchor:profiler.timeline`
- Inheritance posture: `fully_inherited`
- Provenance disclosed: `yes`
- Evidence: `current`
- Marketed on desktop rows: `yes`
- Honored theme modes: `dark_reference, light_parity, high_contrast_dark, high_contrast_light`
- Disclosed inheritance gaps: `none`

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- validate
cargo test -p aureline-shell --test m5_theme_package_fixtures
python3 tools/ci/m5/theme_package_manifest_check.py
```
