//! One reusable M5 breakpoint / device-preview row primitive: the viewport /
//! breakpoint / theme-and-state variant row, the live-versus-mock runtime-truth
//! cue, and the compare / open-source continuity actions for one preview target,
//! resolved once so a preview surface never blurs whether the user is looking at
//! live runtime data, mock data, or a stale / captured view, and never loses the
//! source anchor when the user moves across device targets, variants, or runtime
//! origins.
//!
//! Aureline's frozen visual-designer component matrix
//! ([`crate::freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix`])
//! names the breakpoint / device-preview row as a governed component family and
//! freezes its device-class, data-posture, and mapping-quality vocabulary. The
//! selected-node primitive
//! ([`crate::implement_the_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive`])
//! implements the canvas / tree / inspector families, and the round-trip-honesty
//! primitive
//! ([`crate::implement_the_m5_source_sync_chip_round_trip_conflict_and_generated_or_protected_boundary_primitive`])
//! implements the source-sync-chip / conflict-banner / unsupported-card families.
//! This module *implements* the remaining breakpoint / device-preview family as one
//! reusable primitive: a resolver that takes one preview target's viewport / device
//! situation and produces one [`M5ResolvedBreakpointPreview`] carrying the device
//! preview row, the runtime-truth cue, and the continuity actions — all sharing one
//! target identity.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_breakpoint_preview`] — that takes one
//!    [`M5BreakpointPreviewInput`] (a target's device / viewport class, active
//!    breakpoint, theme / state variant, data posture, runtime origin, freshness,
//!    source-mapping quality, source-sync class, and optional source span) and
//!    produces one [`M5ResolvedBreakpointPreview`]. A preview can never hide whether
//!    it is showing live, mock, or captured / stale data (AC1): every resolved
//!    preview carries an explicit runtime-truth cue naming the data posture, the
//!    runtime origin, and the freshness. Device / breakpoint switching stays
//!    source-anchored and reviewable (AC2): when the preview maps to source the
//!    resolver keeps the source anchor and offers an open-source action, and the
//!    target identity survives across the row, the cue, and the actions so the user
//!    never loses their place. Every degrade below a live, fresh, source-anchored
//!    preview names a typed downgrade trigger from the shared vocabulary rather than
//!    a feature-local label (AC3).
//! 2. A parity matrix — [`M5BreakpointPreviewPacket`] — that binds one row per
//!    claimed M5 visual-design surface family to the shared row / cue / continuity
//!    contract and carries worked resolution cases so framework-pack preview,
//!    browser-runtime handoff, and docs / demo consumers reconstruct preview-runtime
//!    truth from one shared model on every surface.
//!
//! The device-preview class ([`M5DevicePreviewClass`]), the preview-data posture
//! ([`M5PreviewDataPosture`]), the breakpoint mapping-quality
//! ([`M5BreakpointMappingQuality`]), the preview freshness
//! ([`PreviewFreshnessClass`]), the source-sync class ([`SourceSyncClass`]), the
//! downgrade triggers ([`M5VisualDesignerDowngradeTrigger`]), the preview surface
//! ([`PreviewSurface`]), and the surface families ([`M5VisualDesignSurfaceFamily`])
//! are reused verbatim from the frozen matrix and the sibling primitives. This
//! module mints new vocabulary only for what the breakpoint / device-preview
//! primitive itself needs: the preview runtime origin, the continuity action, and
//! the export fields. No M5 surface invents a second breakpoint-preview grammar.
//!
//! Raw source bodies, screenshots, runtime payloads, credentials, and raw URLs
//! never cross this boundary; the primitive carries only typed class tokens, opaque
//! target / span refs, opaque viewport / breakpoint / variant tokens, booleans, and
//! redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-breakpoint-device-preview-primitive.schema.json`](../../../../schemas/ui/m5-breakpoint-device-preview-primitive.schema.json)
//! and the contract doc is
//! [`docs/designer/m5_breakpoint_device_preview_primitive_contract.md`](../../../../docs/designer/m5_breakpoint_device_preview_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-breakpoint-device-preview-primitive/`](../../../../fixtures/ui/m5-breakpoint-device-preview-primitive/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The device-preview class, data posture, mapping quality, preview freshness,
// source-sync class, downgrade triggers, preview surface, and visual-design surface
// families are all frozen once, in the sibling matrices and primitives. This
// primitive reuses them verbatim so it never invents a parallel breakpoint-preview
// vocabulary.
pub use crate::{
    M5BreakpointMappingQuality, M5DevicePreviewClass, M5PreviewDataPosture,
    M5VisualDesignSurfaceFamily, M5VisualDesignerDowngradeTrigger, PreviewFreshnessClass,
    PreviewSurface, SourceSyncClass,
};

/// Stable record-kind tag carried by [`M5BreakpointPreviewPacket`].
pub const M5_BREAKPOINT_PREVIEW_RECORD_KIND: &str =
    "implement_m5_breakpoint_and_device_preview_row_primitive";

/// Schema version for M5 breakpoint / device-preview-primitive records.
pub const M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the breakpoint / device-preview-primitive boundary schema.
pub const M5_BREAKPOINT_PREVIEW_SCHEMA_REF: &str =
    "schemas/ui/m5-breakpoint-device-preview-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BREAKPOINT_PREVIEW_DOC_REF: &str =
    "docs/designer/m5_breakpoint_device_preview_primitive_contract.md";

/// Repo-relative path of the frozen visual-designer component matrix this primitive
/// narrows from.
pub const M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-visual-designer-component-matrix.schema.json";

/// Repo-relative path of the visual-edit-transform contract this primitive binds
/// its runtime origins against.
pub const M5_BREAKPOINT_PREVIEW_VISUAL_EDIT_REF: &str =
    "schemas/preview/visual_edit_transforms.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BREAKPOINT_PREVIEW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-breakpoint-device-preview-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_BREAKPOINT_PREVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-breakpoint-device-preview-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BREAKPOINT_PREVIEW_CSV_REF: &str =
    "artifacts/release/m5-breakpoint-device-preview-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BREAKPOINT_PREVIEW_REPORT_REF: &str =
    "artifacts/components/m5-breakpoint-device-preview-primitive.md";

/// Where a device preview renders its pixels from. Names the runtime origin so a
/// preview never blurs a captured snapshot into a live runtime, and so the
/// live-versus-mock posture is anchored to a real origin rather than a label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PreviewRuntimeOrigin {
    /// A live development runtime (dev server / hot-reload loop).
    LiveDevRuntime,
    /// A local mock / fixture runtime feeding deterministic data.
    LocalMockRuntime,
    /// A captured / pinned snapshot replayed with no live runtime.
    CapturedSnapshot,
    /// A tethered physical device rendering over a transport.
    TetheredDevice,
    /// A simulator / emulator runtime.
    SimulatorRuntime,
}

