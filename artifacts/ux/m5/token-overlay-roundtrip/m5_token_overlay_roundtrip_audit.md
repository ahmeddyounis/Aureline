# M5 token-overlay round-trip audit

Generated from the seeded audit in
[`crate::token_overlays`](../../../../crates/aureline-shell/src/token_overlays/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- report-md > \
  artifacts/ux/m5/token-overlay-roundtrip/m5_token_overlay_roundtrip_audit.md
```

- Report id: `shell:m5_token_overlays:portability:v1`
- Source schema ref: `schemas/ux/token-overlay.schema.json`
- Canonical overlay schema: `schemas/design/token_overlay.schema.json`
- Appearance session: `appearance-session:primary`
- Overlays: `7`
- Override entries: `11` (downgraded `2`, inert `1`)
- Resolved tokens: `5`
- Round trip lossless: `true`
- Unsupported entries preserved: `2`
- Blocking findings: `0`
- Status: **clean**
- Generated at: `2026-06-17T00:00:00Z`

## Scope overlays

| Overlay | Scope | Entries | Downgraded | Validation | Structured |
| ------- | ----- | ------: | ---------: | ---------- | ---------- |
| `overlay:theme_package_default` | `theme_package_default` | 3 | 0 | `valid` | `true` |
| `overlay:imported_theme` | `imported_theme` | 1 | 1 | `inert_unresolved` | `true` |
| `overlay:extension_contributed` | `extension_contributed` | 1 | 1 | `valid_with_warnings` | `true` |
| `overlay:user_global` | `user_global` | 2 | 0 | `valid` | `true` |
| `overlay:profile` | `profile` | 2 | 0 | `valid` | `true` |
| `overlay:workspace` | `workspace` | 1 | 0 | `valid` | `true` |
| `overlay:policy_managed` | `policy_managed` | 1 | 0 | `valid` | `true` |

## Override entries

| Entry | Token | Family | Scope | State | Provenance | Portability | Downgrade |
| ----- | ----- | ------ | ----- | ----- | ---------- | ----------- | --------- |
| `entry:theme_package_default:color.accent.primary` | `color.accent.primary` | `color_functional_accent` | `theme_package_default` | `inherited` | `imported_from_theme_package` | `rides_theme_package` | `none` |
| `entry:theme_package_default:color.semantic.danger` | `color.semantic.danger` | `color_state` | `theme_package_default` | `inherited` | `imported_from_theme_package` | `rides_theme_package` | `none` |
| `entry:theme_package_default:typography.role.code` | `typography.role.code` | `typography_role` | `theme_package_default` | `inherited` | `imported_from_theme_package` | `rides_theme_package` | `none` |
| `entry:imported_theme:color.chart.series_9` | `color.chart.series_9` | `color_chart` | `imported_theme` | `unmapped` | `imported_from_theme_package` | `portable_with_downgrade` | `inert_unsupported_token` |
| `entry:extension_contributed:typography.role.code` | `typography.role.code` | `typography_role` | `extension_contributed` | `deprecated` | `contributed_by_extension` | `portable_with_downgrade` | `deprecated_alias_pending_replacement` |
| `entry:user_global:color.accent.primary` | `color.accent.primary` | `color_functional_accent` | `user_global` | `overridden` | `authored_in_product` | `fully_portable` | `none` |
| `entry:user_global:spacing.density.row` | `spacing.density.row` | `spacing` | `user_global` | `overridden` | `authored_in_product` | `fully_portable` | `none` |
| `entry:profile:color.semantic.danger` | `color.semantic.danger` | `color_state` | `profile` | `overridden` | `authored_in_product` | `fully_portable` | `none` |
| `entry:profile:spacing.density.row` | `spacing.density.row` | `spacing` | `profile` | `overridden` | `authored_in_product` | `fully_portable` | `none` |
| `entry:workspace:color.accent.primary` | `color.accent.primary` | `color_functional_accent` | `workspace` | `overridden` | `authored_in_product` | `fully_portable` | `none` |
| `entry:policy_managed:color.semantic.danger` | `color.semantic.danger` | `color_state` | `policy_managed` | `overridden` | `applied_by_policy` | `scope_local_non_portable` | `none` |

## Resolved tokens (winning versus shadowed)

| Token | Winner | State | Shadowed | Why |
| ----- | ------ | ----- | -------- | --- |
| `color.accent.primary` | `workspace` | `overridden` | user_global, theme_package_default | Workspace accent wins over the user-global override and the theme default. |
| `color.chart.series_9` | `imported_theme` | `unmapped` | — | Imported chart slot is unmapped; it stays an inert placeholder and is never applied. |
| `color.semantic.danger` | `policy_managed` | `overridden` | profile, theme_package_default | Managed-policy danger colour caps the profile override and the theme default. |
| `spacing.density.row` | `profile` | `overridden` | user_global | Profile row spacing wins over the user-global override. |
| `typography.role.code` | `extension_contributed` | `deprecated` | theme_package_default | Extension code-role override wins over the theme default but carries a deprecated-alias downgrade. |

## Round-trip stages

| Seq | Channel | Target | In | Out | Preserved | Downgraded | Dropped | Rewritten |
| --: | ------- | ------ | -: | --: | --------: | ---------: | ------: | --------: |
| 0 | `export_bundle` | `full_support` | 6 | 6 | 6 | 0 | 0 | 0 |
| 1 | `import_bundle` | `reduced_target` | 6 | 6 | 4 | 2 | 0 | 0 |
| 2 | `sync_push` | `full_support` | 6 | 6 | 4 | 2 | 0 | 0 |
| 3 | `sync_pull` | `full_support` | 6 | 6 | 4 | 2 | 0 | 0 |

## Round-trip entry traces

| Entry | Token | Disposition | Downgrade | Scope | Survived |
| ----- | ----- | ----------- | --------- | ----- | -------- |
| `entry:extension_contributed:typography.role.code` | `typography.role.code` | `downgraded` | `deprecated_alias_pending_replacement` | `extension_contributed` | `true` |
| `entry:imported_theme:color.chart.series_9` | `color.chart.series_9` | `downgraded` | `inert_unsupported_token` | `imported_theme` | `true` |
| `entry:profile:color.semantic.danger` | `color.semantic.danger` | `preserved` | `none` | `profile` | `true` |
| `entry:profile:spacing.density.row` | `spacing.density.row` | `preserved` | `none` | `profile` | `true` |
| `entry:user_global:color.accent.primary` | `color.accent.primary` | `preserved` | `none` | `user_global` | `true` |
| `entry:user_global:spacing.density.row` | `spacing.density.row` | `preserved` | `none` | `user_global` | `true` |

## Findings summary

| Scope | Count |
| ----- | ----: |
| `entry` | 0 |
| `overlay` | 0 |
| `resolution` | 0 |
| `round_trip` | 0 |
| `total` | 0 |

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_token_overlays -- validate
cargo test -p aureline-shell --test m5_token_overlays_fixtures
python3 tools/ci/m5/token_overlay_check.py
```
