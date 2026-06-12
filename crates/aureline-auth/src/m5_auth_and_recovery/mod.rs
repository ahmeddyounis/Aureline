//! Managed sign-in, step-up, passkey, browser-handoff, deprovision, and
//! account-recovery cues with local-work continuity across M5 surfaces.
//!
//! The module mints one governed record that brings the M5 managed identity
//! events — sign-in, step-up, re-auth, session revocation, deprovision, and
//! account recovery — into one inspectable, local-first auth-boundary model
//! instead of an opaque "signed-in / signed-out" toggle. Each event discloses
//! its provider/owner, its system-browser handoff method and typed reason, its
//! passkey posture and keyboard-complete fallback, the managed capabilities a
//! live degraded condition pauses, and the local work that stays usable through
//! it. The record also accounts for where refresh credentials and delegated
//! handles live (and that they never leave the device), and replays the
//! passkey-unavailable, browser-handoff-failure, offline-identity,
//! policy-forced-sign-out, and deprovision-on-active-local-work drills as
//! security, accessibility, and recovery exercises, so the desktop shell, CLI
//! inspect, docs/help, and support exports consume one shared auth object model
//! instead of cloning auth status text.

pub mod corpus;
pub mod model;

#[cfg(test)]
mod tests;

pub use corpus::{m5_auth_and_recovery_corpus, M5AuthAndRecoveryScenario, CORPUS_AS_OF};
pub use model::{
    is_canonical_object_ref, AuthCondition, AuthDrill, AuthEventKind, AuthEventRow,
    AuthRecoveryClaim, AuthRecoveryPillars, AuthRecoveryQualification, AuthSurface, BrowserHandoff,
    BuildError, ConditionDisposition, ContinuityCeiling, CredentialClass, CredentialStorageRow,
    CredentialStoreClass, DrillCategory, DrillKind, FallbackPosture, HandoffMethod, HandoffReason,
    LocalCapabilityClass, LocalContinuityBlock, M5AuthAndRecovery, M5AuthAndRecoveryInput,
    ManagedCapabilityClass, NarrowingReason, PasskeyPosture, ProfileChannel, SurfaceClass,
    SurfaceTruthRow, M5_AUTH_AND_RECOVERY_RECORD_KIND, M5_AUTH_AND_RECOVERY_SCHEMA_VERSION,
    M5_AUTH_AND_RECOVERY_SHARED_CONTRACT_REF,
};
