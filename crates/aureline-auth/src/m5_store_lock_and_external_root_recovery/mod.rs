//! Credential-/trust-store lock-state and removable-volume / network-share /
//! external-root recovery truth, with local-state continuity.
//!
//! Aureline's local-first, native-desktop promise has to survive the everyday
//! moment an *OS-backed store locks* or an *external root disappears*: the OS
//! credential store is locked or unreachable, the trust / certificate store
//! drifts out from under a remembered decision, a removable volume is ejected, a
//! network share disconnects, or an external root simply goes missing. Each of
//! those is a real, recoverable failure mode — not a support-only edge case —
//! and each one can quietly turn into a generic downstream error or, worse, a
//! silent disappearance of the user's context. This module makes every such
//! moment an explicit, reviewable [`RecoveryState`] that:
//!
//! - names the unavailable resource with a [`ResourceClass`] and the typed
//!   degradation with a [`DegradedStateClass`], and keeps the four
//!   support-distinguishable families — store lock, trust-store drift, missing
//!   root, and returned root — as distinct [`IncidentClass`] values that never
//!   collapse into one generic error;
//! - says **what is paused** ([`PausedCapability`]) and **what remains
//!   local-only** ([`LocalOnlyCapability`]) so a locked store or a missing root
//!   never implies that local editing, local history, or local exports stopped
//!   working, and never implies a plaintext-secret fallback as a recovery path;
//! - retains a truthful **placeholder** naming the last-seen identity of the
//!   store or root and the **unsaved-local-state posture**
//!   ([`UnsavedLocalStatePosture`]), so context is preserved through the
//!   incident instead of vanishing;
//! - offers precise, typed **recovery actions** ([`RecoveryActionClass`]) —
//!   unlock / repair the store, review a trust change, reconnect a share, remount
//!   a volume, or Locate / Open cached context / Close a missing root — plus a
//!   repair-guidance ref, none of which ever imply writing a secret to plaintext;
//!   and
//! - binds every running session, queued job, and remembered decision affected
//!   by the incident into typed [`ProtectedContinuation`] rows whose
//!   [`ResumeDispositionClass`] is never silent, so nothing is auto-rejoined or
//!   re-run just because a store unlocked or a root reappeared.
//!
//! Every state also names an active-profile owner, a trust / profile / policy
//! checkpoint, and the canonical in-product command, so OS-store recovery never
//! bypasses trust evaluation, and declares its desktop / CLI-headless / support
//! surface parity, so a store-lock or missing-root incident carries the same
//! vocabulary in every flow.
//!
//! The resulting [`StoreLockRecoveryReport`] is the canonical truth object for
//! the store-lock and external-root recovery lane. It is consumed by the live
//! recovery affordances, the headless inspector
//! (`aureline_auth_m5_store_lock_and_external_root_recovery`, the only
//! mint-from-truth path for the JSON fixtures under
//! `fixtures/platform/m5-store-lock-and-missing-root/`), the support-export
//! wrapper and per-incident case exports, the markdown artifact under
//! `artifacts/platform/m5-store-lock-and-external-root-recovery.md`, and the
//! companion doc under `docs/m5/store-lock-and-external-root-recovery.md`.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every required incident kind is present — credential-store locked,
//!    credential-store unavailable, trust-store drift, removable-volume missing,
//!    network-share missing, external-root missing, and returned root — and each
//!    state carries a last-seen identity, a truthful placeholder, a repair-guidance
//!    ref, an active-profile owner, a trust checkpoint, the canonical in-product
//!    command, a continuity note, a non-empty degraded-state vocabulary, a
//!    non-empty local-only disclosure, full desktop / CLI / support surface
//!    parity, at least one platform, a downgrade rule, and
//!    `registered_on_recovery_harness = true`.
//! 2. No state may imply a plaintext-secret fallback
//!    ([`PlaintextFallbackImplied`]); this is a hard guardrail.
//! 3. Local user-owned work stays intact and visibly recoverable
//!    (`local_continuity_preserved = true`); otherwise it is a
//!    [`LocalWorkNotPreserved`] blocker.
//! 4. An active degradation (locked, unavailable, drifted, or missing root) MUST
//!    disclose what is paused ([`MissingPausedDisclosure`]) and offer a recovery
//!    action; a store lock with no recovery is a
//!    [`CredentialStoreLockUnrecoverable`] blocker, a trust-store drift with no
//!    recovery is a distinct [`TrustStoreDriftUnrecoverable`] blocker, and a
//!    missing root with no recovery is a distinct [`MissingRootUnrecoverable`]
//!    blocker — the three never collapse.
//! 5. No running session, queued job, or remembered decision is silently widened
//!    or re-run after unlock or root return: a silent resume posture, a silent
//!    continuation disposition, or a returned root that does not require explicit
//!    resume is a [`SilentResumeOnRecovery`] blocker.
//! 6. A missing placeholder is a [`SilentDisappearance`] blocker so an incident
//!    can never degrade to nothing, and stale evidence on a marketed state is a
//!    blocker so release tooling can narrow the surface instead of shipping it as
//!    implicitly stable.
//!
//! All identifiers, refs, and label strings are deterministic so the checked-in
//! fixtures under `fixtures/platform/m5-store-lock-and-missing-root/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_store_lock_recovery_report`].
//!
//! [`PlaintextFallbackImplied`]: RecoveryBlockingFinding::PlaintextFallbackImplied
//! [`LocalWorkNotPreserved`]: RecoveryBlockingFinding::LocalWorkNotPreserved
//! [`MissingPausedDisclosure`]: RecoveryBlockingFinding::MissingPausedDisclosure
//! [`CredentialStoreLockUnrecoverable`]: RecoveryBlockingFinding::CredentialStoreLockUnrecoverable
//! [`TrustStoreDriftUnrecoverable`]: RecoveryBlockingFinding::TrustStoreDriftUnrecoverable
//! [`MissingRootUnrecoverable`]: RecoveryBlockingFinding::MissingRootUnrecoverable
//! [`SilentResumeOnRecovery`]: RecoveryBlockingFinding::SilentResumeOnRecovery
//! [`SilentDisappearance`]: RecoveryBlockingFinding::SilentDisappearance

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every store-lock / missing-root record.
pub const STORE_LOCK_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every recovery surface.
pub const STORE_LOCK_RECOVERY_SHARED_CONTRACT_REF: &str =
    "auth:m5_store_lock_and_external_root_recovery:v1";

/// Stable record kind for [`StoreLockRecoveryReport`] payloads.
pub const STORE_LOCK_RECOVERY_REPORT_RECORD_KIND: &str =
    "auth_m5_store_lock_and_external_root_recovery_report_record";

/// Stable record kind for [`RecoveryStateRow`] payloads.
pub const STORE_LOCK_RECOVERY_ROW_RECORD_KIND: &str =
    "auth_m5_store_lock_and_external_root_recovery_state_record";

/// Stable record kind for [`StoreLockRecoverySupportExport`] payloads.
pub const STORE_LOCK_RECOVERY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_m5_store_lock_and_external_root_recovery_support_export_record";

/// Stable record kind for [`StoreLockRecoveryCaseExport`] payloads.
pub const STORE_LOCK_RECOVERY_CASE_EXPORT_RECORD_KIND: &str =
    "auth_m5_store_lock_and_external_root_recovery_case_export_record";

/// Stable report id quoted across surfaces.
pub const STORE_LOCK_RECOVERY_REPORT_ID: &str =
    "auth:m5_store_lock_and_external_root_recovery:report:v1";

/// Stable support-export id quoted in the published wrapper.
pub const STORE_LOCK_RECOVERY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-store-lock-and-external-root-recovery:001";

/// Source schema ref for the canonical store-lock / missing-root contract.
pub const STORE_LOCK_RECOVERY_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-store-lock-and-missing-root.schema.json";

/// Path of the published markdown artifact.
pub const STORE_LOCK_RECOVERY_PUBLISHED_REPORT_REF: &str =
    "artifacts/platform/m5-store-lock-and-external-root-recovery.md";

/// Path of the published companion doc.
pub const STORE_LOCK_RECOVERY_PUBLISHED_DOC_REF: &str =
    "docs/m5/store-lock-and-external-root-recovery.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-16T00:00:00Z";

/// The OS-backed store or external root that became unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceClass {
    /// The OS credential / secret store (keychain, secret service, credential
    /// manager).
    CredentialStore,
    /// The trust / certificate / trust-anchor store.
    TrustStore,
    /// A removable volume (USB drive, SD card, mounted image).
    RemovableVolume,
    /// A network share or remote-adjacent mount.
    NetworkShare,
    /// A general external root that is neither removable nor a network share.
    ExternalRoot,
}

impl ResourceClass {
    /// Returns the stable schema token for this resource class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CredentialStore => "credential_store",
            Self::TrustStore => "trust_store",
            Self::RemovableVolume => "removable_volume",
            Self::NetworkShare => "network_share",
            Self::ExternalRoot => "external_root",
        }
    }

    /// Returns a human-facing label for the resource class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::CredentialStore => "Credential store",
            Self::TrustStore => "Trust store",
            Self::RemovableVolume => "Removable volume",
            Self::NetworkShare => "Network share",
            Self::ExternalRoot => "External root",
        }
    }

    /// Returns the five resource classes in canonical order.
    pub const fn all() -> [Self; 5] {
        [
            Self::CredentialStore,
            Self::TrustStore,
            Self::RemovableVolume,
            Self::NetworkShare,
            Self::ExternalRoot,
        ]
    }
}

