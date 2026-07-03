//! Representation-labeled safe-preview and copy/export seed wedge.
//!
//! This crate is the M1 bounded prototype that proves three risky preview
//! lanes can carry an honest representation label, a paired copy/export
//! action set, and a typed downgrade story end to end on one live shell
//! row:
//!
//! - **Risky text** — bidi controls, invisible formatters, and mixed-script
//!   confusables in plain UTF-8 text. The detector outcome that drives
//!   labelling comes verbatim from [`aureline_content_safety`]; this crate
//!   does not re-derive what is suspicious.
//! - **Oversized artifacts** — files / logs / captures whose visible body is
//!   only a windowed slice of the source. The wedge surfaces typed scope and
//!   transform tokens so a rendered preview can never silently claim full
//!   fidelity.
//! - **Generated content** — model-produced summaries / diffs / explanations.
//!   The wedge pins the on-screen representation to `generated`, requires a
//!   citation anchor list for any quoted authoritative bytes, and refuses to
//!   advertise a `copy_raw` action when no canonical source exists.
//!
//! Every preview the wedge emits ([`safe_preview::SafePreviewRecord`])
//! carries:
//!
//! - a [`safe_preview::PrototypeLabel`] chip so the chrome cannot quietly
//!   drop the wedge label;
//! - a typed [`safe_preview::ContentClass`] so support exports name the
//!   risky / oversized / generated lane;
//! - the [`aureline_content_safety::TrustClass`] of the source surface;
//!   the wedge MUST NOT mint a synonym;
//! - a [`safe_preview::OriginClass`] (user-authored vs generated vs unknown)
//!   so a `generated` preview never silently inherits raw-source identity;
//! - a [`safe_preview::CurrentlyVisibleRepresentation`] that names which
//!   representation the user is presently looking at; and
//! - a paired list of [`safe_preview::CopyExportOption`] rows, each
//!   carrying the
//!   [`aureline_content_safety::RepresentationActionId`] and
//!   [`aureline_content_safety::RepresentationClass`] vocabulary verbatim,
//!   plus typed scope / transform / omission tokens and an honest
//!   [`safe_preview::ShareSafety`] posture.
//!
//! The wedge is bounded:
//!
//! - It does not rewrite the broader content viewer; it only owns the
//!   preview record and the named shell consumer at
//!   [`crates/aureline-shell/src/safe_preview_card/`](../../../crates/aureline-shell/src/safe_preview_card/mod.rs).
//! - It does not invent new representation, action, scope, transform, or
//!   omission tokens. Every closed vocabulary mirrors the schema in
//!   [`/schemas/ux/representation_copy_export.schema.json`](../../../schemas/ux/representation_copy_export.schema.json)
//!   and the parity contract in
//!   [`/docs/ux/copy_export_representation_parity.md`](../../../docs/ux/copy_export_representation_parity.md).
//! - It does not silently widen authority. Calling
//!   [`safe_preview::SafePreviewRecord::validate`] surfaces every
//!   representation-honesty rule the spec freezes (e.g. risky text MUST
//!   offer both `copy_raw` and `copy_escaped` paired, generated previews
//!   MUST NOT advertise `copy_raw` without a canonical-source anchor,
//!   oversized previews MUST name a scope token other than
//!   `loaded_materialized_set` when the visible slice is windowed).
//!
//! The reviewer-facing landing page is
//! [`/docs/ux/m1_safe_preview_and_copy_export.md`](../../../docs/ux/m1_safe_preview_and_copy_export.md).

#![doc(html_root_url = "https://docs.rs/aureline-preview/0.0.0")]