impl M5PreviewRuntimeOrigin {
    /// Every runtime origin, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveDevRuntime,
        Self::LocalMockRuntime,
        Self::CapturedSnapshot,
        Self::TetheredDevice,
        Self::SimulatorRuntime,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveDevRuntime => "live_dev_runtime",
            Self::LocalMockRuntime => "local_mock_runtime",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::TetheredDevice => "tethered_device",
            Self::SimulatorRuntime => "simulator_runtime",
        }
    }

    /// A precise, non-generic runtime-origin label safe to render on any surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveDevRuntime => "Live dev runtime",
            Self::LocalMockRuntime => "Local mock runtime",
            Self::CapturedSnapshot => "Captured snapshot",
            Self::TetheredDevice => "Tethered device",
            Self::SimulatorRuntime => "Simulator runtime",
        }
    }

    /// True when this origin is backed by a live runtime whose state can diverge
    /// from a pinned snapshot.
    pub const fn is_live_runtime(self) -> bool {
        matches!(
            self,
            Self::LiveDevRuntime | Self::TetheredDevice | Self::SimulatorRuntime
        )
    }

    /// True when this origin can honestly carry the given data posture. A captured
    /// snapshot can only show captured data; a local mock runtime can only show mock
    /// data; a live runtime shows live or mock data but never claims to be a captured
    /// snapshot. This keeps the live-versus-mock posture anchored to the origin so a
    /// captured view can never masquerade as a live one, and vice versa.
    pub const fn permits_data_posture(self, posture: M5PreviewDataPosture) -> bool {
        match self {
            Self::CapturedSnapshot => matches!(posture, M5PreviewDataPosture::Captured),
            Self::LocalMockRuntime => matches!(posture, M5PreviewDataPosture::Mock),
            Self::LiveDevRuntime | Self::TetheredDevice | Self::SimulatorRuntime => {
                matches!(
                    posture,
                    M5PreviewDataPosture::Live | M5PreviewDataPosture::Mock
                )
            }
        }
    }
}

/// A continuity action a breakpoint preview offers so the user keeps their place
/// while moving across device targets, variants, or runtime origins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreakpointContinuityAction {
    /// Compare this target across device / breakpoint / variant targets, keeping the
    /// selected node.
    CompareAcrossTargets,
    /// Open the canonical source for the selected node at this breakpoint.
    OpenSourceForBreakpoint,
    /// Re-attach the runtime backing this preview.
    ReattachRuntime,
    /// Pin the current view as a captured snapshot.
    PinCapture,
    /// Inspect-only; the preview has no source anchor to open.
    InspectOnly,
}

impl M5BreakpointContinuityAction {
    /// Every continuity action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CompareAcrossTargets,
        Self::OpenSourceForBreakpoint,
        Self::ReattachRuntime,
        Self::PinCapture,
        Self::InspectOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareAcrossTargets => "compare_across_targets",
            Self::OpenSourceForBreakpoint => "open_source_for_breakpoint",
            Self::ReattachRuntime => "reattach_runtime",
            Self::PinCapture => "pin_capture",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// A precise, non-generic action label safe to render.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CompareAcrossTargets => "Compare across device / breakpoint targets",
            Self::OpenSourceForBreakpoint => "Open source for this breakpoint",
            Self::ReattachRuntime => "Re-attach runtime",
            Self::PinCapture => "Pin capture",
            Self::InspectOnly => "Inspect only — no source anchor to open",
        }
    }
}

/// A field the support / export packet carries so preview-runtime truth is
/// reconstructable from the shared model. The first four in
/// [`M5BreakpointExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BreakpointExportField {
    /// The stable target identity, shared across row / cue / continuity.
    TargetId,
    /// The device / viewport class.
    DeviceClass,
    /// The live / mock / captured data posture.
    DataPosture,
    /// The runtime origin.
    RuntimeOrigin,
    /// The preview freshness.
    Freshness,
    /// The source-mapping quality.
    MappingQuality,
}

impl M5BreakpointExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TargetId,
        Self::DeviceClass,
        Self::DataPosture,
        Self::RuntimeOrigin,
        Self::Freshness,
        Self::MappingQuality,
    ];

    /// The export fields every breakpoint-preview export must carry so a consumer
    /// can always tell live from mock from stale / captured.
    pub const MANDATORY: [Self; 4] = [
        Self::TargetId,
        Self::DataPosture,
        Self::RuntimeOrigin,
        Self::Freshness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetId => "target_id",
            Self::DeviceClass => "device_class",
            Self::DataPosture => "data_posture",
            Self::RuntimeOrigin => "runtime_origin",
            Self::Freshness => "freshness",
            Self::MappingQuality => "mapping_quality",
        }
    }
}

/// The full input to the breakpoint-preview resolver for one preview target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointPreviewInput {
    /// The stable target identity that must survive across device targets, variants,
    /// and runtime origins.
    pub target_id: String,
    /// The human-readable node / route label the preview names.
    pub node_label: String,
    /// The viewport label the row shows; never a raw URL.
    pub viewport_label: String,
    /// Opaque active-breakpoint token (e.g. a named breakpoint); never a raw URL.
    pub active_breakpoint_token: String,
    /// Opaque theme-variant token.
    pub theme_variant_token: String,
    /// Opaque state-variant token.
    pub state_variant_token: String,
    /// The device / viewport class.
    pub device_class: M5DevicePreviewClass,
    /// Whether the preview shows live, mock, or captured data.
    pub data_posture: M5PreviewDataPosture,
    /// Where the preview renders its pixels from.
    pub runtime_origin: M5PreviewRuntimeOrigin,
    /// How fresh the preview-runtime view is.
    pub freshness: PreviewFreshnessClass,
    /// How well the preview maps back to source.
    pub mapping_quality: M5BreakpointMappingQuality,
    /// How the preview relates to canonical source.
    pub sync_class: SourceSyncClass,
    /// The opaque source-span ref, present when the target maps to source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_span_ref: Option<String>,
}

/// The resolved breakpoint / device-preview row: the viewport / breakpoint /
/// theme-and-state variant row and its live-versus-mock cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDevicePreviewRow {
    /// The target identity — identical to the cue and the continuity block.
    pub target_id: String,
    /// The previewed node / route.
    pub node_label: String,
    /// The viewport label.
    pub viewport_label: String,
    /// The active breakpoint token.
    pub active_breakpoint_token: String,
    /// The theme variant token.
    pub theme_variant_token: String,
    /// The state variant token.
    pub state_variant_token: String,
    /// The device / viewport class.
    pub device_class: M5DevicePreviewClass,
    /// The live / mock / captured data posture.
    pub data_posture: M5PreviewDataPosture,
    /// The source-mapping quality.
    pub mapping_quality: M5BreakpointMappingQuality,
    /// The runtime origin.
    pub runtime_origin: M5PreviewRuntimeOrigin,
    /// A precise live-versus-mock cue label.
    pub live_vs_mock_label: String,
    /// A precise row label.
    pub row_label: String,
}

/// The resolved runtime-truth cue: the honest answer to "what am I looking at right
/// now — live, mock, or a stale / captured view?".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRuntimeTruthCue {
    /// The target identity.
    pub target_id: String,
    /// The live / mock / captured data posture.
    pub data_posture: M5PreviewDataPosture,
    /// The runtime origin.
    pub runtime_origin: M5PreviewRuntimeOrigin,
    /// The preview freshness.
    pub freshness: PreviewFreshnessClass,
    /// True when the preview is showing live runtime data.
    pub is_live_data: bool,
    /// True when the preview is stale or its freshness is unknown.
    pub is_stale: bool,
    /// A precise runtime-truth label.
    pub truth_label: String,
}

