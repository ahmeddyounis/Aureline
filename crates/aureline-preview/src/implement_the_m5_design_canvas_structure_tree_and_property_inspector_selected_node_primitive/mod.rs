//! One reusable M5 selected-node primitive: the design-canvas frame, the
//! structure-tree row, and the property-inspector rows for a single selected
//! node, resolved once so identity, selection context, value state, source
//! ownership, and support state stay consistent across every claimed M5
//! visual-design surface.
//!
//! Aureline's frozen visual-designer component matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! names the design canvas, the structure-tree row, and the property-inspector
//! row as governed component families and freezes their state vocabulary. This
//! module *implements* those three families as one reusable primitive: a resolver
//! that takes a selected node and produces one [`M5ResolvedVisualSelection`]
//! carrying the canvas frame, the tree row, and the inspector rows that share a
//! single selection identity, so a user can move between canvas, tree, inspector,
//! and source without losing selection context.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_visual_selection`] — that takes one
//!    [`M5VisualSelectionInput`] (a selected node, its canvas / source-ownership
//!    state, its support state, and its per-property edit inputs) and produces one
//!    [`M5ResolvedVisualSelection`]. The resolved selection carries the same
//!    `selection_id` on the canvas frame, the tree row, and the inspector rows so
//!    identity is never lost across surfaces (AC1); it renders a distinct
//!    [`M5PropertyEditorKind`] per [`M5PropertyValueState`] so a token, a bound
//!    expression, an inherited value, and a literal are never flattened into one
//!    ambiguous control (AC2); and it refuses to resolve a mutation on a node that
//!    has not first disclosed source ownership and support state (AC3).
//! 2. A parity matrix — [`M5SelectedNodePrimitivePacket`] — that binds one row per
//!    claimed M5 visual-design surface family (desktop designer, source-first
//!    preview, browser-runtime inspector, framework-pack preview, embedded shell
//!    designer, and support-export replay) to the shared canvas / tree / inspector
//!    contract and carries worked resolution cases so the support / export packet
//!    can reconstruct selected-node truth from one shared model on every surface.
//!
//! The canvas state vocabulary ([`M5CanvasState`]), the structure-node kinds
//! ([`M5StructureNodeKind`]), the property value states
//! ([`M5PropertyValueState`]), the property write scopes
//! ([`M5PropertyWriteScope`]), the device / viewport classes
//! ([`M5DevicePreviewClass`]), and the downgrade triggers
//! ([`M5VisualDesignerDowngradeTrigger`]) are reused verbatim from the frozen
//! component matrix; the protected-path posture ([`ProtectedPathPosture`]), the
//! preview-diff class ([`PreviewDiffClass`]), the mutation-review posture
//! ([`MutationReviewPosture`]), and the preview-surface / source-sync / round-trip
//! vocabulary are reused from the sibling preview matrices. This module mints new
//! vocabulary only for what the frozen matrix left implicit about the selected-node
//! primitive itself: the claimed surface families, the support-state marker shared
//! across canvas / tree / inspector, the property-editor kind per value state, and
//! the export fields. No M5 surface invents a second selected-node grammar.
//!
//! Raw source bodies, diff hunks, file contents, credentials, and raw provider
//! payloads never cross this boundary; the primitive carries only typed class
//! tokens, opaque selection / span refs, booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-selected-node-primitive.schema.json`](../../../../schemas/ui/m5-visual-designer-selected-node-primitive.schema.json)
//! and the contract doc is
//! [`docs/designer/m5_visual_designer_selected_node_primitive_contract.md`](../../../../docs/designer/m5_visual_designer_selected_node_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-visual-designer-selected-node-primitive/`](../../../../fixtures/ui/m5-visual-designer-selected-node-primitive/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The canvas state, structure-node kinds, property value states, property write
// scopes, device classes, and downgrade triggers are frozen once, in the
// visual-designer component matrix. This primitive reuses them verbatim so it
// never invents a parallel designer vocabulary.
pub use crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix::{
    M5CanvasState, M5DevicePreviewClass, M5PropertyValueState, M5PropertyWriteScope,
    M5StructureNodeKind, M5VisualDesignerDowngradeTrigger,
};

// The protected-path posture and preview-diff class are frozen in the
// visual-edit-transform manifest; the mutation-review posture in the
// browser-runtime inspectors; the preview surface in the source-first matrix.
pub use crate::{MutationReviewPosture, PreviewDiffClass, PreviewSurface, ProtectedPathPosture};

/// Stable record-kind tag carried by [`M5SelectedNodePrimitivePacket`].
pub const M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive";

/// Schema version for M5 selected-node-primitive records.
pub const M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the selected-node-primitive boundary schema.
pub const M5_SELECTED_NODE_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-selected-node-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SELECTED_NODE_DOC_REF: &str =
    "docs/designer/m5_visual_designer_selected_node_primitive_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this
/// primitive narrows from.
pub const M5_SELECTED_NODE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the visual-edit-transform contract this primitive binds
/// its write scopes against.
pub const M5_SELECTED_NODE_VISUAL_EDIT_REF: &str =
    "schemas/preview/visual_edit_transforms.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SELECTED_NODE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-selected-node-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_SELECTED_NODE_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-designer-selected-node-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SELECTED_NODE_CSV_REF: &str =
    "artifacts/release/m5-visual-designer-selected-node-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_SELECTED_NODE_REPORT_REF: &str =
    "artifacts/components/m5-visual-designer-selected-node-primitive.md";

/// One claimed M5 visual-design surface family that renders the shared
/// selected-node primitive. These are the surfaces the goal names — desktop
/// designer, source-first preview, browser-runtime inspector, framework-pack
/// preview, embedded shell designer, and support-export replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignSurfaceFamily {
    /// The desktop visual designer.
    DesktopDesigner,
    /// The source-first live preview.
    SourceFirstPreview,
    /// The browser-runtime inspector.
    BrowserRuntimeInspector,
    /// A framework-pack preview surface.
    FrameworkPackPreview,
    /// An embedded designer hosted in the app shell.
    EmbeddedShellDesigner,
    /// A support-export replay of a captured selection.
    SupportExportReplay,
}

