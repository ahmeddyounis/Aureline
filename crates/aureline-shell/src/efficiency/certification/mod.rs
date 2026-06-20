//! Promotion-grade certification of the M5 efficiency-state claim.
//!
//! The parent [`crate::efficiency`] module owns the canonical efficiency-state
//! object model, the [`energy_lab`][crate::efficiency::energy_lab] module captures
//! the over-time energy/thermal traces, and the
//! [`session_pressure`][crate::efficiency::session_pressure] module proves active
//! runs stay correct under pressure. Each of those is *evidence*. This module is
//! the **certification lane** that turns that evidence into a single, inspectable
//! proof packet a release can promote against.
//!
//! The lane answers one question for every **claimed laptop-or-desktop profile**
//! and every **long-running M5 surface family**: can the low-power claim it
//! advertises be backed by *current* efficiency-state behavior, hidden-work
//! suppression, protected-path preservation, and session-aware shedding evidence?
//! For each claimed subject it builds a [`CertificationRow`] that:
//!
//! - runs a fixed [`CertificationDrill`] set against bound evidence, recording one
//!   [`DrillResult`] per drill;
//! - grades the **freshness** of every bound piece of evidence
//!   ([`EvidenceFreshness`]) so a stale, partial, or missing trace can never look
//!   the same as a current one;
//! - fires the [`CertificationNarrowingReason`]s any failed or stale drill implies
//!   and recomputes the row's **effective posture** as the lowest of its published
//!   ceiling and every fired reason's floor; and
//! - records whether the row **blocks promotion** — a claim-bearing row whose
//!   evidence cannot back its ceiling holds the release.
//!
//! The aggregate [`M5EfficiencyProofPacket`] is the canonical, export-safe truth
//! source: release, support, docs, and help consume its rows and its promotion
//! gate instead of cloning a low-power claim. Its vocabulary mirrors the frozen
//! [M5 efficiency-state governance matrix][matrix] so a surface-family row here can
//! never disagree with the matrix row it certifies. The guardrail this lane exists
//! to enforce: a claimed low-power row may not stay green on one good manual test
//! while its current efficiency evidence is stale or incomplete.
//!
//! [matrix]: crate::efficiency::governance::M5_EFFICIENCY_GOVERNANCE_MATRIX_REF

use serde::{Deserialize, Serialize};

use super::energy_lab::{seeded_lab_cases, EfficiencyLabCase, EfficiencyLabTrace, LabProfileClass};
use super::governance::{M5_EFFICIENCY_GOVERNANCE_MATRIX_REF, M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF};
use super::session_pressure::{
    seeded_session_pressure_cases, SessionPressureCase, SessionPressurePosture,
};
use super::surfaces::{EFFICIENCY_DETAILS_SURFACE_REF, EFFICIENCY_INSPECT_COMMAND_ID};
use super::EfficiencyState;

#[cfg(test)]
mod tests;

/// Repo-relative path to the canonical checked-in proof packet.
pub const M5_EFFICIENCY_PROOF_PACKET_REF: &str =
    "artifacts/efficiency/m5-efficiency-proof-packet.json";

/// Repo-relative path to the schema that validates the proof packet.
pub const M5_EFFICIENCY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/efficiency/m5-efficiency-certification.schema.json";

/// Stable record kind for an [`M5EfficiencyProofPacket`] payload.
pub const M5_EFFICIENCY_PROOF_PACKET_RECORD_KIND: &str = "efficiency_m5_proof_packet";

/// Stable record kind for a [`CertificationRow`] payload.
pub const CERTIFICATION_ROW_RECORD_KIND: &str = "efficiency_m5_certification_row";

/// Stable record kind for a [`DrillResult`] payload.
pub const CERTIFICATION_DRILL_RESULT_RECORD_KIND: &str = "efficiency_m5_certification_drill_result";

/// Schema version shared by the proof packet and its rows.
pub const M5_EFFICIENCY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// How many days bound evidence may age before its freshness drops to
/// [`EvidenceFreshness::Stale`]. A stale row may not keep a publishable low-power
/// claim; this is the freshness rule that stops a claim outrunning its evidence.
pub const EVIDENCE_FRESHNESS_WINDOW_DAYS: i64 = 30;

/// The required publication surfaces a certified, claim-bearing row must reach.
pub const REQUIRED_PUBLICATION_SURFACES: [&str; 4] = ["release", "support", "docs", "help"];

/// A claim posture, ordered low to high. Mirrors the frozen governance matrix's
/// claim levels so a certification row's effective posture uses the same
/// vocabulary as the surface row it certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyClaimLevel {
    /// A low-power badge with no materialized evidence. Asserts no claim.
    UndeclaredBadge,
    /// State and source-of-change are materialized, but the claim is not yet
    /// qualified. Not a publishable low-power claim.
    StateDeclared,
    /// A claim-bearing posture: state, shed work, hidden-work suppression, and
    /// protected paths are all qualified under pressure.
    QualifiedLowPower,
    /// The highest claim-bearing posture: qualified plus policy-aware override,
    /// staged recovery, and propagation to every required surface.
    CertifiedLowPower,
}

impl EfficiencyClaimLevel {
    /// Every claim level, lowest rank first.
    pub const ALL: [Self; 4] = [
        Self::UndeclaredBadge,
        Self::StateDeclared,
        Self::QualifiedLowPower,
        Self::CertifiedLowPower,
    ];

    /// Stable token recorded in the proof packet and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UndeclaredBadge => "undeclared_badge",
            Self::StateDeclared => "state_declared",
            Self::QualifiedLowPower => "qualified_low_power",
            Self::CertifiedLowPower => "certified_low_power",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::UndeclaredBadge => "Undeclared badge",
            Self::StateDeclared => "State declared",
            Self::QualifiedLowPower => "Qualified low-power",
            Self::CertifiedLowPower => "Certified low-power",
        }
    }

    /// Integer rank used to compare and narrow postures.
    pub const fn rank(self) -> u8 {
        match self {
            Self::UndeclaredBadge => 0,
            Self::StateDeclared => 1,
            Self::QualifiedLowPower => 2,
            Self::CertifiedLowPower => 3,
        }
    }

    /// True when the level asserts a publishable low-power claim.
    pub const fn is_claim_bearing(self) -> bool {
        matches!(self, Self::QualifiedLowPower | Self::CertifiedLowPower)
    }

    /// One-line description for the self-describing vocabulary block.
    pub const fn description(self) -> &'static str {
        match self {
            Self::UndeclaredBadge => {
                "A low-power badge with no materialized efficiency-state evidence. Asserts no claim and is retained only for diagnosis."
            }
            Self::StateDeclared => {
                "The efficiency state and source-of-change are materialized, but hidden-work suppression or protected-path preservation is not qualified. Not a publishable low-power claim."
            }
            Self::QualifiedLowPower => {
                "A claim-bearing posture: the subject materializes its state, names shed work, suppresses hidden-pane render and polling, and preserves protected paths under pressure."
            }
            Self::CertifiedLowPower => {
                "The highest claim-bearing posture: qualified low-power plus policy-aware override, staged recovery, and propagation to every required release, support, docs, and help surface."
            }
        }
    }

    /// Returns the lower-ranked of two levels.
    fn min(self, other: Self) -> Self {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    /// Resolves a stable token back into its level, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|level| level.as_str() == token)
    }
}

