//! Support-bundle manifest, redaction defaults, local preview, and exact-build capture.
//!
//! See [`crate`] for the seed's posture, what it owns, and what it does
//! not own. The submodules are intentionally narrow:
//!
//! - [`exact_build`] — quotes the canonical `aureline_build_info` record so
//!   the manifest's build identity matches the running binary verbatim.
//! - [`redaction`] — the local-first default redaction profile vocabulary
//!   and rule refs. Mirrors `support.redaction.local_first_default`.
//! - [`vocabulary`] — frozen string tokens shared by the manifest, the
//!   shell copy, and the docs (data class, redaction state, decision
//!   class, exclusion reason, ...).
//! - [`manifest`] — the [`SupportBundleManifest`] record and the
//!   [`SupportBundlePreviewItem`] row.
//! - [`preview`] — [`SupportBundlePreviewBuilder`] and
//!   [`SupportBundlePreview`]: the live local-preview projection the
//!   chrome renders before any export step.
//! - [`deletion_and_hold`] — shared deletion-honesty labels, held-record
//!   selectors, and support destruction-receipt rows.
//! - [`evidence_timeline`] — chronology-ordered delete/hold evidence
//!   timeline packets for support export and headless review.
//! - [`crash_linkage`] — support preview row generation for
//!   [`aureline_crash::CrashIncidentTrail`].
//! - [`notices`] — metadata-only notice digest preview row generation.

pub mod crash_linkage;
pub mod deletion_and_hold;
pub mod evidence_timeline;
pub mod exact_build;
pub mod manifest;
pub mod notices;
pub mod preview;
pub mod records;
pub mod redaction;
pub mod vocabulary;