impl M5VisualDesignSurfaceFamily {
    /// Every claimed visual-design surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopDesigner,
        Self::SourceFirstPreview,
        Self::BrowserRuntimeInspector,
        Self::FrameworkPackPreview,
        Self::EmbeddedShellDesigner,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopDesigner => "desktop_designer",
            Self::SourceFirstPreview => "source_first_preview",
            Self::BrowserRuntimeInspector => "browser_runtime_inspector",
            Self::FrameworkPackPreview => "framework_pack_preview",
            Self::EmbeddedShellDesigner => "embedded_shell_designer",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DesktopDesigner => "Desktop Designer",
            Self::SourceFirstPreview => "Source-First Preview",
            Self::BrowserRuntimeInspector => "Browser-Runtime Inspector",
            Self::FrameworkPackPreview => "Framework-Pack Preview",
            Self::EmbeddedShellDesigner => "Embedded Shell Designer",
            Self::SupportExportReplay => "Support-Export Replay",
        }
    }
}

/// The support-state marker the canvas frame, the tree-row badge, and the
/// inspector note all render for one node, so support state is disclosed the same
/// way on every surface before a user attempts a mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualSupportState {
    /// The node round-trips to source through the shared apply / revert path.
    FullySupported,
    /// The node is editable but a write requires review first.
    PartiallySupported,
    /// The node is shown for reference and takes no write.
    InspectOnly,
    /// The construct cannot round-trip; the surface degrades to a code-first
    /// suggestion.
    UnsupportedConstruct,
    /// The node has no resolvable source span; it stays inspect-only.
    UnmappedNode,
}

impl M5VisualSupportState {
    /// Every support state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullySupported,
        Self::PartiallySupported,
        Self::InspectOnly,
        Self::UnsupportedConstruct,
        Self::UnmappedNode,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::PartiallySupported => "partially_supported",
            Self::InspectOnly => "inspect_only",
            Self::UnsupportedConstruct => "unsupported_construct",
            Self::UnmappedNode => "unmapped_node",
        }
    }

    /// True when this support state permits a write back to source.
    pub const fn permits_write(self) -> bool {
        matches!(self, Self::FullySupported | Self::PartiallySupported)
    }

    /// True when a write under this support state requires review first.
    pub const fn requires_review(self) -> bool {
        matches!(self, Self::PartiallySupported)
    }

    /// A precise, non-generic support note safe to render on any surface.
    pub const fn note(self) -> &'static str {
        match self {
            Self::FullySupported => {
                "Fully supported: this value round-trips to source through the shared apply and revert path"
            }
            Self::PartiallySupported => {
                "Partially supported: edits are allowed but require review before they touch source"
            }
            Self::InspectOnly => {
                "Inspect-only: this value is shown for reference and the inspector takes no write"
            }
            Self::UnsupportedConstruct => {
                "This construct cannot round-trip to source; the inspector degrades to a code-first suggestion"
            }
            Self::UnmappedNode => {
                "This node has no resolvable source span; it stays inspect-only rather than fake a mapping"
            }
        }
    }
}

/// The distinct property-editor control rendered for one property value state, so
/// a token, a bound expression, an inherited value, and a literal are never
/// flattened into one ambiguous control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PropertyEditorKind {
    /// A direct literal-value field.
    LiteralField,
    /// A design-token picker that names the shared token being bound.
    TokenBoundPicker,
    /// A read-only inspector for a runtime-bound expression.
    BoundExpressionInspector,
    /// A read-only trace of where an inherited value comes from.
    InheritedValueTrace,
    /// A mixed-value control for a multi-selection.
    MixedMultiValue,
    /// A placeholder for an unset value that falls back to default.
    UnsetPlaceholder,
}

impl M5PropertyEditorKind {
    /// Every editor kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiteralField,
        Self::TokenBoundPicker,
        Self::BoundExpressionInspector,
        Self::InheritedValueTrace,
        Self::MixedMultiValue,
        Self::UnsetPlaceholder,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiteralField => "literal_field",
            Self::TokenBoundPicker => "token_bound_picker",
            Self::BoundExpressionInspector => "bound_expression_inspector",
            Self::InheritedValueTrace => "inherited_value_trace",
            Self::MixedMultiValue => "mixed_multi_value",
            Self::UnsetPlaceholder => "unset_placeholder",
        }
    }

    /// The distinct editor kind for a property value state. This one-to-one
    /// mapping is what proves the inspector never collapses distinct value states
    /// into one control.
    pub const fn for_value_state(value: M5PropertyValueState) -> Self {
        match value {
            M5PropertyValueState::Literal => Self::LiteralField,
            M5PropertyValueState::DesignToken => Self::TokenBoundPicker,
            M5PropertyValueState::BoundExpression => Self::BoundExpressionInspector,
            M5PropertyValueState::Inherited => Self::InheritedValueTrace,
            M5PropertyValueState::Mixed => Self::MixedMultiValue,
            M5PropertyValueState::Unset => Self::UnsetPlaceholder,
        }
    }
}

/// A field the support / export packet carries so selected-node truth is
/// reconstructable from the shared model. The first four in
/// [`M5SelectedNodeExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectedNodeExportField {
    /// The stable selection identity, shared across canvas / tree / inspector.
    SelectionId,
    /// The structure-node kind.
    NodeKind,
    /// The canvas source-ownership state.
    CanvasState,
    /// The support state marker.
    SupportState,
    /// The per-property value states rendered in the inspector.
    PropertyValueStates,
    /// The opaque source-span ref, when the node maps to source.
    SourceSpanRef,
}

impl M5SelectedNodeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectionId,
        Self::NodeKind,
        Self::CanvasState,
        Self::SupportState,
        Self::PropertyValueStates,
        Self::SourceSpanRef,
    ];

    /// The export fields every selected-node export must carry.
    pub const MANDATORY: [Self; 4] = [
        Self::SelectionId,
        Self::NodeKind,
        Self::CanvasState,
        Self::SupportState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionId => "selection_id",
            Self::NodeKind => "node_kind",
            Self::CanvasState => "canvas_state",
            Self::SupportState => "support_state",
            Self::PropertyValueStates => "property_value_states",
            Self::SourceSpanRef => "source_span_ref",
        }
    }
}