pub mod add_visual_designer_fallback_parity_keyboard_and_screen_reader_navigation_and_no_drag_only_editing_rules;
pub mod browser_runtime_inspectors;
pub mod extension_provider_conformance;
pub mod freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix;
pub mod freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix;
pub mod implement_the_m5_breakpoint_and_device_preview_row_primitive;
pub mod implement_the_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive;
pub mod implement_the_m5_source_sync_chip_round_trip_conflict_and_generated_or_protected_boundary_primitive;
pub mod inspect_to_source_tree;
pub mod preview_drift_recovery;
pub mod preview_origin;
pub mod preview_runtime_certification;
pub mod preview_session_descriptors;
pub mod safe_preview;
pub mod visual_edit_transforms;

pub use add_visual_designer_fallback_parity_keyboard_and_screen_reader_navigation_and_no_drag_only_editing_rules::{
    current_m5_visual_designer_a11y_fallback_export,
    seeded_m5_visual_designer_a11y_fallback_packet, AccessibilityAutoNarrow,
    ComponentAccessibilityArtifactError, ComponentAccessibilityPacket,
    ComponentAccessibilityPacketInput, ComponentAccessibilityRow, ComponentAccessibilityStatus,
    ComponentAccessibilitySummary, ComponentAccessibilityViolation, CopyExportParity,
    DragEditingState, ExportSummaryState, M5AccessibilityRenderingSurface, M5FallbackModality,
    M5VisualDesignerConsumerSurface, NarrowingDisclosureState, NonVisualReachState,
    RenderingNarrowingDisclosure, VISUAL_DESIGNER_A11Y_FALLBACK_ARTIFACT_REF,
    VISUAL_DESIGNER_A11Y_FALLBACK_COMPONENT_MATRIX_REF, VISUAL_DESIGNER_A11Y_FALLBACK_CSV_REF,
    VISUAL_DESIGNER_A11Y_FALLBACK_DOC_REF, VISUAL_DESIGNER_A11Y_FALLBACK_FIXTURE_DIR,
    VISUAL_DESIGNER_A11Y_FALLBACK_RECORD_KIND, VISUAL_DESIGNER_A11Y_FALLBACK_REPORT_REF,
    VISUAL_DESIGNER_A11Y_FALLBACK_ROW_RECORD_KIND, VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_REF,
    VISUAL_DESIGNER_A11Y_FALLBACK_SCHEMA_VERSION,
};

pub use freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::{
    current_m5_preview_inspection_matrix_export, AttachDepthClass, MatrixConsumerProjection,
    MatrixEvidenceFreshness, MatrixGuardrails, PreviewInspectionMatrixArtifactError,
    PreviewInspectionMatrixPacket, PreviewInspectionMatrixPacketInput,
    PreviewInspectionMatrixViolation, PreviewInspectionRow, PreviewMatrixDowngradeTrigger,
    PreviewMatrixQualificationClass, PreviewSessionClass, PreviewSurface,
    RoundTripCapabilityClass, SourceSyncClass, M5_PREVIEW_INSPECTION_MATRIX_ARTIFACT_REF,
    M5_PREVIEW_INSPECTION_MATRIX_DOC_REF, M5_PREVIEW_INSPECTION_MATRIX_FIXTURE_DIR,
    M5_PREVIEW_INSPECTION_MATRIX_RECORD_KIND, M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_REF,
    M5_PREVIEW_INSPECTION_MATRIX_SCHEMA_VERSION, M5_PREVIEW_INSPECTION_MATRIX_SUMMARY_REF,
};

