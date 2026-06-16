//! Typed register of critical-upstream health truth per protected M5 dependency.
//!
//! The sibling [`m5_boundary_and_upstream_durability`](crate::m5_boundary_and_upstream_durability)
//! matrix records, per asset lane, *whether* a critical upstream is owned as one coarse flag,
//! and the [`m5_import_provenance_and_fork_review`](crate::m5_import_provenance_and_fork_review)
//! register records where each protected-path import came from. Neither makes each critical
//! upstream — the third-party packages, protocols, and curated imports a protected M5 family
//! leans on — inspectable as a durable health record: how healthy its maintainer base is, what
//! its security posture is, how fast it still ships, whether its review is on cadence, whether
//! its license is clear, how feasible a replacement would be, who owns it, and — when it is
//! red-risk or unowned — whether a sponsor/fork/replace plan is recorded and the shiproom has
//! been told.
//!
//! This module is that upstream-health layer. For every critical upstream a protected M5 family
//! depends on it records one [`UpstreamHealthRecord`] that states, in one copy-safe record:
//!
//! - the **maintainer health** ([`MaintainerHealth`]): the rating (active maintainers and bus
//!   factor), so a critical upstream is never left to coast on an abandoned maintainer base;
//! - the **security posture** ([`SecurityProfile`]): open advisories and unpatched criticals;
//! - the **update cadence** ([`UpdateCadenceProfile`]): whether the upstream still ships;
//! - the **review cadence** ([`ReviewCadence`]): a current review, a due-for-review reminder,
//!   an overdue review, or a missing one;
//! - the **license clarity** ([`LicenseProfile`]): clear, ambiguous, or incompatible;
//! - the **ownership** ([`UpstreamOwnership`]) and the **replacement feasibility** carried on
//!   the [`ContingencyPlan`], so an upstream is never left ownerless because it is "just
//!   infrastructure";
//! - the **sponsor/fork/replace contingency** ([`ContingencyPlan`]), required for any red-risk
//!   upstream;
//! - the **shiproom escalation** ([`ShiproomEscalation`]), required for any red-risk or unowned
//!   upstream.
//!
//! Each record also carries a [`scan_posture`](UpstreamHealthRecord::scan_posture) (what the
//! upstream-health scan found) and a [`surface_posture`](UpstreamHealthRecord::surface_posture)
//! (what the governance-dashboard/promotion-packet surface shows). The two **must agree**: a
//! record may never show a clean surface over a scan that found gaps, so a green upstream card
//! can never mask an abandoned, unpatched, or unowned dependency.
//!
//! A record is [`HealthState::Cleared`] only when the maintainer base is healthy, the security
//! posture is clean, the cadence has not stalled and the review is on cadence, the license is
//! clear, the upstream is owned, any required contingency plan and shiproom escalation are
//! recorded, the proof is fresh, and the owner signed. Otherwise it narrows on the *specific*
//! axis that thinned out — a maintainer gap, a security gap, a cadence gap, a license gap, an
//! ownership/escalation gap, or stale proof — never collapsing to one global flag. A narrowed
//! record drops its [`UpstreamHealthRecord::effective_label`] below the launch cutline and may
//! never publish an effective label wider than the one it declares.
//!
//! The [`HealthRule`] set names the closed conditions that gate promotion. An *inherited*
//! narrowing — a subject whose declared label already sits below the cutline, or a gap held by
//! an unexpired waiver — is gated upstream and does not itself hold promotion; an
//! *upstream-health* failure on a subject whose declared label is still at or above the cutline
//! holds promotion through a shiproom stop rule, recorded in
//! [`CriticalUpstreamHealthRegister::publication`] — a red-risk or unowned protected-path
//! dependency cannot widen a stable claim without an approved sponsor, fork, or replacement
//! plan. The cross-cutting [`ScanSurfaceParity`] block summarizes scan/surface agreement over
//! every subject.
//!
//! The register is checked in at `artifacts/governance/m5-critical-upstream-health.json` and
//! embedded here, so this typed consumer and the CI gate agree on every record without a cargo
//! build in CI. The model is metadata-only: every field is a typed state, a boolean flag, a
//! small count, a label, or an opaque ref. It carries no credential bodies, raw provider
//! payloads, source contents, or signatures. Date arithmetic (recomputing proof, review, and
//! waiver freshness against an `as_of` date) lives in the CI gate and the integration test;
//! this model enforces the invariants that hold regardless of the clock: scan/surface parity,
//! the no-widening ceiling, control/fact consistency, reason/state coherence, summary
//! agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND: &str = "m5_critical_upstream_health_register";

/// Repo-relative path to the checked-in register.
pub const M5_CRITICAL_UPSTREAM_HEALTH_PATH: &str =
    "artifacts/governance/m5-critical-upstream-health.json";

/// Embedded checked-in register JSON.
pub const M5_CRITICAL_UPSTREAM_HEALTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-critical-upstream-health.json"
));

/// The kind of critical upstream a record governs.
///
/// The same health truth is published for packages, protocols, curated imports, and toolchain
/// components — so a stalled protocol or an abandoned toolchain component cannot hide behind a
/// healthy package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpstreamKind {
    /// A third-party package dependency.
    Package,
    /// A wire/format protocol the family implements.
    Protocol,
    /// A curated/vendored import.
    CuratedImport,
    /// A build/toolchain component.
    ToolchainComponent,
}

impl UpstreamKind {
    /// Every upstream kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Package,
        Self::Protocol,
        Self::CuratedImport,
        Self::ToolchainComponent,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Package => "package",
            Self::Protocol => "protocol",
            Self::CuratedImport => "curated_import",
            Self::ToolchainComponent => "toolchain_component",
        }
    }
}