/// The resolved continuity block: the compare / open-source actions that preserve
/// the selected node while the user moves across device targets, variants, or
/// runtime origins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedContinuity {
    /// The target identity.
    pub target_id: String,
    /// The continuity actions offered (always includes compare-across-targets).
    pub actions: Vec<M5BreakpointContinuityAction>,
    /// The compare-across-targets action is offered; always `true`.
    pub compare_action_available: bool,
    /// The open-source action is offered when the target is source-anchored.
    pub open_source_action_available: bool,
    /// Device / breakpoint switching keeps this preview anchored to source.
    pub source_anchored: bool,
    /// The selected node survives across device targets, variants, and origins;
    /// always `true`.
    pub preserves_selection_context: bool,
}

/// The resolved breakpoint-preview truth shared across row, cue, and continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBreakpointPreview {
    /// The stable target identity.
    pub target_id: String,
    /// The previewed node / route.
    pub node_label: String,
    /// The device / viewport class.
    pub device_class: M5DevicePreviewClass,
    /// The live / mock / captured data posture.
    pub data_posture: M5PreviewDataPosture,
    /// The runtime origin.
    pub runtime_origin: M5PreviewRuntimeOrigin,
    /// The preview freshness.
    pub freshness: PreviewFreshnessClass,
    /// The source-mapping quality.
    pub mapping_quality: M5BreakpointMappingQuality,
    /// The source-sync class disclosed.
    pub sync_class: SourceSyncClass,
    /// The resolved device-preview row.
    pub device_row: M5ResolvedDevicePreviewRow,
    /// The resolved runtime-truth cue.
    pub runtime_truth: M5ResolvedRuntimeTruthCue,
    /// The resolved continuity block.
    pub continuity: M5ResolvedContinuity,
    /// The typed downgrade trigger, when the preview degraded below a live, fresh,
    /// source-anchored view.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5VisualDesignerDowngradeTrigger>,
    /// The preview never hides its runtime truth; always `true`.
    pub no_hidden_runtime_truth: bool,
}

impl M5ResolvedBreakpointPreview {
    /// True when the target identity is identical across the row, the cue, and the
    /// continuity block.
    pub fn identity_consistent(&self) -> bool {
        self.device_row.target_id == self.target_id
            && self.runtime_truth.target_id == self.target_id
            && self.continuity.target_id == self.target_id
    }

    /// AC1: the preview always discloses its runtime truth — the cue names the data
    /// posture, the runtime origin, and the freshness, and its live / stale flags are
    /// consistent with the disclosed posture and freshness.
    pub fn discloses_runtime_truth(&self) -> bool {
        !self.runtime_truth.truth_label.trim().is_empty()
            && self.runtime_truth.is_live_data == (self.data_posture == M5PreviewDataPosture::Live)
            && self.runtime_truth.is_stale == self.freshness.forces_downgrade()
    }

    /// True when the preview is showing anything other than live, fresh data — mock,
    /// captured, or stale — so it is a worked case for the live-versus-mock cue.
    pub fn is_showing_non_live_or_stale(&self) -> bool {
        self.data_posture != M5PreviewDataPosture::Live || self.freshness.forces_downgrade()
    }

    /// AC2: device / breakpoint switching stays source-anchored and reviewable — the
    /// selected node survives, and a source-anchored preview offers an open-source
    /// action.
    pub fn switching_stays_source_anchored(&self) -> bool {
        self.continuity.preserves_selection_context
            && (!self.continuity.source_anchored || self.continuity.open_source_action_available)
    }

    /// True when the preview degraded below a live, fresh, source-anchored view for a
    /// reason support must be able to explain: it is stale, it lost its source anchor,
    /// its mapping is unmapped, or its runtime is unavailable / unidentified.
    pub fn has_runtime_truth_degrade(&self) -> bool {
        self.runtime_truth.is_stale
            || !self.continuity.source_anchored
            || self.mapping_quality == M5BreakpointMappingQuality::Unmapped
            || matches!(
                self.sync_class,
                SourceSyncClass::RuntimeOnlyNoSource
                    | SourceSyncClass::UnidentifiedSourceSync
                    | SourceSyncClass::DriftedFromSource
                    | SourceSyncClass::PendingRebuild
            )
    }

    /// AC3: every degrade below a live, fresh, source-anchored preview names a typed
    /// downgrade trigger so framework-pack, browser-runtime, and docs / demo
    /// consumers quote the same trigger instead of a feature-local label.
    pub fn degrade_is_explained(&self) -> bool {
        !self.has_runtime_truth_degrade() || self.downgrade_trigger.is_some()
    }
}

/// Errors returned by [`resolve_breakpoint_preview`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BreakpointPreviewResolutionError {
    /// The target identity was empty.
    EmptyTargetId,
    /// The node label was empty.
    EmptyNodeLabel,
    /// The viewport label was empty.
    EmptyViewportLabel,
    /// A required variant / breakpoint token was empty.
    EmptyVariantToken,
    /// The declared data posture contradicts the runtime origin (e.g. a captured
    /// snapshot claiming live data, or a live runtime claiming a captured snapshot).
    ContradictoryRuntimeOrigin,
    /// A source-mapped preview (exact / approximate mapping quality) declared no
    /// source span.
    MissingSpanForSourceMapping,
    /// An unmapped preview carried a source span, contradicting its claim to have no
    /// resolvable source mapping.
    ContradictoryUnmappedSpan,
    /// A runtime-only-no-source preview carried a source span, contradicting its
    /// claim to have no saved-source backing.
    ContradictoryRuntimeSpan,
    /// A label or token carried forbidden material.
    ForbiddenMaterial,
}

impl M5BreakpointPreviewResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTargetId => "empty_target_id",
            Self::EmptyNodeLabel => "empty_node_label",
            Self::EmptyViewportLabel => "empty_viewport_label",
            Self::EmptyVariantToken => "empty_variant_token",
            Self::ContradictoryRuntimeOrigin => "contradictory_runtime_origin",
            Self::MissingSpanForSourceMapping => "missing_span_for_source_mapping",
            Self::ContradictoryUnmappedSpan => "contradictory_unmapped_span",
            Self::ContradictoryRuntimeSpan => "contradictory_runtime_span",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BreakpointPreviewResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "breakpoint-preview resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BreakpointPreviewResolutionError {}

