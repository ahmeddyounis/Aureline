//! Safe-mode entry path, recovery-ladder rung stubs, and crash-loop
//! containment offers used by the live shell.
//!
//! The recovery surface is the protected path the shell falls through when
//! the normal start sequence cannot run safely or when a runtime fault
//! triggers the supervisor's crash-loop budget. It shares one truth
//! vocabulary with the restore-prompt projection in [`crate::restore`] and
//! the suspicious-content detector in `aureline-content-safety`, so any
//! later preview or recovery surface can consume the same records without
//! re-deriving state.
//!
//! Three honesty invariants ride on every rung:
//!
//! 1. **No silent widening.** Entering safe mode narrows the surface set;
//!    exiting safe mode is always an explicit, reviewed user action.
//! 2. **No state deletion as recovery.** Cache/index repair stubs and
//!    extension quarantine never discard user-authored files, journals,
//!    workspace trust, credentials, or session-restore artifacts.
//! 3. **Evidence stays exportable.** Every crash-loop containment record
//!    surfaces an export-evidence offer alongside the recovery rungs so the
//!    user can hand off a packet without leaving safe mode first.
//!
//! Cross-surface safe-preview / copy / export product depth is intentionally
//! out of scope here; this module only seeds the shell-level recovery entry
//! and the substrate contract that downstream rows consume.

pub mod crash_loop;
pub mod ladder;
pub mod safe_mode;
pub mod suspicious_save;

pub use crash_loop::{
    CrashLoopContainmentOffer, CrashLoopContainmentRecord, CrashLoopOfferKey,
    CrashLoopReasonClass, CRASH_LOOP_RECORD_SCHEMA_VERSION,
};
pub use ladder::{
    RecoveryLadderRung, RecoveryLadderRungProjection, RecoveryLadderRungState,
    RECOVERY_LADDER_SCHEMA_VERSION,
};
pub use safe_mode::{
    materialize_safe_mode_profile, write_safe_mode_profile_log, SafeModeEntryReason,
    SafeModeProfileRecord, SAFE_MODE_PROFILE_SCHEMA_VERSION,
};
pub use suspicious_save::{
    annotate_save_review_with_suspicious_content, SaveReviewSuspiciousContentAnnotation,
};
