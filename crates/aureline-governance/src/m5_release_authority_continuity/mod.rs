//! Typed register of owner/backup/roster/runbook continuity per protected M5 release lane.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix records, per asset lane, the emergency signing/registry/security authority as one coarse
//! `EmergencyAuthority` block (a primary owner, a backup list, a signer quorum), and the
//! [`m5_critical_upstream_health`](crate::m5_critical_upstream_health) register makes each critical
//! *dependency* inspectable. Neither makes each protected *authority lane* — the release-signing,
//! promotion-approval, registry-moderation, and security-response operations a protected M5 family
//! leans on — inspectable as a durable continuity record: who the named primary owner is, whether a
//! backup owner exists so the lane is not a single-person system, whether the signer roster /
//! promotion quorum / moderation-operator / security-responder roster meets its threshold, whether
//! split (two-person) authority is enforced where required, whether a current backup runbook
//! exists, and — when the lane is critical or already single-owner — whether the shiproom has been
//! told.
//!
//! This module is that authority-continuity layer. For every protected authority lane a protected
//! M5 family runs it records one [`AuthorityContinuityRecord`] that states, in one copy-safe
//! record:
//!
//! - the **owner coverage** ([`OwnerCoverage`]): a named primary owner, so a release/security lane
//!   is never quietly ownerless;
//! - the **backup coverage** ([`BackupCoverage`]): at least one named backup owner, so a protected
//!   lane is never a single-person system by accident — the headline guardrail;
//! - the **roster quorum** ([`RosterProfile`]): the signer roster, promotion-approval quorum,
//!   moderation-operator roster, or security-responder roster, and whether it meets its threshold;
//! - the **split authority** ([`SplitAuthority`]): two-person / split control enforced for critical
//!   lanes;
//! - the **runbook coverage** ([`RunbookCoverage`]): a current backup runbook, a due-for-review
//!   reminder, a stale runbook, or a missing one;
//! - the **shiproom escalation** ([`ShiproomEscalation`]), required for any critical or
//!   single-owner lane.
//!
//! Each record also carries a [`scan_posture`](AuthorityContinuityRecord::scan_posture) (what the
//! continuity scan found) and a [`surface_posture`](AuthorityContinuityRecord::surface_posture)
//! (what the governance-dashboard/promotion-packet surface shows). The two **must agree**: a record
//! may never show a clean surface over a scan that found gaps, so a green authority card can never
//! mask a lane that is single-owner, under quorum, or without a current runbook.
//!
//! A record is [`ContinuityState::Cleared`] only when the lane has a named primary and backup
//! owner, its roster meets quorum, the runbook is current (or only due for review), split authority
//! is enforced where required, any required shiproom escalation is raised, the proof is fresh, and
//! the owner signed. Otherwise it narrows on the *specific* axis that thinned out — an owner gap, a
//! backup (single-owner) gap, a quorum gap, a runbook gap, an authority/escalation gap, or stale
//! proof — never collapsing to one global flag. A narrowed record drops its
//! [`AuthorityContinuityRecord::effective_label`] below the launch cutline and may never publish an
//! effective label wider than the one it declares.
//!
//! The [`ContinuityRule`] set names the closed conditions that gate promotion. An *inherited*
//! narrowing — a subject whose declared label already sits below the cutline, or a gap held by an
//! unexpired waiver — is gated upstream and does not itself hold promotion; a *continuity* failure
//! on a subject whose declared label is still at or above the cutline holds promotion through a
//! shiproom stop rule, recorded in [`ReleaseAuthorityContinuityRegister::publication`] — a
//! single-owner, under-quorum, or runbook-less protected lane cannot widen a stable claim without
//! coverage. The cross-cutting [`ScanSurfaceParity`] block summarizes scan/surface agreement over
//! every subject.
//!
//! The register is checked in at `artifacts/governance/m5-release-authority-continuity.json` and
//! embedded here, so this typed consumer and the CI gate agree on every record without a cargo
//! build in CI. The model is metadata-only: every field is a typed state, a boolean flag, a small
//! count, a label, or an opaque ref. It carries no credential bodies, raw provider payloads, signer
//! identities beyond opaque role refs, or signatures. Date arithmetic (recomputing proof, runbook,
//! and waiver freshness against an `as_of` date) lives in the CI gate and the integration test;
//! this model enforces the invariants that hold regardless of the clock: scan/surface parity, the
//! no-widening ceiling, control/fact consistency, reason/state coherence, summary agreement, and
//! the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_RELEASE_AUTHORITY_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_RELEASE_AUTHORITY_CONTINUITY_RECORD_KIND: &str =
    "m5_release_authority_continuity_register";

/// Repo-relative path to the checked-in register.
pub const M5_RELEASE_AUTHORITY_CONTINUITY_PATH: &str =
    "artifacts/governance/m5-release-authority-continuity.json";

/// Embedded checked-in register JSON.
pub const M5_RELEASE_AUTHORITY_CONTINUITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-release-authority-continuity.json"
));

/// The kind of protected authority lane a record governs.
///
/// The same continuity truth is published for release signing, promotion approval, registry
/// moderation, and security response — so a single-owner security-response lane cannot hide behind
/// a healthy release-signing lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityLane {
    /// Release-artifact signing.
    ReleaseSigning,
    /// Promotion-gate approval quorum.
    PromotionApproval,
    /// Package/extension registry moderation and emergency unpublish.
    RegistryModeration,
    /// Security advisory, CVE/GHSA, and revocation response.
    SecurityResponse,
}

impl AuthorityLane {
    /// Every authority lane, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReleaseSigning,
        Self::PromotionApproval,
        Self::RegistryModeration,
        Self::SecurityResponse,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseSigning => "release_signing",
            Self::PromotionApproval => "promotion_approval",
            Self::RegistryModeration => "registry_moderation",
            Self::SecurityResponse => "security_response",
        }
    }

    /// The roster kind this lane is staffed by.
    pub const fn expected_roster_kind(self) -> RosterKind {
        match self {
            Self::ReleaseSigning => RosterKind::SignerRoster,
            Self::PromotionApproval => RosterKind::PromotionApprovers,
            Self::RegistryModeration => RosterKind::ModerationOperators,
            Self::SecurityResponse => RosterKind::SecurityResponders,
        }
    }
}