/// The risk grade an upstream carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthGrade {
    /// Healthy with no outstanding risk.
    Green,
    /// A risk is present but not release-blocking on its own.
    Amber,
    /// Red-risk: a sponsor/fork/replace plan and escalation are required.
    Red,
    /// Blocked: failure would block the family.
    Blocked,
}

impl HealthGrade {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 4] = [Self::Green, Self::Amber, Self::Red, Self::Blocked];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Amber => "amber",
            Self::Red => "red",
            Self::Blocked => "blocked",
        }
    }

    /// True when the grade is red-risk (`red`/`blocked`): a contingency plan is required and an
    /// escalation must be raised.
    pub fn is_red_risk(self) -> bool {
        matches!(self, Self::Red | Self::Blocked)
    }
}

/// An upstream-health control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Maintainer health: an active maintainer base and a healthy bus factor.
    MaintainerHealth,
    /// Security posture: no open advisories or unpatched criticals.
    SecurityPosture,
    /// Update cadence: the upstream still ships and its review is on cadence.
    UpdateCadence,
    /// License clarity: a clear, compatible license.
    LicenseClarity,
    /// Ownership continuity: an assigned owner, plus a contingency plan and escalation where
    /// required.
    OwnershipContinuity,
    /// Scan/surface parity: the upstream-health scan and the governance surface agree.
    ScanSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 6] = [
        Self::MaintainerHealth,
        Self::SecurityPosture,
        Self::UpdateCadence,
        Self::LicenseClarity,
        Self::OwnershipContinuity,
        Self::ScanSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaintainerHealth => "maintainer_health",
            Self::SecurityPosture => "security_posture",
            Self::UpdateCadence => "update_cadence",
            Self::LicenseClarity => "license_clarity",
            Self::OwnershipContinuity => "ownership_continuity",
            Self::ScanSurfaceParity => "scan_surface_parity",
        }
    }
}

/// Maintainer-health rating for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MaintainerRating {
    /// A healthy maintainer base with redundancy.
    Healthy,
    /// The maintainer base is thinning.
    Thinning,
    /// Effectively a single maintainer (bus-factor risk).
    SingleMaintainer,
    /// The upstream has been abandoned by its maintainers.
    Abandoned,
}

impl MaintainerRating {
    /// Every rating, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Healthy,
        Self::Thinning,
        Self::SingleMaintainer,
        Self::Abandoned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Thinning => "thinning",
            Self::SingleMaintainer => "single_maintainer",
            Self::Abandoned => "abandoned",
        }
    }

    /// True when the maintainer base is anything other than healthy.
    pub fn is_degraded(self) -> bool {
        !matches!(self, Self::Healthy)
    }

    /// True when the maintainer base has thinned (thinning or single-maintainer).
    pub fn is_thinning(self) -> bool {
        matches!(self, Self::Thinning | Self::SingleMaintainer)
    }

    /// True when the maintainer base has been abandoned.
    pub fn is_abandoned(self) -> bool {
        matches!(self, Self::Abandoned)
    }
}

/// Security posture for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityPosture {
    /// No open advisories.
    Clean,
    /// One or more open advisories.
    AdvisoriesOpen,
    /// An unpatched critical vulnerability.
    UnpatchedCritical,
}

impl SecurityPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 3] = [Self::Clean, Self::AdvisoriesOpen, Self::UnpatchedCritical];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::AdvisoriesOpen => "advisories_open",
            Self::UnpatchedCritical => "unpatched_critical",
        }
    }

    /// True when the posture is anything other than clean.
    pub fn is_degraded(self) -> bool {
        !matches!(self, Self::Clean)
    }
}

/// Update cadence for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateCadence {
    /// The upstream still ships on an active cadence.
    Active,
    /// The cadence is slowing (a reminder, not a gap).
    Slowing,
    /// The upstream has stopped shipping releases.
    Stalled,
}

impl UpdateCadence {
    /// Every cadence, in declaration order.
    pub const ALL: [Self; 3] = [Self::Active, Self::Slowing, Self::Stalled];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Slowing => "slowing",
            Self::Stalled => "stalled",
        }
    }

    /// True when the cadence has stalled.
    pub fn is_stalled(self) -> bool {
        matches!(self, Self::Stalled)
    }
}

/// State of an upstream's periodic health review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCadenceState {
    /// The review is current.
    Current,
    /// The next review is coming due (a reminder, not a gap).
    DueForReview,
    /// The review is overdue.
    Overdue,
    /// No review is captured.
    Missing,
}

impl ReviewCadenceState {
    /// Every review-cadence state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::DueForReview,
        Self::Overdue,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForReview => "due_for_review",
            Self::Overdue => "overdue",
            Self::Missing => "missing",
        }
    }

    /// True when the review is overdue.
    pub fn is_overdue(self) -> bool {
        matches!(self, Self::Overdue)
    }

    /// True when no review is captured.
    pub fn is_missing(self) -> bool {
        matches!(self, Self::Missing)
    }
}

/// License clarity for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseClarity {
    /// A clear, identified, compatible license.
    Clear,
    /// The effective license is ambiguous.
    Ambiguous,
    /// The license is incompatible.
    Incompatible,
}

impl LicenseClarity {
    /// Every clarity, in declaration order.
    pub const ALL: [Self; 3] = [Self::Clear, Self::Ambiguous, Self::Incompatible];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Ambiguous => "ambiguous",
            Self::Incompatible => "incompatible",
        }
    }

    /// True when the license is anything other than clear.
    pub fn is_degraded(self) -> bool {
        !matches!(self, Self::Clear)
    }
}

/// Replacement feasibility for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementFeasibility {
    /// A drop-in replacement exists.
    DropIn,
    /// Replacement is moderate effort.
    Moderate,
    /// Replacement is hard.
    Hard,
    /// No known replacement path.
    NoKnownPath,
}

