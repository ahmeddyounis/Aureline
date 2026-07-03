//! Frozen reusable visual-designer component matrix: design canvas, structure-tree
//! rows, property-inspector rows, source-sync chips, breakpoint/device-preview
//! rows, unsupported-construct cards, and round-trip conflict banners.
//!
//! Where
//! [`crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix`]
//! freezes the *qualification* of each claimed preview/runtime surface,
//! [`crate::preview_session_descriptors`] materializes the *per-session* state,
//! [`crate::inspect_to_source_tree`] materializes the *per-node* source-mapping
//! truth, and [`crate::visual_edit_transforms`] materializes the *per-edit*
//! round-trip truth, this module freezes the reusable **visual-designer
//! component** contract: the canvas, tree, inspector, chip, and preview-row
//! primitives users actually touch, so later M5 rows reference one canonical
//! component family instead of restating visual-designer truth in feature-local
//! prose.
//!
//! One [`VisualDesignerComponentMatrix`] packet defines every reusable primitive,
//! its state vocabulary, its required labels, and its export / assistive parity
//! expectations, binding each onto the same write-scope, preview/apply/revert,
//! citation, and degraded-state vocabulary already used by source-first preview,
//! AI apply, and refactor flows — never bespoke per-provider designer chrome. It
//! reuses [`crate::SourceSyncClass`], [`crate::RoundTripCapabilityClass`],
//! [`crate::PreviewSurface`], [`crate::ProtectedPathPosture`],
//! [`crate::PreviewDiffClass`], [`crate::MutationReviewPosture`], and the shared
//! [`crate::UnsupportedConstructCard`] rather than minting parallel synonyms.
//!
//! The honesty rules the spec freezes, carried by every [`ComponentRow`]:
//!
//! - **Source stays canonical; canvas state is derivative and explicit.** A
//!   design-canvas row must declare it is derivative of source and name whether
//!   it is source-bound editable, source-bound read-only, runtime-mirrored, or a
//!   detached snapshot — it never becomes a second writable truth model.
//! - **Tree, canvas, and source selection stay synchronized.** A structure-tree
//!   row that maps to a source span keeps selection synchronized; an unmapped
//!   node must disclose that it has no source span rather than fake a mapping.
//! - **Property editors distinguish value state and never widen write scope
//!   silently.** A property-inspector row names whether the value is a literal,
//!   design token, bound expression, inherited, or mixed value, and names the
//!   exact write scope an edit takes — a token-definition edit can never
//!   masquerade as a single-literal edit.
//! - **Unsupported constructs, generated / protected files, and round-trip
//!   conflicts never collapse into silent writeback.** They degrade to an
//!   [`crate::UnsupportedConstructCard`] or a round-trip conflict banner that
//!   names a real resolution route instead of guessing.
//! - **Breakpoint / device previews keep runtime origin, live-versus-mock
//!   posture, and mapping quality visible.** A device-preview row names its
//!   viewport / device class, whether it is showing live or mock data, and
//!   whether its source mapping is exact, approximate, or unmapped.
//!
//! Raw source bodies, diff hunks, file contents, credentials, and raw provider
//! payloads never cross this boundary; the packet carries only typed class
//! tokens, opaque span / selection / evidence refs, booleans, and redacted
//! labels, so support and diagnostics exports can reconstruct exactly what a
//! component would have shown without leaking source.
//!
//! The boundary schema is
//! [`schemas/ui/m5-visual-designer-component-matrix.schema.json`](../../../../schemas/ui/m5-visual-designer-component-matrix.schema.json).
//! The contract doc is
//! [`docs/preview/m5/m5_visual_designer_component_matrix.md`](../../../../docs/preview/m5/m5_visual_designer_component_matrix.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-visual-designer-components/`](../../../../fixtures/ui/m5-visual-designer-components/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    MutationReviewPosture, PreviewDiffClass, PreviewSurface, ProtectedPathPosture,
    RoundTripCapabilityClass, SourceSyncClass, UnsupportedConstructCard,
};

/// Stable record-kind tag carried by [`VisualDesignerComponentMatrix`].
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_RECORD_KIND: &str =
    "m5_visual_designer_component_matrix";

/// Schema version for the visual-designer component matrix packet.
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_DOC_REF: &str =
    "docs/preview/m5/m5_visual_designer_component_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ui/m5-visual-designer-components";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF: &str =
    "artifacts/preview/m5/m5_visual_designer_component_matrix/support_export.json";

/// Repo-relative path of the checked Markdown matrix summary.
pub const VISUAL_DESIGNER_COMPONENT_MATRIX_SUMMARY_REF: &str =
    "artifacts/design/m5-visual-designer-component-matrix.md";

/// Closed reusable visual-designer component family. Each family is one governed
/// primitive later M5 rows reference by name; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignerComponentFamily {
    /// The visual design canvas the user edits on.
    DesignCanvas,
    /// A row in the structure / layers tree.
    StructureTreeRow,
    /// A row in the property inspector.
    PropertyInspectorRow,
    /// A source-sync chip disclosing how a surface relates to canonical source.
    SourceSyncChip,
    /// A breakpoint / device-preview row.
    BreakpointPreviewRow,
    /// An unsupported-construct card the surface degrades to.
    UnsupportedConstructCard,
    /// A round-trip conflict banner shown when source changed under an edit.
    RoundTripConflictBanner,
}

impl M5VisualDesignerComponentFamily {
    /// Every reusable component family the matrix must define, in declaration
    /// order.
    pub const ALL: [Self; 7] = [
        Self::DesignCanvas,
        Self::StructureTreeRow,
        Self::PropertyInspectorRow,
        Self::SourceSyncChip,
        Self::BreakpointPreviewRow,
        Self::UnsupportedConstructCard,
        Self::RoundTripConflictBanner,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignCanvas => "design_canvas",
            Self::StructureTreeRow => "structure_tree_row",
            Self::PropertyInspectorRow => "property_inspector_row",
            Self::SourceSyncChip => "source_sync_chip",
            Self::BreakpointPreviewRow => "breakpoint_preview_row",
            Self::UnsupportedConstructCard => "unsupported_construct_card",
            Self::RoundTripConflictBanner => "round_trip_conflict_banner",
        }
    }
}

/// Closed design-canvas state vocabulary. Names how the canvas relates to
/// canonical source so it can never quietly become a second writable model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CanvasState {
    /// Bound to source and editable through the shared preview/apply/revert path.
    SourceBoundEditable,
    /// Bound to source but read-only (e.g. protected or inspect-only).
    SourceBoundReadOnly,
    /// Mirrored from a live runtime with no saved-source write path.
    RuntimeMirrored,
    /// A pinned static snapshot projection.
    SnapshotStatic,
    /// Detached from source and needing an explicit resync before edits.
    DetachedNeedsResync,
}

