# M5 Host-Rendered Trust-Component Primitives and Token / State Wiring

Status: implementation contract (M05-762, batch B88)

This contract binds Aureline's M5 trust / config / history component families to
**canonical host-rendered primitives** with shared design-token / state wiring, so
desktop, companion, and extension-backed first consumers cannot each restyle or
relabel them into different meanings. It is the binding layer over the four primitive
implementation lanes that preceded it (settings row, capability sheet, evidence /
activity row, and chronology group / narrative card / export preview) and the frozen
component matrix that governs them.

- Rust module:
  `crates/aureline-shell/src/implement_the_m5_host_rendered_trust_component_primitives_and_token_state_wiring/`
- Emitter bin: `aureline_shell_m5_host_rendered_primitives`
- Boundary schema: `schemas/ui/m5-host-rendered-primitives.schema.json`
- Support export: `artifacts/release/m5-host-rendered-primitives-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-host-rendered-primitives-proof/matrix.csv`
- Report: `artifacts/components/m5-host-rendered-primitives.md`
- Narrowed fixtures: `fixtures/ui/m5-host-rendered-primitives/`

The Rust validator and resolver are the authoritative gate; the schema and this doc
describe the shape.

## Host-rendered primitive families

Five canonical host-rendered primitives, each bound to one or more frozen
`M5TrustComponentFamily`. The union covers every frozen family — the timeline-group
primitive host-renders both the timeline group and its narrative summary card.

| Host-rendered primitive | Frozen families bound | Shell zone |
| --- | --- | --- |
| `settings_row` | settings_row | main_workspace |
| `capability_sheet` | capability_sheet | transient_overlay |
| `event_history_row` | event_history_row | bottom_panel |
| `timeline_group` | timeline_group, narrative_summary_card | bottom_panel |
| `chronology_export_preview` | chronology_export_preview | bottom_panel |

## Host surfaces and render modes

Every primitive renders on all three **host surfaces** — `desktop_app`,
`companion_surface`, and `extension_host` — so its meaning cannot drift between
runtimes.

A first consumer renders through one of three **render modes**:

- `host_rendered_canonical` — rendered directly through the canonical primitive.
- `audited_wrapper` — rendered through an audited wrapper over the canonical
  primitive (an `audited_wrapper_ref` is required). This is the path an extension host
  uses when it cannot render the canonical primitive directly.
- `bespoke_local_variant` — a local re-implementation. **This is the drift this lane
  prevents.** No row permits it, and the resolver classifies any binding that uses it
  as `bespoke_drift`.

## What is fixed vs what may be restyled

This is the contract element extension and contributor authors most need. The
host-rendered layer pins the parts that carry meaning; only cosmetic aspects are open
to restyle.

### Fixed — pinned by the host layer (do NOT override)

**Design-token slots** (`M5DesignTokenSlot`) carry meaning and are wired through the
host layer:

- `source_pill` (settings row only — which source produced the effective value)
- `provenance_badge` (chronology families only — who / what initiated an event)
- `severity_state_color` (never colour-only; always paired with a label)
- `disclosure_affordance` (expand / reveal control)
- `focus_ring`
- `state_label` (the typed current state)
- `density_metric` (compact / comfortable scale)

**Contract parts** (`M5PrimitiveContractPart`) — every primitive pins all seven:
`identity_label`, `typed_state`, `provenance_or_source_attribution`,
`severity_semantics`, `disclosure_control`, `keyboard_route`, `audit_export_anchor`.

### Restylable — cosmetic only (safe to restyle)

`M5RestylableAspect`: `spacing_scale`, `corner_radius`, `accent_tint`,
`typography_family`, `icon_set`, `elevation_shadow`, `motion_curve`. These carry no
meaning, so restyling them never drifts the component's semantics.

## The binding resolver

`resolve_binding(&M5PrimitiveBindingInput) -> Result<M5ResolvedBinding, …>` takes a
first consumer's declared render (primitive family, host surface, render mode,
optional audited-wrapper ref, wired token slots, restyled aspects, and any overridden
contract parts) and produces one conformance verdict:

- `conformant` — renders through the canonical primitive (or an audited wrapper),
  wires every fixed token slot for the family, and overrides no contract part.
- `bespoke_drift` — the render mode is a bespoke local variant. Takes precedence over
  every other fault.
- `contract_part_overridden` — a fixed contract part was overridden.
- `token_wiring_incomplete` — a fixed token slot was left unwired.

Input errors (`EmptyConsumerId`, `WrapperRefMissing`, `UnexpectedWrapperRef`,
`ForbiddenMaterial`) reject malformed declarations before a verdict is reached.

## Acceptance-criteria proofs

The packet validator enforces the acceptance criteria as lints over the worked
binding cases (each case's stored resolution must equal a fresh resolve of its input):

- **AC1 — first consumers render through canonical primitives or audited wrappers,
  not bespoke local variants** (`canonical_rendering_unproven`): every primitive has a
  conformant worked binding that renders through the canonical primitive, and no
  worked binding anywhere is a `bespoke_drift`.
- **AC2 — shared token / state wiring prevents meaning drift between surfaces**
  (`token_wiring_parity_unproven`): every primitive proves identical fixed token
  wiring across two or more host surfaces.
- **AC3 — demos, screenshots, and support / export packets reference the same
  primitive family names** (`naming_parity_unproven`): every row's demo, screenshot,
  and support-export name equals its primitive family token.
- **Whole-matrix coverage** (`matrix_family_coverage_unproven`): the bound component
  families cover every frozen `M5TrustComponentFamily`.

## Hard invariants

Per row, all `false`: `allows_bespoke_local_variant`, `drops_fixed_token_wiring`,
`restyles_fixed_contract_part`, `drops_export_or_audit_truth`.

## Verify

```sh
cargo test -p aureline-shell --lib implement_the_m5_host_rendered
cargo run -q -p aureline-shell --bin aureline_shell_m5_host_rendered_primitives -- validate
```