/// The criticality grade a lane carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneCriticality {
    /// Routine: no split-authority or escalation requirement on its own.
    Routine,
    /// Elevated: a risk is present but not release-blocking on its own.
    Elevated,
    /// Critical: split authority and escalation are required.
    Critical,
    /// Blocking: a continuity failure would block the family.
    Blocking,
}

impl LaneCriticality {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Routine,
        Self::Elevated,
        Self::Critical,
        Self::Blocking,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Routine => "routine",
            Self::Elevated => "elevated",
            Self::Critical => "critical",
            Self::Blocking => "blocking",
        }
    }

    /// True when the grade is critical (`critical`/`blocking`): split authority is required and an
    /// escalation must be raised.
    pub fn is_critical(self) -> bool {
        matches!(self, Self::Critical | Self::Blocking)
    }
}

/// A continuity control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Owner coverage: a named primary owner.
    OwnerCoverage,
    /// Backup coverage: at least one named backup owner (not a single-person system).
    BackupCoverage,
    /// Roster quorum: the signer/approval/operator/responder roster meets its threshold.
    RosterQuorum,
    /// Runbook coverage: a current backup runbook.
    RunbookCoverage,
    /// Authority continuity: split authority and the shiproom escalation where required.
    AuthorityContinuity,
    /// Scan/surface parity: the continuity scan and the governance surface agree.
    ScanSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 6] = [
        Self::OwnerCoverage,
        Self::BackupCoverage,
        Self::RosterQuorum,
        Self::RunbookCoverage,
        Self::AuthorityContinuity,
        Self::ScanSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwnerCoverage => "owner_coverage",
            Self::BackupCoverage => "backup_coverage",
            Self::RosterQuorum => "roster_quorum",
            Self::RunbookCoverage => "runbook_coverage",
            Self::AuthorityContinuity => "authority_continuity",
            Self::ScanSurfaceParity => "scan_surface_parity",
        }
    }
}

/// The kind of roster that staffs a lane's protected actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RosterKind {
    /// Release signers.
    SignerRoster,
    /// Promotion-gate approvers.
    PromotionApprovers,
    /// Registry-moderation operators.
    ModerationOperators,
    /// Security responders.
    SecurityResponders,
}

impl RosterKind {
    /// Every roster kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SignerRoster,
        Self::PromotionApprovers,
        Self::ModerationOperators,
        Self::SecurityResponders,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignerRoster => "signer_roster",
            Self::PromotionApprovers => "promotion_approvers",
            Self::ModerationOperators => "moderation_operators",
            Self::SecurityResponders => "security_responders",
        }
    }
}

/// Whether the lane has a named primary owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerState {
    /// The lane has a named primary owner.
    Assigned,
    /// The lane has no named primary owner.
    Vacant,
}

impl OwnerState {
    /// Every owner state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Assigned, Self::Vacant];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Assigned => "assigned",
            Self::Vacant => "vacant",
        }
    }
}

/// Whether the lane has backup owner coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupState {
    /// At least one named backup owner can act if the primary is unavailable.
    Covered,
    /// No backup owner: the lane is effectively a single-person system.
    SingleOwner,
}

impl BackupState {
    /// Every backup state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Covered, Self::SingleOwner];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::SingleOwner => "single_owner",
        }
    }

    /// True when the lane is a single-person system.
    pub fn is_single_owner(self) -> bool {
        matches!(self, Self::SingleOwner)
    }
}

/// Whether a roster meets its required quorum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuorumState {
    /// The roster meets its required quorum.
    Met,
    /// The roster is below its required quorum.
    BelowThreshold,
}

impl QuorumState {
    /// Every quorum state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Met, Self::BelowThreshold];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Met => "met",
            Self::BelowThreshold => "below_threshold",
        }
    }

    /// True when the roster is below its required quorum.
    pub fn is_below(self) -> bool {
        matches!(self, Self::BelowThreshold)
    }
}

/// State of a lane's split (two-person) authority requirement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SplitAuthorityState {
    /// Split authority is enforced (two independent authorities).
    Satisfied,
    /// Split authority is required but not enforced.
    Unmet,
    /// Split authority is not required for this lane.
    NotRequired,
}

impl SplitAuthorityState {
    /// Every split-authority state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Satisfied, Self::Unmet, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unmet => "unmet",
            Self::NotRequired => "not_required",
        }
    }
}

/// State of a lane's backup runbook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookState {
    /// The runbook is current.
    Current,
    /// The next runbook review is coming due (a reminder, not a gap).
    DueForReview,
    /// The runbook review is overdue.
    Stale,
    /// No runbook is captured.
    Missing,
}

impl RunbookState {
    /// Every runbook state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForReview,
        Self::Stale,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForReview => "due_for_review",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }

    /// True when the runbook review is overdue.
    pub fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }

    /// True when no runbook is captured.
    pub fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// State of a lane's shiproom/governance escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationState {
    /// The escalation has been raised.
    Raised,
    /// An escalation is required but still pending.
    Pending,
    /// An escalation is not required for this lane.
    NotRequired,
}

impl EscalationState {
    /// Every escalation state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Raised, Self::Pending, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raised => "raised",
            Self::Pending => "pending",
            Self::NotRequired => "not_required",
        }
    }
}

/// The posture a scan or a surface reports for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    /// No continuity gap found.
    Clear,
    /// One or more continuity gaps found.
    GapsFound,
}

impl Posture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::GapsFound];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::GapsFound => "gaps_found",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control holds for this lane.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this lane.
    NotApplicable,
}

impl ControlState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The state a record earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityState {
    /// Owner, backup, roster, runbook, authority, and proof all hold.
    Cleared,
    /// The lane has no named primary owner.
    NarrowedOwner,
    /// The lane has no backup owner (effectively single-person).
    NarrowedBackup,
    /// The roster is below its required quorum.
    NarrowedQuorum,
    /// The backup runbook is stale or missing.
    NarrowedRunbook,
    /// Split authority or the shiproom escalation is missing where required.
    NarrowedAuthority,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The lane is withdrawn.
    Withdrawn,
}

