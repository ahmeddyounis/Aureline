//! Live-target handoff packets that validate target existence, current scope / workset visibility,
//! remote / managed route availability, trust posture, and required auth / approval before pivoting from a
//! preserved M5 snapshot into a current live object, across the shell / archive-viewer, help / docs, support,
//! review / incident, runbook-archive, release-center, companion / export, program-governance, and
//! CLI / export surfaces at **one canonical handoff vocabulary and validation-rule set**.
//!
//! This module is the B149 live-target-handoff implement lane over the five non-live-evidence object classes
//! frozen in [`crate::m5_historical_reference_matrix`]. Where the archive-viewer lane
//! ([`crate::m5_archived_snapshot_viewer_and_analysis_only_banner_consumers`]) proves how a preserved snapshot
//! is *shown* as non-live and the compare-flow lane
//! ([`crate::m5_historical_versus_live_compare_flow`]) proves how it is *compared* against its live target,
//! this lane makes "open current live object" a **reviewable, validated handoff** rather than a hidden jump
//! from non-live evidence into live mutable state: every handoff carries a typed
//! [`LiveTargetHandoffRequest`] and either completes the pivot only after all five preconditions clear, or
//! reports the exact blocker and falls back to a metadata-only exit — never a dead end, never a silent widen.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **A seeded snapshot produces a typed packet that either completes safely or names the exact blocker.**
//!    Each binding carries a versioned [`LiveTargetHandoffRequest`] (source snapshot id, target identity,
//!    required route class, trust / auth prerequisites, requested authority class, and a fallback behavior)
//!    plus a [`HandoffPreconditionCheck`] over the five preconditions. The [`HandoffOutcome`] is
//!    [`HandoffOutcome::HandoffCleared`] only when every precondition clears; otherwise it is a blocked
//!    outcome carrying an explicit [`HandoffBlockerNote`] naming exactly why —
//!    [`HandoffBlockerReason::TargetDoesNotExist`], [`HandoffBlockerReason::TargetOutsideCurrentScope`],
//!    [`HandoffBlockerReason::RouteUnavailable`], [`HandoffBlockerReason::TrustPostureInsufficient`],
//!    [`HandoffBlockerReason::AuthOrApprovalMissing`],
//!    [`HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart`], or
//!    [`HandoffBlockerReason::PolicyOrLifecycleBlocked`].
//! 2. **A handoff never widens authority.** The request records both the authority it would open at
//!    (`requested_authority_class`) and the authority the same object grants when opened directly from an
//!    ordinary surface (`direct_open_authority_class`); the requested authority may never exceed the direct
//!    one, and the completed pivot never grants wider authority than a direct open would.
//! 3. **Packets are export-safe and auditable without leaking secrets.** The packet references upstream
//!    historical-reference contracts by id, names auth / approval prerequisites as controlled tokens rather
//!    than embedding secrets, credentials, or private endpoints, and completing an actual approval / auth
//!    refresh is delegated to a separate, reviewed [`ReviewedAuthorityHandoff`] path — this lane defines the
//!    typed handoff and its validation checks, and never bypasses approval, trust, or auth refresh itself.
//!
//! Every binding names the accessibility routes ([`M5HistoricalReferenceAccessibilityRoute`]) through which
//! the handoff state, its provenance, and the open-live-target action can be discovered without pointer-only
//! chrome; keyboard focus and screen-reader announcement are mandatory. The historical side stays visibly
//! non-live and mutation blocked throughout, and the historical-side grammar
//! ([`HandoffHistoricalGrammar`]) is identical across every surface that renders the same profile.
//!
//! The boundary schema is
//! [`schemas/program/m5-live-target-handoff-packet-and-route-validation.schema.json`](../../../../schemas/program/m5-live-target-handoff-packet-and-route-validation.schema.json).
//! The contract doc is
//! [`docs/support/m5_live_target_handoff_packet_and_route_validation.md`](../../../../docs/support/m5_live_target_handoff_packet_and_route_validation.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-live-target-handoff/`](../../../../fixtures/recovery/m5-live-target-handoff/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_live_target_handoff, seeded_m5_live_target_handoff_blocked_target_narrowed,
    seeded_m5_live_target_handoff_needs_prerequisite_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5LiveTargetHandoffPacket`].
pub const M5_LIVE_TARGET_HANDOFF_RECORD_KIND: &str = "m5_live_target_handoff_registry";

/// Schema version for live-target-handoff records.
pub const M5_LIVE_TARGET_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_LIVE_TARGET_HANDOFF_PACKET_ID: &str = "m5-live-target-handoff:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_LIVE_TARGET_HANDOFF_SCHEMA_REF: &str =
    "schemas/program/m5-live-target-handoff-packet-and-route-validation.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LIVE_TARGET_HANDOFF_DOC_REF: &str =
    "docs/support/m5_live_target_handoff_packet_and_route_validation.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LIVE_TARGET_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/support/m5-live-target-handoff/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_LIVE_TARGET_HANDOFF_CSV_REF: &str =
    "artifacts/support/m5-live-target-handoff/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_LIVE_TARGET_HANDOFF_REPORT_REF: &str =
    "artifacts/support/m5-live-target-handoff/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_LIVE_TARGET_HANDOFF_FIXTURE_DIR: &str = "fixtures/recovery/m5-live-target-handoff";

/// Proof-freshness SLO in hours for this lane.
pub const M5_LIVE_TARGET_HANDOFF_PROOF_SLO_HOURS: u32 = 720;

/// Mutation-blocked-posture sentinel words a historical-side grammar may never fall back to; a handoff whose
/// historical role must be present before surfacing as non-live evidence must always keep a real
/// mutation-blocked posture rather than implying the object is editable, live, writable, or the current
/// object.
const MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its
/// canonical contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5HistoricalReferenceConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5HistoricalReferenceConsumerSurface::Support
            | M5HistoricalReferenceConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5HistoricalReferenceRole`] vocabulary.
