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

## Host-rendered primitives for the launch-critical M5 families

Alongside the manifests, this directory holds the versioned **host-primitive
library** — the single host-rendered implementation each launch-critical family
renders through, so the same state, boundary, and review patterns render
equivalently across M5 surfaces instead of as parallel variants. Each primitive
inherits its component manifest's binding (component id, accessibility role,
keyboard chords, foundation token references, and mandatory states) and adds a
render plan per controlled state, the appearance behavior it preserves (density,
motion, contrast, focus, keyboard), and the M5 family surfaces that route through
it — each with a conformance posture, so embedded or extension consumers either
inherit the primitive or declare a reduced posture behind an explicit partial
badge.

- `host-primitive-library.json` — the full library, validated against
  [`m5-host-primitive.schema.json`][primitive-schema].
- `host-primitive-<kind>.json` — one primitive per family, each a single
  primitive extracted from the library so consumers have a stable file per family.

These are minted from the seed builder by
`aureline_design_system_m5_host_primitive`, and the inline tests assert the
checked-in fixtures match the seed, validate, align with the component manifests,
and reference only foundation tokens the foundation package publishes, so any
drift fails `cargo test -p aureline-design-system m5_host_primitive`. See the
[host-primitive doc][primitive-doc] for the full shape.

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- library
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- primitive placeholder_card
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- release-packet
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- audit
```

[primitive-schema]: ../../../schemas/design-system/m5-host-primitive.schema.json
[primitive-doc]: ../../../docs/design-system/m5-host-primitive.md

## Visual / accessibility evidence pack

Alongside the contracts, this directory holds the versioned **evidence pack** —
the reproducible component gallery a shell-quality gate reads instead of a folder
of hand-captured screenshots. For each launch-critical family it renders one
gallery scene per controlled state from the host-primitive library, and captures
each scene under every appearance variant in the *same* pack: normal dark and
light themes, both high-contrast variants, the reduced-motion posture, and two
zoom levels. Each captured variant carries a deterministic `baseline_digest` (the
visual-diff baseline), and each component attaches its owning identity and a
computed freshness so stale evidence auto-narrows that component's claim.

- `evidence-pack.json` — the full pack, validated against
  [`m5-evidence-pack.schema.json`][evidence-schema].
- `evidence-<kind>.json` — one component's evidence per family, extracted from the
  pack so consumers have a stable file per family.

These are minted from the seed builder by
`aureline_design_system_m5_evidence_pack`, and the inline tests assert the
checked-in fixtures match the seed, validate, are rendered from the host-primitive
render plans, and take their owning identity from the component manifests, so any
drift fails `cargo test -p aureline-design-system m5_evidence_pack`. See the
[evidence-pack doc][evidence-doc] for the full shape.

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- pack
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- component placeholder_card
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- release-packet
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- reevaluate 2026-09-14
```

[evidence-schema]: ../../../schemas/design-system/m5-evidence-pack.schema.json
[evidence-doc]: ../../../docs/design-system/m5-evidence-pack.md

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