/// The kind of subject a certification row covers. The lane certifies along two
/// axes the spec keeps separate: claimed hardware/deployment profiles and the
/// long-running M5 surface families.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedSubjectKind {
    /// A claimed laptop or desktop hardware/deployment profile.
    LaptopOrDesktopProfile,
    /// A long-running M5 surface family.
    M5SurfaceFamily,
}

impl CertifiedSubjectKind {
    /// Stable token recorded in rows and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaptopOrDesktopProfile => "laptop_or_desktop_profile",
            Self::M5SurfaceFamily => "m5_surface_family",
        }
    }
}

/// The kind of evidence a drill binds to. Naming the kind keeps a drill traceable
/// to the exact canonical artifact that backs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationEvidenceKind {
    /// An energy/thermal lab trace.
    EnergyThermalTrace,
    /// A hidden-pane render audit embedded in a lab trace.
    HiddenPaneAudit,
    /// An active-session pressure posture.
    SessionPressurePosture,
}

impl CertificationEvidenceKind {
    /// Every evidence kind, in canonical order.
    pub const ALL: [Self; 3] = [
        Self::EnergyThermalTrace,
        Self::HiddenPaneAudit,
        Self::SessionPressurePosture,
    ];

    /// Stable token recorded in drill results.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnergyThermalTrace => "energy_thermal_trace",
            Self::HiddenPaneAudit => "hidden_pane_audit",
            Self::SessionPressurePosture => "session_pressure_posture",
        }
    }

    /// One-line description for the vocabulary block.
    pub const fn description(self) -> &'static str {
        match self {
            Self::EnergyThermalTrace => {
                "An over-time energy/thermal lab trace driving the canonical runtime through battery and thermal transitions."
            }
            Self::HiddenPaneAudit => {
                "A hidden-pane render audit proving hidden, occluded, or off-screen panes commit no paint and pause polling and animation."
            }
            Self::SessionPressurePosture => {
                "An active-session pressure posture proving optional work sheds before any live run's correctness or authority regresses."
            }
        }
    }
}

/// One certification drill: a verification class every claimed subject must
/// survive against current evidence. Each drill names what it proves and the
/// evidence kind it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDrill {
    /// Efficiency-state transitions are materialized, current, and explained.
    EfficiencyStateBehavior,
    /// Hidden, occluded, or off-screen panes suppress render, polling, and motion.
    HiddenWorkSuppression,
    /// Active tasks, debug, local save, navigation, and review stay protected.
    ProtectedPathPreservation,
    /// Optional work sheds first; live runs stay correct and warned before downgrade.
    SessionAwareShedding,
    /// Deferred work resumes in staged order once pressure clears.
    StagedRecovery,
}

impl CertificationDrill {
    /// Every drill, in canonical order.
    pub const ALL: [Self; 5] = [
        Self::EfficiencyStateBehavior,
        Self::HiddenWorkSuppression,
        Self::ProtectedPathPreservation,
        Self::SessionAwareShedding,
        Self::StagedRecovery,
    ];

    /// Stable token recorded in drill results.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EfficiencyStateBehavior => "efficiency_state_behavior",
            Self::HiddenWorkSuppression => "hidden_work_suppression",
            Self::ProtectedPathPreservation => "protected_path_preservation",
            Self::SessionAwareShedding => "session_aware_shedding",
            Self::StagedRecovery => "staged_recovery",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EfficiencyStateBehavior => "Efficiency-state behavior",
            Self::HiddenWorkSuppression => "Hidden-work suppression",
            Self::ProtectedPathPreservation => "Protected-path preservation",
            Self::SessionAwareShedding => "Session-aware shedding",
            Self::StagedRecovery => "Staged recovery",
        }
    }

    /// What surviving this drill proves about the claimed subject.
    pub const fn proves(self) -> &'static str {
        match self {
            Self::EfficiencyStateBehavior => {
                "The subject materializes inspectable efficiency-state transitions with a named state, source-of-change, and a recorded reason for every reduced surface."
            }
            Self::HiddenWorkSuppression => {
                "Hidden, occluded, and off-screen panes commit no render work and pause nonessential polling and animation."
            }
            Self::ProtectedPathPreservation => {
                "Active tasks, debug correctness, local save, navigation, and review authority stay protected at every step under pressure."
            }
            Self::SessionAwareShedding => {
                "Optional assists shed before any live run's correctness or authority regresses, and a material downgrade is warned about before it applies."
            }
            Self::StagedRecovery => {
                "When pressure clears, deferred work resumes in staged order rather than thrashing back at once."
            }
        }
    }

    /// The evidence kind this drill reads.
    pub const fn evidence_kind(self) -> CertificationEvidenceKind {
        match self {
            Self::EfficiencyStateBehavior
            | Self::ProtectedPathPreservation
            | Self::StagedRecovery => CertificationEvidenceKind::EnergyThermalTrace,
            Self::HiddenWorkSuppression => CertificationEvidenceKind::HiddenPaneAudit,
            Self::SessionAwareShedding => CertificationEvidenceKind::SessionPressurePosture,
        }
    }

    /// The narrowing reason a *failed* (but present and current) drill fires.
    pub const fn failure_reason(self) -> CertificationNarrowingReason {
        match self {
            Self::EfficiencyStateBehavior => {
                CertificationNarrowingReason::MissingEfficiencyEvidence
            }
            Self::HiddenWorkSuppression => {
                CertificationNarrowingReason::UnqualifiedHiddenWorkSuppression
            }
            Self::ProtectedPathPreservation => {
                CertificationNarrowingReason::ProtectedPathRegressionUnderPressure
            }
            Self::SessionAwareShedding => CertificationNarrowingReason::SessionShedOrderViolation,
            Self::StagedRecovery => CertificationNarrowingReason::RecoveryNotStaged,
        }
    }

    /// Resolves a stable token back into its drill, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|drill| drill.as_str() == token)
    }
}

/// Freshness grade of one bound piece of evidence relative to the proof packet's
/// `as_of`. The lane treats stale, partial, and missing evidence as distinct
/// failures so a claim can never coast on an old measurement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Evidence is present and within the freshness window.
    Current,
    /// Evidence is present but older than the freshness window.
    Stale,
    /// Some required evidence for the subject is present, but not this drill's.
    Partial,
    /// No evidence backs this drill.
    Missing,
}

impl EvidenceFreshness {
    /// Every freshness grade, in canonical order.
    pub const ALL: [Self; 4] = [Self::Current, Self::Stale, Self::Partial, Self::Missing];

    /// Stable token recorded in drill results.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::Missing => "missing",
        }
    }

    /// True only for evidence that may back a publishable claim.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }

    /// The narrowing reason this freshness grade fires, if any.
    pub const fn narrowing_reason(self) -> Option<CertificationNarrowingReason> {
        match self {
            Self::Current => None,
            Self::Stale => Some(CertificationNarrowingReason::StaleEfficiencyEvidence),
            Self::Partial => Some(CertificationNarrowingReason::PartialEvidenceCoverage),
            Self::Missing => Some(CertificationNarrowingReason::MissingEfficiencyEvidence),
        }
    }
}