/// Resolves one preview target's device / viewport situation into its shared device
/// preview row, runtime-truth cue, and continuity actions.
///
/// The resolved preview carries one target identity on every component so the user
/// keeps their place across the row, the cue, and the actions. A preview can never
/// hide whether it is showing live, mock, or captured / stale data (AC1): the cue
/// always names the data posture, the runtime origin, and the freshness. Device /
/// breakpoint switching stays source-anchored and reviewable (AC2): a source-mapped
/// preview keeps its source anchor and offers an open-source action. Every degrade
/// below a live, fresh, source-anchored view carries a typed downgrade trigger from
/// the shared vocabulary so consumers explain it the same way (AC3).
pub fn resolve_breakpoint_preview(
    input: &M5BreakpointPreviewInput,
) -> Result<M5ResolvedBreakpointPreview, M5BreakpointPreviewResolutionError> {
    if input.target_id.trim().is_empty() {
        return Err(M5BreakpointPreviewResolutionError::EmptyTargetId);
    }
    if input.node_label.trim().is_empty() {
        return Err(M5BreakpointPreviewResolutionError::EmptyNodeLabel);
    }
    if input.viewport_label.trim().is_empty() {
        return Err(M5BreakpointPreviewResolutionError::EmptyViewportLabel);
    }
    if input.active_breakpoint_token.trim().is_empty()
        || input.theme_variant_token.trim().is_empty()
        || input.state_variant_token.trim().is_empty()
    {
        return Err(M5BreakpointPreviewResolutionError::EmptyVariantToken);
    }
    if label_is_forbidden(&input.node_label)
        || label_is_forbidden(&input.viewport_label)
        || label_is_forbidden(&input.active_breakpoint_token)
        || label_is_forbidden(&input.theme_variant_token)
        || label_is_forbidden(&input.state_variant_token)
    {
        return Err(M5BreakpointPreviewResolutionError::ForbiddenMaterial);
    }

    // A captured snapshot can never claim live data, and a live runtime can never
    // claim a captured snapshot; the runtime origin anchors the live-versus-mock
    // posture so a preview cannot blur the two.
    if !input
        .runtime_origin
        .permits_data_posture(input.data_posture)
    {
        return Err(M5BreakpointPreviewResolutionError::ContradictoryRuntimeOrigin);
    }

    let span_present = input
        .source_span_ref
        .as_ref()
        .is_some_and(|span| !span.trim().is_empty());

    // A source-mapped preview must name a span; you cannot claim a source anchor you
    // have no span for.
    if input.mapping_quality != M5BreakpointMappingQuality::Unmapped && !span_present {
        return Err(M5BreakpointPreviewResolutionError::MissingSpanForSourceMapping);
    }
    // An unmapped preview must not carry a source span.
    if input.mapping_quality == M5BreakpointMappingQuality::Unmapped && span_present {
        return Err(M5BreakpointPreviewResolutionError::ContradictoryUnmappedSpan);
    }
    // A runtime-only-no-source preview must not carry a saved-source span.
    if input.sync_class.is_runtime_only() && span_present {
        return Err(M5BreakpointPreviewResolutionError::ContradictoryRuntimeSpan);
    }

    let is_live_data = input.data_posture == M5PreviewDataPosture::Live;
    let is_stale = input.freshness.forces_downgrade();

    // Source-anchored when the preview maps to source, has a span, and is not a
    // runtime-only view with no saved source.
    let source_anchored = input.mapping_quality != M5BreakpointMappingQuality::Unmapped
        && span_present
        && !input.sync_class.is_runtime_only();

    let device_row = M5ResolvedDevicePreviewRow {
        target_id: input.target_id.clone(),
        node_label: input.node_label.clone(),
        viewport_label: input.viewport_label.clone(),
        active_breakpoint_token: input.active_breakpoint_token.clone(),
        theme_variant_token: input.theme_variant_token.clone(),
        state_variant_token: input.state_variant_token.clone(),
        device_class: input.device_class,
        data_posture: input.data_posture,
        mapping_quality: input.mapping_quality,
        runtime_origin: input.runtime_origin,
        live_vs_mock_label: live_vs_mock_label(input.data_posture, input.runtime_origin).to_owned(),
        row_label: row_label(input),
    };

    let runtime_truth = M5ResolvedRuntimeTruthCue {
        target_id: input.target_id.clone(),
        data_posture: input.data_posture,
        runtime_origin: input.runtime_origin,
        freshness: input.freshness,
        is_live_data,
        is_stale,
        truth_label: runtime_truth_label(input.data_posture, input.freshness).to_owned(),
    };

    let continuity = M5ResolvedContinuity {
        target_id: input.target_id.clone(),
        actions: continuity_actions(source_anchored, input),
        compare_action_available: true,
        open_source_action_available: source_anchored,
        source_anchored,
        preserves_selection_context: true,
    };

    let downgrade_trigger = resolve_downgrade_trigger(input, source_anchored, is_stale);

    Ok(M5ResolvedBreakpointPreview {
        target_id: input.target_id.clone(),
        node_label: input.node_label.clone(),
        device_class: input.device_class,
        data_posture: input.data_posture,
        runtime_origin: input.runtime_origin,
        freshness: input.freshness,
        mapping_quality: input.mapping_quality,
        sync_class: input.sync_class,
        device_row,
        runtime_truth,
        continuity,
        downgrade_trigger,
        no_hidden_runtime_truth: true,
    })
}

/// The continuity actions offered for one preview. Compare-across-targets is always
/// offered so the user can move between device targets; a source-anchored preview
/// also offers open-source, while a runtime-only preview offers a re-attach and a
/// non-source-anchored preview falls back to inspect-only.
fn continuity_actions(
    source_anchored: bool,
    input: &M5BreakpointPreviewInput,
) -> Vec<M5BreakpointContinuityAction> {
    let mut actions = vec![M5BreakpointContinuityAction::CompareAcrossTargets];
    if source_anchored {
        actions.push(M5BreakpointContinuityAction::OpenSourceForBreakpoint);
    } else if input.sync_class.is_runtime_only()
        || input.runtime_origin.is_live_runtime()
        || input.freshness.forces_downgrade()
    {
        actions.push(M5BreakpointContinuityAction::ReattachRuntime);
    } else {
        actions.push(M5BreakpointContinuityAction::InspectOnly);
    }
    if input.runtime_origin == M5PreviewRuntimeOrigin::CapturedSnapshot {
        // Already a snapshot; no pin action needed.
    } else {
        actions.push(M5BreakpointContinuityAction::PinCapture);
    }
    actions
}

/// Derives the typed downgrade trigger, when the preview degraded below a live,
/// fresh, source-anchored view. Named so framework-pack, browser-runtime, and docs /
/// demo consumers can quote the same trigger.
fn resolve_downgrade_trigger(
    input: &M5BreakpointPreviewInput,
    source_anchored: bool,
    is_stale: bool,
) -> Option<M5VisualDesignerDowngradeTrigger> {
    if input.sync_class == SourceSyncClass::RuntimeOnlyNoSource {
        return Some(M5VisualDesignerDowngradeTrigger::RuntimeUnavailable);
    }
    if input.mapping_quality == M5BreakpointMappingQuality::Unmapped || !source_anchored {
        return Some(M5VisualDesignerDowngradeTrigger::UnmappedSource);
    }
    if input.sync_class == SourceSyncClass::UnidentifiedSourceSync
        || input.freshness == PreviewFreshnessClass::Unknown
    {
        return Some(M5VisualDesignerDowngradeTrigger::UnidentifiedPosture);
    }
    if is_stale
        || matches!(
            input.sync_class,
            SourceSyncClass::DriftedFromSource | SourceSyncClass::PendingRebuild
        )
    {
        return Some(M5VisualDesignerDowngradeTrigger::DriftedFromSource);
    }
    None
}

/// A precise, non-generic live-versus-mock cue label per data posture and origin.
fn live_vs_mock_label(
    posture: M5PreviewDataPosture,
    origin: M5PreviewRuntimeOrigin,
) -> &'static str {
    match posture {
        M5PreviewDataPosture::Live => match origin {
            M5PreviewRuntimeOrigin::TetheredDevice => "Live data on a tethered device",
            M5PreviewRuntimeOrigin::SimulatorRuntime => "Live data on a simulator runtime",
            _ => "Live data from the dev runtime",
        },
        M5PreviewDataPosture::Mock => "Mock data — not a live runtime feed",
        M5PreviewDataPosture::Captured => "Captured snapshot — not a live view",
    }
}

