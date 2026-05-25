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
pub mod modes;
pub mod orientation;
pub mod orientation_aids;
pub mod outline;
pub mod paint;
pub mod recovery_state_lineage;
pub mod save_fidelity_lineage;
pub mod selection;
pub mod text_nav;
pub mod undo;
pub mod viewport;

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
pub use undo::UndoGroupSummary;
pub use viewport::{
    CaretMove, EditorAction, EditorViewport, EditorViewportSnapshot, ImeComposition,
    SecondarySelectionSnapshot, SelectionDelta, TextPoint, ViewportDamage,
};
