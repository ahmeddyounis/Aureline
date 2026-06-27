# M5 surface-qualification packet

The **surface-qualification packet** is the integrating proof that ties each
claimed M5 workspace surface to the *current* design-system contract and
evidence it depends on. Where the
[contract matrix](m5-design-system-contract-matrix.md) freezes the object model
the design system ships, and the four lanes each ship one slice of truth — the
[foundation package](m5-foundation-package.md) (tokens / density / motion /
contrast / state), the [component manifests](m5-component-manifest.md)
(component contracts), the
[reference-layout package](m5-reference-layout-package.md) (workspace layouts),
and the [evidence pack](m5-evidence-pack.md) (visual / accessibility proof) —
this packet is the layer that **qualifies a surface against all four at once** and
auto-narrows its public claim when the contract or proof behind it goes stale or
missing.

- Packet schema: [`schemas/design-system/m5-surface-qualification.schema.json`](../../schemas/design-system/m5-surface-qualification.schema.json)
- Dashboard schema: [`schemas/design-system/m5-surface-qualification-dashboard.schema.json`](../../schemas/design-system/m5-surface-qualification-dashboard.schema.json)
- Support export: [`artifacts/release/m5-design-system-proof/surface-qualification.json`](../../artifacts/release/m5-design-system-proof/surface-qualification.json)
- Published dashboard: [`artifacts/design-system/m5-surface-qualification-dashboard.json`](../../artifacts/design-system/m5-surface-qualification-dashboard.json)
- Markdown proof: [`artifacts/release/m5-design-system-proof/surface-qualification-proof.md`](../../artifacts/release/m5-design-system-proof/surface-qualification-proof.md)
- Drill fixtures: [`fixtures/ui/m5-surface-qualification/`](../../fixtures/ui/m5-surface-qualification/)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_surface_qualification`

## What a surface qualification records

The packet carries one qualification row per claimed M5 workspace surface
(notebook, data grid, profiler, pipeline, docs, preview, incident, companion).
Each row:

- **names the component families it renders** (`bound_component_kinds`) and binds
  the surface to all four lanes (`lane_bindings`), recording the artifact, schema,
  and version it was qualified against;
- **resolves a green/yellow/red `status`** — `qualified` when every lane is
  conformant and current, `provisional` when a lane is stale or non-conformant,
  `disqualified` when a required contract artifact or usable proof is absent. The
  status reflects *true* conformance and is never softened by a waiver;
- **resolves a `gate_decision`** the release/public-truth automation reads and an
  `effective_class` the stable-claim matrix reads — see below;
- **names the exact `gaps`** that caused any narrowing or block, one per lane
  subject, so a gap is never left invisible.

## The verdict is derived from the four lanes

Every binding's conformance is computed from the checked-in lane packets, so the
qualification can never outrun the contract that backs it:

- **Foundation** — every bound component manifest's `token_dependencies` resolve
  to a token the foundation package publishes, and every canonical controlled
  state is published by the foundation state family. A token or state that does
  not resolve is a **token/state conformance** failure (`nonconformant`).
- **Component contract** — every bound family has a published manifest. A bound
  family with no manifest is `missing`.
- **Reference layout** — the surface's workspace has a published reference layout.
  A workspace with no layout is `missing`.
- **Evidence** — every bound family's evidence is current and complete. The
  evidence pack's per-component claim gate drives this: `certified` →
  `conformant`, `narrowed` (stale) → `stale`, `blocked` (incomplete coverage) →
  `missing`.

## Stale or failing conformance narrows; missing contracts block

The gate follows the same deterministic rule the rest of the design system uses:

- a **missing** contract or usable proof (`component_manifest_missing`,
  `reference_layout_missing`, `evidence_blocked`) **blocks** the surface from
  Stable promotion — the surface is disqualified, its effective claim is held, and
  the gap is named, not hidden;
- a **stale or non-conformant** lane (`evidence_stale`,
  `foundation_token_unresolved`, `foundation_state_unpublished`) **auto-narrows**
  the surface below Stable (floored at Beta) before promotion — the surface ships
  at a reduced, disclosed claim rather than blocking.

A blocking gap can be accepted under an active, disclosed **waiver** scoped to a
single gap subject, which ships the surface auto-narrowed to the waived claim
while its true status stays `disqualified` (red) and the gap is named as waived.

`M5SurfaceQualificationPacket::reevaluate`-style narrowing is driven by the
evidence pack's own `reevaluate(evaluated_at)`: the
`seeded_m5_surface_qualification_packet_stale_narrowed` drill qualifies the
surfaces as-of a later release date, so the surfaces that render older,
freshly-staler component families auto-narrow while surfaces that render only
freshly-captured families stay qualified.

## One output, many consumers

The compact [`M5SurfaceQualificationDashboard`] projection is the published
green/yellow/red scoreboard. **Help/About** surfaces the headline counts and the
narrowed/blocked surface ids; the **release center** and **shiproom** watch the
`release_gate` and the dashboard for regressions; **support exports** ship the
full packet; the **stable-claim matrix** reads each surface's `effective_class`.
They all read the same packet and dashboard rather than maintaining parallel
spreadsheets.

## Privacy and boundary

Qualification packets are metadata-only truth packets. They carry contract
*refs*, lane *versions*, message *ids*, and verdict *tokens* — never raw color
values, screenshots, credential bodies, or provider payloads. The validator scans
the serialized export for forbidden boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in support export, dashboard, Markdown proof, and the four drill
fixtures, and it derives every verdict from the same checked-in foundation,
manifest, layout, and evidence packets Aureline ships. The inline tests assert
the checked-in artifacts match the seed and validate, that every bound manifest
token resolves in the foundation package, and that the stale / token-drift /
missing-manifest / waiver drills narrow and block deterministically, so any drift
fails `cargo test -p aureline-design-system m5_surface_qualification`.

[`M5SurfaceQualificationDashboard`]: ../../crates/aureline-design-system/src/m5_surface_qualification/mod.rs