impl ContinuityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Cleared,
        Self::NarrowedOwner,
        Self::NarrowedBackup,
        Self::NarrowedQuorum,
        Self::NarrowedRunbook,
        Self::NarrowedAuthority,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::NarrowedOwner => "narrowed_owner",
            Self::NarrowedBackup => "narrowed_backup",
            Self::NarrowedQuorum => "narrowed_quorum",
            Self::NarrowedRunbook => "narrowed_runbook",
            Self::NarrowedAuthority => "narrowed_authority",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not cleared, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Cleared | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by a [`ContinuityRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityReason {
    /// The lane has no named primary owner.
    PrimaryOwnerVacant,
    /// The lane has no backup owner: it is a single-person system.
    BackupOwnerMissing,
    /// The roster is below its required quorum.
    RosterQuorumBelowThreshold,
    /// The backup runbook review is overdue.
    RunbookStale,
    /// No backup runbook is captured.
    RunbookMissing,
    /// Split authority is required but not enforced.
    SplitAuthorityUnmet,
    /// A critical or single-owner lane has not been escalated to the shiproom.
    ShiproomEscalationMissing,
    /// The continuity proof packet aged past its freshness SLO.
    ContinuityProofStale,
    /// No continuity proof packet is captured.
    ContinuityProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl ContinuityReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::PrimaryOwnerVacant,
        Self::BackupOwnerMissing,
        Self::RosterQuorumBelowThreshold,
        Self::RunbookStale,
        Self::RunbookMissing,
        Self::SplitAuthorityUnmet,
        Self::ShiproomEscalationMissing,
        Self::ContinuityProofStale,
        Self::ContinuityProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryOwnerVacant => "primary_owner_vacant",
            Self::BackupOwnerMissing => "backup_owner_missing",
            Self::RosterQuorumBelowThreshold => "roster_quorum_below_threshold",
            Self::RunbookStale => "runbook_stale",
            Self::RunbookMissing => "runbook_missing",
            Self::SplitAuthorityUnmet => "split_authority_unmet",
            Self::ShiproomEscalationMissing => "shiproom_escalation_missing",
            Self::ContinuityProofStale => "continuity_proof_stale",
            Self::ContinuityProofMissing => "continuity_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active. The single-person
    /// (backup) guardrail is the worst, then a vacant primary owner, then split-authority /
    /// escalation, then quorum, then runbook, and finally the evidence-staleness axis.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            ContinuityState::NarrowedBackup => 0,
            ContinuityState::NarrowedOwner => 1,
            ContinuityState::NarrowedAuthority => 2,
            ContinuityState::NarrowedQuorum => 3,
            ContinuityState::NarrowedRunbook => 4,
            _ => 5,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> ContinuityState {
        match self {
            Self::PrimaryOwnerVacant => ContinuityState::NarrowedOwner,
            Self::BackupOwnerMissing => ContinuityState::NarrowedBackup,
            Self::RosterQuorumBelowThreshold => ContinuityState::NarrowedQuorum,
            Self::RunbookStale | Self::RunbookMissing => ContinuityState::NarrowedRunbook,
            Self::SplitAuthorityUnmet | Self::ShiproomEscalationMissing => {
                ContinuityState::NarrowedAuthority
            }
            Self::ContinuityProofStale
            | Self::ContinuityProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ContinuityState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::PrimaryOwnerVacant => ControlDimension::OwnerCoverage,
            Self::BackupOwnerMissing => ControlDimension::BackupCoverage,
            Self::RosterQuorumBelowThreshold => ControlDimension::RosterQuorum,
            Self::RunbookStale | Self::RunbookMissing => ControlDimension::RunbookCoverage,
            Self::SplitAuthorityUnmet | Self::ShiproomEscalationMissing => {
                ControlDimension::AuthorityContinuity
            }
            Self::ContinuityProofStale
            | Self::ContinuityProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ScanSurfaceParity,
        }
    }
}

/// An action a [`ContinuityRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Assign a named primary owner.
    AssignPrimaryOwner,
    /// Assign a named backup owner.
    AssignBackupOwner,
    /// Staff the roster to its required quorum.
    StaffRosterToQuorum,
    /// Refresh the backup runbook.
    RefreshContinuityRunbook,
    /// Enforce split (two-person) authority.
    EnforceSplitAuthority,
    /// Raise the shiproom escalation.
    RaiseShiproomEscalation,
    /// Refresh the continuity proof packet.
    RefreshContinuityProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl ContinuityAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::HoldPromotion,
        Self::AssignPrimaryOwner,
        Self::AssignBackupOwner,
        Self::StaffRosterToQuorum,
        Self::RefreshContinuityRunbook,
        Self::EnforceSplitAuthority,
        Self::RaiseShiproomEscalation,
        Self::RefreshContinuityProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::AssignPrimaryOwner => "assign_primary_owner",
            Self::AssignBackupOwner => "assign_backup_owner",
            Self::StaffRosterToQuorum => "staff_roster_to_quorum",
            Self::RefreshContinuityRunbook => "refresh_continuity_runbook",
            Self::EnforceSplitAuthority => "enforce_split_authority",
            Self::RaiseShiproomEscalation => "raise_shiproom_escalation",
            Self::RefreshContinuityProof => "refresh_continuity_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No continuity stop rule fires; promotion may proceed.
    Proceed,
    /// A continuity stop rule fires; hold promotion.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// Primary-owner coverage for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerCoverage {
    /// Owner state.
    pub owner_state: OwnerState,
    /// Named primary owning team or role (empty when vacant).
    pub primary_owner_ref: String,
    /// Reference to the ownership assignment record.
    pub assignment_ref: String,
}

impl OwnerCoverage {
    /// True when the lane has no named primary owner.
    pub fn is_vacant(&self) -> bool {
        self.owner_state == OwnerState::Vacant
    }
}

/// Backup-owner coverage for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BackupCoverage {
    /// Backup state.
    pub backup_state: BackupState,
    /// Number of named backup owners.
    pub backup_owner_count: u32,
    /// Reference to the backup-owner roster.
    pub roster_ref: String,
}

impl BackupCoverage {
    /// True when the lane is a single-person system.
    pub fn is_single_owner(&self) -> bool {
        self.backup_state.is_single_owner()
    }
}

