# Theme-package, icon-slot, illustration-set, and motion-preset fixtures

These fixtures exercise the contract frozen in
[`/docs/ux/theme_and_visual_asset_contract.md`](../../../docs/ux/theme_and_visual_asset_contract.md)
against the three boundary schemas:

- [`/schemas/ux/theme_package_manifest.schema.json`](../../../schemas/ux/theme_package_manifest.schema.json)
- [`/schemas/ux/icon_slot_map.schema.json`](../../../schemas/ux/icon_slot_map.schema.json)
- [`/schemas/ux/motion_preset.schema.json`](../../../schemas/ux/motion_preset.schema.json)

Each YAML file is a single record; a `# yaml-language-server: $schema=...`
header pins the editor to the correct boundary schema.

| Fixture                                                              | Record kind                       | Why it's here                                                                                                                                  |
|----------------------------------------------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------|
| `theme_package_with_deprecated_token_fallback.yaml`                  | `theme_package_manifest_record`   | Acceptance: deprecated-token theme fallback; community pack with substituted_with_fallback row visible to the theme/debug pane.                |
| `theme_package_high_contrast_dark_first_party.yaml`                  | `theme_package_manifest_record`   | Acceptance: high-contrast theme row; built-in pack covering the four themes, the three densities, and the full accessibility-posture set.      |
| `icon_slot_directional_chevron_rtl.yaml`                             | `icon_slot_map_record`            | Acceptance: RTL-aware icon swap; directional chevron pinned to mirror_in_rtl with host-owned override boundary.                                |
| `icon_slot_command_canonical_per_platform.yaml`                      | `icon_slot_map_record`            | Acceptance: platform-variant icon set; command_canonical slot with five distinct platform variants including a universal fallback.             |
| `motion_preset_state_change_with_reduced_fallback.yaml`              | `motion_preset_record`            | Acceptance: reduced-motion preset with equivalent semantic states; idle -> restricted state_change with non_motion_state_marker fallback.      |
| `icon_slot_safety_critical_policy_lock_migration.yaml`               | `icon_slot_map_record`            | Supporting: safety-critical metaphor migration; trust_state policy-lock slot in migrating_with_alias state with a migration decision row.      |
| `theme_package_imported_translated_with_mapping_report.yaml`         | `theme_package_manifest_record`   | Supporting: imported translated theme with mapping report; mixed-state token list including unmapped_inert and unsupported_blocked rows.       |

The fixtures share an `__fixture__` block that names the scenario,
exercised axes, and contract sections; tooling that lints fixture
metadata reads those keys uniformly with the other fixture corpora
under `fixtures/ux/`.
