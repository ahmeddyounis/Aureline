# M5 Selected-Node Primitive: Design-Canvas Frame, Structure-Tree Row, and Property-Inspector Rows

This contract implements the design-canvas, structure-tree-row, and
property-inspector-row families frozen in the
[M5 visual-designer component matrix](m5_visual_designer_component_matrix.md is
tracked under `docs/preview/m5/`) as **one reusable selected-node primitive**.
Aureline resolves a single selected node once and projects it onto three
surfaces — the canvas frame, the tree row, and the inspector rows — that share a
single selection identity, so a user never loses selection context moving between
canvas, tree, inspector, and source.

- **Module:** `crates/aureline-preview/src/implement_the_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive`
- **Boundary schema:** [`schemas/ui/m5-visual-designer-selected-node-primitive.schema.json`](../../schemas/ui/m5-visual-designer-selected-node-primitive.schema.json)
- **Checked support export:** `artifacts/release/m5-visual-designer-selected-node-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-visual-designer-selected-node-proof/matrix.csv`
- **Report:** `artifacts/components/m5-visual-designer-selected-node-primitive.md`
- **Fixtures:** [`fixtures/ui/m5-visual-designer-selected-node-primitive/`](../../fixtures/ui/m5-visual-designer-selected-node-primitive/)

## The two halves

### 1. The resolver

`resolve_visual_selection(&M5VisualSelectionInput) -> Result<M5ResolvedVisualSelection, M5VisualSelectionResolutionError>`
takes one selected node — its identity, kind, canvas source-ownership state,
support state, viewport, optional source span, tree flags, an optional search
query, and its per-property edit inputs — and produces the canvas frame, the tree
row, and the inspector rows.

- **Design-canvas frame** (`M5ResolvedCanvasFrame`): current viewport / device,
  the selected-node outline, the canvas source-ownership state, the support-state
  marker, and a direct open-source action when the node maps to source.
- **Structure-tree row** (`M5ResolvedTreeRow`): node label and kind,
  visibility / lock state, a support-state badge, a selection-sync affordance
  (synchronized only when the node maps to source), and search-match
  highlighting.
- **Property-inspector rows** (`M5ResolvedPropertyRow`): one row per property,
  each rendering the **distinct** `M5PropertyEditorKind` for its
  `M5PropertyValueState` — a literal field, a token-bound picker, a bound-
  expression inspector, an inherited-value trace, a mixed multi-value control, or
  an unset placeholder — plus a reset action, an open-source action, a precise
  support-state note, and whether a write requires review.

### 2. The parity matrix

`M5SelectedNodePrimitivePacket` binds one row per claimed M5 visual-design
surface family — desktop designer, source-first preview, browser-runtime
inspector, framework-pack preview, embedded shell designer, and support-export
replay — and carries worked selection cases whose stored resolution must equal a
fresh resolve of its input. The Rust validator is the authoritative gate.

## Acceptance criteria and how they are enforced

| Acceptance criterion | Mechanism |
| --- | --- |
| Users move between canvas, tree, inspector, and source without losing identity or selection context. | The resolved canvas frame, tree row, and inspector rows all carry the input `selection_id`; `M5ResolvedVisualSelection::identity_consistent` proves it, and the `IdentityPreservationUnproven` lint requires a worked case that demonstrates it. |
| Property editing no longer flattens token, bound, inherited, and literal states into one ambiguous control. | `M5PropertyEditorKind::for_value_state` is a one-to-one mapping from value state to editor control; `value_states_distinguished` proves each row uses it, and the `ValueStateDistinctionUnproven` lint requires a worked case rendering ≥2 distinct value states with distinct editors. |
| Visual-design surfaces expose source ownership and support state before mutation. | A property that claims a writing scope on a node whose canvas is not source-bound editable, whose support state forbids writes, or that targets a blocked protected path is refused with `MutationWithoutSourceOwnership`; the `SourceOwnershipGateUnproven` lint requires a worked inspect-only case with no writable path and every writable case to disclose source ownership. |

## Boundary hygiene

Raw source bodies, diff hunks, file contents, credentials, and raw provider
payloads never cross this boundary. Value representations, node labels, and the
serialized packet are screened for forbidden material (`api_key`, `password`,
`secret`, bearer tokens, URLs, PEM blocks); a violation fails resolution or
packet validation.

## Reused vocabulary

The canvas state (`M5CanvasState`), structure-node kinds (`M5StructureNodeKind`),
property value states (`M5PropertyValueState`), property write scopes
(`M5PropertyWriteScope`), device classes (`M5DevicePreviewClass`), and downgrade
triggers (`M5VisualDesignerDowngradeTrigger`) are reused verbatim from the frozen
visual-designer component matrix. The protected-path posture
(`ProtectedPathPosture`), preview-diff class (`PreviewDiffClass`), mutation-review
posture (`MutationReviewPosture`), and preview surface (`PreviewSurface`) are
reused from the sibling preview matrices. This module mints new vocabulary only
for the surface families, the support state, the property-editor kind, and the
export fields.