/// The signer/approval/operator/responder roster and its quorum for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RosterProfile {
    /// The kind of roster (must match the lane).
    pub roster_kind: RosterKind,
    /// Quorum state.
    pub quorum_state: QuorumState,
    /// Distinct humans required to authorize a protected action.
    pub required_quorum: u32,
    /// Distinct humans actually available on the roster.
    pub available_members: u32,
    /// Reference to the roster/quorum profile.
    pub roster_ref: String,
}

impl RosterProfile {
    /// True when the available members meet the required quorum.
    pub fn is_met(&self) -> bool {
        self.available_members >= self.required_quorum
    }

    /// True when the roster is below its required quorum.
    pub fn is_below(&self) -> bool {
        self.quorum_state.is_below()
    }
}

/// The split (two-person) authority requirement for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SplitAuthority {
    /// Split-authority state.
    pub split_state: SplitAuthorityState,
    /// True when split authority is required for this lane.
    pub required: bool,
    /// Number of independent authorities that must concur.
    pub distinct_authorities: u32,
    /// Reference to the split-authority policy.
    pub policy_ref: String,
}

/// The backup runbook coverage for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunbookCoverage {
    /// Runbook state.
    pub runbook_state: RunbookState,
    /// Review interval in days.
    pub review_interval_days: u32,
    /// Next review due date (`null` when no runbook is captured).
    pub next_review_due: Option<String>,
    /// Reference to the backup runbook.
    pub runbook_ref: String,
}

/// The shiproom/governance escalation for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShiproomEscalation {
    /// Escalation state.
    pub escalation_state: EscalationState,
    /// True when an escalation is required for this lane.
    pub required: bool,
    /// Reference to the shiproom escalation queue.
    pub shiproom_ref: String,
    /// Reference to the governance review record.
    pub governance_ref: String,
}