pub use freeze_the_m5_design_canvas_structure_tree_property_inspector_source_sync_and_breakpoint_preview_component_matrix::{
    current_m5_visual_designer_component_matrix_export, seeded_visual_designer_component_matrix,
    BreakpointPreviewRowDescriptor, ComponentRow, DegradedState, DesignCanvasDescriptor,
    M5BreakpointMappingQuality, M5CanvasState, M5ConflictResolutionRoute, M5DevicePreviewClass,
    M5PreviewDataPosture, M5PropertyValueState, M5PropertyWriteScope, M5RoundTripConflictClass,
    M5StructureNodeKind, M5SyncRecoveryRoute, M5VisualDesignerComponentFamily,
    M5VisualDesignerDowngradeTrigger, M5VisualDesignerRequiredLabel,
    PropertyInspectorRowDescriptor, RoundTripConflictBannerDescriptor, SourceSyncChipDescriptor,
    StructureTreeRowDescriptor, VisualDesignerComponentArtifactError, VisualDesignerComponentMatrix,
    VisualDesignerComponentMatrixInput, VisualDesignerComponentViolation,
    VisualDesignerConsumerProjection, VisualDesignerGuardrails,
    VISUAL_DESIGNER_COMPONENT_MATRIX_ARTIFACT_REF, VISUAL_DESIGNER_COMPONENT_MATRIX_DOC_REF,
    VISUAL_DESIGNER_COMPONENT_MATRIX_FIXTURE_DIR, VISUAL_DESIGNER_COMPONENT_MATRIX_RECORD_KIND,
    VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_REF, VISUAL_DESIGNER_COMPONENT_MATRIX_SCHEMA_VERSION,
    VISUAL_DESIGNER_COMPONENT_MATRIX_SUMMARY_REF,
};

pub use implement_the_m5_breakpoint_and_device_preview_row_primitive::{
    current_stable_m5_breakpoint_preview_export, resolve_breakpoint_preview,
    seeded_m5_breakpoint_preview_packet, M5BreakpointConsumerProjection,
    M5BreakpointContinuityAction, M5BreakpointExportField, M5BreakpointGovernanceReview,
    M5BreakpointPreviewArtifactError, M5BreakpointPreviewCase, M5BreakpointPreviewInput,
    M5BreakpointPreviewPacket, M5BreakpointPreviewPacketInput, M5BreakpointPreviewResolutionError,
    M5BreakpointPreviewViolation, M5BreakpointReleasePosture, M5BreakpointSurfaceRow,
    M5BreakpointVocabularySet, M5PreviewRuntimeOrigin, M5ResolvedBreakpointPreview,
    M5ResolvedContinuity, M5ResolvedDevicePreviewRow, M5ResolvedRuntimeTruthCue,
    M5_BREAKPOINT_PREVIEW_ARTIFACT_REF, M5_BREAKPOINT_PREVIEW_COMPONENT_MATRIX_REF,
    M5_BREAKPOINT_PREVIEW_CSV_REF, M5_BREAKPOINT_PREVIEW_DOC_REF,
    M5_BREAKPOINT_PREVIEW_FIXTURE_DIR, M5_BREAKPOINT_PREVIEW_RECORD_KIND,
    M5_BREAKPOINT_PREVIEW_REPORT_REF, M5_BREAKPOINT_PREVIEW_SCHEMA_REF,
    M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION, M5_BREAKPOINT_PREVIEW_VISUAL_EDIT_REF,
};

pub use implement_the_m5_design_canvas_structure_tree_and_property_inspector_selected_node_primitive::{
    current_stable_m5_selected_node_primitive_export, resolve_visual_selection,
    seeded_m5_selected_node_primitive_packet, M5PropertyEditInput, M5PropertyEditorKind,
    M5ResolvedCanvasFrame, M5ResolvedPropertyRow, M5ResolvedTreeRow, M5ResolvedVisualSelection,
    M5SelectedNodeConsumerProjection, M5SelectedNodeExportField, M5SelectedNodeGovernanceReview,
    M5SelectedNodePrimitiveArtifactError, M5SelectedNodePrimitivePacket,
    M5SelectedNodePrimitivePacketInput, M5SelectedNodePrimitiveViolation,
    M5SelectedNodeReleasePosture, M5SelectedNodeVocabularySet, M5VisualDesignSurfaceFamily,
    M5VisualDesignSurfaceRow, M5VisualSelectionCase, M5VisualSelectionInput,
    M5VisualSelectionResolutionError, M5VisualSupportState, M5_SELECTED_NODE_ARTIFACT_REF,
    M5_SELECTED_NODE_COMPONENT_MATRIX_REF, M5_SELECTED_NODE_CSV_REF, M5_SELECTED_NODE_DOC_REF,
    M5_SELECTED_NODE_FIXTURE_DIR, M5_SELECTED_NODE_PRIMITIVE_RECORD_KIND,
    M5_SELECTED_NODE_PRIMITIVE_SCHEMA_VERSION, M5_SELECTED_NODE_REPORT_REF,
    M5_SELECTED_NODE_SCHEMA_REF, M5_SELECTED_NODE_VISUAL_EDIT_REF,
};

