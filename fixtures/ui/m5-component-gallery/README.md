# M5 component gallery demo fixtures

These fixtures are the canonical, checked-in **demo fixtures** the M5
design-system contract matrix references: the foundations/tokens artifact, the
shell reference-layout artifact, and one launch-critical component-contract
artifact per claimed surface. They are the `demo_fixture` governed object in the
[contract matrix][matrix], and shell, help, presentation, QA, and extension-SDK
guidance render against them.

Each fixture is minted from the same seed builder as the matrix support export by
`aureline_design_system_m5_contract_matrix` and validates against its schema:

- `foundations.json` — [`m5-foundations.schema.json`][foundations-schema]:
  governed token families and the theme / density / motion-posture vocabularies.
- `reference-layout.json` — [`m5-reference-layout.schema.json`][layout-schema]:
  the shell slots and placeholder policy.
- `component-contract-<surface>.json` —
  [`m5-component-contract.schema.json`][component-schema]: the anatomy, states,
  keyboard model, accessibility contract, and token dependencies for a
  launch-critical component (`shell_chrome`, `command_palette`, `trust_prompt`,
  `notification_envelope`).

The inline tests assert each checked-in fixture matches the seed builder and
validates, so any drift fails
`cargo test -p aureline-design-system m5_design_system_contract`.

## Component manifests for the launch-critical M5 families

Alongside the matrix demo fixtures, this directory holds the versioned
**component-manifest package** for the launch-critical M5 component families —
placeholder cards, state blocks, review sheets, job rows, boundary bars, form
controls, and dense collection primitives. Each manifest is the single, cite-able
contract for one family: it declares anatomy, mandatory and optional states,
labels, commands, keyboard model, accessibility contract, foundation token
dependencies, versioned lifecycle/owner metadata, and extension-author
consumption rules.

- `component-manifest-package.json` — the full package, validated against
  [`m5-component-manifest.schema.json`][manifest-schema].
- `component-manifest-<kind>.json` — one manifest per family, each a single
  manifest extracted from the package so consumers have a stable file per family.

These are minted from the seed builder by
`aureline_design_system_m5_component_manifest`, and the inline tests assert the
checked-in fixtures match the seed, validate, and reference only foundation
tokens the foundation package publishes, so any drift fails
`cargo test -p aureline-design-system m5_component_manifest`. See the
[component-manifest doc][manifest-doc] for the full shape.

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- package
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- manifest placeholder_card
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- release-packet
```

[manifest-schema]: ../../../schemas/design-system/m5-component-manifest.schema.json
[manifest-doc]: ../../../docs/design-system/m5-component-manifest.md

## How to regenerate

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-foundations
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-reference-layout
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-component shell_chrome
```

[matrix]: ../../../docs/design-system/m5-design-system-contract-matrix.md
[foundations-schema]: ../../../schemas/design-system/m5-foundations.schema.json
[layout-schema]: ../../../schemas/design-system/m5-reference-layout.schema.json
[component-schema]: ../../../schemas/design-system/m5-component-contract.schema.json