/// One continuity control binding on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One release-authority continuity record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityContinuityRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this lane serves.
    pub family: M5Family,
    /// The kind of authority lane.
    pub lane: AuthorityLane,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// The criticality grade the lane carries.
    pub criticality: LaneCriticality,
    /// Primary-owner coverage.
    pub owner_coverage: OwnerCoverage,
    /// Backup-owner coverage.
    pub backup_coverage: BackupCoverage,
    /// Roster and quorum.
    pub roster: RosterProfile,
    /// Split-authority requirement.
    pub split_authority: SplitAuthority,
    /// Backup runbook coverage.
    pub runbook: RunbookCoverage,
    /// Shiproom/governance escalation.
    pub escalation: ShiproomEscalation,
    /// Per-dimension control bindings.
    pub controls: Vec<ContinuityControl>,
    /// What the continuity scan found.
    pub scan_posture: Posture,
    /// What the governance-dashboard/promotion-packet surface shows.
    pub surface_posture: Posture,
    /// Reference to the continuity scan.
    pub scan_ref: String,
    /// Reference to the governance surface.
    pub surface_ref: String,
    /// Proof packet grounding the record.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// State earned after narrowing.
    pub continuity_state: ContinuityState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ContinuityReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, service-health, release-center, support).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl AuthorityContinuityRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(ContinuityReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: ContinuityReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a cleared state.
    pub fn is_cleared(&self) -> bool {
        self.continuity_state == ContinuityState::Cleared
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when the lane is critical (`critical`/`blocking`).
    pub fn is_critical(&self) -> bool {
        self.criticality.is_critical()
    }

    /// True when the lane has no named primary owner.
    pub fn owner_vacant(&self) -> bool {
        self.owner_coverage.is_vacant()
    }

    /// True when the lane is a single-person system.
    pub fn is_single_owner(&self) -> bool {
        self.backup_coverage.is_single_owner()
    }

    /// True when the roster is below its required quorum.
    pub fn quorum_below(&self) -> bool {
        self.roster.is_below()
    }

    /// True when the runbook is stale or missing.
    pub fn runbook_degraded(&self) -> bool {
        self.runbook.runbook_state.is_stale() || self.runbook.runbook_state.is_missing()
    }

    /// True when this lane requires split (two-person) authority.
    pub fn requires_split_authority(&self) -> bool {
        self.is_critical()
    }

    /// True when this lane requires a shiproom escalation: critical or single-owner.
    pub fn requires_escalation(&self) -> bool {
        self.is_critical() || self.is_single_owner()
    }

    /// True when a required split authority is still unmet.
    pub fn split_authority_unmet(&self) -> bool {
        self.requires_split_authority()
            && self.split_authority.split_state == SplitAuthorityState::Unmet
    }

    /// True when a required shiproom escalation is still pending.
    pub fn escalation_missing(&self) -> bool {
        self.requires_escalation() && self.escalation.escalation_state == EscalationState::Pending
    }

    /// True when any structural continuity gap (other than proof/sign-off) is present.
    pub fn has_continuity_gap(&self) -> bool {
        self.owner_vacant()
            || self.is_single_owner()
            || self.quorum_below()
            || self.runbook_degraded()
            || self.split_authority_unmet()
            || self.escalation_missing()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        match dimension {
            ControlDimension::OwnerCoverage => {
                if self.owner_vacant() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::BackupCoverage => {
                if self.is_single_owner() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::RosterQuorum => {
                if self.quorum_below() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::RunbookCoverage => {
                if self.runbook_degraded() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::AuthorityContinuity => {
                if self.split_authority_unmet() || self.escalation_missing() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::ScanSurfaceParity => {
                if self.scan_posture != self.surface_posture {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
        }
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> ContinuityState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return ContinuityState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => ContinuityState::Cleared,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            ContinuityState::Cleared => self.declared_label,
            ContinuityState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the subject below the cutline: take the less-supported of the
                // declared label and beta.
                if self.declared_label.rank() <= LifecycleLabel::Beta.rank() {
                    self.declared_label
                } else {
                    LifecycleLabel::Beta
                }
            }
        }
    }

    /// The posture implied by the record's state: gaps found iff narrowed.
    pub fn computed_posture(&self) -> Posture {
        if self.continuity_state.is_narrowed() {
            Posture::GapsFound
        } else {
            Posture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by a
    /// continuity gap, declaring a label at or above the cutline, and not held by an unexpired
    /// waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.continuity_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the scan and the surface agree.
    pub fn scan_surface_agree(&self) -> bool {
        self.scan_posture == self.surface_posture
    }
}

/// A closed stop-rule that gates promotion on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: ContinuityReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: ContinuityAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this register binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Signer-roster and signing-quorum register.
    pub signer_roster_ref: String,
    /// Promotion-quorum / promotion-gate register.
    pub promotion_quorum_ref: String,
    /// Registry-moderation operator register.
    pub registry_moderation_ref: String,
    /// Security-response responder register.
    pub security_response_ref: String,
    /// Backup-runbook / continuity-runbook register.
    pub runbook_register_ref: String,
    /// Open/local-boundary and upstream-durability matrix.
    pub durability_matrix_ref: String,
    /// Shiproom gate register.
    pub shiproom_register_ref: String,
    /// Canonical M5 evidence index.
    pub m5_evidence_index_ref: String,
}

/// Promotion verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable promotion-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending record ids.
    pub blocking_record_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Cross-cutting scan/surface parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanSurfaceParity {
    /// Stable parity-gate id.
    pub parity_gate: String,
    /// Total subjects.
    pub subjects_total: usize,
    /// Subjects whose scan and surface agree.
    pub subjects_in_agreement: usize,
    /// Subjects whose scan and surface disagree.
    pub subjects_in_disagreement: usize,
    /// Subjects whose surface reports gaps found.
    pub subjects_with_gaps: usize,
    /// True when every subject's scan and surface agree.
    pub all_subjects_agree: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuitySummary {
    /// Total records.
    pub total_records: usize,
    /// Cleared records.
    pub records_cleared: usize,
    /// Narrowed records.
    pub records_narrowed: usize,
    /// Records in the `cleared` state.
    pub state_cleared: usize,
    /// Records in the `narrowed_owner` state.
    pub state_narrowed_owner: usize,
    /// Records in the `narrowed_backup` state.
    pub state_narrowed_backup: usize,
    /// Records in the `narrowed_quorum` state.
    pub state_narrowed_quorum: usize,
    /// Records in the `narrowed_runbook` state.
    pub state_narrowed_runbook: usize,
    /// Records in the `narrowed_authority` state.
    pub state_narrowed_authority: usize,
    /// Records in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Records in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Release-blocking records.
    pub release_blocking_total: usize,
    /// Release-blocking records that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Records held by an active waiver.
    pub records_on_active_waiver: usize,
    /// Records carrying a primary-owner gap.
    pub owner_gaps: usize,
    /// Records carrying a backup (single-owner) gap.
    pub backup_gaps: usize,
    /// Records carrying a roster-quorum gap.
    pub quorum_gaps: usize,
    /// Records carrying a runbook gap.
    pub runbook_gaps: usize,
    /// Records carrying a split-authority/escalation gap.
    pub authority_gaps: usize,
    /// Critical-criticality records.
    pub critical_total: usize,
    /// Single-owner records.
    pub single_owner_total: usize,
    /// Records that require a shiproom escalation.
    pub escalations_required: usize,
    /// Records whose escalation has been raised.
    pub escalations_raised: usize,
    /// Records whose split authority is enforced.
    pub split_authority_enforced: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of release-authority continuity records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseAuthorityContinuityRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register id.
    pub register_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the register was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub continuity_cutline: ContinuityCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed authority-lane vocabulary.
    pub authority_lanes: Vec<AuthorityLane>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed criticality vocabulary.
    pub criticalities: Vec<LaneCriticality>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed roster-kind vocabulary.
    pub roster_kinds: Vec<RosterKind>,
    /// Closed owner-state vocabulary.
    pub owner_states: Vec<OwnerState>,
    /// Closed backup-state vocabulary.
    pub backup_states: Vec<BackupState>,
    /// Closed quorum-state vocabulary.
    pub quorum_states: Vec<QuorumState>,
    /// Closed split-authority-state vocabulary.
    pub split_authority_states: Vec<SplitAuthorityState>,
    /// Closed runbook-state vocabulary.
    pub runbook_states: Vec<RunbookState>,
    /// Closed escalation-state vocabulary.
    pub escalation_states: Vec<EscalationState>,
    /// Closed posture vocabulary.
    pub postures: Vec<Posture>,
    /// Closed continuity-state vocabulary.
    pub continuity_states: Vec<ContinuityState>,
    /// Closed continuity-reason vocabulary.
    pub continuity_reasons: Vec<ContinuityReason>,
    /// Closed continuity-action vocabulary.
    pub continuity_actions: Vec<ContinuityAction>,
    /// Stop rules.
    pub rules: Vec<ContinuityRule>,
    /// Per-lane records.
    pub records: Vec<AuthorityContinuityRecord>,
    /// Cross-cutting scan/surface parity summary.
    pub scan_surface_parity: ScanSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: ContinuitySummary,
}

impl ReleaseAuthorityContinuityRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&AuthorityContinuityRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the cleared records.
    pub fn records_cleared(&self) -> Vec<&AuthorityContinuityRecord> {
        self.records.iter().filter(|r| r.is_cleared()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&AuthorityContinuityRecord> {
        self.records
            .iter()
            .filter(|r| r.continuity_state.is_narrowed())
            .collect()
    }

    /// Returns the records of a given authority lane.
    pub fn records_of_lane(&self, lane: AuthorityLane) -> Vec<&AuthorityContinuityRecord> {
        self.records.iter().filter(|r| r.lane == lane).collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: ContinuityReason) -> Option<&ContinuityRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a promotion-holding record
    /// carries its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_promotion {
                continue;
            }
            let fires = self.records.iter().any(|r| {
                r.holds_promotion()
                    && r.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&r.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending record ids: promotion-holding records carrying a reason watched by
    /// a firing blocking rule.
    pub fn computed_blocking_record_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.records {
            if !r.holds_promotion() {
                continue;
            }
            let blocked = r.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_promotion && rule.applies_to_labels.contains(&r.declared_label)
                })
            });
            if blocked {
                ids.insert(r.record_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the promotion decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_record_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the cross-cutting scan/surface parity summary.
    pub fn computed_scan_surface_parity(&self) -> ScanSurfaceParity {
        ScanSurfaceParity {
            parity_gate: self.scan_surface_parity.parity_gate.clone(),
            subjects_total: self.records.len(),
            subjects_in_agreement: self
                .records
                .iter()
                .filter(|r| r.scan_surface_agree())
                .count(),
            subjects_in_disagreement: self
                .records
                .iter()
                .filter(|r| !r.scan_surface_agree())
                .count(),
            subjects_with_gaps: self
                .records
                .iter()
                .filter(|r| r.surface_posture == Posture::GapsFound)
                .count(),
            all_subjects_agree: self.records.iter().all(|r| r.scan_surface_agree()),
            rationale: self.scan_surface_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> ContinuitySummary {
        let count_state = |state: ContinuityState| {
            self.records
                .iter()
                .filter(|r| r.continuity_state == state)
                .count()
        };
        ContinuitySummary {
            total_records: self.records.len(),
            records_cleared: self.records_cleared().len(),
            records_narrowed: self.records_narrowed().len(),
            state_cleared: count_state(ContinuityState::Cleared),
            state_narrowed_owner: count_state(ContinuityState::NarrowedOwner),
            state_narrowed_backup: count_state(ContinuityState::NarrowedBackup),
            state_narrowed_quorum: count_state(ContinuityState::NarrowedQuorum),
            state_narrowed_runbook: count_state(ContinuityState::NarrowedRunbook),
            state_narrowed_authority: count_state(ContinuityState::NarrowedAuthority),
            state_narrowed_stale: count_state(ContinuityState::NarrowedStale),
            state_withdrawn: count_state(ContinuityState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.continuity_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            owner_gaps: self.records.iter().filter(|r| r.owner_vacant()).count(),
            backup_gaps: self.records.iter().filter(|r| r.is_single_owner()).count(),
            quorum_gaps: self.records.iter().filter(|r| r.quorum_below()).count(),
            runbook_gaps: self.records.iter().filter(|r| r.runbook_degraded()).count(),
            authority_gaps: self
                .records
                .iter()
                .filter(|r| r.split_authority_unmet() || r.escalation_missing())
                .count(),
            critical_total: self.records.iter().filter(|r| r.is_critical()).count(),
            single_owner_total: self.records.iter().filter(|r| r.is_single_owner()).count(),
            escalations_required: self
                .records
                .iter()
                .filter(|r| r.requires_escalation())
                .count(),
            escalations_raised: self
                .records
                .iter()
                .filter(|r| r.escalation.escalation_state == EscalationState::Raised)
                .count(),
            split_authority_enforced: self
                .records
                .iter()
                .filter(|r| r.split_authority.split_state == SplitAuthorityState::Satisfied)
                .count(),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, service-health, release-center publication,
    /// support exports, and shiproom panels. It carries only the family, lane, declared and
    /// effective labels, criticality, state, the scan/surface-agreement flag, the
    /// owner/backup/quorum/runbook/escalation summary, active reasons, and surfaces — never the
    /// detailed roster, runbook, and proof internals.
    pub fn reuse_projection(&self) -> Vec<AuthorityContinuityReuseRow> {
        self.records
            .iter()
            .map(|r| AuthorityContinuityReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                lane: r.lane,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                criticality: r.criticality,
                continuity_state: r.continuity_state,
                release_blocking: r.release_blocking,
                scan_surface_agree: r.scan_surface_agree(),
                owner_state: r.owner_coverage.owner_state,
                backup_state: r.backup_coverage.backup_state,
                quorum_state: r.roster.quorum_state,
                runbook_state: r.runbook.runbook_state,
                escalation_state: r.escalation.escalation_state,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_RELEASE_AUTHORITY_CONTINUITY_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_RELEASE_AUTHORITY_CONTINUITY_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every authority lane must be exercised by at least one record.
        for lane in AuthorityLane::ALL {
            if !self.records.iter().any(|r| r.lane == lane) {
                v.push(RegisterViolation::AuthorityLaneUncovered { lane });
            }
        }

        // Every reason must have a stop rule.
        for reason in ContinuityReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(RegisterViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for r in &self.records {
            self.validate_record(r, &mut seen, &mut v);
        }

        // Verdict, parity, and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(RegisterViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(RegisterViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_record_ids != self.computed_blocking_record_ids() {
            v.push(RegisterViolation::PublicationBlockingRecordsMismatch);
        }
        if self.scan_surface_parity != self.computed_scan_surface_parity() {
            v.push(RegisterViolation::ScanSurfaceParityMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(RegisterViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<RegisterViolation>) {
        if self.families != M5Family::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "families" });
        }
        if self.authority_lanes != AuthorityLane::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "authority_lanes",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.criticalities != LaneCriticality::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "criticalities",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.roster_kinds != RosterKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "roster_kinds",
            });
        }
        if self.owner_states != OwnerState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "owner_states",
            });
        }
        if self.backup_states != BackupState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "backup_states",
            });
        }
        if self.quorum_states != QuorumState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "quorum_states",
            });
        }
        if self.split_authority_states != SplitAuthorityState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "split_authority_states",
            });
        }
        if self.runbook_states != RunbookState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "runbook_states",
            });
        }
        if self.escalation_states != EscalationState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "escalation_states",
            });
        }
        if self.postures != Posture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "postures" });
        }
        if self.continuity_states != ContinuityState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "continuity_states",
            });
        }
        if self.continuity_reasons != ContinuityReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "continuity_reasons",
            });
        }
        if self.continuity_actions != ContinuityAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "continuity_actions",
            });
        }
        if self.continuity_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "continuity_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &AuthorityContinuityRecord,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<RegisterViolation>,
    ) {
        for (field, value) in [
            ("record_id", &r.record_id),
            ("title", &r.title),
            ("subject_ref", &r.subject_ref),
            ("subject_summary", &r.subject_summary),
            ("rationale", &r.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(RegisterViolation::EmptyField {
                    record_id: r.record_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(r.record_id.clone()) {
            v.push(RegisterViolation::DuplicateRecordId {
                record_id: r.record_id.clone(),
            });
        }
        if r.surfaces.is_empty() {
            v.push(RegisterViolation::RecordMissingSurfaces {
                record_id: r.record_id.clone(),
            });
        }

        self.validate_fact_consistency(r, v);
        self.validate_controls(r, v);
        self.validate_reason_evidence(r, v);
        self.validate_scan_surface(r, v);
        self.validate_state_and_label(r, v);
    }

    /// Each fact block must be internally consistent — so a state token can never sit over a
    /// contradicting fact (an "assigned" owner with no owner ref, a "single_owner" backup with a
    /// non-zero count, a roster whose quorum flag disagrees with its counts, a roster kind that
    /// does not match the lane, or a split-authority / escalation whose applicability disagrees
    /// with the criticality).
    fn validate_fact_consistency(
        &self,
        r: &AuthorityContinuityRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        // assigned ⟺ primary owner ref present.
        let assigned = r.owner_coverage.owner_state == OwnerState::Assigned;
        let owner_present = !r.owner_coverage.primary_owner_ref.trim().is_empty();
        if assigned != owner_present {
            v.push(RegisterViolation::OwnerFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // single_owner ⟺ no backup owners.
        let single = r.backup_coverage.backup_state == BackupState::SingleOwner;
        if single != (r.backup_coverage.backup_owner_count == 0) {
            v.push(RegisterViolation::BackupFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // the roster kind must match the lane.
        if r.roster.roster_kind != r.lane.expected_roster_kind() {
            v.push(RegisterViolation::RosterKindMismatch {
                record_id: r.record_id.clone(),
            });
        }
        // quorum required ≥ 1, and quorum_state must agree with the member counts.
        if r.roster.required_quorum == 0 || r.roster.quorum_state.is_below() == r.roster.is_met() {
            v.push(RegisterViolation::QuorumFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a runbook is captured ⟺ a next-review date is present.
        let runbook_present = r.runbook.next_review_due.is_some();
        if (r.runbook.runbook_state != RunbookState::Missing) != runbook_present {
            v.push(RegisterViolation::RunbookFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // split authority applies iff the lane requires it; the `required` flag must agree, and a
        // satisfied split needs two authorities while an unmet one has fewer.
        let split_applies = r.split_authority.split_state != SplitAuthorityState::NotRequired;
        let split_ok = split_applies == r.requires_split_authority()
            && r.split_authority.required == r.requires_split_authority()
            && (r.split_authority.split_state != SplitAuthorityState::Satisfied
                || r.split_authority.distinct_authorities >= 2)
            && (r.split_authority.split_state != SplitAuthorityState::Unmet
                || r.split_authority.distinct_authorities < 2);
        if !split_ok {
            v.push(RegisterViolation::SplitAuthorityInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // an escalation applies iff the lane is critical or single-owner, and the `required` flag
        // and `escalation_state` must agree on that.
        let escalation_applies = r.escalation.escalation_state != EscalationState::NotRequired;
        if escalation_applies != r.requires_escalation()
            || r.escalation.required != r.requires_escalation()
        {
            v.push(RegisterViolation::EscalationApplicabilityInconsistent {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_controls(&self, r: &AuthorityContinuityRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state must equal
        // the state its facts imply — so a control can never assert "satisfied" over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&ContinuityControl> = r
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .collect();
            if matches.len() != 1 {
                v.push(RegisterViolation::ControlDimensionNotDeclaredOnce {
                    record_id: r.record_id.clone(),
                    dimension,
                });
                continue;
            }
            let expected = r.expected_control_state(dimension);
            if matches[0].state != expected {
                v.push(RegisterViolation::ControlStateInconsistent {
                    record_id: r.record_id.clone(),
                    dimension,
                });
            }
        }
    }

    /// Every active reason must be justified by the record's own facts, and every structural gap
    /// must surface its reason.
    fn validate_reason_evidence(
        &self,
        r: &AuthorityContinuityRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        let owner_vacant = r.owner_vacant();
        let single_owner = r.is_single_owner();
        let quorum_below = r.quorum_below();
        let runbook_stale = r.runbook.runbook_state.is_stale();
        let runbook_missing = r.runbook.runbook_state.is_missing();
        let split_unmet = r.split_authority_unmet();
        let escalation_missing = r.escalation_missing();
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                ContinuityReason::PrimaryOwnerVacant => owner_vacant,
                ContinuityReason::BackupOwnerMissing => single_owner,
                ContinuityReason::RosterQuorumBelowThreshold => quorum_below,
                ContinuityReason::RunbookStale => runbook_stale,
                ContinuityReason::RunbookMissing => runbook_missing,
                ContinuityReason::SplitAuthorityUnmet => split_unmet,
                ContinuityReason::ShiproomEscalationMissing => escalation_missing,
                ContinuityReason::ContinuityProofStale => proof_stale,
                ContinuityReason::ContinuityProofMissing => proof_missing,
                ContinuityReason::OwnerSignoffMissing => signoff_missing,
                ContinuityReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: ContinuityReason, v: &mut Vec<RegisterViolation>| {
            if present && !r.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    record_id: r.record_id.clone(),
                    reason,
                });
            }
        };
        require(owner_vacant, ContinuityReason::PrimaryOwnerVacant, v);
        require(single_owner, ContinuityReason::BackupOwnerMissing, v);
        require(
            quorum_below,
            ContinuityReason::RosterQuorumBelowThreshold,
            v,
        );
        require(runbook_stale, ContinuityReason::RunbookStale, v);
        require(runbook_missing, ContinuityReason::RunbookMissing, v);
        require(split_unmet, ContinuityReason::SplitAuthorityUnmet, v);
        require(
            escalation_missing,
            ContinuityReason::ShiproomEscalationMissing,
            v,
        );
        require(proof_stale, ContinuityReason::ContinuityProofStale, v);
        require(proof_missing, ContinuityReason::ContinuityProofMissing, v);
        require(signoff_missing, ContinuityReason::OwnerSignoffMissing, v);
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a green surface
    /// may never sit over a scan that found a single-owner, under-quorum, or runbook-less lane.
    fn validate_scan_surface(&self, r: &AuthorityContinuityRecord, v: &mut Vec<RegisterViolation>) {
        if r.scan_posture != r.surface_posture {
            v.push(RegisterViolation::ScanSurfaceDisagreement {
                record_id: r.record_id.clone(),
            });
        }
        let computed = r.computed_posture();
        if r.surface_posture != computed || r.scan_posture != computed {
            v.push(RegisterViolation::PostureMismatch {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_state_and_label(
        &self,
        r: &AuthorityContinuityRecord,
        v: &mut Vec<RegisterViolation>,
    ) {
        // cleared ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_cleared() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::ClearedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.continuity_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.continuity_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.continuity_state,
                computed: r.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if r.effective_label.rank() > r.declared_label.rank() {
            v.push(RegisterViolation::EffectiveLabelExceedsDeclared {
                record_id: r.record_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if r.effective_label != r.computed_effective_label() {
            v.push(RegisterViolation::EffectiveLabelMismatch {
                record_id: r.record_id.clone(),
            });
        }
        // a narrowed record must drop below the cutline.
        if r.continuity_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityContinuityReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Authority lane.
    pub lane: AuthorityLane,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Criticality grade.
    pub criticality: LaneCriticality,
    /// Continuity state.
    pub continuity_state: ContinuityState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// Owner posture.
    pub owner_state: OwnerState,
    /// Backup posture.
    pub backup_state: BackupState,
    /// Roster-quorum posture.
    pub quorum_state: QuorumState,
    /// Runbook posture.
    pub runbook_state: RunbookState,
    /// Shiproom-escalation state.
    pub escalation_state: EscalationState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<ContinuityReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the release-authority continuity register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no records.
    EmptyRegister,
    /// An authority lane has no record.
    AuthorityLaneUncovered {
        /// Uncovered lane.
        lane: AuthorityLane,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: ContinuityReason,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate id.
        record_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Record id.
        record_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record lists no reuse surfaces.
    RecordMissingSurfaces {
        /// Record id.
        record_id: String,
    },
    /// A record's owner state disagrees with its primary owner ref.
    OwnerFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's backup state disagrees with its backup-owner count.
    BackupFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's roster kind does not match its lane.
    RosterKindMismatch {
        /// Record id.
        record_id: String,
    },
    /// A record's quorum state disagrees with its member counts.
    QuorumFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's runbook state disagrees with its next-review date.
    RunbookFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's split-authority facts disagree with its criticality.
    SplitAuthorityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's escalation applicability disagrees with its criticality/ownership.
    EscalationApplicabilityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A control dimension is not declared exactly once.
    ControlDimensionNotDeclaredOnce {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// A control's declared state disagrees with the facts it governs.
    ControlStateInconsistent {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// An active reason is not justified by the record's fields.
    ReasonNotJustified {
        /// Record id.
        record_id: String,
        /// Offending reason.
        reason: ContinuityReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: ContinuityReason,
    },
    /// A record's scan and surface postures disagree.
    ScanSurfaceDisagreement {
        /// Record id.
        record_id: String,
    },
    /// A record's posture disagrees with the gaps its state implies.
    PostureMismatch {
        /// Record id.
        record_id: String,
    },
    /// A cleared record carries an active reason.
    ClearedWithActiveReason {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record carries no reason.
    NarrowedWithoutReason {
        /// Record id.
        record_id: String,
    },
    /// The record state disagrees with the active reasons.
    StateReasonMismatch {
        /// Record id.
        record_id: String,
        /// Declared state.
        declared: ContinuityState,
        /// Computed state.
        computed: ContinuityState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Record id.
        record_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record did not drop below the cutline.
    NarrowedAboveCutline {
        /// Record id.
        record_id: String,
    },
    /// The promotion decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking record ids disagree with the computed set.
    PublicationBlockingRecordsMismatch,
    /// The recorded scan/surface parity disagrees with the computed summary.
    ScanSurfaceParityMismatch,
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for RegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no records"),
            Self::AuthorityLaneUncovered { lane } => {
                write!(f, "authority lane {} has no record", lane.as_str())
            }
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate record id {record_id}")
            }
            Self::EmptyField {
                record_id,
                field_name,
            } => write!(f, "record {record_id} has empty field {field_name}"),
            Self::RecordMissingSurfaces { record_id } => {
                write!(f, "record {record_id} lists no reuse surfaces")
            }
            Self::OwnerFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} owner state disagrees with its primary owner ref"
                )
            }
            Self::BackupFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} backup state disagrees with its backup-owner count"
                )
            }
            Self::RosterKindMismatch { record_id } => {
                write!(f, "record {record_id} roster kind does not match its lane")
            }
            Self::QuorumFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} quorum state disagrees with its member counts"
                )
            }
            Self::RunbookFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} runbook state disagrees with its next-review date"
                )
            }
            Self::SplitAuthorityInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} split-authority facts disagree with its criticality"
                )
            }
            Self::EscalationApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} escalation applicability disagrees with its criticality/ownership"
            ),
            Self::ControlDimensionNotDeclaredOnce {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} does not declare control {} exactly once",
                dimension.as_str()
            ),
            Self::ControlStateInconsistent {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} control {} state disagrees with its facts",
                dimension.as_str()
            ),
            Self::ReasonNotJustified { record_id, reason } => write!(
                f,
                "record {record_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason { record_id, reason } => write!(
                f,
                "record {record_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::ScanSurfaceDisagreement { record_id } => {
                write!(f, "record {record_id} scan and surface postures disagree")
            }
            Self::PostureMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} posture disagrees with the gaps its state implies"
                )
            }
            Self::ClearedWithActiveReason { record_id } => {
                write!(
                    f,
                    "cleared record {record_id} carries an active narrowing reason"
                )
            }
            Self::NarrowedWithoutReason { record_id } => {
                write!(f, "narrowed record {record_id} names no reason")
            }
            Self::StateReasonMismatch {
                record_id,
                declared,
                computed,
            } => write!(
                f,
                "record {record_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label is wider than its declared label"
                )
            }
            Self::EffectiveLabelMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label disagrees with its state"
                )
            }
            Self::NarrowedAboveCutline { record_id } => {
                write!(
                    f,
                    "narrowed record {record_id} did not drop below the cutline"
                )
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "promotion decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(
                    f,
                    "publication blocking_rule_ids disagree with the computed set"
                )
            }
            Self::PublicationBlockingRecordsMismatch => {
                write!(
                    f,
                    "publication blocking_record_ids disagree with the computed set"
                )
            }
            Self::ScanSurfaceParityMismatch => {
                write!(f, "scan_surface_parity disagrees with the computed summary")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the records"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded release-authority continuity register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`ReleaseAuthorityContinuityRegister`] — including when a record carries a token outside any
/// closed vocabulary.
pub fn current_m5_release_authority_continuity(
) -> Result<ReleaseAuthorityContinuityRegister, serde_json::Error> {
    serde_json::from_str(M5_RELEASE_AUTHORITY_CONTINUITY_JSON)
}

#[cfg(test)]
mod tests;