/// A mechanically detectable reason a certification row's claim narrows. Every
/// reason names the posture floor it narrows the row to, so the effective posture
/// is the lowest of the published ceiling and each fired reason's floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationNarrowingReason {
    /// No current evidence backs a required drill.
    MissingEfficiencyEvidence,
    /// Bound evidence is older than the freshness window.
    StaleEfficiencyEvidence,
    /// Only part of the required evidence set is present.
    PartialEvidenceCoverage,
    /// A hidden or off-screen pane could not prove qualified render/poll suppression.
    UnqualifiedHiddenWorkSuppression,
    /// A protected interaction regressed under battery or thermal pressure.
    ProtectedPathRegressionUnderPressure,
    /// Optional work did not shed before a live run regressed, or a downgrade was unwarned.
    SessionShedOrderViolation,
    /// Deferred work did not resume in staged order after pressure cleared.
    RecoveryNotStaged,
}

impl CertificationNarrowingReason {
    /// Every narrowing reason, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::MissingEfficiencyEvidence,
        Self::StaleEfficiencyEvidence,
        Self::PartialEvidenceCoverage,
        Self::UnqualifiedHiddenWorkSuppression,
        Self::ProtectedPathRegressionUnderPressure,
        Self::SessionShedOrderViolation,
        Self::RecoveryNotStaged,
    ];

    /// Stable token recorded in rows and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingEfficiencyEvidence => "missing_efficiency_evidence",
            Self::StaleEfficiencyEvidence => "stale_efficiency_evidence",
            Self::PartialEvidenceCoverage => "partial_evidence_coverage",
            Self::UnqualifiedHiddenWorkSuppression => "unqualified_hidden_work_suppression",
            Self::ProtectedPathRegressionUnderPressure => {
                "protected_path_regression_under_pressure"
            }
            Self::SessionShedOrderViolation => "session_shed_order_violation",
            Self::RecoveryNotStaged => "recovery_not_staged",
        }
    }

    /// The posture floor this reason narrows a row to.
    pub const fn narrows_to(self) -> EfficiencyClaimLevel {
        match self {
            Self::MissingEfficiencyEvidence => EfficiencyClaimLevel::UndeclaredBadge,
            Self::StaleEfficiencyEvidence
            | Self::PartialEvidenceCoverage
            | Self::UnqualifiedHiddenWorkSuppression
            | Self::ProtectedPathRegressionUnderPressure
            | Self::SessionShedOrderViolation => EfficiencyClaimLevel::StateDeclared,
            Self::RecoveryNotStaged => EfficiencyClaimLevel::QualifiedLowPower,
        }
    }

    /// What the reason detects.
    pub const fn detects(self) -> &'static str {
        match self {
            Self::MissingEfficiencyEvidence => {
                "A required drill has no current efficiency-state, hidden-pane, or session evidence bound to it."
            }
            Self::StaleEfficiencyEvidence => {
                "Bound evidence is older than the freshness window, so the claim would outrun its measurement."
            }
            Self::PartialEvidenceCoverage => {
                "The subject binds some required evidence but not every drill's, so coverage is incomplete."
            }
            Self::UnqualifiedHiddenWorkSuppression => {
                "A hidden, occluded, or off-screen pane committed paint or kept polling under pressure."
            }
            Self::ProtectedPathRegressionUnderPressure => {
                "A protected interaction or durability invariant regressed under battery or thermal pressure."
            }
            Self::SessionShedOrderViolation => {
                "A live run lost correctness or authority before its optional work shed, or a material downgrade applied without a warning."
            }
            Self::RecoveryNotStaged => {
                "Deferred work resumed without a staged-recovery transition after pressure cleared."
            }
        }
    }

    /// The fail-closed stop rule.
    pub const fn stop_rule(self) -> &'static str {
        match self {
            Self::MissingEfficiencyEvidence => {
                "Quarantine the row to the undeclared-badge floor; it may assert no low-power claim without current evidence."
            }
            Self::StaleEfficiencyEvidence => {
                "Narrow below a publishable claim until fresh evidence is captured; an old measurement does not certify a current claim."
            }
            Self::PartialEvidenceCoverage => {
                "Narrow below a publishable claim until every required drill is backed by current evidence."
            }
            Self::UnqualifiedHiddenWorkSuppression => {
                "Narrow below a publishable claim until hidden-pane suppression is qualified under pressure."
            }
            Self::ProtectedPathRegressionUnderPressure => {
                "Narrow below a publishable claim until protected paths are preserved under pressure."
            }
            Self::SessionShedOrderViolation => {
                "Narrow below a publishable claim until optional work demonstrably sheds first and downgrades are warned."
            }
            Self::RecoveryNotStaged => {
                "Narrow to qualified low-power until staged recovery is proven."
            }
        }
    }

    /// Always true: every certification narrowing reason is mechanically detected.
    pub const fn auto_detectable(self) -> bool {
        true
    }

    /// Resolves a stable token back into its reason, if known.
    pub fn from_token(token: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|reason| reason.as_str() == token)
    }
}

/// The certification outcome of a row, derived from the firing narrowing reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationState {
    /// Every drill passed against current evidence; effective posture equals ceiling.
    Certified,
    /// At least one reason fired; the effective posture is below the ceiling but
    /// still names a state.
    Narrowed,
    /// The row narrowed to the undeclared-badge floor; it asserts no claim.
    Quarantined,
}

impl CertificationState {
    /// Every certification state, in canonical order.
    pub const ALL: [Self; 3] = [Self::Certified, Self::Narrowed, Self::Quarantined];

    /// Stable token recorded in rows and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Quarantined => "quarantined",
        }
    }

    /// True only for a fully certified row.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::Certified)
    }

    /// One-line description for the vocabulary block.
    pub const fn description(self) -> &'static str {
        match self {
            Self::Certified => {
                "Every drill passed against current evidence and the effective posture equals the published ceiling."
            }
            Self::Narrowed => {
                "At least one narrowing reason fired; the effective posture is below the published ceiling. A claim-bearing narrowed row holds promotion."
            }
            Self::Quarantined => {
                "The row narrowed to the undeclared-badge floor. It asserts no low-power claim and is retained only for diagnosis."
            }
        }
    }

    /// Whether a row in this state holds promotion when its published ceiling is
    /// claim-bearing.
    pub const fn blocks_when_claim_bearing(self) -> bool {
        !matches!(self, Self::Certified)
    }

    /// Derives the state from the fired reasons and the effective posture.
    fn derive(fired: &[CertificationNarrowingReason], effective: EfficiencyClaimLevel) -> Self {
        if fired.is_empty() {
            Self::Certified
        } else if effective == EfficiencyClaimLevel::UndeclaredBadge {
            Self::Quarantined
        } else {
            Self::Narrowed
        }
    }
}

/// The result of running one drill against bound evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillResult {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Drill token.
    pub drill: String,
    /// Drill label.
    pub drill_label: String,
    /// What surviving the drill proves.
    pub proves: String,
    /// Evidence-kind token the drill read.
    pub evidence_kind: String,
    /// Evidence references the drill bound to (trace ids, posture ids).
    pub evidence_refs: Vec<String>,
    /// The `as_of` timestamp of the bound evidence, or empty when missing.
    pub evidence_as_of: String,
    /// Freshness grade of the bound evidence.
    pub freshness: String,
    /// Outcome token (`pass`, `fail`, `stale`, `partial`, `missing`).
    pub outcome: String,
    /// True when the drill passed against current evidence.
    pub passed: bool,
    /// The narrowing reason the drill fired, when it did not pass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// Content-free detail naming what the drill observed.
    pub detail: String,
}