impl M5CanvasState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceBoundEditable => "source_bound_editable",
            Self::SourceBoundReadOnly => "source_bound_read_only",
            Self::RuntimeMirrored => "runtime_mirrored",
            Self::SnapshotStatic => "snapshot_static",
            Self::DetachedNeedsResync => "detached_needs_resync",
        }
    }

    /// True when the canvas offers a write path back to source.
    pub const fn is_editable(self) -> bool {
        matches!(self, Self::SourceBoundEditable)
    }
}

/// A design-canvas descriptor. Present only on a [`M5VisualDesignerComponentFamily::DesignCanvas`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignCanvasDescriptor {
    /// How the canvas relates to canonical source.
    pub canvas_state: M5CanvasState,
    /// Canvas state is derivative of source, never a second writable truth model;
    /// must always hold.
    pub is_derivative_of_source: bool,
    /// The canvas keeps its selection synchronized with the tree and source
    /// selection; must always hold.
    pub selection_synced_with_tree_and_source: bool,
    /// Opaque ref to the canonical source revision the canvas derives from; never
    /// raw source bytes.
    pub source_revision_ref: String,
}

impl DesignCanvasDescriptor {
    /// Whether the canvas descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        self.is_derivative_of_source
            && self.selection_synced_with_tree_and_source
            && !self.source_revision_ref.trim().is_empty()
    }
}

/// Closed structure-node kind vocabulary. Names what a tree row maps to so an
/// unmapped node can never fake a source span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StructureNodeKind {
    /// A hand-authored source element with an exact span.
    SourceElement,
    /// A component / widget instance backed by a source span.
    ComponentInstance,
    /// A slot / children projection backed by source.
    SlotOrChildren,
    /// A text leaf backed by source.
    TextLeaf,
    /// A node generated by a conditional / loop with no single authored span.
    GeneratedNode,
    /// A node with no resolvable source span at all.
    UnmappedNode,
}

impl M5StructureNodeKind {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceElement => "source_element",
            Self::ComponentInstance => "component_instance",
            Self::SlotOrChildren => "slot_or_children",
            Self::TextLeaf => "text_leaf",
            Self::GeneratedNode => "generated_node",
            Self::UnmappedNode => "unmapped_node",
        }
    }

    /// True when this node kind resolves to a hand-authored source span.
    pub const fn maps_to_source(self) -> bool {
        matches!(
            self,
            Self::SourceElement | Self::ComponentInstance | Self::SlotOrChildren | Self::TextLeaf
        )
    }
}

/// A structure-tree row descriptor. Present only on a
/// [`M5VisualDesignerComponentFamily::StructureTreeRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructureTreeRowDescriptor {
    /// What kind of node this row is.
    pub node_kind: M5StructureNodeKind,
    /// Whether this row resolves to a canonical source span. Must equal
    /// [`M5StructureNodeKind::maps_to_source`] — an unmapped node may not claim a
    /// mapping.
    pub maps_to_source_span: bool,
    /// Selection stays synchronized with the canvas and source when the node maps
    /// to source; must hold for mapped nodes.
    pub selection_synced_with_canvas_and_source: bool,
    /// Opaque ref to the source span, present only for a mapped node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_ref: Option<String>,
}

impl StructureTreeRowDescriptor {
    /// Whether the tree row descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        if self.maps_to_source_span != self.node_kind.maps_to_source() {
            return false;
        }
        if self.node_kind.maps_to_source() {
            self.selection_synced_with_canvas_and_source
                && self
                    .source_span_ref
                    .as_ref()
                    .is_some_and(|r| !r.trim().is_empty())
        } else {
            // An unmapped / generated node discloses no span and never claims a
            // synchronized source selection.
            self.source_span_ref.is_none()
        }
    }
}

/// Closed property-value-state vocabulary. Names the semantic state of an edited
/// value so a token, a bound expression, an inherited value, and a literal are
/// never conflated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PropertyValueState {
    /// A literal value written directly on this element.
    Literal,
    /// A value resolved from a shared design token.
    DesignToken,
    /// A value bound to a runtime / dynamic expression.
    BoundExpression,
    /// A value inherited from an ancestor / theme / base style.
    Inherited,
    /// A mixed value across a multi-selection.
    Mixed,
    /// No value set (falls back to default).
    Unset,
}

impl M5PropertyValueState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Literal => "literal",
            Self::DesignToken => "design_token",
            Self::BoundExpression => "bound_expression",
            Self::Inherited => "inherited",
            Self::Mixed => "mixed",
            Self::Unset => "unset",
        }
    }

    /// True when a direct visual edit of this value can map to a single literal
    /// source span.
    pub const fn is_single_literal(self) -> bool {
        matches!(self, Self::Literal | Self::Unset)
    }
}

/// Closed property write-scope vocabulary. Names exactly what an edit would write
/// so a property edit can never widen its scope silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PropertyWriteScope {
    /// The edit writes a single literal source span.
    SingleLiteralSpan,
    /// The edit changes a shared token definition and so affects every consumer.
    TokenDefinitionShared,
    /// The edit writes an instance-level override rather than the shared value.
    InstanceOverride,
    /// The value is inspect-only and takes no write.
    NoWriteInspectOnly,
    /// The target is a blocked protected path and takes no write.
    BlockedProtected,
}

impl M5PropertyWriteScope {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleLiteralSpan => "single_literal_span",
            Self::TokenDefinitionShared => "token_definition_shared",
            Self::InstanceOverride => "instance_override",
            Self::NoWriteInspectOnly => "no_write_inspect_only",
            Self::BlockedProtected => "blocked_protected",
        }
    }

    /// True when this scope actually writes source.
    pub const fn writes_source(self) -> bool {
        matches!(
            self,
            Self::SingleLiteralSpan | Self::TokenDefinitionShared | Self::InstanceOverride
        )
    }

    /// Whether this write scope is consistent with a value state: a shared token
    /// value must write a shared or instance scope (never masquerade as a single
    /// literal span), and a bound expression can never take a silent write.
    pub const fn consistent_with_value(self, value: M5PropertyValueState) -> bool {
        match value {
            // A design-token value edited in place either changes the shared
            // definition or forks an instance override; it can never be recorded
            // as a single literal span.
            M5PropertyValueState::DesignToken => matches!(
                self,
                Self::TokenDefinitionShared | Self::InstanceOverride | Self::NoWriteInspectOnly
            ),
            // A bound expression, an inherited value, or a mixed multi-selection
            // never round-trips to one literal span; it degrades to inspect-only or
            // a protected block rather than a silent write.
            M5PropertyValueState::BoundExpression
            | M5PropertyValueState::Inherited
            | M5PropertyValueState::Mixed => {
                matches!(self, Self::NoWriteInspectOnly | Self::BlockedProtected)
            }
            // A literal / unset value may write its own literal span or degrade.
            M5PropertyValueState::Literal | M5PropertyValueState::Unset => true,
        }
    }
}

