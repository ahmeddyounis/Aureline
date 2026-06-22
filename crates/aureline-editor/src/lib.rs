//! Editor viewport, composition, and paint primitives.
//!
//! This crate owns the canonical editor viewport model: scroll offsets, caret
//! and selection state, line-layout caching, and the software compositor used
//! by the current desktop shell raster path. Higher layers (shell zones,
//! command surfaces, and future multi-window wiring) should treat the types in
//! this crate as the single source of truth for editor viewport paint and
//! invalidation semantics.

#![doc(html_root_url = "https://docs.rs/aureline-editor/0.0.0")]

pub mod assist;
pub mod clipboard;
pub mod find_replace;
pub mod highlight;
pub mod large_file;
pub mod large_file_mode;
pub mod large_file_posture;
pub mod m5_advanced_editing;
pub mod m5_assist_descriptors;
pub mod m5_completion_rows;
pub mod m5_constrained_assist;
pub mod m5_editor_assist;
pub mod m5_hover_peek;
pub mod m5_signature_snippet;
pub mod modes;
pub mod orientation;
pub mod orientation_aids;
pub mod outline;
pub mod paint;
pub mod recovery_state_lineage;
pub mod save_fidelity_lineage;
pub mod selection;
pub mod stabilize_clipboard_dragdrop_rich_content_and_paste_guardrails;
pub mod stabilize_modal_editing_leader_register_safety;
pub mod stabilize_orientation_aids_breadcrumbs_folds_minimap;
pub mod text_nav;
pub mod undo;
pub mod viewport;
pub mod voice_input;