/// Whether and why a certification row holds promotion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionBlocker {
    /// True when the row holds the release.
    pub blocks_promotion: bool,
    /// The reasons the row blocks, if any.
    pub blocker_reasons: Vec<String>,
    /// The effective posture label.
    pub posture_label: String,
}

/// One certification row: a claimed laptop/desktop profile or M5 surface family,
/// the drills it ran, the evidence freshness, and the recomputed certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Subject-kind token.
    pub subject_kind: String,
    /// Subject token (profile or surface family).
    pub subject_token: String,
    /// Human-readable subject label.
    pub subject_label: String,
    /// Claimed efficiency-state tokens the subject is certified across.
    pub claimed_efficiency_states: Vec<String>,
    /// The highest posture this row may publish.
    pub published_claim_ceiling: String,
    /// The drills this row is required to survive.
    pub required_drills: Vec<String>,
    /// One result per required drill.
    pub drill_results: Vec<DrillResult>,
    /// The narrowing reasons that fired, deduplicated and in canonical order.
    pub fired_narrowing_reasons: Vec<String>,
    /// The narrowed effective posture after governance.
    pub effective_posture: String,
    /// The certification state derived from the fired reasons.
    pub certification_state: String,
    /// Whether and why the row holds promotion.
    pub promotion_blocker: PromotionBlocker,
    /// True when every protected-path drill passed.
    pub protected_paths_preserved: bool,
    /// True when hidden-work suppression was proven.
    pub hidden_work_suppressed: bool,
    /// True when optional work demonstrably sheds before a live run regresses.
    pub optional_work_sheds_first: bool,
    /// The worst (lowest) freshness across the row's drills.
    pub evidence_freshness: String,
    /// The newest bound-evidence `as_of` the row certifies against, or empty.
    pub evidence_as_of: String,
    /// The surfaces this row publishes its claim to when certified.
    pub publication_targets: Vec<String>,
    /// The governance matrix row this certification aligns with, when applicable.
    pub governance_row_ref: String,
}

impl CertificationRow {
    /// True when this row is fully certified.
    pub fn is_certified(&self) -> bool {
        self.certification_state == CertificationState::Certified.as_str()
    }

    /// True when this row holds promotion.
    pub fn blocks_promotion(&self) -> bool {
        self.promotion_blocker.blocks_promotion
    }
}

/// The release promotion decision the proof packet resolves to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPromotionGate {
    /// Decision token (`proceed` or `hold`).
    pub decision: String,
    /// Row ids that hold promotion.
    pub blocking_row_ids: Vec<String>,
    /// Reasons promotion is held.
    pub blocking_reasons: Vec<String>,
    /// How the gate is computed.
    pub rationale: String,
}

/// A binding declaring how a downstream surface consumes the proof packet rather
/// than cloning a low-power claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationPublicationBinding {
    /// The consuming surface.
    pub surface: String,
    /// The projection it ingests.
    pub projection: String,
    /// What it ingests, in one line.
    pub ingests: String,
}

/// Self-describing vocabulary declaration for a claim level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimLevelDeclaration {
    /// Level token.
    pub level: String,
    /// Integer rank.
    pub rank: u8,
    /// Whether the level asserts a publishable claim.
    pub claim_bearing: bool,
    /// Human-readable label.
    pub label: String,
    /// One-line description.
    pub description: String,
}

/// Self-describing vocabulary declaration for a drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillDeclaration {
    /// Drill token.
    pub drill: String,
    /// Human-readable label.
    pub label: String,
    /// What surviving the drill proves.
    pub proves: String,
    /// The evidence kind it reads.
    pub evidence_kind: String,
    /// The reason a failed drill fires.
    pub failure_reason: String,
}

/// Self-describing vocabulary declaration for an evidence kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceKindDeclaration {
    /// Evidence-kind token.
    pub kind: String,
    /// One-line description.
    pub description: String,
}

/// Self-describing vocabulary declaration for a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowingReasonDeclaration {
    /// Reason token.
    pub reason: String,
    /// The posture floor it narrows to.
    pub narrows_to: String,
    /// What it detects.
    pub detects: String,
    /// Always true.
    pub auto_detectable: bool,
    /// The fail-closed stop rule.
    pub stop_rule: String,
}

/// Self-describing vocabulary declaration for a certification state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationStateDeclaration {
    /// State token.
    pub state: String,
    /// Whether the state is fully certified.
    pub is_certified: bool,
    /// Whether the state holds promotion when claim-bearing.
    pub blocks_when_claim_bearing: bool,
    /// One-line description.
    pub description: String,
}

/// How a reviewer recomputes and inspects the proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationInspection {
    /// How to recompute every row.
    pub how_to_recompute: String,
    /// The freshness rule.
    pub freshness_rule: String,
    /// The promotion rule.
    pub promotion_rule: String,
    /// The governance matrix the surface-family rows align with.
    pub governance_matrix_ref: String,
}

/// Aggregate counts over the proof packet's rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummaryCounts {
    /// Total rows.
    pub total_rows: usize,
    /// Rows certified.
    pub rows_certified: usize,
    /// Rows narrowed.
    pub rows_narrowed: usize,
    /// Rows quarantined.
    pub rows_quarantined: usize,
    /// Profile rows.
    pub profile_rows: usize,
    /// Surface-family rows.
    pub surface_family_rows: usize,
    /// Rows whose published ceiling is claim-bearing.
    pub claim_bearing_rows: usize,
    /// Rows holding promotion.
    pub rows_blocking_promotion: usize,
    /// Covered laptop/desktop profile tokens.
    pub covered_profiles: Vec<String>,
    /// Covered surface-family tokens.
    pub covered_surface_families: Vec<String>,
}

/// The canonical, export-safe M5 efficiency proof packet.
///
/// This is the truth source release, support, docs, and help consume for the
/// low-power claim. Build it with [`certify_m5_efficiency`] from current evidence,
/// or [`seeded_proof_packet`] for the checked-in fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EfficiencyProofPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// The date the packet certifies against (`YYYY-MM-DD`).
    pub as_of: String,
    /// Reviewable title.
    pub title: String,
    /// One-paragraph summary.
    pub summary: String,
    /// Canonical governance matrix the surface-family rows align with.
    pub matrix_ref: String,
    /// Schema validating the governance matrix.
    pub matrix_schema_ref: String,
    /// Schema validating this proof packet.
    pub schema_ref: String,
    /// Source artifacts and code this packet derives from.
    pub source_refs: Vec<String>,
    /// Days bound evidence may age before it is stale.
    pub evidence_freshness_window_days: i64,
    /// Declared claim-level vocabulary.
    pub claim_levels: Vec<ClaimLevelDeclaration>,
    /// Declared drill vocabulary.
    pub drills: Vec<DrillDeclaration>,
    /// Declared evidence-kind vocabulary.
    pub evidence_kinds: Vec<EvidenceKindDeclaration>,
    /// Declared narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReasonDeclaration>,
    /// Declared certification-state vocabulary.
    pub certification_states: Vec<CertificationStateDeclaration>,
    /// The surfaces a certified claim-bearing row must reach.
    pub required_publication_surfaces: Vec<String>,
    /// The certification rows.
    pub rows: Vec<CertificationRow>,
    /// The recomputed promotion gate.
    pub promotion_gate: CertificationPromotionGate,
    /// How downstream surfaces consume this packet.
    pub publication_bindings: Vec<CertificationPublicationBinding>,
    /// Aggregate counts.
    pub summary_counts: CertificationSummaryCounts,
    /// How to recompute and inspect the packet.
    pub inspection: CertificationInspection,
}