/// A property-inspector row descriptor. Present only on a
/// [`M5VisualDesignerComponentFamily::PropertyInspectorRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyInspectorRowDescriptor {
    /// The semantic state of the value.
    pub value_state: M5PropertyValueState,
    /// The exact write scope an edit of this value would take.
    pub write_scope: M5PropertyWriteScope,
    /// How a protected target gates the write (reused shared vocabulary).
    pub protected_path_posture: ProtectedPathPosture,
    /// The diff the user is shown before a write commits; a writing scope shows a
    /// real source diff and an inspect-only scope shows none.
    pub preview_diff: PreviewDiffClass,
    /// The review / confirmation a write requires; present only for a writing
    /// scope.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_posture: Option<MutationReviewPosture>,
    /// The edit never widens its write scope beyond the recorded scope; must
    /// always hold.
    pub widens_write_scope_silently: bool,
}

impl PropertyInspectorRowDescriptor {
    /// Whether the property row descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        if self.widens_write_scope_silently {
            return false;
        }
        if !self.write_scope.consistent_with_value(self.value_state) {
            return false;
        }
        // A blocked protected path can never carry a writing scope.
        if self.protected_path_posture == ProtectedPathPosture::ProtectedBlocked
            && self.write_scope.writes_source()
        {
            return false;
        }
        if self.write_scope.writes_source() {
            self.preview_diff.is_real_source_diff() && self.review_posture.is_some()
        } else {
            self.preview_diff == PreviewDiffClass::NoDiffInspectOnly
                && self.review_posture.is_none()
        }
    }
}

/// Closed source-sync recovery-route vocabulary. Names how a drifted / runtime-only
/// chip recovers so a stale surface always points at a real next step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyncRecoveryRoute {
    /// Nothing to recover; the surface is in sync from source.
    NoneInSync,
    /// Rebuild the preview from the canonical source.
    RebuildFromSource,
    /// Re-resolve the source mapping that drifted.
    ResyncMapping,
    /// Reattach the runtime backing the view.
    ReattachRuntime,
    /// Inspect-only; no recovery to a write path is offered.
    InspectOnlyNoRecovery,
}

impl M5SyncRecoveryRoute {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneInSync => "none_in_sync",
            Self::RebuildFromSource => "rebuild_from_source",
            Self::ResyncMapping => "resync_mapping",
            Self::ReattachRuntime => "reattach_runtime",
            Self::InspectOnlyNoRecovery => "inspect_only_no_recovery",
        }
    }
}

/// A source-sync chip descriptor. Present only on a
/// [`M5VisualDesignerComponentFamily::SourceSyncChip`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSyncChipDescriptor {
    /// The reused source-sync state the chip discloses.
    pub sync_class: SourceSyncClass,
    /// The recovery route offered; must be consistent with the sync class.
    pub recovery_route: M5SyncRecoveryRoute,
}

impl SourceSyncChipDescriptor {
    /// Whether the chip descriptor's recovery route is consistent with its sync
    /// class.
    pub fn is_honest(&self) -> bool {
        match self.sync_class {
            SourceSyncClass::InSyncFromSource => {
                self.recovery_route == M5SyncRecoveryRoute::NoneInSync
            }
            SourceSyncClass::PendingRebuild | SourceSyncClass::DriftedFromSource => matches!(
                self.recovery_route,
                M5SyncRecoveryRoute::RebuildFromSource | M5SyncRecoveryRoute::ResyncMapping
            ),
            SourceSyncClass::RuntimeOnlyNoSource => matches!(
                self.recovery_route,
                M5SyncRecoveryRoute::ReattachRuntime | M5SyncRecoveryRoute::InspectOnlyNoRecovery
            ),
            SourceSyncClass::UnidentifiedSourceSync => {
                self.recovery_route == M5SyncRecoveryRoute::InspectOnlyNoRecovery
            }
        }
    }
}

/// Closed device / breakpoint-preview class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DevicePreviewClass {
    /// A desktop viewport preset.
    DesktopViewport,
    /// A tablet viewport preset.
    TabletViewport,
    /// A mobile viewport preset.
    MobileViewport,
    /// A custom viewport size.
    CustomViewport,
    /// A tethered physical device.
    DeviceTethered,
    /// A simulator / emulator preview.
    SimulatorPreview,
}

impl M5DevicePreviewClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopViewport => "desktop_viewport",
            Self::TabletViewport => "tablet_viewport",
            Self::MobileViewport => "mobile_viewport",
            Self::CustomViewport => "custom_viewport",
            Self::DeviceTethered => "device_tethered",
            Self::SimulatorPreview => "simulator_preview",
        }
    }
}

/// Closed preview-data-posture vocabulary. Names whether a device preview is
/// showing live or mock data so the live-versus-mock posture is never blurred.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreviewDataPosture {
    /// Live data from a real runtime.
    Live,
    /// Mock / fixture data.
    Mock,
    /// Captured / replayed data from a pinned snapshot.
    Captured,
}

impl M5PreviewDataPosture {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Mock => "mock",
            Self::Captured => "captured",
        }
    }
}

/// Closed breakpoint mapping-quality vocabulary. Names how well a device preview
/// maps back to source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreakpointMappingQuality {
    /// Exact source mapping.
    Exact,
    /// Approximate source mapping.
    Approximate,
    /// No resolvable source mapping.
    Unmapped,
}

impl M5BreakpointMappingQuality {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
            Self::Unmapped => "unmapped",
        }
    }
}

/// A breakpoint / device-preview row descriptor. Present only on a
/// [`M5VisualDesignerComponentFamily::BreakpointPreviewRow`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointPreviewRowDescriptor {
    /// The device / viewport class.
    pub device_class: M5DevicePreviewClass,
    /// Whether the preview shows live, mock, or captured data.
    pub data_posture: M5PreviewDataPosture,
    /// How well the preview maps back to source.
    pub mapping_quality: M5BreakpointMappingQuality,
    /// Opaque token naming the runtime origin the preview renders from; never a
    /// raw URL or path. Keeps runtime origin visible.
    pub runtime_origin_token: String,
}

