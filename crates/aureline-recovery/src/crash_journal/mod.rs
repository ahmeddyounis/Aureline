//! Dirty-buffer autosave journals and crash sentinels.
//!
//! The crash journal is the canonical, user-owned recovery state for dirty
//! buffers after abnormal termination. It is persisted separately from
//! disposable caches so recovery surfaces can offer guided restore, inspect,
//! and export flows without inferring behavior from raw logs.

mod marker;
mod records;
mod store;

pub use marker::{CrashMarkerGuard, CrashMarkerOutcome};
pub use records::{
    ActorClass, ActorSurfaceRecord, AutosaveJournalEntryRecord, AutosaveJournalSchemaVersion,
    BaseOnDiskTokenRecord, CaptureClass, CaptureDescriptorRecord, CaptureMode,
    CaptureOmissionReason, ChecksumAlgorithm, DecoderPosture, DowngradeReasonClass,
    EncodingLabelClass, ExternalChangeState, FinalNewlineState, FrameIntegrityState,
    GuidedChoiceClass, IdentityRelation, IntegrityRecord, NewlineMode, ObjectClass,
    ObjectIdentityRecord, ReplayIntegrityPosture, ReplayPostureClass, ReplayPostureRecord,
    RetentionClass, RetentionPostureRecord, SourceClass, SupportBundleInclusionState,
    SupportExportRecord, SurfaceClass, TextFormatRecord, TokenClass, TokenConfidenceClass,
};
pub use store::{CrashJournalError, CrashJournalStore};
pub use store::CrashJournalCaptureInput;