impl M5EfficiencyProofPacket {
    /// True when the packet resolves to a `proceed` promotion decision.
    pub fn promotion_proceeds(&self) -> bool {
        self.promotion_gate.decision == "proceed"
    }

    /// True when no claim-bearing row's effective posture fell below its ceiling.
    pub fn no_claim_outruns_evidence(&self) -> bool {
        self.rows.iter().all(|row| !row.blocks_promotion())
    }

    /// Returns the row with the given id, if present.
    pub fn row(&self, row_id: &str) -> Option<&CertificationRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }
}

/// A declared certification subject: what is claimed and which evidence backs it.
/// The builder turns one of these into a [`CertificationRow`] against the supplied
/// evidence.
#[derive(Debug, Clone)]
pub struct CertificationSubject {
    /// Stable row id.
    pub row_id: String,
    /// Subject kind.
    pub subject_kind: CertifiedSubjectKind,
    /// Subject token.
    pub subject_token: String,
    /// Subject label.
    pub subject_label: String,
    /// Claimed efficiency states.
    pub claimed_states: Vec<EfficiencyState>,
    /// The highest posture this subject may publish.
    pub published_claim_ceiling: EfficiencyClaimLevel,
    /// The drills the subject must survive.
    pub required_drills: Vec<CertificationDrill>,
    /// The energy/thermal lab profile whose trace backs the trace-bound drills.
    pub lab_profile: Option<LabProfileClass>,
    /// The session-pressure case id whose posture backs the session drill.
    pub session_case_id: Option<String>,
    /// The governance matrix row this subject aligns with, when applicable.
    pub governance_row_ref: String,
}

/// Builds a proof packet by running every declared subject's drills against the
/// supplied lab and session evidence and recomputing the promotion gate.
pub fn certify_m5_efficiency(
    packet_id: &str,
    as_of: &str,
    generated_at: &str,
    subjects: &[CertificationSubject],
    lab_cases: &[EfficiencyLabCase],
    session_cases: &[SessionPressureCase],
) -> M5EfficiencyProofPacket {
    let rows = subjects
        .iter()
        .map(|subject| build_row(subject, as_of, lab_cases, session_cases))
        .collect::<Vec<_>>();

    let blocking = rows
        .iter()
        .filter(|row| row.blocks_promotion())
        .collect::<Vec<_>>();
    let promotion_gate = CertificationPromotionGate {
        decision: if blocking.is_empty() { "proceed" } else { "hold" }.to_owned(),
        blocking_row_ids: blocking.iter().map(|row| row.row_id.clone()).collect(),
        blocking_reasons: blocking
            .iter()
            .flat_map(|row| row.promotion_blocker.blocker_reasons.clone())
            .collect(),
        rationale: "Promotion holds when any row whose published ceiling is claim-bearing narrows below that ceiling because a required drill failed or its evidence is stale, partial, or missing."
            .to_owned(),
    };

    let summary_counts = summarize(&rows);

    M5EfficiencyProofPacket {
        record_kind: M5_EFFICIENCY_PROOF_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_EFFICIENCY_CERTIFICATION_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: generated_at.to_owned(),
        as_of: as_of.to_owned(),
        title: "M5 efficiency certification proof packet for every claimed laptop/desktop profile and long-running surface family.".to_owned(),
        summary: "Each claimed laptop-or-desktop profile and long-running M5 surface family runs a fixed drill set — efficiency-state behavior, hidden-work suppression, protected-path preservation, session-aware shedding, and staged recovery — against current energy/thermal traces, hidden-pane audits, and session-pressure postures. Stale, partial, or missing evidence and any failed drill narrow the row's claim and fire the promotion gate, so a low-power claim can never outrun the evidence behind it. Release, support, docs, and help consume this packet instead of cloning a low-power claim.".to_owned(),
        matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
        matrix_schema_ref: M5_EFFICIENCY_GOVERNANCE_SCHEMA_REF.to_owned(),
        schema_ref: M5_EFFICIENCY_CERTIFICATION_SCHEMA_REF.to_owned(),
        source_refs: vec![
            M5_EFFICIENCY_CERTIFICATION_SCHEMA_REF.to_owned(),
            M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
            "artifacts/efficiency/m5-efficiency-traces".to_owned(),
            "crates/aureline-shell/src/efficiency/certification/mod.rs".to_owned(),
            "crates/aureline-shell/src/efficiency/energy_lab/mod.rs".to_owned(),
            "crates/aureline-shell/src/efficiency/session_pressure/mod.rs".to_owned(),
        ],
        evidence_freshness_window_days: EVIDENCE_FRESHNESS_WINDOW_DAYS,
        claim_levels: EfficiencyClaimLevel::ALL
            .iter()
            .map(|level| ClaimLevelDeclaration {
                level: level.as_str().to_owned(),
                rank: level.rank(),
                claim_bearing: level.is_claim_bearing(),
                label: level.label().to_owned(),
                description: level.description().to_owned(),
            })
            .collect(),
        drills: CertificationDrill::ALL
            .iter()
            .map(|drill| DrillDeclaration {
                drill: drill.as_str().to_owned(),
                label: drill.label().to_owned(),
                proves: drill.proves().to_owned(),
                evidence_kind: drill.evidence_kind().as_str().to_owned(),
                failure_reason: drill.failure_reason().as_str().to_owned(),
            })
            .collect(),
        evidence_kinds: CertificationEvidenceKind::ALL
            .iter()
            .map(|kind| EvidenceKindDeclaration {
                kind: kind.as_str().to_owned(),
                description: kind.description().to_owned(),
            })
            .collect(),
        narrowing_reasons: CertificationNarrowingReason::ALL
            .iter()
            .map(|reason| NarrowingReasonDeclaration {
                reason: reason.as_str().to_owned(),
                narrows_to: reason.narrows_to().as_str().to_owned(),
                detects: reason.detects().to_owned(),
                auto_detectable: reason.auto_detectable(),
                stop_rule: reason.stop_rule().to_owned(),
            })
            .collect(),
        certification_states: CertificationState::ALL
            .iter()
            .map(|state| CertificationStateDeclaration {
                state: state.as_str().to_owned(),
                is_certified: state.is_certified(),
                blocks_when_claim_bearing: state.blocks_when_claim_bearing(),
                description: state.description().to_owned(),
            })
            .collect(),
        required_publication_surfaces: REQUIRED_PUBLICATION_SURFACES
            .iter()
            .map(|surface| (*surface).to_owned())
            .collect(),
        rows,
        promotion_gate,
        publication_bindings: publication_bindings(),
        summary_counts,
        inspection: CertificationInspection {
            how_to_recompute: "For each row, grade every required drill's evidence freshness against the packet as_of, run the drill predicate on current evidence, fire the implied narrowing reasons, set the effective posture to the lowest of the published ceiling and each fired reason's floor, derive the certification state, then resolve the promotion gate.".to_owned(),
            freshness_rule: format!(
                "Bound evidence is current within {EVIDENCE_FRESHNESS_WINDOW_DAYS} days of the packet as_of; older evidence is stale, an absent required-drill binding is partial when other evidence exists and missing otherwise."
            ),
            promotion_rule: "Promotion proceeds unless a row whose published ceiling is claim-bearing narrows below that ceiling.".to_owned(),
            governance_matrix_ref: M5_EFFICIENCY_GOVERNANCE_MATRIX_REF.to_owned(),
        },
    }
}

