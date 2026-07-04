# M5 Visual-Designer Component Consumer Contract (M05-809)

This contract proves the frozen M5 visual-designer component families
(`schemas/ui/m5-visual-designer-component-matrix.schema.json`, M05-804) are
reusable **primitives** rather than a single designer-page implementation. Where
M05-805..807 resolve each primitive's per-target truth (selected node, source
round-trip honesty, breakpoint / device preview) and M05-808 certifies their
accessibility fallback, this lane adopts the primitives across the four claimed
M5 handoff consumer classes and proves every consumer — however narrowed — keeps
identical labels, tokens, and degraded-state vocabulary and points back to one
canonical family.

- **Module.**
  `crates/aureline-preview/src/add_shared_framework_pack_preview_runtime_browser_runtime_docs_demo_and_onboarding_visual_designer_component_consumers/`
- **Boundary schema.**
  `schemas/ui/m5-visual-designer-component-consumer.schema.json`
- **Support export (`include_str!` canonical).**
  `artifacts/release/m5-visual-designer-component-consumer-proof/support_export.json`
- **CSV matrix.**
  `artifacts/release/m5-visual-designer-component-consumer-proof/matrix.csv`
- **Fixtures.** `fixtures/ui/m5-visual-designer-component-consumers/`

## Consumer classes

Every `VisualDesignerConsumerRow` declares one of the four claimed
`ConsumerGroup` classes, and each row must be present at least once:

- **`framework_pack`** — a framework-pack preview lane.
- **`preview_runtime`** — the preview-runtime inspector.
- **`browser_runtime_demo`** — the browser-runtime inspector or a demo / share
  handoff.
- **`docs_onboarding`** — a docs / onboarding walkthrough or the help center.

The concrete `M5HandoffConsumerSurface` a row embeds in must belong to the
declared group (`surface_group_consistent`).

## Canonical family binding

Each row points back to exactly one canonical primitive family. `component_family`
resolves — via `canonical_schema_ref_for` / `canonical_packet_ref_for` — to the
one primitive schema and release-proof packet that owns it:

- `design_canvas`, `structure_tree_row`, `property_inspector_row` → the M05-805
  selected-node primitive.
- `source_sync_chip`, `unsupported_construct_card`, `round_trip_conflict_banner` →
  the M05-806 source round-trip honesty primitive.
- `breakpoint_preview_row` → the M05-807 breakpoint / device-preview primitive.

A row must reference the canonical schema and packet
(`points_to_canonical_family`) and set `references_canonical_not_local_prose` —
never clone a surface-local copy.

## Acceptance criteria

- **AC1 — Multiple M5 surfaces point back to one canonical family.** Every row
  satisfies `points_to_canonical_family`, and at least one family is adopted
  across two or more consumer groups (`families_reused_across_groups >= 1`, else
  `NoFamilyReusedAcrossGroups`). All seven frozen families are adopted.
- **AC2 — Degraded / inspect-only consumers stay label- and state-parity with the
  primary designer surface.** A narrowed consumer keeps `label_parity` at
  `disclosed_narrowed` (never `renamed_or_dropped`), preserves the controlled
  label families (`support_class`, `runtime_origin`, `unsupported_construct`,
  `round_trip_conflict`, `open_source_fallback` — collectively covered by
  `label_family_coverage_complete`), keeps its degraded-state vocabulary and the
  reused `M5VisualDesignerRequiredLabel` set, and keeps token / density / motion
  behavior consistent with the design-system contract
  (`keeps_design_system_fidelity`) even when inspect-only or compare-only. Each
  narrowed consumer discloses the reduction with a `ReducedCapabilityBanner`
  whose `capability_state` matches its `authority_mode`, and carries a handoff
  note whenever it punts to another surface (`discloses_narrowing`).
- **AC3 — Docs / help / demo / onboarding flows no longer clone local
  visual-designer semantics.** At least one `docs_onboarding` consumer sets
  `references_canonical_not_local_prose`, else `MissingDocsOnboardingReference`.

## Copy / export & boundary safety

Every row carries text / JSON / Markdown copy-export parity with a named export
field set and `screenshot_only_prohibited` (`copy_export.is_complete`). The packet
is metadata-only: `validate()` rejects raw boundary material (`api_key`,
`password`, `secret`, PEM blocks, bearer tokens) in the export.

## Validation & drift

`VisualDesignerConsumerPacket::validate()` returns a
`Vec<VisualDesignerConsumerViolation>` covering schema / record-kind / identity,
duplicate ids, incomplete rows, surface-group mismatch, canonical-family
mismatch, label-parity breaks, design-system drift, undisclosed narrowing,
missing copy-export, missing consumer groups / families / label families, no
cross-group reuse, missing docs-onboarding reference, summary mismatch, and
forbidden export material. The stored `summary` must equal `computed_summary()`.
The checked-in support export is the `include_str!` source of truth; the fixtures
copy and the `seeded_m5_visual_designer_component_consumers_packet()` builder are
kept byte-identical by `on_disk_export_matches_builder`.
