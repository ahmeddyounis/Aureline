# M5 reference-layout fixtures

These fixtures are the canonical, checked-in **reference-layout package** the M5
design system ships: one descriptor per dominant M5 workspace describing how it
occupies the governed shell zones, collapses responsively, degrades when a
dependency is missing, and reopens or resets. Shell code, docs/help, QA, and
extension guidance consume these layouts so pane placement reads from one
governed source. The [reference-layout doc][doc] explains the model.

Each fixture is minted from the same seed builder as the in-code package by
`aureline_design_system_m5_reference_layout` and validates against its
[schema][schema]:

- `reference-layout-package.json` — the canonical package (version `1.0.0`),
  carrying one layout per workspace family.
- `workspace-<kind>.json` — one fixture per workspace
  (`notebook`, `data_grid`, `profiler`, `pipeline`, `docs`, `preview`,
  `incident`, `companion`), each the layout the package publishes for that kind.

The release-packet projection and the shell-slot conformance packet are checked
in under
[`artifacts/release/m5-design-system-proof/reference-layout-release.json`][release]
and
[`artifacts/release/m5-design-system-proof/reference-layout-conformance.json`][conformance].

The inline tests assert each checked-in fixture matches the seed builder and
validates, so any drift fails
`cargo test -p aureline-design-system m5_reference_layout`.

## How to regenerate

```sh
BIN="cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout --"
$BIN package > fixtures/ui/m5-reference-layout/reference-layout-package.json
for k in notebook data_grid profiler pipeline docs preview incident companion; do
  $BIN workspace "$k" > "fixtures/ui/m5-reference-layout/workspace-$k.json"
done
$BIN release-packet > artifacts/release/m5-design-system-proof/reference-layout-release.json
$BIN conformance > artifacts/release/m5-design-system-proof/reference-layout-conformance.json
```

[doc]: ../../../docs/design-system/m5-reference-layout-package.md
[schema]: ../../../schemas/design-system/m5-reference-layout-package.schema.json
[release]: ../../../artifacts/release/m5-design-system-proof/reference-layout-release.json
[conformance]: ../../../artifacts/release/m5-design-system-proof/reference-layout-conformance.json