/// One support-distinguishable incident family the recovery layer governs.
///
/// These are the kinds the spec requires the matrix to cover, kept distinct so a
/// support packet can tell a store lock from a trust-store drift, a missing root,
/// or a returned root without manual log forensics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentClass {
    /// The OS credential store is locked and needs unlocking.
    CredentialStoreLocked,
    /// The OS credential store backend is unreachable or absent.
    CredentialStoreUnavailable,
    /// The trust / certificate store drifted from a remembered decision.
    TrustStoreDrift,
    /// A removable volume was ejected or removed.
    RemovableVolumeMissing,
    /// A network share disconnected.
    NetworkShareMissing,
    /// An external root went missing.
    ExternalRootMissing,
    /// A previously missing root has returned and awaits explicit resume.
    RootReturned,
}

impl IncidentClass {
    /// Returns the stable schema token for this incident class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CredentialStoreLocked => "credential_store_locked",
            Self::CredentialStoreUnavailable => "credential_store_unavailable",
            Self::TrustStoreDrift => "trust_store_drift",
            Self::RemovableVolumeMissing => "removable_volume_missing",
            Self::NetworkShareMissing => "network_share_missing",
            Self::ExternalRootMissing => "external_root_missing",
            Self::RootReturned => "root_returned",
        }
    }

    /// Returns a human-facing label for the incident class.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::CredentialStoreLocked => "Credential store locked",
            Self::CredentialStoreUnavailable => "Credential store unavailable",
            Self::TrustStoreDrift => "Trust-store drift",
            Self::RemovableVolumeMissing => "Removable volume missing",
            Self::NetworkShareMissing => "Network share missing",
            Self::ExternalRootMissing => "External root missing",
            Self::RootReturned => "Root returned",
        }
    }

    /// Returns the seven required incident kinds in canonical order.
    pub const fn required_kinds() -> [Self; 7] {
        [
            Self::CredentialStoreLocked,
            Self::CredentialStoreUnavailable,
            Self::TrustStoreDrift,
            Self::RemovableVolumeMissing,
            Self::NetworkShareMissing,
            Self::ExternalRootMissing,
            Self::RootReturned,
        ]
    }

    /// `true` when this incident is a credential-store lock / unavailability.
    pub const fn is_store_lock(self) -> bool {
        matches!(
            self,
            Self::CredentialStoreLocked | Self::CredentialStoreUnavailable
        )
    }

    /// `true` when this incident is a missing external root.
    pub const fn is_missing_root(self) -> bool {
        matches!(
            self,
            Self::RemovableVolumeMissing | Self::NetworkShareMissing | Self::ExternalRootMissing
        )
    }
}

/// The typed degradation state a resource is in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedStateClass {
    /// The store is locked: present but needs an unlock to use.
    StoreLocked,
    /// The store backend is unavailable: not reachable or not present.
    StoreUnavailable,
    /// The trust store drifted: anchors changed under a remembered decision.
    TrustStoreDrifted,
    /// An external root is missing: ejected, disconnected, or gone.
    RootMissing,
    /// A previously missing root has returned and is awaiting explicit resume.
    RootReturned,
}

impl DegradedStateClass {
    /// Returns the stable schema token for this degraded state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StoreLocked => "store_locked",
            Self::StoreUnavailable => "store_unavailable",
            Self::TrustStoreDrifted => "trust_store_drifted",
            Self::RootMissing => "root_missing",
            Self::RootReturned => "root_returned",
        }
    }

    /// Returns the five degraded states in canonical order.
    pub const fn all() -> [Self; 5] {
        [
            Self::StoreLocked,
            Self::StoreUnavailable,
            Self::TrustStoreDrifted,
            Self::RootMissing,
            Self::RootReturned,
        ]
    }

    /// `true` while the resource is actively degraded (paused), as opposed to a
    /// recovered, returned root awaiting explicit resume.
    pub const fn is_active_degradation(self) -> bool {
        matches!(
            self,
            Self::StoreLocked
                | Self::StoreUnavailable
                | Self::TrustStoreDrifted
                | Self::RootMissing
        )
    }
}

/// A capability paused while the resource is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PausedCapabilityClass {
    /// Provider / account authentication that needs a stored secret.
    ProviderAuthentication,
    /// A signing or publishing operation that needs a stored key.
    SignedOperation,
    /// Certificate / trust validation for outbound connections.
    CertificateValidation,
    /// Reading or writing files on the missing external root.
    ExternalRootAccess,
    /// Managed / remote sync that depends on the resource.
    ManagedSync,
}

impl PausedCapabilityClass {
    /// Returns the stable schema token for this paused capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthentication => "provider_authentication",
            Self::SignedOperation => "signed_operation",
            Self::CertificateValidation => "certificate_validation",
            Self::ExternalRootAccess => "external_root_access",
            Self::ManagedSync => "managed_sync",
        }
    }
}

/// One paused capability, with the export-safe ref the user surface renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PausedCapability {
    /// What is paused.
    pub capability_class: PausedCapabilityClass,
    /// Export-safe ref the user surface renders. MUST be non-empty.
    pub capability_ref: String,
}

/// A capability that remains available locally despite the incident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalOnlyCapabilityClass {
    /// Local editing of open documents continues.
    LocalEditing,
    /// Local history / timeline continues.
    LocalHistory,
    /// Local export of user-owned work continues.
    LocalExport,
    /// Offline core tooling continues.
    OfflineCoreTools,
    /// Browsing the cached context of the missing root continues.
    CachedContextBrowse,
}

impl LocalOnlyCapabilityClass {
    /// Returns the stable schema token for this local-only capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::LocalHistory => "local_history",
            Self::LocalExport => "local_export",
            Self::OfflineCoreTools => "offline_core_tools",
            Self::CachedContextBrowse => "cached_context_browse",
        }
    }
}

/// One local-only capability, with the export-safe ref the user surface renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalOnlyCapability {
    /// What remains local-only.
    pub capability_class: LocalOnlyCapabilityClass,
    /// Export-safe ref the user surface renders. MUST be non-empty.
    pub capability_ref: String,
}

/// Posture of unsaved local state at incident time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnsavedLocalStatePosture {
    /// Unsaved local state is preserved in place and unaffected by the incident.
    PreservedInPlace,
    /// There is no unsaved local state pending.
    NonePending,
    /// Unsaved local state is preserved and held pending recovery of the
    /// resource.
    PreservedPendingRecovery,
}

impl UnsavedLocalStatePosture {
    /// Returns the stable schema token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedInPlace => "preserved_in_place",
            Self::NonePending => "none_pending",
            Self::PreservedPendingRecovery => "preserved_pending_recovery",
        }
    }
}

/// Whether and how a continuation may resume once the resource recovers.
///
/// There is deliberately no "auto-resume" posture: recovery never silently
/// rejoins a session or re-runs a job.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumePostureClass {
    /// Nothing resumes until the user explicitly asks for it.
    ExplicitResumeRequired,
    /// There is nothing to resume.
    NotApplicable,
}

impl ResumePostureClass {
    /// Returns the stable schema token for this resume posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitResumeRequired => "explicit_resume_required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Class of a continuation affected by the incident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationClass {
    /// A running session (e.g. a provider or collaboration session).
    RunningSession,
    /// A queued job (e.g. a deferred write or sync).
    QueuedJob,
    /// A remembered decision (e.g. a remembered trust acceptance).
    RememberedDecision,
}

impl ContinuationClass {
    /// Returns the stable schema token for this continuation class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunningSession => "running_session",
            Self::QueuedJob => "queued_job",
            Self::RememberedDecision => "remembered_decision",
        }
    }
}

/// Disposition of a protected continuation on recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResumeDispositionClass {
    /// The continuation is resumed only on an explicit user action.
    ExplicitResumeRequired,
    /// The continuation is held for review before any resume.
    HeldForReview,
    /// The continuation would resume silently — never allowed.
    SilentResume,
}

impl ResumeDispositionClass {
    /// Returns the stable schema token for this disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitResumeRequired => "explicit_resume_required",
            Self::HeldForReview => "held_for_review",
            Self::SilentResume => "silent_resume",
        }
    }

    /// `true` when the disposition would silently widen or re-run work.
    pub const fn is_silent(self) -> bool {
        matches!(self, Self::SilentResume)
    }
}

/// A running session, queued job, or remembered decision the incident protects
/// from silent resume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedContinuation {
    /// Export-safe ref for the continuation. MUST be non-empty.
    pub continuation_ref: String,
    /// What kind of continuation this is.
    pub continuation_class: ContinuationClass,
    /// How it resumes once the resource recovers.
    pub resume_disposition: ResumeDispositionClass,
}

/// A typed recovery action offered for an incident.
///
/// None of these imply writing a secret to plaintext or otherwise downgrading
/// the secure store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// Unlock the OS credential store.
    UnlockStore,
    /// Repair the store handle without exposing plaintext.
    RepairStore,
    /// Retry the paused action after the store is unlocked.
    RetryAfterUnlock,
    /// Review the trust-store change before continuing.
    ReviewTrustChange,
    /// Re-evaluate trust against the current store.
    ReEvaluateTrust,
    /// Reconnect the network share.
    ReconnectNetworkShare,
    /// Remount the removable volume.
    RemountVolume,
    /// Locate the missing root.
    LocateRoot,
    /// Open the cached context of the missing root.
    OpenCachedContext,
    /// Close the placeholder for the missing root.
    ClosePlaceholder,
    /// Explicitly confirm resuming held work after recovery.
    ConfirmExplicitResume,
}