/// One property's edit input, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PropertyEditInput {
    /// The stable property key (e.g. a style property name).
    pub property_key: String,
    /// The semantic state of the value.
    pub value_state: M5PropertyValueState,
    /// The write scope an edit of this value would take.
    pub write_scope: M5PropertyWriteScope,
    /// How a protected target gates the write.
    pub protected_path_posture: ProtectedPathPosture,
    /// The opaque, export-safe value representation.
    pub value_repr: String,
}

/// The full input to the selected-node resolver for one node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualSelectionInput {
    /// The stable selection identity that must survive across canvas / tree /
    /// inspector / source.
    pub selection_id: String,
    /// The human-readable node label rendered in the tree.
    pub node_label: String,
    /// What kind of node this is.
    pub node_kind: M5StructureNodeKind,
    /// How the canvas relates to canonical source (the source-ownership state).
    pub canvas_state: M5CanvasState,
    /// The support state disclosed on every surface.
    pub support_state: M5VisualSupportState,
    /// The current viewport / device the canvas shows.
    pub viewport: M5DevicePreviewClass,
    /// The opaque source-span ref, present iff the node kind maps to source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_ref: Option<String>,
    /// Whether the node is hidden in the tree.
    pub visibility_hidden: bool,
    /// Whether the node is locked in the tree.
    pub locked: bool,
    /// A case-insensitive search query that drives tree search-match highlighting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query: Option<String>,
    /// The per-property edit inputs rendered in the inspector.
    pub properties: Vec<M5PropertyEditInput>,
}

/// The resolved design-canvas frame for a selected node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCanvasFrame {
    /// The selection identity — identical to the tree row and inspector.
    pub selection_id: String,
    /// The current viewport / device.
    pub viewport: M5DevicePreviewClass,
    /// The canvas source-ownership state.
    pub canvas_state: M5CanvasState,
    /// The selected node is outlined on the canvas; always holds.
    pub selected_node_outlined: bool,
    /// The support state marker shown on the frame.
    pub support_state: M5VisualSupportState,
    /// The direct open-source action is offered when the node maps to source.
    pub open_source_action_available: bool,
    /// The canvas offers a write path (source-bound editable and support permits).
    pub editable: bool,
}

/// The resolved structure-tree row for a selected node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTreeRow {
    /// The selection identity — identical to the canvas frame and inspector.
    pub selection_id: String,
    /// The node label.
    pub node_label: String,
    /// The node kind.
    pub node_kind: M5StructureNodeKind,
    /// Whether the node is hidden.
    pub visibility_hidden: bool,
    /// Whether the node is locked.
    pub locked: bool,
    /// The support-state badge shown on the row.
    pub support_state: M5VisualSupportState,
    /// Selection stays synchronized with the canvas and source when the node maps
    /// to source.
    pub selection_synced: bool,
    /// The row is highlighted as a search match when the query matches the label.
    pub search_match_highlighted: bool,
    /// The direct open-source action is offered when the node maps to source.
    pub open_source_action_available: bool,
}

/// The resolved property-inspector row for one property of a selected node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPropertyRow {
    /// The property key.
    pub property_key: String,
    /// The semantic value state.
    pub value_state: M5PropertyValueState,
    /// The distinct editor control for this value state.
    pub editor_kind: M5PropertyEditorKind,
    /// The write scope an edit would take.
    pub write_scope: M5PropertyWriteScope,
    /// Whether an edit of this property actually writes source.
    pub writable: bool,
    /// The reset-to-default / reset-to-inherited action is offered.
    pub reset_action_available: bool,
    /// The open-source action is offered when the node maps to source.
    pub open_source_action_available: bool,
    /// A precise support-state note.
    pub support_note: String,
    /// Whether a write requires review first.
    pub requires_review: bool,
}

/// The resolved selected-node truth shared across canvas, tree, and inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedVisualSelection {
    /// The stable selection identity.
    pub selection_id: String,
    /// The resolved canvas frame.
    pub canvas_frame: M5ResolvedCanvasFrame,
    /// The resolved tree row.
    pub tree_row: M5ResolvedTreeRow,
    /// The resolved inspector rows, in input order.
    pub inspector_rows: Vec<M5ResolvedPropertyRow>,
    /// Source ownership (canvas state and, when mapped, source span) is disclosed.
    pub source_ownership_disclosed: bool,
    /// Support state is disclosed on every surface before mutation.
    pub support_state_disclosed: bool,
    /// Whether any edit path (canvas or a property) is offered.
    pub any_writable: bool,
}

impl M5ResolvedVisualSelection {
    /// True when the selection identity is identical across the canvas frame, the
    /// tree row, and the resolved selection (AC1).
    pub fn identity_consistent(&self) -> bool {
        self.canvas_frame.selection_id == self.selection_id
            && self.tree_row.selection_id == self.selection_id
    }

    /// True when every inspector row renders the distinct editor kind for its
    /// value state — the inspector never flattens distinct value states (AC2).
    pub fn value_states_distinguished(&self) -> bool {
        self.inspector_rows
            .iter()
            .all(|row| row.editor_kind == M5PropertyEditorKind::for_value_state(row.value_state))
    }

    /// True when any editable path is gated by disclosed source ownership and
    /// support state (AC3).
    pub fn writes_gated_by_disclosure(&self) -> bool {
        !self.any_writable || (self.source_ownership_disclosed && self.support_state_disclosed)
    }

    /// The distinct value states rendered in the inspector.
    pub fn distinct_value_states(&self) -> BTreeSet<M5PropertyValueState> {
        self.inspector_rows
            .iter()
            .map(|row| row.value_state)
            .collect()
    }
}