/// A precise, non-generic runtime-truth label per data posture and freshness.
fn runtime_truth_label(
    posture: M5PreviewDataPosture,
    freshness: PreviewFreshnessClass,
) -> &'static str {
    match (posture, freshness) {
        (M5PreviewDataPosture::Live, PreviewFreshnessClass::Fresh) => {
            "Showing live runtime data, within the freshness SLO"
        }
        (M5PreviewDataPosture::Live, PreviewFreshnessClass::Aging) => {
            "Showing live runtime data, aging toward the freshness SLO"
        }
        (M5PreviewDataPosture::Live, PreviewFreshnessClass::Stale) => {
            "Showing live runtime data that is now stale — refresh to re-sync"
        }
        (M5PreviewDataPosture::Live, PreviewFreshnessClass::Unknown) => {
            "Showing live runtime data with unknown freshness — treat as stale"
        }
        (M5PreviewDataPosture::Mock, PreviewFreshnessClass::Stale) => {
            "Showing stale mock data — refresh the fixture"
        }
        (M5PreviewDataPosture::Mock, PreviewFreshnessClass::Unknown) => {
            "Showing mock data with unknown freshness"
        }
        (M5PreviewDataPosture::Mock, _) => "Showing mock / fixture data, not a live feed",
        (M5PreviewDataPosture::Captured, PreviewFreshnessClass::Stale) => {
            "Showing a stale captured snapshot — the source may have moved on"
        }
        (M5PreviewDataPosture::Captured, PreviewFreshnessClass::Unknown) => {
            "Showing a captured snapshot with unknown freshness"
        }
        (M5PreviewDataPosture::Captured, _) => {
            "Showing a captured / pinned snapshot, not a live view"
        }
    }
}