pub use aureline_crash::{
    symbolicate_exact_build, CrashDumpManifest, CrashEnvelope, CrashFrame, CrashIncidentTrail,
    CrashIncidentTrailInputs, CrashModule, CrashModuleIdentity, ExactBuildSymbolicationError,
    ExactBuildSymbolicationInput, InTreeSymbolFile, InTreeSymbolFrame, InTreeSymbolModule,
    IncidentEvidenceKind, IncidentEvidenceRef, ModuleIncidentSummary, ModuleMappingQuality,
    NextSafeAction, NextSafeActionKind, SupportBundleLinkage, SupportBundleLinkageState,
    SymbolicatedModuleResult, SymbolicationReport, SymbolicationState,
};
pub use crash_linkage::{
    crash_incident_trail_preview, crash_incident_trail_seed, crash_symbolicated_frame_projections,
    SUPPORT_ITEM_CRASH_INCIDENT_TRAIL,
};
pub use deletion_and_hold::{
    add_destruction_receipt_preview_item, deletion_honesty_disclosure_for_packet,
    destruction_receipt_preview_item_seed, evaluate_support_destruction_receipt,
    held_record_selectors_for_beta_contractual_classes, select_held_records, DeletionHoldError,
    DeletionHonestyDisclosure, DeletionHonestyState, DestructionActionClass,
    DestructionLocalityClass, DestructionReasonClass, DestructionReceiptCounts,
    DestructionReceiptPolicyContext, DestructionReceiptState, DestructionResultClass,
    DestructionScopeRef, HeldRecordSelector, SupportDestructionReceiptInputs,
    SupportDestructionReceiptRecord, DELETION_HOLD_TRUTH_DOC_REF,
    GOVERNANCE_DESTRUCTION_RECEIPT_SCHEMA_REF, SUPPORT_DESTRUCTION_RECEIPT_RECORD_KIND,
    SUPPORT_DESTRUCTION_RECEIPT_REDACTION_CLASS, SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_REF,
    SUPPORT_DESTRUCTION_RECEIPT_SCHEMA_VERSION, SUPPORT_ITEM_DESTRUCTION_RECEIPT,
};
pub use evidence_timeline::{
    add_evidence_timeline_preview_item, evaluate_evidence_timeline_packet,
    evidence_timeline_preview_item_seed, EvidenceTimelineActorClass,
    EvidenceTimelineCurrentStateClass, EvidenceTimelineError, EvidenceTimelineEvent,
    EvidenceTimelineEventInput, EvidenceTimelineLocationClass, EvidenceTimelinePacket,
    EvidenceTimelinePacketInput, EvidenceTimelineRetainedReasonClass, EvidenceTimelineSourceClass,
    EvidenceTimelineStateClass, EvidenceTimelineStateCounts, EvidenceTimelineTimeContext,
    EvidenceTimelineTimezoneBasisClass, EVIDENCE_TIMELINE_ARTIFACT_REF, EVIDENCE_TIMELINE_DOC_REF,
    EVIDENCE_TIMELINE_EVENT_RECORD_KIND, EVIDENCE_TIMELINE_PACKET_RECORD_KIND,
    EVIDENCE_TIMELINE_REDACTION_CLASS, EVIDENCE_TIMELINE_SCHEMA_REF,
    EVIDENCE_TIMELINE_SCHEMA_VERSION, EVIDENCE_TIMELINE_VOCABULARY_REF,
    SUPPORT_ITEM_EVIDENCE_TIMELINE_PACKET,
};
pub use exact_build::ExactBuildCapture;
pub use manifest::{
    ActionPolicySourceContext, ActionReconstructionContext, ActionabilityImpact,
    ActionabilityWarning, BuildIdentity, CollectionContext, CrashSymbolicatedFrameProjection,
    DiagnosisLatencyMeasurementProjection, DiagnosisLatencyMeasurementState,
    DiagnosisLatencyScorecardProjection, ExcludedClass, FileSectionIdentity, ParityBinding,
    PolicyContext, PolicyLock, PolicyNote, PreviewClassificationSummary, PreviewExportParity,
    Redaction, RedactionControl, RedactionReport, ReopenAfterExportPath, ReviewDecision,
    SecretScanSummary, SizeEstimate, SupportBundleManifest, SupportBundlePreviewItem,
    COLLECTION_SCHEMA_VERSION, SUPPORT_BUNDLE_DIAGNOSIS_LATENCY_SCORECARD_RECORD_KIND,
    SUPPORT_BUNDLE_DIAGNOSIS_LATENCY_SCORECARD_SCHEMA_VERSION, SUPPORT_BUNDLE_MANIFEST_RECORD_KIND,
    SUPPORT_BUNDLE_PREVIEW_ITEM_RECORD_KIND,
};
pub use notices::{
    add_notice_digest_preview_item, notice_digest_preview_item_seed, SUPPORT_ITEM_NOTICE_DIGEST,
};
pub use preview::{
    ActionReconstructionSeed, DiagnosisLatencyScorecardProjectionSeed, PreviewItemSeed,
    SupportBundlePreview, SupportBundlePreviewBuilder, SupportBundlePreviewError,
    SUPPORT_BUNDLE_PREVIEW_RECORD_KIND, SUPPORT_BUNDLE_PREVIEW_SEED_SCOPE_NOTICE,
};
pub use records::{
    add_records_governance_preview_item, evaluate_records_governance_packet,
    records_governance_preview_item_seed, ArtifactClass, ChainOfCustodyEvent, CustodyActionClass,
    CustodyActorClass, CustodyLocationClass, DestructionCaveatClass, HoldClass, HoldState,
    RecordsGovernanceError, RecordsGovernanceInputs, RecordsGovernancePacket, RetentionOwnerClass,
    RECORDS_GOVERNANCE_PACKET_DOC_REF, RECORDS_GOVERNANCE_PACKET_RECORD_KIND,
    RECORDS_GOVERNANCE_PACKET_REDACTION_CLASS, RECORDS_GOVERNANCE_PACKET_SCHEMA_REF,
    RECORDS_GOVERNANCE_PACKET_SCHEMA_VERSION, SUPPORT_ITEM_RECORDS_GOVERNANCE_PACKET,
};
pub use redaction::LocalFirstDefaults;
pub use vocabulary::{
    ActionabilityImpactClass, ActorClass, DiagnosticDataClass, ExcludedReasonClass,
    HighRiskContentClass, PolicyNoteSeverity, RedactionState, ReleaseChannelClass,
    ReviewDecidedByClass, ReviewDecisionClass, SecretScanState, TrustState,
};
