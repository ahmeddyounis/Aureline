//! Terminal foundation: the PTY host abstraction and the canonical
//! terminal-session truth model.
//!
//! This crate owns:
//!
//! - one [`pty_host::PtyHost`] abstraction that manages every terminal session
//!   and the local PTY process/runtime behind it,
//!   and
//! - one [`pty_host::SessionHeader`] vocabulary that carries title, cwd hint,
//!   target identity, execution-context reference, trust posture, and
//!   local-vs-managed boundary cue — the same provenance tuple a tab/pane chip
//!   shows in the bottom panel and that a support export quotes verbatim.

#![doc(html_root_url = "https://docs.rs/aureline-terminal/0.0.0")]

/// Stable M4 event-normalization truth packet pinning task, test,
/// debug, and terminal event streams into one export-safe execution
/// ledger.
pub mod harden_task_test_debug_and_terminal_event_normalization;
/// Terminal header strip and target/cwd/runtime/restore chip projection.
pub mod headers;
/// Canonical terminal-session summary, export packet, clipboard posture,
/// shared-role, and restore-class contract consumed by every claimed M3 beta
/// terminal surface.
pub mod protocol_contract;
/// Terminal protocol corpus and conformance projections for escape handling,
/// paste review, clipboard writes, and restore-state proofs.
pub mod protocol_corpus;
pub mod pty_host;
/// Transcript / ended-session restore projection. Restored records never
/// claim a live shell survived; auto-rerun is always forbidden.
pub mod restore;
/// Bounded, redaction-aware scrollback ring used by transcript restore and
/// support / export bundles.
pub mod scrollback;
/// Stable M4 terminal-stabilization truth packet binding host-boundary
/// chips, clipboard posture, transcript export, and restore-no-rerun
/// semantics into one boundary truth.
pub mod stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export;