impl ReplacementFeasibility {
    /// Every feasibility, in declaration order.
    pub const ALL: [Self; 4] = [Self::DropIn, Self::Moderate, Self::Hard, Self::NoKnownPath];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DropIn => "drop_in",
            Self::Moderate => "moderate",
            Self::Hard => "hard",
            Self::NoKnownPath => "no_known_path",
        }
    }
}

/// Whether the upstream has an assigned owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipState {
    /// The upstream has an assigned owner.
    Owned,
    /// The upstream has no assigned owner.
    Unowned,
}

impl OwnershipState {
    /// Every ownership state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Owned, Self::Unowned];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owned => "owned",
            Self::Unowned => "unowned",
        }
    }
}

/// State of an upstream's sponsor/fork/replace contingency plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContingencyState {
    /// A contingency plan is recorded.
    Recorded,
    /// A contingency plan is required but still pending.
    Pending,
    /// A contingency plan is not required for this upstream.
    NotRequired,
}

impl ContingencyState {
    /// Every contingency state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Recorded, Self::Pending, Self::NotRequired];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Recorded => "recorded",
            Self::Pending => "pending",
            Self::NotRequired => "not_required",
        }
    }
}

/// The disposition a recorded contingency plan settles on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContingencyDisposition {
    /// Sponsor the upstream so it stays maintained.
    SponsorUpstream,
    /// Maintain a local fork deliberately.
    MaintainFork,
    /// Replace the dependency with another source.
    ReplaceDependency,
    /// No disposition (no recorded plan).
    None,
}