impl RecoveryActionClass {
    /// Returns the stable schema token for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnlockStore => "unlock_store",
            Self::RepairStore => "repair_store",
            Self::RetryAfterUnlock => "retry_after_unlock",
            Self::ReviewTrustChange => "review_trust_change",
            Self::ReEvaluateTrust => "re_evaluate_trust",
            Self::ReconnectNetworkShare => "reconnect_network_share",
            Self::RemountVolume => "remount_volume",
            Self::LocateRoot => "locate_root",
            Self::OpenCachedContext => "open_cached_context",
            Self::ClosePlaceholder => "close_placeholder",
            Self::ConfirmExplicitResume => "confirm_explicit_resume",
        }
    }
}

/// A flow surface a recovery state is mirrored across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// The desktop product surface.
    Desktop,
    /// The CLI / headless surface.
    CliHeadless,
    /// The support / export surface.
    Support,
}

impl SurfaceClass {
    /// Returns the stable schema token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::Support => "support",
        }
    }

    /// Returns the three surfaces a state MUST carry parity across.
    pub const fn required() -> [Self; 3] {
        [Self::Desktop, Self::CliHeadless, Self::Support]
    }
}

/// A desktop platform the state is claimed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    /// macOS desktop platform.
    Macos,
    /// Windows desktop platform.
    Windows,
    /// Linux desktop platform.
    Linux,
}

impl Platform {
    /// Returns the stable schema token for this platform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three platforms in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// Freshness of the captured recovery-state evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed state.
    Stale,
}

impl EvidenceFreshness {
    /// Returns the stable schema token for this freshness.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// A distinct store-lock / external-root recovery failure class.
///
/// Each class names a materially different way an OS-store lock or a missing
/// root can degrade dishonestly. They are never collapsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryFailureMode {
    /// A recovery path implied a plaintext-secret fallback.
    PlaintextFallbackImplied,
    /// Work would silently widen or re-run after recovery.
    SilentResumeOnRecovery,
    /// Local user-owned work was not preserved through the incident.
    LocalWorkNotPreserved,
    /// The incident degraded to nothing, with no placeholder.
    SilentDisappearance,
    /// A credential-store lock offered no recovery.
    CredentialStoreLockUnrecoverable,
    /// A trust-store drift offered no recovery.
    TrustStoreDriftUnrecoverable,
    /// A missing root offered no recovery.
    MissingRootUnrecoverable,
    /// The state bypassed trust / profile / policy evaluation.
    TrustEvaluationBypassed,
}

impl RecoveryFailureMode {
    /// Returns the stable schema token for this failure mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlaintextFallbackImplied => "plaintext_fallback_implied",
            Self::SilentResumeOnRecovery => "silent_resume_on_recovery",
            Self::LocalWorkNotPreserved => "local_work_not_preserved",
            Self::SilentDisappearance => "silent_disappearance",
            Self::CredentialStoreLockUnrecoverable => "credential_store_lock_unrecoverable",
            Self::TrustStoreDriftUnrecoverable => "trust_store_drift_unrecoverable",
            Self::MissingRootUnrecoverable => "missing_root_unrecoverable",
            Self::TrustEvaluationBypassed => "trust_evaluation_bypassed",
        }
    }
}

/// Cross-links to the canonical upstream packets the recovery layer reuses so
/// store, trust, path, and continuity vocabulary cannot drift independently.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryCrossLinks {
    /// Credential-state / secret-broker contract.
    pub credential_store_ref: String,
    /// Trust-store / native-trust integration matrix.
    pub trust_store_ref: String,
    /// Filesystem-identity / canonical-path lineage packet.
    pub filesystem_identity_ref: String,
    /// Deferred-intent / durable-progress packet.
    pub deferred_intent_ref: String,
    /// Auth-recovery / callback packet.
    pub auth_recovery_ref: String,
    /// Help/About and docs surface the report is ingested by.
    pub help_about_ref: String,
}