pub use implement_the_m5_source_sync_chip_round_trip_conflict_and_generated_or_protected_boundary_primitive::{
    current_stable_m5_round_trip_honesty_export, resolve_round_trip_status,
    seeded_m5_round_trip_honesty_packet, M5ResolvedBoundaryNotice, M5ResolvedConflictBanner,
    M5ResolvedRoundTripStatus, M5ResolvedSourceSyncChip, M5ResolvedUnsupportedCard,
    M5RoundTripConsumerProjection, M5RoundTripExportField, M5RoundTripGovernanceReview,
    M5RoundTripHonestyArtifactError, M5RoundTripHonestyPacket, M5RoundTripHonestyPacketInput,
    M5RoundTripHonestyViolation, M5RoundTripReleasePosture, M5RoundTripResolutionError,
    M5RoundTripStatusCase, M5RoundTripStatusInput, M5RoundTripSurfaceRow, M5RoundTripVocabularySet,
    M5SourceBoundaryClass, M5SourceFirstFallback, M5SourceSyncChipState, M5WriteAuthority,
    M5_ROUND_TRIP_ARTIFACT_REF, M5_ROUND_TRIP_COMPONENT_MATRIX_REF, M5_ROUND_TRIP_CSV_REF,
    M5_ROUND_TRIP_DOC_REF, M5_ROUND_TRIP_FIXTURE_DIR, M5_ROUND_TRIP_HONESTY_RECORD_KIND,
    M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION, M5_ROUND_TRIP_REPORT_REF, M5_ROUND_TRIP_SCHEMA_REF,
    M5_ROUND_TRIP_VISUAL_EDIT_REF,
};

pub use browser_runtime_inspectors::{
    current_m5_browser_runtime_inspectors_export, BrowserRuntimeInspectorArtifactError,
    BrowserRuntimeInspectorPacket, BrowserRuntimeInspectorPacketInput,
    BrowserRuntimeInspectorViolation, BrowserRuntimeTargetKind, InspectorConsumerProjection,
    InspectorDowngradeTrigger, InspectorGuardrails, InspectorKind, InspectorMappingQualityClass,
    InspectorRow, MutationDescriptor, MutationReviewPosture, RedactionPostureClass,
    SessionContinuityClass, SessionFreshnessClass, SideEffectClass,
    BROWSER_RUNTIME_INSPECTORS_ARTIFACT_REF, BROWSER_RUNTIME_INSPECTORS_DOC_REF,
    BROWSER_RUNTIME_INSPECTORS_FIXTURE_DIR, BROWSER_RUNTIME_INSPECTORS_RECORD_KIND,
    BROWSER_RUNTIME_INSPECTORS_SCHEMA_REF, BROWSER_RUNTIME_INSPECTORS_SCHEMA_VERSION,
    BROWSER_RUNTIME_INSPECTORS_SUMMARY_REF,
};