/// Builds the canonical proof packet from the seeded lab and session evidence.
pub fn seeded_proof_packet() -> M5EfficiencyProofPacket {
    let lab_cases = seeded_lab_cases();
    let session_cases = seeded_session_pressure_cases();
    let subjects = seeded_certification_subjects();
    certify_m5_efficiency(
        "aureline.efficiency.m5_certification",
        "2026-06-20",
        "2026-06-20T14:30:00Z",
        &subjects,
        &lab_cases,
        &session_cases,
    )
}

/// The canonical certification subjects: every claimed laptop/desktop profile and
/// long-running M5 surface family, with the ceiling each claims and the evidence
/// that backs it. The companion-adjacent surface family is a worked example of the
/// guardrail: it binds no efficiency-state evidence, so it quarantines.
pub fn seeded_certification_subjects() -> Vec<CertificationSubject> {
    use CertificationDrill as Drill;
    use EfficiencyClaimLevel as Level;
    use EfficiencyState as State;

    let all_drills = CertificationDrill::ALL.to_vec();
    let no_recovery = vec![
        Drill::EfficiencyStateBehavior,
        Drill::HiddenWorkSuppression,
        Drill::ProtectedPathPreservation,
        Drill::SessionAwareShedding,
    ];

    let profile = |row_id: &str,
                   token: &str,
                   label: &str,
                   states: Vec<State>,
                   ceiling: Level,
                   drills: Vec<Drill>,
                   lab: LabProfileClass,
                   session: &str|
     -> CertificationSubject {
        CertificationSubject {
            row_id: row_id.to_owned(),
            subject_kind: CertifiedSubjectKind::LaptopOrDesktopProfile,
            subject_token: token.to_owned(),
            subject_label: label.to_owned(),
            claimed_states: states,
            published_claim_ceiling: ceiling,
            required_drills: drills,
            lab_profile: Some(lab),
            session_case_id: Some(session.to_owned()),
            governance_row_ref: String::new(),
        }
    };

    let surface = |row_id: &str,
                   token: &str,
                   label: &str,
                   states: Vec<State>,
                   ceiling: Level,
                   drills: Vec<Drill>,
                   lab: Option<LabProfileClass>,
                   session: Option<&str>,
                   gov: &str|
     -> CertificationSubject {
        CertificationSubject {
            row_id: row_id.to_owned(),
            subject_kind: CertifiedSubjectKind::M5SurfaceFamily,
            subject_token: token.to_owned(),
            subject_label: label.to_owned(),
            claimed_states: states,
            published_claim_ceiling: ceiling,
            required_drills: drills,
            lab_profile: lab,
            session_case_id: session.map(|id| id.to_owned()),
            governance_row_ref: gov.to_owned(),
        }
    };

    vec![
        // Claimed laptop/desktop profiles.
        profile(
            "cert.profile.battery_ultrabook",
            "battery_ultrabook",
            "Battery ultrabook (laptop)",
            vec![State::EfficiencyAware, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            LabProfileClass::BatteryUltrabook,
            "battery-saver",
        ),
        profile(
            "cert.profile.thermal_workstation",
            "thermal_workstation",
            "Thermal workstation (desktop)",
            vec![State::ThermalConstrained, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            LabProfileClass::ThermalWorkstation,
            "thermal",
        ),
        profile(
            "cert.profile.policy_managed_fleet",
            "policy_managed_fleet",
            "Policy-managed fleet (desktop)",
            vec![State::EfficiencyAware],
            Level::QualifiedLowPower,
            no_recovery.clone(),
            LabProfileClass::PolicyManagedFleet,
            "policy-cap",
        ),
        profile(
            "cert.profile.critical_battery_field",
            "critical_battery_field",
            "Critical-battery field laptop",
            vec![State::EfficiencyAware, State::ProtectCore, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            LabProfileClass::CriticalBatteryField,
            "critical-battery",
        ),
        // Long-running M5 surface families, aligned with the governance matrix.
        surface(
            "cert.surface.notebooks",
            "notebooks",
            "Notebook cell and output panes",
            vec![State::ThermalConstrained, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::ThermalWorkstation),
            Some("thermal"),
            "eff.notebooks.thermal",
        ),
        surface(
            "cert.surface.previews",
            "previews",
            "Preview and embedded browser-runtime panes",
            vec![State::EfficiencyAware, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::BatteryUltrabook),
            Some("battery-saver"),
            "eff.previews.battery_saver",
        ),
        surface(
            "cert.surface.docs_browser_panes",
            "docs_browser_panes",
            "Docs and embedded browser panes",
            vec![State::ProtectCore, State::Recovery],
            Level::QualifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::CriticalBatteryField),
            Some("critical-battery"),
            "eff.docs_browser.critical_battery",
        ),
        surface(
            "cert.surface.traces",
            "traces",
            "Trace, profiler, and timeline panes",
            vec![State::ThermalConstrained, State::Recovery],
            Level::QualifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::ThermalWorkstation),
            Some("thermal"),
            "eff.traces.thermal",
        ),
        surface(
            "cert.surface.pipelines",
            "pipelines",
            "Pipeline, task, and run panes",
            vec![State::EfficiencyAware, State::ProtectCore, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::CriticalBatteryField),
            Some("critical-battery"),
            "eff.pipelines.low_battery",
        ),
        surface(
            "cert.surface.remote_sessions",
            "remote_sessions",
            "Remote-session and reconnect panes",
            vec![State::ProtectCore, State::Recovery],
            Level::QualifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::CriticalBatteryField),
            Some("critical-battery"),
            "eff.remote_sessions.protect_core",
        ),
        surface(
            "cert.surface.support_exports",
            "support_exports",
            "Support-export and diagnostics panes",
            vec![State::EfficiencyAware, State::Recovery],
            Level::CertifiedLowPower,
            all_drills.clone(),
            Some(LabProfileClass::BatteryUltrabook),
            Some("battery-saver"),
            "eff.support_exports.recovery",
        ),
        // Worked guardrail example: a "battery saver" badge with no materialized
        // efficiency-state evidence quarantines to the undeclared-badge floor. It
        // asserts no claim, so it does not hold promotion.
        surface(
            "cert.surface.companion_adjacent",
            "companion_adjacent",
            "Companion-adjacent assistance views",
            vec![],
            Level::UndeclaredBadge,
            CertificationDrill::ALL.to_vec(),
            None,
            None,
            "eff.companion_adjacent.badge",
        ),
    ]
}

