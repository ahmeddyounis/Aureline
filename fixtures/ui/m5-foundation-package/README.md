# M5 foundation package fixtures

These fixtures are the canonical, checked-in **foundation package** the M5 design
system ships: the versioned token / density / motion / contrast / component-state
content shell code, docs/help, screenshots, and extension guidance consume. They
are the content behind the `foundation` object in the
[contract matrix][matrix]; the [foundation-package doc][doc] explains the model.

Each fixture is minted from the same seed builder as the in-code package by
`aureline_design_system_m5_foundation_package` and validates against its
[schema][schema]:

- `foundation-package.json` — the canonical package (version `1.0.0`). One family
  per governed foundation kind; three entries are deliberately downgraded (two
  deprecated, one unsupported) so the downgrade-preservation path is exercised.
- `foundation-package-next.json` — the next package (version `1.1.0`), used as the
  diff drill target. Only the color family changes.
- `foundation-package-diff.json` — the deterministic diff of `1.0.0` → `1.1.0`,
  naming the added, removed, changed, and downgraded entries. Removed and
  downgraded entries are retained, not dropped.

The release-packet projection is checked in under
[`artifacts/release/m5-design-system-proof/foundation-package-release.json`][release].

The inline tests assert each checked-in fixture matches the seed builder and
validates, so any drift fails
`cargo test -p aureline-design-system m5_foundation_package`.

## How to regenerate

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- package > fixtures/ui/m5-foundation-package/foundation-package.json
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- package-next > fixtures/ui/m5-foundation-package/foundation-package-next.json
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- diff > fixtures/ui/m5-foundation-package/foundation-package-diff.json
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- release-packet > artifacts/release/m5-design-system-proof/foundation-package-release.json
```

[matrix]: ../../../docs/design-system/m5-design-system-contract-matrix.md
[doc]: ../../../docs/design-system/m5-foundation-package.md
[schema]: ../../../schemas/design-system/m5-foundation-package.schema.json
[release]: ../../../artifacts/release/m5-design-system-proof/foundation-package-release.json