pub use extension_provider_conformance::{
    current_m5_extension_provider_conformance_export, ClaimedRowRequirement,
    ConformanceConsumerProjection, ConformanceDowngradeTrigger, ConformanceGuardrails,
    HotReloadDeclarationClass, OperatingProfileClass, ProviderConformanceArtifactError,
    ProviderConformancePacket, ProviderConformancePacketInput, ProviderConformanceRow,
    ProviderConformanceViolation, ProviderDeclaration, ProviderOriginClass, ProviderStatusClass,
    RepairActionClass, RepairGuidance, EXTENSION_PROVIDER_CONFORMANCE_ARTIFACT_REF,
    EXTENSION_PROVIDER_CONFORMANCE_DOC_REF, EXTENSION_PROVIDER_CONFORMANCE_FIXTURE_DIR,
    EXTENSION_PROVIDER_CONFORMANCE_RECORD_KIND, EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_REF,
    EXTENSION_PROVIDER_CONFORMANCE_SCHEMA_VERSION, EXTENSION_PROVIDER_CONFORMANCE_SUMMARY_REF,
};

pub use inspect_to_source_tree::{
    current_m5_inspect_to_source_tree_export, ContinuityRoute, InspectNode,
    InspectToSourceTreeArtifactError, InspectToSourceTreePacket, InspectToSourceTreePacketInput,
    InspectToSourceTreeViolation, InspectTreeKind, MappingDowngradeTrigger,
    NodeMappingQualityClass, TreeConsumerProjection, TreeGuardrails,
    INSPECT_TO_SOURCE_TREE_ARTIFACT_REF, INSPECT_TO_SOURCE_TREE_DOC_REF,
    INSPECT_TO_SOURCE_TREE_FIXTURE_DIR, INSPECT_TO_SOURCE_TREE_RECORD_KIND,
    INSPECT_TO_SOURCE_TREE_SCHEMA_REF, INSPECT_TO_SOURCE_TREE_SCHEMA_VERSION,
    INSPECT_TO_SOURCE_TREE_SUMMARY_REF,
};

pub use preview_drift_recovery::{
    current_m5_preview_drift_recovery_drill_set_export, DriftEventClass,
    DriftRecoveryConsumerProjection, DriftRecoveryGuardrails, DriftRecoveryRoute,
    DriftRecoveryTrigger, DriftTruthSnapshot, PreviewDriftRecoveryDrill,
    PreviewDriftRecoveryDrillSet, PreviewDriftRecoveryDrillSetArtifactError,
    PreviewDriftRecoveryDrillSetInput, PreviewDriftRecoveryDrillSetViolation,
    PREVIEW_DRIFT_RECOVERY_DRILL_SET_ARTIFACT_REF, PREVIEW_DRIFT_RECOVERY_DRILL_SET_DOC_REF,
    PREVIEW_DRIFT_RECOVERY_DRILL_SET_FIXTURE_DIR, PREVIEW_DRIFT_RECOVERY_DRILL_SET_RECORD_KIND,
    PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF, PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_VERSION,
    PREVIEW_DRIFT_RECOVERY_DRILL_SET_SUMMARY_REF,
};