impl RecoveryCrossLinks {
    /// Returns the cross-link fields as `(label, ref)` pairs in canonical order.
    pub fn as_pairs(&self) -> [(&'static str, &str); 6] {
        [
            ("credential_store_ref", &self.credential_store_ref),
            ("trust_store_ref", &self.trust_store_ref),
            ("filesystem_identity_ref", &self.filesystem_identity_ref),
            ("deferred_intent_ref", &self.deferred_intent_ref),
            ("auth_recovery_ref", &self.auth_recovery_ref),
            ("help_about_ref", &self.help_about_ref),
        ]
    }

    /// The canonical cross-link set every report carries.
    pub fn canonical() -> Self {
        Self {
            credential_store_ref: "schemas/auth/credential_state.schema.json".to_owned(),
            trust_store_ref: "artifacts/platform/native_trust_integration_matrix.yaml".to_owned(),
            filesystem_identity_ref: "schemas/workspace/canonical_identity_lineage.schema.json"
                .to_owned(),
            deferred_intent_ref: "docs/m5/durable-progress-and-reopen.md".to_owned(),
            auth_recovery_ref: "artifacts/platform/m5-auth-callback-and-deep-link.md".to_owned(),
            help_about_ref: "docs/help/store_lock_and_external_root_recovery.md".to_owned(),
        }
    }
}

/// Canonical descriptor for one store-lock / missing-root recovery state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryState {
    /// Stable state id (e.g. `state:credential_store.locked`).
    pub state_id: String,
    /// The support-distinguishable incident family.
    pub incident_class: IncidentClass,
    /// The OS-backed resource that became unavailable.
    pub resource_class: ResourceClass,
    /// The typed degradation state.
    pub degraded_state_class: DegradedStateClass,
    /// Descriptor revision the report was produced against. MUST be non-empty.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Export-safe ref naming the last-seen identity of the store or root. MUST
    /// be non-empty. Never a raw path or secret body.
    pub last_seen_identity_ref: String,
    /// Truthful placeholder retained while the resource is unavailable. MUST be
    /// non-empty.
    pub placeholder_ref: String,
    /// What is paused while the resource is unavailable.
    pub paused_capabilities: Vec<PausedCapability>,
    /// What remains local-only despite the incident. MUST be non-empty.
    pub local_only_capabilities: Vec<LocalOnlyCapability>,
    /// Posture of unsaved local state at incident time.
    pub unsaved_local_state_posture: UnsavedLocalStatePosture,
    /// `true` once local user-owned work is intact and visibly recoverable. MUST
    /// be `true`.
    pub local_continuity_preserved: bool,
    /// Typed recovery actions offered for the incident.
    pub recovery_actions: Vec<RecoveryActionClass>,
    /// Precise repair-guidance ref. MUST be non-empty.
    pub repair_guidance_ref: String,
    /// `true` only if a recovery path would imply a plaintext-secret fallback.
    /// MUST be `false`.
    pub implies_plaintext_fallback: bool,
    /// How held work resumes once the resource recovers.
    pub resume_posture: ResumePostureClass,
    /// `true` only if work would resume silently on recovery. MUST be `false`.
    pub resumes_silently_on_recovery: bool,
    /// Running sessions, queued jobs, and remembered decisions protected from
    /// silent resume.
    pub protected_continuations: Vec<ProtectedContinuation>,
    /// Active profile owner the state routes through. MUST be non-empty.
    pub active_profile_owner_ref: String,
    /// Trust / profile / policy checkpoint the state routes through. MUST be
    /// non-empty.
    pub trust_checkpoint_ref: String,
    /// Canonical in-product command the recovery reuses. MUST be non-empty.
    pub canonical_command_ref: String,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Exact degraded-state vocabulary user-visible surfaces MUST use. MUST be
    /// non-empty.
    pub degraded_state_vocabulary: Vec<String>,
    /// Surfaces the state is mirrored across. MUST include desktop, CLI/headless,
    /// and support.
    pub surface_parity: Vec<SurfaceClass>,
    /// Claimed platforms. MUST be non-empty.
    pub claimed_platforms: Vec<Platform>,
    /// Freshness of the captured evidence.
    pub evidence_freshness: EvidenceFreshness,
    /// Timestamp the evidence was captured.
    pub evidence_captured_at: String,
    /// Rule user-visible surfaces follow when evidence goes stale. MUST be
    /// non-empty.
    pub downgrade_rule_ref: String,
    /// `true` when the state is marketed and must pass the report or narrow.
    pub marketed: bool,
    /// `true` once the state rides the governed recovery harness. MUST be `true`.
    pub registered_on_recovery_harness: bool,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum RecoveryBlockingFinding {
    /// A recovery path implied a plaintext-secret fallback.
    PlaintextFallbackImplied {
        /// State that exposes the gap.
        state_id: String,
    },
    /// Work would silently widen or re-run after recovery.
    SilentResumeOnRecovery {
        /// State that exposes the gap.
        state_id: String,
    },
    /// Local user-owned work was not preserved through the incident.
    LocalWorkNotPreserved {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The incident degraded to nothing, with no placeholder.
    SilentDisappearance {
        /// State that exposes the gap.
        state_id: String,
    },
    /// A credential-store lock / unavailability offered no recovery.
    CredentialStoreLockUnrecoverable {
        /// State that exposes the gap.
        state_id: String,
    },
    /// A trust-store drift offered no recovery.
    TrustStoreDriftUnrecoverable {
        /// State that exposes the gap.
        state_id: String,
    },
    /// A missing root offered no recovery.
    MissingRootUnrecoverable {
        /// State that exposes the gap.
        state_id: String,
    },
    /// An active degradation did not disclose what is paused.
    MissingPausedDisclosure {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state did not disclose what remains local-only.
    MissingLocalOnlyDisclosure {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no last-seen identity for the placeholder.
    MissingLastSeenIdentity {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no repair-guidance ref.
    MissingRepairGuidance {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state bypassed trust / profile / policy evaluation.
    TrustEvaluationBypassed {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no active-profile owner.
    MissingActiveProfileOwner {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state reused no canonical in-product command.
    MissingCanonicalCommand {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no continuity note.
    MissingContinuityNote {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no degraded-state vocabulary.
    MissingDegradedStateVocabulary {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state did not carry full desktop / CLI / support surface parity.
    SurfaceParityIncomplete {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state claimed no platform.
    MissingClaimedPlatforms {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state recorded no downgrade rule.
    MissingDowngradeRule {
        /// State that exposes the gap.
        state_id: String,
    },
    /// A marketed state carries stale evidence.
    StaleEvidenceOnMarketedState {
        /// State that exposes the gap.
        state_id: String,
    },
    /// The state drives its own recovery off the governed harness.
    StateNotOnHarness {
        /// State that exposes the gap.
        state_id: String,
    },
}

impl RecoveryBlockingFinding {
    /// Returns the stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::PlaintextFallbackImplied { .. } => "plaintext_fallback_implied",
            Self::SilentResumeOnRecovery { .. } => "silent_resume_on_recovery",
            Self::LocalWorkNotPreserved { .. } => "local_work_not_preserved",
            Self::SilentDisappearance { .. } => "silent_disappearance",
            Self::CredentialStoreLockUnrecoverable { .. } => "credential_store_lock_unrecoverable",
            Self::TrustStoreDriftUnrecoverable { .. } => "trust_store_drift_unrecoverable",
            Self::MissingRootUnrecoverable { .. } => "missing_root_unrecoverable",
            Self::MissingPausedDisclosure { .. } => "missing_paused_disclosure",
            Self::MissingLocalOnlyDisclosure { .. } => "missing_local_only_disclosure",
            Self::MissingLastSeenIdentity { .. } => "missing_last_seen_identity",
            Self::MissingRepairGuidance { .. } => "missing_repair_guidance",
            Self::TrustEvaluationBypassed { .. } => "trust_evaluation_bypassed",
            Self::MissingActiveProfileOwner { .. } => "missing_active_profile_owner",
            Self::MissingCanonicalCommand { .. } => "missing_canonical_command",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingDegradedStateVocabulary { .. } => "missing_degraded_state_vocabulary",
            Self::SurfaceParityIncomplete { .. } => "surface_parity_incomplete",
            Self::MissingClaimedPlatforms { .. } => "missing_claimed_platforms",
            Self::MissingDowngradeRule { .. } => "missing_downgrade_rule",
            Self::StaleEvidenceOnMarketedState { .. } => "stale_evidence_on_marketed_state",
            Self::StateNotOnHarness { .. } => "state_not_on_harness",
        }
    }

    /// Returns the state id this finding is attached to.
    pub fn state_id(&self) -> &str {
        match self {
            Self::PlaintextFallbackImplied { state_id }
            | Self::SilentResumeOnRecovery { state_id }
            | Self::LocalWorkNotPreserved { state_id }
            | Self::SilentDisappearance { state_id }
            | Self::CredentialStoreLockUnrecoverable { state_id }
            | Self::TrustStoreDriftUnrecoverable { state_id }
            | Self::MissingRootUnrecoverable { state_id }
            | Self::MissingPausedDisclosure { state_id }
            | Self::MissingLocalOnlyDisclosure { state_id }
            | Self::MissingLastSeenIdentity { state_id }
            | Self::MissingRepairGuidance { state_id }
            | Self::TrustEvaluationBypassed { state_id }
            | Self::MissingActiveProfileOwner { state_id }
            | Self::MissingCanonicalCommand { state_id }
            | Self::MissingContinuityNote { state_id }
            | Self::MissingDegradedStateVocabulary { state_id }
            | Self::SurfaceParityIncomplete { state_id }
            | Self::MissingClaimedPlatforms { state_id }
            | Self::MissingDowngradeRule { state_id }
            | Self::StaleEvidenceOnMarketedState { state_id }
            | Self::StateNotOnHarness { state_id } => state_id,
        }
    }

    /// Returns the distinct failure mode this finding maps to, when it maps to a
    /// contract-honesty failure class (rather than a missing-field gap).
    pub fn failure_mode(&self) -> Option<RecoveryFailureMode> {
        match self {
            Self::PlaintextFallbackImplied { .. } => {
                Some(RecoveryFailureMode::PlaintextFallbackImplied)
            }
            Self::SilentResumeOnRecovery { .. } => {
                Some(RecoveryFailureMode::SilentResumeOnRecovery)
            }
            Self::LocalWorkNotPreserved { .. } => Some(RecoveryFailureMode::LocalWorkNotPreserved),
            Self::SilentDisappearance { .. } => Some(RecoveryFailureMode::SilentDisappearance),
            Self::CredentialStoreLockUnrecoverable { .. } => {
                Some(RecoveryFailureMode::CredentialStoreLockUnrecoverable)
            }
            Self::TrustStoreDriftUnrecoverable { .. } => {
                Some(RecoveryFailureMode::TrustStoreDriftUnrecoverable)
            }
            Self::MissingRootUnrecoverable { .. } => {
                Some(RecoveryFailureMode::MissingRootUnrecoverable)
            }
            Self::TrustEvaluationBypassed { .. } => {
                Some(RecoveryFailureMode::TrustEvaluationBypassed)
            }
            _ => None,
        }
    }
}

/// One per-state recovery row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryStateRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the state.
    pub descriptor: RecoveryState,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<RecoveryBlockingFinding>,
    /// `true` when the state is marketed.
    pub marketed: bool,
}

/// One `(class, count)` blocking-finding tally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryFindingCount {
    /// Finding class token.
    pub class: String,
    /// Number of findings in the class.
    pub count: usize,
}

/// Per-class blocking-finding summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryFindingSummary {
    /// Total blocking findings across all states.
    pub total_blocking_findings: usize,
    /// Per-class tallies, sorted by class token.
    pub by_class: Vec<RecoveryFindingCount>,
}

/// Per-incident-class presence summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentClassCoverage {
    /// Incident class.
    pub incident_class: IncidentClass,
    /// Number of registered states of the class.
    pub state_count: usize,
}

/// Per-resource-class coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceClassCoverage {
    /// Resource class.
    pub resource_class: ResourceClass,
    /// Number of registered states for the resource.
    pub state_count: usize,
    /// Number of states that preserve local continuity.
    pub local_continuity_preserved_count: usize,
}

/// Canonical recovery index entry, one per state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryIndexEntry {
    /// State id.
    pub state_id: String,
    /// Incident class.
    pub incident_class: IncidentClass,
    /// Resource class.
    pub resource_class: ResourceClass,
    /// Degraded state class.
    pub degraded_state_class: DegradedStateClass,
    /// Resume posture.
    pub resume_posture: ResumePostureClass,
    /// Number of recovery actions offered.
    pub recovery_action_count: usize,
}

/// A marketed state release tooling should narrow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateNarrowableEntry {
    /// State id.
    pub state_id: String,
    /// The distinct failure mode, when the finding maps to one.
    pub failure_mode: Option<RecoveryFailureMode>,
    /// Reason the state should narrow.
    pub reason: String,
}

/// The canonical store-lock / external-root recovery report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreLockRecoveryReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required incident kinds, in canonical order.
    pub required_incident_kinds: Vec<IncidentClass>,
    /// Union of claimed platforms across all states, sorted.
    pub claimed_platforms: Vec<Platform>,
    /// Cross-links to upstream packets.
    pub cross_links: RecoveryCrossLinks,
    /// Per-state rows, sorted by `descriptor.state_id`.
    pub entries: Vec<RecoveryStateRow>,
    /// Per-incident-class presence summary, in canonical order.
    pub incident_class_coverage: Vec<IncidentClassCoverage>,
    /// Per-resource-class coverage summary, in canonical order.
    pub resource_class_coverage: Vec<ResourceClassCoverage>,
    /// Per-class blocking-finding summary.
    pub findings_summary: RecoveryFindingSummary,
    /// Canonical recovery index, sorted by state id.
    pub recovery_index: Vec<RecoveryIndexEntry>,
    /// Number of registered states present.
    pub registered_state_count: usize,
    /// Number of states marketed.
    pub marketed_state_count: usize,
    /// Number of states in active degradation (not a returned root).
    pub active_degradation_count: usize,
    /// Marketed states release tooling should narrow.
    pub narrowable_marketed_entries: Vec<StateNarrowableEntry>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this report is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the report can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the report can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the report was generated.
    pub generated_at: String,
}

impl StoreLockRecoveryReport {
    /// Returns `true` when every required incident kind has at least one
    /// registered state.
    pub fn every_kind_present(&self) -> bool {
        IncidentClass::required_kinds().into_iter().all(|kind| {
            self.entries
                .iter()
                .any(|entry| entry.descriptor.incident_class == kind)
        })
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "report: states={}, marketed={}, active={}, blocking={}, clean={}",
            self.registered_state_count,
            self.marketed_state_count,
            self.active_degradation_count,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for entry in &self.entries {
            lines.push(format!(
                "{}: incident={}, resource={}, state={}, resume={}, recovery_actions={}",
                entry.descriptor.state_id,
                entry.descriptor.incident_class.as_str(),
                entry.descriptor.resource_class.as_str(),
                entry.descriptor.degraded_state_class.as_str(),
                entry.descriptor.resume_posture.as_str(),
                entry.descriptor.recovery_actions.len(),
            ));
        }
        for entry in &self.entries {
            for finding in &entry.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {}",
                    finding.class_token(),
                    finding.state_id(),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_entries {
            lines.push(format!(
                "narrowable: {} -- {}",
                narrowable.state_id, narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 store-lock and external-root recovery\n\n");
        out.push_str(
            "Generated from the seeded report in\n\
             [`crate::m5_store_lock_and_external_root_recovery`](../../crates/aureline-auth/src/m5_store_lock_and_external_root_recovery/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- report-md > \\\n  artifacts/platform/m5-store-lock-and-external-root-recovery.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed platforms: {}\n",
            self.claimed_platforms
                .iter()
                .map(|platform| format!("`{}`", platform.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Registered states: `{}`\n",
            self.registered_state_count
        ));
        out.push_str(&format!(
            "- Marketed states: `{}`\n",
            self.marketed_state_count
        ));
        out.push_str(&format!(
            "- Active degradations: `{}`\n",
            self.active_degradation_count
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed states: `{}`\n",
            self.narrowable_marketed_entries.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Cross-links\n\n");
        out.push_str("| Upstream packet | Ref |\n| --------------- | --- |\n");
        for (label, value) in self.cross_links.as_pairs() {
            out.push_str(&format!("| `{label}` | `{value}` |\n"));
        }
        out.push('\n');

        out.push_str("## Per-incident-class coverage\n\n");
        out.push_str(
            "| Incident class | Registered states |\n| -------------- | ----------------: |\n",
        );
        for coverage in &self.incident_class_coverage {
            out.push_str(&format!(
                "| {} | {} |\n",
                coverage.incident_class.display_label(),
                coverage.state_count,
            ));
        }
        out.push('\n');

        out.push_str("## Per-resource coverage\n\n");
        out.push_str(
            "| Resource | States | Local continuity preserved |\n\
             | -------- | -----: | -------------------------: |\n",
        );
        for coverage in &self.resource_class_coverage {
            out.push_str(&format!(
                "| {} | {} | {} |\n",
                coverage.resource_class.display_label(),
                coverage.state_count,
                coverage.local_continuity_preserved_count,
            ));
        }
        out.push('\n');

        out.push_str("## Recovery index\n\n");
        out.push_str(
            "| State | Incident | Resource | Degraded state | Resume posture | Recovery actions |\n\
             | ----- | -------- | -------- | -------------- | -------------- | ---------------: |\n",
        );
        for entry in &self.recovery_index {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                entry.state_id,
                entry.incident_class.as_str(),
                entry.resource_class.as_str(),
                entry.degraded_state_class.as_str(),
                entry.resume_posture.as_str(),
                entry.recovery_action_count,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        for tally in &self.findings_summary.by_class {
            out.push_str(&format!("| `{}` | {} |\n", tally.class, tally.count));
        }
        if self.findings_summary.by_class.is_empty() {
            out.push_str("| _(none)_ | 0 |\n");
        }
        out.push('\n');

        out.push_str("## Per-state rows\n\n");
        for entry in &self.entries {
            let d = &entry.descriptor;
            out.push_str(&format!(
                "### `{}` ({})\n\n",
                d.state_id,
                d.incident_class.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                d.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Resource: `{}` (degraded state `{}`)\n",
                d.resource_class.as_str(),
                d.degraded_state_class.as_str(),
            ));
            out.push_str(&format!(
                "- Last-seen identity: `{}`\n",
                d.last_seen_identity_ref
            ));
            out.push_str(&format!("- Placeholder: `{}`\n", d.placeholder_ref));
            if d.paused_capabilities.is_empty() {
                out.push_str("- Paused: _(nothing paused)_\n");
            } else {
                out.push_str(&format!(
                    "- Paused: {}\n",
                    d.paused_capabilities
                        .iter()
                        .map(|cap| format!("`{}`", cap.capability_class.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!(
                "- Local-only: {}\n",
                d.local_only_capabilities
                    .iter()
                    .map(|cap| format!("`{}`", cap.capability_class.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Unsaved local state: `{}`\n",
                d.unsaved_local_state_posture.as_str()
            ));
            out.push_str(&format!(
                "- Local continuity preserved: `{}`\n",
                if d.local_continuity_preserved {
                    "yes"
                } else {
                    "no"
                }
            ));
            if d.recovery_actions.is_empty() {
                out.push_str("- Recovery actions: _(none)_\n");
            } else {
                out.push_str(&format!(
                    "- Recovery actions: {}\n",
                    d.recovery_actions
                        .iter()
                        .map(|action| format!("`{}`", action.as_str()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!("- Repair guidance: `{}`\n", d.repair_guidance_ref));
            out.push_str(&format!(
                "- Implies plaintext fallback: `{}`\n",
                if d.implies_plaintext_fallback {
                    "yes"
                } else {
                    "no"
                }
            ));
            out.push_str(&format!(
                "- Resume posture: `{}` (silent on recovery: `{}`)\n",
                d.resume_posture.as_str(),
                if d.resumes_silently_on_recovery {
                    "yes"
                } else {
                    "no"
                },
            ));
            if d.protected_continuations.is_empty() {
                out.push_str("- Protected continuations: _(none)_\n");
            } else {
                out.push_str("- Protected continuations:\n");
                for cont in &d.protected_continuations {
                    out.push_str(&format!(
                        "  - `{}` (`{}`) -> `{}`\n",
                        cont.continuation_ref,
                        cont.continuation_class.as_str(),
                        cont.resume_disposition.as_str(),
                    ));
                }
            }
            out.push_str(&format!(
                "- Active profile owner: `{}`\n",
                d.active_profile_owner_ref
            ));
            out.push_str(&format!(
                "- Trust checkpoint: `{}`\n",
                d.trust_checkpoint_ref
            ));
            out.push_str(&format!(
                "- Canonical command: `{}`\n",
                d.canonical_command_ref
            ));
            out.push_str(&format!(
                "- Surface parity: {}\n",
                d.surface_parity
                    .iter()
                    .map(|surface| format!("`{}`", surface.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Claimed platforms: {}\n",
                d.claimed_platforms
                    .iter()
                    .map(|platform| format!("`{}`", platform.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "- Evidence freshness: `{}` (captured `{}`)\n",
                d.evidence_freshness.as_str(),
                d.evidence_captured_at,
            ));
            out.push_str(&format!("- Downgrade rule: `{}`\n", d.downgrade_rule_ref));
            out.push_str(&format!(
                "- Marketed: `{}`\n",
                if entry.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!("- Continuity note: {}\n", d.continuity_note));
            out.push_str("- Degraded-state vocabulary:\n");
            for phrase in &d.degraded_state_vocabulary {
                out.push_str(&format!("  - {phrase}\n"));
            }
            out.push('\n');

            if entry.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &entry.blocking_findings {
                    out.push_str(&format!("- `{}`\n", finding.class_token()));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-auth --bin aureline_auth_m5_store_lock_and_external_root_recovery -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-auth --test m5_store_lock_and_external_root_recovery_fixtures\n",
        );
        out.push_str("python3 tools/ci/m5/store_lock_and_external_root_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the full store-lock / missing-root report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreLockRecoverySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Report quoted in full.
    pub report: StoreLockRecoveryReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl StoreLockRecoverySupportExport {
    /// Builds the support-export wrapper for a report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: StoreLockRecoveryReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for entry in &report.entries {
            case_ids.push(entry.descriptor.state_id.clone());
            case_ids.push(entry.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: STORE_LOCK_RECOVERY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: STORE_LOCK_RECOVERY_SCHEMA_VERSION,
            shared_contract_ref: STORE_LOCK_RECOVERY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Per-incident support-export packet for a single recovery state.
///
/// This is the export a reviewer reproduces a store-lock, trust-store-drift,
/// missing-root, or returned-root incident from — the typed diagnostic that
/// replaces a screenshot and lets support tell the four cases apart without
/// manual log forensics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreLockRecoveryCaseExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable case-export id.
    pub case_export_id: String,
    /// Stable case label (e.g. `missing_root`).
    pub case_label: String,
    /// Incident class that defines the case.
    pub incident_class: IncidentClass,
    /// Resource class of the incident.
    pub resource_class: ResourceClass,
    /// Degraded state class of the incident.
    pub degraded_state_class: DegradedStateClass,
    /// The state row in full.
    pub state: RecoveryStateRow,
    /// Recovery actions the incident offers.
    pub recovery_actions: Vec<RecoveryActionClass>,
    /// Stable reproduction note for support.
    pub reproduction_note: String,
}

impl StoreLockRecoveryCaseExport {
    /// Builds a per-incident case export from a recovery state row.
    pub fn from_row(
        case_export_id: impl Into<String>,
        case_label: impl Into<String>,
        reproduction_note: impl Into<String>,
        row: RecoveryStateRow,
    ) -> Self {
        let incident_class = row.descriptor.incident_class;
        let resource_class = row.descriptor.resource_class;
        let degraded_state_class = row.descriptor.degraded_state_class;
        let recovery_actions = row.descriptor.recovery_actions.clone();
        Self {
            record_kind: STORE_LOCK_RECOVERY_CASE_EXPORT_RECORD_KIND.to_owned(),
            schema_version: STORE_LOCK_RECOVERY_SCHEMA_VERSION,
            shared_contract_ref: STORE_LOCK_RECOVERY_SHARED_CONTRACT_REF.to_owned(),
            case_export_id: case_export_id.into(),
            case_label: case_label.into(),
            incident_class,
            resource_class,
            degraded_state_class,
            state: row,
            recovery_actions,
            reproduction_note: reproduction_note.into(),
        }
    }
}

/// Computes the per-state blocking findings from a descriptor.
fn compute_state_findings(descriptor: &RecoveryState) -> Vec<RecoveryBlockingFinding> {
    let mut findings = Vec::new();
    let state_id = descriptor.state_id.clone();

    // Identity, placeholder, and ownership integrity.
    if descriptor.last_seen_identity_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingLastSeenIdentity {
            state_id: state_id.clone(),
        });
    }
    if descriptor.placeholder_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::SilentDisappearance {
            state_id: state_id.clone(),
        });
    }
    if descriptor.repair_guidance_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingRepairGuidance {
            state_id: state_id.clone(),
        });
    }
    if descriptor.active_profile_owner_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingActiveProfileOwner {
            state_id: state_id.clone(),
        });
    }
    if descriptor.trust_checkpoint_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::TrustEvaluationBypassed {
            state_id: state_id.clone(),
        });
    }
    if descriptor.canonical_command_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingCanonicalCommand {
            state_id: state_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingContinuityNote {
            state_id: state_id.clone(),
        });
    }
    if descriptor
        .degraded_state_vocabulary
        .iter()
        .all(|phrase| phrase.trim().is_empty())
    {
        findings.push(RecoveryBlockingFinding::MissingDegradedStateVocabulary {
            state_id: state_id.clone(),
        });
    }
    if descriptor
        .local_only_capabilities
        .iter()
        .all(|cap| cap.capability_ref.trim().is_empty())
    {
        findings.push(RecoveryBlockingFinding::MissingLocalOnlyDisclosure {
            state_id: state_id.clone(),
        });
    }
    for required in SurfaceClass::required() {
        if !descriptor.surface_parity.contains(&required) {
            findings.push(RecoveryBlockingFinding::SurfaceParityIncomplete {
                state_id: state_id.clone(),
            });
            break;
        }
    }
    if descriptor.claimed_platforms.is_empty() {
        findings.push(RecoveryBlockingFinding::MissingClaimedPlatforms {
            state_id: state_id.clone(),
        });
    }
    if descriptor.downgrade_rule_ref.trim().is_empty() {
        findings.push(RecoveryBlockingFinding::MissingDowngradeRule {
            state_id: state_id.clone(),
        });
    }
    if !descriptor.registered_on_recovery_harness {
        findings.push(RecoveryBlockingFinding::StateNotOnHarness {
            state_id: state_id.clone(),
        });
    }
    if descriptor.marketed && descriptor.evidence_freshness == EvidenceFreshness::Stale {
        findings.push(RecoveryBlockingFinding::StaleEvidenceOnMarketedState {
            state_id: state_id.clone(),
        });
    }

    // Guardrail: a recovery path must never imply a plaintext-secret fallback.
    if descriptor.implies_plaintext_fallback {
        findings.push(RecoveryBlockingFinding::PlaintextFallbackImplied {
            state_id: state_id.clone(),
        });
    }

    // Local user-owned work must remain intact and visibly recoverable.
    if !descriptor.local_continuity_preserved {
        findings.push(RecoveryBlockingFinding::LocalWorkNotPreserved {
            state_id: state_id.clone(),
        });
    }

    // Active-degradation discipline: an active incident must disclose what is
    // paused and offer a recovery action, and each store/trust/root family stays
    // a distinct failure.
    if descriptor.degraded_state_class.is_active_degradation() {
        if descriptor.paused_capabilities.is_empty() {
            findings.push(RecoveryBlockingFinding::MissingPausedDisclosure {
                state_id: state_id.clone(),
            });
        }
        if descriptor.recovery_actions.is_empty() {
            let finding = if descriptor.incident_class.is_store_lock() {
                RecoveryBlockingFinding::CredentialStoreLockUnrecoverable {
                    state_id: state_id.clone(),
                }
            } else if descriptor.incident_class == IncidentClass::TrustStoreDrift {
                RecoveryBlockingFinding::TrustStoreDriftUnrecoverable {
                    state_id: state_id.clone(),
                }
            } else {
                RecoveryBlockingFinding::MissingRootUnrecoverable {
                    state_id: state_id.clone(),
                }
            };
            findings.push(finding);
        }
    }

    // No silent widening: nothing resumes silently after unlock or root return.
    let silent_continuation = descriptor
        .protected_continuations
        .iter()
        .any(|cont| cont.resume_disposition.is_silent());
    let returned_without_explicit_resume = descriptor.degraded_state_class
        == DegradedStateClass::RootReturned
        && descriptor.resume_posture != ResumePostureClass::ExplicitResumeRequired;
    let continuations_without_explicit_resume = !descriptor.protected_continuations.is_empty()
        && descriptor.resume_posture != ResumePostureClass::ExplicitResumeRequired;
    if descriptor.resumes_silently_on_recovery
        || silent_continuation
        || returned_without_explicit_resume
        || continuations_without_explicit_resume
    {
        findings.push(RecoveryBlockingFinding::SilentResumeOnRecovery {
            state_id: state_id.clone(),
        });
    }

    findings
}

/// Builds a [`RecoveryStateRow`] from a descriptor, computing the per-state
/// blocking findings.
pub fn build_store_lock_recovery_row(descriptor: RecoveryState) -> RecoveryStateRow {
    let marketed = descriptor.marketed;
    let blocking_findings = compute_state_findings(&descriptor);

    RecoveryStateRow {
        record_kind: STORE_LOCK_RECOVERY_ROW_RECORD_KIND.to_owned(),
        schema_version: STORE_LOCK_RECOVERY_SCHEMA_VERSION,
        shared_contract_ref: STORE_LOCK_RECOVERY_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        blocking_findings,
        marketed,
    }
}

/// Computes the per-incident, per-resource, and per-class summaries from
/// finished rows.
fn summarize_report(
    entries: &[RecoveryStateRow],
) -> (
    Vec<IncidentClassCoverage>,
    Vec<ResourceClassCoverage>,
    RecoveryFindingSummary,
) {
    let mut incident_coverage: Vec<IncidentClassCoverage> = IncidentClass::required_kinds()
        .into_iter()
        .map(|incident_class| IncidentClassCoverage {
            incident_class,
            state_count: 0,
        })
        .collect();

    let mut resource_coverage: Vec<ResourceClassCoverage> = ResourceClass::all()
        .into_iter()
        .map(|resource_class| ResourceClassCoverage {
            resource_class,
            state_count: 0,
            local_continuity_preserved_count: 0,
        })
        .collect();

    let mut class_counts: Vec<RecoveryFindingCount> = Vec::new();
    let mut total = 0usize;

    for entry in entries {
        let descriptor = &entry.descriptor;
        if let Some(incident_row) = incident_coverage
            .iter_mut()
            .find(|row| row.incident_class == descriptor.incident_class)
        {
            incident_row.state_count += 1;
        }
        if let Some(resource_row) = resource_coverage
            .iter_mut()
            .find(|row| row.resource_class == descriptor.resource_class)
        {
            resource_row.state_count += 1;
            if descriptor.local_continuity_preserved {
                resource_row.local_continuity_preserved_count += 1;
            }
        }
        for finding in &entry.blocking_findings {
            total += 1;
            let class = finding.class_token();
            if let Some(tally) = class_counts.iter_mut().find(|tally| tally.class == class) {
                tally.count += 1;
            } else {
                class_counts.push(RecoveryFindingCount {
                    class: class.to_owned(),
                    count: 1,
                });
            }
        }
    }

    class_counts.sort_by(|left, right| left.class.cmp(&right.class));
    (
        incident_coverage,
        resource_coverage,
        RecoveryFindingSummary {
            total_blocking_findings: total,
            by_class: class_counts,
        },
    )
}

/// Computes the marketed states release tooling should narrow because a control
/// failed or their evidence is stale.
fn compute_narrowable_entries(entries: &[RecoveryStateRow]) -> Vec<StateNarrowableEntry> {
    let mut narrowable = Vec::new();
    for entry in entries {
        if !entry.marketed {
            continue;
        }
        for finding in &entry.blocking_findings {
            narrowable.push(StateNarrowableEntry {
                state_id: entry.descriptor.state_id.clone(),
                failure_mode: finding.failure_mode(),
                reason: format!("blocking_finding:{}", finding.class_token()),
            });
        }
    }
    narrowable
}

/// Builds a full [`StoreLockRecoveryReport`] from per-state rows.
pub fn build_store_lock_recovery_report(entries: Vec<RecoveryStateRow>) -> StoreLockRecoveryReport {
    let mut entries = entries;
    entries.sort_by(|left, right| left.descriptor.state_id.cmp(&right.descriptor.state_id));

    let registered_state_count = entries.len();
    let marketed_state_count = entries.iter().filter(|entry| entry.marketed).count();
    let active_degradation_count = entries
        .iter()
        .filter(|entry| {
            entry
                .descriptor
                .degraded_state_class
                .is_active_degradation()
        })
        .count();

    let (incident_class_coverage, resource_class_coverage, findings_summary) =
        summarize_report(&entries);
    let narrowable_marketed_entries = compute_narrowable_entries(&entries);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut platform_set: Vec<Platform> = Vec::new();
    for entry in &entries {
        for platform in &entry.descriptor.claimed_platforms {
            if !platform_set.contains(platform) {
                platform_set.push(*platform);
            }
        }
    }
    platform_set.sort();

    let mut recovery_index: Vec<RecoveryIndexEntry> = entries
        .iter()
        .map(|entry| RecoveryIndexEntry {
            state_id: entry.descriptor.state_id.clone(),
            incident_class: entry.descriptor.incident_class,
            resource_class: entry.descriptor.resource_class,
            degraded_state_class: entry.descriptor.degraded_state_class,
            resume_posture: entry.descriptor.resume_posture,
            recovery_action_count: entry.descriptor.recovery_actions.len(),
        })
        .collect();
    recovery_index.sort_by(|left, right| left.state_id.cmp(&right.state_id));

    StoreLockRecoveryReport {
        record_kind: STORE_LOCK_RECOVERY_REPORT_RECORD_KIND.to_owned(),
        schema_version: STORE_LOCK_RECOVERY_SCHEMA_VERSION,
        shared_contract_ref: STORE_LOCK_RECOVERY_SHARED_CONTRACT_REF.to_owned(),
        report_id: STORE_LOCK_RECOVERY_REPORT_ID.to_owned(),
        source_schema_ref: STORE_LOCK_RECOVERY_SOURCE_SCHEMA_REF.to_owned(),
        required_incident_kinds: IncidentClass::required_kinds().to_vec(),
        claimed_platforms: platform_set,
        cross_links: RecoveryCrossLinks::canonical(),
        entries,
        incident_class_coverage,
        resource_class_coverage,
        findings_summary,
        recovery_index,
        registered_state_count,
        marketed_state_count,
        active_degradation_count,
        narrowable_marketed_entries,
        report_clean,
        published_report_ref: STORE_LOCK_RECOVERY_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: STORE_LOCK_RECOVERY_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            STORE_LOCK_RECOVERY_PUBLISHED_DOC_REF.to_owned(),
            "docs/help/store_lock_and_external_root_recovery.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-store-lock-and-external-root-recovery".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_store_lock_recovery_report`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum StoreLockRecoveryValidationError {
    /// The report has no registered states.
    NoRegisteredStates,
    /// A required incident kind has no registered state.
    RequiredIncidentKindMissing {
        /// Incident kind token that is missing.
        incident_kind: String,
    },
    /// A blocking finding remains on a state.
    BlockingFindingPresent {
        /// State id the finding is attached to.
        state_id: String,
        /// Finding class token.
        class: String,
    },
    /// A cross-link ref is empty.
    CrossLinkMissing {
        /// Cross-link field that is empty.
        field: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A state's descriptor revision ref is empty.
    MissingDescriptorRevisionRef {
        /// State id that exposes the gap.
        state_id: String,
    },
}

/// Validates a report against the store-lock / missing-root acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_store_lock_recovery_report(
    report: &StoreLockRecoveryReport,
) -> Result<(), Vec<StoreLockRecoveryValidationError>> {
    let mut errors = Vec::new();

    if report.entries.is_empty() {
        errors.push(StoreLockRecoveryValidationError::NoRegisteredStates);
    }

    for kind in IncidentClass::required_kinds() {
        let present = report
            .entries
            .iter()
            .any(|entry| entry.descriptor.incident_class == kind);
        if !present {
            errors.push(
                StoreLockRecoveryValidationError::RequiredIncidentKindMissing {
                    incident_kind: kind.as_str().to_owned(),
                },
            );
        }
    }

    for entry in &report.entries {
        if entry.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                StoreLockRecoveryValidationError::MissingDescriptorRevisionRef {
                    state_id: entry.descriptor.state_id.clone(),
                },
            );
        }
        for finding in &entry.blocking_findings {
            errors.push(StoreLockRecoveryValidationError::BlockingFindingPresent {
                state_id: finding.state_id().to_owned(),
                class: finding.class_token().to_owned(),
            });
        }
    }

    for (field, value) in report.cross_links.as_pairs() {
        if value.trim().is_empty() {
            errors.push(StoreLockRecoveryValidationError::CrossLinkMissing {
                field: field.to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(StoreLockRecoveryValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(StoreLockRecoveryValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_store_lock_recovery_report`].
struct StateSeed {
    state_id: &'static str,
    incident_class: IncidentClass,
    resource_class: ResourceClass,
    degraded_state_class: DegradedStateClass,
    last_seen_identity_ref: &'static str,
    paused: &'static [PausedCapabilityClass],
    local_only: &'static [LocalOnlyCapabilityClass],
    unsaved_local_state_posture: UnsavedLocalStatePosture,
    recovery_actions: &'static [RecoveryActionClass],
    repair_guidance_ref: &'static str,
    resume_posture: ResumePostureClass,
    continuations: &'static [(ContinuationClass, ResumeDispositionClass)],
    canonical_command_ref: &'static str,
    continuity_note: &'static str,
    degraded_state_vocabulary: &'static [&'static str],
}

fn build_state_from_seed(seed: &StateSeed) -> RecoveryStateRow {
    let descriptor = RecoveryState {
        state_id: seed.state_id.to_owned(),
        incident_class: seed.incident_class,
        resource_class: seed.resource_class,
        degraded_state_class: seed.degraded_state_class,
        descriptor_revision_ref: format!("{}:rev:2026.06.01-01", seed.state_id),
        primary_label_ref: format!("label:{}:primary", seed.state_id),
        last_seen_identity_ref: seed.last_seen_identity_ref.to_owned(),
        placeholder_ref: format!("placeholder:{}", seed.state_id),
        paused_capabilities: seed
            .paused
            .iter()
            .map(|class| PausedCapability {
                capability_class: *class,
                capability_ref: format!("paused:{}:{}", seed.state_id, class.as_str()),
            })
            .collect(),
        local_only_capabilities: seed
            .local_only
            .iter()
            .map(|class| LocalOnlyCapability {
                capability_class: *class,
                capability_ref: format!("local_only:{}:{}", seed.state_id, class.as_str()),
            })
            .collect(),
        unsaved_local_state_posture: seed.unsaved_local_state_posture,
        local_continuity_preserved: true,
        recovery_actions: seed.recovery_actions.to_vec(),
        repair_guidance_ref: seed.repair_guidance_ref.to_owned(),
        implies_plaintext_fallback: false,
        resume_posture: seed.resume_posture,
        resumes_silently_on_recovery: false,
        protected_continuations: seed
            .continuations
            .iter()
            .map(|(class, disposition)| ProtectedContinuation {
                continuation_ref: format!("continuation:{}:{}", seed.state_id, class.as_str()),
                continuation_class: *class,
                resume_disposition: *disposition,
            })
            .collect(),
        active_profile_owner_ref: format!("profile-owner:{}", seed.state_id),
        trust_checkpoint_ref: format!("trust:{}:profile_policy", seed.state_id),
        canonical_command_ref: seed.canonical_command_ref.to_owned(),
        continuity_note: seed.continuity_note.to_owned(),
        degraded_state_vocabulary: seed
            .degraded_state_vocabulary
            .iter()
            .map(|phrase| (*phrase).to_owned())
            .collect(),
        surface_parity: SurfaceClass::required().to_vec(),
        claimed_platforms: Platform::all().to_vec(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:store_lock_and_external_root:narrow_on_stale_evidence"
            .to_owned(),
        marketed: true,
        registered_on_recovery_harness: true,
    };
    build_store_lock_recovery_row(descriptor)
}

const STATE_SEEDS: &[StateSeed] = &[
    // ---- Credential-store lock: the OS keychain is locked. ----
    StateSeed {
        state_id: "state:credential_store.locked",
        incident_class: IncidentClass::CredentialStoreLocked,
        resource_class: ResourceClass::CredentialStore,
        degraded_state_class: DegradedStateClass::StoreLocked,
        last_seen_identity_ref: "identity:credential_store.os_keychain_login",
        paused: &[
            PausedCapabilityClass::ProviderAuthentication,
            PausedCapabilityClass::ManagedSync,
        ],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::LocalHistory,
            LocalOnlyCapabilityClass::LocalExport,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedInPlace,
        recovery_actions: &[
            RecoveryActionClass::UnlockStore,
            RecoveryActionClass::RetryAfterUnlock,
            RecoveryActionClass::RepairStore,
        ],
        repair_guidance_ref: "repair:credential_store.unlock",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[
            (
                ContinuationClass::QueuedJob,
                ResumeDispositionClass::ExplicitResumeRequired,
            ),
            (
                ContinuationClass::RememberedDecision,
                ResumeDispositionClass::HeldForReview,
            ),
        ],
        canonical_command_ref: "cmd:identity.store.unlock",
        continuity_note: "The OS credential store is locked: provider authentication and managed sync are paused, but local editing, local history, and local export are unaffected. Recovery is to unlock the store; a queued provider job and a remembered store-preference decision are held for explicit resume, never re-run automatically, and no secret is ever written to plaintext.",
        degraded_state_vocabulary: &[
            "Your secure store is locked",
            "Unlock it to use saved credentials again",
            "Your local work is unaffected and nothing is stored in plain text",
        ],
    },
    // ---- Credential-store unavailable: the backend is unreachable. ----
    StateSeed {
        state_id: "state:credential_store.unavailable",
        incident_class: IncidentClass::CredentialStoreUnavailable,
        resource_class: ResourceClass::CredentialStore,
        degraded_state_class: DegradedStateClass::StoreUnavailable,
        last_seen_identity_ref: "identity:credential_store.os_secret_service",
        paused: &[
            PausedCapabilityClass::ProviderAuthentication,
            PausedCapabilityClass::SignedOperation,
        ],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::OfflineCoreTools,
            LocalOnlyCapabilityClass::LocalExport,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedInPlace,
        recovery_actions: &[
            RecoveryActionClass::RepairStore,
            RecoveryActionClass::RetryAfterUnlock,
        ],
        repair_guidance_ref: "repair:credential_store.reconnect_backend",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[(
            ContinuationClass::RunningSession,
            ResumeDispositionClass::ExplicitResumeRequired,
        )],
        canonical_command_ref: "cmd:identity.store.repair",
        continuity_note: "The OS credential store backend is unreachable: provider authentication and signing are paused while offline core tooling, local editing, and local export continue. Recovery is to repair the backend handle; a running provider session is held for explicit resume and never silently widened, and no plaintext-secret fallback is offered.",
        degraded_state_vocabulary: &[
            "Your secure store is unavailable right now",
            "Repair the connection to use saved credentials again",
            "Local editing keeps working and no secret is exposed in plain text",
        ],
    },
    // ---- Trust-store drift: anchors changed under a remembered decision. ----
    StateSeed {
        state_id: "state:trust_store.drift",
        incident_class: IncidentClass::TrustStoreDrift,
        resource_class: ResourceClass::TrustStore,
        degraded_state_class: DegradedStateClass::TrustStoreDrifted,
        last_seen_identity_ref: "identity:trust_store.system_roots",
        paused: &[
            PausedCapabilityClass::CertificateValidation,
            PausedCapabilityClass::ManagedSync,
        ],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::LocalHistory,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedInPlace,
        recovery_actions: &[
            RecoveryActionClass::ReviewTrustChange,
            RecoveryActionClass::ReEvaluateTrust,
        ],
        repair_guidance_ref: "repair:trust_store.review_drift",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[(
            ContinuationClass::RememberedDecision,
            ResumeDispositionClass::HeldForReview,
        )],
        canonical_command_ref: "cmd:identity.trust.review",
        continuity_note: "The trust store drifted from the anchors a remembered decision was made against: certificate validation and managed sync are paused while local editing and local history continue. Recovery is to review the change and re-evaluate trust; the remembered trust acceptance is held for review and never silently re-applied to the new anchors.",
        degraded_state_vocabulary: &[
            "The trust store changed since you last accepted it",
            "Review the change before connections resume",
            "Your earlier trust decision is held for review, not re-applied automatically",
        ],
    },
    // ---- Removable volume missing: a USB / SD volume was ejected. ----
    StateSeed {
        state_id: "state:removable_volume.missing",
        incident_class: IncidentClass::RemovableVolumeMissing,
        resource_class: ResourceClass::RemovableVolume,
        degraded_state_class: DegradedStateClass::RootMissing,
        last_seen_identity_ref: "identity:removable_volume.last_seen_label",
        paused: &[PausedCapabilityClass::ExternalRootAccess],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::CachedContextBrowse,
            LocalOnlyCapabilityClass::LocalExport,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedPendingRecovery,
        recovery_actions: &[
            RecoveryActionClass::RemountVolume,
            RecoveryActionClass::LocateRoot,
            RecoveryActionClass::OpenCachedContext,
            RecoveryActionClass::ClosePlaceholder,
        ],
        repair_guidance_ref: "repair:removable_volume.remount",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[(
            ContinuationClass::QueuedJob,
            ResumeDispositionClass::ExplicitResumeRequired,
        )],
        canonical_command_ref: "cmd:workspace.root.recover",
        continuity_note: "A removable volume was ejected: access to files on that root is paused, but the placeholder names the last-seen volume label, unsaved local edits are preserved pending recovery, and cached context stays browsable. Recovery offers Remount, Locate, Open cached context, and Close; a write queued to the volume is held for explicit resume, never replayed automatically when the volume returns.",
        degraded_state_vocabulary: &[
            "The removable volume is no longer connected",
            "Locate it, open the cached copy, or close this placeholder",
            "Your unsaved work is kept and nothing is rewritten without you",
        ],
    },
    // ---- Network share missing: a mounted share disconnected. ----
    StateSeed {
        state_id: "state:network_share.missing",
        incident_class: IncidentClass::NetworkShareMissing,
        resource_class: ResourceClass::NetworkShare,
        degraded_state_class: DegradedStateClass::RootMissing,
        last_seen_identity_ref: "identity:network_share.last_seen_mount",
        paused: &[
            PausedCapabilityClass::ExternalRootAccess,
            PausedCapabilityClass::ManagedSync,
        ],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::CachedContextBrowse,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedPendingRecovery,
        recovery_actions: &[
            RecoveryActionClass::ReconnectNetworkShare,
            RecoveryActionClass::LocateRoot,
            RecoveryActionClass::OpenCachedContext,
            RecoveryActionClass::ClosePlaceholder,
        ],
        repair_guidance_ref: "repair:network_share.reconnect",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[(
            ContinuationClass::QueuedJob,
            ResumeDispositionClass::ExplicitResumeRequired,
        )],
        canonical_command_ref: "cmd:workspace.root.recover",
        continuity_note: "A network share disconnected: access to files on that mount is paused while local editing and cached-context browsing continue. The placeholder names the last-seen mount and preserves unsaved edits pending recovery; recovery offers Reconnect, Locate, Open cached context, and Close, and a queued write is held for explicit resume rather than replayed when the share returns.",
        degraded_state_vocabulary: &[
            "The network share is disconnected",
            "Reconnect it, open the cached copy, or close this placeholder",
            "Your unsaved work is preserved and not silently re-sent",
        ],
    },
    // ---- External root missing: a general external root is gone. ----
    StateSeed {
        state_id: "state:external_root.missing",
        incident_class: IncidentClass::ExternalRootMissing,
        resource_class: ResourceClass::ExternalRoot,
        degraded_state_class: DegradedStateClass::RootMissing,
        last_seen_identity_ref: "identity:external_root.last_seen_path_alias",
        paused: &[PausedCapabilityClass::ExternalRootAccess],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::CachedContextBrowse,
            LocalOnlyCapabilityClass::LocalHistory,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedPendingRecovery,
        recovery_actions: &[
            RecoveryActionClass::LocateRoot,
            RecoveryActionClass::OpenCachedContext,
            RecoveryActionClass::ClosePlaceholder,
        ],
        repair_guidance_ref: "repair:external_root.locate",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[(
            ContinuationClass::RunningSession,
            ResumeDispositionClass::HeldForReview,
        )],
        canonical_command_ref: "cmd:workspace.root.recover",
        continuity_note: "An external root went missing: access to its files is paused, the placeholder names the last-seen path alias, unsaved edits are preserved pending recovery, and local history stays available. Recovery offers Locate, Open cached context, and Close; a session bound to the root is held for review and never silently rejoined when the root reappears.",
        degraded_state_vocabulary: &[
            "This external location is no longer available",
            "Locate it, open the cached copy, or close this placeholder",
            "Your local history and unsaved work are intact",
        ],
    },
    // ---- Root returned: a previously missing share is back, awaiting resume. ----
    StateSeed {
        state_id: "state:network_share.returned",
        incident_class: IncidentClass::RootReturned,
        resource_class: ResourceClass::NetworkShare,
        degraded_state_class: DegradedStateClass::RootReturned,
        last_seen_identity_ref: "identity:network_share.returned_mount",
        paused: &[],
        local_only: &[
            LocalOnlyCapabilityClass::LocalEditing,
            LocalOnlyCapabilityClass::LocalHistory,
        ],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedPendingRecovery,
        recovery_actions: &[
            RecoveryActionClass::ConfirmExplicitResume,
            RecoveryActionClass::OpenCachedContext,
        ],
        repair_guidance_ref: "repair:network_share.confirm_resume",
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        continuations: &[
            (
                ContinuationClass::QueuedJob,
                ResumeDispositionClass::ExplicitResumeRequired,
            ),
            (
                ContinuationClass::RunningSession,
                ResumeDispositionClass::ExplicitResumeRequired,
            ),
            (
                ContinuationClass::RememberedDecision,
                ResumeDispositionClass::HeldForReview,
            ),
        ],
        canonical_command_ref: "cmd:workspace.root.confirm_resume",
        continuity_note: "A previously missing network share has returned, but nothing resumes on its own: the held write, the bound session, and the remembered decision all require explicit confirmation before they continue. The placeholder is reconciled to the returned mount and the user confirms what to resume, so a returned root never auto-rejoins a session or replays a deferred write.",
        degraded_state_vocabulary: &[
            "The network share is back",
            "Review what was waiting before it continues",
            "Nothing was resumed automatically",
        ],
    },
];

/// Seeded report builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5-store-lock-and-missing-root/`.
pub fn seeded_store_lock_recovery_report() -> StoreLockRecoveryReport {
    let entries = STATE_SEEDS.iter().map(build_state_from_seed).collect();
    build_store_lock_recovery_report(entries)
}

/// Stable case-id label for the four required incident families: store lock,
/// trust-store drift, missing root, and returned root.
pub const STORE_LOCK_RECOVERY_CASE_LABELS: [(&str, &str); 4] = [
    ("state:credential_store.locked", "credential_store_locked"),
    ("state:trust_store.drift", "trust_store_drift"),
    ("state:external_root.missing", "missing_root"),
    ("state:network_share.returned", "root_returned"),
];

/// Builds the four per-incident case exports from the seeded report, in
/// canonical order.
pub fn seeded_store_lock_recovery_case_exports() -> Vec<StoreLockRecoveryCaseExport> {
    let report = seeded_store_lock_recovery_report();
    STORE_LOCK_RECOVERY_CASE_LABELS
        .iter()
        .filter_map(|(state_id, label)| {
            let row = report
                .entries
                .iter()
                .find(|entry| entry.descriptor.state_id == *state_id)?
                .clone();
            Some(StoreLockRecoveryCaseExport::from_row(
                format!("support-export:m5-store-lock-and-external-root-recovery:case:{label}"),
                *label,
                format!(
                    "Reproduce the {label} incident from this typed diagnostic: the last-seen store/root identity, the paused capabilities, what remains local-only, the unsaved-local-state posture, the offered recovery actions, and the protected continuations held for explicit resume.",
                ),
                row,
            ))
        })
        .collect()
}
