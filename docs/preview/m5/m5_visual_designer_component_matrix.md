# M5 Visual-Designer Component Matrix

Status: frozen (M05-804, batch B94)

This contract freezes the reusable **visual-designer component** lane so
Milestone 5 stops depending on preview-only conventions and instead ships one
canonical canvas / inspector / source-sync contract. Later M5 rows reference one
component family by name instead of restating visual-designer truth in
feature-local prose.

- Rust: `crates/aureline-preview/src/freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix/`
- Schema: `schemas/ui/m5-visual-designer-component-matrix.schema.json`
- Canonical support export (`include_str!`): `artifacts/preview/m5/m5_visual_designer_component_matrix/support_export.json`
- Matrix CSV: `artifacts/preview/m5/m5_visual_designer_component_matrix/matrix.csv`
- Matrix summary: `artifacts/design/m5-visual-designer-component-matrix.md`
- Fixtures: `fixtures/ui/m5-visual-designer-components/`

## Reused vocabulary (no parallel synonyms)

The matrix binds every component onto the write-scope, preview/apply/revert,
citation, and degraded-state vocabulary already used by source-first preview, AI
apply, and refactor flows rather than minting bespoke designer chrome:

- `SourceSyncClass` — source-sync posture (shared with the source-first preview
  matrix and preview-session descriptors).
- `RoundTripCapabilityClass` — round-trip capability claim.
- `PreviewSurface` — which claimed preview surface a component belongs to.
- `ProtectedPathPosture`, `PreviewDiffClass` — write-scope / diff vocabulary from
  the visual-edit transform lane.
- `MutationReviewPosture` — the review a write requires.
- `UnsupportedConstructCard` — the shared degrade card struct, reused verbatim.

## Component families

The matrix defines seven reusable primitives; every one must appear:

| Family | Purpose | Key state vocabulary |
| --- | --- | --- |
| `design_canvas` | The visual canvas the user edits on | `M5CanvasState`; derivative-of-source + selection-sync invariants |
| `structure_tree_row` | A row in the structure / layers tree | `M5StructureNodeKind`; mapped-vs-unmapped span truth |
| `property_inspector_row` | A row in the property inspector | `M5PropertyValueState` × `M5PropertyWriteScope` |
| `source_sync_chip` | A chip disclosing source relationship | `SourceSyncClass` × `M5SyncRecoveryRoute` |
| `breakpoint_preview_row` | A breakpoint / device-preview row | `M5DevicePreviewClass`, `M5PreviewDataPosture`, `M5BreakpointMappingQuality` |
| `unsupported_construct_card` | The degrade card for an unsupported construct | reused `UnsupportedConstructCard` |
| `round_trip_conflict_banner` | A banner shown on a round-trip conflict | `M5RoundTripConflictClass` × `M5ConflictResolutionRoute` |

## Frozen honesty rules (validated by `validate()`)

- **Source stays canonical; canvas state is derivative and explicit.** A canvas
  row declares `is_derivative_of_source` and keeps
  `selection_synced_with_tree_and_source`; it never becomes a second writable
  truth model.
- **Tree, canvas, and source selection stay synchronized.** A mapped tree row
  carries a synchronized selection and a real source span; an unmapped /
  generated node discloses `maps_to_source_span = false` and carries no span
  rather than fake a mapping.
- **Property editors distinguish value state and never widen write scope
  silently.** `widens_write_scope_silently` must be false, a design-token value
  can never be recorded as a `single_literal_span`, and a bound-expression /
  inherited / mixed value degrades to inspect-only or a protected block instead
  of a silent write. A writing scope previews a real source diff and names a
  review posture.
- **Unsupported constructs, generated / protected files, and round-trip conflicts
  never collapse into silent writeback.** The unsupported card preserves the
  selection context with a precise label; the conflict banner sets
  `never_silent_writeback` and names a real resolution route.
- **Breakpoint / device previews keep runtime origin, live-versus-mock posture,
  and mapping quality visible.** Each device row names its `device_class`,
  `data_posture`, `mapping_quality`, and a `runtime_origin_token`.
- **Required labels + parity.** Every row carries the mandatory label set
  (identity, source ownership, state, keyboard route), is `export_safe`, and is
  `assistive_ready`. Source-sync chips never disclose a sync class different from
  their row.

## Required labels

`M5VisualDesignerRequiredLabel`: `identity`, `source_ownership`, `state`,
`sync_or_freshness`, `keyboard_route`. Mandatory subset (must appear on every
row): `identity`, `source_ownership`, `state`, `keyboard_route`.

## Degraded states

Every degraded row carries a typed `M5VisualDesignerDowngradeTrigger`
(`drifted_from_source`, `unmapped_source`, `runtime_unavailable`,
`protected_path_blocked`, `unsupported_construct`, `round_trip_conflict_open`,
`unidentified_posture`) and a precise, non-generic label. The matrix must
demonstrate at least one complete degraded row.

## Boundary safety

Raw source bodies, diff hunks, file contents, credentials, and raw provider
payloads never cross this boundary. The packet carries only typed class tokens,
opaque span / selection / evidence refs, booleans, and redacted labels; the
export is scanned for forbidden material before it is accepted.

## Verify

```
cargo test -p aureline-preview --lib freeze_the_m5_design_canvas
cargo run -p aureline-preview --example dump_m5_visual_designer_component_matrix support
```
