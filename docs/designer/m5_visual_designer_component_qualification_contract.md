# M5 Visual-Designer Component Qualification Contract (M05-811)

The visual-designer component **qualification** packet is the M05-811 capstone that
CLOSES the B94 visual-designer component lane by consolidating the whole lane into a
single referenceable **certification bundle**. Where the earlier B94 lanes froze the
reusable components (M05-804), resolved the selected-node, source-round-trip, and
breakpoint/device-preview primitives (M05-805 / M05-806 / M05-807), certified the
accessibility fallback (M05-808), adopted the components across handoff consumers
(M05-809), and certified per-surface auto-narrowing (M05-810), this lane produces one
qualification packet — keyed on the claim-bearing **consumer** — proving that every
claimed visual-design, preview, framework-pack, docs / demo, and handoff consumer
either passes the shared component parity check on every dimension or narrows
automatically, and that release / help / support packets can cite a single
certification bundle for all of it.

- **Boundary schema:** `schemas/ui/m5-visual-designer-component-qualification.schema.json`
- **Rust contract:** `crates/aureline-preview` module
  `qualify_shared_visual_designer_components_across_every_claimed_consumer_with_one_certification_bundle`
- **Canonical export (the one bundle):**
  `artifacts/release/m5-visual-designer-component-qualification-proof/support_export.json`
  (plus `matrix.csv`); Markdown report at
  `artifacts/components/m5-visual-designer-component-qualification.md`
- **Fixtures:** `fixtures/ui/m5-visual-designer-component-qualification/`

## Model

Each `VisualDesignerQualificationRow` keys on one `M5QualifiedComponentConsumer`
(`visual_design_surface`, `preview_runtime`, `framework_pack_preview`,
`docs_demo_embeds`, `handoff_consumer`, `support_packet`, `help_center`,
`release_evidence`) and qualifies five `M5ComponentQualificationDimension` parity
dimensions:

1. **Source ownership** — source stays canonical, derived state stays explicit.
2. **Mapping quality** — the view discloses how well it maps back to canonical
   source.
3. **Round-trip state** — an unsupported construct or open conflict never collapses
   into a silent write-back.
4. **Token / binding provenance** — token, bound-expression, inherited, and literal
   state stay distinct.
5. **Accessibility / export behavior** — a non-visual fallback and a text / JSON /
   Markdown export are preserved, never a screenshot alone.

Each dimension carries a reused `AxisCertificationState`: `certified` (passes),
`disclosed_narrowed` (weakened and disclosed with a frozen
`M5VisualDesignerDowngradeTrigger` and a precise reason), or `undisclosed_drift`
(hid the drift — rejected). A consumer whose every dimension is certified is
**qualified** (green); one that discloses a narrowing is **qualified-with-narrowing**
(yellow); one that hides drift, drops export truth, or forks the shared components is
**blocked** (red) and may not promote.

## Parity checks (acceptance criteria)

- **AC1 — pass or narrow, using the same shared components.** Every row sets
  `uses_shared_components` and covers all five dimensions. A dimension may narrow
  (`disclosed_narrowed` with a trigger and a precise, non-generic reason) but may
  never hide drift (`undisclosed_drift`). A consumer that drifts on source
  ownership, mapping quality, round-trip state, token/binding provenance, or
  accessibility/export behavior without disclosure fails promotion.
- **AC2 — one certification bundle.** The packet lists every canonical B94 component
  packet in `certified_component_packets` (the frozen matrix, the three primitive
  resolvers, the accessibility fallback, the consumer adoption, and the surface
  certification), and every row cites the single `certification_bundle_ref` and draws
  its `canonical_component_refs` only from that consolidated set. The support, help,
  and release evidence consumers are all qualified so release / help / support
  packets can reference one bundle.
- **AC3 — export parity.** Every row's export preserves the mandatory per-dimension
  parity fields (`consumer_identity`, the five dimension states, and `verdict`) as
  text / JSON / Markdown, and a narrowed consumer additionally exports its
  `narrowed_reason` so support / release exports can reconstruct exactly how the
  consumer narrowed.

## Boundary safety

The packet is metadata-only. It carries typed class tokens, opaque summary /
evidence refs, booleans, and redacted labels — never raw source bodies, diff hunks,
screenshots, runtime payloads, or credentials. `validate()` rejects any obviously
forbidden material.

The checked-in export, the fixtures copy, the example dump
(`cargo run -p aureline-preview --example dump_m5_visual_designer_component_qualification`),
and the in-crate builder (`seeded_m5_visual_designer_component_qualification_packet`)
all share one source of truth and are verified byte-aligned by
`on_disk_export_matches_builder`.
