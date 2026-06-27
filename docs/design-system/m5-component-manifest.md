# M5 design-system component manifests

The **component-manifest package** is the versioned, machine-readable set of
component contracts the launch-critical M5 component families ship. Where the
[contract matrix](m5-design-system-contract-matrix.md) governs *which* objects
exist and the [foundation package](m5-foundation-package.md) ships the *tokens,
density, motion, contrast, and state vocabulary*, the manifest package carries
the durable *component contracts* the M5 depth surfaces reuse — so anatomy, state
behavior, keyboard reachability, and accessibility obligations are explicit
enough for engineering, QA, docs, and extensions to cite instead of reading shell
code or screenshots.

- Schema: [`schemas/design-system/m5-component-manifest.schema.json`](../../schemas/design-system/m5-component-manifest.schema.json)
- Canonical package: [`fixtures/ui/m5-component-gallery/component-manifest-package.json`](../../fixtures/ui/m5-component-gallery/component-manifest-package.json)
- Per-component fixtures: `fixtures/ui/m5-component-gallery/component-manifest-<kind>.json`
- Release packet: [`artifacts/release/m5-design-system-proof/component-manifest-release.json`](../../artifacts/release/m5-design-system-proof/component-manifest-release.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_component_manifest`
- Extension guidance: [`docs/sdk/extension-ui-component-contracts.md`](../sdk/extension-ui-component-contracts.md)

## Component families

The package publishes one manifest per launch-critical family:

| Component kind | What it is | Lifecycle |
| -------------- | ---------- | --------- |
| `placeholder_card` | Empty-but-ready surface that offers a useful next route | stable |
| `state_block` | Block that renders a controlled state with title, detail, and recovery action | stable |
| `review_sheet` | Modal review surface that stages a decision and records its outcome | preview |
| `job_row` | Row in a dense job / activity collection | stable |
| `boundary_bar` | Embedded-surface boundary indicator naming route, trust, and capability | preview |
| `form_control` | Labelled input with validation and submission semantics | stable |
| `dense_collection` | Virtualizable collection primitive (tree / table / log / list) | experimental |

## What a manifest declares

Each manifest is the single, cite-able contract for one family. It records:

- **anatomy** — the named parts and their roles, marking the parts that are
  always present (`required`).
- **states** — the **mandatory** and **optional** controlled-state families the
  component renders. Together they classify every
  [`CanonicalStateClass`](../../crates/aureline-design-system/src/lib.rs)
  (`empty`, `loading`, `pending`, `degraded`, `blocked`, `error`, `completed`),
  so mandatory vs. optional is explicit rather than implied by a screenshot.
- **labels** — the governed label message ids the component announces.
- **commands** — the commands the component offers, each with its label message
  id and default key chord.
- **keyboard model** — the key chords and the actions they trigger.
- **accessibility** — the role, the screen-reader label rule, the focus-order
  rule, and additional notes engineering and QA must honor.
- **token dependencies** — the foundation token references the component renders
  from. These name entries the
  [foundation package](m5-foundation-package.md) actually publishes, so the two
  lanes read from one shared source.
- **extension guidance** — the consumption rules an extension author reads, plus
  a ref to the extension-SDK guidance, so extensions point at this manifest
  instead of copying shell behavior.

## Versioned lifecycle and ownership

Every manifest carries a `lifecycle` block — an `owner_role`, a `lifecycle_state`
(`experimental`, `preview`, `stable`, or `deprecated`), a monotonic
`manifest_version`, and the `introduced_in_package_version` — so design QA,
support exports, and release packets can all point at the same contract revision
and detect drift.

## Release-packet inclusion

`release_packet` projects a `m5_design_system_component_manifest_release` packet
with one lifecycle-and-shape summary per manifest (lifecycle state, manifest
version, anatomy/command/keyboard/token counts, mandatory-state count). The
release center and support exports consume this projection.

## Privacy and boundary

Component manifests are metadata-only truth packets. They carry semantic token
*references* and message *ids* — never raw color values, credential bodies, or
provider payloads. The validator scans the serialized export for forbidden
boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in package fixture, the per-component fixtures, and the release packet,
and the inline tests assert the checked-in artifacts match the seed, validate,
and reference only foundation tokens the foundation package publishes, so any
drift fails `cargo test -p aureline-design-system m5_component_manifest`.