impl ContingencyDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SponsorUpstream,
        Self::MaintainFork,
        Self::ReplaceDependency,
        Self::None,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SponsorUpstream => "sponsor_upstream",
            Self::MaintainFork => "maintain_fork",
            Self::ReplaceDependency => "replace_dependency",
            Self::None => "none",
        }
    }

    /// True when the disposition names a settled choice.
    pub fn is_settled(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// State of an upstream's shiproom/governance escalation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EscalationState {
    /// The escalation has been raised.
    Raised,
    /// An escalation is required but still pending.
    Pending,
    /// An escalation is not required for this upstream.
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

/// The posture a scan or a surface reports for an upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    /// No health gap found.
    Clear,
    /// One or more health gaps found.
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
    /// The control holds for this upstream.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this upstream.
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
pub enum HealthState {
    /// Maintainer, security, cadence, license, ownership, and proof all hold.
    Cleared,
    /// The maintainer base has thinned or been abandoned.
    NarrowedMaintainer,
    /// An open advisory or unpatched critical is present.
    NarrowedSecurity,
    /// The update cadence has stalled or a review is overdue/missing.
    NarrowedCadence,
    /// The license is ambiguous or incompatible.
    NarrowedLicense,
    /// The upstream is unowned, or a required contingency/escalation is missing.
    NarrowedOwnership,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The upstream is withdrawn.
    Withdrawn,
}

impl HealthState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Cleared,
        Self::NarrowedMaintainer,
        Self::NarrowedSecurity,
        Self::NarrowedCadence,
        Self::NarrowedLicense,
        Self::NarrowedOwnership,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::NarrowedMaintainer => "narrowed_maintainer",
            Self::NarrowedSecurity => "narrowed_security",
            Self::NarrowedCadence => "narrowed_cadence",
            Self::NarrowedLicense => "narrowed_license",
            Self::NarrowedOwnership => "narrowed_ownership",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not cleared, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Cleared | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by a [`HealthRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthReason {
    /// The maintainer base has thinned to a bus-factor risk.
    MaintainerHealthThinning,
    /// The upstream has been abandoned by its maintainers.
    MaintainerAbandoned,
    /// One or more advisories are open.
    SecurityAdvisoriesOpen,
    /// An unpatched critical vulnerability is open.
    SecurityUnpatchedCritical,
    /// The update cadence has stalled.
    UpdateCadenceStalled,
    /// The upstream-health review is overdue.
    ReviewCadenceOverdue,
    /// No upstream-health review is captured.
    ReviewCadenceMissing,
    /// The effective license is ambiguous.
    LicenseAmbiguous,
    /// The license is incompatible.
    LicenseIncompatible,
    /// The critical upstream has no recorded owner.
    UpstreamUnowned,
    /// A red-risk upstream has no recorded sponsor/fork/replace plan.
    ContingencyPlanMissing,
    /// A red-risk or unowned upstream has not been escalated to the shiproom.
    ShiproomEscalationMissing,
    /// The upstream-health proof packet aged past its freshness SLO.
    HealthProofStale,
    /// No upstream-health proof packet is captured.
    HealthProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl HealthReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::MaintainerHealthThinning,
        Self::MaintainerAbandoned,
        Self::SecurityAdvisoriesOpen,
        Self::SecurityUnpatchedCritical,
        Self::UpdateCadenceStalled,
        Self::ReviewCadenceOverdue,
        Self::ReviewCadenceMissing,
        Self::LicenseAmbiguous,
        Self::LicenseIncompatible,
        Self::UpstreamUnowned,
        Self::ContingencyPlanMissing,
        Self::ShiproomEscalationMissing,
        Self::HealthProofStale,
        Self::HealthProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaintainerHealthThinning => "maintainer_health_thinning",
            Self::MaintainerAbandoned => "maintainer_abandoned",
            Self::SecurityAdvisoriesOpen => "security_advisories_open",
            Self::SecurityUnpatchedCritical => "security_unpatched_critical",
            Self::UpdateCadenceStalled => "update_cadence_stalled",
            Self::ReviewCadenceOverdue => "review_cadence_overdue",
            Self::ReviewCadenceMissing => "review_cadence_missing",
            Self::LicenseAmbiguous => "license_ambiguous",
            Self::LicenseIncompatible => "license_incompatible",
            Self::UpstreamUnowned => "upstream_unowned",
            Self::ContingencyPlanMissing => "contingency_plan_missing",
            Self::ShiproomEscalationMissing => "shiproom_escalation_missing",
            Self::HealthProofStale => "health_proof_stale",
            Self::HealthProofMissing => "health_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active. Ownership (the
    /// unowned / red-risk-without-plan guardrail) is the worst, then security, then maintainer,
    /// cadence, license, and finally the evidence-staleness axis.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            HealthState::NarrowedOwnership => 0,
            HealthState::NarrowedSecurity => 1,
            HealthState::NarrowedMaintainer => 2,
            HealthState::NarrowedCadence => 3,
            HealthState::NarrowedLicense => 4,
            _ => 5,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> HealthState {
        match self {
            Self::MaintainerHealthThinning | Self::MaintainerAbandoned => {
                HealthState::NarrowedMaintainer
            }
            Self::SecurityAdvisoriesOpen | Self::SecurityUnpatchedCritical => {
                HealthState::NarrowedSecurity
            }
            Self::UpdateCadenceStalled
            | Self::ReviewCadenceOverdue
            | Self::ReviewCadenceMissing => HealthState::NarrowedCadence,
            Self::LicenseAmbiguous | Self::LicenseIncompatible => HealthState::NarrowedLicense,
            Self::UpstreamUnowned
            | Self::ContingencyPlanMissing
            | Self::ShiproomEscalationMissing => HealthState::NarrowedOwnership,
            Self::HealthProofStale
            | Self::HealthProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => HealthState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::MaintainerHealthThinning | Self::MaintainerAbandoned => {
                ControlDimension::MaintainerHealth
            }
            Self::SecurityAdvisoriesOpen | Self::SecurityUnpatchedCritical => {
                ControlDimension::SecurityPosture
            }
            Self::UpdateCadenceStalled
            | Self::ReviewCadenceOverdue
            | Self::ReviewCadenceMissing => ControlDimension::UpdateCadence,
            Self::LicenseAmbiguous | Self::LicenseIncompatible => ControlDimension::LicenseClarity,
            Self::UpstreamUnowned
            | Self::ContingencyPlanMissing
            | Self::ShiproomEscalationMissing => ControlDimension::OwnershipContinuity,
            Self::HealthProofStale
            | Self::HealthProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ScanSurfaceParity,
        }
    }
}

/// An action a [`HealthRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Assign a backup maintainer.
    AssignBackupMaintainer,
    /// Sponsor or replace the abandoned upstream.
    SponsorOrReplaceUpstream,
    /// Remediate the open advisory.
    RemediateOpenAdvisory,
    /// Patch the critical vulnerability.
    PatchCriticalVulnerability,
    /// Escalate the stalled cadence.
    EscalateStalledCadence,
    /// Refresh the upstream-health review.
    RefreshUpstreamReview,
    /// Clarify the upstream license.
    ClarifyUpstreamLicense,
    /// Assign an upstream owner.
    AssignUpstreamOwner,
    /// Record the sponsor/fork/replace contingency plan.
    RecordContingencyPlan,
    /// Raise the shiproom escalation.
    RaiseShiproomEscalation,
    /// Refresh the upstream-health proof packet.
    RefreshHealthProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl HealthAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::HoldPromotion,
        Self::AssignBackupMaintainer,
        Self::SponsorOrReplaceUpstream,
        Self::RemediateOpenAdvisory,
        Self::PatchCriticalVulnerability,
        Self::EscalateStalledCadence,
        Self::RefreshUpstreamReview,
        Self::ClarifyUpstreamLicense,
        Self::AssignUpstreamOwner,
        Self::RecordContingencyPlan,
        Self::RaiseShiproomEscalation,
        Self::RefreshHealthProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::AssignBackupMaintainer => "assign_backup_maintainer",
            Self::SponsorOrReplaceUpstream => "sponsor_or_replace_upstream",
            Self::RemediateOpenAdvisory => "remediate_open_advisory",
            Self::PatchCriticalVulnerability => "patch_critical_vulnerability",
            Self::EscalateStalledCadence => "escalate_stalled_cadence",
            Self::RefreshUpstreamReview => "refresh_upstream_review",
            Self::ClarifyUpstreamLicense => "clarify_upstream_license",
            Self::AssignUpstreamOwner => "assign_upstream_owner",
            Self::RecordContingencyPlan => "record_contingency_plan",
            Self::RaiseShiproomEscalation => "raise_shiproom_escalation",
            Self::RefreshHealthProof => "refresh_health_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No upstream-health stop rule fires; promotion may proceed.
    Proceed,
    /// An upstream-health stop rule fires; hold promotion.
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

/// Maintainer-health facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaintainerHealth {
    /// Maintainer rating.
    pub rating: MaintainerRating,
    /// Number of active maintainers.
    pub active_maintainer_count: u32,
    /// Bus factor (maintainers who can independently ship).
    pub bus_factor: u32,
    /// Reference to the maintainer assessment.
    pub assessment_ref: String,
}

impl MaintainerHealth {
    /// True when the maintainer base is degraded.
    pub fn is_degraded(&self) -> bool {
        self.rating.is_degraded()
    }
}

/// Security-posture facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecurityProfile {
    /// Security posture.
    pub posture: SecurityPosture,
    /// Number of open advisories.
    pub open_advisory_count: u32,
    /// Reference to the advisory record.
    pub advisory_ref: String,
}

impl SecurityProfile {
    /// True when the security posture is degraded.
    pub fn is_degraded(&self) -> bool {
        self.posture.is_degraded()
    }
}

/// Update-cadence facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateCadenceProfile {
    /// Update cadence.
    pub cadence: UpdateCadence,
    /// Days since the last upstream release.
    pub days_since_last_release: u32,
    /// Reference to the release history.
    pub release_ref: String,
}