pub use assist::{
    AssistContractError, AssistSchemaVersion, AssistSessionStore, AssistSourceCounts,
    AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass,
    AssistSourceLabelProjection, AssistSurfaceSnapshot, AssistSurfaceSnapshotRequest,
    AssistSurfaceStateClass, CodeActionPreviewDecisionClass, CodeActionPreviewRecord,
    CodeActionPreviewRequest, CodeActionPreviewSchemaVersion, CompletionAcceptanceContract,
    CompletionItemInit, CompletionItemKindClass, CompletionItemRecord, CompletionListRequest,
    CompletionListSnapshot, CompletionSideEffectClass, QuickFixEvidenceTrustClass,
    SignatureHelpInit, SignatureHelpRecord, SignaturePlacementClass, SnippetCursorPostureClass,
    SnippetImePostureClass, SnippetKeyIntentClass, SnippetKeyOutcomeClass, SnippetKeyOutcomeRecord,
    SnippetSessionController, SnippetSessionInit, SnippetSessionRecord, SnippetSessionStateClass,
    SnippetTabBehaviorClass, SnippetUnrelatedKeyPolicyClass, ASSIST_SCHEMA_VERSION,
    CODE_ACTION_PREVIEW_SCHEMA_VERSION,
};
pub use find_replace::{FindOptions, FindReplaceError, FindReplaceMode, FindReplaceState};
pub use highlight::{
    EditorTextRange, HighlightOverlaySet, HighlightSpan, SyntaxHighlightKind,
    SyntaxHighlightSourceClass, SyntaxHighlightSpan,
};
pub use large_file::{
    open_document, BomKind, ClassificationDecision, ClassificationPolicy, DocumentOpenDisposition,
    DocumentOpenError, DocumentOpenOutcome, FileMode, LargeFileDocument, LargeFileModeNotice,
    LargeFileOverrideInfo, LargeFileTrigger, LargeFileViewer, LargeFileViewerConfig,
    LargeFileViewerError, NormalDocument, ReaderMetrics,
};
pub use large_file_mode::{
    default_limited_mode_capabilities, LimitedModeActivationTrigger, LimitedModeCapabilityRecord,
    LimitedModeCapabilityState, LimitedModeEditPolicyClass, LimitedModeFileRecord,
    LimitedModeOverrideAction, LimitedModeSafePreviewClass, LimitedModeWritePolicyClass,
    LIMITED_MODE_FILE_RECORD_KIND, LIMITED_MODE_FILE_SCHEMA_REF, LIMITED_MODE_FILE_SCHEMA_VERSION,
};
pub use large_file_posture::{
    default_large_file_inspection_hooks, large_file_posture_lines, project_large_file_posture,
    project_large_file_posture_with_hooks, InspectionHook, InspectionHookClass,
    LargeFileActivationSummary, LargeFileClassificationObservation, LargeFilePostureNarrowReason,
    LargeFilePostureQualification, LargeFilePostureRecord, PreviewFidelitySummary,
    RestrictedWritePosture, LARGE_FILE_POSTURE_RECORD_KIND, LARGE_FILE_POSTURE_SCHEMA_REF,
    LARGE_FILE_POSTURE_SCHEMA_VERSION,
};
pub use m5_advanced_editing::{
    advanced_editing_model, advanced_editing_model_lines, AdvancedEditingInvariant,
    AdvancedEditingModel, AdvancedEditorSnapshot, FoldRiskClass, FoldRiskSummary,
    OverviewAidParity, RenderAwarenessPolicy, SelectionModeClass, SelectionSemanticsClass,
    SelectionSummaryStrip, UnsupportedOperationNote, M5_ADVANCED_EDITING_AS_OF,
    M5_ADVANCED_EDITING_MODEL_ID, M5_ADVANCED_EDITING_RECORD_KIND, M5_ADVANCED_EDITING_SCHEMA_REF,
    M5_ADVANCED_EDITING_SCHEMA_VERSION,
};
pub use m5_assist_descriptors::{
    assist_descriptor_model, assist_descriptor_model_lines, AccessibilityProfile,
    ActionabilityClass, AssistConfidenceClass, AssistDescriptor, AssistDescriptorFamily,
    AssistDescriptorModel, AssistFreshnessClass, DensityTier, DescriptorSource, ModelInvariant,
    MotionClass, PlacementClass, PrecedenceConflictCase, RenderContext, ResolutionScenario,
    ResolvedDescriptor, SuppressionReason, TextAnchor, VisibilityVerdict, ZoomTier,
    M5_ASSIST_DESCRIPTORS_AS_OF, M5_ASSIST_DESCRIPTORS_MODEL_ID, M5_ASSIST_DESCRIPTORS_RECORD_KIND,
    M5_ASSIST_DESCRIPTORS_SCHEMA_REF, M5_ASSIST_DESCRIPTORS_SCHEMA_VERSION,
};
pub use m5_completion_rows::{
    completion_row_model, completion_row_model_lines, AdditionalEditCue, CompletionAssistClass,
    CompletionAvailabilityClass, CompletionProviderPosture, CompletionRow, CompletionRowCounts,
    CompletionRowInit, CompletionRowInvariant, CompletionRowModel, CompletionRowSnapshot,
    TrustWeightClass, M5_COMPLETION_ROWS_AS_OF, M5_COMPLETION_ROWS_MODEL_ID,
    M5_COMPLETION_ROWS_RECORD_KIND, M5_COMPLETION_ROWS_SCHEMA_REF,
    M5_COMPLETION_ROWS_SCHEMA_VERSION,
};
pub use m5_constrained_assist::{
    constrained_assist_model, constrained_assist_model_lines, AssistNarrowingCell,
    ConstrainedAssistInvariant, ConstrainedAssistModel, ConstrainedFileStateClass,
    ConstrainedStateProfile, ConsumerSurfaceProof, DegradedProviderCase, NarrowReasonClass,
    NextSafeActionClass, M5_CONSTRAINED_ASSIST_AS_OF, M5_CONSTRAINED_ASSIST_MODEL_ID,
    M5_CONSTRAINED_ASSIST_RECORD_KIND, M5_CONSTRAINED_ASSIST_SCHEMA_REF,
    M5_CONSTRAINED_ASSIST_SCHEMA_VERSION,
};
pub use m5_editor_assist::{
    editor_assist_matrix, editor_assist_matrix_lines, AssistChannelClass, AssistDegradeClass,
    ClassDescriptor, CodeLensClass, DecorationClass, EditorAssistMatrix, EditorLayerClass,
    EditorSurfaceClass, HoverPeekModeClass, IdentityContract, InlayHintClass, MatrixInvariant,
    MicroSurfaceKind, PrecedenceLayer, SignatureHelpStateClass, SupportExportMinimum,
    SurfaceAssistCell, SurfaceAssistProfile, TruthTier, M5_EDITOR_ASSIST_AS_OF,
    M5_EDITOR_ASSIST_MATRIX_ID, M5_EDITOR_ASSIST_RECORD_KIND, M5_EDITOR_ASSIST_SCHEMA_REF,
    M5_EDITOR_ASSIST_SCHEMA_VERSION,
};
pub use m5_hover_peek::{
    hover_peek_model, hover_peek_model_lines, HoverPeekCard, HoverPeekContextClass,
    HoverPeekInvariant, HoverPeekModel, HoverPeekPresentationClass, HoverPeekSnapshot,
    HoverPeekStateClass, HoverPeekTargetRef, MappingQualityClass, PeekPromotion,
    PeekPromotionPathClass, RawRenderedModeClass, M5_HOVER_PEEK_AS_OF, M5_HOVER_PEEK_MODEL_ID,
    M5_HOVER_PEEK_RECORD_KIND, M5_HOVER_PEEK_SCHEMA_REF, M5_HOVER_PEEK_SCHEMA_VERSION,
};
pub use m5_signature_snippet::{
    signature_snippet_model, signature_snippet_model_lines, AcceptSideEffectClass,
    AssistBlockReason, SignatureCard, SignatureSnippetInvariant, SignatureSnippetModel,
    SignatureSnippetSnapshot, SnippetExitPath, SnippetStrip, M5_SIGNATURE_SNIPPET_AS_OF,
    M5_SIGNATURE_SNIPPET_MODEL_ID, M5_SIGNATURE_SNIPPET_RECORD_KIND,
    M5_SIGNATURE_SNIPPET_SCHEMA_REF, M5_SIGNATURE_SNIPPET_SCHEMA_VERSION,
};
pub use modes::{
    build_alpha_mode_state_record, AlphaModeStateInput, EditorModeClass, EditorModeStateRecord,
    MacroReplayOutcomeClass, MacroReplayReviewRecord, ModeRecoveryAction, PendingOperatorRecord,
    RegisterRouteAvailability, RegisterRouteKind, RegisterRouteRecord, SequenceGuideOption,
    SequenceGuideRecord, SequenceGuideState, MODE_STATE_SCHEMA_VERSION,
};
pub use orientation::{
    build_alpha_orientation_truth_record, AlphaOrientationInput, BreadcrumbContinuityRecord,
    EditorOrientationTruthRecord, FoldSummaryRecord, HiddenStateCounts, MultiCursorIndicatorRecord,
    OrientationAidAvailability, OverviewAidKind, OverviewAidRecord,
    ORIENTATION_TRUTH_SCHEMA_VERSION,
};
pub use orientation_aids::{
    build_beta_orientation_aid_state_record, BetaOrientationAidInput,
    BreadcrumbContinuityStateRecord, FoldSummaryStateRecord, GutterMarkerStateRecord,
    HiddenMarkerCount, MarkerFamilyClass, MultiCursorAttributionRecord, MultiCursorModePosture,
    OrientationAidAvailabilityClass, OrientationAidStateRecord, OrientationSurfaceClass,
    OverviewAidKindClass, OverviewAidStateRecord, UndoGroupingClass,
    FOLD_SUMMARY_STATE_SCHEMA_VERSION, ORIENTATION_AID_STATE_SCHEMA_VERSION,
};
pub use outline::{
    EditorStructuralSnapshot, FoldRange, FoldVisibilityState, OutlineNode, OutlineNodeKind,
    StructuralEditorAnalyzer, StructuralFeatureState, StructuralProviderClass,
    StructuralSnapshotSchemaVersion, StructuralSurfaceState,
};
pub use paint::{EditorTextRuntime, ViewportCompositor, ViewportPaintStyle};
pub use recovery_state_lineage::{
    project_recovery_state_lineage, recovery_state_lineage_lines, ActorLineageSummary,
    BufferRecoverySummary, CanonicalPathTruth, CompensationPostureClass, RecoveryNarrowReason,
    RecoveryStableQualification, RecoveryStateLineageRecord, RestoreSafetyPosture,
    UndoGroupLineageEntry, UndoGroupObservation, UndoRecoveryClass,
    RECOVERY_STATE_LINEAGE_RECORD_KIND, RECOVERY_STATE_LINEAGE_SCHEMA_REF,
    RECOVERY_STATE_LINEAGE_SCHEMA_VERSION,
};
pub use save_fidelity_lineage::{
    project_save_fidelity_lineage, save_fidelity_lineage_lines, FixActionClass,
    LineageNarrowReason, LineageStableQualification, PreviewReason, RecoveryActionClass,
    SaveFidelityLineageRecord, SaveParticipantLineageEntry, SaveParticipantStage,
    SourceFidelitySummary, SAVE_FIDELITY_LINEAGE_RECORD_KIND, SAVE_FIDELITY_LINEAGE_SCHEMA_REF,
    SAVE_FIDELITY_LINEAGE_SCHEMA_VERSION,
};
pub use selection::{CaretSelection, SelectionState, TextEditOutcome, TextEditScope};
pub use stabilize_clipboard_dragdrop_rich_content_and_paste_guardrails::{
    transfer_safety_corpus, BoundaryClass, BoundaryContext, DropPreview, DropVerb,
    LargeTransferFeedback, PasteGuardrail, RecoveryClass, RepresentationTruth, RichContentTrust,
    RichTrustClass, SensitiveReview, SurfaceProjection, TransferActionClass,
    TransferRepresentationClass, TransferSafetyBuildError, TransferSafetyInput,
    TransferSafetyPacket, TransferSafetyScenario, TransferSurfaceClass, UndoGroupTruth,
    TRANSFER_SAFETY_CORPUS_AS_OF, TRANSFER_SAFETY_PACKET_RECORD_KIND, TRANSFER_SAFETY_SCHEMA_REF,
    TRANSFER_SAFETY_SCHEMA_VERSION,
};
pub use stabilize_modal_editing_leader_register_safety::{
    modal_editing_safety_corpus, BuildError, KeymapImportOutcomeClass,
    KeymapImportRegressionRecord, ModalEditingSafetyInput, ModalEditingSafetyPacket,
    ModalEditingSafetyScenario, SurfaceDowngradeKind, SurfaceDowngradeRecord,
    MODAL_CUE_LATENCY_BUDGET_MICROS, MODAL_EDITING_SAFETY_CORPUS_AS_OF,
    MODAL_EDITING_SAFETY_PACKET_RECORD_KIND, MODAL_EDITING_SAFETY_SCHEMA_REF,
    MODAL_EDITING_SAFETY_SCHEMA_VERSION,
};
pub use stabilize_orientation_aids_breadcrumbs_folds_minimap::{
    orientation_aids_stability_corpus, BuildError as OrientationAidsBuildError,
    OrientationAidsStabilityInput, OrientationAidsStabilityPacket,
    OrientationAidsStabilityScenario, ORIENTATION_AIDS_STABILITY_CORPUS_AS_OF,
    ORIENTATION_AIDS_STABILITY_PACKET_RECORD_KIND, ORIENTATION_AIDS_STABILITY_SCHEMA_REF,
    ORIENTATION_AIDS_STABILITY_SCHEMA_VERSION, ORIENTATION_AID_FILE_SWITCH_BUDGET_MICROS,
    ORIENTATION_AID_LATENCY_BUDGET_MICROS, ORIENTATION_AID_SCROLL_BUDGET_MICROS,
    ORIENTATION_AID_TYPING_BUDGET_MICROS,
};
pub use undo::UndoGroupSummary;
pub use viewport::{
    CaretMove, EditorAction, EditorViewport, EditorViewportSnapshot, ImeComposition,
    SecondarySelectionSnapshot, SelectionDelta, TextPoint, ViewportDamage,
};
pub use voice_input::{
    seeded_dictation_edit_parity_packet, CaptureStatus, CorrectionGesture, DictationApplyOutcome,
    DictationCaptureSession, DictationCaptureSummary, DictationEditParityPacket,
    DictationEditRecord, DictationEffectClass, DictationError, DictationIntent,
    DictationIntentClass, DictationParityInvariantManifest, DictationParityScenario,
    DictationRecognitionLocality, DictationScenarioOutcomeClass, DictationSurface,
    DictationSurfaceClass, DictationSurfaceCoverageRow, DictationSurfaceSupport, FormattingIntent,
    InterimDictation, PunctuationMark, UndoRedoRoundtrip, DICTATION_EDIT_PARITY_DOC_REF,
    DICTATION_EDIT_PARITY_FIXTURES_DIR_REF, DICTATION_EDIT_PARITY_PACKET_ID,
    DICTATION_EDIT_PARITY_PACKET_RECORD_KIND, DICTATION_EDIT_PARITY_SCHEMA_VERSION,
};