/// A precise, non-generic device-preview row label.
fn row_label(input: &M5BreakpointPreviewInput) -> String {
    format!(
        "{} — {} @ {} ({} / {})",
        input.node_label,
        input.viewport_label,
        input.active_breakpoint_token,
        input.theme_variant_token,
        input.state_variant_token,
    )
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs preview-runtime truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointPreviewCase {
    /// The resolver input.
    pub input: M5BreakpointPreviewInput,
    /// The resolved preview truth. Must equal
    /// `resolve_breakpoint_preview(&input)`.
    pub resolved: M5ResolvedBreakpointPreview,
}

impl M5BreakpointPreviewCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BreakpointPreviewInput) -> Self {
        let resolved =
            resolve_breakpoint_preview(&input).expect("seed breakpoint-preview case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_breakpoint_preview(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one visual-design surface family bound to the
/// shared breakpoint-preview contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointSurfaceRow {
    /// The visual-design surface family.
    pub surface_family: M5VisualDesignSurfaceFamily,
    /// The claimed preview surface this row maps onto (reused vocabulary).
    pub preview_surface: PreviewSurface,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Device-preview classes this surface can present (must be non-empty).
    pub device_classes: Vec<M5DevicePreviewClass>,
    /// Data postures this surface can disclose (must be non-empty).
    pub data_postures: Vec<M5PreviewDataPosture>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5BreakpointExportField>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5VisualDesignerDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection.
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_previews: Vec<M5BreakpointPreviewCase>,
    /// Hard invariant: this row never blurs live-versus-mock data. MUST be `false`.
    pub blurs_live_vs_mock: bool,
    /// Hard invariant: this row never hides a stale or captured view. MUST be
    /// `false`.
    pub hides_stale_or_captured: bool,
    /// Hard invariant: this row never drops the source anchor silently. MUST be
    /// `false`.
    pub drops_source_anchor_silently: bool,
    /// Hard invariant: this row never invents a feature-local preview label. MUST be
    /// `false`.
    pub invents_local_preview_labels: bool,
}

impl M5BreakpointSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BreakpointExportField> =
            self.export_fields.iter().copied().collect();
        M5BreakpointExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.blurs_live_vs_mock
            && !self.hides_stale_or_captured
            && !self.drops_source_anchor_silently
            && !self.invents_local_preview_labels
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointVocabularySet {
    /// Visual-design surface-family tokens (reused).
    pub surface_families: Vec<String>,
    /// Device-preview-class tokens (reused).
    pub device_classes: Vec<String>,
    /// Data-posture tokens (reused).
    pub data_postures: Vec<String>,
    /// Runtime-origin tokens.
    pub runtime_origins: Vec<String>,
    /// Freshness tokens (reused).
    pub freshness_classes: Vec<String>,
    /// Mapping-quality tokens (reused).
    pub mapping_qualities: Vec<String>,
    /// Source-sync-class tokens (reused).
    pub sync_classes: Vec<String>,
    /// Continuity-action tokens.
    pub continuity_actions: Vec<String>,
    /// Downgrade-trigger tokens (reused).
    pub downgrade_triggers: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
}

impl M5BreakpointVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5VisualDesignSurfaceFamily::ALL, |v| v.as_str()),
            device_classes: tokens(&DEVICE_CLASS_ALL, |v| v.as_str()),
            data_postures: tokens(&DATA_POSTURE_ALL, |v| v.as_str()),
            runtime_origins: tokens(&M5PreviewRuntimeOrigin::ALL, |v| v.as_str()),
            freshness_classes: tokens(&FRESHNESS_ALL, |v| v.as_str()),
            mapping_qualities: tokens(&MAPPING_QUALITY_ALL, |v| v.as_str()),
            sync_classes: tokens(&SYNC_CLASS_ALL, |v| v.as_str()),
            continuity_actions: tokens(&M5BreakpointContinuityAction::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
            export_fields: tokens(&M5BreakpointExportField::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The device-preview classes this primitive presents, in a stable order. The frozen
/// [`M5DevicePreviewClass`] enum is a pure token set, so the order is pinned here.
const DEVICE_CLASS_ALL: [M5DevicePreviewClass; 6] = [
    M5DevicePreviewClass::DesktopViewport,
    M5DevicePreviewClass::TabletViewport,
    M5DevicePreviewClass::MobileViewport,
    M5DevicePreviewClass::CustomViewport,
    M5DevicePreviewClass::DeviceTethered,
    M5DevicePreviewClass::SimulatorPreview,
];

/// The data postures this primitive discloses, in a stable order.
const DATA_POSTURE_ALL: [M5PreviewDataPosture; 3] = [
    M5PreviewDataPosture::Live,
    M5PreviewDataPosture::Mock,
    M5PreviewDataPosture::Captured,
];

/// The freshness classes this primitive discloses, in a stable order.
const FRESHNESS_ALL: [PreviewFreshnessClass; 4] = [
    PreviewFreshnessClass::Fresh,
    PreviewFreshnessClass::Aging,
    PreviewFreshnessClass::Stale,
    PreviewFreshnessClass::Unknown,
];

/// The mapping-quality classes this primitive discloses, in a stable order.
const MAPPING_QUALITY_ALL: [M5BreakpointMappingQuality; 3] = [
    M5BreakpointMappingQuality::Exact,
    M5BreakpointMappingQuality::Approximate,
    M5BreakpointMappingQuality::Unmapped,
];

/// The source-sync classes this primitive discloses, in a stable order.
const SYNC_CLASS_ALL: [SourceSyncClass; 5] = [
    SourceSyncClass::InSyncFromSource,
    SourceSyncClass::PendingRebuild,
    SourceSyncClass::DriftedFromSource,
    SourceSyncClass::RuntimeOnlyNoSource,
    SourceSyncClass::UnidentifiedSourceSync,
];

/// The downgrade triggers this primitive emits, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5VisualDesignerDowngradeTrigger; 7] = [
    M5VisualDesignerDowngradeTrigger::DriftedFromSource,
    M5VisualDesignerDowngradeTrigger::UnmappedSource,
    M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
    M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked,
    M5VisualDesignerDowngradeTrigger::UnsupportedConstruct,
    M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen,
    M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointGovernanceReview {
    /// One primitive carries the row, cue, and continuity on every surface.
    pub one_primitive_carries_row_cue_continuity: bool,
    /// Live / mock / captured / stale data is always disclosed, never blurred.
    pub live_mock_captured_always_disclosed: bool,
    /// Device / breakpoint switching stays source-anchored and reviewable.
    pub switching_stays_source_anchored: bool,
    /// Selection continuity survives across device targets, variants, and origins.
    pub selection_continuity_preserved_across_targets: bool,
    /// Every degrade is explained with a shared downgrade trigger in the export.
    pub degrade_explained_in_support_export: bool,
    /// No surface invents a second breakpoint-preview grammar.
    pub no_surface_invents_second_preview_grammar: bool,
    /// Later M5 rows cannot invent parallel breakpoint-preview vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointConsumerProjection {
    /// Framework-pack / browser-runtime / docs / demo surfaces all consume the shared
    /// primitive.
    pub preview_consumers_use_shared_primitive: bool,
    /// The resolver reads a single canonical preview model.
    pub resolver_reads_single_preview_model: bool,
    /// The runtime-truth cue reads a single canonical posture / freshness source.
    pub cue_reads_single_runtime_truth_source: bool,
    /// Support / export reads a single canonical preview source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the breakpoint-preview primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting preview audit.
    pub preview_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BreakpointPreviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BreakpointPreviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BreakpointSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BreakpointVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BreakpointGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BreakpointConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BreakpointReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 breakpoint / device-preview-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BreakpointPreviewPacket {
    /// Record kind; must equal [`M5_BREAKPOINT_PREVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5BreakpointSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BreakpointVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BreakpointGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BreakpointConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5BreakpointReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BreakpointPreviewPacket {
    /// Builds an M5 breakpoint-preview packet from stable-lane input.
    pub fn new(input: M5BreakpointPreviewPacketInput) -> Self {
        Self {
            record_kind: M5_BREAKPOINT_PREVIEW_RECORD_KIND.to_owned(),
            schema_version: M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION,
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

    /// Validates the M5 breakpoint-preview invariants.
    pub fn validate(&self) -> Vec<M5BreakpointPreviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BREAKPOINT_PREVIEW_RECORD_KIND {
            violations.push(M5BreakpointPreviewViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION {
            violations.push(M5BreakpointPreviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BreakpointPreviewViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 breakpoint preview packet serializes"),
        ) {
            violations.push(M5BreakpointPreviewViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 breakpoint preview packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,preview_surface,owner,device_classes,data_postures,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.preview_surface.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.device_classes, |v| v.as_str()),
                join_tokens(&row.data_postures, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_previews.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Breakpoint / Device-Preview Row Primitive: Device Row, Runtime-Truth Cue, and Continuity Actions\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Visual-design surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5VisualDesignSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Runtime origins: {}\n",
            self.vocabulary_set.runtime_origins.join(", ")
        ));
        out.push_str(&format!(
            "- Data postures: {}\n",
            self.vocabulary_set.data_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Continuity actions: {}\n",
            self.vocabulary_set.continuity_actions.join(", ")
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
                "  - Worked previews: {}\n",
                row.example_previews.len()
            ));
            for case in &row.example_previews {
                out.push_str(&format!(
                    "    - `{}` → node `{}` device `{}`, posture `{}`, origin `{}`, freshness `{}`\n",
                    case.resolved.target_id,
                    case.resolved.node_label,
                    case.resolved.device_class.as_str(),
                    case.resolved.data_posture.as_str(),
                    case.resolved.runtime_origin.as_str(),
                    case.resolved.freshness.as_str(),
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 breakpoint-preview export.
#[derive(Debug)]
pub enum M5BreakpointPreviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BreakpointPreviewViolation>),
}

impl fmt::Display for M5BreakpointPreviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 breakpoint preview export parse failed: {error}"
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
                    "m5 breakpoint preview export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BreakpointPreviewArtifactError {}

/// Validation failures emitted by [`M5BreakpointPreviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BreakpointPreviewViolation {
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
    /// A surface row declares no device-preview classes.
    DeviceClassMissing,
    /// A surface row declares no data postures.
    DataPostureMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked preview cases.
    ExamplePreviewMissing,
    /// A worked preview case does not match a fresh resolve of its input.
    ExamplePreviewDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked preview proves a mock / captured / stale view disclosed its runtime
    /// truth (AC1).
    RuntimeTruthDisclosureUnproven,
    /// No worked preview proves source-anchored device / breakpoint switching (AC2).
    SourceAnchoredSwitchingUnproven,
    /// No worked preview proves a degrade explained with a downgrade trigger (AC3).
    DegradeExplanationUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BreakpointPreviewViolation {
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
            Self::DeviceClassMissing => "device_class_missing",
            Self::DataPostureMissing => "data_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExamplePreviewMissing => "example_preview_missing",
            Self::ExamplePreviewDrift => "example_preview_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::RuntimeTruthDisclosureUnproven => "runtime_truth_disclosure_unproven",
            Self::SourceAnchoredSwitchingUnproven => "source_anchored_switching_unproven",
            Self::DegradeExplanationUnproven => "degrade_explanation_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 breakpoint-preview export.
pub fn current_stable_m5_breakpoint_preview_export(
) -> Result<M5BreakpointPreviewPacket, M5BreakpointPreviewArtifactError> {
    let packet: M5BreakpointPreviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-breakpoint-device-preview-proof/support_export.json"
    )))
    .map_err(M5BreakpointPreviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BreakpointPreviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BREAKPOINT_PREVIEW_SCHEMA_REF,
        M5_BREAKPOINT_PREVIEW_DOC_REF,
        M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF,
        M5_BREAKPOINT_PREVIEW_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BreakpointPreviewViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BreakpointPreviewViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let present: BTreeSet<M5VisualDesignSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5VisualDesignSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BreakpointPreviewViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5BreakpointPreviewViolation::SurfaceRowIncomplete);
        }
        if row.device_classes.is_empty() {
            violations.push(M5BreakpointPreviewViolation::DeviceClassMissing);
        }
        if row.data_postures.is_empty() {
            violations.push(M5BreakpointPreviewViolation::DataPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BreakpointPreviewViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BreakpointPreviewViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BreakpointPreviewViolation::ConsumerSurfacesMissing);
        }
        if row.example_previews.is_empty() {
            violations.push(M5BreakpointPreviewViolation::ExamplePreviewMissing);
        }
        if row
            .example_previews
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BreakpointPreviewViolation::ExamplePreviewDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5BreakpointPreviewViolation::SurfaceInvariantViolated);
        }
    }
}

/// The three acceptance criteria must each be demonstrated by at least one worked
/// preview across the matrix: a mock / captured / stale view disclosing its runtime
/// truth (AC1), a source-anchored device / breakpoint switch (AC2), and a degrade
/// explained with a downgrade trigger (AC3). The stronger per-preview invariants are
/// also enforced across every case.
fn validate_acceptance_criteria_covered(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let cases: Vec<&M5ResolvedBreakpointPreview> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_previews.iter().map(|case| &case.resolved))
        .collect();

    let disclosure_proven = cases.iter().any(|resolved| {
        resolved.is_showing_non_live_or_stale() && resolved.discloses_runtime_truth()
    }) && cases
        .iter()
        .all(|resolved| resolved.discloses_runtime_truth());
    if !disclosure_proven {
        violations.push(M5BreakpointPreviewViolation::RuntimeTruthDisclosureUnproven);
    }

    let anchored_proven = cases
        .iter()
        .any(|resolved| resolved.continuity.source_anchored)
        && cases.iter().all(|resolved| {
            resolved.switching_stays_source_anchored() && resolved.identity_consistent()
        });
    if !anchored_proven {
        violations.push(M5BreakpointPreviewViolation::SourceAnchoredSwitchingUnproven);
    }

    let degrade_proven = cases.iter().any(|resolved| {
        resolved.has_runtime_truth_degrade() && resolved.downgrade_trigger.is_some()
    }) && cases.iter().all(|resolved| resolved.degrade_is_explained());
    if !degrade_proven {
        violations.push(M5BreakpointPreviewViolation::DegradeExplanationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_row_cue_continuity,
        review.live_mock_captured_always_disclosed,
        review.switching_stays_source_anchored,
        review.selection_continuity_preserved_across_targets,
        review.degrade_explained_in_support_export,
        review.no_surface_invents_second_preview_grammar,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BreakpointPreviewViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.preview_consumers_use_shared_primitive,
        projection.resolver_reads_single_preview_model,
        projection.cue_reads_single_runtime_truth_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BreakpointPreviewViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5BreakpointPreviewPacket,
    violations: &mut Vec<M5BreakpointPreviewViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.preview_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BreakpointPreviewViolation::ReleasePostureIncomplete);
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

/// True when a label carries obviously forbidden material.
fn label_is_forbidden(value: &str) -> bool {
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
        serde_json::Value::String(s) => label_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in M5 breakpoint-preview packet. This is the one
/// source of truth shared by the tests, the example dump, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_breakpoint_preview_packet() -> M5BreakpointPreviewPacket {
    M5BreakpointPreviewPacket::new(M5BreakpointPreviewPacketInput {
        packet_id: "m5-breakpoint-device-preview-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Breakpoint / Device-Preview Row Primitive: Device Row, Runtime-Truth Cue, and Continuity Actions"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5BreakpointVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-03T00:00:00Z".to_owned(),
    })
}

fn all_export_fields() -> Vec<M5BreakpointExportField> {
    M5BreakpointExportField::ALL.to_vec()
}

fn seeded_surface_rows() -> Vec<M5BreakpointSurfaceRow> {
    vec![
        // Desktop designer — a live, fresh, source-anchored desktop breakpoint that
        // maps exactly to source (baseline source-anchored). Proves AC2.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::DesktopDesigner,
            preview_surface: PreviewSurface::VisualSurfaceMapping,
            owner_role: "Visual Designer Platform".to_owned(),
            scope_summary:
                "Desktop designer breakpoint row for a live, source-anchored element across viewports"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
            ],
            consumer_surfaces: vec!["product_designer".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF.to_owned()],
            example_previews: vec![
                M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                    target_id: "target:desktop:hero-heading:0001".to_owned(),
                    node_label: "HeroHeading".to_owned(),
                    viewport_label: "Desktop — 1440×900".to_owned(),
                    active_breakpoint_token: "lg".to_owned(),
                    theme_variant_token: "light".to_owned(),
                    state_variant_token: "default".to_owned(),
                    device_class: M5DevicePreviewClass::DesktopViewport,
                    data_posture: M5PreviewDataPosture::Live,
                    runtime_origin: M5PreviewRuntimeOrigin::LiveDevRuntime,
                    freshness: PreviewFreshnessClass::Fresh,
                    mapping_quality: M5BreakpointMappingQuality::Exact,
                    sync_class: SourceSyncClass::InSyncFromSource,
                    source_span_ref: Some("span:desktop:hero-heading".to_owned()),
                }),
                M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                    target_id: "target:desktop:hero-heading:0001".to_owned(),
                    node_label: "HeroHeading".to_owned(),
                    viewport_label: "Mobile — 390×844".to_owned(),
                    active_breakpoint_token: "sm".to_owned(),
                    theme_variant_token: "dark".to_owned(),
                    state_variant_token: "hover".to_owned(),
                    device_class: M5DevicePreviewClass::MobileViewport,
                    data_posture: M5PreviewDataPosture::Live,
                    runtime_origin: M5PreviewRuntimeOrigin::LiveDevRuntime,
                    freshness: PreviewFreshnessClass::Fresh,
                    mapping_quality: M5BreakpointMappingQuality::Exact,
                    sync_class: SourceSyncClass::InSyncFromSource,
                    source_span_ref: Some("span:desktop:hero-heading".to_owned()),
                }),
            ],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
        // Source-first preview — a mock, source-anchored tablet breakpoint fed by a
        // local mock runtime (live-versus-mock disclosed). Proves AC1.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SourceFirstPreview,
            preview_surface: PreviewSurface::SourceFirstFrameworkPreview,
            owner_role: "Source-First Preview".to_owned(),
            scope_summary:
                "Source-first preview breakpoint row disclosing mock data on a source-anchored node"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
            ],
            consumer_surfaces: vec!["preview_runtime".to_owned(), "docs_demo".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_VISUAL_EDIT_REF.to_owned()],
            example_previews: vec![M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                target_id: "target:preview:pricing-card:0001".to_owned(),
                node_label: "PricingCard".to_owned(),
                viewport_label: "Tablet — 834×1112".to_owned(),
                active_breakpoint_token: "md".to_owned(),
                theme_variant_token: "light".to_owned(),
                state_variant_token: "default".to_owned(),
                device_class: M5DevicePreviewClass::TabletViewport,
                data_posture: M5PreviewDataPosture::Mock,
                runtime_origin: M5PreviewRuntimeOrigin::LocalMockRuntime,
                freshness: PreviewFreshnessClass::Fresh,
                mapping_quality: M5BreakpointMappingQuality::Exact,
                sync_class: SourceSyncClass::InSyncFromSource,
                source_span_ref: Some("span:preview:pricing-card".to_owned()),
            })],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
        // Browser-runtime inspector — a runtime-only-no-source mobile node on a live
        // dev runtime: no source anchor, so the preview degrades with a
        // runtime-unavailable trigger and offers a re-attach. Proves AC1 + AC3.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::BrowserRuntimeInspector,
            preview_surface: PreviewSurface::BrowserRuntimeInspection,
            owner_role: "Browser Runtime Inspector".to_owned(),
            scope_summary:
                "Browser-runtime inspector breakpoint row for a runtime-only node with no saved source"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
            ],
            consumer_surfaces: vec!["browser_runtime".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_VISUAL_EDIT_REF.to_owned()],
            example_previews: vec![M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                target_id: "target:runtime:status-badge:0001".to_owned(),
                node_label: "StatusBadge".to_owned(),
                viewport_label: "Mobile — 360×800".to_owned(),
                active_breakpoint_token: "sm".to_owned(),
                theme_variant_token: "dark".to_owned(),
                state_variant_token: "active".to_owned(),
                device_class: M5DevicePreviewClass::MobileViewport,
                data_posture: M5PreviewDataPosture::Live,
                runtime_origin: M5PreviewRuntimeOrigin::LiveDevRuntime,
                freshness: PreviewFreshnessClass::Fresh,
                mapping_quality: M5BreakpointMappingQuality::Unmapped,
                sync_class: SourceSyncClass::RuntimeOnlyNoSource,
                source_span_ref: None,
            })],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
        // Framework-pack preview — a live, source-anchored breakpoint on a tethered
        // device whose freshness went stale: the runtime truth discloses the stale
        // view and a drifted-from-source trigger fires. Proves AC1 + AC3.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::FrameworkPackPreview,
            preview_surface: PreviewSurface::DeviceOrSimulatorPreview,
            owner_role: "Framework Packs".to_owned(),
            scope_summary:
                "Framework-pack device preview row for a tethered device whose live view went stale"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
                M5VisualDesignerDowngradeTrigger::RuntimeUnavailable,
            ],
            consumer_surfaces: vec!["framework_pack".to_owned(), "docs_demo".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF.to_owned()],
            example_previews: vec![M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                target_id: "target:framework:cart-badge:0001".to_owned(),
                node_label: "CartBadge".to_owned(),
                viewport_label: "Pixel 8 — 412×915".to_owned(),
                active_breakpoint_token: "sm".to_owned(),
                theme_variant_token: "dark".to_owned(),
                state_variant_token: "default".to_owned(),
                device_class: M5DevicePreviewClass::DeviceTethered,
                data_posture: M5PreviewDataPosture::Live,
                runtime_origin: M5PreviewRuntimeOrigin::TetheredDevice,
                freshness: PreviewFreshnessClass::Stale,
                mapping_quality: M5BreakpointMappingQuality::Approximate,
                sync_class: SourceSyncClass::InSyncFromSource,
                source_span_ref: Some("span:framework:cart-badge".to_owned()),
            })],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
        // Embedded shell designer — a captured snapshot on a custom viewport: the
        // runtime truth discloses the captured view and, being unmapped, the preview
        // degrades with an unmapped-source trigger. Proves AC1 + AC3.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::EmbeddedShellDesigner,
            preview_surface: PreviewSurface::EmbeddedWebviewPreview,
            owner_role: "Embedded Designer".to_owned(),
            scope_summary:
                "Embedded shell designer breakpoint row replaying a captured snapshot on a custom viewport"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
                M5VisualDesignerDowngradeTrigger::DriftedFromSource,
            ],
            consumer_surfaces: vec!["app_shell".to_owned(), "support_export".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF.to_owned()],
            example_previews: vec![M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                target_id: "target:shell:onboarding-card:0001".to_owned(),
                node_label: "OnboardingCard".to_owned(),
                viewport_label: "Custom — 1024×640".to_owned(),
                active_breakpoint_token: "custom".to_owned(),
                theme_variant_token: "light".to_owned(),
                state_variant_token: "default".to_owned(),
                device_class: M5DevicePreviewClass::CustomViewport,
                data_posture: M5PreviewDataPosture::Captured,
                runtime_origin: M5PreviewRuntimeOrigin::CapturedSnapshot,
                freshness: PreviewFreshnessClass::Aging,
                mapping_quality: M5BreakpointMappingQuality::Unmapped,
                sync_class: SourceSyncClass::InSyncFromSource,
                source_span_ref: None,
            })],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
        // Support-export replay — a captured snapshot on a simulator whose freshness
        // is unknown: the runtime truth discloses the captured view and an
        // unidentified-posture trigger fires. Proves AC1 + AC3.
        M5BreakpointSurfaceRow {
            surface_family: M5VisualDesignSurfaceFamily::SupportExportReplay,
            preview_surface: PreviewSurface::SupportExportProjection,
            owner_role: "Support Export".to_owned(),
            scope_summary:
                "Support-export replay of a captured breakpoint preview with unknown freshness"
                    .to_owned(),
            device_classes: DEVICE_CLASS_ALL.to_vec(),
            data_postures: DATA_POSTURE_ALL.to_vec(),
            export_fields: all_export_fields(),
            downgrade_triggers: vec![
                M5VisualDesignerDowngradeTrigger::UnidentifiedPosture,
                M5VisualDesignerDowngradeTrigger::UnmappedSource,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: vec![M5_BREAKPOINT_PREVIEW_ARTIFACT_REF.to_owned()],
            example_previews: vec![M5BreakpointPreviewCase::resolved(M5BreakpointPreviewInput {
                target_id: "target:support:list-item:0001".to_owned(),
                node_label: "ListItemRow".to_owned(),
                viewport_label: "iPhone 15 — 393×852".to_owned(),
                active_breakpoint_token: "sm".to_owned(),
                theme_variant_token: "dark".to_owned(),
                state_variant_token: "pressed".to_owned(),
                device_class: M5DevicePreviewClass::SimulatorPreview,
                data_posture: M5PreviewDataPosture::Captured,
                runtime_origin: M5PreviewRuntimeOrigin::CapturedSnapshot,
                freshness: PreviewFreshnessClass::Unknown,
                mapping_quality: M5BreakpointMappingQuality::Unmapped,
                sync_class: SourceSyncClass::InSyncFromSource,
                source_span_ref: None,
            })],
            blurs_live_vs_mock: false,
            hides_stale_or_captured: false,
            drops_source_anchor_silently: false,
            invents_local_preview_labels: false,
        },
    ]
}

