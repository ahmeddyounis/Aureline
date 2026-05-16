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
//! - [`crash_linkage`] — support preview row generation for
//!   [`aureline_crash::CrashIncidentTrail`].
//! - [`notices`] — metadata-only notice digest preview row generation.

pub mod crash_linkage;
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
