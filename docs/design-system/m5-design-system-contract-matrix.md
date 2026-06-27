# M5 design-system contract matrix

This page is the human entry point for the **design-system contract matrix**:
the frozen object model Aureline ships for foundations/tokens, component
contracts, reference layouts, state-semantic families, demo fixtures, and proof
packets across the claimed M5 surfaces. The product treats those concepts as
first-class governed artifacts with owners and proof — not informal design files
or best-effort implementation notes — and the matrix is the single inventory
later M5 families point at instead of re-describing component, token, layout, or
state behavior in local docs.

The matrix has two coordinated halves: a governed **inventory** of contract
objects, and a claimed-surface **coverage gate** that keeps a surface from
claiming shell parity without checked-in design-system evidence.

## The governed inventory

One row per governed design-system object. Each row names its accountable owner,
its first consumer, the canonical artifact that materializes it, the release
packet that keeps it current, and the proof lane that blocks drift.

| Object kind | What it governs | Canonical artifact schema |
| ----------- | --------------- | ------------------------- |
| `foundation` | Token packages, themes, density, and motion postures | [`m5-foundations.schema.json`][foundations-schema] |
| `component_contract` | Launch-critical component anatomy, states, keyboard/accessibility behavior, token dependencies, and extension guidance | [`m5-component-contract.schema.json`][component-schema] |
| `reference_layout` | Shell slots and placeholder behavior | [`m5-reference-layout.schema.json`][layout-schema] |
| `state_semantic_family` | The canonical state classes and their cue requirements | matrix packet |
| `demo_fixture` | The checked-in component-gallery example a surface renders from | matrix packet |
| `proof_packet` | The visual/a11y/token evidence generated from the contract | matrix packet |

The full human-readable inventory — every object, owner, first consumer,
canonical artifact, and proof lane — is the checked-in
[governance matrix][governance].

## The coverage gate (auto-narrowing)

Each claimed M5 surface maps the contract objects it must point at. The status is
**derived, never asserted**: the builder recomputes it from the surface's
required objects, the published inventory, and its active waivers.

- **green — conformant** — every required object is mapped and its proof is
  current; the surface keeps its Stable claim and the gate is
  `certified_promote`.
- **yellow — retest-pending** — a required object exists but its design-system
  proof has fallen out of its freshness SLO. The surface auto-narrows below
  Stable (gate `auto_narrowed`, effective claim `beta`) but keeps shipping, with
  the stale object named.
- **red — uncovered** — a required object is **unmapped** (not published in the
  inventory) or its proof is missing. Without a waiver the surface is **blocked**
  from Stable promotion (gate `blocked`, effective claim `held`) and named in the
  release packet rather than left invisible.

A blocking gap can be accepted under an **active waiver** scoped to a single
object. The waiver is disclosed with its accountable owner, expiry, and the
reduced claim it accepts; the surface then ships `auto_narrowed` at the waived
claim while its true status stays red. Waivers never re-grant Stable.

## The release gate

The release/public-truth automation reads the packet-level release gate:

- `blocks_stable_promotion` is `true` when **any** surface is blocked, so a
  claimed M5 surface that lacks a mapped contract object can never keep Stable
  green silently.
- A surface that lacks current design-system proof **auto-narrows before Stable
  promotion** rather than implying silent parity.
- The gate names the blocked, auto-narrowed, conformant, and waived surfaces.

## The dashboard

The compact [dashboard][dashboard] is the published green/yellow/red scoreboard.
It names the inventory object counts per kind, the stale objects, the conformant
/ retest-pending / uncovered surfaces, the surfaces that auto-narrowed or are
blocked, the surfaces carrying active waivers, the active waiver ids, and the
exact contract gaps across all surfaces. It carries the same matrix id as the
contract matrix so consumers can resolve the full rows.

## Consumers

Shell, help, onboarding, presentation, the extension SDK, release center, QA,
support exports, and the stable-claim matrix consume this matrix directly rather
than re-describing component/token/layout/state behavior by hand. Extenders read
the [design-system extension guidance][ext-ds] and
[component-contract extension guidance][ext-cc].

## Where the truth lives

- Matrix schema: [`schemas/design-system/m5-design-system-contract-matrix.schema.json`][matrix-schema]
- Dashboard schema: [`schemas/design-system/m5-design-system-dashboard.schema.json`][dashboard-schema]
- Foundations / component-contract / reference-layout schemas: [`schemas/design-system/`][schemas-dir]
- Matrix support export: `artifacts/release/m5-design-system-proof/support_export.json`
- Markdown proof: `artifacts/release/m5-design-system-proof/contract-matrix-proof.md`
- Published dashboard: `artifacts/design-system/m5-design-system-dashboard.json`
- Governance matrix: `artifacts/design-system/m5-design-system-contract-governance.md`
- Component gallery demo fixtures: `fixtures/ui/m5-component-gallery/`
- Drill fixtures: `fixtures/ui/m5-design-system-contract-matrix/`
- Foundation package: [`m5-foundation-package.md`](m5-foundation-package.md)
- Launch-critical component manifests: [`m5-component-manifest.md`](m5-component-manifest.md)

## How to regenerate

The headless emitter is the only mint-from-truth path:

```sh
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- support-export
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- dashboard
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- markdown
cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- validate
```

The inline tests assert the checked-in support export, dashboard, and gallery
fixtures match the seed builder, so any drift fails
`cargo test -p aureline-design-system m5_design_system_contract`.

[foundations-schema]: ../../schemas/design-system/m5-foundations.schema.json
[component-schema]: ../../schemas/design-system/m5-component-contract.schema.json
[layout-schema]: ../../schemas/design-system/m5-reference-layout.schema.json
[matrix-schema]: ../../schemas/design-system/m5-design-system-contract-matrix.schema.json
[dashboard-schema]: ../../schemas/design-system/m5-design-system-dashboard.schema.json
[schemas-dir]: ../../schemas/design-system/
[dashboard]: ../../artifacts/design-system/m5-design-system-dashboard.json
[governance]: ../../artifacts/design-system/m5-design-system-contract-governance.md
[ext-ds]: ../sdk/extension-ui-design-system.md
[ext-cc]: ../sdk/extension-ui-component-contracts.md