fn seeded_governance_review() -> M5BreakpointGovernanceReview {
    M5BreakpointGovernanceReview {
        one_primitive_carries_row_cue_continuity: true,
        live_mock_captured_always_disclosed: true,
        switching_stays_source_anchored: true,
        selection_continuity_preserved_across_targets: true,
        degrade_explained_in_support_export: true,
        no_surface_invents_second_preview_grammar: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5BreakpointConsumerProjection {
    M5BreakpointConsumerProjection {
        preview_consumers_use_shared_primitive: true,
        resolver_reads_single_preview_model: true,
        cue_reads_single_runtime_truth_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5BreakpointReleasePosture {
    M5BreakpointReleasePosture {
        release_packet_ref:
            "artifacts/release/m5-breakpoint-device-preview-proof/support_export.json".to_owned(),
        preview_audit_ref: "artifacts/components/m5-breakpoint-device-preview-primitive.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    vec![
        M5_BREAKPOINT_PREVIEW_SCHEMA_REF.to_owned(),
        M5_BREAKPOINT_PREVIEW_DOC_REF.to_owned(),
        M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF.to_owned(),
        M5_BREAKPOINT_PREVIEW_ARTIFACT_REF.to_owned(),
        M5_BREAKPOINT_PREVIEW_VISUAL_EDIT_REF.to_owned(),
    ]
}