pub fn is_known_historical_reference_role_token(token: &str) -> bool {
    historical_reference_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5HistoricalReferenceRole`], if it is one.
pub fn historical_reference_role_from_token(token: &str) -> Option<M5HistoricalReferenceRole> {
    M5HistoricalReferenceRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// The route class a handoff must reopen the current live object through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveTargetRouteClass {
    /// An in-process workspace object opened locally.
    InProcessWorkspace,
    /// A remote / managed service object reopened over a managed route.
    RemoteManagedService,
    /// A companion / browser surface reopened through the companion route.
    CompanionBrowserSurface,
    /// A CLI / export reopen path.
    CliReopenPath,
}

impl LiveTargetRouteClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InProcessWorkspace,
        Self::RemoteManagedService,
        Self::CompanionBrowserSurface,
        Self::CliReopenPath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessWorkspace => "in_process_workspace",
            Self::RemoteManagedService => "remote_managed_service",
            Self::CompanionBrowserSurface => "companion_browser_surface",
            Self::CliReopenPath => "cli_reopen_path",
        }
    }
}

/// The trust posture a handoff requires before it may reopen a live target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveTargetTrustPosture {
    /// The current session is already trusted for the target route.
    TrustedCurrentSession,
    /// Trust must be revalidated before the target may be reopened.
    NeedsTrustRevalidation,
    /// The target route is untrusted and the handoff cannot proceed.
    Untrusted,
}

impl LiveTargetTrustPosture {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::TrustedCurrentSession,
        Self::NeedsTrustRevalidation,
        Self::Untrusted,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedCurrentSession => "trusted_current_session",
            Self::NeedsTrustRevalidation => "needs_trust_revalidation",
            Self::Untrusted => "untrusted",
        }
    }
}

/// The authority class a reopened live object is granted.
///
/// Ordered least-to-most, so a handoff can prove it never widens authority: the authority a handoff opens at
/// may never exceed the authority the same object grants when opened directly from an ordinary surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveTargetAuthorityClass {
    /// Read-only inspection of the current live object.
    ReadOnlyInspect,
    /// Scoped edit authority over the current live object.
    ScopedEdit,
    /// Elevated / administrative authority over the current live object.
    ElevatedAdmin,
}

impl LiveTargetAuthorityClass {
    /// Every variant, in declaration order (least to most authority).
    pub const ALL: [Self; 3] = [Self::ReadOnlyInspect, Self::ScopedEdit, Self::ElevatedAdmin];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyInspect => "read_only_inspect",
            Self::ScopedEdit => "scoped_edit",
            Self::ElevatedAdmin => "elevated_admin",
        }
    }
}

/// A named auth / approval prerequisite a handoff must satisfy before reopening a live target.
///
/// These are controlled tokens — never secrets, credentials, or ambient authority — so the packet stays
/// export-safe and auditable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAuthPrerequisite {
    /// The current session is authenticated.
    CurrentSessionAuthenticated,
    /// A credential is fresh within its time-to-live (named, not embedded).
    FreshCredentialWithinTtl,
    /// An approval is on record for the target route.
    ApprovalOnRecordForRoute,
    /// The requesting authority scope is confirmed for the target.
    AuthorityScopeConfirmed,
}

impl HandoffAuthPrerequisite {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentSessionAuthenticated,
        Self::FreshCredentialWithinTtl,
        Self::ApprovalOnRecordForRoute,
        Self::AuthorityScopeConfirmed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentSessionAuthenticated => "current_session_authenticated",
            Self::FreshCredentialWithinTtl => "fresh_credential_within_ttl",
            Self::ApprovalOnRecordForRoute => "approval_on_record_for_route",
            Self::AuthorityScopeConfirmed => "authority_scope_confirmed",
        }
    }
}

/// The outcome of validating a live-target handoff.
///
/// The outcome governs the discoverable action set, parity state, and blocker disclosure — never the
/// historical-side grammar: a blocked handoff still carries the same historical-role, snapshot-label,
/// capture-time, provenance, and mutation-blocked-posture words and discloses the block through an explicit
/// blocker note plus a metadata-only or satisfy-prerequisite fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOutcome {
    /// Every precondition cleared; the handoff completes safely and reopens the current live object at the
    /// validated authority class.
    HandoffCleared,
    /// The live target exists and is in scope, but a route, trust, or auth / approval prerequisite is unmet;
    /// the handoff is blocked and offers a satisfy-prerequisite-then-retry fallback (it never bypasses the
    /// prerequisite itself).
    BlockedNeedsPrerequisite,
    /// No live target exists or it is outside the current scope / workset; the handoff falls back to a
    /// metadata-only exit instead of a dead end.
    BlockedTargetUnavailable,
    /// A policy or lifecycle rule blocks the live reopen; the handoff falls back to a metadata-only exit
    /// instead of a dead end.
    BlockedByPolicy,
}

impl HandoffOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::HandoffCleared,
        Self::BlockedNeedsPrerequisite,
        Self::BlockedTargetUnavailable,
        Self::BlockedByPolicy,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandoffCleared => "handoff_cleared",
            Self::BlockedNeedsPrerequisite => "blocked_needs_prerequisite",
            Self::BlockedTargetUnavailable => "blocked_target_unavailable",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }

    /// Whether this outcome blocks the live pivot.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::HandoffCleared)
    }

    /// The blocker reasons this outcome is allowed to name. A cleared handoff names none; every blocked
    /// outcome must name exactly one reason from its allowed set.
    pub const fn allowed_blocker_reasons(self) -> &'static [HandoffBlockerReason] {
        match self {
            Self::HandoffCleared => &[],
            Self::BlockedNeedsPrerequisite => &[
                HandoffBlockerReason::RouteUnavailable,
                HandoffBlockerReason::TrustPostureInsufficient,
                HandoffBlockerReason::AuthOrApprovalMissing,
            ],
            Self::BlockedTargetUnavailable => &[
                HandoffBlockerReason::TargetDoesNotExist,
                HandoffBlockerReason::TargetOutsideCurrentScope,
                HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart,
            ],
            Self::BlockedByPolicy => &[
                HandoffBlockerReason::PolicyOrLifecycleBlocked,
                HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart,
            ],
        }
    }
}

/// The action a handoff surface may expose.
///
/// The set is deliberately closed and analysis-only apart from the single validated pivot: there is no
/// apply / sync / restore action, and `OpenCurrentLiveObject` appears only when the handoff has cleared every
/// precondition, so a handoff surface can never reopen live state from an unvalidated snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffAction {
    /// Inspect the preserved historical packet metadata-only.
    InspectHistoricalPacket,
    /// Export the live-target handoff packet.
    ExportHandoffPacket,
    /// Open the current live object — only when every precondition has cleared.
    OpenCurrentLiveObject,
}

impl HandoffAction {
    /// The analysis-only base action set present on every handoff surface.
    pub const ANALYSIS_ONLY_BASE: [Self; 2] =
        [Self::InspectHistoricalPacket, Self::ExportHandoffPacket];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectHistoricalPacket => "inspect_historical_packet",
            Self::ExportHandoffPacket => "export_handoff_packet",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// Why a handoff blocked the live pivot below a cleared handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBlockerReason {
    /// The live target does not exist / was removed.
    TargetDoesNotExist,
    /// The live target is outside the current scope / workset visibility.
    TargetOutsideCurrentScope,
    /// The remote / managed route to the live target is unavailable.
    RouteUnavailable,
    /// The trust posture is insufficient (revalidation required or untrusted).
    TrustPostureInsufficient,
    /// A required auth / approval prerequisite is missing.
    AuthOrApprovalMissing,
    /// The snapshot describes a retired capability with no live counterpart.
    RetiredCapabilityNoLiveCounterpart,
    /// A policy or lifecycle rule blocks reopening the live target.
    PolicyOrLifecycleBlocked,
}

impl HandoffBlockerReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TargetDoesNotExist,
        Self::TargetOutsideCurrentScope,
        Self::RouteUnavailable,
        Self::TrustPostureInsufficient,
        Self::AuthOrApprovalMissing,
        Self::RetiredCapabilityNoLiveCounterpart,
        Self::PolicyOrLifecycleBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetDoesNotExist => "target_does_not_exist",
            Self::TargetOutsideCurrentScope => "target_outside_current_scope",
            Self::RouteUnavailable => "route_unavailable",
            Self::TrustPostureInsufficient => "trust_posture_insufficient",
            Self::AuthOrApprovalMissing => "auth_or_approval_missing",
            Self::RetiredCapabilityNoLiveCounterpart => "retired_capability_no_live_counterpart",
            Self::PolicyOrLifecycleBlocked => "policy_or_lifecycle_blocked",
        }
    }

    /// Whether the precondition check supports this blocker reason (the mapped precondition is unmet).
    ///
    /// This ties each named blocker to a real failed precondition, so the packet cannot claim a blocker while
    /// every precondition cleared.
    pub const fn supported_by(self, check: &HandoffPreconditionCheck) -> bool {
        match self {
            Self::TargetDoesNotExist | Self::RetiredCapabilityNoLiveCounterpart => {
                !check.target_exists
            }
            Self::TargetOutsideCurrentScope => !check.target_in_current_scope,
            Self::RouteUnavailable | Self::PolicyOrLifecycleBlocked => !check.route_available,
            Self::TrustPostureInsufficient => !check.trust_posture_satisfied,
            Self::AuthOrApprovalMissing => !check.auth_and_approval_satisfied,
        }
    }
}

/// The next action a blocked handoff offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffBlockerNextAction {
    /// Satisfy the named prerequisite through its reviewed path, then retry the handoff.
    SatisfyPrerequisiteThenRetry,
    /// Inspect the historical packet metadata-only when no live reopen is possible.
    InspectHistoricalPacketOnly,
}

impl HandoffBlockerNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SatisfyPrerequisiteThenRetry => "satisfy_prerequisite_then_retry",
            Self::InspectHistoricalPacketOnly => "inspect_historical_packet_only",
        }
    }
}

/// The fallback behavior a handoff declares for when the target cannot be reopened live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFallbackBehavior {
    /// The validated live target is opened (used when the handoff cleared).
    OpenValidatedLiveTarget,
    /// Offer the reviewed prerequisite path, then retry.
    OfferPrerequisiteThenRetry,
    /// Fall back to a metadata-only exit; the historical packet stays inspectable.
    MetadataOnlyExit,
}

impl HandoffFallbackBehavior {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenValidatedLiveTarget => "open_validated_live_target",
            Self::OfferPrerequisiteThenRetry => "offer_prerequisite_then_retry",
            Self::MetadataOnlyExit => "metadata_only_exit",
        }
    }
}

/// Whether a binding completed a cleared handoff or discloses a blocked one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffParityState {
    /// Historical grammar and a cleared, validated live pivot are preserved and shown.
    HandoffClearedCompleted,
    /// Historical grammar is preserved and a blocked handoff is explicitly disclosed.
    HandoffBlockedDisclosed,
}

impl HandoffParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandoffClearedCompleted => "handoff_cleared_completed",
            Self::HandoffBlockedDisclosed => "handoff_blocked_disclosed",
        }
    }
}

/// Downgrade trigger that can narrow this handoff lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveTargetHandoffDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Historical grammar drifted between surfaces for the same profile.
    HandoffGrammarDriftDetected,
    /// A historical side dropped its mutation-blocked posture and began to imply the object is live.
    MutationBlockedPostureDropped,
    /// A surface reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A handoff widened authority beyond what a direct open would grant.
    AuthorityWidenedBeyondDirectOpen,
    /// A surface dead-ended when the target was unavailable instead of a metadata-only exit.
    DeadEndsWhenTargetUnavailable,
    /// A packet leaked a secret or ambient credential into the export.
    SecretOrAmbientCredentialLeaked,
    /// A surface presented a snapshot as a current live object.
    PresentsSnapshotAsCurrentLiveObject,
    /// An accessibility route for the handoff state, provenance, or open-live-target action was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl LiveTargetHandoffDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::HandoffGrammarDriftDetected,
        Self::MutationBlockedPostureDropped,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::AuthorityWidenedBeyondDirectOpen,
        Self::DeadEndsWhenTargetUnavailable,
        Self::SecretOrAmbientCredentialLeaked,
        Self::PresentsSnapshotAsCurrentLiveObject,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::HandoffGrammarDriftDetected => "handoff_grammar_drift_detected",
            Self::MutationBlockedPostureDropped => "mutation_blocked_posture_dropped",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::AuthorityWidenedBeyondDirectOpen => "authority_widened_beyond_direct_open",
            Self::DeadEndsWhenTargetUnavailable => "dead_ends_when_target_unavailable",
            Self::SecretOrAmbientCredentialLeaked => "secret_or_ambient_credential_leaked",
            Self::PresentsSnapshotAsCurrentLiveObject => "presents_snapshot_as_current_live_object",
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled historical-side grammar a preserved-snapshot profile presents.
///
/// These five words describe the historical (non-live) side of the handoff and must be identical across every
/// consumer surface that shows the same profile. The historical-role word must be a frozen
/// [`M5HistoricalReferenceRole`] token; the rest are controlled words the snapshot carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffHistoricalGrammar {
    /// Historical-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub historical_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the snapshot is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the snapshot is attributed to.
    pub provenance_word: String,
    /// The mutation-blocked-posture word (read-only, non-authoritative-for-mutation).
    pub mutation_blocked_posture_word: String,
}

impl HandoffHistoricalGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.historical_role_word.trim().is_empty()
            && !self.snapshot_label_word.trim().is_empty()
            && !self.capture_time_word.trim().is_empty()
            && !self.provenance_word.trim().is_empty()
            && !self.mutation_blocked_posture_word.trim().is_empty()
    }

    /// Whether the historical-role word is a member of the frozen role vocabulary.
    pub fn historical_role_word_in_vocabulary(&self) -> bool {
        is_known_historical_reference_role_token(self.historical_role_word.trim())
    }

    /// Whether the profile honours the mutation-blocked rule: a historical-side role that must be present
    /// before the object may be surfaced as non-live evidence must pair it with a real mutation-blocked
    /// posture word and never collapse to an editable / live / writable / current-object sentinel.
    pub fn mutation_blocked_posture_satisfied(&self) -> bool {
        match historical_reference_role_from_token(self.historical_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_non_live_evidence() => {
                let posture = self.mutation_blocked_posture_word.trim().to_lowercase();
                !posture.is_empty()
                    && !MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS.contains(&posture.as_str())
            }
            _ => true,
        }
    }
}

/// The identity of the current live object a handoff targets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetIdentity {
    /// Stable id of the current live object.
    pub target_id: String,
    /// Human-readable label of the current live object.
    pub target_label: String,
    /// The kind of live object (for example `workspace_object`, `runbook_run`, `incident_object`).
    pub target_kind: String,
}

impl LiveTargetIdentity {
    /// Whether every identity field is present.
    pub fn all_present(&self) -> bool {
        !self.target_id.trim().is_empty()
            && !self.target_label.trim().is_empty()
            && !self.target_kind.trim().is_empty()
    }
}

/// The five preconditions a handoff validates before completing the live pivot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffPreconditionCheck {
    /// The live target exists.
    pub target_exists: bool,
    /// The live target is visible in the current scope / workset.
    pub target_in_current_scope: bool,
    /// The remote / managed route to the live target is available.
    pub route_available: bool,
    /// The trust posture is satisfied for the target route.
    pub trust_posture_satisfied: bool,
    /// The required auth / approval prerequisites are satisfied.
    pub auth_and_approval_satisfied: bool,
}

impl HandoffPreconditionCheck {
    /// Whether every precondition cleared.
    pub const fn all_cleared(&self) -> bool {
        self.target_exists
            && self.target_in_current_scope
            && self.route_available
            && self.trust_posture_satisfied
            && self.auth_and_approval_satisfied
    }
}

/// The versioned, typed live-target handoff packet a preserved snapshot produces.
///
/// It carries the source snapshot id, the target identity, the required route class, the trust and auth
/// prerequisites, the requested authority class (and the authority a direct open would grant, so widening can
/// be proven impossible), the precondition check, and the fallback behavior when the target cannot be
/// reopened live.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetHandoffRequest {
    /// Stable id of the source historical snapshot the handoff pivots from.
    pub source_snapshot_id: String,
    /// The identity of the current live object the handoff targets.
    pub target_identity: LiveTargetIdentity,
    /// The route class the handoff must reopen through.
    pub required_route_class: LiveTargetRouteClass,
    /// The trust posture the handoff requires.
    pub required_trust_posture: LiveTargetTrustPosture,
    /// The named auth / approval prerequisites the handoff requires.
    pub required_auth_prerequisites: Vec<HandoffAuthPrerequisite>,
    /// The authority class the handoff would open the live object at.
    pub requested_authority_class: LiveTargetAuthorityClass,
    /// The authority class opening the same object directly from an ordinary surface would grant.
    pub direct_open_authority_class: LiveTargetAuthorityClass,
    /// The result of the five-precondition check.
    pub precondition_check: HandoffPreconditionCheck,
    /// The fallback behavior when the target cannot be reopened live.
    pub fallback_behavior: HandoffFallbackBehavior,
}

impl LiveTargetHandoffRequest {
    /// Whether the requested authority never exceeds what a direct open would grant.
    pub fn authority_not_widened(&self) -> bool {
        self.requested_authority_class <= self.direct_open_authority_class
    }

    /// Whether every required string / identity field is present.
    pub fn all_present(&self) -> bool {
        !self.source_snapshot_id.trim().is_empty()
            && self.target_identity.all_present()
            && !self.required_auth_prerequisites.is_empty()
    }
}

/// The explicit note a blocked handoff shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffBlockerNote {
    /// Why the handoff blocked.
    pub reason: HandoffBlockerReason,
    /// A never-omitted explanation of the exact blocker.
    pub explanation: String,
    /// Note that the historical side stays preserved and non-live (never omitted).
    pub preserved_historical_note: String,
    /// The fallback behavior the block routes into.
    pub fallback_behavior: HandoffFallbackBehavior,
    /// The next action offered.
    pub next_action: HandoffBlockerNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// An explicit, reviewed authority / approval / auth-refresh path that owns any actual elevation.
///
/// This lane never bypasses approval, trust, or auth refresh; when a prerequisite is unmet, this handoff names
/// the separate, reviewed path that owns satisfying it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedAuthorityHandoff {
    /// Stable id of the reviewed authority / approval path.
    pub reviewed_path_id: String,
    /// Human-readable label of the reviewed authority / approval path.
    pub reviewed_path_label: String,
}

/// Disclosures a handoff binding must carry, derived from its outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandoffRenderDisclosure {
    /// The parity state the outcome requires.
    pub parity_state: HandoffParityState,
    /// The next action the blocker note must offer, if any.
    pub blocker_next_action: Option<HandoffBlockerNextAction>,
    /// The fallback behavior the outcome requires.
    pub fallback_behavior: HandoffFallbackBehavior,
    /// Whether the binding must carry an explicit blocker note.
    pub needs_blocker_note: bool,
    /// Whether the binding requires every precondition cleared.
    pub requires_cleared_preconditions: bool,
    /// Whether the binding offers a validated open-current-live-object action.
    pub offers_open_live_target: bool,
}

/// Resolves the render disclosures a handoff binding must carry from its outcome.
///
/// A cleared handoff renders the full analysis-only action set plus a validated open-current-live-object
/// action and requires every precondition cleared. A blocked handoff narrows the pivot and discloses the block
/// through an explicit note plus a satisfy-prerequisite or metadata-only fallback — but all keep every
/// historical grammar word.
pub const fn resolve_handoff_render_disclosure(outcome: HandoffOutcome) -> HandoffRenderDisclosure {
    match outcome {
        HandoffOutcome::HandoffCleared => HandoffRenderDisclosure {
            parity_state: HandoffParityState::HandoffClearedCompleted,
            blocker_next_action: None,
            fallback_behavior: HandoffFallbackBehavior::OpenValidatedLiveTarget,
            needs_blocker_note: false,
            requires_cleared_preconditions: true,
            offers_open_live_target: true,
        },
        HandoffOutcome::BlockedNeedsPrerequisite => HandoffRenderDisclosure {
            parity_state: HandoffParityState::HandoffBlockedDisclosed,
            blocker_next_action: Some(HandoffBlockerNextAction::SatisfyPrerequisiteThenRetry),
            fallback_behavior: HandoffFallbackBehavior::OfferPrerequisiteThenRetry,
            needs_blocker_note: true,
            requires_cleared_preconditions: false,
            offers_open_live_target: false,
        },
        HandoffOutcome::BlockedTargetUnavailable => HandoffRenderDisclosure {
            parity_state: HandoffParityState::HandoffBlockedDisclosed,
            blocker_next_action: Some(HandoffBlockerNextAction::InspectHistoricalPacketOnly),
            fallback_behavior: HandoffFallbackBehavior::MetadataOnlyExit,
            needs_blocker_note: true,
            requires_cleared_preconditions: false,
            offers_open_live_target: false,
        },
        HandoffOutcome::BlockedByPolicy => HandoffRenderDisclosure {
            parity_state: HandoffParityState::HandoffBlockedDisclosed,
            blocker_next_action: Some(HandoffBlockerNextAction::InspectHistoricalPacketOnly),
            fallback_behavior: HandoffFallbackBehavior::MetadataOnlyExit,
            needs_blocker_note: true,
            requires_cleared_preconditions: false,
            offers_open_live_target: false,
        },
    }
}

/// One handoff binding: a preserved-snapshot object class handed off to its live target on one consumer
/// surface in one outcome for one preserved-snapshot profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetHandoffBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable preserved-snapshot-profile id (shared across surfaces that show the same profile).
    pub snapshot_profile_id: String,
    /// Human-readable preserved-snapshot-profile identity.
    pub snapshot_profile_label: String,
    /// Which preserved-snapshot object class this binding hands off.
    pub object_class: M5HistoricalReferenceObject,
    /// Which consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// The outcome of validating this handoff.
    pub outcome: HandoffOutcome,
    /// The controlled historical-side grammar presented (identical across surfaces for one profile).
    pub historical_grammar: HandoffHistoricalGrammar,
    /// The typed, versioned live-target handoff packet.
    pub handoff_request: LiveTargetHandoffRequest,
    /// Whether a cleared handoff is completed or a block is disclosed.
    pub parity_state: HandoffParityState,
    /// The discoverable action set allowed on this handoff surface.
    pub allowed_actions: Vec<HandoffAction>,
    /// The accessibility routes through which the handoff state, provenance, and open-live-target action can
    /// be discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The explicit blocker note; required and complete when the handoff blocks.
    pub blocker_note: Option<HandoffBlockerNote>,
    /// An explicit, reviewed authority / approval handoff that owns any actual elevation; absent by default,
    /// since this lane never bypasses approval, trust, or auth refresh.
    pub reviewed_authority_handoff: Option<ReviewedAuthorityHandoff>,
    /// The historical side stays mutation blocked. MUST be `true`.
    pub historical_side_mutation_blocked: bool,
    /// Guardrail: this surface reopens a live target without validating identity, trust, route, and
    /// authority. MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this surface widens authority beyond what a direct open would grant. MUST be `false`.
    pub widens_authority_beyond_direct_open: bool,
    /// Guardrail: this surface dead-ends when the target is unavailable instead of a metadata-only exit. MUST
    /// be `false`.
    pub dead_ends_when_target_unavailable: bool,
    /// Guardrail: this surface leaks a secret or ambient credential into the handoff packet. MUST be `false`.
    pub leaks_secret_or_ambient_credential: bool,
    /// Guardrail: this surface presents a snapshot as a current live object. MUST be `false`.
    pub presents_snapshot_as_current_live_object: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl LiveTargetHandoffBinding {
    /// Disclosures this binding must carry, derived from its outcome.
    pub const fn disclosure(&self) -> HandoffRenderDisclosure {
        resolve_handoff_render_disclosure(self.outcome)
    }

    /// Whether this binding blocks the live pivot.
    pub const fn is_blocked(&self) -> bool {
        self.outcome.is_blocked()
    }

    /// Whether every guardrail row-invariant holds (historical side mutation blocked, all guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.historical_side_mutation_blocked
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.widens_authority_beyond_direct_open
            && !self.dead_ends_when_target_unavailable
            && !self.leaks_secret_or_ambient_credential
            && !self.presents_snapshot_as_current_live_object
    }

    /// Whether the analysis-only base action set is present.
    pub fn has_analysis_only_base_actions(&self) -> bool {
        HandoffAction::ANALYSIS_ONLY_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no apply / sync affordance leaked in (structurally guaranteed by the closed action enum, but
    /// checked so the invariant is explicit).
    pub fn action_set_is_analysis_only(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                HandoffAction::InspectHistoricalPacket
                    | HandoffAction::ExportHandoffPacket
                    | HandoffAction::OpenCurrentLiveObject
            )
        })
    }

    /// Whether the open-current-live-object action is present exactly when the outcome (a cleared handoff)
    /// offers it.
    pub fn open_live_action_matches_outcome(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&HandoffAction::OpenCurrentLiveObject);
        offered == present
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5HistoricalReferenceAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5HistoricalReferenceAccessibilityRoute::ScreenReaderAnnounced)
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object_class.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetHandoffTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same historical grammar across surfaces.
    pub same_profile_same_historical_grammar_across_surfaces: bool,
    /// Every historical-role word is a frozen role token.
    pub historical_role_words_stay_in_frozen_vocabulary: bool,
    /// A historical side's mutation-blocked posture never masquerades as a live, writable, or current object.
    pub mutation_blocked_posture_never_masquerades_as_live: bool,
    /// Every handoff validates target identity, scope, route, trust, and auth before completing.
    pub every_handoff_validates_before_completing: bool,
    /// A completed handoff never widens authority beyond a direct open.
    pub handoff_never_widens_authority_beyond_direct_open: bool,
    /// A blocked handoff never dead-ends; the historical packet stays inspectable.
    pub blocked_handoff_never_dead_ends: bool,
    /// A snapshot is never presented as a current live object.
    pub snapshot_never_presented_as_current_live_object: bool,
    /// Auth / approval prerequisites are named as controlled tokens, never embedded secrets.
    pub auth_prerequisites_named_never_embedded_as_secrets: bool,
    /// Any actual auth / approval elevation is delegated to a reviewed authority handoff.
    pub actual_elevation_delegated_to_reviewed_authority_handoff: bool,
    /// Accessibility routes for the handoff state, provenance, and open-live-target action are present.
    pub accessibility_routes_present_for_state_provenance_and_open_live_target: bool,
    /// Blocking is disclosed across cleared, needs-prerequisite, target-unavailable, and policy outcomes.
    pub blocking_disclosed_across_outcomes: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl LiveTargetHandoffTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_historical_grammar_across_surfaces
            && self.historical_role_words_stay_in_frozen_vocabulary
            && self.mutation_blocked_posture_never_masquerades_as_live
            && self.every_handoff_validates_before_completing
            && self.handoff_never_widens_authority_beyond_direct_open
            && self.blocked_handoff_never_dead_ends
            && self.snapshot_never_presented_as_current_live_object
            && self.auth_prerequisites_named_never_embedded_as_secrets
            && self.actual_elevation_delegated_to_reviewed_authority_handoff
            && self.accessibility_routes_present_for_state_provenance_and_open_live_target
            && self.blocking_disclosed_across_outcomes
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetHandoffProjection {
    /// The shell / archive-viewer surface consumes the shared handoff packet.
    pub shell_consumes_handoff: bool,
    /// The help / docs surface consumes the shared handoff packet.
    pub help_docs_consumes_handoff: bool,
    /// The support bundle viewer consumes the shared handoff packet.
    pub support_consumes_handoff: bool,
    /// The review / incident surface consumes the shared handoff packet.
    pub review_incident_consumes_handoff: bool,
    /// The runbook-archive surface consumes the shared handoff packet.
    pub runbook_archive_consumes_handoff: bool,
    /// The release-center retirement snapshot page consumes the shared handoff packet.
    pub release_center_consumes_handoff: bool,
    /// The companion / export path consumes the shared handoff packet.
    pub companion_export_consumes_handoff: bool,
    /// The program-governance review consumes the shared handoff packet.
    pub program_governance_consumes_handoff: bool,
    /// The CLI / export path consumes the shared handoff packet.
    pub cli_export_consumes_handoff: bool,
    /// Every object class is handed off by two or more consumers.
    pub every_object_class_handed_off_by_two_or_more_consumers: bool,
    /// Historical grammar is identical for the same profile.
    pub historical_grammar_identical_for_same_profile: bool,
    /// Blocking is disclosed rather than hidden.
    pub blocking_disclosed_not_hidden: bool,
    /// Export maps a handoff row back to one historical-reference object class.
    pub handoff_maps_back_to_one_historical_reference_object: bool,
}

impl LiveTargetHandoffProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_handoff
            && self.help_docs_consumes_handoff
            && self.support_consumes_handoff
            && self.review_incident_consumes_handoff
            && self.runbook_archive_consumes_handoff
            && self.release_center_consumes_handoff
            && self.companion_export_consumes_handoff
            && self.program_governance_consumes_handoff
            && self.cli_export_consumes_handoff
            && self.every_object_class_handed_off_by_two_or_more_consumers
            && self.historical_grammar_identical_for_same_profile
            && self.blocking_disclosed_not_hidden
            && self.handoff_maps_back_to_one_historical_reference_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveTargetHandoffProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5LiveTargetHandoffPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LiveTargetHandoffPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Handoff bindings.
    pub handoff_bindings: Vec<LiveTargetHandoffBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<LiveTargetHandoffDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: LiveTargetHandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: LiveTargetHandoffProjection,
    /// Proof freshness block.
    pub proof_freshness: LiveTargetHandoffProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe live-target-handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveTargetHandoffPacket {
    /// Record kind; must equal [`M5_LIVE_TARGET_HANDOFF_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LIVE_TARGET_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Handoff bindings.
    pub handoff_bindings: Vec<LiveTargetHandoffBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<LiveTargetHandoffDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: LiveTargetHandoffTrustReview,
    /// Consumer projection block.
    pub consumer_projection: LiveTargetHandoffProjection,
    /// Proof freshness block.
    pub proof_freshness: LiveTargetHandoffProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LiveTargetHandoffPacket {
    /// Builds a live-target-handoff packet from stable-lane input.
    pub fn new(input: M5LiveTargetHandoffPacketInput) -> Self {
        Self {
            record_kind: M5_LIVE_TARGET_HANDOFF_RECORD_KIND.to_owned(),
            schema_version: M5_LIVE_TARGET_HANDOFF_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            handoff_bindings: input.handoff_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the live-target-handoff invariants.
    pub fn validate(&self) -> Vec<M5LiveTargetHandoffViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LIVE_TARGET_HANDOFF_RECORD_KIND {
            violations.push(M5LiveTargetHandoffViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LIVE_TARGET_HANDOFF_SCHEMA_VERSION {
            violations.push(M5LiveTargetHandoffViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LiveTargetHandoffViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5LiveTargetHandoffViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5LiveTargetHandoffViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5LiveTargetHandoffViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5LiveTargetHandoffViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5LiveTargetHandoffViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("handoff packet serializes"),
        ) {
            violations.push(M5LiveTargetHandoffViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("handoff packet serializes")
    }

    /// Deterministic matrix CSV, one row per handoff binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,consumer,outcome,route_class,trust_posture,requested_authority,direct_open_authority,parity_state\n",
        );
        for binding in &self.handoff_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.outcome.as_str(),
                binding.handoff_request.required_route_class.as_str(),
                binding.handoff_request.required_trust_posture.as_str(),
                binding.handoff_request.requested_authority_class.as_str(),
                binding.handoff_request.direct_open_authority_class.as_str(),
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked = self
            .handoff_bindings
            .iter()
            .filter(|binding| binding.is_blocked())
            .count();

        let mut out = String::new();
        out.push_str("# Live-Target Handoff Packets: One Validation Across Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Handoff bindings: {} ({} blocked)\n",
            self.handoff_bindings.len(),
            blocked
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Handoff bindings\n\n");
        for binding in &self.handoff_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, outcome `{}`, route `{}`, authority `{}`->`{}`, role `{}`\n",
                binding.snapshot_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.outcome.as_str(),
                binding.handoff_request.required_route_class.as_str(),
                binding.handoff_request.requested_authority_class.as_str(),
                binding.handoff_request.direct_open_authority_class.as_str(),
                binding.historical_grammar.historical_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in handoff export.
#[derive(Debug)]
pub enum M5LiveTargetHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LiveTargetHandoffViolation>),
}

impl fmt::Display for M5LiveTargetHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "handoff export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "handoff export failed validation: {tokens}")
            }
        }
    }
}

impl Error for M5LiveTargetHandoffArtifactError {}

/// Validation failures emitted by [`M5LiveTargetHandoffPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LiveTargetHandoffViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No handoff bindings are present.
    HandoffBindingsMissing,
    /// A handoff binding is incomplete.
    BindingIncomplete,
    /// A binding's historical grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's historical-role word is not a frozen role token.
    HistoricalRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its mutation-blocked posture.
    MutationBlockedPostureMissingForGateRole,
    /// A binding's handoff request is missing required fields.
    HandoffRequestIncomplete,
    /// A binding's parity state does not match its outcome.
    ParityStateMismatch,
    /// A binding's precondition check does not match its outcome.
    PreconditionOutcomeMismatch,
    /// A binding requested wider authority than a direct open would grant.
    AuthorityWidenedBeyondDirectOpen,
    /// A binding's declared no-widen flag disagrees with its authority classes.
    AuthorityWidenFlagInconsistent,
    /// A binding's fallback behavior does not match its outcome.
    FallbackBehaviorMismatch,
    /// Two surfaces show the same profile with different historical grammar.
    HandoffGrammarDriftAcrossSurfaces,
    /// A shared object class is not handed off by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A blocked binding is missing its explicit blocker note.
    BlockerNoteMissing,
    /// A blocker note's reason is not allowed for the outcome.
    BlockerReasonNotAllowedForOutcome,
    /// A blocker note's reason is not supported by a failed precondition.
    BlockerReasonNotSupportedByPrecondition,
    /// A blocker note's next action does not match the required next action.
    BlockerNextActionMismatch,
    /// A blocker note's fallback does not match the required fallback.
    BlockerFallbackMismatch,
    /// A blocker note is missing its explanation.
    BlockerExplanationMissing,
    /// A blocker note is missing its preserved-historical note.
    BlockerPreservedHistoricalNoteMissing,
    /// A blocker note is missing its next-action copy.
    BlockerNextActionLabelMissing,
    /// A cleared-handoff binding carries a blocker note it must not.
    UnexpectedBlockerNote,
    /// A binding is missing the analysis-only base action set.
    AnalysisOnlyBaseActionsMissing,
    /// A binding's action set is not analysis-only.
    ActionSetNotAnalysisOnly,
    /// A binding's open-current-live-object action does not match its outcome.
    OpenLiveActionOutcomeMismatch,
    /// A reviewed authority handoff is present but incomplete.
    ReviewedAuthorityHandoffIncomplete,
    /// A binding cannot discover its handoff state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's historical side is not mutation blocked.
    HistoricalSideNotMutationBlocked,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding widens authority beyond a direct open (guardrail form).
    WidensAuthorityBeyondDirectOpen,
    /// A binding dead-ends when the target is unavailable.
    DeadEndsWhenTargetUnavailable,
    /// A binding leaks a secret or ambient credential into the handoff packet.
    LeaksSecretOrAmbientCredential,
    /// A binding presents a snapshot as a current live object.
    PresentsSnapshotAsCurrentLiveObject,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every handoff outcome appears among the bindings.
    OutcomeCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5LiveTargetHandoffViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::HandoffBindingsMissing => "handoff_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::HistoricalRoleWordOutsideVocabulary => "historical_role_word_outside_vocabulary",
            Self::MutationBlockedPostureMissingForGateRole => {
                "mutation_blocked_posture_missing_for_gate_role"
            }
            Self::HandoffRequestIncomplete => "handoff_request_incomplete",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::PreconditionOutcomeMismatch => "precondition_outcome_mismatch",
            Self::AuthorityWidenedBeyondDirectOpen => "authority_widened_beyond_direct_open",
            Self::AuthorityWidenFlagInconsistent => "authority_widen_flag_inconsistent",
            Self::FallbackBehaviorMismatch => "fallback_behavior_mismatch",
            Self::HandoffGrammarDriftAcrossSurfaces => "handoff_grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::BlockerNoteMissing => "blocker_note_missing",
            Self::BlockerReasonNotAllowedForOutcome => "blocker_reason_not_allowed_for_outcome",
            Self::BlockerReasonNotSupportedByPrecondition => {
                "blocker_reason_not_supported_by_precondition"
            }
            Self::BlockerNextActionMismatch => "blocker_next_action_mismatch",
            Self::BlockerFallbackMismatch => "blocker_fallback_mismatch",
            Self::BlockerExplanationMissing => "blocker_explanation_missing",
            Self::BlockerPreservedHistoricalNoteMissing => {
                "blocker_preserved_historical_note_missing"
            }
            Self::BlockerNextActionLabelMissing => "blocker_next_action_label_missing",
            Self::UnexpectedBlockerNote => "unexpected_blocker_note",
            Self::AnalysisOnlyBaseActionsMissing => "analysis_only_base_actions_missing",
            Self::ActionSetNotAnalysisOnly => "action_set_not_analysis_only",
            Self::OpenLiveActionOutcomeMismatch => "open_live_action_outcome_mismatch",
            Self::ReviewedAuthorityHandoffIncomplete => "reviewed_authority_handoff_incomplete",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::HistoricalSideNotMutationBlocked => "historical_side_not_mutation_blocked",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::WidensAuthorityBeyondDirectOpen => "widens_authority_beyond_direct_open",
            Self::DeadEndsWhenTargetUnavailable => "dead_ends_when_target_unavailable",
            Self::LeaksSecretOrAmbientCredential => "leaks_secret_or_ambient_credential",
            Self::PresentsSnapshotAsCurrentLiveObject => "presents_snapshot_as_current_live_object",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::OutcomeCoverageMissing => "outcome_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable handoff export.
pub fn current_stable_m5_live_target_handoff_export(
) -> Result<M5LiveTargetHandoffPacket, M5LiveTargetHandoffArtifactError> {
    let packet: M5LiveTargetHandoffPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-live-target-handoff/support_export.json"
    )))
    .map_err(M5LiveTargetHandoffArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LiveTargetHandoffArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LiveTargetHandoffPacket,
    violations: &mut Vec<M5LiveTargetHandoffViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_LIVE_TARGET_HANDOFF_SCHEMA_REF,
        M5_LIVE_TARGET_HANDOFF_DOC_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    ];
    // The five object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5LiveTargetHandoffViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5LiveTargetHandoffPacket,
    violations: &mut Vec<M5LiveTargetHandoffViolation>,
) {
    if packet.handoff_bindings.is_empty() {
        violations.push(M5LiveTargetHandoffViolation::HandoffBindingsMissing);
        return;
    }

    // One vocabulary: the historical grammar must be identical for every binding that renders the same
    // preserved-snapshot profile.
    let mut profile_grammar: BTreeMap<&str, &HandoffHistoricalGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be handed off by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();
    let mut seen_outcomes: BTreeSet<HandoffOutcome> = BTreeSet::new();

    for binding in &packet.handoff_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.snapshot_profile_id.trim().is_empty()
            || binding.snapshot_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5LiveTargetHandoffViolation::BindingIncomplete);
        }
        if !binding.historical_grammar.all_present() {
            violations.push(M5LiveTargetHandoffViolation::GrammarFacetIncomplete);
        }
        if !binding
            .historical_grammar
            .historical_role_word_in_vocabulary()
        {
            violations.push(M5LiveTargetHandoffViolation::HistoricalRoleWordOutsideVocabulary);
        }
        if !binding
            .historical_grammar
            .mutation_blocked_posture_satisfied()
        {
            violations.push(M5LiveTargetHandoffViolation::MutationBlockedPostureMissingForGateRole);
        }

        if !binding.handoff_request.all_present() {
            violations.push(M5LiveTargetHandoffViolation::HandoffRequestIncomplete);
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5LiveTargetHandoffViolation::ParityStateMismatch);
        }

        // Precondition / outcome consistency: a cleared handoff clears every precondition and vice versa.
        let all_cleared = binding.handoff_request.precondition_check.all_cleared();
        if all_cleared != (binding.outcome == HandoffOutcome::HandoffCleared) {
            violations.push(M5LiveTargetHandoffViolation::PreconditionOutcomeMismatch);
        }

        // AC2: a handoff never widens authority beyond a direct open.
        if !binding.handoff_request.authority_not_widened() {
            violations.push(M5LiveTargetHandoffViolation::AuthorityWidenedBeyondDirectOpen);
        }
        // The declared guardrail flag must equal the actual widening: it is true exactly when the requested
        // authority is *not* within the direct-open ceiling.
        if binding.widens_authority_beyond_direct_open
            == binding.handoff_request.authority_not_widened()
        {
            violations.push(M5LiveTargetHandoffViolation::AuthorityWidenFlagInconsistent);
        }

        // The request's fallback behavior must match the outcome.
        if binding.handoff_request.fallback_behavior != disclosure.fallback_behavior {
            violations.push(M5LiveTargetHandoffViolation::FallbackBehaviorMismatch);
        }

        // Blocking disclosure.
        if disclosure.needs_blocker_note {
            match &binding.blocker_note {
                None => {
                    violations.push(M5LiveTargetHandoffViolation::BlockerNoteMissing);
                }
                Some(note) => {
                    if !binding
                        .outcome
                        .allowed_blocker_reasons()
                        .contains(&note.reason)
                    {
                        violations
                            .push(M5LiveTargetHandoffViolation::BlockerReasonNotAllowedForOutcome);
                    }
                    if !note
                        .reason
                        .supported_by(&binding.handoff_request.precondition_check)
                    {
                        violations.push(
                            M5LiveTargetHandoffViolation::BlockerReasonNotSupportedByPrecondition,
                        );
                    }
                    if Some(note.next_action) != disclosure.blocker_next_action {
                        violations.push(M5LiveTargetHandoffViolation::BlockerNextActionMismatch);
                    }
                    if note.fallback_behavior != disclosure.fallback_behavior {
                        violations.push(M5LiveTargetHandoffViolation::BlockerFallbackMismatch);
                    }
                    if note.explanation.trim().is_empty() {
                        violations.push(M5LiveTargetHandoffViolation::BlockerExplanationMissing);
                    }
                    if note.preserved_historical_note.trim().is_empty() {
                        violations.push(
                            M5LiveTargetHandoffViolation::BlockerPreservedHistoricalNoteMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations
                            .push(M5LiveTargetHandoffViolation::BlockerNextActionLabelMissing);
                    }
                }
            }
        } else if binding.blocker_note.is_some() {
            violations.push(M5LiveTargetHandoffViolation::UnexpectedBlockerNote);
        }

        // Reviewed authority handoff, when present, must be complete.
        if let Some(handoff) = &binding.reviewed_authority_handoff {
            if handoff.reviewed_path_id.trim().is_empty()
                || handoff.reviewed_path_label.trim().is_empty()
            {
                violations.push(M5LiveTargetHandoffViolation::ReviewedAuthorityHandoffIncomplete);
            }
        }

        // Action rules.
        if !binding.has_analysis_only_base_actions() {
            violations.push(M5LiveTargetHandoffViolation::AnalysisOnlyBaseActionsMissing);
        }
        if !binding.action_set_is_analysis_only() {
            violations.push(M5LiveTargetHandoffViolation::ActionSetNotAnalysisOnly);
        }
        if !binding.open_live_action_matches_outcome() {
            violations.push(M5LiveTargetHandoffViolation::OpenLiveActionOutcomeMismatch);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5LiveTargetHandoffViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.historical_side_mutation_blocked {
            violations.push(M5LiveTargetHandoffViolation::HistoricalSideNotMutationBlocked);
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5LiveTargetHandoffViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.widens_authority_beyond_direct_open {
            violations.push(M5LiveTargetHandoffViolation::WidensAuthorityBeyondDirectOpen);
        }
        if binding.dead_ends_when_target_unavailable {
            violations.push(M5LiveTargetHandoffViolation::DeadEndsWhenTargetUnavailable);
        }
        if binding.leaks_secret_or_ambient_credential {
            violations.push(M5LiveTargetHandoffViolation::LeaksSecretOrAmbientCredential);
        }
        if binding.presents_snapshot_as_current_live_object {
            violations.push(M5LiveTargetHandoffViolation::PresentsSnapshotAsCurrentLiveObject);
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5LiveTargetHandoffViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.snapshot_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.snapshot_profile_id.as_str(),
                    &binding.historical_grammar,
                );
            }
            Some(existing) => {
                if **existing != binding.historical_grammar && !drift_reported {
                    violations
                        .push(M5LiveTargetHandoffViolation::HandoffGrammarDriftAcrossSurfaces);
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object_class);
        seen_outcomes.insert(binding.outcome);
    }

    // Coverage: every consumer surface, object class, and outcome must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5LiveTargetHandoffViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5LiveTargetHandoffViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for outcome in HandoffOutcome::ALL {
        if !seen_outcomes.contains(&outcome) {
            violations.push(M5LiveTargetHandoffViolation::OutcomeCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be handed off by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5LiveTargetHandoffViolation::ObjectClassReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