impl BreakpointPreviewRowDescriptor {
    /// Whether the breakpoint descriptor is internally complete and honest: it
    /// keeps a runtime origin visible, and an unmapped preview never hides its
    /// unmapped mapping quality.
    pub fn is_honest(&self) -> bool {
        !self.runtime_origin_token.trim().is_empty()
    }
}

/// Closed round-trip conflict class vocabulary. Names why a round-trip conflict
/// banner appeared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoundTripConflictClass {
    /// The canonical source changed underneath an in-flight edit.
    SourceChangedUnderEdit,
    /// The target is a generated / protected file that blocks writeback.
    GeneratedFileProtected,
    /// The source mapping is ambiguous and a write could land wrong.
    AmbiguousMapping,
    /// A concurrent external edit touched the same span.
    ConcurrentExternalEdit,
    /// The transform would be lossy and was refused.
    LossyTransformRefused,
}

impl M5RoundTripConflictClass {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceChangedUnderEdit => "source_changed_under_edit",
            Self::GeneratedFileProtected => "generated_file_protected",
            Self::AmbiguousMapping => "ambiguous_mapping",
            Self::ConcurrentExternalEdit => "concurrent_external_edit",
            Self::LossyTransformRefused => "lossy_transform_refused",
        }
    }
}

/// Closed round-trip conflict resolution-route vocabulary. Every conflict names a
/// real resolution route rather than a silent writeback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConflictResolutionRoute {
    /// Reload the canonical source, then re-apply the edit.
    ReloadSourceReapply,
    /// Keep source and discard the visual change.
    KeepSourceDiscardVisual,
    /// Open the source for a manual merge.
    OpenSourceManualMerge,
    /// Inspect-only; the surface takes no write at all.
    InspectOnlyNoWrite,
}

impl M5ConflictResolutionRoute {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReloadSourceReapply => "reload_source_reapply",
            Self::KeepSourceDiscardVisual => "keep_source_discard_visual",
            Self::OpenSourceManualMerge => "open_source_manual_merge",
            Self::InspectOnlyNoWrite => "inspect_only_no_write",
        }
    }
}

/// A round-trip conflict banner descriptor. Present only on a
/// [`M5VisualDesignerComponentFamily::RoundTripConflictBanner`] row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripConflictBannerDescriptor {
    /// Why the conflict banner appeared.
    pub conflict_class: M5RoundTripConflictClass,
    /// The resolution route offered.
    pub resolution_route: M5ConflictResolutionRoute,
    /// The conflict never collapses into a silent writeback; must always hold.
    pub never_silent_writeback: bool,
    /// The edit's selection context is preserved across the conflict; must hold.
    pub preserves_selection_context: bool,
}

impl RoundTripConflictBannerDescriptor {
    /// Whether the conflict banner descriptor is internally complete and honest.
    pub fn is_honest(&self) -> bool {
        self.never_silent_writeback && self.preserves_selection_context
    }
}

/// Closed required-label vocabulary. Names the labels a reusable visual-designer
/// component must render; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignerRequiredLabel {
    /// The component's stable identity.
    Identity,
    /// Which surface / source the component owns or derives from.
    SourceOwnership,
    /// The component's current state.
    State,
    /// The source-sync or freshness posture.
    SyncOrFreshness,
    /// The keyboard / assistive route into the component.
    KeyboardRoute,
}

impl M5VisualDesignerRequiredLabel {
    /// Every required label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Identity,
        Self::SourceOwnership,
        Self::State,
        Self::SyncOrFreshness,
        Self::KeyboardRoute,
    ];

    /// The mandatory subset that must appear on every row.
    pub const MANDATORY: [Self; 4] = [
        Self::Identity,
        Self::SourceOwnership,
        Self::State,
        Self::KeyboardRoute,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SourceOwnership => "source_ownership",
            Self::State => "state",
            Self::SyncOrFreshness => "sync_or_freshness",
            Self::KeyboardRoute => "keyboard_route",
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a component row is in a degraded
/// state so support can reconstruct the narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualDesignerDowngradeTrigger {
    /// The surface drifted from canonical source.
    DriftedFromSource,
    /// The source mapping could not be resolved.
    UnmappedSource,
    /// The runtime backing the view is unavailable.
    RuntimeUnavailable,
    /// The target is a blocked protected path.
    ProtectedPathBlocked,
    /// The construct is unsupported for a round-trip write.
    UnsupportedConstruct,
    /// A round-trip conflict is open.
    RoundTripConflictOpen,
    /// The data posture is not yet identified.
    UnidentifiedPosture,
}

impl M5VisualDesignerDowngradeTrigger {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DriftedFromSource => "drifted_from_source",
            Self::UnmappedSource => "unmapped_source",
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::ProtectedPathBlocked => "protected_path_blocked",
            Self::UnsupportedConstruct => "unsupported_construct",
            Self::RoundTripConflictOpen => "round_trip_conflict_open",
            Self::UnidentifiedPosture => "unidentified_posture",
        }
    }
}

/// A typed degraded-state block. When present, the component is narrowed below its
/// full capability and names why with an explicit, non-generic label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DegradedState {
    /// Why the component is degraded.
    pub trigger: M5VisualDesignerDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub degraded_label: String,
}

impl DegradedState {
    /// Whether the degraded label is precise rather than a generic non-answer.
    pub fn is_honest(&self) -> bool {
        !label_is_generic(&self.degraded_label)
    }
}

/// One reusable visual-designer component: the shared truth row every consumer
/// surface ingests instead of cloning designer chrome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentRow {
    /// Stable component id.
    pub component_id: String,
    /// Which reusable component family this row is.
    pub family: M5VisualDesignerComponentFamily,
    /// Human-readable label of the surface the component appears on.
    pub surface_label: String,
    /// The claimed preview surface this component belongs to (reused vocabulary).
    pub preview_surface: PreviewSurface,
    /// The source-sync posture the component discloses (reused vocabulary).
    pub source_sync: SourceSyncClass,
    /// The round-trip capability the component's surface claims (reused
    /// vocabulary).
    pub round_trip: RoundTripCapabilityClass,
    /// The required labels this component renders; must include every mandatory
    /// label.
    pub required_labels: Vec<M5VisualDesignerRequiredLabel>,
    /// The component projects an export-safe support summary; must hold.
    pub export_safe: bool,
    /// The component exposes a keyboard / assistive route; must hold.
    pub assistive_ready: bool,
    /// The design-canvas descriptor, present only for a canvas row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub design_canvas: Option<DesignCanvasDescriptor>,
    /// The structure-tree descriptor, present only for a tree-row row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structure_tree_row: Option<StructureTreeRowDescriptor>,
    /// The property-inspector descriptor, present only for a property-row row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub property_inspector_row: Option<PropertyInspectorRowDescriptor>,
    /// The source-sync chip descriptor, present only for a chip row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sync_chip: Option<SourceSyncChipDescriptor>,
    /// The breakpoint-preview descriptor, present only for a breakpoint-row row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakpoint_preview_row: Option<BreakpointPreviewRowDescriptor>,
    /// The unsupported-construct card (reused struct), present only for a card row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported_construct_card: Option<UnsupportedConstructCard>,
    /// The round-trip conflict banner, present only for a conflict-banner row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round_trip_conflict_banner: Option<RoundTripConflictBannerDescriptor>,
    /// The typed degraded-state block, present only when the component is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
    /// Human-readable label summary safe to render on the row.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the component state was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
}

