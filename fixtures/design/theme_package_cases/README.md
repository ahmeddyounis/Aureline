# Theme-package appearance manifest fixtures

Worked fixtures for the theme-package appearance manifest and supported-mode
matrix contract frozen in
[`/docs/design/theme_package_manifest_contract.md`](../../../docs/design/theme_package_manifest_contract.md).

Each YAML file conforms to:

- [`/schemas/design/theme_package_manifest.schema.json`](../../../schemas/design/theme_package_manifest.schema.json)
  — `theme_package_appearance_manifest_record`.

These fixtures exist so theme packages become inspectable, versioned artifacts
instead of screenshot-level promises, and so missing supported modes or missing
token-set coverage stay visible in QA, docs, and support exports.

## Fixtures

- [`first_party_default_theme_manifest.yaml`](./first_party_default_theme_manifest.yaml)
  — Built-in first-party package with all four supported modes, full density
  set, and full motion posture set.
- [`imported_midnight_horizon_manifest.yaml`](./imported_midnight_horizon_manifest.yaml)
  — Imported-translated package that declares only three supported modes, two
  density classes, and the standard/reduced motion postures.
- [`extension_neon_glow_manifest.yaml`](./extension_neon_glow_manifest.yaml)
  — Extension-contributed package that declares only dark + light modes and
  narrows density/motion coverage explicitly.

