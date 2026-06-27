# M5 design-system reference-layout package

The **reference-layout package** is the versioned, machine-readable truth for how
the dominant M5 workspaces occupy the governed shell zones, so feature
implementations place panes against a checked-in descriptor instead of ad hoc.
Where the [contract matrix](m5-design-system-contract-matrix.md) governs *which*
design-system objects exist, the [foundation package](m5-foundation-package.md)
ships the *tokens*, and the [component manifests](m5-component-manifest.md) ship
the durable *component contracts*, this package ships the *reference layouts*:
one descriptor per launch-critical M5 workspace family.

- Schema: [`schemas/design-system/m5-reference-layout-package.schema.json`](../../schemas/design-system/m5-reference-layout-package.schema.json)
- Canonical package: [`fixtures/ui/m5-reference-layout/reference-layout-package.json`](../../fixtures/ui/m5-reference-layout/reference-layout-package.json)
- Per-workspace fixtures: [`fixtures/ui/m5-reference-layout/`](../../fixtures/ui/m5-reference-layout/)
- Release packet: [`artifacts/release/m5-design-system-proof/reference-layout-release.json`](../../artifacts/release/m5-design-system-proof/reference-layout-release.json)
- Shell-slot conformance packet: [`artifacts/release/m5-design-system-proof/reference-layout-conformance.json`](../../artifacts/release/m5-design-system-proof/reference-layout-conformance.json)
- Producer: `cargo run -p aureline-design-system --bin aureline_design_system_m5_reference_layout`

## Workspace families

The package publishes one reference layout per workspace family. The
`workspace_kind` token is stable and shared with shell code, docs/help, and
support exports:

| `workspace_kind` | Workspace |
| ---------------- | --------- |
| `notebook` | Notebook cell working set, kernel runtime, and outputs |
| `data_grid` | Dense, virtualizable result grid with a query/source surface |
| `profiler` | Capture working set with a flame/detail inspector |
| `pipeline` | Stage cards and a job/run review surface |
| `docs` | Embedded docs/browser reading pane with navigation |
| `preview` | Live preview canvas with route and trust truth |
| `incident` | Incident timeline with linked evidence and actions |
| `companion` | Cross-device companion surface that mirrors a primary session |

## What a descriptor records

Each `M5WorkspaceReferenceLayout` carries versioned lifecycle and owner metadata
plus four governed blocks:

- **`zone_occupancy`** — which governed shell `zone` the workspace claims, the
  `slot_id` it fills, the `surface_kind` (`host_chrome`, `first_party`,
  `provider_backed`, `extension_contributed`) that fills it, whether the zone is
  `required`, and the `placeholder_behavior` the zone shows before its content
  resolves. Every workspace claims — and marks `required` — the
  `main_workspace` work surface and the `status_bar`.
- **`responsive_collapse`** — one rule per adaptive class (`compact_desktop`,
  `standard_desktop`, `expanded_desktop`) naming which zones collapse and the
  `placement` they collapse to (`sheet`, `overflow`, or in-slot `placeholder`).
  The persistent zones (`main_workspace`, `status_bar`) never collapse, and the
  widest class collapses nothing.
- **`missing_dependency_rules`** — when a dependency is absent, the
  `placeholder_class` (`missing_remote`, `missing_provider`, …), the governed
  `placeholder_message_id`, and the `degraded_behavior` the affected zone shows,
  so a missing kernel, dev server, provider, or paired session degrades to the
  declared placeholder instead of a blank pane.
- **`reopen_routes`** — the routes that reopen a closed surface or reset the
  workspace to its reference layout, each with its governed `command_message_id`
  and key chord. Every workspace offers at least one `reopen` route and exactly
  one `reset` route.

The governed zone, slot, fallback-placement, and placeholder-class tokens match
the canonical shell vocabulary, so shell code, docs/help, and support exports
name the same layout identities and collapse states users actually see.

## Shell-slot conformance packet

`M5ReferenceLayoutPackage::conformance_packet` projects the
`M5ShellSlotConformancePacket` — the flattened, **slot-keyed** layout truth a
feature implementation tests against. For each workspace it enumerates:

- `slot_expectations` — the slots the workspace claims, their `surface_kind`,
  whether each is `required`, and the placeholder behavior the slot must show.
- `collapse_expectations` — per adaptive class, the exact **slot ids** that
  collapse (resolved from the zones) and their placement, so a feature test names
  slots rather than zones.
- `missing_dependency_expectations` — each missing dependency resolved to the
  slot id it feeds and the placeholder class the slot must show.
- `reopen_route_expectations` — the reopen/reset routes the workspace must offer.

A notebook, profiler, or pipeline implementation asserts against the conformance
packet so its zone, collapse, placeholder, and reopen behavior is checked against
the same descriptor the design system ships — not a hand-written assertion list
that can drift.

## Release-packet inclusion

`release_packet` projects a `m5_design_system_reference_layout_release` packet
with one lifecycle-and-shape summary per layout (lifecycle state, layout version,
and counts of zones, required zones, collapse rules, missing-dependency rules,
and reopen routes), so a release record names the layout revision QA and support
exports cite.

## Privacy and boundary

Reference layouts are metadata-only truth packets. They carry semantic slot
*ids* and message *ids* — never raw geometry payloads, credential bodies, or
provider payloads. The validator scans the serialized export for forbidden
boundary material as defense in depth.

## Drift control

The seed builder in `aureline-design-system` is the single producer of the
checked-in package fixture, the per-workspace fixtures, the release packet, and
the conformance packet, and the inline tests assert each checked-in artifact
matches the seed and validates, so any drift fails
`cargo test -p aureline-design-system m5_reference_layout`.