impl ComponentRow {
    /// Whether the family-specific payload is present exactly for this family and
    /// absent for every other family.
    pub fn payload_matches_family(&self) -> bool {
        let present = [
            self.design_canvas.is_some(),
            self.structure_tree_row.is_some(),
            self.property_inspector_row.is_some(),
            self.source_sync_chip.is_some(),
            self.breakpoint_preview_row.is_some(),
            self.unsupported_construct_card.is_some(),
            self.round_trip_conflict_banner.is_some(),
        ];
        // Exactly one payload present, and it is the one this family names.
        if present.iter().filter(|p| **p).count() != 1 {
            return false;
        }
        match self.family {
            M5VisualDesignerComponentFamily::DesignCanvas => self.design_canvas.is_some(),
            M5VisualDesignerComponentFamily::StructureTreeRow => self.structure_tree_row.is_some(),
            M5VisualDesignerComponentFamily::PropertyInspectorRow => {
                self.property_inspector_row.is_some()
            }
            M5VisualDesignerComponentFamily::SourceSyncChip => self.source_sync_chip.is_some(),
            M5VisualDesignerComponentFamily::BreakpointPreviewRow => {
                self.breakpoint_preview_row.is_some()
            }
            M5VisualDesignerComponentFamily::UnsupportedConstructCard => {
                self.unsupported_construct_card.is_some()
            }
            M5VisualDesignerComponentFamily::RoundTripConflictBanner => {
                self.round_trip_conflict_banner.is_some()
            }
        }
    }

    /// Whether the family payload, where present, is internally honest.
    pub fn payload_honest(&self) -> bool {
        self.design_canvas.as_ref().map_or(true, |d| d.is_honest())
            && self
                .structure_tree_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .property_inspector_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .source_sync_chip
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .breakpoint_preview_row
                .as_ref()
                .map_or(true, |d| d.is_honest())
            && self
                .unsupported_construct_card
                .as_ref()
                .map_or(true, |d| d.is_complete())
            && self
                .round_trip_conflict_banner
                .as_ref()
                .map_or(true, |d| d.is_honest())
    }

    /// Whether the source-sync chip, when present, discloses the same source-sync
    /// class the row records (the chip never invents a second sync story).
    pub fn chip_matches_row_sync(&self) -> bool {
        self.source_sync_chip
            .as_ref()
            .map_or(true, |c| c.sync_class == self.source_sync)
    }

    /// Whether every mandatory required label is present on the row.
    pub fn mandatory_labels_present(&self) -> bool {
        let present: BTreeSet<M5VisualDesignerRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5VisualDesignerRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the degraded block, when present, is honest.
    pub fn degraded_ok(&self) -> bool {
        self.degraded.as_ref().map_or(true, |d| d.is_honest())
    }

    /// True when this row is a complete, honest degraded / narrowed component.
    pub fn is_degraded(&self) -> bool {
        self.degraded.is_some() && self.is_complete()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} surface={surface} sync={sync} round_trip={round_trip} \
export_safe={export_safe} assistive={assistive}",
            family = self.family.as_str(),
            surface = self.preview_surface.as_str(),
            sync = self.source_sync.as_str(),
            round_trip = self.round_trip.as_str(),
            export_safe = self.export_safe,
            assistive = self.assistive_ready,
        )
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.component_id.trim().is_empty()
            && !self.surface_label.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.export_safe
            && self.assistive_ready
            && self.payload_matches_family()
            && self.payload_honest()
            && self.chip_matches_row_sync()
            && self.mandatory_labels_present()
            && self.degraded_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block for the visual-designer component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerGuardrails {
    /// Source remains canonical; no component is a second writable truth model.
    pub source_remains_canonical: bool,
    /// Canvas state stays derivative and explicit.
    pub canvas_state_derivative_and_explicit: bool,
    /// Tree, canvas, and source selection stay synchronized.
    pub tree_canvas_source_selection_synchronized: bool,
    /// Property editors distinguish token / bound-expression / inherited / literal
    /// state and never widen write scope silently.
    pub property_editors_distinguish_value_state_no_silent_widening: bool,
    /// Unsupported constructs, generated / protected files, and round-trip
    /// conflicts never collapse into silent writeback.
    pub unsupported_generated_protected_conflicts_never_silent: bool,
    /// Breakpoint / device previews keep runtime origin, live-versus-mock posture,
    /// and mapping quality visible.
    pub breakpoint_previews_keep_origin_posture_and_mapping_visible: bool,
    /// Components bind to the shared write-scope, preview/apply/revert, citation,
    /// and degraded-state vocabulary rather than bespoke designer chrome.
    pub components_bound_to_shared_vocabulary: bool,
    /// The matrix does not widen into a new designer engine, framework pack, or
    /// preview runtime.
    pub no_new_engine_framework_or_runtime: bool,
}

impl VisualDesignerGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_remains_canonical
            && self.canvas_state_derivative_and_explicit
            && self.tree_canvas_source_selection_synchronized
            && self.property_editors_distinguish_value_state_no_silent_widening
            && self.unsupported_generated_protected_conflicts_never_silent
            && self.breakpoint_previews_keep_origin_posture_and_mapping_visible
            && self.components_bound_to_shared_vocabulary
            && self.no_new_engine_framework_or_runtime
    }
}

/// Consumer-projection block for the visual-designer component matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerConsumerProjection {
    /// Product surfaces ingest these component rows instead of cloning chrome.
    pub product_ingests_components: bool,
    /// Docs / help ingests the same component rows.
    pub docs_help_ingests_components: bool,
    /// Diagnostics ingests the same component rows.
    pub diagnostics_ingests_components: bool,
    /// Support export ingests the same component rows.
    pub support_export_ingests_components: bool,
    /// Release-control surfaces ingest the same component rows.
    pub release_control_ingests_components: bool,
    /// Later M5 rows reference one canonical component family instead of
    /// restating visual-designer truth in feature-local prose.
    pub later_rows_reference_one_canonical_family: bool,
}