/// Builds one certification row by running its drills against the bound evidence.
fn build_row(
    subject: &CertificationSubject,
    as_of: &str,
    lab_cases: &[EfficiencyLabCase],
    session_cases: &[SessionPressureCase],
) -> CertificationRow {
    let trace = subject
        .lab_profile
        .and_then(|profile| lookup_trace(lab_cases, profile));
    let posture = subject
        .session_case_id
        .as_deref()
        .and_then(|case_id| lookup_posture(session_cases, case_id));
    let has_any_evidence = trace.is_some() || posture.is_some();

    let drill_results = subject
        .required_drills
        .iter()
        .map(|drill| build_drill_result(*drill, as_of, trace, posture, has_any_evidence))
        .collect::<Vec<_>>();

    // Collect fired reasons in canonical order, deduplicated.
    let mut fired = Vec::new();
    for reason in CertificationNarrowingReason::ALL {
        if drill_results
            .iter()
            .any(|result| result.narrowing_reason.as_deref() == Some(reason.as_str()))
        {
            fired.push(reason);
        }
    }

    let mut effective = subject.published_claim_ceiling;
    for reason in &fired {
        effective = effective.min(reason.narrows_to());
    }
    let certification_state = CertificationState::derive(&fired, effective);

    let blocks = subject.published_claim_ceiling.is_claim_bearing()
        && effective.rank() < subject.published_claim_ceiling.rank();
    let blocker_reasons = if blocks {
        fired
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect()
    } else {
        Vec::new()
    };

    let worst_freshness = worst_freshness(&drill_results);
    let evidence_as_of = drill_results
        .iter()
        .map(|result| result.evidence_as_of.clone())
        .filter(|value| !value.is_empty())
        .max()
        .unwrap_or_default();

    let protected_paths_preserved = drill_passed(
        &drill_results,
        CertificationDrill::ProtectedPathPreservation,
    );
    let hidden_work_suppressed =
        drill_passed(&drill_results, CertificationDrill::HiddenWorkSuppression);
    let optional_work_sheds_first =
        drill_passed(&drill_results, CertificationDrill::SessionAwareShedding);

    let publication_targets = if certification_state.is_certified()
        && subject.published_claim_ceiling.is_claim_bearing()
    {
        REQUIRED_PUBLICATION_SURFACES
            .iter()
            .map(|surface| (*surface).to_owned())
            .collect()
    } else {
        Vec::new()
    };

    CertificationRow {
        record_kind: CERTIFICATION_ROW_RECORD_KIND.to_owned(),
        row_id: subject.row_id.clone(),
        subject_kind: subject.subject_kind.as_str().to_owned(),
        subject_token: subject.subject_token.clone(),
        subject_label: subject.subject_label.clone(),
        claimed_efficiency_states: subject
            .claimed_states
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect(),
        published_claim_ceiling: subject.published_claim_ceiling.as_str().to_owned(),
        required_drills: subject
            .required_drills
            .iter()
            .map(|drill| drill.as_str().to_owned())
            .collect(),
        drill_results,
        fired_narrowing_reasons: fired
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect(),
        effective_posture: effective.as_str().to_owned(),
        certification_state: certification_state.as_str().to_owned(),
        promotion_blocker: PromotionBlocker {
            blocks_promotion: blocks,
            blocker_reasons,
            posture_label: effective.label().to_owned(),
        },
        protected_paths_preserved,
        hidden_work_suppressed,
        optional_work_sheds_first,
        evidence_freshness: worst_freshness.as_str().to_owned(),
        evidence_as_of,
        publication_targets,
        governance_row_ref: subject.governance_row_ref.clone(),
    }
}

/// Runs one drill against the bound evidence and returns its result.
fn build_drill_result(
    drill: CertificationDrill,
    as_of: &str,
    trace: Option<&EfficiencyLabTrace>,
    posture: Option<&SessionPressurePosture>,
    has_any_evidence: bool,
) -> DrillResult {
    let kind = drill.evidence_kind();
    // Resolve the bound evidence for this drill's kind.
    let (evidence_refs, evidence_as_of, predicate, predicate_detail): (
        Vec<String>,
        String,
        Option<bool>,
        String,
    ) = match kind {
        CertificationEvidenceKind::EnergyThermalTrace => match trace {
            Some(trace) => (
                vec![trace.trace_id.clone()],
                trace.generated_at.clone(),
                Some(trace_predicate(drill, trace)),
                trace_detail(drill, trace),
            ),
            None => (Vec::new(), String::new(), None, String::new()),
        },
        CertificationEvidenceKind::HiddenPaneAudit => match trace {
            Some(trace) => (
                vec![format!("{}#hidden-pane-audit", trace.trace_id)],
                trace.generated_at.clone(),
                Some(trace.hidden_panes_passed),
                format!(
                    "{} hidden/off-screen surface(s) audited; no hidden pane painted, animated, or polled.",
                    trace
                        .steps
                        .iter()
                        .map(|step| step.hidden_surface_count)
                        .sum::<usize>()
                ),
            ),
            None => (Vec::new(), String::new(), None, String::new()),
        },
        CertificationEvidenceKind::SessionPressurePosture => match posture {
            Some(posture) => (
                vec![posture.support_export_ref.clone()],
                posture.observed_at.clone(),
                Some(
                    posture.optional_work_sheds_first()
                        && posture.preserves_active_session_correctness()
                        && posture.warns_before_material_downgrade(),
                ),
                "Optional assists shed before any live run regressed; correctness and authority preserved; material downgrades warned first.".to_owned(),
            ),
            None => (Vec::new(), String::new(), None, String::new()),
        },
    };

    let (freshness, outcome, passed, narrowing_reason, detail) = match predicate {
        None => {
            let freshness = if has_any_evidence {
                EvidenceFreshness::Partial
            } else {
                EvidenceFreshness::Missing
            };
            (
                freshness,
                freshness.as_str().to_owned(),
                false,
                freshness.narrowing_reason(),
                format!(
                    "No {} bound; this drill cannot be certified.",
                    kind.as_str()
                ),
            )
        }
        Some(passed) => {
            let freshness = freshness_for(&evidence_as_of, as_of);
            match freshness.narrowing_reason() {
                // Evidence is stale: it cannot certify a current claim regardless
                // of whether the predicate held on the old measurement.
                Some(reason) => (
                    freshness,
                    freshness.as_str().to_owned(),
                    false,
                    Some(reason),
                    format!(
                        "Evidence is {} relative to the packet as_of.",
                        freshness.as_str()
                    ),
                ),
                None => {
                    if passed {
                        (freshness, "pass".to_owned(), true, None, predicate_detail)
                    } else {
                        (
                            freshness,
                            "fail".to_owned(),
                            false,
                            Some(drill.failure_reason()),
                            predicate_detail,
                        )
                    }
                }
            }
        }
    };

    DrillResult {
        record_kind: CERTIFICATION_DRILL_RESULT_RECORD_KIND.to_owned(),
        drill: drill.as_str().to_owned(),
        drill_label: drill.label().to_owned(),
        proves: drill.proves().to_owned(),
        evidence_kind: kind.as_str().to_owned(),
        evidence_refs,
        evidence_as_of,
        freshness: freshness.as_str().to_owned(),
        outcome,
        passed,
        narrowing_reason: narrowing_reason.map(|reason| reason.as_str().to_owned()),
        detail,
    }
}