/// Errors returned by [`resolve_visual_selection`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5VisualSelectionResolutionError {
    /// The selection identity was empty.
    EmptySelectionId,
    /// The node label was empty.
    EmptyNodeLabel,
    /// The source-span ref presence disagreed with the node kind's mapping.
    SourceSpanMismatch,
    /// A property key was empty.
    EmptyPropertyKey,
    /// The same property key appeared more than once.
    DuplicatePropertyKey(String),
    /// A property's write scope was inconsistent with its value state.
    WriteScopeInconsistentWithValue,
    /// A property claimed a writing scope on a node that has not disclosed source
    /// ownership or whose support state forbids a write.
    MutationWithoutSourceOwnership,
    /// A value representation carried forbidden material.
    ForbiddenValueMaterial,
}

impl M5VisualSelectionResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySelectionId => "empty_selection_id",
            Self::EmptyNodeLabel => "empty_node_label",
            Self::SourceSpanMismatch => "source_span_mismatch",
            Self::EmptyPropertyKey => "empty_property_key",
            Self::DuplicatePropertyKey(_) => "duplicate_property_key",
            Self::WriteScopeInconsistentWithValue => "write_scope_inconsistent_with_value",
            Self::MutationWithoutSourceOwnership => "mutation_without_source_ownership",
            Self::ForbiddenValueMaterial => "forbidden_value_material",
        }
    }
}

impl fmt::Display for M5VisualSelectionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "selected-node resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5VisualSelectionResolutionError {}

/// Resolves one selected node into its shared canvas frame, tree row, and
/// inspector rows.
///
/// The resolved selection carries one selection identity on all three surfaces so
/// selection context is never lost when moving between canvas, tree, inspector,
/// and source. Each inspector row renders the distinct editor kind for its value
/// state so a token, a bound expression, an inherited value, and a literal never
/// collapse into one control. A property that claims a writing scope on a node
/// that has not disclosed source ownership — or whose support state forbids a
/// write — is refused rather than silently applied.
pub fn resolve_visual_selection(
    input: &M5VisualSelectionInput,
) -> Result<M5ResolvedVisualSelection, M5VisualSelectionResolutionError> {
    if input.selection_id.trim().is_empty() {
        return Err(M5VisualSelectionResolutionError::EmptySelectionId);
    }
    if input.node_label.trim().is_empty() {
        return Err(M5VisualSelectionResolutionError::EmptyNodeLabel);
    }

    let maps_to_source = input.node_kind.maps_to_source();
    match &input.source_span_ref {
        Some(span) if maps_to_source && !span.trim().is_empty() => {}
        None if !maps_to_source => {}
        _ => return Err(M5VisualSelectionResolutionError::SourceSpanMismatch),
    }

    // A node has disclosed source ownership when the canvas is source-bound and,
    // for a mapped node, its span is present. Support state must additionally
    // permit a write. Both gate any property mutation (AC3).
    let node_permits_write =
        input.canvas_state.is_editable() && input.support_state.permits_write();

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for property in &input.properties {
        if property.property_key.trim().is_empty() {
            return Err(M5VisualSelectionResolutionError::EmptyPropertyKey);
        }
        if !seen.insert(property.property_key.as_str()) {
            return Err(M5VisualSelectionResolutionError::DuplicatePropertyKey(
                property.property_key.clone(),
            ));
        }
        if value_repr_is_forbidden(&property.value_repr) {
            return Err(M5VisualSelectionResolutionError::ForbiddenValueMaterial);
        }
        if !property
            .write_scope
            .consistent_with_value(property.value_state)
        {
            return Err(M5VisualSelectionResolutionError::WriteScopeInconsistentWithValue);
        }
        // A writing scope on a node without disclosed source ownership, on a
        // support state that forbids writes, or on a blocked protected path, is
        // never resolved into a silent mutation.
        if property.write_scope.writes_source()
            && (!node_permits_write
                || property.protected_path_posture == ProtectedPathPosture::ProtectedBlocked)
        {
            return Err(M5VisualSelectionResolutionError::MutationWithoutSourceOwnership);
        }
    }

    if value_repr_is_forbidden(&input.node_label) {
        return Err(M5VisualSelectionResolutionError::ForbiddenValueMaterial);
    }

    let open_source_available = input.source_span_ref.is_some();

    let canvas_frame = M5ResolvedCanvasFrame {
        selection_id: input.selection_id.clone(),
        viewport: input.viewport,
        canvas_state: input.canvas_state,
        selected_node_outlined: true,
        support_state: input.support_state,
        open_source_action_available: open_source_available,
        editable: node_permits_write,
    };

    let tree_row = M5ResolvedTreeRow {
        selection_id: input.selection_id.clone(),
        node_label: input.node_label.clone(),
        node_kind: input.node_kind,
        visibility_hidden: input.visibility_hidden,
        locked: input.locked,
        support_state: input.support_state,
        selection_synced: maps_to_source,
        search_match_highlighted: search_matches(&input.search_query, &input.node_label),
        open_source_action_available: open_source_available,
    };

    let inspector_rows: Vec<M5ResolvedPropertyRow> = input
        .properties
        .iter()
        .map(|property| {
            let writable = property.write_scope.writes_source();
            M5ResolvedPropertyRow {
                property_key: property.property_key.clone(),
                value_state: property.value_state,
                editor_kind: M5PropertyEditorKind::for_value_state(property.value_state),
                write_scope: property.write_scope,
                writable,
                reset_action_available: value_is_resettable(property.value_state),
                open_source_action_available: open_source_available,
                support_note: input.support_state.note().to_owned(),
                requires_review: writable
                    && (input.support_state.requires_review()
                        || property.write_scope == M5PropertyWriteScope::TokenDefinitionShared
                        || property.protected_path_posture
                            == ProtectedPathPosture::ProtectedReviewRequired),
            }
        })
        .collect();

    let any_writable = canvas_frame.editable || inspector_rows.iter().any(|row| row.writable);

    Ok(M5ResolvedVisualSelection {
        selection_id: input.selection_id.clone(),
        canvas_frame,
        tree_row,
        inspector_rows,
        // Source ownership and support state are always disclosed structurally —
        // the canvas state, the support marker, and (for a mapped node) the span
        // are all rendered before any mutation is offered.
        source_ownership_disclosed: true,
        support_state_disclosed: true,
        any_writable,
    })
}