/// Review-cadence facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewCadence {
    /// Review-cadence state.
    pub cadence_state: ReviewCadenceState,
    /// Review interval in days.
    pub review_interval_days: u32,
    /// Next review due date (`null` when no review is captured).
    pub next_review_due: Option<String>,
    /// Reference to the last recorded review.
    pub last_reviewed_ref: String,
}

/// License-clarity facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseProfile {
    /// License clarity.
    pub clarity: LicenseClarity,
    /// SPDX license id (empty unless the license is clear).
    pub spdx_license_id: String,
    /// Reference to the license record.
    pub license_ref: String,
}

/// Ownership facts for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamOwnership {
    /// Ownership state.
    pub ownership_state: OwnershipState,
    /// Owning team or role (empty when unowned).
    pub owner_ref: String,
    /// Owner of the escalation path for this upstream.
    pub escalation_owner_ref: String,
}

impl UpstreamOwnership {
    /// True when the upstream has no assigned owner.
    pub fn is_unowned(&self) -> bool {
        self.ownership_state == OwnershipState::Unowned
    }
}

/// The sponsor/fork/replace contingency plan for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContingencyPlan {
    /// Plan state.
    pub plan_state: ContingencyState,
    /// Settled disposition.
    pub disposition: ContingencyDisposition,
    /// Replacement feasibility for this upstream.
    pub replacement_feasibility: ReplacementFeasibility,
    /// Reference to the plan record.
    pub plan_ref: String,
}

/// The shiproom/governance escalation for an upstream.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShiproomEscalation {
    /// Escalation state.
    pub escalation_state: EscalationState,
    /// True when an escalation is required for this upstream.
    pub required: bool,
    /// Reference to the shiproom escalation queue.
    pub shiproom_ref: String,
    /// Reference to the governance review record.
    pub governance_ref: String,
}

