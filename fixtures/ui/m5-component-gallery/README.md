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
