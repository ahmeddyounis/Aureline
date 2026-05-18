//! Staged save coordinator and root-capability-aware write strategies.
//!
//! This module owns the canonical save sequencing for workspace-backed
//! documents:
//!
//! 1. stage a buffer snapshot,
//! 2. run save participants on staged content,
//! 3. compare-before-write against the pinned save target,
//! 4. select a write strategy using the root capability envelope, and
//! 5. commit the write through an atomic or declared degraded lane.
//!
//! Consumers MUST NOT bypass this coordinator with ad hoc filesystem writes,
//! because doing so breaks conflict safety, journaling, and later support/export
//! attribution.

pub mod coordinator;
pub mod drift_detection;
pub mod risk;
pub mod source_fidelity;
pub mod write_strategy;

pub use coordinator::{
    SaveParticipant, SaveParticipantError, SaveResult, StagedSaveCoordinator, StagedSaveRequest,
};
pub use drift_detection::{detect_external_drift, ExternalDriftConflict};
pub use risk::{
    summarize_staged_file_effect, FileEffectSummary, SaveParticipantCheckpointPolicyClass,
    SaveParticipantClass, SaveParticipantFixSafetyClass, SaveParticipantOutputOrigin,
    SaveParticipantReviewTriggerClass, SaveParticipantRiskDeclaration, SaveParticipantRiskEntry,
    SaveParticipantRiskOutcomeClass, SaveParticipantRiskReview, SaveParticipantRunStateClass,
    SourceFidelityRewriteClass, SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND,
    SAVE_PARTICIPANT_RISK_SCHEMA_REF, SAVE_PARTICIPANT_RISK_SCHEMA_VERSION,
};
pub use source_fidelity::{
    detect_and_decode_for_buffer, encode_for_save, source_fidelity_adjustments, BomStateDetected,
    DetectedEncoding, DetectionSource, ExecutableIntent, FinalNewlineDetected, NewlineModeDetected,
    SourceFidelityAdjustment, SourceFidelityOpenOutcome, SourceFidelityRecord,
};
pub use write_strategy::WriteStrategy;