/// Evaluates a trace-bound drill's predicate.
fn trace_predicate(drill: CertificationDrill, trace: &EfficiencyLabTrace) -> bool {
    match drill {
        CertificationDrill::EfficiencyStateBehavior => {
            !trace.transitions.is_empty()
                && trace.every_slowdown_explained
                && trace.trace_is_content_free
        }
        CertificationDrill::ProtectedPathPreservation => trace.protected_paths_held,
        CertificationDrill::StagedRecovery => trace_has_recovery(trace),
        // The hidden-pane and session drills do not read the trace directly.
        CertificationDrill::HiddenWorkSuppression | CertificationDrill::SessionAwareShedding => {
            trace.promotion_gates_pass()
        }
    }
}

/// A content-free detail sentence for a trace-bound drill.
fn trace_detail(drill: CertificationDrill, trace: &EfficiencyLabTrace) -> String {
    match drill {
        CertificationDrill::EfficiencyStateBehavior => format!(
            "{} transition(s) materialized; every reduced surface carries a content-free reason.",
            trace.transitions.len()
        ),
        CertificationDrill::ProtectedPathPreservation => {
            "Protected interactions and durability held at every step under pressure.".to_owned()
        }
        CertificationDrill::StagedRecovery => format!(
            "Recovery transition observed; final recovery state is {}.",
            trace.final_recovery_state
        ),
        _ => "Trace promotion gates hold.".to_owned(),
    }
}

/// True when the trace exercised a staged recovery transition.
fn trace_has_recovery(trace: &EfficiencyLabTrace) -> bool {
    trace.final_recovery_state == "staged_resume"
        || trace
            .steps
            .iter()
            .any(|step| step.active_state == EfficiencyState::Recovery.as_str())
}

/// True when the named drill passed in the result set.
fn drill_passed(results: &[DrillResult], drill: CertificationDrill) -> bool {
    results
        .iter()
        .filter(|result| result.drill == drill.as_str())
        .all(|result| result.passed)
        && results.iter().any(|result| result.drill == drill.as_str())
}

/// The lowest (worst) freshness across a row's drills.
fn worst_freshness(results: &[DrillResult]) -> EvidenceFreshness {
    let mut worst = EvidenceFreshness::Current;
    for result in results {
        let freshness = parse_freshness(&result.freshness);
        if freshness_rank(freshness) > freshness_rank(worst) {
            worst = freshness;
        }
    }
    worst
}

/// Rank used to compare freshness grades; higher is worse.
fn freshness_rank(freshness: EvidenceFreshness) -> u8 {
    match freshness {
        EvidenceFreshness::Current => 0,
        EvidenceFreshness::Stale => 1,
        EvidenceFreshness::Partial => 2,
        EvidenceFreshness::Missing => 3,
    }
}

/// Parses a freshness token back into its grade, defaulting to current.
fn parse_freshness(token: &str) -> EvidenceFreshness {
    EvidenceFreshness::ALL
        .into_iter()
        .find(|freshness| freshness.as_str() == token)
        .unwrap_or(EvidenceFreshness::Current)
}

/// Grades present evidence as current or stale against the packet as_of.
fn freshness_for(evidence_as_of: &str, packet_as_of: &str) -> EvidenceFreshness {
    match (parse_date(evidence_as_of), parse_date(packet_as_of)) {
        (Some(evidence), Some(packet)) => {
            if packet - evidence > EVIDENCE_FRESHNESS_WINDOW_DAYS {
                EvidenceFreshness::Stale
            } else {
                EvidenceFreshness::Current
            }
        }
        _ => EvidenceFreshness::Current,
    }
}

/// Parses the leading `YYYY-MM-DD` of a timestamp into a civil day count.
fn parse_date(value: &str) -> Option<i64> {
    let date = value.get(0..10)?;
    let mut parts = date.split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Days since the civil epoch (1970-01-01) for a Gregorian date.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Finds the lab trace for a profile in the supplied cases.
fn lookup_trace(
    cases: &[EfficiencyLabCase],
    profile: LabProfileClass,
) -> Option<&EfficiencyLabTrace> {
    cases
        .iter()
        .find(|case| case.trace.profile_class == profile.as_str())
        .map(|case| &case.trace)
}

/// Finds the session posture for a case id in the supplied cases.
fn lookup_posture<'a>(
    cases: &'a [SessionPressureCase],
    case_id: &str,
) -> Option<&'a SessionPressurePosture> {
    cases
        .iter()
        .find(|case| case.case_id == case_id)
        .map(|case| &case.posture)
}

/// Builds the aggregate summary counts over a row set.
fn summarize(rows: &[CertificationRow]) -> CertificationSummaryCounts {
    let mut counts = CertificationSummaryCounts {
        total_rows: rows.len(),
        rows_certified: 0,
        rows_narrowed: 0,
        rows_quarantined: 0,
        profile_rows: 0,
        surface_family_rows: 0,
        claim_bearing_rows: 0,
        rows_blocking_promotion: 0,
        covered_profiles: Vec::new(),
        covered_surface_families: Vec::new(),
    };
    for row in rows {
        match row.certification_state.as_str() {
            "certified" => counts.rows_certified += 1,
            "narrowed" => counts.rows_narrowed += 1,
            "quarantined" => counts.rows_quarantined += 1,
            _ => {}
        }
        if row.subject_kind == CertifiedSubjectKind::LaptopOrDesktopProfile.as_str() {
            counts.profile_rows += 1;
            counts.covered_profiles.push(row.subject_token.clone());
        } else {
            counts.surface_family_rows += 1;
            counts
                .covered_surface_families
                .push(row.subject_token.clone());
        }
        if EfficiencyClaimLevel::from_token(&row.published_claim_ceiling)
            .is_some_and(EfficiencyClaimLevel::is_claim_bearing)
        {
            counts.claim_bearing_rows += 1;
        }
        if row.blocks_promotion() {
            counts.rows_blocking_promotion += 1;
        }
    }
    counts.covered_profiles.sort();
    counts.covered_surface_families.sort();
    counts
}

/// The downstream consumers of the proof packet.
fn publication_bindings() -> Vec<CertificationPublicationBinding> {
    vec![
        CertificationPublicationBinding {
            surface: "release".to_owned(),
            projection: "promotion_gate".to_owned(),
            ingests: "The promotion decision, blocking rows, and each row's certification state and effective posture.".to_owned(),
        },
        CertificationPublicationBinding {
            surface: "support".to_owned(),
            projection: "redaction_safe_rows".to_owned(),
            ingests: "Drill outcomes, freshness grades, and effective postures only — never raw traces or content.".to_owned(),
        },
        CertificationPublicationBinding {
            surface: "docs".to_owned(),
            projection: "certified_claim_vocabulary".to_owned(),
            ingests: "Each subject's effective posture and certification label, derived from one packet rather than cloned prose.".to_owned(),
        },
        CertificationPublicationBinding {
            surface: "help".to_owned(),
            projection: "certified_claim_vocabulary".to_owned(),
            ingests: "The same effective postures and certification labels the docs surface renders.".to_owned(),
        },
    ]
}

/// The command id a surface invokes to open the full efficiency-state details.
pub const CERTIFICATION_INSPECT_COMMAND_ID: &str = EFFICIENCY_INSPECT_COMMAND_ID;

/// The surface ref the open-details command opens.
pub const CERTIFICATION_DETAILS_SURFACE_REF: &str = EFFICIENCY_DETAILS_SURFACE_REF;