impl VisualDesignerConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_components
            && self.docs_help_ingests_components
            && self.diagnostics_ingests_components
            && self.support_export_ingests_components
            && self.release_control_ingests_components
            && self.later_rows_reference_one_canonical_family
    }
}

/// Constructor input for [`VisualDesignerComponentMatrix::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisualDesignerComponentMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: VisualDesignerGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VisualDesignerConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe visual-designer component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDesignerComponentMatrix {
    /// Record kind; must equal [`VISUAL_DESIGNER_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub set_label: String,
    /// Per-component rows.
    pub components: Vec<ComponentRow>,
    /// Guardrail invariants block.
    pub guardrails: VisualDesignerGuardrails,
    /// Consumer projection block.
    pub consumer_projection: VisualDesignerConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl VisualDesignerComponentMatrix {
    /// Builds a visual-designer component matrix packet.
    pub fn new(input: VisualDesignerComponentMatrixInput) -> Self {
        Self {
            record_kind: VISUAL_DESIGNER_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            components: input.components,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Families represented by some row in this matrix.
    pub fn represented_families(&self) -> BTreeSet<M5VisualDesignerComponentFamily> {
        self.components.iter().map(|r| r.family).collect()
    }

    /// Count of rows that are complete, honest degraded / narrowed components.
    pub fn degraded_row_count(&self) -> usize {
        self.components.iter().filter(|r| r.is_degraded()).count()
    }

    /// Validates the visual-designer component matrix invariants.
    pub fn validate(&self) -> Vec<VisualDesignerComponentViolation> {
        let mut violations = Vec::new();

        if self.record_kind != VISUAL_DESIGNER_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(VisualDesignerComponentViolation::WrongRecordKind);
        }
        if self.schema_version != VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(VisualDesignerComponentViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(VisualDesignerComponentViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("visual-designer component matrix serializes"),
        ) {
            violations.push(VisualDesignerComponentViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("visual-designer component matrix serializes")
    }

    /// Deterministic CSV of the component rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "component_id,family,preview_surface,source_sync,round_trip,export_safe,assistive_ready,degraded\n",
        );
        for row in &self.components {
            out.push_str(&format!(
                "{id},{family},{surface},{sync},{round_trip},{export_safe},{assistive},{degraded}\n",
                id = row.component_id,
                family = row.family.as_str(),
                surface = row.preview_surface.as_str(),
                sync = row.source_sync.as_str(),
                round_trip = row.round_trip.as_str(),
                export_safe = row.export_safe,
                assistive = row.assistive_ready,
                degraded = row
                    .degraded
                    .as_ref()
                    .map_or("none", |d| d.trigger.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Visual-Designer Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Components: {} across {} / {} families ({} degraded)\n",
            self.components.len(),
            self.represented_families().len(),
            M5VisualDesignerComponentFamily::ALL.len(),
            self.degraded_row_count(),
        ));
        out.push_str("\n## Components\n\n");
        for row in &self.components {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.component_id,
                row.family.as_str(),
                row.surface_label,
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!("  - {}\n", row.chip_tokens()));
            if let Some(degraded) = &row.degraded {
                out.push_str(&format!(
                    "  - Degraded: trigger={} — {}\n",
                    degraded.trigger.as_str(),
                    degraded.degraded_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in visual-designer component export.
#[derive(Debug)]
pub enum VisualDesignerComponentArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<VisualDesignerComponentViolation>),
}

impl fmt::Display for VisualDesignerComponentArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "visual-designer component export parse failed: {error}"
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
                    "visual-designer component export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for VisualDesignerComponentArtifactError {}

/// Validation failures emitted by [`VisualDesignerComponentMatrix::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VisualDesignerComponentViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required reusable component family is defined by no row.
    RequiredFamilyMissing,
    /// The matrix demonstrates no complete degraded / narrowed row.
    DegradedCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A row's family-specific payload is missing, extra, or wrong for its family.
    PayloadFamilyMismatch,
    /// A row's family payload is internally dishonest.
    PayloadDishonest,
    /// A source-sync chip discloses a sync class different from its row.
    ChipSyncMismatch,
    /// A row omits a mandatory required label.
    MandatoryLabelMissing,
    /// A row is not export-safe or not assistive-ready.
    ParityMissing,
    /// A degraded block carries a generic non-answer label.
    DegradedLabelGeneric,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl VisualDesignerComponentViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DegradedCaseMissing => "degraded_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::PayloadFamilyMismatch => "payload_family_mismatch",
            Self::PayloadDishonest => "payload_dishonest",
            Self::ChipSyncMismatch => "chip_sync_mismatch",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ParityMissing => "parity_missing",
            Self::DegradedLabelGeneric => "degraded_label_generic",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in visual-designer component export.
pub fn current_m5_visual_designer_component_matrix_export(
) -> Result<VisualDesignerComponentMatrix, VisualDesignerComponentArtifactError> {
    let packet: VisualDesignerComponentMatrix = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/m5_visual_designer_component_matrix/support_export.json"
    )))
    .map_err(VisualDesignerComponentArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(VisualDesignerComponentArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &VisualDesignerComponentMatrix,
    violations: &mut Vec<VisualDesignerComponentViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_REF,
        VISUAL_DESIGNER_COMPONENT_MATRIX_DOC_REF,
        VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(VisualDesignerComponentViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &VisualDesignerComponentMatrix,
    violations: &mut Vec<VisualDesignerComponentViolation>,
) {
    let families = packet.represented_families();
    for required in M5VisualDesignerComponentFamily::ALL {
        if !families.contains(&required) {
            violations.push(VisualDesignerComponentViolation::RequiredFamilyMissing);
            break;
        }
    }
    if packet.degraded_row_count() == 0 {
        violations.push(VisualDesignerComponentViolation::DegradedCaseMissing);
    }
}

fn validate_rows(
    packet: &VisualDesignerComponentMatrix,
    violations: &mut Vec<VisualDesignerComponentViolation>,
) {
    for row in &packet.components {
        if !row.is_complete() {
            violations.push(VisualDesignerComponentViolation::RowIncomplete);
        }
        if !row.payload_matches_family() {
            violations.push(VisualDesignerComponentViolation::PayloadFamilyMismatch);
        }
        if !row.payload_honest() {
            violations.push(VisualDesignerComponentViolation::PayloadDishonest);
        }
        if !row.chip_matches_row_sync() {
            violations.push(VisualDesignerComponentViolation::ChipSyncMismatch);
        }
        if !row.mandatory_labels_present() {
            violations.push(VisualDesignerComponentViolation::MandatoryLabelMissing);
        }
        if !row.export_safe || !row.assistive_ready {
            violations.push(VisualDesignerComponentViolation::ParityMissing);
        }
        if !row.degraded_ok() {
            violations.push(VisualDesignerComponentViolation::DegradedLabelGeneric);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(VisualDesignerComponentViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &VisualDesignerComponentMatrix,
    violations: &mut Vec<VisualDesignerComponentViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(VisualDesignerComponentViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &VisualDesignerComponentMatrix,
    violations: &mut Vec<VisualDesignerComponentViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(VisualDesignerComponentViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded / card label is a generic non-answer rather than a precise
/// label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "cannot edit"
            | "no mapping"
            | "blocked"
            | "degraded"
            | "fallback"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds the canonical, checked-in visual-designer component matrix packet. This
/// is the one source of truth shared by the tests, the example dump, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_visual_designer_component_matrix() -> VisualDesignerComponentMatrix {
    VisualDesignerComponentMatrix::new(VisualDesignerComponentMatrixInput {
        packet_id: "m5-visual-designer-component-matrix:stable:0001".to_owned(),
        set_label: "M5 Visual-Designer Component Matrix".to_owned(),
        components: seeded_components(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-03T00:00:00Z".to_owned(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:visual-designer:{id}")]
}

fn mandatory_labels() -> Vec<M5VisualDesignerRequiredLabel> {
    vec![
        M5VisualDesignerRequiredLabel::Identity,
        M5VisualDesignerRequiredLabel::SourceOwnership,
        M5VisualDesignerRequiredLabel::State,
        M5VisualDesignerRequiredLabel::SyncOrFreshness,
        M5VisualDesignerRequiredLabel::KeyboardRoute,
    ]
}

fn seeded_components() -> Vec<ComponentRow> {
    vec![
        // Design canvas — source-bound editable, derivative of source, selection
        // synchronized with the tree and source.
        ComponentRow {
            component_id: "component:design-canvas:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::DesignCanvas,
            surface_label: "Visual design canvas bound to the canonical source revision".to_owned(),
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: Some(DesignCanvasDescriptor {
                canvas_state: M5CanvasState::SourceBoundEditable,
                is_derivative_of_source: true,
                selection_synced_with_tree_and_source: true,
                source_revision_ref: "source_revision:canvas:0001".to_owned(),
            }),
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: None,
            label_summary: "Source-bound editable canvas whose state is derivative of source and whose selection stays synchronized with the tree and source".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("design-canvas:0001"),
        },
        // Structure-tree row — a mapped source element with a synchronized
        // selection and a real span.
        ComponentRow {
            component_id: "component:structure-tree-row:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::StructureTreeRow,
            surface_label: "Structure tree row for a hand-authored source element".to_owned(),
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: Some(StructureTreeRowDescriptor {
                node_kind: M5StructureNodeKind::SourceElement,
                maps_to_source_span: true,
                selection_synced_with_canvas_and_source: true,
                source_span_ref: Some("span:tree:0001".to_owned()),
            }),
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: None,
            label_summary: "A source-element tree row that maps to an exact span and keeps selection synchronized with the canvas and source".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("structure-tree-row:0001"),
        },
        // Structure-tree row — an unmapped generated node that discloses it has no
        // source span and narrows.
        ComponentRow {
            component_id: "component:structure-tree-row:0002".to_owned(),
            family: M5VisualDesignerComponentFamily::StructureTreeRow,
            surface_label: "Structure tree row for a loop-generated node with no source span".to_owned(),
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            source_sync: SourceSyncClass::UnidentifiedSourceSync,
            round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: Some(StructureTreeRowDescriptor {
                node_kind: M5StructureNodeKind::UnmappedNode,
                maps_to_source_span: false,
                selection_synced_with_canvas_and_source: false,
                source_span_ref: None,
            }),
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::UnmappedSource,
                degraded_label: "This node has no resolvable source span; it is inspect-only and never claims a mapping".to_owned(),
            }),
            label_summary: "An unmapped node discloses it has no source span and stays inspect-only rather than fake a mapping".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("structure-tree-row:0002"),
        },
        // Property-inspector row — a design-token value whose edit takes a shared
        // token-definition scope, previews a real source diff, and requires review.
        ComponentRow {
            component_id: "component:property-inspector-row:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::PropertyInspectorRow,
            surface_label: "Property inspector row for a design-token color value".to_owned(),
            preview_surface: PreviewSurface::VisualEditTransform,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::ApproximateSourceRoundTrip,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: Some(PropertyInspectorRowDescriptor {
                value_state: M5PropertyValueState::DesignToken,
                write_scope: M5PropertyWriteScope::TokenDefinitionShared,
                protected_path_posture: ProtectedPathPosture::ProtectedReviewRequired,
                preview_diff: PreviewDiffClass::RealSourceMultiFileDiff,
                review_posture: Some(MutationReviewPosture::ReviewRequired),
                widens_write_scope_silently: false,
            }),
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: None,
            label_summary: "A design-token edit names its shared token-definition write scope, previews the real multi-file source diff, and requires review".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("property-inspector-row:0001"),
        },
        // Property-inspector row — a bound expression that is inspect-only, showing
        // no diff and taking no silent write.
        ComponentRow {
            component_id: "component:property-inspector-row:0002".to_owned(),
            family: M5VisualDesignerComponentFamily::PropertyInspectorRow,
            surface_label: "Property inspector row for a runtime-bound style value".to_owned(),
            preview_surface: PreviewSurface::VisualEditTransform,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::SourceOnlyFallback,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: Some(PropertyInspectorRowDescriptor {
                value_state: M5PropertyValueState::BoundExpression,
                write_scope: M5PropertyWriteScope::NoWriteInspectOnly,
                protected_path_posture: ProtectedPathPosture::Unprotected,
                preview_diff: PreviewDiffClass::NoDiffInspectOnly,
                review_posture: None,
                widens_write_scope_silently: false,
            }),
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
                degraded_label: "This value is bound to a runtime expression; the inspector stays inspect-only rather than widen the write scope".to_owned(),
            }),
            label_summary: "A runtime-bound value is inspect-only; the inspector shows no diff and never widens the write scope silently".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("property-inspector-row:0002"),
        },
        // Source-sync chip — drifted from source, offering a rebuild recovery route.
        ComponentRow {
            component_id: "component:source-sync-chip:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::SourceSyncChip,
            surface_label: "Source-sync chip on a preview that drifted from source".to_owned(),
            preview_surface: PreviewSurface::SourceFirstFrameworkPreview,
            source_sync: SourceSyncClass::DriftedFromSource,
            round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: Some(SourceSyncChipDescriptor {
                sync_class: SourceSyncClass::DriftedFromSource,
                recovery_route: M5SyncRecoveryRoute::RebuildFromSource,
            }),
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                degraded_label: "This preview drifted from the canonical source; rebuild from source before relying on it".to_owned(),
            }),
            label_summary: "A drifted source-sync chip discloses the drift and offers a rebuild-from-source recovery route".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("source-sync-chip:0001"),
        },
        // Breakpoint-preview row — a mobile viewport showing live data with an exact
        // source mapping and a visible runtime origin.
        ComponentRow {
            component_id: "component:breakpoint-preview-row:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::BreakpointPreviewRow,
            surface_label: "Breakpoint preview row for a mobile viewport over a live runtime".to_owned(),
            preview_surface: PreviewSurface::DeviceOrSimulatorPreview,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::ApproximateSourceRoundTrip,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: Some(BreakpointPreviewRowDescriptor {
                device_class: M5DevicePreviewClass::MobileViewport,
                data_posture: M5PreviewDataPosture::Live,
                mapping_quality: M5BreakpointMappingQuality::Exact,
                runtime_origin_token: "runtime_origin:local_dev_server".to_owned(),
            }),
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: None,
            label_summary: "A mobile-viewport preview keeps its runtime origin, live data posture, and exact mapping quality visible".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("breakpoint-preview-row:0001"),
        },
        // Breakpoint-preview row — a simulator showing mock data with an unmapped
        // source mapping; narrows.
        ComponentRow {
            component_id: "component:breakpoint-preview-row:0002".to_owned(),
            family: M5VisualDesignerComponentFamily::BreakpointPreviewRow,
            surface_label: "Breakpoint preview row for a simulator showing mock data".to_owned(),
            preview_surface: PreviewSurface::DeviceOrSimulatorPreview,
            source_sync: SourceSyncClass::RuntimeOnlyNoSource,
            round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: Some(BreakpointPreviewRowDescriptor {
                device_class: M5DevicePreviewClass::SimulatorPreview,
                data_posture: M5PreviewDataPosture::Mock,
                mapping_quality: M5BreakpointMappingQuality::Unmapped,
                runtime_origin_token: "runtime_origin:simulator_session".to_owned(),
            }),
            unsupported_construct_card: None,
            round_trip_conflict_banner: None,
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::UnmappedSource,
                degraded_label: "This simulator preview shows mock data with no source mapping; it stays inspect-only and never claims live source fidelity".to_owned(),
            }),
            label_summary: "A simulator preview names its mock data posture and unmapped mapping quality rather than blur the runtime truth".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("breakpoint-preview-row:0002"),
        },
        // Unsupported-construct card — reuses the shared card struct to degrade a
        // dynamically bound construct with the selection preserved.
        ComponentRow {
            component_id: "component:unsupported-construct-card:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::UnsupportedConstructCard,
            surface_label: "Unsupported-construct card for a dynamically bound attribute".to_owned(),
            preview_surface: PreviewSurface::VisualEditTransform,
            source_sync: SourceSyncClass::InSyncFromSource,
            round_trip: RoundTripCapabilityClass::SourceOnlyFallback,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: Some(UnsupportedConstructCard {
                reason: crate::UnsupportedConstructReason::DynamicBinding,
                preserves_selection_context: true,
                card_label: "This attribute is bound to a runtime expression; the visual edit degrades to a code-first source suggestion rather than guess the binding".to_owned(),
            }),
            round_trip_conflict_banner: None,
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
                degraded_label: "This construct is dynamically bound and cannot round-trip; the surface degrades to a code-first suggestion with the selection preserved".to_owned(),
            }),
            label_summary: "An unsupported-construct card degrades a dynamically bound attribute to a code-first suggestion and preserves the selection context".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("unsupported-construct-card:0001"),
        },
        // Round-trip conflict banner — source changed under an in-flight edit;
        // offers a reload-and-reapply route and never writes back silently.
        ComponentRow {
            component_id: "component:round-trip-conflict-banner:0001".to_owned(),
            family: M5VisualDesignerComponentFamily::RoundTripConflictBanner,
            surface_label: "Round-trip conflict banner after source changed under an edit".to_owned(),
            preview_surface: PreviewSurface::VisualEditTransform,
            source_sync: SourceSyncClass::PendingRebuild,
            round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
            required_labels: mandatory_labels(),
            export_safe: true,
            assistive_ready: true,
            design_canvas: None,
            structure_tree_row: None,
            property_inspector_row: None,
            source_sync_chip: None,
            breakpoint_preview_row: None,
            unsupported_construct_card: None,
            round_trip_conflict_banner: Some(RoundTripConflictBannerDescriptor {
                conflict_class: M5RoundTripConflictClass::SourceChangedUnderEdit,
                resolution_route: M5ConflictResolutionRoute::ReloadSourceReapply,
                never_silent_writeback: true,
                preserves_selection_context: true,
            }),
            degraded: Some(DegradedState {
                trigger: M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
                degraded_label: "The canonical source changed under this edit; reload the source and re-apply rather than write back over the change".to_owned(),
            }),
            label_summary: "A round-trip conflict banner names the source-changed-under-edit conflict and offers a reload-and-reapply route instead of a silent writeback".to_owned(),
            observed_at: "2026-07-03T00:00:00Z".to_owned(),
            evidence_refs: ev("round-trip-conflict-banner:0001"),
        },
    ]
}

fn seeded_guardrails() -> VisualDesignerGuardrails {
    VisualDesignerGuardrails {
        source_remains_canonical: true,
        canvas_state_derivative_and_explicit: true,
        tree_canvas_source_selection_synchronized: true,
        property_editors_distinguish_value_state_no_silent_widening: true,
        unsupported_generated_protected_conflicts_never_silent: true,
        breakpoint_previews_keep_origin_posture_and_mapping_visible: true,
        components_bound_to_shared_vocabulary: true,
        no_new_engine_framework_or_runtime: true,
    }
}

fn seeded_consumer_projection() -> VisualDesignerConsumerProjection {
    VisualDesignerConsumerProjection {
        product_ingests_components: true,
        docs_help_ingests_components: true,
        diagnostics_ingests_components: true,
        support_export_ingests_components: true,
        release_control_ingests_components: true,
        later_rows_reference_one_canonical_family: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        VISUAL_DESIGNER_COMPONENT_MATRIX_DOC_REF.to_owned(),
        VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF.to_owned(),
        "schemas/preview/visual_edit_transforms.schema.json".to_owned(),
        "schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json".to_owned(),
    ]
}