/// True when a value state has something to reset to a default / inherited value.
const fn value_is_resettable(value: M5PropertyValueState) -> bool {
    matches!(
        value,
        M5PropertyValueState::Literal
            | M5PropertyValueState::DesignToken
            | M5PropertyValueState::BoundExpression
            | M5PropertyValueState::Mixed
    )
}

/// Case-insensitive substring match for tree search-match highlighting.
fn search_matches(query: &Option<String>, label: &str) -> bool {
    match query {
        Some(q) if !q.trim().is_empty() => label.to_lowercase().contains(&q.trim().to_lowercase()),
        _ => false,
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs selected-node truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualSelectionCase {
    /// The resolver input.
    pub input: M5VisualSelectionInput,
    /// The resolved selected-node truth. Must equal
    /// `resolve_visual_selection(&input)`.
    pub resolved: M5ResolvedVisualSelection,
}

impl M5VisualSelectionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5VisualSelectionInput) -> Self {
        let resolved = resolve_visual_selection(&input).expect("seed selection case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_visual_selection(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one visual-design surface family bound to the
/// shared selected-node contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualDesignSurfaceRow {
    /// The visual-design surface family.
    pub surface_family: M5VisualDesignSurfaceFamily,
    /// The claimed preview surface this row maps onto (reused vocabulary).
    pub preview_surface: PreviewSurface,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Property value states this surface renders (must be non-empty).
    pub value_states: Vec<M5PropertyValueState>,
    /// Support states this surface can disclose (must be non-empty).
    pub support_states: Vec<M5VisualSupportState>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5SelectedNodeExportField>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5VisualDesignerDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection.
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_selections: Vec<M5VisualSelectionCase>,
    /// Hard invariant: this row never flattens distinct property value states.
    /// MUST be `false`.
    pub flattens_property_value_states: bool,
    /// Hard invariant: this row never loses selection identity across surfaces.
    /// MUST be `false`.
    pub loses_selection_identity_across_surfaces: bool,
    /// Hard invariant: this row never mutates before source ownership is
    /// disclosed. MUST be `false`.
    pub mutates_before_source_ownership_disclosed: bool,
    /// Hard invariant: this row never invents a private designer grammar. MUST be
    /// `false`.
    pub invents_private_designer_grammar: bool,
}

impl M5VisualDesignSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SelectedNodeExportField> =
            self.export_fields.iter().copied().collect();
        M5SelectedNodeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.flattens_property_value_states
            && !self.loses_selection_identity_across_surfaces
            && !self.mutates_before_source_ownership_disclosed
            && !self.invents_private_designer_grammar
    }
}

/// Self-describing controlled-vocabulary set minted by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectedNodeVocabularySet {
    /// Visual-design surface-family tokens.
    pub surface_families: Vec<String>,
    /// Support-state tokens.
    pub support_states: Vec<String>,
    /// Property-editor-kind tokens.
    pub editor_kinds: Vec<String>,
    /// Property-value-state tokens (reused from the frozen matrix).
    pub value_states: Vec<String>,
    /// Property-write-scope tokens (reused from the frozen matrix).
    pub write_scopes: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
}

