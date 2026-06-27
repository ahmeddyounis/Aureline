# M5 design-system foundation package

The **foundation package** is the versioned, machine-readable content the M5
design system ships and shell code, docs/help, screenshots, and extension
guidance all consume. Where the
[contract matrix](m5-design-system-contract-matrix.md) governs *which* objects
exist and whether each claimed surface maps them, the foundation package carries
the actual *foundations*: semantic tokens, density / motion / contrast rows, and
the controlled component-state family — each versioned so they cannot drift by
surface family.

- Schema: [`schemas/design-system/m5-foundation-package.schema.json`](../../schemas/design-system/m5-foundation-package.schema.json)
- Canonical package: [`fixtures/ui/m5-foundation-package/foundation-package.json`](../../fixtures/ui/m5-foundation-package/foundation-package.json)
- Release packet: [`artifacts/release/m5-design-system-proof/foundation-package-release.json`](../../artifacts/release/m5-design-system-proof/foundation-package-release.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_foundation_package`

The [component-manifest package](m5-component-manifest.md) declares its token
dependencies as references into this package, so the component contracts and the
foundations read from one shared source.

## Shape

A package carries a `package_id`, a semver `package_version`, and one family per
governed foundation kind. Each family has its **own** `family_version` and a list
of entries:

| Family kind | What it carries | Reads from |
| ----------- | --------------- | ---------- |
| `color` | Semantic color token references | design tokens |
| `spacing` | Spacing-scale token references | geometry tokens |
| `typography` | Typography token references | typography tokens |
| `icon` | Icon token references | icon tokens |
| `density` | Density-class tokens | `aureline_ui::density::DensityClass` |
| `motion` | Motion-posture tokens | `aureline_ui::themes::AccessibilityPostureClass` |
| `contrast` | Contrast / theme-class tokens | `aureline_ui::tokens::ThemeClass` |
| `component_state` | Controlled state tokens | `CanonicalStateClass` |

The density, reduced-motion, power-saving, and high-contrast rows resolve from
the `density`, `motion`, and `contrast` families respectively, so they read from
one governed source rather than feature-local wiring. The package validator
rejects a package whose density / motion / contrast / component-state families
fall out of step with that canonical vocabulary, so the rows cannot silently
diverge.

## Entries, support state, and downgrades

Every entry names a stable `value_token` and an explicit `support_state`:

- `supported` — resolves and carries no downgrade.
- `deprecated` — still resolves, on a removal path, and **must** carry a
  `downgrade` to its supported replacement.
- `unsupported` — no longer resolves on the target surfaces and **must** carry a
  `downgrade` to its fallback.

A deprecated or unsupported entry stays published with its downgrade target,
reason message id, and the version it was downgraded at, so unsupported or
deprecated tokens remain inspectable and explicitly downgraded instead of being
silently dropped.

## Lifecycle operations

The package supports three operations, all of which preserve unsupported and
downgraded-state information:

- **Export / import** — `export_safe_json` mints deterministic JSON and
  `from_json` reads it back; the import is revalidated by the caller.
- **Diff** — `diff` produces a `m5_design_system_foundation_package_diff` packet
  naming added, removed, changed, and downgraded entries per family. Removed and
  downgraded entries are retained in the diff with their last support state, never
  dropped. The diff is the basis for review when a package version bumps.
- **Release-packet inclusion** — `release_packet` projects a
  `m5_design_system_foundation_package_release` packet with per-family support
  counts and a `downgraded_entries` block that enumerates every deprecated or
  unsupported entry for the release record.

## Privacy and boundary

Foundation packages are metadata-only truth packets. They carry semantic token
*references* and posture / class *tokens* — never raw color values, credential
bodies, or provider payloads. The validator scans the serialized export for
forbidden boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in package fixtures, the diff fixture, and the release packet, and the
inline tests assert the checked-in artifacts match the seed and validate, so any
drift fails `cargo test -p aureline-design-system m5_foundation_package`.