pub use preview_origin::{
    BrowserRuntimeSessionOrigin, BrowserSessionOriginClass, BrowserSessionScopeClass,
    CrossOriginPostureClass, DeviceCapabilityClass, HotReloadEventClass, HotReloadStateDescriptor,
    HotReloadStateRecoveryRoute, HotReloadUnderlyingStateClass, MutationActionKind,
    MutationBlastClass, MutationReviewRequirement, PreviewLaneClass, PreviewOriginClass,
    PreviewOriginDescriptor, PreviewOriginFinding, PreviewOriginLifecyclePhase,
    PreviewOriginSharingPosture, PreviewTargetClass, PreviewTargetDescriptor,
    PreviewTargetReducedCapabilityReason, ProtocolPostureClass, RuntimeMutationActionPlan,
    SourceMappingDescriptor, SourceMappingQualityClass, BROWSER_SESSION_ORIGIN_RECORD_KIND,
    BROWSER_SESSION_ORIGIN_SCHEMA_VERSION, HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND,
    HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION, PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND,
    PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION, PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND,
    PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION, RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND,
    RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION,
};
pub use preview_runtime_certification::{
    current_m5_preview_runtime_certification_export, CertificationClass,
    CertificationConsumerProjection, CertificationDowngradeTrigger, CertificationEvidenceFreshness,
    CertificationGuardrails, CertificationLane, CertificationRow, LaneProof, LaneProofStatus,
    PreviewRuntimeCertificationArtifactError, PreviewRuntimeCertificationPacket,
    PreviewRuntimeCertificationPacketInput, PreviewRuntimeCertificationViolation,
    PREVIEW_RUNTIME_CERTIFICATION_ARTIFACT_REF, PREVIEW_RUNTIME_CERTIFICATION_DOC_REF,
    PREVIEW_RUNTIME_CERTIFICATION_FIXTURE_DIR, PREVIEW_RUNTIME_CERTIFICATION_RECORD_KIND,
    PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_REF, PREVIEW_RUNTIME_CERTIFICATION_SCHEMA_VERSION,
    PREVIEW_RUNTIME_CERTIFICATION_SUMMARY_REF,
};
pub use preview_session_descriptors::{
    current_m5_preview_session_descriptor_set_export, PreviewConsumerSurface,
    PreviewDataPostureClass, PreviewFreshnessClass, PreviewSessionDescriptor,
    PreviewSessionDescriptorSet, PreviewSessionDescriptorSetArtifactError,
    PreviewSessionDescriptorSetInput, PreviewSessionDescriptorSetViolation,
    SessionConsumerProjection, SessionDowngradeTrigger, SessionGuardrails,
    PREVIEW_SESSION_DESCRIPTOR_SET_ARTIFACT_REF, PREVIEW_SESSION_DESCRIPTOR_SET_DOC_REF,
    PREVIEW_SESSION_DESCRIPTOR_SET_FIXTURE_DIR, PREVIEW_SESSION_DESCRIPTOR_SET_RECORD_KIND,
    PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_REF, PREVIEW_SESSION_DESCRIPTOR_SET_SCHEMA_VERSION,
    PREVIEW_SESSION_DESCRIPTOR_SET_SUMMARY_REF,
};
pub use safe_preview::{
    build_generated_content_preview, build_oversized_artifact_preview, build_risky_text_preview,
    ContentClass, CopyExportActionKind, CopyExportOption, CurrentlyVisibleRepresentation,
    GeneratedContentInput, OmissionReason, OmissionSummary, OriginClass, OversizedArtifactInput,
    PrototypeLabel, RiskyTextInput, SafePreviewClaimLimit, SafePreviewInvariantViolation,
    SafePreviewRecord, ScopeClass, ShareSafety, TransformKind, SAFE_PREVIEW_RECORD_KIND,
    SAFE_PREVIEW_SCHEMA_VERSION,
};
pub use visual_edit_transforms::{
    current_m5_visual_edit_transforms_export, PreviewDiffClass, ProtectedPathPosture,
    RollbackClass, TransformConstructClass, TransformManifest, UnsupportedConstructCard,
    UnsupportedConstructReason, VisualEditOutcomeClass, VisualEditRow,
    VisualEditTransformArtifactError, VisualEditTransformConsumerProjection,
    VisualEditTransformGuardrails, VisualEditTransformPacket, VisualEditTransformPacketInput,
    VisualEditTransformViolation, VISUAL_EDIT_TRANSFORMS_ARTIFACT_REF,
    VISUAL_EDIT_TRANSFORMS_DOC_REF, VISUAL_EDIT_TRANSFORMS_FIXTURE_DIR,
    VISUAL_EDIT_TRANSFORMS_RECORD_KIND, VISUAL_EDIT_TRANSFORMS_SCHEMA_REF,
    VISUAL_EDIT_TRANSFORMS_SCHEMA_VERSION, VISUAL_EDIT_TRANSFORMS_SUMMARY_REF,
};