impl M5SelectedNodeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5VisualDesignSurfaceFamily::ALL, |v| v.as_str()),
            support_states: tokens(&M5VisualSupportState::ALL, |v| v.as_str()),
            editor_kinds: tokens(&M5PropertyEditorKind::ALL, |v| v.as_str()),
            value_states: tokens(&VALUE_STATE_ALL, |v| v.as_str()),
            write_scopes: tokens(&WRITE_SCOPE_ALL, |v| v.as_str()),
            export_fields: tokens(&M5SelectedNodeExportField::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The property value states this primitive renders, in a stable order. The
/// frozen [`M5PropertyValueState`] enum is a pure token set, so the order is
/// pinned here.
const VALUE_STATE_ALL: [M5PropertyValueState; 6] = [
    M5PropertyValueState::Literal,
    M5PropertyValueState::DesignToken,
    M5PropertyValueState::BoundExpression,
    M5PropertyValueState::Inherited,
    M5PropertyValueState::Mixed,
    M5PropertyValueState::Unset,
];

/// The property write scopes this primitive binds against, in a stable order.
const WRITE_SCOPE_ALL: [M5PropertyWriteScope; 5] = [
    M5PropertyWriteScope::SingleLiteralSpan,
    M5PropertyWriteScope::TokenDefinitionShared,
    M5PropertyWriteScope::InstanceOverride,
    M5PropertyWriteScope::NoWriteInspectOnly,
    M5PropertyWriteScope::BlockedProtected,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectedNodeGovernanceReview {
    /// One selected-node primitive carries canvas / tree / inspector truth on
    /// every surface.
    pub one_primitive_carries_canvas_tree_inspector: bool,
    /// Selection identity is preserved across canvas, tree, inspector, and source.
    pub selection_identity_preserved_across_surfaces: bool,
    /// Property value states are never flattened into one ambiguous control.
    pub property_value_states_never_flattened: bool,
    /// Source ownership and support state are disclosed before mutation.
    pub source_ownership_and_support_disclosed_before_mutation: bool,
    /// The support / export packet reconstructs selected-node truth.
    pub support_export_reconstructs_selected_node: bool,
    /// No surface invents a second selected-node grammar.
    pub no_surface_invents_second_grammar: bool,
    /// Later M5 rows cannot invent parallel selected-node vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectedNodeConsumerProjection {
    /// Desktop / preview / browser / framework / shell / support surfaces all
    /// consume the shared primitive.
    pub visual_surfaces_consume_shared_primitive: bool,
    /// The selection resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The inspector reads a single canonical editor-kind source.
    pub inspector_reads_single_editor_kind_source: bool,
    /// Support / export reads a single canonical selected-node source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the selected-node primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectedNodeReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting selected-node audit.
    pub selected_node_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SelectedNodePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SelectedNodePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5VisualDesignSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SelectedNodeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SelectedNodeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SelectedNodeConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5SelectedNodeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 selected-node-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectedNodePrimitivePacket {
    /// Record kind; must equal [`M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5VisualDesignSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SelectedNodeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SelectedNodeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SelectedNodeConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5SelectedNodeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SelectedNodePrimitivePacket {
    /// Builds an M5 selected-node-primitive packet from stable-lane input.
    pub fn new(input: M5SelectedNodePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 selected-node-primitive invariants.
    pub fn validate(&self) -> Vec<M5SelectedNodePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND {
            violations.push(M5SelectedNodePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5SelectedNodePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SelectedNodePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 selected-node primitive packet serializes"),
        ) {
            violations.push(M5SelectedNodePrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 selected-node primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,preview_surface,owner,value_states,support_states,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.preview_surface.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.value_states, |v| v.as_str()),
                join_tokens(&row.support_states, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_selections.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Selected-Node Primitive: Canvas Frame, Tree Row, and Inspector Rows\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Visual-design surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5VisualDesignSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Support states: {}\n",
            self.vocabulary_set.support_states.join(", ")
        ));
        out.push_str(&format!(
            "- Editor kinds: {}\n",
            self.vocabulary_set.editor_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Value states: {}\n",
            self.vocabulary_set.value_states.join(", ")
        ));
        out.push_str("\n## Visual-design surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface_family.label(),
                row.preview_surface.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked selections: {}\n",
                row.example_selections.len()
            ));
            for case in &row.example_selections {
                out.push_str(&format!(
                    "    - `{}` → node `{}` ({}) support `{}`, {} inspector rows\n",
                    case.resolved.selection_id,
                    case.resolved.tree_row.node_kind.as_str(),
                    case.resolved.tree_row.node_label,
                    case.resolved.canvas_frame.support_state.as_str(),
                    case.resolved.inspector_rows.len(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 selected-node-primitive export.
#[derive(Debug)]
pub enum M5SelectedNodePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SelectedNodePrimitiveViolation>),
}

impl fmt::Display for M5SelectedNodePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 selected-node primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 selected-node primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SelectedNodePrimitiveArtifactError {}

/// Validation failures emitted by [`M5SelectedNodePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SelectedNodePrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required visual-design surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no property value states.
    ValueStateMissing,
    /// A surface row declares no support states.
    SupportStateMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked selection cases.
    ExampleSelectionMissing,
    /// A worked selection case does not match a fresh resolve of its input.
    ExampleSelectionDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked selection across the matrix proves selection identity preserved
    /// across canvas, tree, and inspector.
    IdentityPreservationUnproven,
    /// No worked selection proves two or more distinct value states rendered with
    /// distinct editor kinds.
    ValueStateDistinctionUnproven,
    /// No worked selection proves a mutation gated by source ownership and support
    /// state.
    SourceOwnershipGateUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SelectedNodePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::ValueStateMissing => "value_state_missing",
            Self::SupportStateMissing => "support_state_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleSelectionMissing => "example_selection_missing",
            Self::ExampleSelectionDrift => "example_selection_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::ValueStateDistinctionUnproven => "value_state_distinction_unproven",
            Self::SourceOwnershipGateUnproven => "source_ownership_gate_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 selected-node-primitive export.
pub fn current_stable_m5_selected_node_primitive_export(
) -> Result<M5SelectedNodePrimitivePacket, M5SelectedNodePrimitiveArtifactError> {
    let packet: M5SelectedNodePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-designer-selected-node-proof/support_export.json"
    )))
    .map_err(M5SelectedNodePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SelectedNodePrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SELECTED_NODE_SCHEMA_REF,
        M5_SELECTED_NODE_DOC_REF,
        M5_SELECTED_NODE_COMPONENT_MATRIX_REF,
        M5_SELECTED_NODE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SelectedNodePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SelectedNodePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let present: BTreeSet<M5VisualDesignSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5VisualDesignSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5SelectedNodePrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5SelectedNodePrimitiveViolation::SurfaceRowIncomplete);
        }
        if row.value_states.is_empty() {
            violations.push(M5SelectedNodePrimitiveViolation::ValueStateMissing);
        }
        if row.support_states.is_empty() {
            violations.push(M5SelectedNodePrimitiveViolation::SupportStateMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5SelectedNodePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SelectedNodePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SelectedNodePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.example_selections.is_empty() {
            violations.push(M5SelectedNodePrimitiveViolation::ExampleSelectionMissing);
        }
        if row
            .example_selections
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SelectedNodePrimitiveViolation::ExampleSelectionDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5SelectedNodePrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// The three acceptance criteria must each be demonstrated by at least one worked
/// selection across the matrix: identity preserved across surfaces (AC1), distinct
/// value states rendered with distinct editor kinds (AC2), and a mutation gated by
/// disclosed source ownership and support state (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let cases: Vec<&M5ResolvedVisualSelection> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_selections.iter().map(|case| &case.resolved))
        .collect();

    let identity_proven = cases
        .iter()
        .any(|resolved| !resolved.inspector_rows.is_empty() && resolved.identity_consistent());
    if !identity_proven {
        violations.push(M5SelectedNodePrimitiveViolation::IdentityPreservationUnproven);
    }

    let distinction_proven = cases.iter().any(|resolved| {
        resolved.value_states_distinguished() && resolved.distinct_value_states().len() >= 2
    });
    if !distinction_proven {
        violations.push(M5SelectedNodePrimitiveViolation::ValueStateDistinctionUnproven);
    }

    // A mutation gate is proven when a non-writable support state resolves with no
    // writable path, and every writable case discloses source ownership.
    let gate_proven = cases.iter().any(|resolved| {
        !resolved.canvas_frame.support_state.permits_write() && !resolved.any_writable
    }) && cases
        .iter()
        .all(|resolved| resolved.writes_gated_by_disclosure());
    if !gate_proven {
        violations.push(M5SelectedNodePrimitiveViolation::SourceOwnershipGateUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_canvas_tree_inspector,
        review.selection_identity_preserved_across_surfaces,
        review.property_value_states_never_flattened,
        review.source_ownership_and_support_disclosed_before_mutation,
        review.support_export_reconstructs_selected_node,
        review.no_surface_invents_second_grammar,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SelectedNodePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.visual_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.inspector_reads_single_editor_kind_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5SelectedNodePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5SelectedNodePrimitivePacket,
    violations: &mut Vec<M5SelectedNodePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.selected_node_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SelectedNodePrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single value representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in M5 selected-node-primitive packet. This is the
/// one source of truth shared by the tests, the example dump, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_selected_node_primitive_packet() -> M5SelectedNodePrimitivePacket {
    M5SelectedNodePrimitivePacket::new(M5SelectedNodePrimitivePacketInput {
        packet_id: "m5-visual-designer-selected-node-primitive:stable:0001".to_owned(),
        matrix_label: "M5 Selected-Node Primitive: Canvas Frame, Tree Row, and Inspector Rows"
            .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5SelectedNodeVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-03T00:00:00Z".to_owned(),
    })
}

fn all_export_fields() -> Vec<M5SelectedNodeExportField> {
    M5SelectedNodeExportField::ALL.to_vec()
}

/// A property edit input helper.
fn prop(
    key: &str,
    value_state: M5PropertyValueState,
    write_scope: M5PropertyWriteScope,
    protected: ProtectedPathPosture,
    value_repr: &str,
) -> M5PropertyEditInput {
    M5PropertyEditInput {
        property_key: key.to_owned(),
        value_state,
        write_scope,
        protected_path_posture: protected,
        value_repr: value_repr.to_owned(),
    }
}

fn seeded_surface_rows() -> Vec<M5VisualDesignSurfaceRow> {
    vec![
        // Desktop designer — a fully supported, source-bound editable element with
        // literal, design-token, and inherited properties rendered with distinct
        // editors (proves AC1 identity + AC2 distinction).
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::DesktopDesigner,
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            owner_role: "Visual Designer Platform".to_owned(),
            scope_summary:
                "Desktop designer canvas, structure tree, and property inspector for a source-bound element"
                    .to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
            ],
            consumer_surfaces: vec![
                "product_designer".to_owned(),
                "support_export".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_COMPONENT_MATRIX_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:desktop:hero-heading:0001".to_owned(),
                node_label: "HeroHeading".to_owned(),
                node_kind: M5StructureNodeKind::SourceElement,
                canvas_state: M5CanvasState::SourceBoundEditable,
                support_state: M5VisualSupportState::FullySupported,
                viewport: M5DevicePreviewClass::DesktopViewport,
                source_span_ref: Some("span:desktop:hero-heading".to_owned()),
                visibility_hidden: false,
                locked: false,
                search_query: Some("hero".to_owned()),
                properties: vec![
                    prop(
                        "font_size",
                        M5PropertyValueState::Literal,
                        M5PropertyWriteScope::SingleLiteralSpan,
                        ProtectedPathPosture::Unprotected,
                        "24px",
                    ),
                    prop(
                        "color",
                        M5PropertyValueState::DesignToken,
                        M5PropertyWriteScope::TokenDefinitionShared,
                        ProtectedPathPosture::ProtectedReviewRequired,
                        "token.color.brand.primary",
                    ),
                    prop(
                        "line_height",
                        M5PropertyValueState::Inherited,
                        M5PropertyWriteScope::NoWriteInspectOnly,
                        ProtectedPathPosture::Unprotected,
                        "inherited from body",
                    ),
                ],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
        // Source-first preview — a component instance with an instance override,
        // partially supported (edits need review).
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SourceFirstPreview,
            preview_surface: PreviewSurface::SourceFirstFrameworkPreview,
            owner_role: "Source-First Preview".to_owned(),
            scope_summary:
                "Source-first preview canvas and inspector for a reviewed component instance"
                    .to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
            ],
            consumer_surfaces: vec![
                "preview_runtime".to_owned(),
                "support_export".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_VISUAL_EDIT_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:preview:card-instance:0001".to_owned(),
                node_label: "PricingCard".to_owned(),
                node_kind: M5StructureNodeKind::ComponentInstance,
                canvas_state: M5CanvasState::SourceBoundEditable,
                support_state: M5VisualSupportState::PartiallySupported,
                viewport: M5DevicePreviewClass::TabletViewport,
                source_span_ref: Some("span:preview:card-instance".to_owned()),
                visibility_hidden: false,
                locked: false,
                search_query: None,
                properties: vec![
                    prop(
                        "padding",
                        M5PropertyValueState::Literal,
                        M5PropertyWriteScope::InstanceOverride,
                        ProtectedPathPosture::Unprotected,
                        "16px",
                    ),
                    prop(
                        "elevation",
                        M5PropertyValueState::Unset,
                        M5PropertyWriteScope::NoWriteInspectOnly,
                        ProtectedPathPosture::Unprotected,
                        "unset",
                    ),
                ],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
        // Browser-runtime inspector — a runtime-mirrored node with a bound
        // expression; inspect-only, so no write path (proves AC3 gate).
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::BrowserRuntimeInspector,
            preview_surface: PreviewSurface::BrowserRuntimeInspection,
            owner_role: "Browser Runtime Inspector".to_owned(),
            scope_summary:
                "Browser-runtime inspector for a runtime-mirrored node whose value is a bound expression"
                    .to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
                M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
            ],
            consumer_surfaces: vec![
                "browser_runtime".to_owned(),
                "diagnostics".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_VISUAL_EDIT_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:runtime:bound-badge:0001".to_owned(),
                node_label: "StatusBadge".to_owned(),
                node_kind: M5StructureNodeKind::ComponentInstance,
                canvas_state: M5CanvasState::RuntimeMirrored,
                support_state: M5VisualSupportState::InspectOnly,
                viewport: M5DevicePreviewClass::DesktopViewport,
                source_span_ref: Some("span:runtime:bound-badge".to_owned()),
                visibility_hidden: false,
                locked: false,
                search_query: None,
                properties: vec![
                    prop(
                        "label_text",
                        M5PropertyValueState::BoundExpression,
                        M5PropertyWriteScope::NoWriteInspectOnly,
                        ProtectedPathPosture::Unprotected,
                        "bound to status.label",
                    ),
                    prop(
                        "variant",
                        M5PropertyValueState::Mixed,
                        M5PropertyWriteScope::NoWriteInspectOnly,
                        ProtectedPathPosture::Unprotected,
                        "mixed across instances",
                    ),
                ],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
        // Framework-pack preview — a text leaf, fully supported literal edit.
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::FrameworkPackPreview,
            preview_surface: PreviewSurface::VisualEditTransform,
            owner_role: "Framework Packs".to_owned(),
            scope_summary:
                "Framework-pack preview inspector for a hand-authored text leaf".to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
                M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
            ],
            consumer_surfaces: vec![
                "framework_pack".to_owned(),
                "support_export".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_COMPONENT_MATRIX_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:framework:cta-label:0001".to_owned(),
                node_label: "CtaLabel".to_owned(),
                node_kind: M5StructureNodeKind::TextLeaf,
                canvas_state: M5CanvasState::SourceBoundEditable,
                support_state: M5VisualSupportState::FullySupported,
                viewport: M5DevicePreviewClass::MobileViewport,
                source_span_ref: Some("span:framework:cta-label".to_owned()),
                visibility_hidden: false,
                locked: false,
                search_query: Some("cta".to_owned()),
                properties: vec![prop(
                    "text_content",
                    M5PropertyValueState::Literal,
                    M5PropertyWriteScope::SingleLiteralSpan,
                    ProtectedPathPosture::Unprotected,
                    "Get started",
                )],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
        // Embedded shell designer — a source-bound read-only node (protected), so
        // no write path.
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::EmbeddedShellDesigner,
            preview_surface: PreviewSurface::EmbeddedWebviewPreview,
            owner_role: "Embedded Designer".to_owned(),
            scope_summary:
                "Embedded shell designer inspector for a protected, read-only source element"
                    .to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
                M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
            ],
            consumer_surfaces: vec![
                "app_shell".to_owned(),
                "support_export".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_COMPONENT_MATRIX_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:shell:generated-footer:0001".to_owned(),
                node_label: "GeneratedFooter".to_owned(),
                node_kind: M5StructureNodeKind::SourceElement,
                canvas_state: M5CanvasState::SourceBoundReadOnly,
                support_state: M5VisualSupportState::InspectOnly,
                viewport: M5DevicePreviewClass::DesktopViewport,
                source_span_ref: Some("span:shell:generated-footer".to_owned()),
                visibility_hidden: false,
                locked: true,
                search_query: None,
                properties: vec![prop(
                    "background",
                    M5PropertyValueState::DesignToken,
                    M5PropertyWriteScope::NoWriteInspectOnly,
                    ProtectedPathPosture::ProtectedBlocked,
                    "token.color.surface.footer",
                )],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
        // Support-export replay — an unmapped generated node with no source span;
        // inspect-only, tree not synced (proves unmapped disclosure + AC3 gate).
        M5VisualDesignSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SupportExportReplay,
            preview_surface: PreviewSurface::SupportExportProjection,
            owner_role: "Support Export".to_owned(),
            scope_summary:
                "Support-export replay of a captured selection for a loop-generated, unmapped node"
                    .to_owned(),
            value_states: VALUE_STATE_ALL.to_vec(),
            support_states: M5VisualSupportState::ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
                M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
            ],
            consumer_surfaces: vec![
                "support_export".to_owned(),
                "diagnostics".to_owned(),
            ],
            source_contract_refs: vec![M5_SELECTED_NODE_ARTIFACT_REF.to_owned()],
            example_selections: vec![M5VisualSelectionCase::resolved(M5VisualSelectionInput {
                selection_id: "selection:support:loop-item:0001".to_owned(),
                node_label: "ListItemGenerated".to_owned(),
                node_kind: M5StructureNodeKind::GeneratedNode,
                canvas_state: M5CanvasState::SnapshotStatic,
                support_state: M5VisualSupportState::UnmappedNode,
                viewport: M5DevicePreviewClass::CustomViewport,
                source_span_ref: None,
                visibility_hidden: false,
                locked: false,
                search_query: Some("nomatch".to_owned()),
                properties: vec![prop(
                    "gap",
                    M5PropertyValueState::Inherited,
                    M5PropertyWriteScope::NoWriteInspectOnly,
                    ProtectedPathPosture::Unprotected,
                    "inherited from list",
                )],
            })],
            flattens_property_value_states: false,
            loses_selection_identity_across_surfaces: false,
            mutates_before_source_ownership_disclosed: false,
            invents_private_designer_grammar: false,
        },
    ]
}

fn seeded_governance_review() -> M5SelectedNodeGovernanceReview {
    M5SelectedNodeGovernanceReview {
        one_primitive_carries_canvas_tree_inspector: true,
        selection_identity_preserved_across_surfaces: true,
        property_value_states_never_flattened: true,
        source_ownership_and_support_disclosed_before_mutation: true,
        support_export_reconstructs_selected_node: true,
        no_surface_invents_second_grammar: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5SelectedNodeConsumerProjection {
    M5SelectedNodeConsumerProjection {
        visual_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        inspector_reads_single_editor_kind_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5SelectedNodeReleasePosture {
    M5SelectedNodeReleasePosture {
        release_packet_ref:
            "artifacts/release/m5-visual-designer-selected-node-proof/support_export.json"
                .to_owned(),
        selected_node_audit_ref:
            "artifacts/components/m5-visual-designer-selected-node-primitive.md".to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        M5_SELECTED_NODE_SCHEMA_REF.to_owned(),
        M5_SELECTED_NODE_DOC_REF.to_owned(),
        M5_SELECTED_NODE_COMPONENT_MATRIX_REF.to_owned(),
        M5_SELECTED_NODE_ARTIFACT_REF.to_owned(),
        M5_SELECTED_NODE_VISUAL_EDIT_REF.to_owned(),
    ]
}