pub use harden_task_test_debug_and_terminal_event_normalization::{
    current_stable_event_normalization_truth_packet,
    ConsumerSurface as EventNormalizationConsumerSurface,
    ConsumerSurfaceBindingClass as EventNormalizationConsumerSurfaceBindingClass,
    DowngradeAutomationClass as EventNormalizationDowngradeAutomationClass,
    EnvelopeFieldClass as EventNormalizationEnvelopeFieldClass, EventNormalizationConfidenceClass,
    EventNormalizationConsumerProjection, EventNormalizationLaneClass, EventNormalizationRow,
    EventNormalizationRowClass, EventNormalizationTruthArtifactError,
    EventNormalizationTruthPacket, EventNormalizationTruthPacketInput,
    EventNormalizationTruthSupportExport, EvidenceClass as EventNormalizationEvidenceClass,
    FindingKind as EventNormalizationFindingKind,
    FindingSeverity as EventNormalizationFindingSeverity,
    KnownLimitClass as EventNormalizationKnownLimitClass,
    LifecycleEventClass as EventNormalizationLifecycleEventClass,
    PromotionState as EventNormalizationPromotionState,
    SourceKindClass as EventNormalizationSourceKindClass,
    SupportClass as EventNormalizationSupportClass,
    ValidationFinding as EventNormalizationValidationFinding,
    WedgeClass as EventNormalizationWedgeClass, EVENT_NORMALIZATION_TRUTH_ARTIFACT_DOC_REF,
    EVENT_NORMALIZATION_TRUTH_DOC_REF, EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR,
    EVENT_NORMALIZATION_TRUTH_PACKET_ARTIFACT_REF, EVENT_NORMALIZATION_TRUTH_PACKET_RECORD_KIND,
    EVENT_NORMALIZATION_TRUTH_SCHEMA_REF, EVENT_NORMALIZATION_TRUTH_SCHEMA_VERSION,
    EVENT_NORMALIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
pub use headers::{
    TerminalHeaderChip, TerminalHeaderChipKind, TerminalHeaderChipState, TerminalHeaderRecord,
    TerminalHeaderRestoreState, TerminalHeaderSourceKind, TerminalRuntimeChipSource,
    TERMINAL_HEADER_RECORD_KIND, TERMINAL_HEADER_SCHEMA_VERSION,
};
pub use portable_pty::PtySize;
pub use protocol_contract::{
    lifecycle_state_requires_reconnect, TerminalAiPromotedSlice, TerminalBoundary,
    TerminalBracketedPasteClass, TerminalClipboardPostureClass, TerminalDenialReasonClass,
    TerminalExportClass, TerminalExportPacket, TerminalLinkificationClass,
    TerminalLiveAuthorityClass, TerminalPromotedRangeProvenance, TerminalReconnectDriftClass,
    TerminalRecordingClass, TerminalRecoveryPosture, TerminalSessionClass, TerminalSessionSummary,
    TerminalSessionSummaryValidationReport, TerminalSharedRoleClass,
    TERMINAL_AI_PROMOTED_SLICE_RECORD_KIND, TERMINAL_EXPORT_PACKET_RECORD_KIND,
    TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION, TERMINAL_SESSION_SUMMARY_RECORD_KIND,
    TERMINAL_SESSION_SUMMARY_VALIDATION_REPORT_KIND,
};
pub use protocol_corpus::{
    canonical_sequence_for_normalized_event, evaluate_clipboard_write, evaluate_escape_control,
    evaluate_paste_review, normalize_terminal_protocol_sequence, restore_conformance_from_header,
    terminal_protocol_sequence_fixtures, TerminalClipboardSuppressionClass,
    TerminalClipboardWriteInput, TerminalClipboardWriteKind, TerminalClipboardWriteReport,
    TerminalEscapeControlInput, TerminalEscapeControlReport, TerminalGateDisposition,
    TerminalMouseButton, TerminalMouseEventKind, TerminalMouseModifiers, TerminalMouseProtocol,
    TerminalNormalizedProtocolEvent, TerminalOscColorRegister, TerminalPastePolicyResult,
    TerminalPasteReviewInput, TerminalPasteReviewReport, TerminalPasteSubmitBehavior,
    TerminalProtocolCorpusCaseKind, TerminalProtocolSequenceFixture,
    TerminalProtocolSequenceFixtureKind, TerminalRestoreConformanceReport,
    TerminalRestoreConformanceState, TERMINAL_ALPHA_REQUIRED_ESCAPE_SEQUENCE_TOKENS,
    TERMINAL_PROTOCOL_CORPUS_CASE_KIND, TERMINAL_PROTOCOL_CORPUS_FIXTURE_SET_ID,
    TERMINAL_PROTOCOL_CORPUS_MANIFEST_KIND, TERMINAL_PROTOCOL_CORPUS_SCHEMA_VERSION,
    TERMINAL_PROTOCOL_SEQUENCE_FIXTURE_RECORD_KIND,
};
pub use pty_host::{
    HostClass, OpenSessionRequest, PtyCommand, PtyHost, PtyHostError, PtyLaunchFailureReason,
    PtyOutputDrain, PtySession, PtySessionId, SessionHeader, SessionLifecycleState,
    SessionLifecycleTransition, TerminalEnvironmentScope, TerminalLastCommandClass,
    TerminalSessionRestoreMetadata, TerminalShellFamily, TerminalTrustState,
    DEFAULT_PTY_OUTPUT_RING_CAPACITY, DEFAULT_PTY_SIZE,
};
pub use restore::{
    decline_session_restore, restore_session_as_transcript, RestoreDeclinedReason,
    RestoredTerminalKind, RestoredTerminalRecord, TerminalRestoreDecision, TerminalRestoreLevel,
    RESTORED_TERMINAL_RECORD_KIND, RESTORED_TERMINAL_SCHEMA_VERSION,
    TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID,
};
pub use scrollback::{
    ScrollbackBound, ScrollbackLineRecord, ScrollbackRedactionClass, TerminalScrollback,
    TerminalScrollbackSnapshot, DEFAULT_SCROLLBACK_LINE_BOUND, SCROLLBACK_LINE_RECORD_KIND,
    SCROLLBACK_SCHEMA_VERSION, SCROLLBACK_SNAPSHOT_RECORD_KIND,
};
pub use stabilize_integrated_terminal_boundaries_clipboard_posture_transcript_export::{
    current_stable_terminal_stabilization_truth_packet,
    ClipboardPostureClass as TerminalStabilizationClipboardPostureClass,
    ConsumerSurface as TerminalStabilizationConsumerSurface,
    DowngradeAutomationClass as TerminalStabilizationDowngradeAutomationClass,
    EvidenceClass as TerminalStabilizationEvidenceClass,
    FindingKind as TerminalStabilizationFindingKind,
    FindingSeverity as TerminalStabilizationFindingSeverity,
    HostBoundaryFieldClass as TerminalStabilizationHostBoundaryFieldClass,
    KnownLimitClass as TerminalStabilizationKnownLimitClass,
    PromotionState as TerminalStabilizationPromotionState,
    SupportClass as TerminalStabilizationSupportClass, TerminalStabilizationConfidenceClass,
    TerminalStabilizationConsumerProjection, TerminalStabilizationLaneClass,
    TerminalStabilizationRow, TerminalStabilizationRowClass,
    TerminalStabilizationTruthArtifactError, TerminalStabilizationTruthPacket,
    TerminalStabilizationTruthPacketInput, TerminalStabilizationTruthSupportExport,
    TranscriptExportFieldClass as TerminalStabilizationTranscriptExportFieldClass,
    ValidationFinding as TerminalStabilizationValidationFinding,
    WedgeClass as TerminalStabilizationWedgeClass, TERMINAL_STABILIZATION_TRUTH_ARTIFACT_DOC_REF,
    TERMINAL_STABILIZATION_TRUTH_DOC_REF, TERMINAL_STABILIZATION_TRUTH_FIXTURE_DIR,
    TERMINAL_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF,
    TERMINAL_STABILIZATION_TRUTH_PACKET_RECORD_KIND, TERMINAL_STABILIZATION_TRUTH_SCHEMA_REF,
    TERMINAL_STABILIZATION_TRUTH_SCHEMA_VERSION,
    TERMINAL_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND,
};