/// One upstream-health control binding on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One critical-upstream health record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamHealthRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this upstream serves.
    pub family: M5Family,
    /// The kind of upstream.
    pub upstream_kind: UpstreamKind,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this upstream is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// The risk grade the upstream carries.
    pub risk_grade: HealthGrade,
    /// Maintainer-health facts.
    pub maintainer: MaintainerHealth,
    /// Security-posture facts.
    pub security: SecurityProfile,
    /// Update-cadence facts.
    pub update_cadence: UpdateCadenceProfile,
    /// Review-cadence facts.
    pub review_cadence: ReviewCadence,
    /// License-clarity facts.
    pub license: LicenseProfile,
    /// Ownership facts.
    pub ownership: UpstreamOwnership,
    /// Sponsor/fork/replace contingency plan.
    pub contingency: ContingencyPlan,
    /// Shiproom/governance escalation.
    pub escalation: ShiproomEscalation,
    /// Per-dimension control bindings.
    pub controls: Vec<HealthControl>,
    /// What the upstream-health scan found.
    pub scan_posture: Posture,
    /// What the governance-dashboard/promotion-packet surface shows.
    pub surface_posture: Posture,
    /// Reference to the upstream-health scan.
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
    pub health_state: HealthState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<HealthReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, service-health, release-center, support).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl UpstreamHealthRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(HealthReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: HealthReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a cleared state.
    pub fn is_cleared(&self) -> bool {
        self.health_state == HealthState::Cleared
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// True when the upstream is red-risk (`red`/`blocked`).
    pub fn is_red_risk(&self) -> bool {
        self.risk_grade.is_red_risk()
    }

    /// True when the upstream is unowned.
    pub fn is_unowned(&self) -> bool {
        self.ownership.is_unowned()
    }

    /// True when this upstream requires a sponsor/fork/replace contingency plan.
    pub fn requires_contingency(&self) -> bool {
        self.is_red_risk()
    }

    /// True when this upstream requires a shiproom escalation.
    pub fn requires_escalation(&self) -> bool {
        self.is_red_risk() || self.is_unowned()
    }

    /// True when a required contingency plan is still pending.
    pub fn contingency_missing(&self) -> bool {
        self.requires_contingency() && self.contingency.plan_state == ContingencyState::Pending
    }

    /// True when a required shiproom escalation is still pending.
    pub fn escalation_missing(&self) -> bool {
        self.requires_escalation() && self.escalation.escalation_state == EscalationState::Pending
    }

    /// True when the update cadence has stalled or a required review is overdue/missing.
    pub fn cadence_degraded(&self) -> bool {
        self.update_cadence.cadence.is_stalled()
            || self.review_cadence.cadence_state.is_overdue()
            || self.review_cadence.cadence_state.is_missing()
    }

    /// True when any structural health gap (other than proof/sign-off) is present.
    pub fn has_health_gap(&self) -> bool {
        self.maintainer.is_degraded()
            || self.security.is_degraded()
            || self.cadence_degraded()
            || self.license.clarity.is_degraded()
            || self.is_unowned()
            || self.contingency_missing()
            || self.escalation_missing()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        match dimension {
            ControlDimension::MaintainerHealth => {
                if self.maintainer.is_degraded() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::SecurityPosture => {
                if self.security.is_degraded() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::UpdateCadence => {
                if self.cadence_degraded() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::LicenseClarity => {
                if self.license.clarity.is_degraded() {
                    ControlState::Unsatisfied
                } else {
                    ControlState::Satisfied
                }
            }
            ControlDimension::OwnershipContinuity => {
                if self.is_unowned() || self.contingency_missing() || self.escalation_missing() {
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
    pub fn computed_state(&self) -> HealthState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return HealthState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => HealthState::Cleared,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            HealthState::Cleared => self.declared_label,
            HealthState::Withdrawn => LifecycleLabel::Withdrawn,
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
        if self.health_state.is_narrowed() {
            Posture::GapsFound
        } else {
            Posture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by an
    /// upstream-health gap, declaring a label at or above the cutline, and not held by an
    /// unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.health_state.is_narrowed()
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
pub struct HealthRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: HealthReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: HealthAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HealthCutline {
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
    /// Upstream-health scorecard.
    pub upstream_scorecard_ref: String,
    /// Dependency register.
    pub dependency_register_ref: String,
    /// Security-advisory register.
    pub advisory_register_ref: String,
    /// Import-provenance and fork-review register.
    pub import_register_ref: String,
    /// Package inventory (protected-path posture).
    pub package_inventory_ref: String,
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
pub struct HealthSummary {
    /// Total records.
    pub total_records: usize,
    /// Cleared records.
    pub records_cleared: usize,
    /// Narrowed records.
    pub records_narrowed: usize,
    /// Records in the `cleared` state.
    pub state_cleared: usize,
    /// Records in the `narrowed_maintainer` state.
    pub state_narrowed_maintainer: usize,
    /// Records in the `narrowed_security` state.
    pub state_narrowed_security: usize,
    /// Records in the `narrowed_cadence` state.
    pub state_narrowed_cadence: usize,
    /// Records in the `narrowed_license` state.
    pub state_narrowed_license: usize,
    /// Records in the `narrowed_ownership` state.
    pub state_narrowed_ownership: usize,
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
    /// Records carrying a maintainer-health gap.
    pub maintainer_gaps: usize,
    /// Records carrying a security gap.
    pub security_gaps: usize,
    /// Records carrying an update/review cadence gap.
    pub cadence_gaps: usize,
    /// Records carrying a license gap.
    pub license_gaps: usize,
    /// Records carrying an ownership/contingency/escalation gap.
    pub ownership_gaps: usize,
    /// Red-risk records.
    pub red_risk_total: usize,
    /// Unowned records.
    pub unowned_total: usize,
    /// Records that require a shiproom escalation.
    pub escalations_required: usize,
    /// Records whose escalation has been raised.
    pub escalations_raised: usize,
    /// Records with a recorded contingency plan.
    pub contingency_plans_recorded: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of critical-upstream health records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CriticalUpstreamHealthRegister {
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
    pub health_cutline: HealthCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed upstream-kind vocabulary.
    pub upstream_kinds: Vec<UpstreamKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed health-grade vocabulary.
    pub health_grades: Vec<HealthGrade>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed maintainer-rating vocabulary.
    pub maintainer_ratings: Vec<MaintainerRating>,
    /// Closed security-posture vocabulary.
    pub security_postures: Vec<SecurityPosture>,
    /// Closed update-cadence vocabulary.
    pub update_cadences: Vec<UpdateCadence>,
    /// Closed review-cadence-state vocabulary.
    pub review_cadence_states: Vec<ReviewCadenceState>,
    /// Closed license-clarity vocabulary.
    pub license_clarities: Vec<LicenseClarity>,
    /// Closed replacement-feasibility vocabulary.
    pub replacement_feasibilities: Vec<ReplacementFeasibility>,
    /// Closed ownership-state vocabulary.
    pub ownership_states: Vec<OwnershipState>,
    /// Closed contingency-state vocabulary.
    pub contingency_states: Vec<ContingencyState>,
    /// Closed contingency-disposition vocabulary.
    pub contingency_dispositions: Vec<ContingencyDisposition>,
    /// Closed escalation-state vocabulary.
    pub escalation_states: Vec<EscalationState>,
    /// Closed posture vocabulary.
    pub postures: Vec<Posture>,
    /// Closed health-state vocabulary.
    pub health_states: Vec<HealthState>,
    /// Closed health-reason vocabulary.
    pub health_reasons: Vec<HealthReason>,
    /// Closed health-action vocabulary.
    pub health_actions: Vec<HealthAction>,
    /// Stop rules.
    pub rules: Vec<HealthRule>,
    /// Per-upstream records.
    pub records: Vec<UpstreamHealthRecord>,
    /// Cross-cutting scan/surface parity summary.
    pub scan_surface_parity: ScanSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: HealthSummary,
}

impl CriticalUpstreamHealthRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&UpstreamHealthRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the cleared records.
    pub fn records_cleared(&self) -> Vec<&UpstreamHealthRecord> {
        self.records.iter().filter(|r| r.is_cleared()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&UpstreamHealthRecord> {
        self.records
            .iter()
            .filter(|r| r.health_state.is_narrowed())
            .collect()
    }

    /// Returns the records of a given upstream kind.
    pub fn records_of_kind(&self, kind: UpstreamKind) -> Vec<&UpstreamHealthRecord> {
        self.records
            .iter()
            .filter(|r| r.upstream_kind == kind)
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: HealthReason) -> Option<&HealthRule> {
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

    /// Recomputes the offending record ids: promotion-holding records carrying a reason watched
    /// by a firing blocking rule.
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
    pub fn computed_summary(&self) -> HealthSummary {
        let count_state = |state: HealthState| {
            self.records
                .iter()
                .filter(|r| r.health_state == state)
                .count()
        };
        HealthSummary {
            total_records: self.records.len(),
            records_cleared: self.records_cleared().len(),
            records_narrowed: self.records_narrowed().len(),
            state_cleared: count_state(HealthState::Cleared),
            state_narrowed_maintainer: count_state(HealthState::NarrowedMaintainer),
            state_narrowed_security: count_state(HealthState::NarrowedSecurity),
            state_narrowed_cadence: count_state(HealthState::NarrowedCadence),
            state_narrowed_license: count_state(HealthState::NarrowedLicense),
            state_narrowed_ownership: count_state(HealthState::NarrowedOwnership),
            state_narrowed_stale: count_state(HealthState::NarrowedStale),
            state_withdrawn: count_state(HealthState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.health_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            maintainer_gaps: self
                .records
                .iter()
                .filter(|r| r.maintainer.is_degraded())
                .count(),
            security_gaps: self
                .records
                .iter()
                .filter(|r| r.security.is_degraded())
                .count(),
            cadence_gaps: self.records.iter().filter(|r| r.cadence_degraded()).count(),
            license_gaps: self
                .records
                .iter()
                .filter(|r| r.license.clarity.is_degraded())
                .count(),
            ownership_gaps: self
                .records
                .iter()
                .filter(|r| r.is_unowned() || r.contingency_missing() || r.escalation_missing())
                .count(),
            red_risk_total: self.records.iter().filter(|r| r.is_red_risk()).count(),
            unowned_total: self.records.iter().filter(|r| r.is_unowned()).count(),
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
            contingency_plans_recorded: self
                .records
                .iter()
                .filter(|r| r.contingency.plan_state == ContingencyState::Recorded)
                .count(),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, service-health, release-center
    /// publication, support exports, and shiproom panels. It carries only the family, kind,
    /// declared and effective labels, risk grade, state, the scan/surface-agreement flag, the
    /// ownership/contingency/escalation summary, active reasons, and surfaces — never the
    /// detailed scan, review, and proof internals.
    pub fn reuse_projection(&self) -> Vec<UpstreamHealthReuseRow> {
        self.records
            .iter()
            .map(|r| UpstreamHealthReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                upstream_kind: r.upstream_kind,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                risk_grade: r.risk_grade,
                health_state: r.health_state,
                release_blocking: r.release_blocking,
                scan_surface_agree: r.scan_surface_agree(),
                ownership_state: r.ownership.ownership_state,
                contingency_state: r.contingency.plan_state,
                escalation_state: r.escalation.escalation_state,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_CRITICAL_UPSTREAM_HEALTH_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_CRITICAL_UPSTREAM_HEALTH_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every upstream kind must be exercised by at least one record.
        for kind in UpstreamKind::ALL {
            if !self.records.iter().any(|r| r.upstream_kind == kind) {
                v.push(RegisterViolation::UpstreamKindUncovered { kind });
            }
        }

        // Every reason must have a stop rule.
        for reason in HealthReason::ALL {
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
        if self.upstream_kinds != UpstreamKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "upstream_kinds",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.health_grades != HealthGrade::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "health_grades",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.maintainer_ratings != MaintainerRating::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "maintainer_ratings",
            });
        }
        if self.security_postures != SecurityPosture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "security_postures",
            });
        }
        if self.update_cadences != UpdateCadence::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "update_cadences",
            });
        }
        if self.review_cadence_states != ReviewCadenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "review_cadence_states",
            });
        }
        if self.license_clarities != LicenseClarity::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "license_clarities",
            });
        }
        if self.replacement_feasibilities != ReplacementFeasibility::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "replacement_feasibilities",
            });
        }
        if self.ownership_states != OwnershipState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "ownership_states",
            });
        }
        if self.contingency_states != ContingencyState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "contingency_states",
            });
        }
        if self.contingency_dispositions != ContingencyDisposition::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "contingency_dispositions",
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
        if self.health_states != HealthState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "health_states",
            });
        }
        if self.health_reasons != HealthReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "health_reasons",
            });
        }
        if self.health_actions != HealthAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "health_actions",
            });
        }
        if self.health_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "health_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &UpstreamHealthRecord,
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
    /// contradicting fact (a "clear" license with no SPDX id, an "owned" upstream with no owner,
    /// an "abandoned" maintainer with active maintainers, a "recorded" plan with no disposition,
    /// or an escalation/contingency whose applicability disagrees with the risk).
    fn validate_fact_consistency(&self, r: &UpstreamHealthRecord, v: &mut Vec<RegisterViolation>) {
        // clear license ⟺ spdx id present.
        let spdx_present = !r.license.spdx_license_id.trim().is_empty();
        if (r.license.clarity == LicenseClarity::Clear) != spdx_present {
            v.push(RegisterViolation::LicenseFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // owned ⟺ owner present.
        let owned = r.ownership.ownership_state == OwnershipState::Owned;
        let owner_present = !r.ownership.owner_ref.trim().is_empty();
        if owned != owner_present {
            v.push(RegisterViolation::OwnershipFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // abandoned ⟺ no active maintainers, and the bus factor never exceeds the headcount.
        let abandoned = r.maintainer.rating == MaintainerRating::Abandoned;
        if abandoned != (r.maintainer.active_maintainer_count == 0)
            || r.maintainer.bus_factor > r.maintainer.active_maintainer_count
        {
            v.push(RegisterViolation::MaintainerFactInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a contingency plan applies iff the upstream is red-risk.
        let contingency_applies = r.contingency.plan_state != ContingencyState::NotRequired;
        if contingency_applies != r.requires_contingency() {
            v.push(RegisterViolation::ContingencyApplicabilityInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // a settled disposition ⟺ a recorded plan.
        let recorded = r.contingency.plan_state == ContingencyState::Recorded;
        if recorded != r.contingency.disposition.is_settled() {
            v.push(RegisterViolation::ContingencyDispositionInconsistent {
                record_id: r.record_id.clone(),
            });
        }
        // an escalation applies iff the upstream is red-risk or unowned, and the `required` flag
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

    fn validate_controls(&self, r: &UpstreamHealthRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state must
        // equal the state its facts imply — so a control can never assert "satisfied" over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&HealthControl> = r
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
    fn validate_reason_evidence(&self, r: &UpstreamHealthRecord, v: &mut Vec<RegisterViolation>) {
        let maintainer_thinning = r.maintainer.rating.is_thinning();
        let maintainer_abandoned = r.maintainer.rating.is_abandoned();
        let advisories_open = r.security.posture == SecurityPosture::AdvisoriesOpen;
        let unpatched_critical = r.security.posture == SecurityPosture::UnpatchedCritical;
        let cadence_stalled = r.update_cadence.cadence.is_stalled();
        let review_overdue = r.review_cadence.cadence_state.is_overdue();
        let review_missing = r.review_cadence.cadence_state.is_missing();
        let license_ambiguous = r.license.clarity == LicenseClarity::Ambiguous;
        let license_incompatible = r.license.clarity == LicenseClarity::Incompatible;
        let unowned = r.is_unowned();
        let contingency_missing = r.contingency_missing();
        let escalation_missing = r.escalation_missing();
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                HealthReason::MaintainerHealthThinning => maintainer_thinning,
                HealthReason::MaintainerAbandoned => maintainer_abandoned,
                HealthReason::SecurityAdvisoriesOpen => advisories_open,
                HealthReason::SecurityUnpatchedCritical => unpatched_critical,
                HealthReason::UpdateCadenceStalled => cadence_stalled,
                HealthReason::ReviewCadenceOverdue => review_overdue,
                HealthReason::ReviewCadenceMissing => review_missing,
                HealthReason::LicenseAmbiguous => license_ambiguous,
                HealthReason::LicenseIncompatible => license_incompatible,
                HealthReason::UpstreamUnowned => unowned,
                HealthReason::ContingencyPlanMissing => contingency_missing,
                HealthReason::ShiproomEscalationMissing => escalation_missing,
                HealthReason::HealthProofStale => proof_stale,
                HealthReason::HealthProofMissing => proof_missing,
                HealthReason::OwnerSignoffMissing => signoff_missing,
                HealthReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: HealthReason, v: &mut Vec<RegisterViolation>| {
            if present && !r.has_active_reason(reason) {
                v.push(RegisterViolation::GapWithoutReason {
                    record_id: r.record_id.clone(),
                    reason,
                });
            }
        };
        require(
            maintainer_thinning,
            HealthReason::MaintainerHealthThinning,
            v,
        );
        require(maintainer_abandoned, HealthReason::MaintainerAbandoned, v);
        require(advisories_open, HealthReason::SecurityAdvisoriesOpen, v);
        require(
            unpatched_critical,
            HealthReason::SecurityUnpatchedCritical,
            v,
        );
        require(cadence_stalled, HealthReason::UpdateCadenceStalled, v);
        require(review_overdue, HealthReason::ReviewCadenceOverdue, v);
        require(review_missing, HealthReason::ReviewCadenceMissing, v);
        require(license_ambiguous, HealthReason::LicenseAmbiguous, v);
        require(license_incompatible, HealthReason::LicenseIncompatible, v);
        require(unowned, HealthReason::UpstreamUnowned, v);
        require(contingency_missing, HealthReason::ContingencyPlanMissing, v);
        require(
            escalation_missing,
            HealthReason::ShiproomEscalationMissing,
            v,
        );
        require(proof_stale, HealthReason::HealthProofStale, v);
        require(proof_missing, HealthReason::HealthProofMissing, v);
        require(signoff_missing, HealthReason::OwnerSignoffMissing, v);
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a green
    /// surface may never sit over a scan that found an abandoned, unpatched, or unowned upstream.
    fn validate_scan_surface(&self, r: &UpstreamHealthRecord, v: &mut Vec<RegisterViolation>) {
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

    fn validate_state_and_label(&self, r: &UpstreamHealthRecord, v: &mut Vec<RegisterViolation>) {
        // cleared ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_cleared() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::ClearedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.health_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.health_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.health_state,
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
        if r.health_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpstreamHealthReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Upstream kind.
    pub upstream_kind: UpstreamKind,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Risk grade.
    pub risk_grade: HealthGrade,
    /// Health state.
    pub health_state: HealthState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// Ownership posture.
    pub ownership_state: OwnershipState,
    /// Contingency-plan state.
    pub contingency_state: ContingencyState,
    /// Shiproom-escalation state.
    pub escalation_state: EscalationState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<HealthReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the critical-upstream health register.
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
    /// An upstream kind has no record.
    UpstreamKindUncovered {
        /// Uncovered kind.
        kind: UpstreamKind,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: HealthReason,
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
    /// A record's license clarity disagrees with its SPDX id.
    LicenseFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's ownership state disagrees with its owner ref.
    OwnershipFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's maintainer rating disagrees with its maintainer counts.
    MaintainerFactInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's contingency applicability disagrees with its risk grade.
    ContingencyApplicabilityInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's contingency disposition disagrees with its plan state.
    ContingencyDispositionInconsistent {
        /// Record id.
        record_id: String,
    },
    /// A record's escalation applicability disagrees with its risk/ownership.
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
        reason: HealthReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: HealthReason,
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
        declared: HealthState,
        /// Computed state.
        computed: HealthState,
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
            Self::UpstreamKindUncovered { kind } => {
                write!(f, "upstream kind {} has no record", kind.as_str())
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
            Self::LicenseFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} license clarity disagrees with its SPDX id"
                )
            }
            Self::OwnershipFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} ownership state disagrees with its owner ref"
                )
            }
            Self::MaintainerFactInconsistent { record_id } => {
                write!(
                    f,
                    "record {record_id} maintainer rating disagrees with its maintainer counts"
                )
            }
            Self::ContingencyApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} contingency applicability disagrees with its risk grade"
            ),
            Self::ContingencyDispositionInconsistent { record_id } => write!(
                f,
                "record {record_id} contingency disposition disagrees with its plan state"
            ),
            Self::EscalationApplicabilityInconsistent { record_id } => write!(
                f,
                "record {record_id} escalation applicability disagrees with its risk/ownership"
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

/// Loads the embedded critical-upstream health register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`CriticalUpstreamHealthRegister`] — including when a record carries a token outside any
/// closed vocabulary.
pub fn current_m5_critical_upstream_health(
) -> Result<CriticalUpstreamHealthRegister, serde_json::Error> {
    serde_json::from_str(M5_CRITICAL_UPSTREAM_HEALTH_JSON)
}

#[cfg(test)]
mod tests;
